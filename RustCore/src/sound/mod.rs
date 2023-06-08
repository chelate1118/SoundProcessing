mod filter;

use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::sync::Mutex;
use filter::BandPass;

lazy_static! {
    static ref SAMPLE: Mutex<i32> = Mutex::new(0);
    static ref FILTERS: Mutex<Vec<BandPass>> = Mutex::new(Vec::new());
}

pub(crate) fn audio_play() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device.");

    let mut supported_config_range = device.supported_output_configs().expect("error while querying configs");
    let supported_config = supported_config_range.next().expect("no supported config").with_max_sample_rate();

    dbg!(&supported_config);

    let stream = device.build_output_stream(
        &supported_config.into(),
        write,
        move |err| eprintln!("an error occurred on the output audio stream: {err}"),
        None
    ).unwrap();

    stream.play().unwrap();

    println!("Audio Start");

    spin_sleep::sleep(std::time::Duration::from_millis(2000));
    add_filter();
    println!("Filter!");
    spin_sleep::sleep(std::time::Duration::from_secs(5));
}

pub(crate) fn next_sample() -> i32 {
    let next = (*SAMPLE.lock().unwrap() + 1) % 48000;
    *SAMPLE.lock().unwrap() = next;

    next
}

pub(super) fn add_filter() {
    FILTERS.lock().unwrap().push(
        BandPass::new(450.0, 1.0, 0.8)
    )
}

pub(super) fn write(
    data: &mut[f32], _: &cpal::OutputCallbackInfo
) {
    for frame in data.chunks_mut(2) {
        let sample = next_sample();
        let mut value = (sample as f32 / 48000. * 440. * 2. * std::f32::consts::PI).sin() * 0.1;
        for filter in FILTERS.lock().unwrap().iter_mut() {
            value = filter.next_value(value, 1.0 / 48000.0);
        }

        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}