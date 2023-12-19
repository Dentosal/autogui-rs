use core_graphics::display::CGDisplay;

use image;

pub fn all_screens() -> Vec<image::RgbaImage> {
    let display_ids = CGDisplay::active_displays().expect("Could not get list of active displays");
    let mut result = Vec::new();

    for display_id in display_ids {
        let display = CGDisplay::new(display_id);
        let img = display.image().unwrap();
        let w: u32 = img.width() as u32;
        let h: u32 = img.height() as u32;

        let data: Vec<u8> = img
            .data()
            .bytes()
            .chunks(4)
            .flat_map(|chunk| {
                // reorder color components
                if let &[b, g, r, a] = chunk {
                    vec![r, g, b, a]
                } else {
                    unreachable!()
                }
            })
            .collect();

        result.push(image::ImageBuffer::from_raw(w, h, data).unwrap());
    }

    result
}
