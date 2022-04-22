use crate::bool_flag::BoolFlagSet;
use slicing::byte_slice::ByteSliceBoolStorage;

/// Backed by a packed slice of bits (rounded up to bytes). That results not
/// only in 8x less storage,  but in less cache & RAM bandwidth => faster.
pub type Set<'s, T, I, const N: usize> = BoolFlagSet<'s, T, I, ByteSliceBoolStorage<'s, N>, N>;
