#![cfg(all(feature = "symphonia-aac", feature = "symphonia-isomp4"))]
use std::io::BufReader;

use rodio::Sample;

#[test]
fn test_mp4a_encodings() {
    // mp4a codec downloaded from YouTube
    // "Monkeys Spinning Monkeys"
    // Kevin MacLeod (incompetech.com)
    // Licensed under Creative Commons: By Attribution 3.0
    // http://creativecommons.org/licenses/by/3.0/
    let file = std::fs::File::open("assets/monkeys.mp4a").unwrap();
    let mut decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
    assert!(!decoder.all(|x| x.is_zero())); // Assert not all zeros
}
