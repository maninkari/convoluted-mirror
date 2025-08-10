use js_sys::{Object, Reflect};
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
    // Accumulation for windowed averaging
    acc_sums: Vec<u32>,
    acc_count: u32,
    prev_avg: Option<Frame>,
}

#[wasm_bindgen]
impl Mirror {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, w: u32, h: u32) -> Mirror {
        // Request a 2D context optimized for frequent readbacks (getImageData)
        let options = Object::new();
        Reflect::set(
            &options,
            &JsValue::from_str("willReadFrequently"),
            &JsValue::from_bool(true),
        )
        .expect("failed to set willReadFrequently on context options");

        let ctx = canvas
            .get_context_with_context_options("2d", &options)
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
            acc_sums: vec![0u32; (w * h * 4) as usize],
            acc_count: 0,
            prev_avg: None,
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

    #[wasm_bindgen(method)]
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

    /// Compares batches of `window_size` frames by averaging each batch and diffing averages
    #[wasm_bindgen(method)]
    pub fn convolute_kernel_windowed(
        &mut self,
        renderctx: CanvasRenderingContext2d,
        js_objects: &JsValue,
        c: u32,
        window_size: u32,
    ) -> Result<(), JsValue> {
        // Update kernel from JS input
        self.kernel = js_objects.into_serde().unwrap();

        // Read current frame pixels
        let imd = self
            .context
            .get_image_data(0.0, 0.0, self.width as f64, self.height as f64)
            .unwrap();

        let current_frame = frame::frame_from_imgdata(imd);

        // Accumulate sums
        for (i, v) in current_frame.dump_pixels().iter().enumerate() {
            self.acc_sums[i] += *v as u32;
        }
        self.acc_count += 1;

        // Not yet enough frames collected; do not update output
        if self.acc_count < window_size {
            return Ok(());
        }

        // Compute average frame of the current window
        let mut avg_pixels: Vec<u8> = vec![0u8; (self.width * self.height * 4) as usize];
        for (i, sum) in self.acc_sums.iter().enumerate() {
            avg_pixels[i] = (sum / window_size as u32) as u8;
        }
        // Build ImageData from average pixels then wrap into Frame via helper
        // Build Frame directly from owned pixel buffer to avoid JS typed array detachment
        let current_avg = frame::frame_from_pixels(avg_pixels, self.width, self.height);

        // Reset accumulator for the next window
        self.acc_sums.fill(0);
        self.acc_count = 0;

        // If we have a previous average, compute delta between averages
        if let Some(prev) = &self.prev_avg {
            let mut delta_avg = frame::frame_from_delta(current_avg.clone(), prev.clone());
            delta_avg.convolute(c, &self.kernel);

            let data = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&delta_avg.dump_pixels()),
                self.width,
                self.height,
            )?;
            renderctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
            renderctx.put_image_data(&data, 0.0, 0.0)?;
        } else {
            // First window: draw the average itself
            let data = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&current_avg.dump_pixels()),
                self.width,
                self.height,
            )?;
            renderctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
            renderctx.put_image_data(&data, 0.0, 0.0)?;
        }

        // Update previous average
        self.prev_avg = Some(current_avg);

        Ok(())
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
