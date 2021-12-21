use std::time::Instant;

fn play_rec(
    side: u8,
    p1: u8,
    s1: u8,
    p2: u8,
    s2: u8,
    outcomes: &Vec<(u64, u8)>,
    count: u64,
    s1win: &mut u64,
    s2win: &mut u64,
) {
    for (num, roll) in outcomes {
        if side == 0 {
            let pos = (p1 + roll) % 10;
            let ns1 = s1 + (pos + 1);
            if ns1 >= 21 {
                *s1win += count * num;
            } else {
                play_rec(1, pos, ns1, p2, s2, outcomes, count * num, s1win, s2win);
            }
        } else {
            let pos = (p2 + roll) % 10;
            let ns2 = s2 + (pos + 1);
            if ns2 >= 21 {
                *s2win += count * num;
            } else {
                play_rec(0, p1, s1, pos, ns2, outcomes, count * num, s1win, s2win);
            }
        }
    }
}

fn main() {
    let probas: Vec<(u64, u8)> = vec![(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

    let mut p1win = 0;
    let mut p2win = 0;
    let now = Instant::now();
    play_rec(0, 5, 0, 6, 0, &probas, 1, &mut p1win, &mut p2win);

    println!(
        "{} p1win {} p2win {}",
        now.elapsed().as_secs_f32(),
        p1win,
        p2win
    );
}
