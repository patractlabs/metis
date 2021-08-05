# Access Control

Contract module that allows children to implement role-based access control mechanisms. This is a lightweight version that doesn't allow enumerating role members except through off-chain means by accessing the contract event logs. Some applications may benefit from on-chain enumerability, for those cases see `access-control-eunmerable`.

Roles are referred to by their `RoleId` which is a 32-bytes. These should be exposed in the external API and be unique. The best way to achieve this is by using hash digests.

Roles can be used to represent a set of permissions. To restrict access to a function call, use `ensure_caller_role`:

```rust
    // need role
    #[ink(message)]
    pub fn func(&mut self) {
        access_control::Impl::ensure_caller_role(self, ROLE_ID_XXX);

        // other logics
    }
```

Roles can be granted and revoked dynamically via the `grant_role` and `revoke_role` functions. Each role has an associated admin role, and only accounts that have a role's admin role can call `grant_role` and `revoke_role`.

## Usage

To use the Access Control Component, First is import the `access_control`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod example {
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
    pub struct Contract {
        access_control: access_control::Data<Contract>,

        // other modules
    }
```

Define the Event for access control:

```rust
    /// Emitted when `new_admin_role` is set as ``role``'s
    /// admin role, replacing `previous_admin_role`
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
    /// `sender` is the account that originated the contract call,
    /// an admin role bearer except when using {_setupRole}.
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
```

Impl the `constructor` of the contract:

```rust
    #[ink(constructor)]
    pub fn new(
        fliper: AccountId,
        setter: AccountId,
        admin: AccountId,
    ) -> Self {
        let mut instance = Self {
            // Need add access_control module
            access_control: access_control::Data::new(),
        };

        // Use the `_setup_role` set the ROLEs
        access_control::Impl::_setup_role(&mut instance, ROLE_ID_FLIPER, fliper);
        access_control::Impl::_setup_role(&mut instance, ROLE_ID_SETTER, setter);
        access_control::Impl::_setup_role(&mut instance, ROLE_ID_ADMIN, admin);

        // Use `_set_role_admin` set the role admin
        access_control::Impl::_set_role_admin(
            &mut instance,
            ROLE_ID_FLIPER,
            ROLE_ID_ADMIN,
        );

        // create the default role
        instance
    }
```

> WARNNING: In metis, access control not have a `default admin role` which be admin role of all roles, so we need defined the role releations in constructor.

In constructor, we can use `_setup_role` and `_set_role_admin` to set the role releations for contract.

Next, Add the message to control the role releations.

```rust
    /// Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    pub fn has_role(&self, role: RoleId, account: AccountId) -> bool {
        access_control::Impl::has_role(self, role, account)
    }

    /// @dev Returns the admin role that controls `role`. See {grant_role} and
    /// {revoke_role}.
    ///
    /// To change a role's admin, use {_setRoleAdmin}.
    #[ink(message)]
    pub fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
        access_control::Impl::get_role_admin(self, role)
    }

    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    #[ink(message)]
    pub fn grant_role(&mut self, role: RoleId, account: AccountId) {
        access_control::Impl::grant_role(self, role, account)
    }

    /// @dev Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    #[ink(message)]
    pub fn revoke_role(&mut self, role: RoleId, account: AccountId) {
        access_control::Impl::revoke_role(self, role, account)
    }

    /// @dev Revokes `role` from the calling account.
    ///
    /// Roles are often managed via {grant_role} and {revoke_role}: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// If the calling account had been granted `role`, emits a {RoleRevoked}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must be `account`.
    #[ink(message)]
    pub fn renounce_role(&mut self, role: RoleId, account: AccountId) {
        access_control::Impl::renounce_role(self, role, account)
    }
```

## Module

The module contains the `roles` and `admin_roles`:

```rust
/// The Data of access control component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
    /// the account - role relationship map
    pub roles: StorageHashMap<(RoleId, E::AccountId), ()>,

    /// the admin role of a role
    pub admin_roles: StorageHashMap<RoleId, RoleId>,
}
```

## Messages for Txs

### grant_role

Grants `role` to `account`. If `account` had not been already granted `role`, emits a {RoleGranted}
event.

Requirements:

- the caller must have ``role``'s admin role.

```rust
    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn grant_role(&mut self, role: RoleId, account: E::AccountId) {
        // check the admin role
        self.ensure_admin_role(role, Self::caller());

        self._setup_role(role, account);
    }
