use freezable::Freezable;
#[cfg(feature = "printable_complex_example")]
use freezable::FreezableComplex;

use rand::Rng;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

/// simulates the `Executor`
///
/// calls `unfreeze()` on the tasks when they first arrive,
/// then puts them in a queue, and awaits on the relevant I/O resources to become ready
/// when these resources are ready, calls `unfreeze` on the relevant tasks
///
/// assumes the current states of the tasks given to this Executor are not:
/// Finished or Cancelled
///
/// this executor does not do any error handling for simplicity. It just ignores the errors.
pub fn start_executor(
    #[cfg(not(feature = "printable_complex_example"))] tasks: &mut [impl Freezable],
    #[cfg(feature = "printable_complex_example")] tasks: &mut [FreezableComplex],
    event_sender: Sender<u8>,
    awake_signal_recv: Receiver<u8>,
) {
    let mut rng = rand::thread_rng();
    let mut task_event_map: HashMap<usize, u8> = HashMap::new();

    #[cfg(not(feature = "printable_complex_example"))]
    {
        // call the first unfreeze() on all the tasks
        tasks.iter_mut().for_each(|task| {
            let _ = task.unfreeze();
        });
    }

    #[cfg(feature = "printable_complex_example")]
    {
        // call the first unfreeze() on all the tasks
        tasks.iter_mut().enumerate().for_each(|(id, task)| {
            let _ = task.unfreeze();
            println!("state of the task #{id}:");
            print_state(task);
        });
    }

    println!("---------");

    // for each non finished task, identify the event (I/O resource)
    // and send it to the reactor
    tasks.iter().enumerate().for_each(|(task_id, task)| {
        if !task.is_finished() {
            let event_id: u8 = rng.gen();
            println!("for the task #{task_id}, requesting the I/O resource: {event_id}");
            task_event_map.insert(task_id, event_id);
            event_sender
                .send(event_id)
                .expect("Reactor should be listening");
        }
    });

    println!("---------");

    loop {
        // the executor has nothing to do but to execute tasks
        // in order to execute tasks, we need to know the corresponding I/O
        // resources are not ready, so we better wait till we hear a message
        // from the `Reactor`
        if let Ok(resource_id) = awake_signal_recv.recv() {
            println!("The I/O resource: {resource_id}, is now ready!");

            // for each task that relies on the resource that is ready now,
            // call `unfreeze()` on them
            let mut progressing_tasks = vec![];
            task_event_map.iter().for_each(|(&task_id, &event_id)| {
                if resource_id == event_id {
                    println!("Calling unfreeze on task #{task_id}");
                    let _ = tasks[task_id].unfreeze();
                    #[cfg(feature = "printable_complex_example")]
                    {
                        println!("state of the task #{task_id}:");
                        print_state(&tasks[task_id]);
                    }
                    progressing_tasks.push(task_id);
                }
            });

            println!("---------");

            // if the task is still not finished after being progressed,
            // send another event_id to the `Reactor`, so that it can notify us later
            progressing_tasks.iter().for_each(|&task_id| {
                if !tasks[task_id].is_finished() {
                    let event_id: u8 = rng.gen();
                    println!("for the task #{task_id}, requesting the I/O resource: {event_id}");
                    task_event_map.insert(task_id, event_id);
                    event_sender
                        .send(event_id)
                        .expect("Reactor should be listening");
                }
            });

            println!("---------");

            // if all the tasks are finished :)
            if tasks.iter().all(|task| task.is_finished()) {
                println!("all tasks are finished!");
                break;
            }
        } else {
            panic!("Reactor closed the sender!");
        }
    }
}

#[cfg(feature = "printable_complex_example")]
fn print_state(task: &FreezableComplex) {
    match task {
        FreezableComplex::Chunk0(val) => println!("frozen in state: 0, with value: {val}"),
        FreezableComplex::Chunk1(val) => println!("frozen in state: 1, with value: {val}"),
        FreezableComplex::Chunk2(val1, val2) => {
            println!("frozen in state 2, with values: {val1} and {val2}")
        }
        FreezableComplex::Chunk3(val) => println!("frozen in state: 3, with value: {:?}", val),
        FreezableComplex::Finished => println!("Finished!"),
        FreezableComplex::Cancelled => println!("Cancelled"),
    }
}
