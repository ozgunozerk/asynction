use freezable::{Freezable, FreezableError, FreezableState};

/// State Machine for our Freezable that will generate 4 numbers
/// first 4 states are for generating 4 numbers
/// and extra 2 for `Finished` and `Cancelled` states
enum Freezable4 {
    Chunk0(u8),
    Chunk1(u8),
    Chunk2(u8),
    Chunk3(u8),
    Finished,
    Cancelled,
}

impl Freezable4 {
    /// initializes the Freezable
    fn start(begin: u8) -> Self {
        Freezable4::Chunk0(begin)
    }
}

impl Freezable for Freezable4 {
    type Output = u8;

    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError> {
        match self {
            Freezable4::Chunk0(num) => {
                let current_num = *num;
                *self = Freezable4::Chunk1(current_num + 1);
                Ok(FreezableState::Frozen(current_num))
            }
            Freezable4::Chunk1(num) => {
                let current_num = *num;
                *self = Freezable4::Chunk2(current_num + 1);
                Ok(FreezableState::Frozen(current_num))
            }
            Freezable4::Chunk2(num) => {
                let current_num = *num;
                *self = Freezable4::Chunk3(current_num + 1);
                Ok(FreezableState::Frozen(current_num))
            }
            Freezable4::Chunk3(num) => {
                let current_num = *num;
                *self = Freezable4::Finished;
                Ok(FreezableState::Finished(current_num))
            }
            Freezable4::Finished => Err(FreezableError::AlreadyFinished),
            Freezable4::Cancelled => Err(FreezableError::Cancelled),
        }
    }

    fn cancel(&mut self) {
        *self = Freezable4::Cancelled
    }

    fn is_cancelled(&self) -> bool {
        return matches!(self, Freezable4::Cancelled);
    }
}

fn main() {
    println!("- 1st Scenario -> Calling `unfreeze` on the Freezable");
    println!("*************");
    let mut my_iter = Freezable4::start(5);
    println!("First call: {:?}", my_iter.unfreeze());
    println!("Second call: {:?}", my_iter.unfreeze());
    println!("Third call: {:?}", my_iter.unfreeze());
    println!("Fourth call: {:?}", my_iter.unfreeze());
    println!("Fifth call: {:?}", my_iter.unfreeze());
    println!("Sixth call: {:?}", my_iter.unfreeze());

    println!();

    println!("- 2nd Scenario -> Calling `cancel` in a random state");
    println!("*************");
    let mut my_second_iter = Freezable4::start(10);
    println!("First call: {:?}", my_second_iter.unfreeze());
    println!("Second call: {:?}", my_second_iter.unfreeze());
    println!("Canceling the Freezable!");
    my_second_iter.cancel();
    println!("Third call: {:?}", my_second_iter.unfreeze());
    println!("Fourth call: {:?}", my_second_iter.unfreeze());
}
