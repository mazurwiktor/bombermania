extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d,
    WebSocket
};

use stdweb::web::event::{
    MouseMoveEvent,
    ResizeEvent,
    KeyPressEvent,
};

use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::InputElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
                $y
        }
    };
}

fn attach_ws() {
    let ws = WebSocket::new_with_protocols(
        "ws://127.0.0.1:8081", &vec!["rust-websocket"]).unwrap();

    let text_entry: InputElement = document().query_selector( ".form input" ).unwrap().unwrap().try_into().unwrap();
    text_entry.add_event_listener( enclose!( (text_entry) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let text: String = text_entry.raw_value();
            if text.is_empty() == false {
                text_entry.set_raw_value("");
                ws.send_text(&text).unwrap();
            }
        }
    }));
}

fn main() {
    stdweb::initialize();
    attach_ws();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    //canvas.add_event_listener( enclose!( (context) move |event: MouseMoveEvent| {
    //    context.fill_rect(f64::from(event.client_x() - 5), f64::from(event.client_y() - 5), 10.0, 10.0);
    //}));

    stdweb::event_loop();
}
