use std::env;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, PartialEq, Clone)]
struct DigitLine<'a> {
    codes: Vec<&'a str>,
    digits: Vec<&'a str>,
}

fn is_simple(s: &&str) -> bool {
    let ln = s.len();
    ln == 2 || ln == 3 || ln == 4 || ln == 7
}

fn as_bits(s: &str) -> u8 {
    s.chars()
        .fold(0, |n, c| n + (1 << ((c as u8) - ('a' as u8))))
}

fn encoding_for_690(digits: &[u8; 10], bits: u8) -> usize {
    if bits & digits[4] == digits[4] {
        9
    } else if bits & digits[7] == digits[7] {
        0
    } else {
        6
    }
}

fn encoding_for_235(digits: &[u8; 10], bits: u8) -> usize {
    if bits & digits[1] == digits[1] {
        3
    } else if bits & digits[6] == bits {
        5
    } else {
        2
    }
}

fn encoding_for_1478(_digits: &[u8; 10], bits: u8) -> usize {
    match bits.count_ones() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => panic!("unknown encoding"),
    }
}

fn encode(codes: &Vec<&str>) -> [u8; 10] {
    let mut digits: [u8; 10] = [0; 10];

    let mut encoder = |filter: for<'a> fn(&'a &&str) -> bool,
                       encoding: fn(&[u8; 10], u8) -> usize| {
        codes.iter().filter(filter).for_each(|s| {
            let bits = as_bits(s);
            let pos = encoding(&digits, bits);
            digits[pos] = bits;
        })
    };

    encoder(|s| is_simple(s), encoding_for_1478);
    encoder(|s| s.len() == 6, encoding_for_690);
    encoder(|s| s.len() == 5, encoding_for_235);

    digits
}

fn decode(encoded: [u8; 10]) -> impl FnMut(&&str) -> u8 {
    return move |s| {
        let bits = as_bits(s);
        let mut res = 0;
        encoded.iter().enumerate().for_each(|(i, v)| {
            if *v == bits {
                res = i;
            }
        });
        res as u8
    };
}

fn solve<'a>(puzzle: &Vec<DigitLine<'a>>) -> u64 {
    let mut num = 0_u64;
    for DigitLine { codes, digits } in puzzle {
        let encoded = encode(&codes);
        num += digits
            .iter()
            .map(decode(encoded))
            .fold(0, |n, d| (n * 10) + d as u64);
    }
    num
}

fn parse_digits<'a>(lines: &Vec<&'a str>) -> Option<Vec<DigitLine<'a>>> {
    let mut res = vec![];
    for s in lines {
        let parts: Vec<&str> = s.split(" | ").collect();
        res.push(DigitLine {
            codes: parts[0].split(' ').collect(),
            digits: parts[1].split(' ').collect(),
        });
    }
    Some(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("expecting a file argument");
        process::exit(1);
    }

    if let Ok(input) = read_to_string(&args[1]) {
        if let Some(puzzle) = parse_digits(&input.split("\n").filter(|s| !s.is_empty()).collect()) {
            let solution = solve(&puzzle);
            println!("{}", solution);
        }
    } else {
        println!("fail to parse {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let sample = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"];

        if let Some(res) = parse_digits(&sample) {
            println!("{:?}", res);
            assert_eq!(
                res[0],
                DigitLine {
                    codes: vec![
                        "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                        "fabcd", "edb"
                    ],
                    digits: vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]
                }
            );
        } else {
            panic!("cannot parse input");
        }
    }

    #[test]
    fn convert_to_bits_field() {
        assert_eq!(as_bits("acd"), 13);
    }

    #[test]
    fn define_encoding() {
        let sample = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];

        let encoded = encode(&sample);

        assert_eq!(
            sample.iter().map(decode(encoded)).collect::<Vec<u8>>()[0..],
            [8, 5, 2, 3, 7, 9, 6, 4, 0, 1]
        );
    }
}
