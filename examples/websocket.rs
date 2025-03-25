//! Example for how to use the WebSocket connection

use servicepoint::{
    Bitmap, BitmapCommand, ClearCommand, Connection, Grid, WebsocketConnection,
};

fn main() {
    let uri = "ws://localhost:8080".parse().unwrap();
    let connection = WebsocketConnection::open(uri).unwrap();

    connection.send(ClearCommand).unwrap();

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    let command = BitmapCommand::from(pixels);
    connection.send(command).unwrap();
}
