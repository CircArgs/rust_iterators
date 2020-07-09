pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(itm) = inner_iter.next() {
                    return Some(itm);
                } else {
                    self.inner = None;
                }
            }
            let mut next_inner = self.outer.next()?.into_iter();
            self.inner = Some(next_inner);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(
            flatten(vec![Vec::<i32>::new(), vec![], vec![], vec![]].iter()).count(),
            0
        );
    }
    #[test]
    fn one() {
        assert_eq!(flatten(vec![vec![1]].iter()).count(), 1);
    }
    #[test]
    fn two_two() {
        assert_eq!(flatten(vec![vec![1], vec![2]].iter()).count(), 2);
    }
    #[test]
    fn two() {
        assert_eq!(flatten(vec![vec![1, 2]].iter()).count(), 2);
    }
    #[test]
    fn one_not_empty() {
        assert_eq!(
            flatten(vec![Vec::<i32>::new(), vec![1], vec![], vec![]].iter()).count(),
            1
        );
    }
}
