use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Frame {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Frame {
    #[wasm_bindgen(constructor)]
    pub fn new(imgdata: ImageData) -> Self {
        Frame {
            pixels: imgdata.data().to_vec(),
            width: imgdata.width(),
            height: imgdata.height(),
        }
    }

    #[wasm_bindgen(method)]
    pub fn draw(&mut self, ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixels),
            self.width,
            self.height,
        )?;
        ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        ctx.put_image_data(&data, 0.0, 0.0)
    }
}
