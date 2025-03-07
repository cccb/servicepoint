//! Example for how to use the WebSocket connection

use servicepoint::connection::Websocket;
use servicepoint::*;

fn main() {
    let uri = "ws://localhost:8080".parse().unwrap();
    let connection = Websocket::open(uri).unwrap();

    connection.send(command::Clear).unwrap();

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    let command = command::BitmapLinearWin {
        origin: Origin::ZERO,
        bitmap: pixels,
        compression: CompressionCode::default(),
    };
    connection.send(command).unwrap();
}
