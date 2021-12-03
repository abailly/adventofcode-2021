use aoc2021::files::read_lines;
use aoc2021::parser::{parse_bits, to_int, Bit};

fn main() {
    if let Ok(lines) = read_lines("./day3/input.txt") {
        let bits: Vec<Vec<Bit>> = lines
            .map(|line| parse_bits(&line.unwrap()).unwrap())
            .collect();

        let apply_bits = |acc: Vec<(i32, i32)>, bits: &Vec<Bit>| {
            acc.iter()
                .zip(bits.iter())
                .map(|((one, zero), bit)| match bit {
                    // For some reason, adding '+ 0' turns the reference into a value
                    Bit::One => (one + 1, zero + 0),
                    Bit::Zero => (one + 0, zero + 1),
                })
                .collect()
        };

        let bits_freq: Vec<(i32, i32)> = bits.iter().fold(vec![(0, 0); 12], apply_bits);
        let gamma = &bits_freq
            .iter()
            .map(|(one, zero)| if one <= zero { Bit::Zero } else { Bit::One })
            .collect();
        let epsilon = &bits_freq
            .iter()
            .map(|(one, zero)| if one >= zero { Bit::Zero } else { Bit::One })
            .collect();

        let gamma_value = to_int(&gamma);
        let epsilon_value = to_int(&epsilon);
        println!("{}", gamma_value * epsilon_value);
    }
}
