//! Example for how to use the WebSocket connection

use servicepoint::{
    Bitmap, Command, CompressionCode, Connection, Grid, Origin,
};

fn main() {
    // make connection mut
    let mut connection =
        Connection::open_websocket("ws://localhost:8080".parse().unwrap())
            .unwrap();

    // use send_mut instead of send
    connection.send_mut(Command::Clear).unwrap();

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    // use send_mut instead of send
    connection
        .send_mut(Command::BitmapLinearWin(
            Origin::ZERO,
            pixels,
            CompressionCode::Lzma,
        ))
        .unwrap();
}
