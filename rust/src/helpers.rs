use std::sync::Mutex;
use spacetimedb_sdk::{DbContext, Error, Identity, Status};
use tokio::sync::oneshot::{self, Sender};
use lazy_static::lazy_static;
use std::fs;
use std::io;

use crate::module_bindings::{join, move_to, say, throw_to, DbConnection, ErrorContext, ReducerEventContext, Vector3D};

const TOKEN_PATH: &str = "./token.txt";

lazy_static! {
    static ref SENDER: Mutex<Option<Sender<()>>> = Mutex::new(None);
    static ref MY_IDENTITY: Mutex<Option<Identity>> = Mutex::new(None);
    static ref CONN: Mutex<Option<DbConnection>> = Mutex::new(None);
}

pub struct SpaceVsTime;

impl SpaceVsTime {
    pub fn connect(host_name: &str, then: impl FnOnce(&DbConnection) -> ()) -> impl Future<Output = ()> {
        let (sender, receiver) = oneshot::channel();
        match connect_to_spacetimedb(host_name) {
            Ok(conn) => {
                let mut mutex = SENDER.lock().unwrap();
                *mutex = Some(sender);

                register_callbacks(&conn);
                let mut mutex_conn = CONN.lock().unwrap();
                conn.run_threaded();
                
                handle_ctrlc();
                
                then(&conn);
                *mutex_conn = Some(conn);
            },
            Err(_) => {
                eprintln!("Could not connect to SpaceTimeDB");
                match sender.send(()) {
                    Ok(_) => { },
                    Err(_) => { eprintln!("Could not complete the asynchronous operation"); }
                }
            }
        }
        
        async move {
            receiver.await.unwrap_or(())
        }
    }
}

fn handle_ctrlc() {
    ctrlc::set_handler(move || {
        let mut sender_conn = CONN.lock().unwrap();
        if let Some(conn) = sender_conn.take() {
            match conn.disconnect() {
                Ok(_) => {},
                Err(err) => { eprintln!("Error during disconnect: {}", err); },
            }
        }

        let mut sender_mutex = SENDER.lock().unwrap();
        if let Some(sender) = sender_mutex.take() {
            match sender.send(()) {
                Ok(_) => { },
                Err(_) => { eprintln!("Error during async operation completion"); },
            }
        }
    })
    .expect("Error setting Ctrl-C handler");
}

fn register_callbacks(conn: &DbConnection) {
    conn.reducers.on_join(on_join);
    conn.reducers.on_move_to(on_move_to);
    conn.reducers.on_throw_to(on_throw_to);
    conn.reducers.on_say(on_say);
}

fn on_join(ctx: &ReducerEventContext, name: &String) {
    on_reducer_result(ctx, format!("You joined as {}", name).as_str(), "join failed");
}

fn on_throw_to(ctx: &ReducerEventContext, x: &f32, y: &f32) {
    on_reducer_result(ctx, format!("You started throwing a ball to {},{}", x, y).as_str(), "throw_to failed");
}

fn on_move_to(ctx: &ReducerEventContext, x: &f32, y: &f32) {
    on_reducer_result(ctx, format!("You started moving to {},{}", x, y).as_str(), "move_to failed");
}

fn on_say(ctx: &ReducerEventContext, text: &String) {
    on_reducer_result(ctx, format!("You said '{}'", text).as_str(), "say failed");
}

fn on_reducer_result(ctx: &ReducerEventContext, success_message: &str, failure_message: &str) {
    if let Some(my_identity) = *MY_IDENTITY.lock().unwrap() {
        if ctx.event.caller_identity == my_identity {
            match ctx.event.status {
                Status::Committed => { println!("✅ {}", success_message); },
                Status::Failed(_) => { println!("❌ {}: {:?}", failure_message, ctx.event.status); },
                _ => {}
            }
        }
    }
}

fn connect_to_spacetimedb(host_name: &str) -> Result<DbConnection, Error> {
    DbConnection::builder()
        .on_connect(on_connected)
        .on_connect_error(on_connect_error)
        .on_disconnect(on_disconnected)
        .with_token(read_token())
        .with_module_name("space-vs-time")
        .with_uri(host_name)
        .build()
}

fn on_connected(ctx: &DbConnection, identity: Identity, token: &str) {
    match write_token(token) {
        Ok(_) => { },
        Err(err) => { eprintln!("Failed to save token: {:?}", err); }
    }

    let mut my_identity = MY_IDENTITY.lock().unwrap();
    *my_identity = Some(identity);

    ctx.subscription_builder()
       .on_error(|_, err| eprintln!("Ciao {}", err))
       .subscribe(vec!["SELECT * FROM movables"]);

    println!("Connected to SpaceTimeDB with Identity: {}", identity);
}

fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected.");
        std::process::exit(0);
    }
}

fn read_token() -> Option<String> {
    match fs::read_to_string(TOKEN_PATH) {
        Ok(token) => Some(token),
        Err(_) => None,
    }
}

fn write_token(token: &str) -> io::Result<()> {
    fs::write(TOKEN_PATH, token)
}

pub trait VectorHelper {
    fn distance(self, other: Vector3D) -> f32;
}

impl VectorHelper for Vector3D {
    fn distance(self: Vector3D, other: Vector3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}