// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `Inspect` and `Mutate` traits for working with regular balances.

use crate::{
	dispatch::DispatchError,
	ensure,
	traits::{
		tokens::{
			misc::{Balance, DepositConsequence, KeepAlive, WithdrawConsequence},
			Imbalance as ImbalanceT,
		},
		SameOrOther, TryDrop,
	},
};
use sp_arithmetic::traits::{CheckedAdd, CheckedSub};
use sp_runtime::{traits::Saturating, ArithmeticError, DispatchResult, TokenError};
use sp_std::marker::PhantomData;

use super::{CreditOf, DebtOf, HandleImbalanceDrop, Imbalance};

/// Trait for providing balance-inspection access to a fungible asset.
pub trait Inspect<AccountId> {
	/// Scalar type for representing balance of an account.
	type Balance: Balance;

	/// The total amount of issuance in the system.
	fn total_issuance() -> Self::Balance;

	/// The total amount of issuance in the system excluding those which are controlled by the
	/// system.
	fn active_issuance() -> Self::Balance {
		Self::total_issuance()
	}

	/// The minimum balance any single account may have.
	fn minimum_balance() -> Self::Balance;

	/// Get the total amount of funds whose ultimate bneficial ownership can be determined as `who`.
	///
	/// This may include funds which are wholly inaccessible to `who`, either temporarily or even
	/// indefinitely.
	///
	/// For the amount of the balance which is currently free to be removed from the account without
	/// error, use `reducible_balance`.
	///
	/// For the amount of the balance which may eventually be free to be removed from the account,
	/// use `balance()`.
	fn total_balance(who: &AccountId) -> Self::Balance;

	/// Get the balance of `who` which does not include funds which are exclusively allocated to
	/// subsystems of the chain ("on hold" or "reserved").
	///
	/// In general this isn't especially useful outside of tests, and for practical purposes, you'll
	/// want to use `reducible_balance()`.
	fn balance(who: &AccountId) -> Self::Balance;

	/// Get the maximum amount that `who` can withdraw/transfer successfully based on whether the
	/// account should be kept alive (`keep_alive`) or whether we are willing to force the reduction
	/// and potentially go below user-level restrictions on the minimum amount of the account.
	///
	/// Always less than `balance()`.
	fn reducible_balance(who: &AccountId, keep_alive: KeepAlive, force: bool) -> Self::Balance;

	/// Returns `true` if the balance of `who` may be increased by `amount`.
	///
	/// - `who`: The account of which the balance should be increased by `amount`.
	/// - `amount`: How much should the balance be increased?
	/// - `mint`: Will `amount` be minted to deposit it into `account`?
	fn can_deposit(who: &AccountId, amount: Self::Balance, mint: bool) -> DepositConsequence;

	/// Returns `Failed` if the balance of `who` may not be decreased by `amount`, otherwise
	/// the consequence.
	fn can_withdraw(who: &AccountId, amount: Self::Balance) -> WithdrawConsequence<Self::Balance>;
}

/// A fungible token class where the balance can be set arbitrarily.
///
/// **WARNING**
/// Do not use this directly unless you want trouble, since it allows you to alter account balances
/// without keeping the issuance up to date. It has no safeguards against accidentally creating
/// token imbalances in your system leading to accidental imflation or deflation. It's really just
/// for the underlying datatype to implement so the user gets the much safer `Balanced` trait to
/// use.
pub trait Unbalanced<AccountId>: Inspect<AccountId> {
	/// Forcefully set the balance of `who` to `amount`.
	///
	/// If this call executes successfully, you can `assert_eq!(Self::balance(), amount);`.
	///
	/// For implementations which include one or more balances on hold, then these are *not*
	/// included in the `amount`.
	///
	/// This function does its best to force the balance change through, but will not break system
	/// invariants such as any Existential Deposits needed or overflows/underflows.
	/// If this cannot be done for some reason (e.g. because the account cannot be created, deleted
	/// or would overflow) then an `Err` is returned.
	fn set_balance(who: &AccountId, amount: Self::Balance) -> DispatchResult;

	/// Set the total issuance to `amount`.
	fn set_total_issuance(amount: Self::Balance);

	/// Reduce the balance of `who` by `amount`.
	///
	/// If `best_effort` is `false` and it cannot be reduced by that amount for
	/// some reason, return `Err` and don't reduce it at all. If `best_effort` is `true`, then
	/// reduce the balance of `who` by the most that is possible, up to `amount`.
	///
	/// In either case, if `Ok` is returned then the inner is the amount by which is was reduced.
	/// Minimum balance will be respected and thus the returned amount may be up to
	/// `Self::minimum_balance() - 1` greater than `amount` in the case that the reduction caused
	/// the account to be deleted.
	fn decrease_balance(
		who: &AccountId,
		mut amount: Self::Balance,
		best_effort: bool,
		keep_alive: KeepAlive,
		force: bool,
	) -> Result<Self::Balance, DispatchError> {
		let old_balance = Self::balance(who);
		let free = Self::reducible_balance(who, keep_alive, force);
		if best_effort {
			amount = amount.min(free);
		}
		let new_balance = old_balance.checked_sub(&amount).ok_or(TokenError::FundsUnavailable)?;
		Self::set_balance(who, new_balance)?;
		Ok(old_balance.saturating_sub(new_balance))
	}

