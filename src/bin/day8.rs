use std::env;
use std::fs::read_to_string;
use std::process;

#[derive(Debug, PartialEq, Clone)]
struct DigitLine<'a> {
    codes: Vec<&'a str>,
    digits: Vec<&'a str>,
}

fn is_simple(s: &str) -> bool {
    let ln = s.len();
    ln == 2 || ln == 3 || ln == 4 || ln == 7
}

fn as_bits(s: &str) -> u8 {
    s.chars()
        .fold(0, |n, c| n + (1 << ((c as u8) - ('a' as u8))))
}

fn encode(codes: &Vec<&str>) -> [u8; 10] {
    let mut digits: [u8; 10] = [0; 10];
    // first handle simple digits
    codes.iter().filter(|s| is_simple(s)).for_each(|s| {
        let bits = as_bits(s);
        match s.len() {
            2 => digits[1] = bits,
            3 => digits[7] = bits,
            4 => digits[4] = bits,
            7 => digits[8] = bits,
            _ => panic!("unknown encoding"),
        }
    });
    codes.iter().filter(|s| s.len() == 6).for_each(|s| {
        let bits = as_bits(s);
        if bits & digits[4] == digits[4] {
            digits[9] = bits;
        } else if bits & digits[7] == digits[7] {
            digits[0] = bits;
        } else {
            digits[6] = bits;
        }
    });
    codes.iter().filter(|s| s.len() == 5).for_each(|s| {
        let bits = as_bits(s);
        if bits & digits[1] == digits[1] {
            digits[3] = bits;
        } else if bits & digits[6] == bits {
            digits[5] = bits;
        } else {
            digits[2] = bits;
        }
    });
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
