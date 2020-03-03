use std::collections::HashMap;

struct UnionFind {
    parents: Vec<isize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let parents = vec![-1; n];
        UnionFind { parents: parents }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 {
            x
        } else {
            let xx = self.parents[x] as usize;
            let y = self.find(xx);
            self.parents[x] = y as isize;
            y
        }
    }
    fn unite(&mut self, x: usize, y: usize) -> bool {
        use std::mem::swap;
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.parents[x] > self.parents[y] {
            swap(&mut x, &mut y);
        }
        self.parents[x] += self.parents[y];
        self.parents[y] = x as isize;
        true
    }
    fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        (-self.parents[x]) as usize
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parents.len())
            .into_iter()
            .filter(|i| self.find(*i) == root)
            .collect::<Vec<usize>>()
    }
    fn roots(&mut self) -> Vec<usize> {
        (0..self.parents.len())
            .into_iter()
            .filter(|i| self.parents[*i] < 0)
            .collect::<Vec<usize>>()
    }
    fn all_group_members(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut groups_map = HashMap::new();
        for x in 0..self.parents.len() {
            let r = self.find(x);
            groups_map.entry(r).or_insert(vec![]).push(x);
        }
        groups_map
    }
}


