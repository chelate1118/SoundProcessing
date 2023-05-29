
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat, FromSample};

fn main() {
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
    
    println!("Hello, World!");
    
    std::thread::sleep(std::time::Duration::from_millis(5000));
}

fn write(data: &mut[f32], _: &cpal::OutputCallbackInfo) {    
    for (i, sample) in data.iter_mut().enumerate() {
//        *sample = (i as f32/100f32).sin() * 0.3;
        *sample = f32::EQUILIBRIUM;
    }
}