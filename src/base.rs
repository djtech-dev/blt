use std::collections::HashMap;
use std::hash::Hash;

/// Bi-lateral Look-up Table
pub trait Blt {}

/// Bi-lateral Look-up Table
/// Requires the `Hash` trait.
pub struct FastBlt<T: Hash> {
    main_table: Vec<T>,
    hash_table: Vec<u64>,
}

/// Slow BLT.
/// Doesn't require the `Hash` trait.
pub struct SlowBlt<T> {
    main_table: HashMap<u64, T>
}

