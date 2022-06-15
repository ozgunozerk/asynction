//! An example of a `DesugaredFreezable` function:
//! it will do meaningless operations
//! just to show that this concept can be used for any purpose
//! check out `freezable_generator_4.rs` for a slightly less complex, and more straight-forward example
//!
//! Original Code:
//! ```ignore
//! fn freezable_complex(begin: usize) -> String {
//!     let current_num = begin;
//!     freeze();  // freezes the function, and returns no partial value
//!
//!     let (num1, num2) = (current_num + 1, current_num - 1);
//!     freeze();
//!
//!     let mult_str = (num1 * num2).to_string();
//!     freeze();
//!
//!     mult_str.push_str(" a random text");
//!     mult_str.truncate(10);
//!     mult_str
//! }
//! ```
//!
//! See below for the desugared version of the above code

use crate::{DesugaredFreezable, FreezableError, FreezableState};

pub enum FreezableComplex {
    Chunk0(u8),
    Chunk1(u8),
    Chunk2(u8, u8),
    Chunk3(String),
    Finished,
    Cancelled,
}
impl FreezableComplex {
    /// initializes the Freezable
    pub fn start(begin: u8) -> Self {
        FreezableComplex::Chunk0(begin)
    }
}
impl DesugaredFreezable for FreezableComplex {
    type Output = String;
    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError> {
        match self {
            FreezableComplex::Chunk0(num) => {
                let current_num = *num;
                *self = FreezableComplex::Chunk1(current_num);
                Ok(FreezableState::Frozen(None))
            }
            FreezableComplex::Chunk1(num) => {
                let (current_num1, current_num2) = (*num + 1, *num - 1);
                *self = FreezableComplex::Chunk2(current_num1, current_num2);
                Ok(FreezableState::Frozen(None))
            }
            FreezableComplex::Chunk2(num1, num2) => {
                let (current_num1, current_num2) = (*num1, *num2);
                let result = (current_num1 * current_num2).to_string();
                *self = FreezableComplex::Chunk3(result);
                Ok(FreezableState::Frozen(None))
            }
            FreezableComplex::Chunk3(result) => {
                let mut current_result = result.clone();
                current_result.push_str(" a random text");
                current_result.truncate(10);
                *self = FreezableComplex::Finished;
                Ok(FreezableState::Finished(current_result))
            }
            FreezableComplex::Finished => Err(FreezableError::AlreadyFinished),
            FreezableComplex::Cancelled => Err(FreezableError::Cancelled),
        }
    }
    fn cancel(&mut self) {
        *self = FreezableComplex::Cancelled
    }
    fn is_cancelled(&self) -> bool {
        return matches!(self, FreezableComplex::Cancelled);
    }
}
