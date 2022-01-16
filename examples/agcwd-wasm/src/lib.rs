#[wasm_bindgen::prelude::wasm_bindgen]
pub fn enhance_rgba_image(pixels: &mut [u8], agcwd_alpha: f32) {
    agcwd::Agcwd::new(agcwd_alpha).enhance_rgba_image(pixels);
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn enhance_i420_image(pixels: &mut [u8], width: usize, agcwd_alpha: f32) {
    agcwd::Agcwd::new(agcwd_alpha).enhance_i420_image(pixels, width);
}
