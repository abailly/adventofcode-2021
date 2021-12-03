use aoc2021::files::read_lines;
use aoc2021::parser::{parse_bits, to_int, Bit};
use std::cmp::Ordering;

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
        let oxy_selector: fn(&Vec<Vec<Bit>>, usize) -> Bit = |bits, pos| {
            let (one, zero) = bits.iter().fold((0, 0), |(one, zero), vec| match vec[pos] {
                // For some reason, adding '+ 0' turns the reference into a value
                Bit::One => (one + 1, zero + 0),
                Bit::Zero => (one + 0, zero + 1),
            });
            match one.cmp(&zero) {
                Ordering::Less => Bit::Zero,
                Ordering::Equal => Bit::One,
                Ordering::Greater => Bit::One,
            }
        };

        let co2_selector: fn(&Vec<Vec<Bit>>, usize) -> Bit = |bits, pos| {
            let (one, zero) = bits.iter().fold((0, 0), |(one, zero), vec| match vec[pos] {
                // For some reason, adding '+ 0' turns the reference into a value
                Bit::One => (one + 1, zero + 0),
                Bit::Zero => (one + 0, zero + 1),
            });
            match one.cmp(&zero) {
                Ordering::Less => Bit::One,
                Ordering::Equal => Bit::Zero,
                Ordering::Greater => Bit::Zero,
            }
        };

        let oxygen_rating = select_with_filter(bits.clone(), oxy_selector, 0);
        let co2_rating = select_with_filter(bits.clone(), co2_selector, 0);

        println!("{}", gamma_value * epsilon_value);
        println!("{}", oxygen_rating * co2_rating);
    }
}

fn select_with_filter(
    numbers: Vec<Vec<Bit>>,
    selector: fn(&Vec<Vec<Bit>>, usize) -> Bit,
    pos: usize,
) -> i32 {
    if numbers.len() == 1 {
        let rating = to_int(&numbers[0]);
        print!("rating {}, numbers {:?}\n", rating, &numbers[0]);
        return rating;
    }

    let sign_bit: Bit = selector(&numbers, pos);
    print!("bit {:?}\n", sign_bit);
    let filtered = numbers
        .into_iter()
        .filter(|bits| bits[pos] == sign_bit)
        .collect();

    select_with_filter(filtered, selector, pos + 1)
}
