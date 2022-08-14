This library will provide the macro for turning a function to it's `freezable` version.

**Important Note!**
If you don't like to torture yourself, don't try to understand the macro code. I tried to write it as clean as I can,
but still, it turned out to be one of the ugliest codes I've ever written, due to innate ugliness of macro concept.
The important/exciting part of this project is not the macro, it is the desugared version of the code.
To see the generated code, run: ```cargo expand``` (you must have installed `cargo-expand`, if not: install it with `cargo install cargo-expand`).
To recap:
- `freezable` crate gives you an introduction on **how to freeze a function?**
- `freezable-macro` crate allows you to be lazy, and write high-level code (like you do in async world),
and converts your lazy code into the actual code that will be run. Implementing the macro for doing this job is
taking the shortcut (it is actually the compiler's job to do that, and I won't write a compiler for this project).
So, my suggestion is, just look at the expanded code of your *lazy code*, and move on with your life.


If you insist on learning how this macro works, I suggest the following:
1. read the documentation of `quote!` macro (don't spend too much time on it, it is easy).
2. look at the documentation of `syn` crate.
3. pretty-print the interesting variables with `println!("{:#?}", var)`, so that you will witness the atrocious structure of the proc-macro world.
4. after that, try to follow the source-code of the `syn` crate for the types you printed.

Repeating one more time, *I don't recommend doing that :)*

Original Code:
```rust
use freezable_macro::freezable;
use freezable::freeze;

#[freezable]
fn freezable_complex(begin: u8) -> String {
    let current_num: u8 = begin;
    freeze!(); // freezes the function, and returns no partial value
    let (num1, num2): (u8, u8) = (current_num + 1, current_num - 1);
    freeze!();
    let mut mult_str: String = (num1 * num2).to_string();
    freeze!();
    mult_str.push_str(" a random text");
    mult_str.truncate(10);
    mult_str
}
```

into ->

Generated Code:
```rust
/// State Machine for our Freezable that will run 3 chunks of code
/// first state is for initial state
/// `Chunk1`, `Chunk2`, and `Chunk3` states are for 3 chunks of code
/// and extra 2 for `Finished` and `Cancelled` states
#[allow(non_camel_case_types)]
pub enum freezable_complex {
    Chunk0(Option<u8>),
    Chunk1(Option<u8>, Option<u8>),
    Chunk2(Option<u8>, Option<u8>, Option<u8>, Option<u8>),
    Chunk3(
        Option<u8>,
        Option<u8>,
        Option<u8>,
        Option<u8>,
        Option<String>,
    ),
    Finished,
    Cancelled,
}

impl freezable_complex {
    pub fn start(begin: u8) -> Self {
        freezable_complex::Chunk0(Some(begin))
    }
}

#[allow(unused_variables)]
#[allow(unused_mut)]
impl Freezable for freezable_complex {
    type Output = String;

    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError> {
        match self {
            freezable_complex::Chunk0(begin) => {
                let mut begin = begin.take().expect("value is always present");
                let current_num: u8 = begin;
                *self = freezable_complex::Chunk1(Some(begin), Some(current_num));
                Ok(FreezableState::Frozen(None))
            }
            freezable_complex::Chunk1(begin, current_num) => {
                let mut begin = begin.take().expect("value is always present");
                let mut current_num = current_num.take().expect("value is always present");
                let (num1, num2): (u8, u8) = (current_num + 1, current_num - 1);
                *self = freezable_complex::Chunk2(
                    Some(begin),
                    Some(current_num),
                    Some(num1),
                    Some(num2),
                );
                Ok(FreezableState::Frozen(None))
            }
            freezable_complex::Chunk2(begin, current_num, num1, num2) => {
                let mut begin = begin.take().expect("value is always present");
                let mut current_num = current_num.take().expect("value is always present");
                let mut num1 = num1.take().expect("value is always present");
                let mut num2 = num2.take().expect("value is always present");
                let mut mult_str: String = (num1 * num2).to_string();
                *self = freezable_complex::Chunk3(
                    Some(begin),
                    Some(current_num),
                    Some(num1),
                    Some(num2),
                    Some(mult_str),
                );
                Ok(FreezableState::Frozen(None))
            }
            freezable_complex::Chunk3(begin, current_num, num1, num2, mult_str) => {
                let mut begin = begin.take().expect("value is always present");
                let mut current_num = current_num.take().expect("value is always present");
                let mut num1 = num1.take().expect("value is always present");
                let mut num2 = num2.take().expect("value is always present");
                let mut mult_str = mult_str.take().expect("value is always present");
                mult_str.push_str(" a random text");
                mult_str.truncate(10);
                *self = freezable_complex::Finished;
                Ok(FreezableState::Finished(mult_str))
            }
            freezable_complex::Finished => Err(FreezableError::AlreadyFinished),
            freezable_complex::Cancelled => Err(FreezableError::Cancelled),
        }
    }

    fn cancel(&mut self) {
        *self = freezable_complex::Cancelled
    }

    fn is_cancelled(&self) -> bool {
        matches!(self, FreezableComplex::Cancelled)
    }

    fn is_finished(&self) -> bool {
        matches!(self, FreezableComplex::Finished)
    }
}
```
