use freezable_desugared::{DesugaredFreezable, FreezableComplex, FreezableGenerator4};

fn generator_example() {
    println!("- 1st Scenario -> Calling `unfreeze` on the Freezable");
    println!("*************");
    let mut my_iter = FreezableGenerator4::start(5);
    println!("First call: {:?}", my_iter.unfreeze());
    println!("Second call: {:?}", my_iter.unfreeze());
    println!("Third call: {:?}", my_iter.unfreeze());
    println!("Fourth call: {:?}", my_iter.unfreeze());
    println!("Fifth call: {:?}", my_iter.unfreeze());
    println!("Sixth call: {:?}", my_iter.unfreeze());

    println!();

    println!("- 2nd Scenario -> Calling `cancel` in a random state");
    println!("*************");
    let mut my_second_iter = FreezableGenerator4::start(10);
    println!("First call: {:?}", my_second_iter.unfreeze());
    println!("Second call: {:?}", my_second_iter.unfreeze());
    println!("Canceling the Freezable!");
    my_second_iter.cancel();
    println!("Third call: {:?}", my_second_iter.unfreeze());
    println!("Fourth call: {:?}", my_second_iter.unfreeze());
}

fn complex_example() {
    println!("- 1st Scenario -> Calling `unfreeze` on the Freezable");
    println!("*************");
    let mut my_iter = FreezableComplex::start(5);
    println!("First call: {:?}", my_iter.unfreeze());
    println!("Second call: {:?}", my_iter.unfreeze());
    println!("Third call: {:?}", my_iter.unfreeze());
    println!("Fourth call: {:?}", my_iter.unfreeze());
    println!("Fifth call: {:?}", my_iter.unfreeze());
    println!("Sixth call: {:?}", my_iter.unfreeze());

    println!();

    println!("- 2nd Scenario -> Calling `cancel` in a random state");
    println!("*************");
    let mut my_second_iter = FreezableComplex::start(10);
    println!("First call: {:?}", my_second_iter.unfreeze());
    println!("Second call: {:?}", my_second_iter.unfreeze());
    println!("Canceling the Freezable!");
    my_second_iter.cancel();
    println!("Third call: {:?}", my_second_iter.unfreeze());
    println!("Fourth call: {:?}", my_second_iter.unfreeze());
}

fn main() {
    println!("RUNNING THE GENERATOR EXAMPLE:");
    generator_example();

    println!();
    println!("RUNNING THE COMPLEX EXAMPLE:");
    complex_example();
}
