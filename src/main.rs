mod term;
mod visualiser;

use clap::Parser;
use cpal::traits::StreamTrait;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = '|')]
    draw_char: char,

    #[arg(long, default_value_t = String::from("green"))]
    color: String,

    #[arg(long, default_value_t = false)]
    fill: bool,
}

fn main() {
    let args = Args::parse();
    let visualiser = visualiser::Visualiser::new(args.draw_char, args.fill);
    let mut term = term::Terminal::new();

    // TODO: Improve scaling based on terminal height
    let scale_factor: f32 = 4.0; 

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no output device available");
    use cpal::traits::{DeviceTrait, HostTrait};

    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let config = supported_config.into();
    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                term.clear();
                visualiser.simple(data, &mut term, scale_factor);
                term.flush();
            },
            move |_err| {
                // errors
            },
            None,
        )
        .unwrap();

    let _ = stream.play();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
