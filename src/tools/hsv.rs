use crate::tools::frame::Frame;
use std::cmp::{max, min};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn hsv_from_delta(f1: Frame, f2: Frame) -> HsvFrame {
    let f1_w = f1.width();
    let f2_w = f2.width();
    let f1_h = f1.height();
    let f2_h = f2.height();

    let f1_hsv = HsvFrame::new(f1);
    let f2_hsv = HsvFrame::new(f2);

    let len = (f1_w * f1_h * 4) as usize;
    let mut hsva_pixels = vec![
        HSVA {
            h: 0.0,
            s: 0.0,
            v: 0.0,
            a: 0.0
        };
        len
    ];

    if f1_w == f2_w && f1_h == f2_h {
        for i in 0..f1_w * f1_h {
            let i = i as usize;

            let mut h = 0.0;
            let mut s = 0.0;
            let mut v = 0.0;
            let mut a = 0.0;

            // both points in intersection, both alphas != 0
            if f1_hsv.pixels[i].a == f2_hsv.pixels[i].a && f1_hsv.pixels[i].a == 1.0 {
                h = f1_hsv.pixels[i].h - f2_hsv.pixels[i].h;
                s = f1_hsv.pixels[i].s - f2_hsv.pixels[i].s;
                v = f1_hsv.pixels[i].v - f2_hsv.pixels[i].v;

                h = if h < 0.0 { 0.0 } else { h };
                s = if s < 0.0 { 0.0 } else { s };
                v = if v < 0.0 { 0.0 } else { v };
                a = 1.0;
            }

            hsva_pixels[i].h = h;
            hsva_pixels[i].s = s;
            hsva_pixels[i].v = v;
            hsva_pixels[i].a = a;
        }
    }

    HsvFrame {
        pixels: hsva_pixels,
        width: f1_w,
        height: f1_h
    }
}

// h, s, l and a in [0..1]
#[derive(Copy, Clone)]
pub struct HSVA {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

#[wasm_bindgen]
pub struct HsvFrame {
    pixels: Vec<HSVA>,
    width: u32,
    height: u32
}

#[wasm_bindgen]
impl HsvFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(rgba: Frame) -> Self {
        rgba_2_hsva(rgba.pixels(), rgba.width(), rgba.height())
    }

    #[wasm_bindgen(method)]
    pub fn draw(&mut self, ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let mut pxs = hsva_2_rgba(self.pixels.to_vec(), self.width, self.height);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut pxs),
            self.width,
            self.height,
        )?;
        ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        ctx.put_image_data(&data, 0.0, 0.0)
    }

    #[wasm_bindgen(method)] // dumps RGBA pixels
    pub fn dump_pixels(&self, n: usize) -> Vec<u8> {
        let bright_component = self
            .pixels
            .iter()
            .map(|x| HSVA {
                h: if n & 1 == 0 { 0.0 } else { x.h },
                s: if n >> 1 & 1 == 0 { 0.0 } else { x.s },
                v: if n >> 2 & 1 == 0 { 0.0 } else { x.v },
                a: x.a,
            })
            .collect::<Vec<HSVA>>();

        hsva_2_rgba(bright_component, self.width, self.height)
    }

    #[wasm_bindgen(method)]
    pub fn convolute(&mut self, c: u32) {
        let w = self.width as usize;
        let h = self.height as usize;

        let mut convpixels = vec![
            HSVA {
                h: 0.0,
                s: 0.0,
                v: 0.0,
                a: 0.0
            }; w * h * 4];

        let kernel = vec![1.0, -2.0, 1.0, -2.0, 4.0, -2.0, 1.0, -2.0, 1.0];

        for y in 0..h {
            for x in 0..w {
                // indices
                let n = y * w + x;

                if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                    convpixels[n] = self.pixels[n];
                } else {
                    let row1 = (y - 1) * w + (x - 1);
                    let row2 = row1 +  w;
                    let row3 = row2 + w;

                    let mat = vec![
                        self.pixels[row1],
                        self.pixels[row1 + 1],
                        self.pixels[row1 + 2],
                        self.pixels[row2],
                        self.pixels[row2 + 1],
                        self.pixels[row2 + 2],
                        self.pixels[row3],
                        self.pixels[row3 + 1],
                        self.pixels[row3 + 2],
                    ];

                    let mut ph = 0.0;
                    let mut ps = 0.0;
                    let mut pv = 0.0;

                    for i in 0..9 {
                        let i = i as usize;
                        ph += kernel[i] * mat[i].h;
                        ps += kernel[i] * mat[i].s;
                        pv += kernel[i] * mat[i].v;
                    }

                    // warning: truncation
                    let p_h = ph.abs();
                    let p_s = ps.abs();
                    let p_v = pv.abs();

                    convpixels[n].h = if c & 1 == 0 { 0.0 } else { p_h };
                    convpixels[n].s = if c >> 1 & 1 == 0 { 0.0 } else { p_s };
                    convpixels[n].v = if c >> 2 & 1 == 0 { 0.0 } else { p_v };
                    convpixels[n].a = self.pixels[n].a;
                }
            }
        }

        self.pixels = convpixels;
    }
}

