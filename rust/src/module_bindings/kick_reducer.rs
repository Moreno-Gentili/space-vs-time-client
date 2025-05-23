// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct KickArgs {
    pub name: String,
}

impl From<KickArgs> for super::Reducer {
    fn from(args: KickArgs) -> Self {
        Self::Kick { name: args.name }
    }
}

impl __sdk::InModule for KickArgs {
    type Module = super::RemoteModule;
}

pub struct KickCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `kick`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait kick {
    /// Request that the remote module invoke the reducer `kick` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_kick`] callbacks.
    fn kick(&self, name: String) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `kick`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`KickCallbackId`] can be passed to [`Self::remove_on_kick`]
    /// to cancel the callback.
    fn on_kick(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &String) + Send + 'static,
    ) -> KickCallbackId;
    /// Cancel a callback previously registered by [`Self::on_kick`],
    /// causing it not to run in the future.
    fn remove_on_kick(&self, callback: KickCallbackId);
}

impl kick for super::RemoteReducers {
    fn kick(&self, name: String) -> __sdk::Result<()> {
        self.imp.call_reducer("kick", KickArgs { name })
    }
    fn on_kick(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &String) + Send + 'static,
    ) -> KickCallbackId {
        KickCallbackId(self.imp.on_reducer(
            "kick",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::Kick { name },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, name)
            }),
        ))
    }
    fn remove_on_kick(&self, callback: KickCallbackId) {
        self.imp.remove_on_reducer("kick", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `kick`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_kick {
    /// Set the call-reducer flags for the reducer `kick` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn kick(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_kick for super::SetReducerFlags {
    fn kick(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("kick", flags);
    }
}
