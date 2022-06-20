elrond_wasm::imports!();

use crate::structs::*;

#[elrond_wasm::module]
pub trait StorageModule {

    #[view(getStructTest)]
    #[storage_mapper("structTest")]
    fn struct_test(&self) -> SingleValueMapper<Self::Api, StructTest<Self::Api>>;

    #[view(getLastValidStructId)]
    #[storage_mapper("lastValidStructId")]
    fn last_valid_struct_id(&self) -> SingleValueMapper<Self::Api, u64>;
}
