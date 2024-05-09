use servicepoint2::{Connection, Origin, PIXEL_HEIGHT, PIXEL_WIDTH, PixelGrid, Size, TILE_WIDTH, Window};
use servicepoint2::Command::BitmapLinearWin;

fn main() {
    let connection = Connection::open("localhost:2342").unwrap();

    for x_offset in 0..usize::MAX {
        let mut pixels = PixelGrid::max_sized();

        for y in 0..PIXEL_HEIGHT as usize {
            pixels.set((y + x_offset) % PIXEL_WIDTH as usize, y, true);
        }

        let window = Window(Origin(0, 0), Size(TILE_WIDTH, PIXEL_HEIGHT));
        connection.send(BitmapLinearWin(window, pixels)).unwrap();
    }
}
