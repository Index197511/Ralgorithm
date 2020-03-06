fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 { x } else { gcd(y, x % y) }
}

fn lcm(x: i64, y: i64) -> i64 {
    let t = gcd(x, y);
    t * x / t * y / t
}

#[test]
fn test() {
    assert_eq!(gcd(6, 20), 2);
    assert_eq!(lcm(2, 3), 6);
}