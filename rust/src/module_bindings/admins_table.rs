// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::admin_type::Admin;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `admins`.
///
/// Obtain a handle from the [`AdminsTableAccess::admins`] method on [`super::RemoteTables`],
/// like `ctx.db.admins()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.admins().on_insert(...)`.
pub struct AdminsTableHandle<'ctx> {
    imp: __sdk::TableHandle<Admin>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `admins`.
///
/// Implemented for [`super::RemoteTables`].
pub trait AdminsTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`AdminsTableHandle`], which mediates access to the table `admins`.
    fn admins(&self) -> AdminsTableHandle<'_>;
}

impl AdminsTableAccess for super::RemoteTables {
    fn admins(&self) -> AdminsTableHandle<'_> {
        AdminsTableHandle {
            imp: self.imp.get_table::<Admin>("admins"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct AdminsInsertCallbackId(__sdk::CallbackId);
pub struct AdminsDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for AdminsTableHandle<'ctx> {
    type Row = Admin;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = Admin> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = AdminsInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> AdminsInsertCallbackId {
        AdminsInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: AdminsInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = AdminsDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> AdminsDeleteCallbackId {
        AdminsDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: AdminsDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<Admin>("admins");
    _table.add_unique_constraint::<__sdk::Identity>("identity", |row| &row.identity);
}
pub struct AdminsUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for AdminsTableHandle<'ctx> {
    type UpdateCallbackId = AdminsUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> AdminsUpdateCallbackId {
        AdminsUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: AdminsUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<Admin>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<Admin>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `identity` unique index on the table `admins`,
/// which allows point queries on the field of the same name
/// via the [`AdminsIdentityUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.admins().identity().find(...)`.
pub struct AdminsIdentityUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<Admin, __sdk::Identity>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> AdminsTableHandle<'ctx> {
    /// Get a handle on the `identity` unique index on the table `admins`.
    pub fn identity(&self) -> AdminsIdentityUnique<'ctx> {
        AdminsIdentityUnique {
            imp: self
                .imp
                .get_unique_constraint::<__sdk::Identity>("identity"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> AdminsIdentityUnique<'ctx> {
    /// Find the subscribed row whose `identity` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sdk::Identity) -> Option<Admin> {
        self.imp.find(col_val)
    }
}
