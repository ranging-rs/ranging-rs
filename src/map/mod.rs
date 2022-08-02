use core::borrow::Borrow;
use slicing::abstra::NewEmptyLike;

/// Abstract map, either general or for a given range. While some
/// implementations (like `std::collections::HashMap`) could be possible without
/// `K: Clone`, range-based implementations need `K: Clone` so that they can
/// store the range's start/base key itself (rather than a reference). See also
/// `crate::set::Set`. `V` has to implement `Clone`, so that `Self::ITER` clones
/// the values.
pub trait Map<K: Clone + PartialEq, V: Clone>: Clone + NewEmptyLike {
    type ITER<'a>: Iterator<Item = (K, V)>
    where
        K: 'a,
        V: 'a,
        Self: 'a;

    type ITERREF<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
        Self: 'a;

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>;

    /// Insert. Return true if this value was not present yet. (Based on
    /// std::collections::HashSet.)
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn insert_all(&mut self, iter: impl Iterator<Item = (K, V)>) {
        iter.for_each(|(key, value)| {
            self.insert(key, value);
        });
    }

    /// Remove. Return true if this value was actually present. (Based on
    /// std::collections::HashSet.)
    fn remove<Q: ?Sized>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>;

    /// Required.
    fn iter<'a>(&'a self) -> Self::ITER<'a>;

    /// Optional. Return `true` if `iter_ref` is supported.
    fn supports_iter_ref() -> bool {
        false
    }

    /// Optional. If supported, then `supports_iter_ref` returns `true`.
    fn iter_ref<'a>(&'a self) -> Self::ITERREF<'a> {
        unimplemented!("Not supported.");
    }
}
