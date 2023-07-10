extern crate cpal;
extern crate rand;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;
use std::env;
use std::time::Duration;

const MAX_KEY: u32 = 0x1f; // Five bits set
const RANGE: f64 = 128.0;

pub struct PinkNoise {
    white_values: [f64; 5],
    key: u32,
    max_key: u32,
    range: f64,
}

impl PinkNoise {
    pub fn new() -> PinkNoise {
        PinkNoise {
            white_values: [0.0; 5],
            key: 0,
            max_key: MAX_KEY,
            range: RANGE,
        }
    }

    pub fn next_value(&mut self) -> f64 {
        let last_key = self.key;
        let mut sum = 0.0;

        self.key += 1;
        if self.key > self.max_key {
            self.key = 0;
        }

        let diff = self.key ^ last_key;

        for i in 0..5 {
            if diff & (1 << i) != 0 {
                self.white_values[i] = rand::thread_rng().gen_range(-self.range..self.range);
            }
            sum += self.white_values[i];
        }

        sum / 5.0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: pink_noise <duration>");
        std::process::exit(1);
    }
    let duration = args[1]
        .parse::<u64>()
        .expect("Failed to parse duration argument");

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let config = device.default_output_config().unwrap();
    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, sample_rate)
            },
            err_fn,
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(duration * 60));
}

fn write_data(output: &mut [f32], channels: usize, sample_rate: u32) {
    let mut pink_noise = PinkNoise::new();

    for frame in output.chunks_mut(channels) {
        let value = pink_noise.next_value() as f32;
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
