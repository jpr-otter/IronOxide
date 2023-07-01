use std::io;
use std::fs;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Enter the lower bound of the range: ");
    let mut lower_input = String::new();
    io::stdin().read_line(&mut lower_input).unwrap();
    let lower: u64 = lower_input.trim().parse().unwrap();

    println!("Enter the upper bound of the range: ");
    let mut upper_input = String::new();
    io::stdin().read_line(&mut upper_input).unwrap();
    let upper: u64 = upper_input.trim().parse().unwrap();

    let max_factors = Arc::new(Mutex::new(0));
    let max_number = Arc::new(Mutex::new(0));
    let max_number_factors = Arc::new(Mutex::new(HashSet::new()));
    
    let num_threads = 4;
    let chunk_size = (upper - lower + 1) / num_threads;
    
    let mut handles = Vec::new();
    
    for i in 0..num_threads {
        let max_factors = Arc::clone(&max_factors);
        let max_number = Arc::clone(&max_number);
        let max_number_factors = Arc::clone(&max_number_factors);
        
        let start = lower + i * chunk_size;
        let end = if i == num_threads - 1 { upper } else { start + chunk_size - 1 };
        
        let handle = thread::spawn(move || {
            for n in start..=end {
                let factors = prime_factors(n);
                let unique_factors: HashSet<u64> = factors.into_iter().collect();
                if unique_factors.len() > *max_factors.lock().unwrap() {
                    *max_factors.lock().unwrap() = unique_factors.len();
                    *max_number.lock().unwrap() = n;
                    *max_number_factors.lock().unwrap() = unique_factors;
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("The number with the most unique prime factors in the range {} to {} is {} with {} unique prime factors.", lower, upper, *max_number.lock().unwrap(), *max_factors.lock().unwrap());
    println!("The unique prime factors of {} are: {:?}", *max_number.lock().unwrap(), *max_number_factors.lock().unwrap());
    
    fs::write(r"C:\prime_factors.txt", format!("{:?}", *max_number_factors.lock().unwrap())).expect("Unable to write file");
    
    println!("\nPress enter to exit.");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
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
    
    factors
