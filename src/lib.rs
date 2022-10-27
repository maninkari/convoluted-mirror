use std::fmt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub mod frame;
pub use self::frame::Frame;

// let mut kernel = vec![1, -2, 1, -2, 4, -2, 1, -2, 1];

// let mut kernel = vec![
//     1, 1, 2, 1, 1,
//     1, 1, 4, 1, 1,
//     4, 4, 4, 4, 4,
//     1, 1, 4, 1, 1,
//     1, 1, 2, 1, 1,
// ];

// let mut kernel = vec![
//     1, 4, 6, 4, 1,
//     4, 16, 24, 16, 4,
//     6, 24, -476, 24, 6,
//     4, 16, 24, 16, 4,
//     1, 4, 6, 4, 1,
// ];

// let mut kernel = vec![
//     1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, -270, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1,
//     1, 1, 1, 1, 1, 1, 1,
// ];

// let mut kernel = vec![
// 4, 3, -11, -13, -11, 3, 4, 3, -16, 0, 16, 0, -16, 3, -11, 0, 0, 64, 0, 0, -11, -13,
// 16, 64, 63, 64, 16, -13, -11, 0, 0, 64, 0, 0, -11, 3, -16, 0, 16, 0, -16, 3, 4, 3,
// -11, -13, -11, 3, 4,
// ];

// let mut kernel = vec![
// 0, -3, 3, 6, 3, -3, 0,
// -3, 3, 1, 16, 1, 3, -3,
// 3, 1, -25, 44, -25, 1, 3,
// 6, 16, 44, 120, 44, 16, 6,
// 3, 1, -25, 44, -25, 1, 3,
// -3, 3, 1, 16, 1, 3, -3,
// 0, -3, 3, 6, 3, -3, 0
// ];

#[wasm_bindgen]
pub struct Mirror {
    f1: Frame,
    f2: Frame,
    delta: Frame,
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    kernel: Vec<i32>,
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
            kernel: vec![
                0, -3, 3, 6, 3, -3, 0, -3, 3, 1, 16, 1, 3, -3, 3, 1, -25, 44, -25, 1, 3, 6, 16, 44,
                120, 44, 16, 6, 3, 1, -25, 44, -25, 1, 3, -3, 3, 1, 16, 1, 3, -3, 0, -3, 3, 6, 3,
                -3, 0,
            ],
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
        self.delta.convolute(7, &self.kernel);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.delta.dump_pixels()),
            self.width,
            self.height,
        )?;
        renderctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        renderctx.put_image_data(&data, 0.0, 0.0)
    }

    pub fn convolute_kernel(
        &mut self,
        renderctx: CanvasRenderingContext2d,
        js_objects: &JsValue,
        c: u32,
    ) -> Result<(), JsValue> {
        let imd = self
            .context
            .get_image_data(0.0, 0.0, self.width as f64, self.height as f64)
            .unwrap();

        self.kernel = js_objects.into_serde().unwrap();
        self.f2 = self.f1.clone();
        self.f1 = frame::frame_from_imgdata(imd);
        self.delta = frame::frame_from_delta(self.f1.clone(), self.f2.clone());
        self.delta.convolute(c, &self.kernel);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.delta.dump_pixels()),
            self.width,
            self.height,
        )?;
        renderctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        renderctx.put_image_data(&data, 0.0, 0.0)
    }

    #[wasm_bindgen(method)]
    pub fn convolute_f1(&mut self, renderctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let imd = self
            .context
            .get_image_data(0.0, 0.0, self.width as f64, self.height as f64)
            .unwrap();

        self.f2 = self.f1.clone();
        self.f1 = frame::frame_from_imgdata(imd);
        // self.delta = frame::frame_from_delta(self.f1.clone(), self.f2.clone());
        self.f1.convolute(7, &self.kernel);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.f1.dump_pixels()),
            self.width,
            self.height,
        )?;
        renderctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        renderctx.put_image_data(&data, 0.0, 0.0)
    }

    #[wasm_bindgen(method)]
    pub fn convolute_kernel_f1(
        &mut self,
        renderctx: CanvasRenderingContext2d,
        js_objects: &JsValue,
        c: u32,
    ) -> Result<(), JsValue> {
        let imd = self
            .context
            .get_image_data(0.0, 0.0, self.width as f64, self.height as f64)
            .unwrap();

        self.kernel = js_objects.into_serde().unwrap();
        self.f2 = self.f1.clone();
        self.f1 = frame::frame_from_imgdata(imd);
        // self.delta = frame::frame_from_delta(self.f1.clone(), self.f2.clone());
        self.f1.convolute(c, &self.kernel);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.f1.dump_pixels()),
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
