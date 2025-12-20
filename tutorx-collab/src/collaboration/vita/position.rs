struct Position3D {
    coords: (i32, i32, i32),
}

fn move_position(pos: &mut Position3D, x: i32, y: i32, z: i32) {
    pos.coords = (x, y, z);
}

fn main() {
    let mut position = Position3D {
        coords: (0, 0, 0),
    };

    move_position(&mut position, 1, 2, 3);

    println!(
        "Posisi sekarang: {}, {}, {}",
        position.coords.0,
        position.coords.1,
        position.coords.2
    );
}