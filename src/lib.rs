#![doc = include_str!("../README.md")]

pub mod algorithms;

pub trait RandomAlgorithm {
	fn from_seed(seed: u32) -> Self;

	fn next_value(&mut self) -> u32;
}
