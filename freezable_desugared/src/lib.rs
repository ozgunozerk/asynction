//! This library is created for the purpose of simulating the desugared version of a
//! `freezable` function. Which could be:
//! - a generator
//! - an iterator
//! - an async function
//!
//! It will be cumbersome and not intuitive to write the desugared version of a such `freezable` function
//! The aim of this library is to uncover the secrets and underlying code of such concepts :)

mod freezable_complex;
mod freezable_generator_4;

pub use freezable_complex::FreezableComplex;
pub use freezable_generator_4::FreezableGenerator4;
use std::fmt::Debug;

/// Freezable trait, but Desugared :)
///
/// Runs a chunk of code, and then freezes itself.
/// Have the methods: `unfreeze`, `cancel` and `is_cancelled`
///
/// imitates a function, that has chunks of code, and between each chunk, `freeze()` is called
/// `freeze()` is what makes the function stop there, and allows us to continue from where its left off
/// when we call the function again. We won't be seeing the `freeze()` calls here. Since this is the
/// desugared version.
/// Refer to `freezable_complex.rs` and `freezable_generator_4.rs` to see `freeze()` calls in the
/// imaginary original code (remember, this code is the desugared one of the imaginary original one)
pub trait DesugaredFreezable {
    type Output: Debug;

    /// should generate the next item in the sequence, then it will freeze itself again
    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError>;

    /// should cancel the Freezable, makes it impossible to call `unfreeze` again
    fn cancel(&mut self);

    /// checks whether the Freezable is cancelled
    fn is_cancelled(&self) -> bool;
}

/// States for our Freezable
///
/// Frozen means, we can call the `unfreeze` operation again
/// Inside frozen, our function may want to give a partial result, or may not :)
///
/// Finished means, if we call the `unfreeze` operation, it will return an error
/// Finished state should always have the result ready in it
/// if there is nothing to be returned, then it should be simply `()`
#[derive(Debug, PartialEq, Eq)]
pub enum FreezableState<T> {
    Finished(T),
    Frozen(Option<T>),
}

/// Potential errors for our Freezable
#[derive(Debug, PartialEq, Eq)]
pub enum FreezableError {
    Cancelled,
    AlreadyFinished,
}
