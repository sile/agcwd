use std::cmp::{max, min};

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
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

pub fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
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
            debug_assert!(n == 0 || n == 6, "n: {}", n);
            g = g * (255 * 255 - s * (255 - f)) / (255 * 255);
            b = b * (255 - s) / 255;
        }
    }

    (r as u8, g as u8, b as u8)
}

pub fn yuv_to_hsv(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let (r, g, b) = yuv_to_rgb(y, u, v);
    rgb_to_hsv(r, g, b)
}

pub fn hsv_to_yuv(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    let (r, g, b) = hsv_to_rgb(h, s, v);
    rgb_to_yuv(r, g, b)
}

// See: https://en.wikipedia.org/wiki/YUV
pub fn yuv_to_rgb(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let c = i32::from(y) - 16;
    let d = i32::from(u) - 128;
    let e = i32::from(v) - 128;

    let r = (298 * c + 409 * e + 128) >> 8;
    let g = (298 * c - 100 * d - 208 * e + 128) >> 8;
    let b = (298 * c + 516 * d + 128) >> 8;

    fn to_u8(x: i32) -> u8 {
        min(max(x, 0), 255) as u8
    }

    (to_u8(r), to_u8(g), to_u8(b))
}

// See: https://en.wikipedia.org/wiki/YUV
pub fn rgb_to_yuv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = i32::from(r);
    let g = i32::from(g);
    let b = i32::from(b);

    let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
    let u = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
    let v = ((112 * r - 94 * g - 18 * b) >> 8) + 128;

    (y as u8, u as u8, v as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_hsv_works() {
        let inputs = [(255, 0, 0), (10, 30, 200), (222, 222, 222)];
        for i in inputs {
            let (h, s, v) = rgb_to_hsv(i.0, i.1, i.2);
            let (r, g, b) = hsv_to_rgb(h, s, v);

            dbg!(i);
            dbg!((r, g, b));

            assert!((i32::from(r) - i32::from(i.0)).abs() <= 2);
            assert!((i32::from(g) - i32::from(i.1)).abs() <= 2);
            assert!((i32::from(b) - i32::from(i.2)).abs() <= 2);
        }
    }

    #[test]
    fn rgb_to_yuv_works() {
        let inputs = [(255, 0, 0), (10, 30, 200), (222, 222, 222), (0, 133, 0)];
        for i in inputs {
            let (y, u, v) = rgb_to_yuv(i.0, i.1, i.2);
            let (r, g, b) = yuv_to_rgb(y, u, v);

            dbg!(i);
            dbg!((r, g, b));

            assert!((i32::from(r) - i32::from(i.0)).abs() <= 2);
            assert!((i32::from(g) - i32::from(i.1)).abs() <= 2);
            assert!((i32::from(b) - i32::from(i.2)).abs() <= 2);
        }
    }

    #[test]
    fn yuv_to_hsv_works() {
        let inputs = [(83, 89, 79), (188, 128, 128), (124, 90, 55)];
        for i in inputs {
            let (h, s, v) = yuv_to_hsv(i.0, i.1, i.2);
            let (y, u, v) = hsv_to_yuv(h, s, v);

            dbg!(i);
            dbg!((y, u, v));

            assert!((i32::from(y) - i32::from(i.0)).abs() <= 2);
            assert!((i32::from(u) - i32::from(i.1)).abs() <= 2);
            assert!((i32::from(v) - i32::from(i.2)).abs() <= 2);
        }
    }
}