```

### revoke_role

Revokes `role` from `account`. If `account` had been granted `role`, emits a {RoleRevoked} event.

Requirements:

- the caller must have ``role``'s admin role.

```rust
    /// @dev Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn revoke_role(&mut self, role: RoleId, account: E::AccountId) {
        let caller = Self::caller();

        // check the admin role
        self.ensure_admin_role(role, caller.clone());

        // if has not role
        self.get_mut()
            .revoke_role(role, account.clone())
            .expect("no has role");

        // emit if revoke role success
        self.emit_event_role_revoked(role, account, caller);
    }
```

### renounce_role

Revokes `role` from the calling account.

Roles are often managed via `grant_role` and `revoke_role`: this function's purpose is to provide a mechanism for accounts to lose their privileges if they are compromised (such as when a trusted device is misplaced).

If the calling account had been granted `role`, emits a `RoleRevoked` event.

Requirements:

- the caller must be `account`.

```rust
    /// @dev Revokes `role` from the calling account.
    ///
    /// Roles are often managed via {grant_role} and {revoke_role}: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// If the calling account had been granted `role`, emits a {RoleRevoked}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must be `account`.
    fn renounce_role(&mut self, role: RoleId, account: E::AccountId) {
        let caller = Self::caller();

        // check the caller is account
        assert!(caller == account, "account not caller");

        // if has not role
        self.get_mut()
            .revoke_role(role, account.clone())
            .expect("no has role");

        // emit if revoke role success
        self.emit_event_role_revoked(role, account, caller);
    }
```

## Message for Querys

Use `has_role` and `get_role_admin` can to get the role releations of accounts.

### has_role

Returns `true` if `account` has been granted `role`.

```rust
    /// Returns `true` if `account` has been granted `role`.
    fn has_role(&self, role: RoleId, account: E::AccountId) -> bool {
        self.get().has_role(role, account)
    }
```

### get_role_admin

Returns the admin role that controls `role`. See {grant_role} and {revoke_role}.

To change a role's admin, use `_set_role_admin`.

```rust
    /// @dev Returns the admin role that controls `role`. See {grant_role} and
    /// {revoke_role}.
    ///
    /// To change a role's admin, use {_set_role_admin}.
    fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
        self.get().admin_roles.get(&role).copied()
    }
```

## APIs

Use Apis to check and ensure account has role.

### ensure_role

Panic if `owner` is not an owner.

```rust
    /// Panic if `owner` is not an owner
    fn ensure_role(&self, role: RoleId, account: E::AccountId) {
        assert!(self.has_role(role, account), "role missing");
    }
```

### ensure_caller_role

Panic if caller is not granted role.

```rust
    /// Panic if caller is not granted role
    fn ensure_caller_role(&self, role: RoleId) {
        self.ensure_role(role, Self::caller());
    }
```

### ensure_admin_role

Panic error if `account` is missing the admin role of the `role`.

```rust
    /// Panic error if `account` is missing the admin role of the `role`.
    fn ensure_admin_role(&self, role: RoleId, account: E::AccountId) {
        match self.get_role_admin(role) {
            Some(admin_role) => self.ensure_role(admin_role, account),
            None => panic!("admin role missing"),
        }
    }
```

### check_role

Return error if `account` is missing `role`.

```rust
    /// Return error if `account` is missing `role`.
    fn check_role(&self, role: RoleId, account: E::AccountId) -> Result<()> {
        if self.has_role(role, account) {
            Ok(())
        } else {
            Err(Error::RoleNotFound)
        }
    }
```

### check_admin_role

Return error if `account` is missing the admin role of the `role`.

```rust
    /// Return error if `account` is missing the admin role of the `role`.
    fn check_admin_role(&self, role: RoleId, account: E::AccountId) -> Result<()> {
        match self.get_role_admin(role) {
            Some(admin_role) => self.check_role(admin_role, account),
            None => Err(Error::AdminRoleNotFound),
        }
    }
```

## Events

### RoleAdminChanged

Emitted when `new_admin_role` is set as ``role``'s admin role, replacing `previous_admin_role`

- will emit by call `_set_role_admin`.

```rust
    /// Emitted when `new_admin_role` is set as ``role``'s
    /// admin role, replacing `previous_admin_role`
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
```

### RoleGranted

Emitted when `account` is granted `role`.

- will emit by call `_setup_role`.

```rust
    /// Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call,
    /// an admin role bearer except when using {_setup_role}.
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
```

### RoleRevoked

Emitted when `account` is revoked `role`.

`sender` is the account that originated the contract call:

- if using `revoke_role`, it is the admin role bearer
- if using `renounce_role`, it is the role bearer (i.e. `account`)

```rust
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
```
