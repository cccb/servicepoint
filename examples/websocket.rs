//! Example for how to use the WebSocket connection

use servicepoint::{
    Bitmap, Command, CompressionCode, Connection, Grid, Origin,
};

fn main() {
    let connection =
        Connection::open_websocket("ws://localhost:8080".parse().unwrap())
            .unwrap();

    connection.send(Command::Clear).unwrap();

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    connection
        .send(Command::BitmapLinearWin(
            Origin::ZERO,
            pixels,
            CompressionCode::Lzma,
        ))
        .unwrap();
}