	/// Increase the balance of `who` by `amount`.
	///
	/// If it cannot be increased by that amount for some reason, return `Err` and don't increase
	/// it at all. If Ok, return the imbalance.
	/// Minimum balance will be respected and an error will be returned if
	/// `amount < Self::minimum_balance()` when the account of `who` is zero.
	fn increase_balance(
		who: &AccountId,
		amount: Self::Balance,
		best_effort: bool,
	) -> Result<Self::Balance, DispatchError> {
		let old_balance = Self::balance(who);
		let new_balance = if best_effort {
			old_balance.saturating_add(amount)
		} else {
			old_balance.checked_add(&amount).ok_or(ArithmeticError::Overflow)?
		};
		if new_balance < Self::minimum_balance() {
			// Attempt to increase from 0 to below minimum -> stays at zero.
			if best_effort {
				Ok(Default::default())
			} else {
				Err(TokenError::BelowMinimum.into())
			}
		} else {
			if new_balance == old_balance {
				Ok(Default::default())
			} else {
				Self::set_balance(who, new_balance)?;
				Ok(new_balance.saturating_sub(old_balance))
			}
		}
	}

	/// Reduce the active issuance by some amount.
	fn deactivate(_: Self::Balance) {}

	/// Increase the active issuance by some amount, up to the outstanding amount reduced.
	fn reactivate(_: Self::Balance) {}
}

/// Trait for providing a basic fungible asset.
pub trait Mutate<AccountId>: Inspect<AccountId> + Unbalanced<AccountId> {
	/// Increase the balance of `who` by exactly `amount`, minting new tokens. If that isn't
	/// possible then an `Err` is returned and nothing is changed.
	fn mint_into(who: &AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
		Self::total_issuance().checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
		let actual = Self::increase_balance(who, amount, false)?;
		Self::set_total_issuance(Self::total_issuance().saturating_add(actual));
		Self::done_mint_into(who, amount);
		Ok(actual)
	}

	/// Decrease the balance of `who` by at least `amount`, possibly slightly more in the case of
	/// minimum-balance requirements, burning the tokens. If that isn't possible then an `Err` is
	/// returned and nothing is changed. If successful, the amount of tokens reduced is returned.
	fn burn_from(
		who: &AccountId,
		amount: Self::Balance,
		best_effort: bool,
		force: bool,
	) -> Result<Self::Balance, DispatchError> {
		let actual = Self::reducible_balance(who, KeepAlive::CanKill, force).min(amount);
		ensure!(actual == amount || best_effort, TokenError::FundsUnavailable);
		Self::total_issuance().checked_sub(&actual).ok_or(ArithmeticError::Overflow)?;
		let actual = Self::decrease_balance(who, actual, true, KeepAlive::CanKill, force)?;
		Self::set_total_issuance(Self::total_issuance().saturating_sub(actual));
		Self::done_burn_from(who, actual);
		Ok(actual)
	}

	/// Attempt to increase the `asset` balance of `who` by `amount`.
	///
	/// Equivalent to `burn_from`, except with an expectation that within the bounds of some
	/// universal issuance, the total assets `suspend`ed and `resume`d will be equivalent. The
	/// implementation may be configured such that the total assets suspended may never be less than
	/// the total assets resumed (which is the invariant for an issuing system), or the reverse
	/// (which the invariant in a non-issuing system).
	///
	/// Because of this expectation, any metadata associated with the asset is expected to survive
	/// the suspect-resume cycle.
	fn shelve(who: &AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
		let actual = Self::reducible_balance(who, KeepAlive::CanKill, false).min(amount);
		ensure!(actual == amount, TokenError::FundsUnavailable);
		Self::total_issuance().checked_sub(&actual).ok_or(ArithmeticError::Overflow)?;
		let actual = Self::decrease_balance(who, actual, true, KeepAlive::CanKill, false)?;
		Self::set_total_issuance(Self::total_issuance().saturating_sub(actual));
		Self::done_shelve(who, actual);
		Ok(actual)
	}

