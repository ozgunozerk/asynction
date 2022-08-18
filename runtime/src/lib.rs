mod executor;
mod os_simulation;
mod reactor;

pub use executor::start_executor;
pub use os_simulation::simulate_os;
pub use reactor::start_reactor;

#[cfg(feature = "printable_states")]
use freezable::FreezableGenerator4;

use freezable::Freezable;
use std::sync::mpsc;
use std::thread;

/// Runs the given `freezable` tasks to completion asynchronously
///
/// Uses the `Executor`, `Reactor`, and the `simulate_os` for that
///
/// you can create your custom `freezable` tasks via using the `freezable-macro`,
/// and supply them to `runtime` via the `tasks` argument, and have the most fun time of your life!
pub fn runtime(
    #[cfg(not(feature = "printable_states"))] tasks: &mut [impl Freezable],
    #[cfg(feature = "printable_states")] tasks: &mut [FreezableGenerator4],
) {
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

    start_executor(tasks, event_sender, awake_signal_recv);

    assert!(tasks.iter().all(|task| task.is_finished()));
}
