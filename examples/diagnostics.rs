// This example requires `rodio`'s "tracing" feature to be enabled.
#![cfg(feature = "tracing")]

//! `rodio` reports some non-fatal errors and warnings via
//! [`tracing`](https://github.com/tokio-rs/tracing/tree/v0.1.x) API.
//! Set up a `tracing` subscriber to access them.
//! There are many options of such subscribers already available, including compatibility
//! adapters for `log` create API. This example demonstrates one of the simplest versions
//! immediately writing event messages into standard output.

use rodio::{Decoder, Source};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use tracing::Level;
use tracing_subscriber;

fn main() -> Result<(), Box<dyn Error>> {
    // This installs global default `tracing` listener.
    tracing_subscriber::fmt::fmt()
        // AGC logs messages at trace level, configuring that to see them.
        .with_max_level(Level::TRACE)
        .init();

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    let file = BufReader::new(File::open("assets/music.flac")?);
    sink.append(Decoder::new(file)?.automatic_gain_control(0.1, 4.0, 0.005, 5.0));

    // AGC filter now should log current gain value into stdout.
    thread::sleep(Duration::from_micros(200));

    Ok(())
}
