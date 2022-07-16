use crate::set::Set;
use core::hash::Hash;
use std::collections::{hash_set, hash_set::Iter, HashSet};

impl<T: Hash + Eq + Clone> Set<T> for HashSet<T> {
    type ITER<'a> = HashSetIter<'a, T>
    where
        T: 'a,
        Self: 'a,
    ;

    type ITERREF<'a> = Iter<'a, T>
    where
        T: 'a,
        Self: 'a,
    ;

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
    fn insert(&mut self, value: T) -> bool {
        self.insert(value)
    }
    fn remove(&mut self, value: &T) -> bool {
        self.remove(value)
    }
    fn iter<'a>(&'a self) -> Self::ITER<'a> {
        HashSetIter {
            set_iter: self.iter(),
        }
    }

    fn supports_iter_ref() -> bool {
        true
    }

    fn iter_ref<'a>(&'a self) -> Self::ITERREF<'a> {
        self.iter()
    }
}

pub struct HashSetIter<'a, T: 'a> {
    set_iter: hash_set::Iter<'a, T>,
}

impl<'a, T: Clone> Iterator for HashSetIter<'a, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        self.set_iter.next().cloned()
    }
}