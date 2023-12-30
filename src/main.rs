mod visualiser;

use clap::Parser;
use cpal::traits::StreamTrait;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 350)]
    width: u32,

    #[arg(long, default_value_t = 200)]
    height: u32,

    #[arg(long, default_value_t = 150)]
    wave_height: u32,
}

fn main() {
    let args = Args::parse();

    let mut visualiser = visualiser::Visualiser::new(args.width, args.height, args.wave_height);

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
                visualiser.draw(&data.to_vec());
                visualiser.draw_to_terminal();
                visualiser.clear();
            },
            move |_err| {
                // errors
            },
            None, // None=blocking, Some(Duration)=timeout
        )
        .unwrap();

    loop {
        let _ = stream.play();
    }
}
