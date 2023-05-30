mod filter;

use std::sync::Mutex;
use filter::BandPass;

lazy_static! {
    static ref SAMPLE: Mutex<i32> = Mutex::new(0);
    static ref FILTERS: Mutex<Vec<BandPass>> = Mutex::new(Vec::new());
}

fn next_sample() -> i32 {
    let next = (*SAMPLE.lock().unwrap() + 1) % 48000;
    *SAMPLE.lock().unwrap() = next;
    
    next
}

pub(super) fn write(data: &mut[f32], _: &cpal::OutputCallbackInfo) {
    for frame in data.chunks_mut(2) {
        let value = (next_sample() as f32 / 48000.0 * 440.0 * 2.0 * std::f32::consts::PI).sin();

        for sample in frame {
            *sample = value * 0.1;
        }
    }
}