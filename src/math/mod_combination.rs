fn repeat_pow_mod(n: i64, k: i64, m: i64) -> i64 {
    // n^k % m
    let mut res = 1;
    if k > 0 {
        res = repeat_pow_mod(n, k / 2, m);
        if k % 2 == 0 {
            res = res * res % m;
        }else {
            res = (res * res % m) * n % m;
        }
    }

    return res
}
fn mod_combination(n: i64, k: i64, m: i64) -> i64 {
    //nCk % m
    let num = (n - k + 1..n + 1).fold(1, |prod, x| prod * x % m);
    let den = (1..k + 1).fold(1, |prod, x| prod * x % m);
    num * repeat_pow_mod(den, m - 2, m) % m
}

