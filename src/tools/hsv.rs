use crate::tools::frame::Frame;
use std::cmp::{max, min};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn hsv_from_delta(f1: Frame, f2: Frame) -> HsvFrame {
    let f1_width = f1.width();
    let f2_width = f2.width();
    let f1_height = f1.height();
    let f2_height = f2.height();

    let f1_hsv = HsvFrame::new(f1);
    let f2_hsv = HsvFrame::new(f2);

    let len = (f1_width * f1_height * 4) as usize;
    let mut hsva_pixels = vec![
        HSVA {
            h: 0.0,
            s: 0.0,
            v: 0.0,
            a: 0.0
        };
        len
    ];

    if f1_width == f2_width && f1_height == f2_height {
        for i in 0..f1_width * f1_height {
            let i = i as usize;

            let mut hue = 0.0;
            let mut saturation = 0.0;
            let mut value = 0.0;
            let mut alpha = 0.0;

            // both points in intersection, both alphas != 0
            if (f1_hsv.pixels[i].a - f2_hsv.pixels[i].a).abs() < f32::EPSILON
                && (f1_hsv.pixels[i].a - 1.0).abs() < f32::EPSILON
            {
                hue = f1_hsv.pixels[i].h - f2_hsv.pixels[i].h;
                saturation = f1_hsv.pixels[i].s - f2_hsv.pixels[i].s;
                value = f1_hsv.pixels[i].v - f2_hsv.pixels[i].v;
                alpha = 1.0;
            }

            hsva_pixels[i].h = hue.abs();
            hsva_pixels[i].s = saturation.abs();
            hsva_pixels[i].v = value.abs();
            hsva_pixels[i].a = alpha;
        }
    }

    HsvFrame {
        pixels: hsva_pixels,
        width: f1_width,
        height: f1_height,
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
    height: u32,
}

#[wasm_bindgen]
impl HsvFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(rgba: Frame) -> Self {
        rgba_2_hsva(rgba.pixels(), rgba.width(), rgba.height())
    }

    #[wasm_bindgen(method)]
    pub fn draw(&mut self, ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let pxs = hsva_2_rgba(self.pixels.to_vec(), self.width, self.height);

        let data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pxs), self.width, self.height)?;
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
        let width = self.width as usize;
        let height = self.height as usize;

        let mut convpixels = vec![
            HSVA {
                h: 0.0,
                s: 0.0,
                v: 0.0,
                a: 0.0
            };
            width * height * 4
        ];

        // let kernel = vec![1.0, -2.0, 1.0, -2.0, 4.0, -2.0, 1.0, -2.0, 1.0];
        let kernel = vec![-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];

        for y in 0..height {
            for x in 0..width {
                // indices
                let n = y * width + x;

                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    convpixels[n] = self.pixels[n];
                } else {
                    let row1 = (y - 1) * width + (x - 1);
                    let row2 = row1 + width;
                    let row3 = row2 + width;

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

pub fn rgba_2_hsva(pixels: Vec<u8>, width: u32, height: u32) -> HsvFrame {
    let len = (width * height) as usize;
    let mut hsva_pixels = vec![
        HSVA {
            h: 0.0,
            s: 0.0,
            v: 0.0,
            a: 0.0
        };
        len
    ];

    for (i, hsva_pixel) in hsva_pixels.iter_mut().enumerate().take(len) {
        let i4 = i * 4;
        let red = pixels[i4] as f32 / 255.0;
        let green = pixels[i4 + 1] as f32 / 255.0;
        let blue = pixels[i4 + 2] as f32 / 255.0;
        let alpha = pixels[i4 + 3] as f32 / 255.0;

        let x_max = max(max(pixels[i4], pixels[i4 + 1]), pixels[i4 + 2]) as f32 / 255.0;
        let x_min = min(min(pixels[i4], pixels[i4 + 1]), pixels[i4 + 2]) as f32 / 255.0;

        let delta = x_max - x_min;
        let mut hue = 0.0;
        let saturation = if x_max == 0.0 { 0.0 } else { delta / x_max };
        let value = x_max;

        if (x_min - x_max).abs() > f32::EPSILON {
            hue = if (x_max - red).abs() < f32::EPSILON {
                if green < blue {
                    6.0 + (green - blue) / delta
                } else {
                    (green - blue) / delta
                }
            } else if (x_max - green).abs() < f32::EPSILON {
                2.0 + (blue - red) / delta
            } else {
                4.0 + (red - green) / delta
            };

            hue /= 6.0;
        };

        hsva_pixel.h = hue;
        hsva_pixel.s = saturation;
        hsva_pixel.v = value;
        hsva_pixel.a = alpha;
    }

    HsvFrame {
        pixels: hsva_pixels,
        width,
        height,
    }
}

fn hsva_2_rgba(hsva_pixels: Vec<HSVA>, width: u32, height: u32) -> Vec<u8> {
    let len = (width * height) as usize;
    let mut rgba_pixels = vec![0; len * 4];

    for (i, hsva_pixel) in hsva_pixels.iter().enumerate().take(len) {
        let i4 = i * 4;

        let j = (hsva_pixel.h * 6.0).floor();
        let f = hsva_pixel.h * 6.0 - j;
        let p = hsva_pixel.v * (1.0 - hsva_pixel.s);
        let q = hsva_pixel.v * (1.0 - f * hsva_pixel.s);
        let t = hsva_pixel.v * (1.0 - (1.0 - f) * hsva_pixel.s);

        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;

        let j = j as i32;

        match j % 6 {
            0 => {
                red = hsva_pixel.v;
                green = t;
                blue = p;
            }
            1 => {
                red = q;
                green = hsva_pixel.v;
                blue = p;
            }
            2 => {
                red = p;
                green = hsva_pixel.v;
                blue = t;
            }
            3 => {
                red = p;
                green = q;
                blue = hsva_pixel.v;
            }
            4 => {
                red = t;
                green = p;
                blue = hsva_pixel.v;
            }
            5 => {
                red = hsva_pixel.v;
                green = p;
                blue = q;
            }
            _ => (),
        }

        rgba_pixels[i4] = (red * 255.0) as u8;
        rgba_pixels[i4 + 1] = (green * 255.0) as u8;
        rgba_pixels[i4 + 2] = (blue * 255.0) as u8;
        rgba_pixels[i4 + 3] = (255.0 * hsva_pixel.a) as u8;
    }

    rgba_pixels
}
