/// Freezable trait.
/// Generates the next item, and freezes itself.
/// Have the methods: `unfreeze`, `cancel` and `is_cancelled`
pub trait Freezable {
    type Output;

    /// should generate the next item in the sequence, then it will freeze itself again
    fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError>;

    /// should cancel the Freezable, makes it impossible to call `unfreeze` again
    fn cancel(&mut self);

    /// checks whether the Freezable is cancelled
    fn is_cancelled(&self) -> bool;
}

#[derive(Debug)]
/// States for our Freezable
/// Frozen means, we can call the `unfreeze` operation again
/// Finished means, if we call the `unfreeze` operation, it will return an error
pub enum FreezableState<T> {
    Finished(T),
    Frozen(T),
}

/// Potential errors for our Freezable
#[derive(Debug)]
pub enum FreezableError {
    Cancelled,
    AlreadyFinished,
}
