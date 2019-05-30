#![feature(duration_float)]
#[macro_use] extern crate log;
#[macro_use] extern crate specs_derive;
extern crate serde;
extern crate simplelog;
extern crate snowflake;
extern crate specs;
extern crate websocket;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use simplelog::{TermLogger, LevelFilter, Level, Config};
use specs::{World, Builder, RunNow, DispatcherBuilder};
use websocket::OwnedMessage;
use websocket::sync::Server;

mod engine;

static TICK: u64 = 2000;

fn configure_logger() {
    let logger_config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Debug),
        location: Some(Level::Debug),
        time_format: Some("%T%.3f") };
    TermLogger::init(LevelFilter::Debug, logger_config);
}

fn main() {
    configure_logger();

    type EvtTx = mpsc::Sender<engine::context::Event>;
    type EvtRx = mpsc::Receiver<engine::context::Event>;
    let (evt_tx, evt_rx): (EvtTx, EvtRx) = mpsc::channel();

    thread::spawn(move || {
        info!("Creating game context");
        let mut ctx = engine::context::Context::new();
        info!("Listening messages from clients");
        loop {
            ctx.evt(&evt_rx.recv().unwrap());  // TODO: very safe ;/
        }
    });

    let tick_evt_tx = evt_tx.clone();
    thread::spawn(move || {
        loop {
            let now = std::time::Instant::now();
            std::thread::sleep(std::time::Duration::from_millis(TICK));
            tick_evt_tx.send(engine::context::Event::Tick(now.elapsed()));
            // TODO there should go compressed data delta for the client
        }
    });

    let server = Server::bind("127.0.0.1:8081").unwrap();
    for request in server.filter_map(Result::ok) {
        let client_evt_tx = evt_tx.clone();
        thread::spawn(move || {
            // TODO: the fuck is rust-websocket protocol?
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return;
            }

            let mut client = request.use_protocol("rust-websocket").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            let id = engine::types::Id::new();
            info!("New connection, IP: [{}], ID: [{}]", ip, id);
            client_evt_tx.send(engine::context::Event::Join(id));

            let cfg = engine::interface::messages::Configuration{
                grid_x: 10,
                grid_y: 10
            };
            let message = OwnedMessage::Text(serde_json::to_string(&cfg).unwrap());
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                if let Ok(message) = message {
                    match message {
                        OwnedMessage::Close(_) => {
                            let message = OwnedMessage::Close(None);
                            sender.send_message(&message).unwrap();
                            println!("Client {} disconnected", ip);
                            return;
                        }
                        OwnedMessage::Binary(content) => info!("Got binary! {:?}", content),
                        // TODO: text interface only for dev purposes
                        OwnedMessage::Text(content) => {
                            let event = engine::context::InputEvent{
                                id: id,
                                content: engine::interface::adapters::str2input(&content)
                            };
                            client_evt_tx.send(engine::context::Event::Input(event));
                        },
                        _ => {}
                    }
                }
                else
                {
                    info!("IP: [{}], ID: [{}] left", ip, id);
                    return;
                }
            }
        });
    }
}
