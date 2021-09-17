use super::SMA;
use crate::common::cast_to_divisor_type;
use num_traits::{FromPrimitive, Zero};
use std::{
	collections::VecDeque,
	marker::{self, PhantomData},
	ops::{AddAssign, Div},
};

/// An SMA implementation that does not cache any intermediate sample sum.
pub struct NoSumSMA<Sample, Divisor, const WINDOW_SIZE: usize> {
	samples: VecDeque<Sample>,
	zero: Sample,
	_marker: marker::PhantomData<Divisor>,
}

impl<Sample, Divisor, const WINDOW_SIZE: usize> SMA<Sample, Divisor>
	for NoSumSMA<Sample, Divisor, WINDOW_SIZE>
where
	Sample: Copy + AddAssign + Div<Divisor, Output = Sample>,
	Divisor: FromPrimitive,
{
	fn add_sample(&mut self, new_sample: Sample) {
		if WINDOW_SIZE == 0 {
			return;
		}

		if self.samples.len() == WINDOW_SIZE {
			self.samples.pop_back();
		}

		self.samples.push_front(new_sample);
	}

	fn get_average(&self) -> Sample {
		let num_samples = self.samples.len();

		if num_samples == 0 {
			return self.zero;
		}

		let sum = {
			let mut sum = self.zero;
			for sample in &self.samples {
				sum += *sample;
			}
			sum
		};

		sum / cast_to_divisor_type(num_samples)
	}

	fn get_most_recent_sample(&self) -> Option<Sample> {
		self.samples.front().cloned()
	}

	fn get_samples(&mut self) -> &[Sample] {
		self.samples.make_contiguous();
		self.samples.as_slices().0
	}

	fn get_num_samples(&self) -> usize {
		self.samples.len()
	}

	fn get_sample_window_size(&self) -> usize {
		WINDOW_SIZE
	}
}

impl<Sample: Zero, Divisor, const WINDOW_SIZE: usize> NoSumSMA<Sample, Divisor, WINDOW_SIZE> {
	/// Constructs a new [NoSumSMA] with window size `WINDOW_SIZE`. This constructor is
	/// only available for `Sample` types that implement [num_traits::Zero]. If the `Sample` type
	/// does not, use the [from_zero](NoSumSMA::from_zero) constructor instead.
	///
	/// Note that the `Divisor` type usually cannot be derived by the compiler when using this
	/// constructor and must be explicitly stated, even if it is the same as the `Sample` type.
	pub fn new() -> Self {
		Self {
			samples: VecDeque::with_capacity(WINDOW_SIZE),
			zero: Sample::zero(),
			_marker: PhantomData,
		}
	}
}

impl<Sample, Divisor, const WINDOW_SIZE: usize> NoSumSMA<Sample, Divisor, WINDOW_SIZE> {
	/// Constructs a new [NoSumSMA] with window size `WINDOW_SIZE` from the given
	/// `zero` sample. If the `Sample` type implements [num_traits::Zero], the
	/// [new](NoSumSMA::new) constructor might be preferable to this.
	pub fn from_zero(zero: Sample) -> Self {
		Self {
			samples: VecDeque::with_capacity(WINDOW_SIZE),
			zero,
			_marker: PhantomData,
		}
	}
}
