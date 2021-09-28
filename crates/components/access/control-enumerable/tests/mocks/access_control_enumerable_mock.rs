#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod mock {
    pub const ROLE_ID_FLIPER: RoleId = RoleId::new([0x01; 32]);
    pub const ROLE_ID_SETTER: RoleId = RoleId::new([0x02; 32]);
    pub const ROLE_ID_ADMIN: RoleId = RoleId::new([0x03; 32]);

    pub use access_control_enumerable::{
        Error,
        Result,
        RoleId,
    };
    use metis_access_control as access_control;
    use metis_access_control_enumerable as access_control_enumerable;
    use metis_lang::{
        import,
        metis,
    };

    #[ink(storage)]
    #[import(access_control, access_control_enumerable)]
    pub struct AccessControl {
        access_control: access_control::Data<AccessControl>,
        access_control_enumerable: access_control_enumerable::Data<AccessControl>,

        value: bool,
    }

    impl access_control_enumerable::Impl<AccessControl> for AccessControl {}

    /// Emitted when `new_admin_role` is set as ``role``'s admin role, replacing `previous_admin_role`
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
    /// bearer except when using {_setup_role}.
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
    ///   - if using `revoke_role`, it is the admin role bearer
    ///   - if using `renounce_role`, it is the role bearer (i.e. `account`)
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
                access_control_enumerable: access_control_enumerable::Data::new(),

                value: init_value,
            };

            access_control_enumerable::Impl::_setup_role(
                &mut instance,
                ROLE_ID_FLIPER,
                fliper,
            );
            access_control_enumerable::Impl::_setup_role(
                &mut instance,
                ROLE_ID_SETTER,
                setter,
            );
            access_control_enumerable::Impl::_setup_role(
                &mut instance,
                ROLE_ID_ADMIN,
                admin,
            );

            access_control_enumerable::Impl::_set_role_admin(
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
            access_control_enumerable::Impl::ensure_caller_role(self, ROLE_ID_FLIPER);

            self.value = !self.value;

            Ok(())
        }

        // set the state of contract, need setter role
        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            access_control_enumerable::Impl::ensure_caller_role(self, ROLE_ID_SETTER);

            self.value = value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Returns `true` if `account` has been granted `role`.
        #[ink(message)]
        pub fn has_role(&self, role: RoleId, account: AccountId) -> bool {
            access_control_enumerable::Impl::has_role(self, role, account)
        }

        #[ink(message)]
        pub fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
            access_control_enumerable::Impl::get_role_admin(self, role)
        }

        #[ink(message)]
        pub fn grant_role(&mut self, role: RoleId, account: AccountId) {
            access_control_enumerable::Impl::grant_role(self, role, account)
        }

        #[ink(message)]
        pub fn revoke_role(&mut self, role: RoleId, account: AccountId) {
            access_control_enumerable::Impl::revoke_role(self, role, account)
        }

        #[ink(message)]
        pub fn renounce_role(&mut self, role: RoleId, account: AccountId) {
            access_control_enumerable::Impl::renounce_role(self, role, account)
        }

        #[ink(message)]
        pub fn get_role_member(&self, role: RoleId, index: u32) -> AccountId {
            access_control_enumerable::Impl::get_role_member(self, &role, index as usize)
        }

        #[ink(message)]
        pub fn get_role_member_count(&self, role: RoleId) -> u32 {
            access_control_enumerable::Impl::get_role_member_count(self, &role) as u32
        }

        #[ink(message)]
        pub fn _setup_role(&mut self, role: RoleId, account: AccountId) {
            access_control_enumerable::Impl::_setup_role(self, role, account)
        }
    }
}
