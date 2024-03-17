use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
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

// fn parse_complex(s)

#[test]
fn test_parse_paire() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}