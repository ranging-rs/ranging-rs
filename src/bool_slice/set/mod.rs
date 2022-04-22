use crate::bool_flag::BoolFlagSet;
use slicing::byte_slice::ByteSliceBoolStorage;
use slicing::slices::BoolSlice;

/// Backed by a slice of booleans (not packed, but ordinary).
pub type UnpackedSet<'s, T, I, const N: usize> = BoolFlagSet<'s, T, I, BoolSlice<'s, N>, N>;

pub type PackedSet<'s, T, I, const N: usize> =
    BoolFlagSet<'s, T, I, ByteSliceBoolStorage<'s, N>, N>;