pub fn rgba_2_hsva(pixels: Vec<u8>, w: u32, h: u32) -> HsvFrame {
    let len = (w * h) as usize;
    let mut hsva_pixels = vec![
        HSVA {
            h: 0.0,
            s: 0.0,
            v: 0.0,
            a: 0.0
        };
        len
    ];

    for i in 0..len {
        let i4 = i * 4;
        let r = pixels[i4] as f32 / 255.0;
        let g = pixels[i4 + 1] as f32 / 255.0;
        let b = pixels[i4 + 2] as f32 / 255.0;
        let a = pixels[i4 + 3] as f32 / 255.0;

        let x_max = max(max(pixels[i4], pixels[i4 + 1]), pixels[i4 + 2]) as f32 / 255.0;
        let x_min = min(min(pixels[i4], pixels[i4 + 1]), pixels[i4 + 2]) as f32 / 255.0;

        let delta = x_max - x_min;
        let mut h = 0.0;
        let s = if x_max == 0.0 { 0.0 } else { delta / x_max };
        let v = x_max;

        if x_min != x_max {
            h = if x_max == r {
                if g < b {
                    6.0 + (g - b) / delta
                } else {
                    (g - b) / delta
                }
            } else if x_max == g {
                2.0 + (b - r) / delta
            } else {
                4.0 + (r - g) / delta
            };

            h /= 6.0;
        };

        hsva_pixels[i].h = h;
        hsva_pixels[i].s = s;
        hsva_pixels[i].v = v;
        hsva_pixels[i].a = a;
    }

    HsvFrame {
        pixels: hsva_pixels,
        width: w,
        height: h
    }
}

pub fn hsva_2_rgba(pixels: Vec<HSVA>, w: u32, h: u32) -> Vec<u8> {
    let len = (w * h) as usize;
    let mut rgba_pixels = vec![0; len * 4];

    for i in 0..len {
        let i4 = i * 4;

        let j = (pixels[i].h * 6.0).floor();
        let f = pixels[i].h * 6.0 - j;
        let p = pixels[i].v * (1.0 - pixels[i].s);
        let q = pixels[i].v * (1.0 - f * pixels[i].s);
        let t = pixels[i].v * (1.0 - (1.0 - f) * pixels[i].s);

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        let jj = j as i32;

        match jj % 6 {
            0 => {
                r = pixels[i].v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = pixels[i].v;
                b = p;
            }
            2 => {
                r = p;
                g = pixels[i].v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = pixels[i].v;
            }
            4 => {
                r = t;
                g = p;
                b = pixels[i].v;
            }
            5 => {
                r = pixels[i].v;
                g = p;
                b = q;
            }
            _ => (),
        }

        rgba_pixels[i4] = (r * 255.0) as u8;
        rgba_pixels[i4 + 1] = (g * 255.0) as u8;
        rgba_pixels[i4 + 2] = (b * 255.0) as u8;
        rgba_pixels[i4 + 3] = (255.0 * pixels[i].a) as u8;
    }

    rgba_pixels
}
