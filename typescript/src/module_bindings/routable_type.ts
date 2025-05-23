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
import { Vector3D as __Vector3D } from "./vector_3_d_type";
import { RouteAction as __RouteAction } from "./route_action_type";

export type Routable = {
  name: string,
  counter: number,
  destination: __Vector3D,
  action: __RouteAction,
};

/**
 * A namespace for generated helper functions.
 */
export namespace Routable {
  /**
  * A function which returns this type represented as an AlgebraicType.
  * This function is derived from the AlgebraicType used to generate this type.
  */
  export function getTypeScriptAlgebraicType(): AlgebraicType {
    return AlgebraicType.createProductType([
      new ProductTypeElement("name", AlgebraicType.createStringType()),
      new ProductTypeElement("counter", AlgebraicType.createU8Type()),
      new ProductTypeElement("destination", __Vector3D.getTypeScriptAlgebraicType()),
      new ProductTypeElement("action", __RouteAction.getTypeScriptAlgebraicType()),
    ]);
  }

  export function serialize(writer: BinaryWriter, value: Routable): void {
    Routable.getTypeScriptAlgebraicType().serialize(writer, value);
  }

  export function deserialize(reader: BinaryReader): Routable {
    return Routable.getTypeScriptAlgebraicType().deserialize(reader);
  }

}


