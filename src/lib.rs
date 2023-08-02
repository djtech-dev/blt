/*/// Simple BLTs Implementation
pub mod base;
/// Serde BLTs Implementation
pub mod serde;

pub use base::*;*/

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::hash::Hash;

/// Bi-lateral Look-up Table
pub trait Blt<T: Serialize, Deserialize> {
	pub fn get_by_index();
	pub fn get_id_by_value();
	pub fn get_by_value() {}

	/// Get &mut to entry (requires index)
	pub fn get_mut_by_index();
	/// Get &mut to entry (requires value)
	pub fn get_mut_by_value();

	/// Swap two entries; requires two indexes
	pub fn swap_index();
	/// Swap two entries; requires two values
	pub fn swap_value();

	/// Push new value
	pub fn push();

	/// Length of BLT
	pub fn len();

	/// Iterates indexes
	pub fn iter_index();
	/// Iterate values
	pub fn iter_value();

	/// Remove all entries with a certain value
	pub fn remove(e: T);
	/// Remove entry; requires index
	pub fn pop(i: usize);
	/// Drop entry; requires index
	pub fn drop(i: usize);
}

/// Bi-lateral (serializable and deserializable) Look-up Table
pub struct FastBlt<T: Hash, Serialize, Deserialize> {
    main_table: Vec<T>,
    hash_table: Vec<u64>,
}
impl FastBlt {
	pub fn validate(&self) {
		assert!(self.main_table.len() == self.hash_table.len());
	}
}

/// Slow (serializable and deserializable) BLT
pub struct SlowBlt<T: Serialize, Deserialize> {
    main_table: HashMap<u64, T>
}


impl Blt for FastBlt {}
impl Blt for SlowBlt {}