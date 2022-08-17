use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

/// simulates a Reactor
///
/// gets the events that we are awaiting on from the `Executor`, and subscribes to necessary
/// I/O events on the OS side. When the OS notifies the `Reactor` on some events/resource being ready,
/// the Reactor then sends IDs of the corresponding ready tasks to the Executor
///
/// `event` and `resource` corresponds to the same thing,
/// but we will use `event` for the communication between `executor` <-> `reactor`
/// and `resource` between `reactor` <-> `OS`
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

        thread::sleep(time::Duration::from_millis(100)); // ZA WARUDO
    }
}
