//! Example for how to use the WebSocket connection

use servicepoint::connection::Websocket;
use servicepoint::{
    Bitmap, Command, CompressionCode, Connection, Grid, Origin,
};

fn main() {
    let connection =
        Websocket::open("ws://localhost:8080".parse().unwrap()).unwrap();

    connection.send(Command::Clear).unwrap();

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    connection
        .send(Command::BitmapLinearWin(
            Origin::ZERO,
            pixels,
            CompressionCode::default(),
        ))
        .unwrap();
}
