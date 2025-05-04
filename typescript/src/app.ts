import { DbConnection, EventContext, Movable } from './module_bindings';
import { SpaceVsTime, distance } from './helpers';

SpaceVsTime.connect('127.0.0.1').then(init);

function init(conn: DbConnection) {
    // TODO: Scrivi codice qui
    // conn.reducers...
}