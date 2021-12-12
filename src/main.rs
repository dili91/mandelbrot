use std::str::FromStr;

use num::Complex;

fn main() {
    let point = Complex { re: 0.1, im: 0.3 };
    let result = escape_time(point, 1000);

    match result {
        Some(i) => println!(
            "Point {:?} left the mandlebrot set after {} iterations.",
            point, i
        ),
        None => println!("Point {:?} is in the mandlebrot set.", point),
    }
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
