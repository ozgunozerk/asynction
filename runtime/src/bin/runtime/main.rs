use runtime::runtime;

use freezable::{Freezable, FreezableGenerator4};

#[cfg(not(feature = "printable_states"))]
use freezable::{FreezableError, FreezableState};
#[cfg(not(feature = "printable_states"))]
use freezable_macro::freezable;

// LOOK BELOW (main_custom function) FOR THE CUSTOM EXAMPLE THAT YOU CAN USE YOUR OWN FUNCTION!

fn main() {
    println!(
        "Did you know you can easily create own `freezable` functions via our macro?
    It's only 5 cents per function!
    Joking, just look below on how to do it, I prepared a template for you :)"
    );
    println!("Initializing 3 async tasks...");
    println!("These 3 async tasks will produce 4 values each.");
    println!("The first one will start from 10, and produce 11, 12, 13");
    println!("The second one will start from 20, and produce 21, 22, 23");
    println!("And the third one will start from 30, and produce 31, 32, 33");
    println!(
        "\n You can track the progress of the tasks easily,
    for example: if the value at the current state is 12,
    that means the 1st task is on the 2nd state"
    );
    println!(
        "\nOne can imagine all these tasks will retrieve all these values from a website,
     or reading them from a file, so depending on the I/O resource being ready,
     the relevant task will progress to completion."
    );
    println!(
        "\nThis binary is demonstrating that we are able to stop and continue on the tasks,
    with a single thread"
    );
    println!("\nHere we go!");
    println!("--------------");

    let async_task1 = FreezableGenerator4::start(10);
    let async_task2 = FreezableGenerator4::start(20);
    let async_task3 = FreezableGenerator4::start(30);
    let mut tasks = vec![async_task1, async_task2, async_task3];

    runtime(&mut tasks);

    assert!(tasks.iter().all(|task| task.is_finished()));
}

// if you want to use your custom function, use this function instead of the actual `main`
#[cfg(not(feature = "printable_states"))]
#[allow(dead_code)]
fn main_custom() {
    println!("Running your custom functions!");
    println!("--------------");

    #[freezable]
    // simply, change the function `freezable_generator_4` below with your own function
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

    // change the `freezbale_generator_4` names bloew with your function's name
    let async_task1 = freezable_generator_4::start(10);
    let async_task2 = freezable_generator_4::start(20);
    let async_task3 = freezable_generator_4::start(30);
    let mut tasks = vec![async_task1, async_task2, async_task3];

    runtime(&mut tasks);

    assert!(tasks.iter().all(|task| task.is_finished()));
}
