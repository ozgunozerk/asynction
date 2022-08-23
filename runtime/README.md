# Introduction
`freezable` functions can be used as `generators`, `iterators`, or `async` functions.
In this example, I wanted to demonstrate how to use `freezable` functions as `async` functions, and run
several of them asynchronously.

# The Binary
## How to run the binary

- To run the application and see the whole output: `cargo run --bin runtime --features=printable_states`
- If you do not want to print the states of the tasks for each checkpoint,
run with: `cargo run --bin runtime`

Example output:
```
Here we go!
--------------
STATE OF THE TASK #0: frozen in state: 1, with value: 10
STATE OF THE TASK #1: frozen in state: 1, with value: 20
STATE OF THE TASK #2: frozen in state: 1, with value: 30
---------
for the task #0, requesting the I/O resource: 8
for the task #1, requesting the I/O resource: 41
for the task #2, requesting the I/O resource: 70
---------
The I/O resource: 41, is now ready!
Calling unfreeze on task #1
STATE OF THE TASK #1: frozen in state: 2, with value: 21
---------
for the task #1, requesting the I/O resource: 74
---------
The I/O resource: 8, is now ready!
Calling unfreeze on task #0
STATE OF THE TASK #0: frozen in state: 2, with value: 11
---------
for the task #0, requesting the I/O resource: 26
---------
The I/O resource: 26, is now ready!
Calling unfreeze on task #0
STATE OF THE TASK #0: frozen in state: 3, with value: 12
---------
for the task #0, requesting the I/O resource: 124
---------
The I/O resource: 74, is now ready!
Calling unfreeze on task #1
STATE OF THE TASK #1: frozen in state: 3, with value: 22
---------
for the task #1, requesting the I/O resource: 74
---------
The I/O resource: 70, is now ready!
Calling unfreeze on task #2
STATE OF THE TASK #2: frozen in state: 2, with value: 31
---------
for the task #2, requesting the I/O resource: 101
---------
The I/O resource: 124, is now ready!
Calling unfreeze on task #0
STATE OF THE TASK #0: Finished!
---------
---------
The I/O resource: 74, is now ready!
Calling unfreeze on task #1
STATE OF THE TASK #1: Finished!
---------
---------
The I/O resource: 101, is now ready!
Calling unfreeze on task #2
STATE OF THE TASK #2: frozen in state: 3, with value: 32
---------
for the task #2, requesting the I/O resource: 144
---------
The I/O resource: 144, is now ready!
Calling unfreeze on task #2
STATE OF THE TASK #2: Finished!
---------
---------
all tasks are finished!
```

**in the above example:**

*being awaited I/O resource queue: [8, 41, 70, 74, 26, 124, 74, 101, 144]*

*became ready I/O resource queue: [41, 8, 26, 74, 70, 124, 74, 101, 144]*

This example is a good demonstration of we were able to run interruptible functions in an asynchronous
context, and did it from scratch (without using any `async`, `yield`, or `future`)!


# The Library

This library provides 3 things:
- os-simulation
- reactor
- executor

## OS-Simulation
**why simulate, but not directly implement the communication with OS (a.k.a `mio`)?**

Recall that this project's main goal is to demystify async concepts and demonstrate functions can be
interrupted and can be continued on again from where they left off. Interacting with the OS
(Epoll for linux, Kqueue for BSD(macOS), and IOCP for Windows) is a rabbit hole on its own as mentioned
in several other places in this project, and implementing this will make this project much more complicated
than it already is. On top of that, interaction with the OS part is not directly helping us to
uncover the secrets of interruptible functions.

**how to think OS as a black-box for this project?**

If we look at the OS from the perspective of the async world, it is just a message relayer that telling us
some I/O resource we are awaiting on has become ready. In other words: we will subscribe to an event
on the OS side, then wait for some unknown time, then the OS will somehow notify us that the subscribed event
is ready now.

So, we can simulate the OS as the following:
1. we will spawn a thread (this will be `OS`)
2. we will be able to send some integers to this thread. These integers will represent the ID of the resource
3. our thread (`OS`) will wait for some random time for each resource_id (the integer we sent to it).
This random wait will simulate the resource becoming ready.
4. then this thread (`OS`) will notify `the Reactor` when the subscribed resource_id has become ready.

## The Reactor and The Executor

The reactor gets the events that we are awaiting on from the `Executor`, and subscribes to necessary
I/O events on the OS side. When the OS notifies the `Reactor` on some events/resource being ready,
the Reactor then notifies the `Executor` about these.

In practice, when the `Reactor` receives a notification from the OS about a resource being ready,
it will then call the `Waker.wake()` on the waker objects of the relevant tasks (the tasks that are waiting
for that resource), making them ready to be polled for the `Executor`
(refer to the root README for further details).

Again, to keep this project as simple as possible, I choose to abstract away this `Waker` concept as well.
In practice, these `Waker` objects are created/initialized by the `Executor`, and the `Reactor` has access
to these `Waker` objects, so it can call the `Waker.wake()` method on them when a relevant notification
arrives from the `OS`. The information of the relevant I/O resource is stored in the `Waker` object,
so `Reactor` can now which `Waker` to call when a notification arrives (this is an oversimplification).

In summary, the `Executor` initializes the `Waker`, which stores which I/O resource is related to which task.
Then, the `Reactor` calls `Waker.wake()` when it receives a notification from the OS.

To make things simpler, here is what we are going to do:
1. The `Executor` will create mapping between the tasks and the awaited I/O resource for these tasks.
Then send the ID of the necessary I/O resource to the `Reactor`. This makes sense, since `Executor`
was responsible from initializing the `Waker` object.
2. Then, the `Reactor` will listen to the notifications from the OS, and then notify
the `Executor` about these. This also makes, since the `Reactor` should be the one listening to the `OS`.
3. Then, the `Executor` will use the mapping between I/O resources and tasks, and call `unfreeze()` on the
relevant tasks.

*note: in practice, `Reactor` is awaking the tasks using `Waker.wake()`. Then the executor polls the*
*awakened tasks. In this example, however, there are no waking mechanisms. The executor will know which*
*task to poll directly.*

*another note: you may realize that, `Reactor` is not that useful in our scenario. The `Executor` could*
*have directly subscribed to the `OS`, and eliminate the `Reactor` from the equation. In fact, this*
*is another strategy in practice too. The reason that I included a `Reactor` is: it shall be easier to*
*understand more complex mechanisms where `Reactor` is included after understanding this example.*


### Caveat about the `Executor`
The tasks given to the `Executor` needs to be the same type. This restriction can be eliminated
with `Vec<Box<dyn Trait>>`. But I did not like the idea of storing all the async tasks in the heap
for this project. If we are to utilize the heap for massive memory allocation, we could as well store the
necessary information related to each state in the heap, and have a much simpler state machine structire
as well.

Not using the heap is also good for the embedded systems (some machines do not have heap, or very
limited heap). I want this example to be as simple as possible. So that it would serve as a stepping stone
for what can be further done.

Another strategy for eliminating requirement of the tasks being the same type, would be using
macros (take a look at `select` from tokio: https://docs.rs/tokio/latest/tokio/macro.select.html).

*Note that macros are utilized in practice (in tokio), instead of heaps :)*

The same structure you see in the link can be easily applied to our `Executor` as well,
but again for simplicity, I've chosen to keep it as is. In the end, the `Executor`'s aim is prove that
concurrently running some interruptible tasks in a single thread is possible. And it is accomplishing
this goal :)
