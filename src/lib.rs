use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub mod frame;
pub use self::frame::Frame;

#[wasm_bindgen]
pub struct Mirror {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Mirror {
    #[wasm_bindgen(constructor)]
    pub fn new(canvasctx: CanvasRenderingContext2d, w: u32, h: u32) -> Mirror {
        Mirror {
            context: canvasctx,
            width: w,
            height: h,
        }
    }

    #[wasm_bindgen(method)]
    pub fn convolute(&mut self, renderctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let imgdata = self
            .context
            .get_image_data(0.0, 0.0, self.width as f64, self.height as f64)
            .unwrap();

        let mut frm = frame::Frame::new(imgdata);
        frm.draw(renderctx)
    }
}
