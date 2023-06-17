use std::{thread, time};

fn main() {
    let letters = "HelloWorld!";
    let wave_length = 50;
    let delay = time::Duration::from_millis(200); 
    let num_iterations = 50; 

    for i in 0..num_iterations {
        let mut output = String::new();
        for (j, c) in letters.chars().enumerate() {
            let pos = (i + j) % wave_length;
            for _ in 0..pos {
                output.push(' ');
            }
            output.push(c);
            output.push('\n');
        }
        print!("{}", output);
        thread::sleep(delay);
        print!("\x1B[2J\x1B[1;1H");
    }
}
