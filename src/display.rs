//!  Handles the drawing


use embedded_graphics::{
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle},
    pixelcolor::Rgb888
};

//use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent};
use clap::{arg, crate_version, App};
use rpi_led_matrix::{args, LedColor, LedMatrix};

pub struct MatrixDisplay {}

impl MatrixDisplay {

    /// Create a new display 
    pub fn main() {
        

        let app = args::add_matrix_args(
            App::new("C++ Library Example")
                .about("shows basic usage of matrix arguments")
                .version(crate_version!()
            )
        );

	let matches = app.get_matches();
	let (options, rt_options) = args::matrix_options_from_args(&matches);

	let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
        let mut display = matrix.offscreen_canvas();

        // create new display with same size in pixels as my LED display
        //let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(32, 16));

//   let color = LedColor {
//        red: 255,
//        green: 255,
//        blue: 255,
//    };
	let color = Rgb888::new(10, 10, 80);
    
//        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
	let line_style = PrimitiveStyle::with_stroke(color, 1);

        let mut heights:[i32;16] =  [16, 12, 14, 6, 4, 10, 8, 10, 12, 8, 4, 2, 0, 8, 10, 2];
      
        loop {

            // clear the display
            display.clear();

            let mut x =0;
            for h in heights {
    
                Rectangle::new(Point::new(x, 16-h), Size::new(2, h as u32))
                    .into_styled(line_style)
                    .draw(&mut display);        
    
                x += 2;
            }

            //window.update(&display);
            display = matrix.swap(display); // is called canvas

            heights.rotate_left(1);

            std::thread::sleep(std::time::Duration::from_millis(150));
        }
    }


}
