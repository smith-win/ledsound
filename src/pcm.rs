//! Handles the listening to and evaluation of PCM (sound data)


// use hound::WavReader;
// use std::{
//     fs::{OpenOptions},
//     io::{Result, Error, Read, BufReader}
// };

// pub fn main () -> std::io::Result<()>{
//     // let mut reader = hound::WavReader::open().unwrap();

//     let file = OpenOptions::new().read(true).open("/tmp/stuart_fifo") ?;
//     let reader = BufReader::new(file);


//     let sqr_sum = WavReader::new(reader).unwrap().into_samples::<i16>()
//         .take(100)
//         .fold(0.0, |sqr_sum, s| {
//             let sample = s.unwrap() as f64;
//             sqr_sum + sample * sample
//     });
//     println!("RMS is {}", (sqr_sum as f64).sqrt());
//     Ok(())
// })