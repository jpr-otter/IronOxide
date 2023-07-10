use std::io;

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    println!("Do you want to encode or decode? ([e]ncode/[d]ecode)");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    println!("Enter the text or number to be encoded/decoded:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if choice == "e" {
        let encoded = base64_encode(input.as_bytes());
        println!("Encoded: {}", encoded);
    } else if choice == "d" {
        let decoded = base64_decode(input);
        let decoded_str = String::from_utf8(decoded).unwrap();
        println!("Decoded: {}", decoded_str);
    } else {
        println!("Invalid choice");
    }
    println!("Press Enter to exit");
    let mut _exit = String::new();
    io::stdin().read_line(&mut _exit).unwrap();
}

fn base64_encode(input: &[u8]) -> String {
    let mut result = Vec::new();
    let mut temp = 0u32;
    for (i, &byte) in input.iter().enumerate() {
        temp = (temp << 8) + byte as u32;
        if i % 3 == 2 {
            result.push(BASE64_CHARS[((temp >> 18) & 0x3F) as usize]);
            result.push(BASE64_CHARS[((temp >> 12) & 0x3F) as usize]);
            result.push(BASE64_CHARS[((temp >> 6) & 0x3F) as usize]);
            result.push(BASE64_CHARS[(temp & 0x3F) as usize]);
            temp = 0;
        }
    }
    let rem = input.len() % 3;
    if rem > 0 {
        temp <<= (3 - rem) * 8;
        result.push(BASE64_CHARS[((temp >> 18) & 0x3F) as usize]);
        result.push(BASE64_CHARS[((temp >> 12) & 0x3F) as usize]);
        if rem == 2 {
            result.push(BASE64_CHARS[((temp >> 6) & 0x3F) as usize]);
        } else {
            result.push(b'=');
        }
        result.push(b'=');
    }
    String::from_utf8(result).unwrap()
}

fn base64_decode(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut temp = 0u32;
    for (i, c) in input.chars().filter(|&c| c != '=').enumerate() {
        let index = BASE64_CHARS.iter().position(|&ch| ch as char == c).unwrap() as u32;
        temp = (temp << 6) + index;
        if i % 4 == 3 {
            result.push((temp >> 16) as u8);
            result.push((temp >> 8) as u8);
            result.push(temp as u8);
            temp = 0;
        }
    }
    let rem = input.chars().filter(|&c| c != '=').count() % 4;
    if rem > 1 {
        temp <<= (4 - rem) * 6;
        result.push((temp >> 16) as u8);
        if rem == 3 {
            result.push((temp >> 8) as u8);
        }
    }
    result
}
