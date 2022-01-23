use wasm_bindgen::prelude::*;

#[derive(Debug, serde::Deserialize)]
struct Options {
    #[serde(default)]
    alpha: f32,

    #[serde(default)]
    fusion: bool,
}

#[wasm_bindgen]
pub fn enhance_rgba_image(pixels: &mut [u8], options: &JsValue) -> Result<(), JsError> {
    assert!(options.is_object());
    let options: Options = options.into_serde()?;
    let options = agcwd::AgcwdOptions {
        alpha: options.alpha,
        fusion: options.fusion,
    };
    agcwd::Agcwd::with_options(options).enhance_rgba_image(pixels);
    Ok(())
}
