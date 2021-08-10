use std::iter::FromIterator;
use std::ops::Index;
pub struct MyVec<T> {
    vec: Box<Vec<T>>,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            vec: Box::new(Default::default()),
        }
    }
    pub fn push(&mut self, val: T) {
        self.vec.push(val);
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        let vec = self.vec.get(idx);
        vec
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn iter(&self) ->Iter<T> {
        self.into_iter()
    }
    /*
    pub fn into_vec(self) -> Vec<T> {
        //self.vec.into_inner() error
        Box::<Vec<T>>::into_inner
    }
    */
    pub fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }
}
impl<T> Default for MyVec<T> {
    fn default() -> Self {
        MyVec::new()
    }
}
impl<T> From<Vec<T>> for MyVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            vec: Box::new(vec),
        }
    }
}
impl<T> Index<usize> for MyVec<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        self.get(idx).unwrap_or_else(|| {
            panic!("index out of bounds: the len is {:?} but the index is  {:?}"
            ,self.len(), idx)
        })
    }
}
impl<A> FromIterator<A> for MyVec<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
        {
            let vec: Vec<_> = iter.into_iter().collect();
            vec.into()
        }
}
pub struct Iter<'a, T> {
    vec: &'a MyVec<T>,
    idx: usize,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if let Some(ret) = self.vec.get(self.idx) {
            self.idx += 1;
            Some(ret)
        } else {
            None
        }
    }
}
impl<'a, T> IntoIterator for &'a MyVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        Iter { vec: self, idx: 0 }
    }
}
#[test]
fn test() {
    let vec = vec!["a", "b", "c", "d"];
    let myvec: MyVec<_> = vec.clone().into();
    
}
