pub(crate) mod sound;
pub(crate) mod kotlin;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    println!("Start");
    std::thread::spawn(sound::audio_play);
    kotlin::start_connection().await.unwrap();

//    spin_sleep::sleep(std::time::Duration::from_secs(std::u64::MAX));
}
