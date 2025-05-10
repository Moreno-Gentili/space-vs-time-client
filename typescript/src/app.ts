import { DbConnection, EventContext, Movable } from './module_bindings';
import { SpaceVsTime, distance } from './helpers';

// TODO: Modifica l'indirizzo IP qui
SpaceVsTime.connect('127.0.0.1').then(init);

function init(conn: DbConnection) {
    // TODO: Scrivi codice qui per controllare il tuo personaggio
    // conn.reducers...
}