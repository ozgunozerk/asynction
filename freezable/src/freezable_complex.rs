//! An example of a `Freezable` function:
//! it will do meaningless operations
//! just to show that this concept can be used for any purpose
//! check out `freezable_generator_4.rs` for a slightly less complex, and more straight-forward example
//!
//! Original Code:
//! ```ignore
//! fn freezable_complex(begin: u8) -> String {
//!     let current_num = begin;
//!     freeze!();  // freezes the function, and returns no partial value
//!
//!     let (num1, num2) = (current_num + 1, current_num - 1);
//!     freeze!();
//!
//!     let mult_str = (num1 * num2).to_string();
//!     freeze!();
//!
//!     mult_str.push_str(" a random text");
//!     mult_str.truncate(10);
//!     mult_str
//! }
//! ```
//!
//! See below for the desugared version of the above code

use crate::{Freezable, FreezableError, FreezableState};

/// State Machine for our Freezable that will run 3 chunks of code
/// first state is for initial state
/// `Chunk1`, `Chunk2`, and `Chunk3` states are for 3 chunks of code
/// and extra 2 for `Finished` and `Cancelled` states
pub enum FreezableComplex {
    Chunk0(u8),
    Chunk1(u8),
    Chunk2(u8, u8),
    Chunk3(Option<String>), // `Option` is used as a trick to get the ownership of the `String`,
    // to eliminate unnecessary `clone()` calls
    // an alternative would be `mem::take()`, but then we would require the inner type to implement `default`
    // which may not always be the case. `mem::replace()` is making the code more complex
    Finished,
    Cancelled,
}
impl FreezableComplex {
    /// initializes the Freezable
    pub fn start(begin: u8) -> Self {
        FreezableComplex::Chunk0(begin)
    }
}
impl Freezable for FreezableComplex {
    type Output = String;

    #[allow(unused_mut)]
    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError> {
        match self {
            FreezableComplex::Chunk0(begin) => {
                let current_num = *begin;
                *self = FreezableComplex::Chunk1(current_num);
                Ok(FreezableState::Frozen(None))
            }
            FreezableComplex::Chunk1(current_num) => {
                let (num1, num2) = (*current_num + 1, *current_num - 1);
                *self = FreezableComplex::Chunk2(num1, num2);
                Ok(FreezableState::Frozen(None))
            }
            FreezableComplex::Chunk2(num1, num2) => {
                let mult_str = (*num1 * *num2).to_string();
                *self = FreezableComplex::Chunk3(Some(mult_str));
                Ok(FreezableState::Frozen(None))
            }
            FreezableComplex::Chunk3(mult_str) => {
                let mult_str = mult_str
                    .take()
                    .expect("macro always puts value in the option");
                let mut result = mult_str;
                result.push_str(" a random text");
                result.truncate(10);
                *self = FreezableComplex::Finished;
                Ok(FreezableState::Finished(result))
            }
            FreezableComplex::Finished => Err(FreezableError::AlreadyFinished),
            FreezableComplex::Cancelled => Err(FreezableError::Cancelled),
        }
    }

    fn cancel(&mut self) {
        *self = FreezableComplex::Cancelled
    }

    fn is_cancelled(&self) -> bool {
        matches!(self, FreezableComplex::Cancelled)
    }

    fn is_finished(&self) -> bool {
        matches!(self, FreezableComplex::Finished)
    }
}

#[test]
fn cancel_test() {
    let mut complex_5 = FreezableComplex::start(5);
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert!(!complex_5.is_cancelled());
    complex_5.cancel();
    assert!(complex_5.is_cancelled());
}

#[test]
fn is_finished_test() {
    let mut complex_5 = FreezableComplex::start(5);
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(
        complex_5.unfreeze(),
        Ok(FreezableState::Finished("24 a rando".to_string()))
    );
    assert!(complex_5.is_finished());
}

#[test]
fn unfreeze_test() {
    let mut complex_5 = FreezableComplex::start(5);
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(
        complex_5.unfreeze(),
        Ok(FreezableState::Finished("24 a rando".to_string()))
    );
    assert_eq!(complex_5.unfreeze(), Err(FreezableError::AlreadyFinished));
}

#[test]
fn unfreeze_after_cancel_test() {
    let mut complex_5 = FreezableComplex::start(5);
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    complex_5.cancel();
    assert_eq!(complex_5.unfreeze(), Err(FreezableError::Cancelled));
    assert_eq!(complex_5.unfreeze(), Err(FreezableError::Cancelled));
}
