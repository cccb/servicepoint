//! Example for how to use the WebSocket connection

use servicepoint::*;

fn main() {
    let uri = "ws://localhost:8080".parse().unwrap();
    let connection = WebsocketConnection::open(uri).unwrap();

    connection.send(ClearCommand).unwrap();

    let mut pixels = Bitmap::max_sized();
    pixels.fill(true);

    let command = BitmapCommand {
        bitmap: pixels,
        origin: Origin::ZERO,
        compression: CompressionCode::default(),
    };
    connection.send(command).unwrap();
}
