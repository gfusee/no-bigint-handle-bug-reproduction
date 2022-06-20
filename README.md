# elrond-bigint-handle-issue-rs

This smart contract is a reproduction of the `no bigInt under the given handle` error.

## Contract structure

This contract contains 3 endpoints :

### createStruct

This endpoint create an instance of StructTest with fixed values and store it in the storage.

### uselessEndpoint

This endpoint is totally useless and used nowhere, we will talk more about later.

### problematicEndpoint

This endpoint is the endpoint that produce the bug.

## The bug

### Explanation

When calling `problematicEndpoint` after `createStruct`, an unexpected execution failed error `no bigInt under the given handle` appears.
This error ONLY happens when running tests via mandos using `erdpy contract test` command. When running tests with Rust Testing Framework everything works as expected.

### Strange behaviour

The bug does NOT appear in the following (stranges) cases :

- We delete `uselessEndpoint`.
- In `problematic_function`, we return a fixed value, for example if we replace line 122 by `OptionalValue::Some(1)`
- We remove ANY of the `require!(...)` in `problematic_function`