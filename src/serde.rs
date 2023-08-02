use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::hash::Hash;

/// Bi-lateral (serializable and deserializable) Look-up Table
/// [Look `base::FastBlt` for more]
pub struct SerdeFastBlt<T: Hash, Serialize, Deserialize> {
    main_table: Vec<T>,
    hash_table: Vec<u64>,
}

/// Slow (serializable and deserializable) BLT
/// [Look `base::SlowBlt` for more]
pub struct SerdeSlowBlt<T: Serialize, Deserialize> {
    main_table: HashMap<u64, T>
}

pub struct SerdeFlashBlt<T: