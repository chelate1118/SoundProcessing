mod sound;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device.");

    let mut supported_config_range = device.supported_output_configs().expect("error while querying configs");
    let supported_config = supported_config_range.next().expect("no supported config").with_max_sample_rate();

    dbg!(&supported_config);

    let stream = device.build_output_stream(
        &supported_config.into(),
        sound::write,
        move |err| eprintln!("an error occurred on the output audio stream: {err}"),
        None
    ).unwrap();

    stream.play().unwrap();

    println!("Hello, World!");

    std::thread::sleep(std::time::Duration::from_millis(2000));
}
