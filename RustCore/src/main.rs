mod sound;
mod kotlin;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let audio_play = sound::play::AudioPlay::new();
    audio_play.start_stream();

    let kotlin = kotlin::Kotlin::new().await;
    kotlin.listen().await.unwrap();

    println!("Hello, World!");

    std::thread::sleep(std::time::Duration::from_millis(2000));
    sound::add_filter();
    println!("Filter!");
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
