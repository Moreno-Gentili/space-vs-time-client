#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod module_bindings;
mod helpers;
use helpers::{SpaceVsTime, VectorHelper};
use module_bindings::*;
use spacetimedb_sdk::{Error, Table, TableWithPrimaryKey};

#[tokio::main]
async fn main() {
    // TODO: Modifica l'indirizzo IP qui
    SpaceVsTime::connect("127.0.0.1", init).await;
}

fn init(conn: &DbConnection) {
    // TODO: Scrivi codice qui per controllare il tuo personaggio
    // conn.reducers...
}




// TODO: Questa servirà dopo...
/*
fn on_update(ctx: &EventContext, _previous: &Movable, current: &Movable) {
    if current.name == "WAN" {
        match current.state {
            EntityState::ReadyToThrow => {
                ctx.reducers.throw_to(-current.position.x, current.position.y);
            },
            EntityState::ReadyToMove => {
                if let Some(first_usable_ball) = ctx.db.movables().iter().find(|m|
                    m.kind == EntityKind::Ball &&
                    m.position.x.signum() == current.position.x.signum()) {
                        ctx.reducers.move_to(first_usable_ball.position.x, first_usable_ball.position.y);
                    }
            },
            _ => {}
        }
    }
}
*/