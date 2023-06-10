mod filter;

use cpal::Sample;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use rodio::Decoder;
use std::ops::DerefMut;
use std::sync::Mutex;
use filter::BandPass;
use std::io::BufReader;
use std::fs::File;


lazy_static! {
    static ref SAMPLE: Mutex<usize> = Mutex::new(0);
    static ref FILTERS: Mutex<Vec<BandPass>> = Mutex::new(Vec::new());
    static ref AUDIO: Mutex<Option<Decoder<BufReader<File>>>> = Mutex::new(None);
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

    println!("Audio Start");
    play_audio_file();

    spin_sleep::sleep(std::time::Duration::from_secs(4));

    println!("{}", next_sample());
    play_audio_file();
    add_filter();
    println!("Filter!");
    spin_sleep::sleep(std::time::Duration::from_secs(4));

    println!("{}", next_sample());
}

fn next_sample() -> usize {
    let next = *SAMPLE.lock().unwrap() + 1;
    *SAMPLE.lock().unwrap() = next;

    next
}

fn reset_sample() {
    *SAMPLE.lock().unwrap() = 0;
}

pub(super) fn add_filter() {
    FILTERS.lock().unwrap().push(
        BandPass::new(254.0, 1.0, 0.8)
    )
}

pub(crate) fn play_audio_file() {
    reset_sample();

    let file = BufReader::new(File::open("../sample_audio.mp3").unwrap());
    let source = Decoder::new(file).unwrap();

    *AUDIO.lock().unwrap() = Some(source)
}

pub(super) fn write(
    data: &mut[f32], sample_rate: u32
) {
    for frame in data.chunks_mut(2) {
        let sample = next_sample();

        let mut value: f32 = Sample::EQUILIBRIUM;

        if let Some(iter) = AUDIO.lock().unwrap().deref_mut() {
            if let Some(next) = iter.next() {
                value = next as f32 * 0.001;
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