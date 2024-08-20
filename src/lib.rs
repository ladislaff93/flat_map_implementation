fn flat_map<O,F,U>(iter: O, f: F) -> MySimpleIterator<O::IntoIter,F,U> 
where 
    O:IntoIterator,
    U:IntoIterator,
    F: FnMut(O::Item) -> U
{
    MySimpleIterator::new(iter.into_iter(), f)
}

struct MySimpleIterator<O,F,U> 
where
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U
{
    outer: O,
    inner: Option<U::IntoIter>,
    func: F,
}


impl <O,F,U> MySimpleIterator<O,F,U>
where 
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U
{
    fn new(iter: O, f: F) -> Self {
        MySimpleIterator {
            outer: iter,
            func: f,
            inner: None
        } 
    }
}

impl <O,F,U> Iterator for MySimpleIterator<O,F,U>
where 
    O: Iterator,
    U: IntoIterator,
    F: FnMut(O::Item) -> U,
{
    type Item=U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // we check if there is value in self.inner
            if let Some(ref mut inner) = self.inner {
                // if is we will yield the value of iterator that is returned by mapping func
                if let Some(i) = inner.next() {
                    return Some(i);
                } 
                // if there is no value to yield next we will make self.inner none so we can next
                // yield the outer iterator
                self.inner = None;
            }
            // this will first yield outer eg (vec<vec<>,..>) -> vec<> then put it in func which
            // will remapped it and then return iterator that would be put into self.inner 
            // if there is no value to yield from outer iterator we will return None by ? operator
            self.inner = Some((self.func)(self.outer.next()?).into_iter());
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        assert_eq!(flat_map(Vec::<Vec<i8>>::new(), |s| {s}).collect::<Vec<i8>>(),vec![]);
    }
    #[test]
    fn test_nested() {
        assert_eq!(flat_map(vec![vec![1,2],vec![2],vec![3],vec![4]], |c|{c}).collect::<Vec<i32>>(), vec![1,2,2,3,4]);
    }
    
    #[test]
    fn test_str() {
        assert_eq!(flat_map(vec!["a", "b", "c"].into_iter(), |s| s.chars()).collect::<String>(), "abc");
    }

    #[test]
    fn test_empty_first() {
        assert_eq!(flat_map(vec!["", "b", "c"].into_iter(), |s| s.chars()).collect::<String>(), "bc");
    }

    #[test]
    fn test_empty_last() {
        assert_eq!(flat_map(vec!["a", "b", ""].into_iter(), |s| s.chars()).collect::<String>(), "ab");
    }

    #[test]
    fn test_empty_middle() {
        assert_eq!(flat_map(vec!["alpha", "", "beta", "", "", "gamma"].into_iter(), |s| s.chars()).collect::<String>(), "alphabetagamma");
    }

    #[test]
    fn test_long_str() {
        assert_eq!(flat_map(vec!["abc", "def", "ghi"].into_iter(), |s| s.chars()).collect::<String>(), "abcdefghi");
    }

    #[test]
    fn flat_map_primitive() {
        assert_eq!(vec!["abc", "def", "ghi"].into_iter()
            .map(|s|s.chars())
            .flatten()
            .collect::<String>()
            .len(),9) 
    }

}
