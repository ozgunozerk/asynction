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

#[test]
fn complex_cancel_test() {
    let mut complex_5 = freezable_complex::start(5);
    let mut states = vec![];
    while !complex_5.is_finished() {
        states
            .iter_mut()
            .for_each(|instance: &mut freezable_complex| {
                let _ = instance.unfreeze();
            });
        states.push(freezable_complex::start(5));
        let _ = complex_5.unfreeze();
    }
    states
        .iter_mut()
        .for_each(|instance: &mut freezable_complex| {
            instance.cancel();
            assert!(instance.is_cancelled());
        });
    assert!(complex_5.is_finished());
}

#[test]
fn complex_is_finished_test() {
    let mut complex_5 = freezable_complex::start(5);
    let mut states = vec![];
    while !complex_5.is_finished() {
        states
            .iter_mut()
            .for_each(|instance: &mut freezable_complex| {
                let _ = instance.unfreeze();
            });
        states.push(freezable_complex::start(5));
        let _ = complex_5.unfreeze();
    }
    states
        .iter_mut()
        .for_each(|instance: &mut freezable_complex| {
            instance.cancel();
            assert_eq!(instance.is_finished(), false);
        });
    assert!(complex_5.is_finished());
}

#[test]
fn complex_unfreeze_test() {
    let mut complex_5 = freezable_complex::start(5);
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(
        complex_5.unfreeze(),
        Ok(FreezableState::Finished("24 a rando".to_string()))
    );
    assert_eq!(complex_5.unfreeze(), Err(FreezableError::AlreadyFinished));
}

#[test]
fn complex_unfreeze_after_cancel_test() {
    let mut complex_5 = freezable_complex::start(5);
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    assert_eq!(complex_5.unfreeze(), Ok(FreezableState::Frozen(None)));
    complex_5.cancel();
    assert_eq!(complex_5.unfreeze(), Err(FreezableError::Cancelled));
    assert_eq!(complex_5.unfreeze(), Err(FreezableError::Cancelled));
}

#[test]
fn generator_cancel_test() {
    let mut generator_5 = freezable_generator_4::start(5);
    let mut states = vec![];
    while !generator_5.is_finished() {
        states
            .iter_mut()
            .for_each(|instance: &mut freezable_generator_4| {
                let _ = instance.unfreeze();
            });
        states.push(freezable_generator_4::start(5));
        let _ = generator_5.unfreeze();
    }
    states
        .iter_mut()
        .for_each(|instance: &mut freezable_generator_4| {
            instance.cancel();
            assert!(instance.is_cancelled());
        });
    assert!(generator_5.is_finished());
}

#[test]
fn generator_is_finished_test() {
    let mut generator_5 = freezable_generator_4::start(5);
    let mut states = vec![];
    while !generator_5.is_finished() {
        states
            .iter_mut()
            .for_each(|instance: &mut freezable_generator_4| {
                let _ = instance.unfreeze();
            });
        states.push(freezable_generator_4::start(5));
        let _ = generator_5.unfreeze();
    }
    states
        .iter_mut()
        .for_each(|instance: &mut freezable_generator_4| {
            instance.cancel();
            assert_eq!(instance.is_finished(), false);
        });
    assert!(generator_5.is_finished());
}

#[test]
fn generator_unfreeze_test() {
    let mut generator_5 = freezable_generator_4::start(5);
    assert_eq!(generator_5.unfreeze(), Ok(FreezableState::Frozen(Some(5))));
    assert_eq!(generator_5.unfreeze(), Ok(FreezableState::Frozen(Some(6))));
    assert_eq!(generator_5.unfreeze(), Ok(FreezableState::Frozen(Some(7))));
    assert_eq!(generator_5.unfreeze(), Ok(FreezableState::Finished(8)));
    assert_eq!(generator_5.unfreeze(), Err(FreezableError::AlreadyFinished));
}

#[test]
fn generator_unfreeze_after_cancel_test() {
    let mut generator_5 = freezable_generator_4::start(5);
    assert_eq!(generator_5.unfreeze(), Ok(FreezableState::Frozen(Some(5))));
    assert_eq!(generator_5.unfreeze(), Ok(FreezableState::Frozen(Some(6))));
    generator_5.cancel();
    assert_eq!(generator_5.unfreeze(), Err(FreezableError::Cancelled));
    assert_eq!(generator_5.unfreeze(), Err(FreezableError::Cancelled));
}
