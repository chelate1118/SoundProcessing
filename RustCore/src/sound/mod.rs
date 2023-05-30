mod filter;

use std::sync::Mutex;
use filter::BandPass;
use noise::{NoiseFn, Perlin};

lazy_static! {
    static ref SAMPLE: Mutex<i32> = Mutex::new(0);
    static ref FILTERS: Mutex<Vec<BandPass>> = Mutex::new(Vec::new());
    static ref PERLIN: Perlin = Perlin::new(1);
}

pub(super) fn add_filter() {
    FILTERS.lock().unwrap().push(
        BandPass::new(450.0, 1.0, 0.8)
    )
}

fn next_sample() -> i32 {
    let next = (*SAMPLE.lock().unwrap() + 1) % 48000;
    *SAMPLE.lock().unwrap() = next;
    
    next
}

pub(super) fn write(data: &mut[f32], _: &cpal::OutputCallbackInfo) {
    for frame in data.chunks_mut(2) {
        let sample = next_sample();
        let mut value = PERLIN.get([sample as f64 / 100.0, 1.0]) as f32;
        for filter in FILTERS.lock().unwrap().iter_mut() {
            value = filter.next_value(value, 1.0 / 48000.0);
        }

        for sample in frame {
            *sample = value * 0.1;
        }
    }
}