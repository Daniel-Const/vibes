use cpal::traits::StreamTrait;
use textplots::{Chart, ColorPlot, Shape};
use rgb::RGB8;

fn main() {

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
            let mut points = vec![];
            let mut idx: f32 = 0.0;
            for &sample in data {
                points.push((idx, sample));
                idx += 1.0;
            }

            let mut chart = Chart::new(400, 300, 0.0, points.len() as f32);
            chart.linecolorplot(
                &Shape::Bars(points.as_slice()),
                RGB8 {
                    r: 0,
                    g: 255,
                    b: 0,
                }
            )
            .display();
        },
        move |_err| {
            // react to errors here.
        },
        None // None=blocking, Some(Duration)=timeout
    ).unwrap();

    loop {
        let _ = stream.play().unwrap();
    }
}
