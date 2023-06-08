mod sound;
mod kotlin;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    println!("Start");
    std::thread::spawn(sound::audio_play);
    spin_sleep::sleep(std::time::Duration::from_secs(7));

//    kotlin::start_connection().await.unwrap();
}
