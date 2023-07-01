use std::io;
use std::thread;

fn main() {
    loop {
        println!("Enter a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: u64 = input.trim().parse().unwrap();

        let handle = thread::spawn(move || {
            prime_factors(n);
        });

        handle.join().unwrap();

        println!("\nPress enter to restart or Ctrl+C to exit.");
        let mut restart = String::new();
        io::stdin().read_line(&mut restart).unwrap();
    }
}

fn prime_factors(mut n: u64) {
    let mut factors = Vec::new();
    println!("Here are all the prime factors of the number {} you chose:", n);
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        factors.push(n);
    }
    
    println!("{:?}", factors);
    
    print_factor_tree(factors, 0);
}

fn print_factor_tree(factors: Vec<u64>, level: usize) {
    if factors.len() == 1 {
        println!("{:>width$}", factors[0], width=level*4);
    } else if factors.len() == 2 {
        println!("{:>width$}", factors[0], width=level*4);
        println!("{:>width$}", factors[1], width=level*4);
    } else {
        let mid = factors.len() / 2;
        print_factor_tree(factors[..mid].to_vec(), level+1);
        println!("{:>width$}", "*", width=level*4);
        print_factor_tree(factors[mid..].to_vec(), level+1);
    }
}
