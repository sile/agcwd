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

mod color_format;

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
        self.enhance_image(RgbImage::<3>::new(pixels));
    }

    /// Enhances the contrast of an RGBA image.
    pub fn enhance_rgba_image(&self, pixels: &mut [u8]) {
        self.enhance_image(RgbImage::<4>::new(pixels));
    }

    fn enhance_image(&self, mut image: impl Image) {
        let pdf = Pdf::new(&image);
        let pdf_w = pdf.to_weighting_distribution(self.alpha);
        let cdf_w = Cdf::new(&pdf_w);
        let curve = IntensityTransformationCurve::new(&cdf_w);
        image.update_intensities(&curve);
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

trait Image {
    fn len(&self) -> usize;
    fn intensity_histogram(&self) -> [usize; 256];
    fn update_intensities(&mut self, curve: &IntensityTransformationCurve);
}

#[derive(Debug)]
struct RgbImage<'a, const N: usize> {
    pixels: &'a mut [u8],
}

impl<'a, const N: usize> RgbImage<'a, N> {
    fn new(pixels: &'a mut [u8]) -> Self {
        Self { pixels }
    }

    fn intensities(&self) -> impl '_ + Iterator<Item = u8> {
        self.pixels
            .chunks_exact(N)
            .map(|p| std::cmp::max(p[0], std::cmp::max(p[1], p[2])))
    }
}

impl<'a, const N: usize> Image for RgbImage<'a, N> {
    fn len(&self) -> usize {
        self.pixels.len() / N
    }

    fn intensity_histogram(&self) -> [usize; 256] {
        let mut histogram = [0; 256];
        for intensity in self.intensities() {
            histogram[usize::from(intensity)] += 1;
        }
        histogram
    }

    fn update_intensities(&mut self, curve: &IntensityTransformationCurve) {
        for p in self.pixels.chunks_exact_mut(N) {
            let (h, s, v) = self::color_format::rgb_to_hsv(p[0], p[1], p[2]);
            let (r, g, b) = self::color_format::hsv_to_rgb(h, s, curve.0[usize::from(v)]);
            p[0] = r;
            p[1] = g;
            p[2] = b;
        }
    }
}

#[derive(Debug, Clone)]
struct Pdf([f32; 256]);

impl Pdf {
    fn new(image: &impl Image) -> Self {
        let histogram = image.intensity_histogram();
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
