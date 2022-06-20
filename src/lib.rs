#![no_std]

elrond_wasm::imports!();

pub mod structs;

use structs::*;

mod storage;

#[elrond_wasm::contract]
pub trait IssueContract:
    storage::StorageModule
{
    #[init]
    fn init(&self) {
    }

    #[endpoint(createStruct)]
    fn create_struct(
        &self,
    ) {
        let struct_test = StructTest {
            struct_type: StructType::First,
            sub_struct: SubStruct {
                sub_prop1: EgldOrEsdtTokenIdentifier::egld(),
                sub_prop2: 0
            },
            u64_prop: 0
        };

        self.struct_test().set(&struct_test);
    }

    #[endpoint(uselessEndpoint)]
    fn useless_endpoint(
        &self,
        arg1: u64,
        optional_arg2: OptionalValue<u64>
    ) -> OptionalValue<u64> {

        self.problematic_function(
            self.blockchain().get_caller(),
            TokenIdentifier::from(ManagedBuffer::from(b"TEST-123456")),
            0,
            BigUint::zero(),
            arg1,
            optional_arg2
        )
    }

    #[endpoint(problematicEndpoint)]
    fn problematic_endpoint(
        &self
    ) {
        let caller = self.blockchain().get_caller();

        let struct_test = self.struct_test().get();

        let payment_struct = EgldOrEsdtTokenPayment {
            token_identifier: EgldOrEsdtTokenIdentifier::egld(),
            token_nonce: struct_test.sub_struct.sub_prop2,
            amount: BigUint::from(10u64)
        };

        let _ = self.problematic_function(
            caller,
            TokenIdentifier::from(ManagedBuffer::from(b"TEST-123456")),
            payment_struct.token_nonce,
            payment_struct.amount,
            1,
            OptionalValue::None
        );

    }

    fn problematic_function(
        &self,
        arg1: ManagedAddress<Self::Api>,
        arg2: TokenIdentifier<Self::Api>,
        arg3: u64,
        arg4: BigUint<Self::Api>,
        arg5: u64,
        optional_arg6: OptionalValue<u64>
    ) -> OptionalValue<u64> {
        let mut struct_test = self.struct_test().get();

        if arg5 == 1 { //this is always true after createStruct endpoint call
            require!(
                arg4 > 0, //this line cause "no bigInt under the given handle" error
                "OK 1"
            );
            sc_panic!("expected error");
        }

        require!(
            arg1 != ManagedAddress::zero(),
            "OK 2"
        );
        require!(
            arg2 == TokenIdentifier::from(ManagedBuffer::from(b"TEST-123456"))
            && arg3 == 0,
            "OK 3"
        );

        let opt_arg8 = optional_arg6.into_option();

        if let Some(arg8) = opt_arg8 {
            require!(
                arg8 > 0,
                "OK 4"
            )
        }

        let mut result: OptionalValue<u64> = OptionalValue::None;

        // update auction bid and winner
        if struct_test.struct_type == StructType::First {
            result = OptionalValue::Some(struct_test.u64_prop);
        }

        result
    }
}