// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet-broker`.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet-broker`.
pub trait WeightInfo {
	fn configure() -> Weight;
	fn reserve() -> Weight;
	fn unreserve() -> Weight;
	fn set_lease() -> Weight;
	fn start_sales() -> Weight;
	fn purchase() -> Weight;
	fn renew() -> Weight;
	fn transfer() -> Weight;
	fn partition() -> Weight;
	fn interlace() -> Weight;
	fn assign() -> Weight;
	fn pool() -> Weight;
	fn claim_revenue(m: u32,) -> Weight;
	fn purchase_credit() -> Weight;
	fn drop_region() -> Weight;
	fn drop_contribution() -> Weight;
	fn drop_history() -> Weight;
	fn request_core_count() -> Weight;
	fn process_core_count() -> Weight;
	fn process_revenue() -> Weight;
	fn rotate_sale() -> Weight;
	fn process_pool() -> Weight;
	fn process_core_schedule() -> Weight;
	fn request_revenue_info_at() -> Weight;
}

/// Weights for `pallet-broker` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn configure() -> Weight { Weight::from_parts(123, 456) }
	fn reserve() -> Weight { Weight::from_parts(123, 456) }
	fn unreserve() -> Weight { Weight::from_parts(123, 456) }
	fn set_lease() -> Weight { Weight::from_parts(123, 456) }
	fn start_sales() -> Weight { Weight::from_parts(123, 456) }
	fn purchase() -> Weight { Weight::from_parts(123, 456) }
	fn renew() -> Weight { Weight::from_parts(123, 456) }
	fn transfer() -> Weight { Weight::from_parts(123, 456) }
	fn partition() -> Weight { Weight::from_parts(123, 456) }
	fn interlace() -> Weight { Weight::from_parts(123, 456) }
	fn assign() -> Weight { Weight::from_parts(123, 456) }
	fn pool() -> Weight { Weight::from_parts(123, 456) }
	fn claim_revenue(_: u32,) -> Weight { Weight::from_parts(123, 456) }
	fn purchase_credit() -> Weight { Weight::from_parts(123, 456) }
	fn drop_region() -> Weight { Weight::from_parts(123, 456) }
	fn drop_contribution() -> Weight { Weight::from_parts(123, 456) }
	fn drop_history() -> Weight { Weight::from_parts(123, 456) }
	fn request_core_count() -> Weight { Weight::from_parts(123, 456) }
	fn process_core_count() -> Weight { Weight::from_parts(123, 456) }
	fn process_revenue() -> Weight { Weight::from_parts(123, 456) }
	fn rotate_sale() -> Weight { Weight::from_parts(123, 456) }
	fn process_pool() -> Weight { Weight::from_parts(123, 456) }
	fn process_core_schedule() -> Weight { Weight::from_parts(123, 456) }
	fn request_revenue_info_at() -> Weight { Weight::from_parts(123, 456) }
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn configure() -> Weight { Weight::from_parts(123, 456) }
	fn reserve() -> Weight { Weight::from_parts(123, 456) }
	fn unreserve() -> Weight { Weight::from_parts(123, 456) }
	fn set_lease() -> Weight { Weight::from_parts(123, 456) }
	fn start_sales() -> Weight { Weight::from_parts(123, 456) }
	fn purchase() -> Weight { Weight::from_parts(123, 456) }
	fn renew() -> Weight { Weight::from_parts(123, 456) }
	fn transfer() -> Weight { Weight::from_parts(123, 456) }
	fn partition() -> Weight { Weight::from_parts(123, 456) }
	fn interlace() -> Weight { Weight::from_parts(123, 456) }
	fn assign() -> Weight { Weight::from_parts(123, 456) }
	fn pool() -> Weight { Weight::from_parts(123, 456) }
	fn claim_revenue(_: u32,) -> Weight { Weight::from_parts(123, 456) }
	fn purchase_credit() -> Weight { Weight::from_parts(123, 456) }
	fn drop_region() -> Weight { Weight::from_parts(123, 456) }
	fn drop_contribution() -> Weight { Weight::from_parts(123, 456) }
	fn drop_history() -> Weight { Weight::from_parts(123, 456) }
	fn request_core_count() -> Weight { Weight::from_parts(123, 456) }
	fn process_core_count() -> Weight { Weight::from_parts(123, 456) }
	fn process_revenue() -> Weight { Weight::from_parts(123, 456) }
	fn rotate_sale() -> Weight { Weight::from_parts(123, 456) }
	fn process_pool() -> Weight { Weight::from_parts(123, 456) }
	fn process_core_schedule() -> Weight { Weight::from_parts(123, 456) }
	fn request_revenue_info_at() -> Weight { Weight::from_parts(123, 456) }
}
