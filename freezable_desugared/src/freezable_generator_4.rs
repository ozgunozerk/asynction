//! An example of a `DesugaredFreezable` function:
//! it will generate 4 numbers in a sequence,
//! but will freeze itself after generating each number
//!
//! Original Code:
//! ```ignore
//! fn freezable_generator_4(begin: usize) -> usize {
//!     let mut next = begin;
//!     freeze(next);  // freezes the function, but also return the partial result
//!
//!     next += 1;
//!     freeze(next);
//!
//!     next += 1;
//!     freeze(next);
//!
//!     next += 1;
//!     next
//! }
//! ```
//!
//! See below for the desugared version of the above code

use crate::{DesugaredFreezable, FreezableError, FreezableState};

/// State Machine for our Freezable that will generate 4 numbers
/// first 4 states are for generating 4 numbers
/// and extra 2 for `Finished` and `Cancelled` states
pub enum FreezableGenerator4 {
    Chunk0(u8),
    Chunk1(u8),
    Chunk2(u8),
    Chunk3(u8),
    Finished,
    Cancelled,
}

impl FreezableGenerator4 {
    /// initializes the Freezable
    pub fn start(begin: u8) -> Self {
        FreezableGenerator4::Chunk0(begin)
    }
}

impl DesugaredFreezable for FreezableGenerator4 {
    type Output = u8;

    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError> {
        match self {
            FreezableGenerator4::Chunk0(num) => {
                let current_num = *num;
                *self = FreezableGenerator4::Chunk1(current_num);
                Ok(FreezableState::Frozen(Some(current_num)))
            }
            FreezableGenerator4::Chunk1(num) => {
                let current_num = *num;
                *self = FreezableGenerator4::Chunk2(current_num + 1);
                Ok(FreezableState::Frozen(Some(current_num + 1)))
            }
            FreezableGenerator4::Chunk2(num) => {
                let current_num = *num;
                *self = FreezableGenerator4::Chunk3(current_num + 1);
                Ok(FreezableState::Frozen(Some(current_num + 1)))
            }
            FreezableGenerator4::Chunk3(num) => {
                let current_num = *num;
                *self = FreezableGenerator4::Finished;
                Ok(FreezableState::Finished(current_num + 1))
            }
            FreezableGenerator4::Finished => Err(FreezableError::AlreadyFinished),
            FreezableGenerator4::Cancelled => Err(FreezableError::Cancelled),
        }
    }

    fn cancel(&mut self) {
        *self = FreezableGenerator4::Cancelled
    }

    fn is_cancelled(&self) -> bool {
        return matches!(self, FreezableGenerator4::Cancelled);
    }
}
