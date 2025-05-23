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
    public sealed partial class Game
    {
        [DataMember(Name = "id")]
        public byte Id;
        [DataMember(Name = "name")]
        public string Name;
        [DataMember(Name = "space_score")]
        public int SpaceScore;
        [DataMember(Name = "time_score")]
        public int TimeScore;
        [DataMember(Name = "remaining")]
        public sbyte Remaining;
        [DataMember(Name = "state")]
        public GameState State;
        [DataMember(Name = "mvp")]
        public string Mvp;

        public Game(
            byte Id,
            string Name,
            int SpaceScore,
            int TimeScore,
            sbyte Remaining,
            GameState State,
            string Mvp
        )
        {
            this.Id = Id;
            this.Name = Name;
            this.SpaceScore = SpaceScore;
            this.TimeScore = TimeScore;
            this.Remaining = Remaining;
            this.State = State;
            this.Mvp = Mvp;
        }

        public Game()
        {
            this.Name = "";
            this.Mvp = "";
        }
    }
}
