use crate::set::Set;
use core::ops::Sub;
use core::{fmt, marker::PhantomData};
use slicing::abstra::NewLike;
use slicing::index::{Indexer, RangeIndexer};
use slicing::slices::SliceDefault;

/// A Set backed by an iterable storage of boolean flags.
/// See `crate::slices::SliceStorage` for const generic parameter `N`.
#[derive(Debug)]
pub struct BoolFlagSet<
    's,
    T: 's + Clone + PartialEq,
    I: Indexer<T>,
    SL: SliceDefault<'s, bool, N>,
    const N: usize,
> {
    slice: SL,
    /// Stored owned, not by reference - good for CPU cache affinity.
    indexer: I,
    _items: PhantomData<T>, // so that we don't mix BoolSliceSet of various item types
    _s_lifetimed: PhantomData<&'s ()>,
}

impl<
        's,
        T: 's + Eq + Clone + Copy + Default,
        I: Indexer<T>,
        SL: 's + SliceDefault<'s, bool, N> + Clone + NewLike,
        const N: usize,
    > Set<T> for BoolFlagSet<'s, T, I, SL, N>
where
    SL::ITER<'s>: 's,
{
    type ITER<'a> = BoolFlagSetIter<'a, T, I, SL::ITER<'a>> where T: 'a, Self: 'a;

    type ITERREF<'a> = UnimplementedIterRef<'a, T>
    where
        T: 'a,
        Self: 'a,
    ;

    fn contains(&self, value: &T) -> bool {
        self.slice.get(self.indexer.index(value))
    }
    fn insert(&mut self, value: T) -> bool {
        let index = self.indexer.index(&value);
        self.slice.check_and_set(index, &true)
    }
    fn remove(&mut self, value: &T) -> bool {
        let index = self.indexer.index(&value);
        self.slice.check_and_set(index, &false)
    }

    fn iter<'a>(&'a self) -> Self::ITER<'a> {
        //fn iter(&'s self) -> Self::ITER<'s> {
        BoolFlagSetIter {
            slice_enum: self.slice.iter().enumerate(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
        }
    }
}

impl<
        's,
        T: 's + Clone + PartialEq,
        I: Indexer<T>,
        SL: SliceDefault<'s, bool, N>,
        const N: usize,
    > BoolFlagSet<'s, T, I, SL, N>
{
    pub fn new(slice: SL, indexer: I) -> Self {
        Self {
            slice,
            indexer,
            _items: PhantomData,
            _s_lifetimed: PhantomData,
        }
    }
}

impl<
        's,
        T: 's + Eq + Clone + Copy + Default,
        I: Indexer<T>,
        SL: SliceDefault<'s, bool, N> + NewLike,
        const N: usize,
    > NewLike for BoolFlagSet<'s, T, I, SL, N>
{
    fn new_like(&self) -> Self {
        Self {
            slice: self.slice.new_like(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
            _s_lifetimed: PhantomData,
        }
    }
}

impl<'s, T: Eq + Clone, I: Indexer<T>, SL: SliceDefault<'s, bool, N> + Clone, const N: usize> Clone
    for BoolFlagSet<'s, T, I, SL, N>
{
    fn clone(&self) -> Self {
        Self {
            slice: self.slice.clone(),
            indexer: self.indexer.clone(),
            _items: PhantomData,
            _s_lifetimed: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct BoolFlagSetIter<'a, T: Clone, IND: Indexer<T>, SLIT: Iterator<Item = &'a bool>> {
    slice_enum: core::iter::Enumerate<SLIT>,
    /// Not a reference, but cloned & owned (better for CPU cache affinity)
    /// when indexers are small. Having a big indexer? Then make the Indexer implementation refer to it.
    indexer: IND,
    _items: PhantomData<T>,
}
impl<'a, T: Clone, IND: Indexer<T>, SLIT: Iterator<Item = &'a bool>> Iterator
    for BoolFlagSetIter<'a, T, IND, SLIT>
{
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        loop {
            if let Some((index, &value_present)) = self.slice_enum.next() {
                if value_present {
                    break Some(self.indexer.key(index));
                }
            } else {
                break None;
            }
        }
    }
}

pub struct UnimplementedIterRef<'a, T: Clone + PartialEq> {
    phantom: PhantomData<&'a T>,
}

impl<'a, T: Clone + PartialEq> Iterator for UnimplementedIterRef<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        unimplemented!();
    }
}

impl<'s, T: 's + Eq + Clone + Default + Sub<T>, SL: SliceDefault<'s, bool, N>, const N: usize>
    BoolFlagSet<'s, T, RangeIndexer<T>, SL, N>
where
    T: TryInto<usize>,
    usize: TryFrom<T>,
    usize: TryFrom<<T as Sub>::Output>,
    T: TryFrom<usize>,
    <usize as TryFrom<<T as Sub>::Output>>::Error: fmt::Debug,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    <T as TryInto<usize>>::Error: fmt::Debug,
{
    // internal helper
    fn from_bool_slice(slice: SL, start: &T) -> Self {
        Self {
            slice,
            indexer: RangeIndexer::<T>::new(start),
            _items: PhantomData,
            _s_lifetimed: PhantomData,
        }
    }
}
