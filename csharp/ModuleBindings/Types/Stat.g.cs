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
    public sealed partial class Stat
    {
        [DataMember(Name = "name")]
        public string Name;
        [DataMember(Name = "points")]
        public float Points;

        public Stat(
            string Name,
            float Points
        )
        {
            this.Name = Name;
            this.Points = Points;
        }

        public Stat()
        {
            this.Name = "";
        }
    }
}
