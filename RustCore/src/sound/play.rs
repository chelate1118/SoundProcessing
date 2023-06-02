#![allow(unused)]

use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use noise::NoiseFn;

use super::*;

pub(crate) struct AudioPlay {
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::StreamConfig
}

impl AudioPlay {
    pub(crate) fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device.");
        let mut supported_config_range = device.supported_output_configs().expect("error while querying configs");
        let supported_config = supported_config_range.next().expect("no supported config").with_max_sample_rate();
        
        AudioPlay {
            host,
            device,
            config: supported_config.into()
        }
    }
    
    pub(crate) fn start_stream(&self) {
        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut[f32], _: &cpal::OutputCallbackInfo| {
                write(data)
            },
            move |err| eprintln!("an error occurred on the output audio stream: {err}"),
            None
        ).unwrap();
        
        stream.play().unwrap();
    }
}