// extern crate vips_rs;

use libvips::{ops, VipsApp, VipsImage};
use rinf::debug_print;

use crate::messages;

pub async fn overlay_image_with_watermark() {
    use messages::vips_overlay::*;

    let mut overlay_input_data = OverlayInputData::get_dart_signal_receiver();

    while let Some(dart_signal) = overlay_input_data.recv().await {
        let input: OverlayInputData = dart_signal.message;

        debug_print!("My object is {input:?}");

        // Initialize VipsApp (necessary for using libvips)
        VipsApp::new("vips-rust", false).expect("Failed to initialize VipsApp");

        // Load the main image and overlay image from bytes
        let main_image =
            VipsImage::new_from_buffer(&input.input_image, "").expect("Failed to load main image");
        let mut overlay_image = VipsImage::new_from_buffer(&input.overlay_image, "")
            .expect("Failed to load overlay image");

        // Resize the overlay image to the specified width and height
        overlay_image = ops::resize(
            &overlay_image,
            input.overlay_width as f64 / input.overlay_height as f64,
        )
        .expect("Failed to resize overlay image");

        // Composite the overlay image onto the main image at the specified position
        let result_image: VipsImage = ops::insert(
            &main_image,
            &overlay_image,
            input.overlay_x,
            input.overlay_y,
        )
        .expect("Failed to overlay image");

        let result_bytes = result_image.image_write_to_memory();

        OverlayOutputData {
            output_image: result_bytes,
        }
        .send_signal_to_dart();

        //     .write_to_memory(".png", &mut result_bytes)
        //     .expect("Failed to write image to buffer");

        // Ok(result_bytes)

        // Export the final image to bytes
        // let result_bytes = result_image.image_write_to_buffer(".png");

        // match result_bytes {
        //     Err(_) => println!("error: {}", app.error_buffer().unwrap()),
        //     Ok(_) => Ok(result_bytes),
        // }

        // let data = overlay_image_with_watermark(message);
        // match data {
        //     Err(_) => println!("error: in overlay "),
        //     Ok(d) => OverlayOutputData { output_image: d }.send_signal_to_dart(),
        // }
    }
}
