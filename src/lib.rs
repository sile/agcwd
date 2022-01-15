//! An implementation of the AGCWD algorithm.
//!
//! AGCWD is described in the paper ["Efficient Contrast Enhancement Using Adaptive Gamma Correction With Weighting Distribution"][AGCWD].
//!
//! [AGCWD]: https://ieeexplore.ieee.org/abstract/document/6336819/
//!
//! # Examples
//!
//! ```
//! // An example image containing 2 RGB pixels.
//! let mut pixels = vec![0, 1, 2, 3, 4, 5];
//!
//! let agcwd = agcwd::Agcwd::new(0.5);
//! agcwd.enhance_rgb_image(&mut pixels);
//! ```
#![warn(missing_docs)]

/// [`Agcwd`] provides methods to enhance image contrast based on the [AGCWD] algorithm.
///
/// [AGCWD]: https://ieeexplore.ieee.org/abstract/document/6336819/
#[derive(Debug)]
pub struct Agcwd {
    alpha: f32,
}

impl Agcwd {
    /// Makes a new [`Agcwd`] instance.
    ///
    /// `alpha` is an algorithm parameter to adjust the shape of weighting distribution (WD).
    pub fn new(alpha: f32) -> Self {
        Self { alpha }
    }

    /// Enhances the contrast of an RGB image.
    pub fn enhance_rgb_image(&self, pixels: &mut [u8]) {
        self.enhance_image::<3>(pixels);
    }

    /// Enhances the contrast of an RGBA image.
    pub fn enhance_rgba_image(&self, pixels: &mut [u8]) {
        self.enhance_image::<4>(pixels);
    }

    fn enhance_image<const N: usize>(&self, pixels: &mut [u8]) {
        let mut image = Image::<N>::new(pixels);
        let pdf = Pdf::new(&image);
        let pdf_w = pdf.to_weighting_distribution(self.alpha);
        let cdf_w = Cdf::new(&pdf_w);
        let curve = IntensityTransformationCurve::new(&cdf_w);
        image.update_pixels(|r, g, b| {
            let (h, s, v) = rgb_to_hsv(r, g, b);
            hsv_to_rgb(h, s, curve.0[usize::from(v)])
        });
    }
}

#[derive(Debug)]
struct IntensityTransformationCurve([u8; 256]);

impl IntensityTransformationCurve {
    fn new(cdf: &Cdf) -> Self {
        let mut curve = [0; 256];
        for (i, x) in cdf.0.iter().copied().enumerate() {
            curve[i] = (255.0 * (i as f32 / 255.0).powf(1.0 - x)).round() as u8;
        }
        Self(curve)
    }
}

#[derive(Debug)]
struct Image<'a, const N: usize> {
    pixels: &'a mut [u8],
    size: usize,
}

impl<'a, const N: usize> Image<'a, N> {
    fn new(pixels: &'a mut [u8]) -> Self {
        let size = pixels.len() / N;
        Self { pixels, size }
    }

    fn intensities(&self) -> impl '_ + Iterator<Item = u8> {
        self.pixels
            .chunks_exact(N)
            .map(|p| std::cmp::max(p[0], std::cmp::max(p[1], p[2])))
    }

    fn len(&self) -> usize {
        self.size
    }

    fn update_pixels<F>(&mut self, f: F)
    where
        F: Fn(u8, u8, u8) -> (u8, u8, u8),
    {
        for p in self.pixels.chunks_exact_mut(N) {
            let rgb = f(p[0], p[1], p[2]);
            p[0] = rgb.0;
            p[1] = rgb.1;
            p[2] = rgb.2;
        }
    }
}

#[derive(Debug, Clone)]
struct Pdf([f32; 256]);

impl Pdf {
    fn new<const N: usize>(image: &Image<'_, N>) -> Self {
        let mut histogram = [0; 256];
        for intensity in image.intensities() {
            histogram[usize::from(intensity)] += 1;
        }

        let mut pdf = [0.0; 256];
        let n = image.len() as f32;
        for (i, c) in histogram.into_iter().enumerate() {
            pdf[i] = c as f32 / n;
        }
        Self(pdf)
    }

    fn to_weighting_distribution(&self, alpha: f32) -> Self {
        let mut max_intensity = self.0[0];
        let mut min_intensity = self.0[0];
        for &x in &self.0[1..] {
            max_intensity = max_intensity.max(x);
            min_intensity = min_intensity.min(x);
        }

        let mut pdf_w = self.0;
        let range = max_intensity - min_intensity + f32::EPSILON;
        for x in &mut pdf_w {
            *x = max_intensity * ((*x - min_intensity) / range).powf(alpha);
        }
        Self(pdf_w)
    }
}

#[derive(Debug)]
struct Cdf([f32; 256]);

impl Cdf {
    fn new(pdf: &Pdf) -> Self {
        let mut cdf = [0.0; 256];
        let mut sum = 0.0;
        for (i, x) in pdf.0.iter().copied().enumerate() {
            sum += x;
            cdf[i] = sum;
        }
        for x in &mut cdf {
            *x /= sum;
        }
        Self(cdf)
    }
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = usize::from(r);
    let g = usize::from(g);
    let b = usize::from(b);
    let max = std::cmp::max(r, std::cmp::max(g, b));
    let min = std::cmp::min(r, std::cmp::min(g, b));
    let n = max - min;

    let s = if max == 0 { 0 } else { n * 255 / max };
    let v = max;
    let h = if n == 0 {
        0
    } else if max == r {
        if g < b {
            (6 * 255) + (g * 255 / n) - (b * 255 / n)
        } else {
            (g - b) * 255 / n
        }
    } else if max == g {
        2 * 255 + b * 255 / n - r * 255 / n
    } else {
        4 * 255 + r * 255 / n - g * 255 / n
    } / 6;

    (h as u8, s as u8, v as u8)
}

fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    if s == 0 {
        return (v, v, v);
    }

    let mut r = usize::from(v);
    let mut g = usize::from(v);
    let mut b = usize::from(v);
    let s = usize::from(s);
    let h6 = usize::from(h) * 6;

    let f = h6 % 255;
    match h6 / 255 {
        1 => {
            r = r * (255 * 255 - s * f) / (255 * 255);
            b = b * (255 - s) / 255;
        }
        2 => {
            r = r * (255 - s) / 255;
            b = b * (255 * 255 - s * (255 - f)) / (255 * 255);
        }
        3 => {
            r = r * (255 - s) / 255;
            g = g * (255 * 255 - s * f) / (255 * 255);
        }
        4 => {
            r = r * (255 * 255 - s * (255 - f)) / (255 * 255);
            g = g * (255 - s) / 255;
        }
        5 => {
            g = g * (255 - s) / 255;
            b = b * (255 * 255 - s * f) / (255 * 255);
        }
        n => {
            debug_assert!(n == 1 || n == 6);
            g = g * (255 * 255 - s * (255 - f)) / (255 * 255);
            b = b * (255 - s) / 255;
        }
    }

    (r as u8, g as u8, b as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enhance_rgb_image_works() {
        let mut pixels = [1, 2, 3, 4, 5, 6];
        let agcwd = Agcwd::new(0.5);
        agcwd.enhance_rgb_image(&mut pixels);
    }

    #[test]
    fn enhance_rgba_image_works() {
        let mut pixels = [1, 2, 3, 4, 5, 6, 7, 8];
        let agcwd = Agcwd::new(0.5);
        agcwd.enhance_rgba_image(&mut pixels);
    }
}
