#[allow(unused_imports)]
use freezable::freeze;
use freezable_macro::freezable;

#[freezable]
fn freezable_complex(begin: u8) -> String {
    let current_num: u8 = begin;
    freeze!(); // freezes the function, and returns no partial value
    let (num1, num2): (u8, u8) = (current_num + 1, current_num - 1);
    freeze!();
    let mut mult_str: String = (num1 * num2).to_string();
    freeze!(mult_str);
    mult_str.push_str(" a random text");
    mult_str.truncate(10);
}

fn main() {
    println!("hey");
}
