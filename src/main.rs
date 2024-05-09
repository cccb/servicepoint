use std::thread;
use std::time::Duration;
use servicepoint2::{Command, Connection, Origin, PIXEL_HEIGHT, PIXEL_WIDTH, PixelGrid, Size, TILE_WIDTH, Window};

fn main() {
    // 172.23.42.29
    let connection = Connection::open("localhost:2342").unwrap();
    connection.send(Command::Clear).unwrap();

    connection.send(Command::Cp437Data(Window(Origin(0, 0), Size(3, 2)), Vec::from("abcdef"))).unwrap();

    loop {
        for x_offset in 0..usize::MAX {
            let mut pixels = PixelGrid::new(PIXEL_WIDTH as usize, PIXEL_HEIGHT as usize);
            for y in 0..PIXEL_HEIGHT as usize {
                for x_add in 0..=y % 8 {
                    pixels.set((y + x_offset + x_add) % PIXEL_WIDTH as usize, y, true);
                }
            }

            let window = Window(Origin(0, 0), Size(TILE_WIDTH, PIXEL_HEIGHT));
            connection.send(Command::BitmapLinearWin(window, pixels)).unwrap();

            thread::sleep(Duration::from_millis(100));
        }
    }
}