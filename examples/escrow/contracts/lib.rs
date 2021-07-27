#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod mock {
    use metis_escrow as escrow;
    use metis_lang::{
        import,
        metis,
    };
    use metis_ownable as ownable;

    #[ink(storage)]
    #[import(ownable, escrow)]
    pub struct Escrow {
        ownable: ownable::Data<Escrow>,
        escrow: escrow::Data<Escrow>,
    }

    /// Event emitted when payee deposit amount
    #[ink(event)]
    #[metis(escrow)]
    pub struct Deposited {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }

    /// Event emitted when payee withdraw
    #[ink(event)]
    #[metis(escrow)]
    pub struct Withdrawn {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    #[metis(ownable)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    // impl
    impl Escrow {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self {
                ownable: ownable::Data::new(),
                escrow: escrow::Data::new(),
            };

            ownable::Impl::init(&mut instance);

            instance
        }

        #[ink(message)]
        pub fn deposits_of(&self, payee: AccountId) -> Balance {
            escrow::Impl::deposits_of(self, &payee)
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self, payee: AccountId) {
            escrow::Impl::deposit(self, payee)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, payee: AccountId) {
            escrow::Impl::withdraw(self, payee)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            // Constructor works.
            let accounts = default_accounts();

            let mut escrow = create_contract(0);

            assert_eq!(
                escrow.deposits_of(accounts.alice),
                0,
                "deposits init should ok"
            );

            set_sender(accounts.alice, 123456);
            set_balance(accounts.alice, 123456);
            escrow.deposit(accounts.alice);

            assert_eq!(
                escrow.deposits_of(accounts.alice),
                123456,
                "deposits should ok"
            );
        }

        #[ink::test]
        fn withdraw_ok() {
            // Constructor works.
            let accounts = default_accounts();

            let mut escrow = create_contract(0);

            assert_eq!(
                escrow.deposits_of(accounts.alice),
                0,
                "deposits init should ok"
            );

            set_sender(accounts.alice, 123456);
            set_balance(accounts.alice, 0);
            escrow.deposit(accounts.alice);

            set_sender(accounts.alice, 0);
            set_balance(accounts.alice, 0);
            set_balance(contract_id(), 123456);
            
            escrow.withdraw(accounts.alice);

            assert_eq!(
                escrow.deposits_of(accounts.alice),
                0,
                "deposits now should ok"
            );
            assert_eq!(
                get_balance(contract_id()),
                0,
                "after contract balance should add"
            );
            assert_eq!(
                get_balance(accounts.alice),
                123456,
                "after balance should add"
            );
        }

        fn create_contract(initial_balance: Balance) -> Escrow {
            let accounts = default_accounts();
            set_sender(accounts.alice, 0);
            set_balance(contract_id(), initial_balance);
            Escrow::new()
        }

        fn contract_id() -> AccountId {
            test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id")
        }

        fn set_sender(sender: AccountId, endowment: Balance) {
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                endowment,
                test::CallData::new(call::Selector::new([0x00; 4])), // dummy
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Off-chain environment should have been initialized already")
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
                .expect("Cannot set account balance");
        }

        fn get_balance(account_id: AccountId) -> Balance {
            test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot set account balance")
        }
    }
}
