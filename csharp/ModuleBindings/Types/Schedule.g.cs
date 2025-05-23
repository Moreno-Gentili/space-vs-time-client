// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#nullable enable

using System;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    [SpacetimeDB.Type]
    [DataContract]
    public sealed partial class Schedule
    {
        [DataMember(Name = "scheduled_id")]
        public ulong ScheduledId;
        [DataMember(Name = "scheduled_at")]
        public SpacetimeDB.ScheduleAt ScheduledAt;

        public Schedule(
            ulong ScheduledId,
            SpacetimeDB.ScheduleAt ScheduledAt
        )
        {
            this.ScheduledId = ScheduledId;
            this.ScheduledAt = ScheduledAt;
        }

        public Schedule()
        {
            this.ScheduledAt = null!;
        }
    }
}
