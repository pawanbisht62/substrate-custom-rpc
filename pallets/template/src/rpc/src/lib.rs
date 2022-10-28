pub use pallet_runtime_api::StorageQueryApi;
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;


#[rpc(client, server)]
pub trait StorageQueryApi<BlockHash> {
	#[method(name = "template_get_data")]
	fn get_data(&self, at: Option<BlockHash>) -> RpcResult<u32>;
}

/// Astruct that implements StorageQeuryApi
pub struct TemplatePallet<C, Block> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TemplatePallet<C, Block> {
	/// Create new `TemplatePallet` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {client, _marker: Default::default() }
	}
}

impl<C, Block> StorageQueryApiServer<<Block as BlockT>::Hash> for TemplatePallet<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: StorageQueryApi<Block>
{
	fn get_data(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		api.get_data(&at).map_err(runtime_error_into_rpc_err)
	}

}

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Runtime error",
		Some(format!("{:?}", err)),
	)).into()
}

