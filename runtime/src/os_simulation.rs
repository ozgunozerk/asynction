use rand::Rng;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

/// Simulates the os via waiting for a random amount of time for the requested resource,
/// then notifies the subscriber on the awaited resource.
pub fn simulate_os(notification_sender: Sender<u8>, subscription_recv: Receiver<u8>) {
    let mut rng = rand::thread_rng();
    let mut current_turn = 0;
    let mut resource_map = HashMap::<u8, u8>::new();

    loop {
        // try to receive message immediately without blocking (try_recv)
        // because there may not be any message, and the OS needs to keep track of the
        // events in the meantime. So it will quickly listen for a new subscription,
        // and move on to its own things, the new subscription request will be handled in the next turn
        // that's why, keep the loop short
        if let Ok(resource_id) = subscription_recv.try_recv() {
            // if resource id is new
            resource_map.entry(resource_id).or_insert_with(|| {
                let turns: u8 = rng.gen_range(1..10); // how many turns (100 milliseconds) does this resource take to be ready
                current_turn + turns
            });
        }
        // for each resource that is ready, send the notification
        let mut to_be_removed: Vec<u8> = vec![];
        resource_map.iter().for_each(|(&resource, &turn)| {
            if current_turn == turn {
                to_be_removed.push(resource)
            }
        });
        // can't mutate the resource map in the above iteration, hence this is necessary
        to_be_removed.iter().for_each(|&resource| {
            resource_map.remove(&resource);
            if notification_sender.send(resource).is_err() {
                panic!("The Reactor is not listening");
            }
        });

        thread::sleep(time::Duration::from_millis(100)); // ZA WARUDO
        current_turn += 1;
    }
}
