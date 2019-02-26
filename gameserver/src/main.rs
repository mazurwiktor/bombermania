#[macro_use] extern crate log;
#[macro_use] extern crate specs_derive;
extern crate futures;
extern crate simplelog;
extern crate snowflake;
extern crate specs;
extern crate tokio;
extern crate websocket;

use simplelog::{TermLogger, LevelFilter, Level, Config};
use specs::{World, Builder, RunNow, DispatcherBuilder};

mod engine;

use websocket::async::Server;
use websocket::message::OwnedMessage;
use websocket::server::InvalidConnection;

use tokio::runtime;
use tokio::runtime::TaskExecutor;

use futures::future::{self, Loop};
use futures::sync::mpsc;
use futures::{Future, Sink, Stream};

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

type CommonEventTx = mpsc::UnboundedSender<engine::context::CommonEvent>;
type CommonEventRx = mpsc::UnboundedReceiver<engine::context::CommonEvent>;
type InputEventTx = mpsc::UnboundedSender<engine::context::InputEvent>;
type InputEventRx = mpsc::UnboundedReceiver<engine::context::InputEvent>;

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


    // --- playground
    /*
    let fake_id_1 = engine::types::Id::new();
    let fake_id_2 = engine::types::Id::new();
    ctx.evt_common(&engine::context::CommonEvent::Join(fake_id_1));
    ctx.evt_common(&engine::context::CommonEvent::Join(fake_id_2));
    let mut fake_keystrokes = engine::interface::Input::new();
    fake_keystrokes.up = true;
    let evt = engine::context::InputEvent{id: fake_id_1, content: fake_keystrokes};
    ctx.evt_input(&evt);
    ctx.evt_input(&evt); // y for player 1 should be now y+=1
    */
    // --- playground end

    jump_into_the_loop();
}

fn jump_into_the_loop() {
    let mut ctx = engine::context::Context::new();

    let runtime = runtime::Builder::new().build().unwrap();
    let executor = runtime.executor();
    let reactor = runtime.reactor().clone();
    let server = Server::bind("127.0.0.1:8081", &reactor).expect("Failed to create server");
    let connections = Arc::new(RwLock::new(HashMap::new()));

    let (receive_channel_out, receive_channel_in) = mpsc::unbounded();
    let connections_inner = connections.clone();

    let connection_handler = server
        .incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(move |(upgrade, addr)| {
            let connections_inner = connections_inner.clone();
            println!("Got a connection from: {}", addr);
            let channel = receive_channel_out.clone();
            let executor_inner = executor.clone();
            let f = upgrade.accept().and_then(move |(framed, _)| {
                let id = engine::types::Id::new();
                let (sink, stream) = framed.split();
                let f = channel.send((id, stream));
                spawn_future(f, "Send stream to connection pool", &executor_inner);
                connections_inner.write().unwrap().insert(id, sink);
                Ok(())
            });
            spawn_future(f, "Handle new connection", &executor);
            Ok(())
        })
    .map_err(|_| ());

    let receive_handler = receive_channel_in.for_each(move |(id, stream)| {
        stream.for_each(move |msg| {
            handle_rx(id, &msg);
            Ok(())
        }).map_err(|_| ())
    });

    let (send_channel_out, send_channel_in) = mpsc::unbounded();

    let connections_inner = connections.clone();
    let executor = runtime.executor();
    let executor_inner = executor.clone();

    // TODO: make it fn handle_rx
    let send_handler = send_channel_in
        .for_each(move |(id, msg): (engine::types::Id, String)| {
            let connections = connections_inner.clone();
            let sink = connections
                .write()
                .unwrap()
                .remove(&id)
                .expect("Tried to send to invalid client id");

            println!(">>> Sending message '{}' to id {}", msg, id);
            let f = sink
                .send(OwnedMessage::Text(msg))
                .and_then(move |sink| {
                    connections.write().unwrap().insert(id, sink);
                    Ok(())
                })
            .map_err(|_| ());
            executor_inner.spawn(f);
            Ok(())
        })
    .map_err(|_| ());

    let main_loop = future::loop_fn((), move |_| {
        let connections = connections.clone();
        let send_channel_out = send_channel_out.clone();
        let executor = executor.clone();

        // TODO: tick duration should be read from config file - I think in the future I'd be
        // messing around to find exact value of the tick
        tokio::timer::Delay::new(Instant::now() + Duration::from_millis(10000))
            .map_err(|_| ())
            .and_then(move |_| {
                let should_continue = on_tick(connections, send_channel_out, &executor);
                match should_continue {
                    Ok(true) => Ok(Loop::Continue(())),
                    Ok(false) => Ok(Loop::Break(())),
                    Err(()) => Err(()),
                }
            })
    });

    let handlers =
        main_loop.select2(connection_handler.select2(receive_handler.select(send_handler)));

    runtime
        .block_on_all(handlers)
        .map_err(|_| println!("Error while running core loop"))
        .unwrap();
}

fn spawn_future<F, I, E>(f: F, desc: &'static str, executor: &TaskExecutor)
    where
    F: Future<Item = I, Error = E> + 'static + Send,
    E: Debug
{
    executor.spawn(
        f.map_err(move |e| println!("Error in {}: '{:?}'", desc, e))
        .map(move |_| println!("{}: Finished.", desc)),
        );
}

fn handle_rx(id: engine::types::Id, msg: &OwnedMessage) {
    if let OwnedMessage::Text(ref txt) = *msg {
        println!("<<< Received message '{}' from id {}", txt, id);
    }
}

type SinkContent = websocket::client::async::Framed<
    tokio::net::TcpStream,
    websocket::async::MessageCodec<OwnedMessage>,
>;
type SplitSink = futures::stream::SplitSink<SinkContent>;

fn on_tick(
    connections: Arc<RwLock<HashMap<engine::types::Id, SplitSink>>>,
    channel: mpsc::UnboundedSender<(engine::types::Id, String)>,
    executor: &TaskExecutor) -> Result<bool, ()> {
    let executor_inner = executor.clone();
    executor.spawn(futures::lazy(move || {
        for (id, _) in connections.read().unwrap().iter() {
            let f = channel.clone().send((*id, "THERE SHOULD GO DELTA CHANGE".to_owned()));
            spawn_future(f, "Send message to write handler", &executor_inner);
        }
        Ok(())
    }));

    Ok(true)
}
