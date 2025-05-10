import { DbConnection, EntityKind, EntityState, EventContext, Movable } from './module_bindings';
import { SpaceVsTime, distance } from './helpers';

// TODO: Modifica l'indirizzo IP qui
SpaceVsTime.connect('127.0.0.1').then(init);

function init(conn: DbConnection) {
    // TODO: Scrivi codice qui per unirti alla partita
    // conn.reducers...
}



// TODO: questa servirà dopo...
/*
function onUpdate(ctx: EventContext, previous: Movable, current: Movable): void {
    if (current.name == "...") {
        switch (current.state.tag) {
            case EntityState.ReadyToThrow.tag:
                ctx.reducers.throwTo(-current.position.x, current.position.y);
                break;

            case EntityState.ReadyToMove.tag:
                const firstUsableBall = [...ctx.db.movables.iter()].find(m =>
                    m.kind.tag == EntityKind.Ball.tag &&
                    Math.sign(m.position.x) == Math.sign(current.position.x));

                if (firstUsableBall)
                    ctx.reducers.moveTo(firstUsableBall.position.x, firstUsableBall.position.y);

                break;
        }
    }
}
*/
