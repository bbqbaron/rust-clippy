fn odds_out(x: i32) -> Option<i32> {
    if x % 2 == 0 {
        Some(x)
    } else {
        None
    }
}

fn main() {
    let evens: Vec<i32> = vec![1, 2, 3]
        .into_iter()
        .map(odds_out)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();
}
