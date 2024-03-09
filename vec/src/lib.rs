#![feature(allocator_api)]

mod contructor;

use core::alloc::Allocator;
use std::{alloc::Global, ptr::NonNull};

pub use contructor::*;

pub struct Vec<T, A = Global>
where
	A: Allocator,
{
	ptr: NonNull<T>,
	cap: usize,
	len: usize,
	alloc: A,
}

impl<T, A: Allocator> Vec<T, A> {
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn capacity(&self) -> usize {
		self.cap
	}
}
