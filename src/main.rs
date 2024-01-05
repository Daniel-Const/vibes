mod term;
mod vis;

use std::{thread, time::Duration};

use clap::Parser;
use cpal::traits::StreamTrait;
use vis::{simple::Simple, visualiser::Visualiser};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::term::TermState;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = '=')]
    draw_char: char,

    #[arg(long, default_value_t = String::from("green"))]
    color: String,

    #[arg(long, default_value_t = true)]
    fill: bool,
}

fn main() {
    let args = Args::parse();
    let visualiser = Simple::new(args.fill);
    let mut terminal = term::Terminal::new(args.draw_char);

    let _ = enable_raw_mode();

    let handle_audio_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        terminal.clear();
        terminal.draw_from_coords(visualiser.map_coords(data, terminal.rows, terminal.cols));
        terminal.flush();
        thread::sleep(Duration::from_millis(10));
    };

    // Setup audio stream
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
            handle_audio_data,
            move |_err| {
                // errors
            },
            None,
        )
        .unwrap();

    let _ = stream.play();

    loop {
        match term::handle_events() {
            Ok(TermState::Quit) => {
                break;
            }
            _ => (),
        }
    }

    let _ = disable_raw_mode();
}
