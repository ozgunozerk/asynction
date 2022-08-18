use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

/// Simulates the `Reactor`
///
/// Gets the events that we are awaiting on from the `Executor`, and subscribes to necessary
/// I/O events on the OS side. When the OS notifies the `Reactor` on some events/resource being ready,
/// the Reactor then notifies the `Executor` about these.
///
/// `event` and `resource` corresponds to the same thing,
/// but we will use `event` for the communication between `executor` <-> `reactor`
/// and `resource` between `reactor` <-> `OS`.
///
/// In this example, the `Reactor` may not make much sense because of all the simplifications we made
/// the `Executor` could do the job of `Reactor` as well. So why did I put `Reactor` in here?
/// Because in practice, the `Reactor` is very useful (unlike this example). And I believe this example
/// will make it much more easier to understand the real `Reactor`.
///
/// In practice, the `Reactor` will listen to the OS for the notifications,
/// and then call the `Waker.wake()` on the relevant tasks, making them ready to be polled for
/// the `Executor`. Refer to the root README for further details

pub fn start_reactor(
    event_recv: Receiver<u8>,
    awake_signal_sender: Sender<u8>,
    subscription_sender: Sender<u8>,
    notification_recv: Receiver<u8>,
) {
    let mut events = HashSet::new();

    loop {
        // try to receive the interested events (the events that executor is waiting on)
        // immediately without blocking (try_recv), because we still need to continuously
        // listen to the notifications from the OS
        if let Ok(event_id) = event_recv.try_recv() {
            // if event id is new
            if !events.contains(&event_id) {
                events.insert(event_id);
                subscription_sender
                    .send(event_id)
                    .expect("OS should always be listening");
            }
            // else is omitted, since more than 1 task may be related to the same event, it is ok
        }

        if let Ok(resource_id) = notification_recv.try_recv() {
            if !events.remove(&resource_id) {
                panic!("the completed resource is not in the Reactor's list!");
            }
            awake_signal_sender
                .send(resource_id)
                .expect("Executor should be always listening");
        }

        thread::sleep(time::Duration::from_millis(10)); // ZA WARUDO
    }
}
