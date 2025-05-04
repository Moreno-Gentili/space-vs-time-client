import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { DbConnection, ErrorContext, EventContext, Movable, ReducerEventContext, Vector3D } from "./module_bindings";
import { Identity } from '@clockworklabs/spacetimedb-sdk';

const tokenFile = './token.txt';

export function distance(pos1: Vector3D, pos2: Vector3D): number {
    const dx = pos1.x - pos2.x;
    const dy = pos1.y - pos2.y;
    const dz = pos1.z - pos2.z;
    return Math.sqrt(dx * dx + dy * dy + dz * dz);
}

export class SpaceVsTime {
    static connect(hostname: string): Promise<DbConnection> {
        return new Promise((resolve, reject) => {
            const conn = DbConnection.builder()
            .withUri(`ws://${hostname}`)
            .withModuleName('space-vs-time')
            .withToken(existsSync(tokenFile) ? readFileSync(tokenFile).toString() : undefined)
            .onConnect(onConnect.bind(this, resolve))
            .onConnectError(onConnectError.bind(this, reject))
            .onDisconnect(onDisconnect)
            .build();

            handleCancellation(conn);
        }).catch(() => console.error('❌ Error connecting to SpaceTimeDB')) as Promise<DbConnection>;
    }
}

function onConnect(resolve: (conn: DbConnection) => void, conn: DbConnection, identity: Identity, token: string) {
    writeFileSync(tokenFile, token);
    console.log('✅ Connected to SpacetimeDB with identity:', identity.toHexString());
    conn.reducers.onJoin(onJoin.bind(conn));
    conn.reducers.onMoveTo(onMoveTo.bind(conn));
    conn.reducers.onThrowTo(onThrowTo.bind(conn));
    conn.reducers.onSay(onSay.bind(conn));
    resolve(conn);
};

function onConnectError(reject: (reason: any) => void, _conn: ErrorContext, err: Error) {
    reject(err);
};

function onDisconnect() {
    console.log('Disconnected from SpacetimeDB');
};

function onJoin(ctx: ReducerEventContext, name: string): void {
    onReducerResult.call(this, ctx, `You joined as ${name}`, 'Join failed');
}

function onMoveTo(ctx: ReducerEventContext, x: number, y: number): void {
    onReducerResult.call(this, ctx, `You started moving to ${x},${y}`, 'MoveTo failed');
}

function onThrowTo(ctx: ReducerEventContext, x: number, y: number): void {
    onReducerResult.call(this, ctx, `You started throwing a ball to ${x},${y}`, 'ThrowTo failed');
}

function onSay(ctx: ReducerEventContext, text: string): void {
    onReducerResult.call(this, ctx, `You say: ${text}`, 'Say failed');
}

function onReducerResult(ctx: ReducerEventContext, successMessage: string, failureMessage: string): void {
    if(ctx.event.callerIdentity.data == this.identity?.data) {
        switch (ctx.event.status.tag) {
            case 'Committed':
                console.log(`✅ ${successMessage}`);
                break;
            case 'Failed':
                console.error(`❌ ${failureMessage}: ${ctx.event.status.value}`);
                break;
        }
    }
}

function handleCancellation(conn: DbConnection) {
    process.on("SIGINT", function() {
        conn.disconnect();
        console.log("Disconnected");
        process.exit();
    });
}