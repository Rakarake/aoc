pub trait Lel {
    fn transpose(self) -> Self;
    fn reverse_rows(self) -> Self;
    fn rotate_counter_clockwise(self) -> Self;
}

impl<T: Clone> Lel for Vec<Vec<T>> {
    // Yes, very inefficient
    // Easy peasy when
    fn transpose(self) -> Self {
        let mut o: Vec<Vec<Option<T>>> = vec![vec![None; self.len()]; self.first().unwrap().len()];
        self.into_iter().enumerate().for_each(|(y, l)| {
            l.into_iter().enumerate().for_each(|(x, e)| {
                o[x][y] = Some(e);
            })
        });
        o.into_iter()
            .map(|r| r.into_iter().map(|e| e.unwrap()).collect())
            .collect()
    }

    fn reverse_rows(self) -> Self {
        let mut o = self.clone();
        o.reverse();
        o
    }

    fn rotate_counter_clockwise(self) -> Self {
        self.transpose().reverse_rows()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transpose() {
        let x = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(x.transpose(), result);
    }

    #[test]
    fn test_rotate_counter_clockwise() {
        let x = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
        assert_eq!(x.rotate_counter_clockwise(), result);
    }
}
