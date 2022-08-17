use runtime::simulate_os;
use runtime::start_executor;
use runtime::start_reactor;

use freezable::Freezable;
use freezable::FreezableComplex;
use std::sync::mpsc;
use std::thread;

fn main() {
    #[cfg(feature = "printable_complex_example")]
    {
        let mut async_task1 = FreezableComplex::start(3);
        let _ = async_task1.unfreeze();
        let async_task2 = FreezableComplex::start(7);
        let async_task3 = FreezableComplex::start(12);
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
}
