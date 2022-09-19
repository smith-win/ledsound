//!  Handles the drawing


use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle},
};

use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent};

pub struct MatrixDisplay {}

impl MatrixDisplay {

    /// Create a new display 
    pub fn main() -> Result<(), core::convert::Infallible> {
        
        // create new display with same size in pixels as my LED display
        let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(32, 16));
    
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let mut heights:[i32;16] =  [16, 12, 14, 6, 4, 10, 8, 10, 12, 8, 4, 2, 0, 8, 10, 2];

        // TODO: unwrap
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .scale(8)
            .pixel_spacing(2)
            .build();


        let mut window = Window::new("LED Matrix Sim", &output_settings);
        
        loop {

            // clear the display
            display.clear(BinaryColor::Off).unwrap();

            let mut x =0;
            for h in heights {
    
                Rectangle::new(Point::new(x, 16-h), Size::new(2, h as u32))
                    .into_styled(line_style)
                    .draw(&mut display)?;        
    
                x += 2;
            }

            window.update(&display);

            heights.rotate_left(1);

            std::thread::sleep(std::time::Duration::from_millis(150));
            if window.events().any(|e| e == SimulatorEvent::Quit) {
                break;
            }

        }
    
        Ok(())
    }


}