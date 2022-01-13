#[derive(Debug)]
pub struct Options {
    alpha: Option<f32>,
    th: f32, // frame-difference-threshold(?)
}

impl Default for Options {
    fn default() -> Self {
        Self {
            alpha: None,
            th: 0.05,
        }
    }
}

#[derive(Debug)]
pub struct Agcwd {
    options: Options,
    pdf: Pdf,
}

impl Agcwd {
    pub fn new() -> Self {
        Self {
            options: Options::default(),
            pdf: Pdf([0.0; 256]),
        }
    }

    pub fn enhance(&mut self, pixels: &mut [u8]) {
        let mut image = Image::new(pixels);
        let pdf = Pdf::new(&image);
        let pdf_w = pdf.to_weighting_distribution(self.options.alpha.unwrap_or(0.5));
        let cdf_w = Cdf::new(&pdf_w);
        let curve = IntensityTransformationCurve::new(&cdf_w);
        image.update_pixels(|r, g, b| {
            let (h, s, v) = rgb_to_hsv(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
            let v = curve.0[(v * 255.0) as usize];
            let rgb = hsv_to_rgb(h, s, v);
            (
                (rgb.0 * 255.0).round() as u8,
                (rgb.1 * 255.0).round() as u8,
                (rgb.2 * 255.0).round() as u8,
            )
        });
    }
}

#[derive(Debug)]
struct IntensityTransformationCurve([f32; 256]);

impl IntensityTransformationCurve {
    fn new(cdf: &Cdf) -> Self {
        let mut curve = [0.0; 256];
        for (i, x) in cdf.0.iter().copied().enumerate() {
            curve[i] = (i as f32 / 255.0).powf(1.0 - x);
        }
        Self(curve)
    }
}

#[derive(Debug)]
struct Image<'a> {
    pixels: &'a mut [u8],
    size: usize,
}

impl<'a> Image<'a> {
    fn new(pixels: &'a mut [u8]) -> Self {
        let size = pixels.len() / 3;
        Self { pixels, size }
    }

    fn intensities(&self) -> impl '_ + Iterator<Item = u8> {
        self.pixels
            .chunks_exact(3)
            .map(|p| std::cmp::max(p[0], std::cmp::max(p[1], p[2])))
    }

    fn len(&self) -> usize {
        self.size
    }

    fn update_pixels<F>(&mut self, f: F)
    where
        F: Fn(u8, u8, u8) -> (u8, u8, u8),
    {
        for p in self.pixels.chunks_exact_mut(3) {
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
    fn new(image: &Image) -> Self {
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

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let mut h = max - min;
    if h > 0.0 {
        if max == r {
            h = (g - b) / h;
            if h < 0.0 {
                h += 6.0;
            }
        } else if max == g {
            h = 2.0 + (b - r) / h;
        } else {
            h = 4.0 + (r - g) / h;
        }
    }
    h /= 6.0;
    let mut s = max - min;
    if max != 0.0 {
        s /= max;
    }
    let v = max;
    (h, s, v)
}

fn hsv_to_rgb(mut h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let mut r = v;
    let mut g = v;
    let mut b = v;

    if s > 0.0 {
        h *= 6.0;
        let f = h.fract();
        match h.floor() as usize {
            1 => {
                r *= 1.0 - s * f;
                b *= 1.0 - s;
            }
            2 => {
                r *= 1.0 - s;
                b *= 1.0 - s * (1.0 - f);
            }
            3 => {
                r *= 1.0 - s;
                g *= 1.0 - s * f;
            }
            4 => {
                r *= 1.0 - s * (1.0 - f);
                g *= 1.0 - s;
            }
            5 => {
                g *= 1.0 - s;
                b *= 1.0 - s * f;
            }
            // 0 | 6
            _ => {
                g *= 1.0 - s * (1.0 - f);
                b *= 1.0 - s;
            }
        }
    }

    (r, g, b)
}
