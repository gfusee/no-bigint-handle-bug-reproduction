elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct StructTest<M: ManagedTypeApi> {
    pub struct_type: StructType,
    pub sub_struct: SubStruct<M>,
    pub u64_prop: u64
}

#[derive(Clone, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum StructType {
    None,
    First,
    Second
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct SubStruct<M: ManagedTypeApi> {
    pub sub_prop1: EgldOrEsdtTokenIdentifier<M>,
    pub sub_prop2: u64,
}