	/// Attempt to increase the `asset` balance of `who` by `amount`.
	///
	/// Equivalent to `mint_into`, except with an expectation that within the bounds of some
	/// universal issuance, the total assets `suspend`ed and `resume`d will be equivalent. The
	/// implementation may be configured such that the total assets suspended may never be less than
	/// the total assets resumed (which is the invariant for an issuing system), or the reverse
	/// (which the invariant in a non-issuing system).
	///
	/// Because of this expectation, any metadata associated with the asset is expected to survive
	/// the suspect-resume cycle.
	fn restore(who: &AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
		Self::total_issuance().checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
		let actual = Self::increase_balance(who, amount, false)?;
		Self::set_total_issuance(Self::total_issuance().saturating_add(actual));
		Self::done_restore(who, amount);
		Ok(actual)
	}

	/// Transfer funds from one account into another.
	fn transfer(
		source: &AccountId,
		dest: &AccountId,
		amount: Self::Balance,
		keep_alive: KeepAlive,
	) -> Result<Self::Balance, DispatchError> {
		let _extra =
			Self::can_withdraw(source, amount).into_result(keep_alive != KeepAlive::CanKill)?;
		Self::can_deposit(dest, amount, false).into_result()?;
		Self::decrease_balance(source, amount, true, keep_alive, false)?;
		// This should never fail as we checked `can_deposit` earlier. But we do a best-effort
		// anyway.
		let _ = Self::increase_balance(dest, amount, true);
		Self::done_transfer(source, dest, amount);
		Ok(amount)
	}

	/// Simple infallible function to force an account to have a particular balance, good for use
	/// in tests and benchmarks but not recommended for production code owing to the lack of
	/// error reporting.
	///
	/// Returns the new balance.
	fn make_balance_be(who: &AccountId, amount: Self::Balance) -> Self::Balance {
		let b = Self::balance(who);
		if b > amount {
			Self::burn_from(who, b - amount, true, true).map(|d| amount.saturating_sub(d))
		} else {
			Self::mint_into(who, amount - b).map(|d| amount.saturating_add(d))
		}
		.unwrap_or(b)
	}

	fn done_mint_into(_who: &AccountId, _amount: Self::Balance) {}
	fn done_burn_from(_who: &AccountId, _amount: Self::Balance) {}
	fn done_shelve(_who: &AccountId, _amount: Self::Balance) {}
	fn done_restore(_who: &AccountId, _amount: Self::Balance) {}
	fn done_transfer(_source: &AccountId, _dest: &AccountId, _amount: Self::Balance) {}
}

/// Simple handler for an imbalance drop which increases the total issuance of the system by the
/// imbalance amount. Used for leftover debt.
pub struct IncreaseIssuance<AccountId, U>(PhantomData<(AccountId, U)>);
impl<AccountId, U: Unbalanced<AccountId>> HandleImbalanceDrop<U::Balance>
	for IncreaseIssuance<AccountId, U>
{
	fn handle(amount: U::Balance) {
		U::set_total_issuance(U::total_issuance().saturating_add(amount))
	}
}

/// Simple handler for an imbalance drop which decreases the total issuance of the system by the
/// imbalance amount. Used for leftover credit.
pub struct DecreaseIssuance<AccountId, U>(PhantomData<(AccountId, U)>);
impl<AccountId, U: Unbalanced<AccountId>> HandleImbalanceDrop<U::Balance>
	for DecreaseIssuance<AccountId, U>
{
	fn handle(amount: U::Balance) {
		U::set_total_issuance(U::total_issuance().saturating_sub(amount))
	}
}

/// A fungible token class where any creation and deletion of tokens is semi-explicit and where the
/// total supply is maintained automatically.
///
/// This is auto-implemented when a token class has `Unbalanced` implemented.
pub trait Balanced<AccountId>: Inspect<AccountId> + Unbalanced<AccountId> {
	/// The type for managing what happens when an instance of `Debt` is dropped without being used.
	type OnDropDebt: HandleImbalanceDrop<Self::Balance>;
	/// The type for managing what happens when an instance of `Credit` is dropped without being
	/// used.
	type OnDropCredit: HandleImbalanceDrop<Self::Balance>;

	/// Reduce the total issuance by `amount` and return the according imbalance. The imbalance will
	/// typically be used to reduce an account by the same amount with e.g. `settle`.
	///
	/// This is infallible, but doesn't guarantee that the entire `amount` is burnt, for example
	/// in the case of underflow.
	fn rescind(amount: Self::Balance) -> DebtOf<AccountId, Self> {
		let old = Self::total_issuance();
		let new = old.saturating_sub(amount);
		Self::set_total_issuance(new);
		let delta = old - new;
		Self::done_rescind(delta);
		Imbalance::<Self::Balance, Self::OnDropDebt, Self::OnDropCredit>::new(delta)
	}

