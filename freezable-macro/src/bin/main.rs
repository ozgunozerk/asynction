use freezable::freeze;
use freezable_macro::freezable;

#[freezable]
fn my_func() -> u32 {
    let a = 5;
    freeze!();
    a + 3
}

fn main() {
    println!("{}", my_func());
}
