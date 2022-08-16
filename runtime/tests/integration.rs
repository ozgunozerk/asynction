use runtime::simulate_os;

use rand::Rng;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

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
}
