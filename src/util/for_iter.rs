trait UtilForIter<A = Self>: Sized {
    fn sum<I: Iterator<Item=A>>(iter: I) -> Self;
    fn product<I: Iterator<Item=A>>(iter: I) -> Self;
}

macro_rules! util_integer {
    ($($a:ty)*) => ($(
        impl UtilForIter for $a {
            fn sum<I: Iterator<Item=$a>>(iter: I) -> $a {
               iter.fold(0, |sum, x| sum + x)
            }
            fn product<I: Iterator<Item=$a>>(iter: I) -> $a {
                iter.fold(1, |prod, x| prod * x)
            }
        }
    )*);
}

macro_rules! util_float {
    ($($a:ty)*) => ($(
        impl UtilForIter for $a {
            fn sum<I: Iterator<Item=$a>>(iter: I) -> $a {
               iter.fold(0.0, |sum, x| sum + x)
            }
            fn product<I: Iterator<Item=$a>>(iter: I) -> $a {
                iter.fold(1.0, |prod, x| prod * x)
            }
        }
    )*)
}

util_integer! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }
util_float! { f32 f64 }

#[test]
fn test() {
    let vi64: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vi32: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vf32: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_eq!(vi64.into_iter().sum::<i64>(), 55);
    assert_eq!(vi32.into_iter().product::<i32>(), 3628800);
    assert_eq!(vf32.into_iter().sum::<f32>(), 55.0);
}
