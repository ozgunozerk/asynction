use runtime::{runtime, simulate_os, start_executor, start_reactor};

use freezable::FreezableGenerator4;
use freezable::{Freezable, FreezableError, FreezableState};
use freezable_macro::freezable;
use rand::Rng;
use std::collections::HashSet;
use std::sync::mpsc;
use std::{thread, time};

#[test]
fn os_simulation() {
    let mut rng = rand::thread_rng();

    let (subscription_sender, subscription_recv) = mpsc::channel();
    let (notification_sender, notification_recv) = mpsc::channel();
    thread::spawn(|| simulate_os(notification_sender, subscription_recv));

    // 5 resources to be subscribed
    let mut resource_set = HashSet::new();
    for _ in 0..5 {
        resource_set.insert(rng.gen::<u8>());
    }

    // requesting 5 random resources with 100 millisecond interval
    for &resource in &resource_set {
        subscription_sender.send(resource).unwrap();
    }

    for _ in 0..resource_set.len() {
        let resource = notification_recv.recv().unwrap();
        assert!(resource_set.remove(&resource));
    }

    assert!(resource_set.is_empty())
}

#[test]
fn reactor_and_os_simulation() {
    let mut rng = rand::thread_rng();

    let (subscription_sender, subscription_recv) = mpsc::channel();
    let (notification_sender, notification_recv) = mpsc::channel();
    let (event_sender, event_recv) = mpsc::channel();
    let (awake_signal_sender, awake_signal_recv) = mpsc::channel();

    thread::spawn(|| simulate_os(notification_sender, subscription_recv));
    thread::spawn(|| {
        start_reactor(
            event_recv,
            awake_signal_sender,
            subscription_sender,
            notification_recv,
        )
    });

    // there are 5 events that we are interested in (we are awaiting on these 5 I/O resources)
    let mut event_set = HashSet::new();
    for _ in 0..5 {
        event_set.insert(rng.gen::<u8>());
    }

    // telling to the `Reactor` that we are interested in 5 events with 100 millisecond interval
    for &event in &event_set {
        event_sender.send(event).unwrap();
        thread::sleep(time::Duration::from_millis(100)); // ZA WARUDO
    }

    for _ in 0..event_set.len() {
        let event = awake_signal_recv.recv().unwrap();
        assert!(event_set.remove(&event));
    }

    assert!(event_set.is_empty())
}

#[test]
fn executor_reactor_and_os_simulation() {
    let async_task1 = FreezableGenerator4::start(3);
    let async_task2 = FreezableGenerator4::start(7);
    let async_task3 = FreezableGenerator4::start(10);
    let mut tasks = vec![async_task1, async_task2, async_task3];

    let (subscription_sender, subscription_recv) = mpsc::channel();
    let (notification_sender, notification_recv) = mpsc::channel();
    let (event_sender, event_recv) = mpsc::channel();
    let (awake_signal_sender, awake_signal_recv) = mpsc::channel();

    thread::spawn(|| simulate_os(notification_sender, subscription_recv));
    thread::spawn(|| {
        start_reactor(
            event_recv,
            awake_signal_sender,
            subscription_sender,
            notification_recv,
        )
    });

    start_executor(&mut tasks, event_sender, awake_signal_recv);

    assert!(tasks.iter().all(|task| task.is_finished()));
}

#[test]
fn runtime_with_macro() {
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

    let async_task1 = freezable_generator_4::start(3);
    let async_task2 = freezable_generator_4::start(7);
    let async_task3 = freezable_generator_4::start(12);
    let mut tasks = vec![async_task1, async_task2, async_task3];

    runtime(&mut tasks);

    assert!(tasks.iter().all(|task| task.is_finished()));
}
