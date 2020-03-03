trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}
impl <T: Ord> BinarySearch<T> for Vec<T> {
    fn lower_bound(&self, x: T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (high + low) / 2;
            if self[mid] < x {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }

    fn upper_bound(&self, x: T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (high + low) / 2;
            if self[mid] > x {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

