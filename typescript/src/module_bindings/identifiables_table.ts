// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
import {
  AlgebraicType,
  AlgebraicValue,
  BinaryReader,
  BinaryWriter,
  CallReducerFlags,
  ConnectionId,
  DbConnectionBuilder,
  DbConnectionImpl,
  DbContext,
  ErrorContextInterface,
  Event,
  EventContextInterface,
  Identity,
  ProductType,
  ProductTypeElement,
  ReducerEventContextInterface,
  SubscriptionBuilderImpl,
  SubscriptionEventContextInterface,
  SumType,
  SumTypeVariant,
  TableCache,
  TimeDuration,
  Timestamp,
  deepEqual,
} from "@clockworklabs/spacetimedb-sdk";
import { Identifiable } from "./identifiable_type";
import { EntityKind as __EntityKind } from "./entity_kind_type";

import { EventContext, Reducer, RemoteReducers, RemoteTables } from ".";

/**
 * Table handle for the table `identifiables`.
 *
 * Obtain a handle from the [`identifiables`] property on [`RemoteTables`],
 * like `ctx.db.identifiables`.
 *
 * Users are encouraged not to explicitly reference this type,
 * but to directly chain method calls,
 * like `ctx.db.identifiables.on_insert(...)`.
 */
export class IdentifiablesTableHandle {
  tableCache: TableCache<Identifiable>;

  constructor(tableCache: TableCache<Identifiable>) {
    this.tableCache = tableCache;
  }

  count(): number {
    return this.tableCache.count();
  }

  iter(): Iterable<Identifiable> {
    return this.tableCache.iter();
  }
  /**
   * Access to the `name` unique index on the table `identifiables`,
   * which allows point queries on the field of the same name
   * via the [`IdentifiablesNameUnique.find`] method.
   *
   * Users are encouraged not to explicitly reference this type,
   * but to directly chain method calls,
   * like `ctx.db.identifiables.name().find(...)`.
   *
   * Get a handle on the `name` unique index on the table `identifiables`.
   */
  name = {
    // Find the subscribed row whose `name` column value is equal to `col_val`,
    // if such a row is present in the client cache.
    find: (col_val: string): Identifiable | undefined => {
      for (let row of this.tableCache.iter()) {
        if (deepEqual(row.name, col_val)) {
          return row;
        }
      }
    },
  };

  onInsert = (cb: (ctx: EventContext, row: Identifiable) => void) => {
    return this.tableCache.onInsert(cb);
  }

  removeOnInsert = (cb: (ctx: EventContext, row: Identifiable) => void) => {
    return this.tableCache.removeOnInsert(cb);
  }

  onDelete = (cb: (ctx: EventContext, row: Identifiable) => void) => {
    return this.tableCache.onDelete(cb);
  }

  removeOnDelete = (cb: (ctx: EventContext, row: Identifiable) => void) => {
    return this.tableCache.removeOnDelete(cb);
  }

  // Updates are only defined for tables with primary keys.
  onUpdate = (cb: (ctx: EventContext, oldRow: Identifiable, newRow: Identifiable) => void) => {
    return this.tableCache.onUpdate(cb);
  }

  removeOnUpdate = (cb: (ctx: EventContext, onRow: Identifiable, newRow: Identifiable) => void) => {
    return this.tableCache.removeOnUpdate(cb);
  }}
