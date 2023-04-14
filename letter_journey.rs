use std::io::{self, Write};
use std::time::Duration;
use std::{thread, process};

fn main() {
    let square = [
        ['a', 'a', 'a', 'a'],
        ['z', ' ', ' ', 'z'],
        ['z', ' ', ' ', 'z'],
        ['a', 'a', 'a', 'a']
    ];
    let mut rotated_square = square;

    loop {
        print!("\x1B[2J\x1B[1;1H");
        for row in rotated_square.iter() {
            for ch in row.iter() {
                print!("{}", ch);
            }
            println!();
        }
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
        rotated_square = rotate_square(&rotated_square);
    }
}

fn rotate_square(square: &[[char; 4]; 4]) -> [[char; 4]; 4] {
    let mut rotated = [[square[0][0]; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            rotated[j][3 - i] = square[i][j];
        }
    }
    rotated
}
