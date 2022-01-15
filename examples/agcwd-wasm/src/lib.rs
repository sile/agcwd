#[wasm_bindgen::prelude::wasm_bindgen]
pub fn enhance_rgba_image(pixels: &mut [u8], agcwd_alpha: f32) {
    agcwd::Agcwd::new(agcwd_alpha).enhance_rgba_image(pixels);
}
