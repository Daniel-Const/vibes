mod visualiser;

use cpal::traits::StreamTrait;

fn main() {
    // TODO: get width / height from cli args 
    let mut visualiser = visualiser::Visualiser::new(420, 300, 200, 200);

    let host = cpal::default_host();
    let device = host.default_input_device().expect("no output device available");
    use cpal::traits::{DeviceTrait, HostTrait};

    let mut supported_configs_range = device.supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let config = supported_config.into();
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            visualiser.draw(&data.to_vec());
            visualiser.draw_to_terminal();
            visualiser.clear();
        },
        move |_err| {
            // errors
        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();

    loop {
        let _ = stream.play();
    }
}
