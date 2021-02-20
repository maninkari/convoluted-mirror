use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn frame_from_imgdata(imgdata: ImageData) -> Frame {
    Frame {
        pixels: imgdata.data().to_vec(),
        width: imgdata.width(),
        height: imgdata.height(),
    }
}

#[wasm_bindgen]
pub fn frame_from_delta(f1: Frame, f2: Frame) -> Frame {
    if f1.width == f2.width && f1.height == f2.height && f1.pixels.len() == f2.pixels.len() {
        let mut pxs = vec![0; f1.pixels.len() as usize];

        for i in 0..f1.width * f1.height {
            let i = i as usize;
            let i4 = i * 4;

            if f1.pixels[i4 + 3] == f2.pixels[i4 + 3] && f1.pixels[i4 + 3] == 255 {
                let p_r = f1.pixels[i4] - f2.pixels[i4];
                let p_g = f1.pixels[i4 + 1] - f2.pixels[i4 + 1];
                let p_b = f1.pixels[i4 + 2] - f2.pixels[i4 + 2];

                pxs[i4] = if p_r > 250 || p_r < 150 { 0 } else { p_r };
                pxs[i4 + 1] = if p_g > 250 || p_g < 150 { 0 } else { p_g };
                pxs[i4 + 2] = if p_b > 250 || p_b < 150 { 0 } else { p_b };
                pxs[i4 + 3] = 255;
            } else {
                pxs[i4] = 0;
                pxs[i4 + 1] = 0;
                pxs[i4 + 2] = 0;
                pxs[i4 + 3] = 0;
            }
        }

        Frame {
            pixels: pxs,
            width: f1.width,
            height: f1.height,
        }
    } else {
        Frame::new(f1.width, f1.height)
    }
}

// returns black frame
pub fn alpha_on(i: usize) -> u8 {
    if (i + 1) % 4 == 0 {
        255
    } else {
        0
    }
}

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
    pub fn new(w: u32, h: u32) -> Self {
        let pxs = vec![255; (w * h * 4) as usize];

        let trans = pxs
            .iter()
            .enumerate()
            .map(|(i, _x): (usize, &u8)| alpha_on(i))
            .collect::<Vec<u8>>();

        Frame {
            pixels: trans,
            width: w,
            height: h,
        }
    }

    pub fn convolute(&mut self, c: u32) {
        let w = self.width as usize;
        let h = self.height as usize;

        let mut convpixels = vec![0; w * h * 4];
        let kernel = vec![1, -2, 1, -2, 4, -2, 1, -2, 1];

        for y in 0..h {
            for x in 0..w {
                let red = 4 * (y * w + x);
                let green = red + 1;
                let blue = green + 1;
                let alpha = blue + 1;

                if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                    convpixels[red] = self.pixels[red];
                    convpixels[green] = self.pixels[green];
                    convpixels[blue] = self.pixels[blue];
                    convpixels[alpha] = self.pixels[alpha];
                } else {
                    let mut row1 = 4 * ((y - 1) * w + (x - 1));
                    let mut row2 = row1 + 4 * w;
                    let mut row3 = row2 + 4 * w;

                    let m_r = vec![
                        self.pixels[row1],
                        self.pixels[row1 + 4],
                        self.pixels[row1 + 8],
                        self.pixels[row2],
                        self.pixels[row2 + 4],
                        self.pixels[row2 + 8],
                        self.pixels[row3],
                        self.pixels[row3 + 4],
                        self.pixels[row3 + 8],
                    ];

                    row1 += 1;
                    row2 += 1;
                    row3 += 1;

                    let m_g = vec![
                        self.pixels[row1],
                        self.pixels[row1 + 4],
                        self.pixels[row1 + 8],
                        self.pixels[row2],
                        self.pixels[row2 + 4],
                        self.pixels[row2 + 8],
                        self.pixels[row3],
                        self.pixels[row3 + 4],
                        self.pixels[row3 + 8],
                    ];

                    row1 += 1;
                    row2 += 1;
                    row3 += 1;

                    let m_b = vec![
                        self.pixels[row1],
                        self.pixels[row1 + 4],
                        self.pixels[row1 + 8],
                        self.pixels[row2],
                        self.pixels[row2 + 4],
                        self.pixels[row2 + 8],
                        self.pixels[row3],
                        self.pixels[row3 + 4],
                        self.pixels[row3 + 8],
                    ];

                    let mut pr = 0;
                    let mut pg = 0;
                    let mut pb = 0;

                    for i in 0..9 {
                        let i = i as usize;
                        pr += (kernel[i] as i32) * (m_r[i] as i32);
                        pg += (kernel[i] as i32) * (m_g[i] as i32);
                        pb += (kernel[i] as i32) * (m_b[i] as i32);
                    }

                    // warning: truncation
                    let p_r = pr.abs() as u8;
                    let p_g = pg.abs() as u8;
                    let p_b = pb.abs() as u8;

                    convpixels[red] = if c & 1 == 0 { 0 } else { p_r };
                    convpixels[green] = if c >> 1 & 1 == 0 { 0 } else { p_g };
                    convpixels[blue] = if c >> 2 & 1 == 0 { 0 } else { p_b };
                    convpixels[alpha] = self.pixels[alpha];
                }
            }
        }

        self.pixels = convpixels;
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

    #[wasm_bindgen(method)]
    pub fn dump_pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}
