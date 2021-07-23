# Access Control Enumerable

Extension of `Access Control` Component that allows enumerating the members of each role.

## Usage

`Access Control Enumerable` is the extension of `Access Control`, to use this, need import `Access Control`:

```rust
#[metis_lang::contract]
pub mod contracts {
    pub use access_control_enumerable::{
        Error,
        Result,
        RoleId,
    };
    use metis_access_control_enumerable as access_control_enumerable;
    use metis_access_control as access_control;
    use metis_lang::{
        import,
        metis,
    };

    #[ink(storage)]
    #[import(access_control, access_control_enumerable)]
    pub struct AccessControl {
        access_control: access_control::Data<AccessControl>,
        access_control_enumerable : access_control_enumerable::Data<AccessControl>,

        value: bool,
    }

    // ...
}
```

Then Impl the `access_control_enumerable` for contract:

```rust
    // Note `use metis_access_control_enumerable as access_control_enumerable;`
    impl access_control_enumerable::Impl<AccessControl> for AccessControl{}
```

Define the events by `Access Control Enumerable` Component:

```rust
    /// Emitted when `new_admin_role` is set as ``role``'s admin role,
    /// replacing `previous_admin_role`
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
```

Other is same as `Access Control` Component, Need to use `access_control_enumerable` as it changed some logic for `Access Control` Component.

Impl the constructor:

```rust
    #[ink(constructor)]
    pub fn new(
        fliper: AccountId,
        setter: AccountId,
        admin: AccountId,
    ) -> Self {
        // need new both `access_control` and `access_control_enumerable`
        let mut instance = Self {
            access_control: access_control::Data::new(),
            access_control_enumerable: access_control_enumerable::Data::new(),

            // other logics
        };

        // use _setup_role to initialize the role
        access_control_enumerable::Impl::_setup_role(&mut instance, ROLE_ID_A, fliper);
        access_control_enumerable::Impl::_setup_role(&mut instance, ROLE_ID_B, setter);
        access_control_enumerable::Impl::_setup_role(&mut instance, ROLE_ID_C, admin);

        // use _set_role_admin to initialize the admin role
        access_control_enumerable::Impl::_set_role_admin(
            &mut instance,
            ROLE_ID_A,
            ROLE_ID_C,
        );

        // create the default role
        instance
    }
```

> **WARNNING** : MUST use `access_control_enumerable::Impl::xxxx` function call to use access_control_enumerable

Define Messages for `Access Control` Component:

```rust
    /// Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    pub fn has_role(&self, role: RoleId, account: AccountId) -> bool {
        access_control_enumerable::Impl::has_role(self, role, account)
    }

    /// Returns the admin role that controls `role`. See {grant_role} and
    /// {revoke_role}.
    ///
    /// To change a role's admin, use {_set_role_admin}.
    #[ink(message)]
    pub fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
        access_control_enumerable::Impl::get_role_admin(self, role)
    }

    /// Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    #[ink(message)]
    pub fn grant_role(&mut self, role: RoleId, account: AccountId) {
        access_control_enumerable::Impl::grant_role(self, role, account)
    }

    /// Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    #[ink(message)]
    pub fn revoke_role(&mut self, role: RoleId, account: AccountId) {
        access_control_enumerable::Impl::revoke_role(self, role, account)
    }

    /// Revokes `role` from the calling account.
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
        access_control_enumerable::Impl::renounce_role(self, role, account)
    }
```

Note the message from `Access Control Enumerable` Component:

```rust
    /// Returns one of the accounts that have `role`. `index` must be a
    /// value between 0 and {get_role_member_count}, non-inclusive.
    ///
    /// Role bearers are not sorted in any particular way, and their ordering may
    /// change at any point.
    ///
    /// WARNING: When using {get_role_member} and {get_role_member_count}, make sure
    /// you perform all queries on the same block.
    #[ink(message)]
    pub fn get_role_member(&self, role: RoleId, index: u32) -> AccountId {
        access_control_enumerable::Impl::get_role_member(self, &role, index as usize)
    }

    /// Returns the number of accounts that have `role`. Can be used
    /// together with {getRoleMember} to enumerate all bearers of a role.
    #[ink(message)]
    pub fn get_role_member_count(&self, role: RoleId) -> u32 {
        access_control_enumerable::Impl::get_role_member_count(self, &role) as u32
    }
```

To Use Roles to control the access in contract:

```rust
        // set the state of contract, need setter role
        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            access_control_enumerable::Impl::ensure_caller_role(self, ROLE_ID_SETTER);

            self.value = value;
        }
```

This functions can use:

- ensure_role : Panic if `owner` is not an owner
- ensure_caller_role : Panic if caller is not granted role
- ensure_admin_role : Panic error if `account` is missing the admin role of the `role`.
- check_role : Return error if `account` is missing `role`.
- check_admin_role : Return error if `account` is missing the admin role of the `role`.

In [Access Control APIs](./access-control.md#apis)

## Module

As the `Access Control Enumerable` is the extension of `Access Control`, so it is just has the `role_members` for check.

```rust
pub struct Data<E: Env> {
    role_members: StorageHashMap<RoleId, Vec<E::AccountId>>,
}
```

## Messages

The `Access Control Enumerable` add `get_role_member` and `get_role_member_count` to `Access Control`.

### get_role_member

Returns one of the accounts that have `role`. `index` must be a value between 0 and `get_role_member_count`, non-inclusive.

Role bearers are not sorted in any particular way, and their ordering may change at any point.

> WARNING: When using `get_role_member` and `get_role_member_count`, make sure
> you perform all queries on the same block. See the following
> [forum post](https://forum.openzeppelin.com/t/iterating-over-elements-on-enumerableset-in-openzeppelin-contracts/2296)
> for more information.

```rust
    /// Returns one of the accounts that have `role`. `index` must be a
    /// value between 0 and {get_role_member_count}, non-inclusive.
    ///
    /// Role bearers are not sorted in any particular way, and their ordering may
    /// change at any point.
    fn get_role_member(&self, role: &RoleId, index: usize) -> E::AccountId {
        match Storage::<E, Data<E>>::get(self).role_members.get(role) {
            None => panic!("no found role by id"),
            Some(members) => members[index].clone(), // will panic when out of index
        }
    }
```

### get_role_member_count

Returns the number of accounts that have `role`. Can be used together with {getRoleMember} to enumerate all bearers of a role.

```rust
    /// Returns the number of accounts that have `role`. Can be used
    /// together with {getRoleMember} to enumerate all bearers of a role.
    fn get_role_member_count(&self, role: &RoleId) -> usize {
        match Storage::<E, Data<E>>::get(self).role_members.get(role) {
            None => panic!("no found role by id"),
            Some(members) => members.len(),
        }
    }
```
