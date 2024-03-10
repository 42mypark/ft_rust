#![feature(try_trait_v2)]

use core::ops::{ControlFlow, Try};
use std::ops::FromResidual;

#[derive(PartialEq, Eq, Debug)]
enum RevOpt<T> {
	None,
	Some(T),
}

impl<T> RevOpt<T> {
	fn unwrap(self) -> T {
		if let RevOpt::Some(x) = self {
			x
		} else {
			panic!("called `Option::unwrap()` on a `None` value")
		}
	}
}

impl<T> FromResidual for RevOpt<T> {
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		residual
	}
}

impl<T> Try for RevOpt<T> {
	type Output = ();
	type Residual = RevOpt<T>;
	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
		if let RevOpt::None = self {
			ControlFlow::Continue(())
		} else {
			ControlFlow::Break(self)
		}
	}

	fn from_output(_: Self::Output) -> Self {
		RevOpt::None
	}
}

#[cfg(test)]
mod test {
	use std::ops::Try;

	use crate::RevOpt;

	#[test]
	fn test_unwrap_some() {
		let a: RevOpt<usize> = RevOpt::Some(1);
		assert_eq!(a.unwrap(), 1);
	}

	#[test]
	#[should_panic]
	fn test_unwrap_none() {
		let a: RevOpt<usize> = RevOpt::None;
		a.unwrap();
	}

	#[test]
	fn test_branch_none() {
		fn branch() -> RevOpt<usize> {
			let a = RevOpt::None;

			let b = a?;

			RevOpt::from_output(b)
		}

		assert_eq!(branch(), RevOpt::None);
	}

	#[test]
	fn test_branch_some() {
		fn branch() -> RevOpt<usize> {
			let a = RevOpt::Some(1);

			let _ = a?;

			unreachable!()
		}

		assert_eq!(branch(), RevOpt::Some(1));
	}
}
