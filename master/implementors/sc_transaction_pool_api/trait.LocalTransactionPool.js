(function() {var implementors = {
"sc_transaction_pool":[["impl&lt;Block, Client&gt; <a class=\"trait\" href=\"sc_transaction_pool_api/trait.LocalTransactionPool.html\" title=\"trait sc_transaction_pool_api::LocalTransactionPool\">LocalTransactionPool</a> for <a class=\"struct\" href=\"sc_transaction_pool/struct.BasicPool.html\" title=\"struct sc_transaction_pool::BasicPool\">BasicPool</a>&lt;<a class=\"struct\" href=\"sc_transaction_pool/struct.FullChainApi.html\" title=\"struct sc_transaction_pool::FullChainApi\">FullChainApi</a>&lt;Client, Block&gt;, Block&gt;<span class=\"where fmt-newline\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,\n    Client: ProvideRuntimeApi&lt;Block&gt; + <a class=\"trait\" href=\"sc_client_api/client/trait.BlockBackend.html\" title=\"trait sc_client_api::client::BlockBackend\">BlockBackend</a>&lt;Block&gt; + <a class=\"trait\" href=\"sp_blockchain/backend/trait.HeaderBackend.html\" title=\"trait sp_blockchain::backend::HeaderBackend\">HeaderBackend</a>&lt;Block&gt; + <a class=\"trait\" href=\"sp_runtime/traits/trait.BlockIdTo.html\" title=\"trait sp_runtime::traits::BlockIdTo\">BlockIdTo</a>&lt;Block&gt; + <a class=\"trait\" href=\"sp_blockchain/header_metadata/trait.HeaderMetadata.html\" title=\"trait sp_blockchain::header_metadata::HeaderMetadata\">HeaderMetadata</a>&lt;Block, Error = <a class=\"enum\" href=\"sp_blockchain/error/enum.Error.html\" title=\"enum sp_blockchain::error::Error\">Error</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    Client::Api: <a class=\"trait\" href=\"sp_transaction_pool/runtime_api/trait.TaggedTransactionQueue.html\" title=\"trait sp_transaction_pool::runtime_api::TaggedTransactionQueue\">TaggedTransactionQueue</a>&lt;Block&gt;,</span>"]],
"sc_transaction_pool_api":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()