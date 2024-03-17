use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixcels = vec![0; bounds.0 * bounds.1];

    render(&mut pixcels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixcels, bounds).expect("error writing PNG file");
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c
    }
}

/// 'c'가 망델브로 집합에 속하는지 아닌지를 판판단단하하며며, 결론 내리는 데 필요한 반복 횟수는 최대
/// 'limit'회로 제한한다.
/// 
/// 'c'가 망델브로 집합에 속하지 않으며 'Some(i)'를 반환하는데, 여기서 'i'는 'c'가 원점을
/// 중심으로 반경이 2인 원을 벗어나는 데 걸린 반복 횟수다. 'c'가 망델브로 집합에 속하는 것
/// 같으면(좀 더 정확히 말해서 반복 횟수가 'limit'이 될 때까지도 'c'가 망델브로 집합에
/// 속하지 않는다는 걸 입증하지 못하면) 'None'을 반환한다.

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

/// 's'를 '"400x600"'이나 '"1.0.0.5"'와 같은 좌표 쌍으로 파싱한다.
/// 
/// 's'는 정확히 <left><sep><right> 형식으로 되어 있어야 하는데, 여기서 <sep>은
/// 'separator' 인수에 넘기는 문자이고 <left>와 <right>는 둘 다 'T::from_str'로
/// 파싱될 수 있는 문자열이다. 'saparator'는 반드시 아스키 문자여야 한다.
/// 
/// 's'가 올바른 형식으로 되어 있으면 'Some<(x, y)>'를 반환한다. 제대로 파싱되지 않으면
/// 'None'은 반환한다.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

/// 쉼표로 구분된 부동소수점 수 쌍을 복소수로 파싱한다.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

/// 결과 이미지의 픽셀 좌표가 주어지면 여기에 대응하는 복소평면 위의 점을 반환한다.
/// 
/// 'bounds'는 픽셀 단위로 된 이미지의 포고가 높이를 갖는 쌍이다. 'pixel'은 이미지의 특정
/// 이미지가 커버하는 영역을 지정하는  복소평면 위의 두 점이다.
fn pixel_to_point(bounds: (usize, usize),
                pixel: (usize, usize),
                upper_left: Complex<f64>,
                lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                            upper_left.im - lower_right.im);
    
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

/// 직사각형 모양의 망델브로 집합을 픽셀 버퍼에 랜더링한다.
/// 
/// 'bounds' 인수는 한 바이트 안에 회색조로 된 픽셀 하나가 들어가는 'pixels' 버퍼의 폭과
/// 높이를 낮는다. 'upper_left'와 'lower_right' 인수는 픽셀 버퍼의 왼쪽 위 모서리와
/// 오른쪽 아래 모서리에 해당하는 복소평면 위의 두 점을 지정한다.
fn render(pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
            };
        }
    }
}

fn write_image(filename: &str, pixcels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixcels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex {re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                Complex {re: -1.0, im: 1.0},
                Complex {re: 1.0, im: -1.0}),
            Complex {re: -0.5, im: -0.75});
}