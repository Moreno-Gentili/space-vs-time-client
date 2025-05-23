// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct SpawnTimeBallsArgs {
    pub count: u8,
}

impl From<SpawnTimeBallsArgs> for super::Reducer {
    fn from(args: SpawnTimeBallsArgs) -> Self {
        Self::SpawnTimeBalls { count: args.count }
    }
}

impl __sdk::InModule for SpawnTimeBallsArgs {
    type Module = super::RemoteModule;
}

pub struct SpawnTimeBallsCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `spawn_time_balls`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait spawn_time_balls {
    /// Request that the remote module invoke the reducer `spawn_time_balls` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_spawn_time_balls`] callbacks.
    fn spawn_time_balls(&self, count: u8) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `spawn_time_balls`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`SpawnTimeBallsCallbackId`] can be passed to [`Self::remove_on_spawn_time_balls`]
    /// to cancel the callback.
    fn on_spawn_time_balls(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &u8) + Send + 'static,
    ) -> SpawnTimeBallsCallbackId;
    /// Cancel a callback previously registered by [`Self::on_spawn_time_balls`],
    /// causing it not to run in the future.
    fn remove_on_spawn_time_balls(&self, callback: SpawnTimeBallsCallbackId);
}

impl spawn_time_balls for super::RemoteReducers {
    fn spawn_time_balls(&self, count: u8) -> __sdk::Result<()> {
        self.imp
            .call_reducer("spawn_time_balls", SpawnTimeBallsArgs { count })
    }
    fn on_spawn_time_balls(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &u8) + Send + 'static,
    ) -> SpawnTimeBallsCallbackId {
        SpawnTimeBallsCallbackId(self.imp.on_reducer(
            "spawn_time_balls",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::SpawnTimeBalls { count },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, count)
            }),
        ))
    }
    fn remove_on_spawn_time_balls(&self, callback: SpawnTimeBallsCallbackId) {
        self.imp.remove_on_reducer("spawn_time_balls", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `spawn_time_balls`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_spawn_time_balls {
    /// Set the call-reducer flags for the reducer `spawn_time_balls` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn spawn_time_balls(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_spawn_time_balls for super::SetReducerFlags {
    fn spawn_time_balls(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("spawn_time_balls", flags);
    }
}
