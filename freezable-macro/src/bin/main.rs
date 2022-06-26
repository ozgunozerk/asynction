// TODO: try to remove this when macro is finalized
// right now, since we are removing the `freeze();` from the function, clippy is warning us about unused import
#[allow(unused_imports)]
use freezable::freeze;
use freezable_macro::freezable;

#[freezable]
fn my_func() -> u32 {
    let a = 5;
    freeze();
    a + 3
}

fn main() {
    println!("{}", my_func());
}
