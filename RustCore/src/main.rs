mod sound;
mod kotlin;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    println!("Start");
    sound::audio_play();

//    kotlin::start_connection().await.unwrap();
}
