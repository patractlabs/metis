#[allow(dead_code)]
pub fn next_call_by(account: &ink_env::AccountId) {
    // Get contract address.
    let callee =
        ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
    // Create call.
    let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4]));

    data.push_arg(account);

    // Push the new execution context to set from as caller.
    ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
        account.clone(),
        callee,
        1000000,
        1000000,
        data,
    );
}
