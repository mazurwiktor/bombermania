#[macro_use] extern crate log;
#[macro_use] extern crate specs_derive;
extern crate simplelog;
extern crate snowflake;
extern crate specs;
extern crate websocket;

use std::sync::mpsc;

use simplelog::{TermLogger, LevelFilter, Level, Config};
use specs::{World, Builder, RunNow, DispatcherBuilder};

mod engine;

fn configure_logger() {
    let logger_config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Debug),
        location: Some(Level::Debug),
        time_format: Some("%T%.3f") };
    let _ = TermLogger::init(LevelFilter::Debug, logger_config);
}

fn main() {
    configure_logger();

    let mut ctx = engine::context::Context::new();

    // --- playground
    let fake_id_1 = engine::types::Id::new();
    let fake_id_2 = engine::types::Id::new();
    ctx.add_player(&fake_id_1, 11, 11);
    ctx.add_player(&fake_id_2, 22, 22);
    let mut fake_keystrokes = engine::interface::Input::new();
    fake_keystrokes.up = true;
    let evt = engine::context::InputEvent{id: fake_id_1, content: fake_keystrokes};
    ctx.event(&evt);
    ctx.event(&evt); // y for player 1 should be now y+=1

    // --- playground end

    let (tx, rx): (
        mpsc::Sender<engine::context::InputEvent>,
        mpsc::Receiver<engine::context::InputEvent>) = mpsc::channel();

    info!("websocket@20000 is listening");
    let server = websocket::sync::Server::bind("0.0.0.0:20000").unwrap();
    for request in server.filter_map(Result::ok) {
        let thread_tx = tx.clone();

        // TODO: consider async here instead of threads
        // (or maybe predefined pool of threads?)
        std::thread::spawn(move || {
            let mut client = request.use_protocol("rust-websocket").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            // TODO: id should be generated for new unique IPs (connection lost should be handled
            // somehow)
            let id = engine::types::Id::new();
            debug!("connection from {}; given id: {}", ip, id);

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                if !message.is_ok() { continue }
                let message = message.unwrap();

                match message {
                    websocket::OwnedMessage::Close(_) => {
                        let message = websocket::OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        debug!("Client {} disconnected", ip);
                        return;
                    },
                    websocket::OwnedMessage::Ping(ping) => {
                        let message = websocket::OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    },
                    websocket::OwnedMessage::Text(text) => {
                        debug!("received: {}", &text);
                        let input_event = engine::context::InputEvent {
                            content: engine::interface::adapters::str2input(&text),
                            id: id
                        };
                        thread_tx.send(input_event);
                    },
                    _ => {
                        error!("unexpected msg: {:?}", &message)
                    }
                }
            }
        });
        let received = rx.recv().unwrap();  // stream to ECS
    }
}
