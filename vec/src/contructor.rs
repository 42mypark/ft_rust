use super::Vec;

use core::alloc::Allocator;
use std::{alloc::Global, ptr::NonNull};

impl<T> Vec<T, Global> {
	pub fn new() -> Self {
		Self {
			ptr: NonNull::dangling(),
			cap: 0,
			len: 0,
			alloc: Global,
		}
	}
}

impl<T, A: Allocator> Vec<T, A> {
	pub fn new_in(alloc: A) -> Self {
		Self {
			ptr: NonNull::dangling(),
			cap: 0,
			len: 0,
			alloc,
		}
	}
}
