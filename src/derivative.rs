use std::cmp::Ordering::Less;

/// Count the number items in a vector that are greater
/// than the previous one.
pub fn count_increasing(windows: &Vec<i32>) -> usize {
    let windows_tail = &windows[1..];
    windows
        .iter()
        .zip(windows_tail.iter())
        .map(|(a, b)| a.cmp(b))
        .filter(|ord| *ord == Less)
        .count()
}
