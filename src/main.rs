use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("wind", "win.wav");

    audio.play("startup");

    // CLEANUP
    audio.wait();
    Ok(())
}
