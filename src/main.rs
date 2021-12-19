use std::{env, fs::File, str::FromStr};

use image::{png::PNGEncoder, ColorType, ImageError};
use num::Complex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.2",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds: (usize, usize) = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing the upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing the lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing the PNG file")
}

/// try to determine if `c` is in the Mandlebrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, returns `Some(i)`, where `i` is the number of
/// iterations it tok for `c` to leave the circle of radius 2 centered on the origin.
/// If `c` seems to be a member (more precisely, if we reached the iteration limit without
/// being able to prove that `c` is not a member), return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Parse the string `s` as a coordinate pair, like `"200x300"` or `"1.0,0.4"`
///
/// Specifically, `s` should have the form `<left><sep><right>`, where `<sep>` is the
/// character given by the `separator` argument, and `<left>` and `<right>` both strings
/// that can be parsed by `T::from_str`. `separator` must be an ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse correctly,
/// return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
        None => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("1,", ','), None);
    assert_eq!(parse_pair::<f64>("0.1,0.2", ','), Some((0.1, 0.2)));
    assert_eq!(parse_pair::<f64>("500x", 'x'), None);
    assert_eq!(parse_pair::<i32>("500x300", 'x'), Some((500, 300)));
}

/// Parse a pair of floating point numbers separated by a comma
/// as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("0.1,0.3"), Some(Complex { re: 0.1, im: 0.3 }));
    assert_eq!(parse_complex(",0.3"), None);
}

/// Given the row and the column of a pixel in the output image, return
/// the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and the height of the image in pixels.
/// `pixel` is a (column,row) pair inidicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // pixel.1 increases as we go down, the imaginary component increases as we go up
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and the height of the buffer `pixels`,
/// which holds one grayscale pizel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            // if escape_time says that point belongs to the set, render colors
            // the corresponding pixel black (0). Otherwise, render assigns darker colors
            // to the numbers that tool longer to escape the circle.
            pixels[row * bounds.0 + column] = {
                match escape_time(point, 255) {
                    Some(count) => 255 - count as u8,
                    None => 0,
                }
            }
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to
/// the file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}
