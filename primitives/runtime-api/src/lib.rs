#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait StorageQueryApi {
		fn get_data() -> u32;
	}
}
