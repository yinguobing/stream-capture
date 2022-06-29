use opencv::{highgui, prelude::*, videoio};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    highgui::named_window("window", highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); 
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 27 {
            break;
        }
    }
    Ok(())
}