	/// Increase the total issuance by `amount` and return the according imbalance. The imbalance
	/// will typically be used to increase an account by the same amount with e.g.
	/// `resolve_into_existing` or `resolve_creating`.
	///
	/// This is infallible, but doesn't guarantee that the entire `amount` is issued, for example
	/// in the case of overflow.
	fn issue(amount: Self::Balance) -> CreditOf<AccountId, Self> {
		let old = Self::total_issuance();
		let new = old.saturating_add(amount);
		Self::set_total_issuance(new);
		let delta = new - old;
		Self::done_issue(delta);
		Imbalance::<Self::Balance, Self::OnDropCredit, Self::OnDropDebt>::new(delta)
	}

	/// Produce a pair of imbalances that cancel each other out exactly.
	///
	/// This is just the same as burning and issuing the same amount and has no effect on the
	/// total issuance.
	fn pair(amount: Self::Balance) -> (DebtOf<AccountId, Self>, CreditOf<AccountId, Self>) {
		(Self::rescind(amount), Self::issue(amount))
	}

	/// Mints `value` into the account of `who`, creating it as needed.
	///
	/// If `best_effort` is `true` and `value` in full could not be minted (e.g. due to overflow),
	/// then the maximum is minted, up to `value`. If `best_effort` is `false`, then exactly `value`
	/// must be minted into the account of `who` or the operation will fail with an `Err` and
	/// nothing will change.
	///
	/// If the operation is successful, this will return `Ok` with a `Debt` of the total value
	/// added to the account.
	fn deposit(
		who: &AccountId,
		value: Self::Balance,
		best_effort: bool,
	) -> Result<DebtOf<AccountId, Self>, DispatchError> {
		let increase = Self::increase_balance(who, value, best_effort)?;
		Self::done_deposit(who, increase);
		Ok(Imbalance::<Self::Balance, Self::OnDropDebt, Self::OnDropCredit>::new(increase))
	}

	/// Removes `value` balance from `who` account if possible.
	///
	/// If `best_effort` is `true` and `value` in full could not be removed (e.g. due to underflow),
	/// then the maximum is removed, up to `value`. If `best_effort` is `false`, then exactly
	/// `value` must be removed from the account of `who` or the operation will fail with an `Err`
	/// and nothing will change.
	///
	/// If the removal is needed but not possible, then it returns `Err` and nothing is changed.
	/// If the account needed to be deleted, then slightly more than `value` may be removed from the
	/// account owning since up to (but not including) minimum balance may also need to be removed.
	///
	/// If the operation is successful, this will return `Ok` with a `Credit` of the total value
	/// removed from the account.
	fn withdraw(
		who: &AccountId,
		value: Self::Balance,
		best_effort: bool,
		keep_alive: KeepAlive,
		force: bool,
	) -> Result<CreditOf<AccountId, Self>, DispatchError> {
		let decrease = Self::decrease_balance(who, value, best_effort, keep_alive, force)?;
		Self::done_withdraw(who, decrease);
		Ok(Imbalance::<Self::Balance, Self::OnDropCredit, Self::OnDropDebt>::new(decrease))
	}

	/// The balance of `who` is increased in order to counter `credit`. If the whole of `credit`
	/// cannot be countered, then nothing is changed and the original `credit` is returned in an
	/// `Err`.
	///
	/// Please note: If `credit.peek()` is less than `Self::minimum_balance()`, then `who` must
	/// already exist for this to succeed.
	fn resolve(
		who: &AccountId,
		credit: CreditOf<AccountId, Self>,
	) -> Result<(), CreditOf<AccountId, Self>> {
		let v = credit.peek();
		let debt = match Self::deposit(who, v, false) {
			Err(_) => return Err(credit),
			Ok(d) => d,
		};
		let result = credit.offset(debt).try_drop();
		debug_assert!(result.is_ok(), "ok deposit return must be equal to credit value; qed");
		Ok(())
	}

	/// The balance of `who` is decreased in order to counter `debt`. If the whole of `debt`
	/// cannot be countered, then nothing is changed and the original `debt` is returned in an
	/// `Err`.
	fn settle(
		who: &AccountId,
		debt: DebtOf<AccountId, Self>,
		keep_alive: KeepAlive,
	) -> Result<CreditOf<AccountId, Self>, DebtOf<AccountId, Self>> {
		let amount = debt.peek();
		let credit = match Self::withdraw(who, amount, false, keep_alive, false) {
			Err(_) => return Err(debt),
			Ok(d) => d,
		};

		match credit.offset(debt) {
			SameOrOther::None => Ok(CreditOf::<AccountId, Self>::zero()),
			SameOrOther::Same(dust) => Ok(dust),
			SameOrOther::Other(rest) => {
				debug_assert!(false, "ok withdraw return must be at least debt value; qed");
				Err(rest)
			},
		}
	}

	fn done_rescind(_amount: Self::Balance) {}
	fn done_issue(_amount: Self::Balance) {}
	fn done_deposit(_who: &AccountId, _amount: Self::Balance) {}
	fn done_withdraw(_who: &AccountId, _amount: Self::Balance) {}
}
