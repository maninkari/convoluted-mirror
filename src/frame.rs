use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};

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
        let min_delta = 150;

        for i in 0..f1.width * f1.height {
            let i = i as usize;
            let i4 = i * 4;

            if f1.pixels[i4 + 3] == f2.pixels[i4 + 3] && f1.pixels[i4 + 3] == 255 {
                let p_r = ((f1.pixels[i4] - f2.pixels[i4]) as f32).abs() as u8;
                let p_g = ((f1.pixels[i4 + 1] - f2.pixels[i4 + 1]) as f32).abs() as u8;
                let p_b = ((f1.pixels[i4 + 2] - f2.pixels[i4 + 2]) as f32).abs() as u8;

                pxs[i4] = if p_r > 250 || p_r < min_delta { 0 } else { p_r };
                pxs[i4 + 1] = if p_g > 250 || p_g < min_delta { 0 } else { p_g };
                pxs[i4 + 2] = if p_b > 250 || p_b < min_delta { 0 } else { p_b };
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
pub fn black_px(i: usize) -> u8 {
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

impl Frame {
    pub fn new(w: u32, h: u32) -> Self {
        let pxs = vec![255; (w * h * 4) as usize];
        console::log_1(&"constructor: ".into());

        let trans = pxs
            .iter()
            .enumerate()
            .map(|(i, _x): (usize, &u8)| black_px(i))
            .collect::<Vec<u8>>();

        Frame {
            pixels: trans,
            width: w,
            height: h,
        }
    }

    pub fn convolute(&mut self, c: u32, kernel: &Vec<i32>) {
        let w = self.width as usize;
        let h = self.height as usize;

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

        let mut convpixels = vec![0; w * h * 4];
        let kernel_sq = (kernel.len() as f32).sqrt() as usize;
        let kernel_delta = (kernel_sq as f32 / 2_f32) as usize;  
        
        // console::log_2(&"kernel delta: ".into(), &JsValue::from(kernel_delta));

        for y in 0..h {
            for x in 0..w {
                let red = 4 * (y * w + x);
                let green = red + 1;
                let blue = green + 1;
                let alpha = blue + 1;

                if x < kernel_delta || x >= w - kernel_delta || y < kernel_delta || y >= h - kernel_delta {
                    convpixels[red] = 0;
                    convpixels[green] = 0;
                    convpixels[blue] = 0;
                    convpixels[alpha] = self.pixels[alpha];
                } else {
                    let init = 4 * ((y - kernel_delta) * w + (x - kernel_delta));
                    
                    let pr = self.conv_pixel(init, 4*w, kernel, kernel_sq) as i32;
                    let pg = self.conv_pixel(init + 1, 4*w, kernel, kernel_sq) as i32;
                    let pb = self.conv_pixel(init + 2, 4*w, kernel, kernel_sq) as i32;

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
        };

        self.pixels = convpixels
    }

    fn conv_pixel(&mut self, init: usize, row_len: usize, kernel: &Vec<i32>, kernel_sq: usize ) -> u8 {
        let mut i = init;
        let mut ret = 0;

        //  kernel by surrounding pixels matrix 
        for j in 0..kernel_sq {
            for k in 0..kernel_sq {
                ret += self.pixels[i + 4*k] as i32 * kernel[k + j*kernel_sq];
            }

            i += row_len;
        };

        ret as u8
    }

    pub fn draw(&mut self, ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixels),
            self.width,
            self.height,
        )?;
        ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        ctx.put_image_data(&data, 0.0, 0.0)
    }

    pub fn dump_pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}
