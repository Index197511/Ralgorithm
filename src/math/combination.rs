fn combination(n: i64, k: i64) -> i64 {
    let num = (n - k + 1..n + 1).fold(1, |prod, x| prod * x);
    let den = (1..k + 1).fold(1, |prod, x| prod * x);
    num / den
}

#[test]
fn test() {
    assert_eq!(10, combination(5, 3));
}
