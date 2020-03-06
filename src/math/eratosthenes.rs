fn eratosthenes(n: i64) -> Result<Vec<i64>, String> {
    match n {
        n if n < 2 => {
            return Err("n is more than 2".to_owned());
        }
        _ => {
            let mut prime: Vec<i64> = Vec::new();
            let limit = (n as f64).sqrt();
            let mut table: Vec<i64> = (2..(n + 1)).collect();
            let mut p = 0;

            loop {
                p = table[0];
                if limit <= p as f64 {
                    prime.append(&mut table);
                    return Ok(prime);
                }
                prime.push(p);
                table = table.into_iter().filter(|e| e % p != 0).collect();
            }
        }
    };
}

#[test]
fn test() {
    let prime: Vec<i64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    assert_eq!(prime, eratosthenes(100).unwrap());
}
