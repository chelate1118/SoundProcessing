pub(crate) mod filter;

use cpal::Sample;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use rodio::{Decoder, Source};
use std::sync::Mutex;
use filter::BandPass;
use std::io::BufReader;
use std::fs::File;


lazy_static! {
    static ref SAMPLE: Mutex<usize> = Mutex::new(0);
    static ref FILTERS: Mutex<Vec<BandPass>> = Mutex::new(Vec::new());
    static ref AUDIO_DATA: Mutex<Option<Vec<f32>>> = Mutex::new(None);
}

pub(crate) fn audio_play() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device.");

    let mut supported_config_range = device.supported_output_configs().expect("error while querying configs");
    let supported_config = supported_config_range.next().expect("no supported config").with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0;
    dbg!(&supported_config);

    let stream = device.build_output_stream(
        &supported_config.into(),
        move |data: &mut[f32], _: &cpal::OutputCallbackInfo| {
            write(data, sample_rate);
        },
        move |err| eprintln!("an error occurred on the output audio stream: {err}"),
        None
    ).unwrap();

    stream.play().unwrap();

    spin_sleep::sleep(std::time::Duration::from_secs(std::u64::MAX));
}

fn next_sample() -> usize {
    let next = *SAMPLE.lock().unwrap() + 1;
    *SAMPLE.lock().unwrap() = next;

    next
}

fn reset_sample() {
    *SAMPLE.lock().unwrap() = 0;
}

pub(crate) fn add_filter(filter: BandPass) {
    FILTERS.lock().unwrap().push(
        filter
    )
}

pub(crate) fn move_filter(index: usize, filter: BandPass) {
    FILTERS.lock().unwrap()[index].change_to(filter);
}

pub(crate) fn play_audio_file(path: &str) {
    reset_sample();

    let file = BufReader::new(File::open(path).unwrap());
    let decoder = Decoder::new(file).unwrap();

    let audio_data: Vec<f32> = decoder
        .convert_samples::<f32>()
        .map(|sample| sample as f32)
        .collect();

    *AUDIO_DATA.lock().unwrap() = Some(audio_data);
}

pub(super) fn write(
    data: &mut[f32], sample_rate: u32
) {
    for frame in data.chunks_mut(2) {
        let sample = next_sample();

        let mut value: f32 = Sample::EQUILIBRIUM;

        if let Some(data) = &*AUDIO_DATA.lock().unwrap() {
            if let Some(next) = data.get(sample) {
                value = *next;
            }
        }

        for filter in FILTERS.lock().unwrap().iter_mut() {
            value = filter.next_value(value, 1.0 / sample_rate as f32);
        }

        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}