use std::fmt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub mod frame;
pub use self::frame::Frame;

#[wasm_bindgen]
pub struct Mirror {
    f1: Frame,
    f2: Frame,
    delta: Frame,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Mirror {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, w: u32, h: u32) -> Mirror {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("failed to obtain 2d rendering context for target <canvas>");

        Mirror {
            context: ctx,
            f1: frame::Frame::new(w, h),
            f2: frame::Frame::new(w, h),
            delta: frame::Frame::new(w, h),
            width: w,
            height: h,
        }
    }

    #[wasm_bindgen(method)]
    pub fn convolute(&mut self, renderctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let imd = self
            .context
            .get_image_data(0.0, 0.0, self.width as f64, self.height as f64)
            .unwrap();

        self.f2 = self.f1.clone();
        self.f1 = frame::frame_from_imgdata(imd);
        self.delta = frame::frame_from_delta(self.f1.clone(), self.f2.clone());
        self.delta.convolute(7);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.delta.dump_pixels()),
            self.width,
            self.height,
        )?;
        renderctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        renderctx.put_image_data(&data, 0.0, 0.0)
    }

    #[wasm_bindgen(method)]
    pub fn talk(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mirror: I'm mirroring you..")
    }
}
