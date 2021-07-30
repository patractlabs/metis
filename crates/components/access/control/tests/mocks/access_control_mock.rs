#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod access_control_mock {
    pub const ROLE_ID_FLIPER: RoleId = RoleId::new([0x01; 32]);
    pub const ROLE_ID_SETTER: RoleId = RoleId::new([0x02; 32]);
    pub const ROLE_ID_ADMIN: RoleId = RoleId::new([0x03; 32]);

    pub use access_control::{
        Error,
        Result,
        RoleId,
    };
    use metis_access_control as access_control;
    use metis_lang::{
        import,
        metis,
    };

    #[ink(storage)]
    #[import(access_control)]
    pub struct AccessControl {
        access_control: access_control::Data<AccessControl>,

        value: bool,
    }

    /// Emitted when `newAdminRole` is set as ``role``'s admin role, replacing `previousAdminRole`
    ///
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// {RoleAdminChanged} not being emitted signaling this.
    #[ink(event)]
    #[metis(access_control)]
    pub struct RoleAdminChanged {
        #[ink(topic)]
        pub role: RoleId,
        #[ink(topic)]
        pub previous_admin_role: Option<RoleId>,
        #[ink(topic)]
        pub new_admin_role: RoleId,
    }

    /// Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call, an admin role
    /// bearer except when using {_setupRole}.
    #[ink(event)]
    #[metis(access_control)]
    pub struct RoleGranted {
        #[ink(topic)]
        pub role: RoleId,
        #[ink(topic)]
        pub account: AccountId,
        #[ink(topic)]
        pub sender: AccountId,
    }

    /// Emitted when `account` is revoked `role`.
    ///
    /// `sender` is the account that originated the contract call:
    ///   - if using `revokeRole`, it is the admin role bearer
    ///   - if using `renounceRole`, it is the role bearer (i.e. `account`)
    #[ink(event)]
    #[metis(access_control)]
    pub struct RoleRevoked {
        #[ink(topic)]
        pub role: RoleId,
        #[ink(topic)]
        pub account: AccountId,
        #[ink(topic)]
        pub sender: AccountId,
    }

    // impl
    impl AccessControl {
        #[ink(constructor)]
        pub fn new(
            init_value: bool,
            fliper: AccountId,
            setter: AccountId,
            admin: AccountId,
        ) -> Self {
            let mut instance = Self {
                access_control: access_control::Data::new(),

                value: init_value,
            };

            access_control::Impl::_setup_role(&mut instance, ROLE_ID_FLIPER, fliper);
            access_control::Impl::_setup_role(&mut instance, ROLE_ID_SETTER, setter);
            access_control::Impl::_setup_role(&mut instance, ROLE_ID_ADMIN, admin);

            access_control::Impl::_set_role_admin(
                &mut instance,
                ROLE_ID_FLIPER,
                ROLE_ID_ADMIN,
            );

            // create the default role
            instance
        }

        // flip the state of contract, need flipper role
        #[ink(message)]
        pub fn flip(&mut self) -> Result<()> {
            access_control::Impl::ensure_caller_role(self, ROLE_ID_FLIPER);

            self.value = !self.value;

            Ok(())
        }

        // set the state of contract, need setter role
        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            access_control::Impl::ensure_caller_role(self, ROLE_ID_SETTER);

            self.value = value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Returns `true` if `account` has been granted `role`.
        #[ink(message)]
        pub fn has_role(&self, role: RoleId, account: AccountId) -> bool {
            access_control::Impl::has_role(self, role, account)
        }

        #[ink(message)]
        pub fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
            access_control::Impl::get_role_admin(self, role)
        }

        #[ink(message)]
        pub fn grant_role(&mut self, role: RoleId, account: AccountId) {
            access_control::Impl::grant_role(self, role, account)
        }

        #[ink(message)]
        pub fn revoke_role(&mut self, role: RoleId, account: AccountId) {
            access_control::Impl::revoke_role(self, role, account)
        }

        #[ink(message)]
        pub fn renounce_role(&mut self, role: RoleId, account: AccountId) {
            access_control::Impl::renounce_role(self, role, account)
        }
    }
}
