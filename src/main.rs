#[macro_use]
extern crate clap;

mod display;
mod pcm;


pub fn main() {

    crate::display::MatrixDisplay::main();

}
