#[allow(unused_imports)]
use freezable::{freeze, Freezable, FreezableError, FreezableState};
use freezable_macro::freezable;

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

#[freezable]
fn freezable_generator_4(begin: u8) -> u8 {
    let mut next: u8 = begin;
    freeze!(next); // freezes the function, but also return the partial result
    next += 1;
    freeze!(next);
    next += 1;
    freeze!(next);
    next += 1;
    next
}

fn main() {
    let mut generator_example = freezable_generator_4::start(5);
    let mut complex_example = freezable_complex::start(5);

    println!("running the generator example!");
    let mut counter = 1;
    while let Ok(state) = generator_example.unfreeze() {
        println!("Call #{counter}:");
        match state {
            FreezableState::Finished(val) => {
                println!("the task is finished with value: {:?}", val)
            }
            FreezableState::Frozen(val) => {
                println!("the task is frozen with value: {:?}", val)
            }
        }
        counter += 1;
    }
    println!("******************************");
    println!("running the complex example!");
    let mut counter = 1;
    while let Ok(state) = complex_example.unfreeze() {
        println!("Call #{counter}:");
        match state {
            FreezableState::Finished(val) => {
                println!("the task is finished with value: {:#?}", val)
            }
            FreezableState::Frozen(val) => {
                println!("the task is frozen with value: {:#?}", val)
            }
        }
        counter += 1;
    }
}
