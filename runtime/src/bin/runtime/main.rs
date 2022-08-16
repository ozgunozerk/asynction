fn main() {
    println!("Hello, world!");

    // algorithm to simulate os:
    // have a thread listening to incoming tasks with 100 ms delay in a continuous loop
    // each task will come with an identifier
    // using this identifier, generate a random number
    // this random number will be the necessary amount of time for the completion of this task
    // we do not need to `sleep` explicitly for that necessary amount of time
    // we can just check how much time is passed
    // since we will be waiting 100ms in each loop iteration, we can just add these 100ms in a counter
    // and store the value of counter when a task arrives in a variable
    // then add the necessary time for completion to the current value of counter
    // store this time of completion in an array along with the identifier of the task
    // (array, because there might be multiple tasks)
    // for each iteration of the loop, check this array
    // if the counter is >= than any of the time values in there:
    // notify the waker about the task by sending the tasks ID via a channel

    // algorithm to simulate reactor (waker):
    // this also will be a thread on its own
    // when a new freezable is created, its identity should be available to the waker
    // so that the waker will send this identity to the OS, and wait for a signal from OS
    // this signal should be sent to the waker along with the identity of the task
    // then the waker should turn the `pending` state into `canContinue` for the task

    // for now, there won't be an executor, we will simulate the executor with a loop
    // in the loop, check for the state of each freezable task, and call `unfreeze()` on those
    // who are in `canContinue` state.
    // When all the tasks are either `finished` or `cancelled`, stop the loop
}
