mod module_bindings;
mod helpers;
use helpers::{SpaceVsTime, VectorHelper};
use module_bindings::*;
use spacetimedb_sdk::Error;

#[tokio::main]
async fn main() {
    // TODO: Modifica l'indirizzo IP qui
    SpaceVsTime::connect("127.0.0.1", init).await;
}

fn init(conn: &DbConnection) -> Result<(), Error> {
    // TODO: Scrivi codice qui per controllare il tuo personaggio
    // conn.reducers...
    
    Ok(())
}