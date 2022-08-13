use freezable::{Freezable, FreezableComplex, FreezableGenerator4};

fn call_unfreeze<T>(freezable: &mut T)
where
    T: Freezable,
{
    println!("- calling `unfreeze` on the Freezable");
    let mut counter = 1;
    while let Ok(state) = freezable.unfreeze() {
        println!("Call #{counter}:");
        match state {
            freezable::FreezableState::Finished(_) => println!("the task is finished"),
            freezable::FreezableState::Frozen(_) => println!("the task is frozen"),
        }
        counter += 1;
    }
}

fn call_cancel_randomly<T>(freezable: &mut T)
where
    T: Freezable,
{
    println!("- calling `cancel` in a random state");
    println!("Call #1: {:?}", freezable.unfreeze());
    println!("Call #2: {:?}", freezable.unfreeze());
    println!("Canceling the Freezable!");
    freezable.cancel();
    println!("Call #3: {:?}", freezable.unfreeze());
    println!("Call #4: {:?}", freezable.unfreeze());
}

fn generator_example() {
    let mut generator_5 = FreezableGenerator4::start(5);
    call_unfreeze(&mut generator_5);
    println!("********");
    let mut generator_10 = FreezableGenerator4::start(5);
    call_cancel_randomly(&mut generator_10);
}

fn complex_example() {
    let mut complex_5 = FreezableComplex::start(5);
    call_unfreeze(&mut complex_5);
    println!("********");
    let mut complex_10 = FreezableComplex::start(5);
    call_cancel_randomly(&mut complex_10);
}

fn main() {
    println!("RUNNING THE GENERATOR EXAMPLE:");
    generator_example();

    println!();

    println!("RUNNING THE COMPLEX EXAMPLE:");
    complex_example();
}
