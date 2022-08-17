# Asynction

This repository is for revealing the magic of the `async` functions.
It tries to uncover every single secret of the `async` concept.

If you are thinking that:
***There are already some async libraries out there, that is working pretty well. What is the purpose of this one?***

The answer is: this library is not for re-implementing the async world. It is to show what is going on
behind the scenes in the simplest way possible. This project won't be a 3rd party library. It is a working
demo of the async world, including all the concepts. Since the goal is to demystify all the concepts
related to the async world, we won't be using any 3rd party library, and also nothing magical from the
standart library. There won't be any `tokio`, `mio`, `future`, `async`...
We are only allowed to use basic synchronous code in this project.

I prepared a short summary of what is happening when you use the `async` keyword, and how your function is
actually put to sleep and is awaken again. Considering that I've watched 15+ hours of video, and read countless pages
of articles about it, here is a very short summary (believe me, it is the shortest you can
find that will answer your questions about async) of what is actually happening, and what should we do to replicate it:

<br>
<br>

## The Summary
<details><summary>Click to expand!</summary>

### Async

***The Main problem:*** *we have some I/O related tasks in our hand (network, file read, etc.).**
**And we don’t want our code to sit idly while waiting for this I/O task to finish.**
**It should be able to continue doing some other work while waiting for the result of the I/O task at hand.**

There are 3 approaches for that (Carl Fredrik Samson explained it better than I can, so quoting from his [book](https://cfsamson.github.io/book-exploring-async-basics/5_strategies_for_handling_io.html)):

>
>
>
> ## 1. Using OS threads
>
> Now one way of accomplishing this is letting the OS take care of everything for us.
> We do this by simply spawning a new OS thread for each task we want to accomplish and write code
> like we normally would.
>
> **Pros:**
>
> - Simple
> - Easy to code
> - Reasonably performant
> - You get parallelism for free
>
> **Cons:**
>
> - OS level threads come with a rather large stack. If you have many tasks waiting simultaneously
> (like you would in a web-server under heavy load) you'll run out of memory pretty soon.
> - There are a lot of syscalls involved. This can be pretty costly when the number of tasks is high.
> - The OS has many things it needs to handle. It might not switch back to your thread as fast as you'd wish
> - The OS doesn't know which tasks to prioritize, and you might want to give some tasks a higher priority
> than others.
>
> ## 2. Green threads
>
> Another common way of handling this is green threads. Languages like Go uses this to great success.
> In many ways this is similar to what the OS does but the runtime can be better adjusted and suited to
> your specific needs.
>
> **Pros:**
>
> - Simple to use for the user. The code will look like it does when using OS threads
> - Reasonably performant
> - Abundant memory usage is less of a problem
> - You are in full control over how threads are scheduled and if you want you can prioritize them differently.
>
> **Cons:**
>
> - You need a runtime, and by having that you are duplicating part of the work the OS already does.
> The runtime will have a cost which in some cases can be substantial.
> - Can be difficult to implement in a flexible way to handle a wide variety of tasks
>
> ## 3. Poll based event loops supported by the OS
>
> The third way we're covering today is the one that most closely matches an ideal solution.
> In this solution we register an interest in an event, and then let the OS tell us when it's ready.
>
> The way this works is that we tell the OS that we're interested in knowing when data is arriving
> for us on the network card. The network card issues an interrupt when something has happened,
> in which case the driver lets the OS know that the data is ready.
>
> Now, we still need a way to "suspend" many tasks while waiting, and this is where Node's "runtime"
> or Rust's Futures come in to play.
>
> **Pros:**
>
> - Close to optimal resource utilization
> - It's very efficient
> - Gives us the maximum amount of flexibility to decide how to handle the events that occurs
>
> **Cons:**
>
> - Different operating systems have different ways of handling these kind of queues. Some of them are
> difficult to reconcile with each other. Some operating systems have limitations on what I/O operations
> support this method.
> - Great flexibility comes with a good deal of complexity
> - Difficult to write an ergonomic API with an abstraction layer that accounts for the differences
> between the operating systems without introducing unwanted costs.
> - Only solves part of the problem—the programmer still needs a strategy for suspending tasks that are
> waiting.
>
> Rust's async story is modeled around option 3, and one of the reasons it has taken a long time is
> related to the *cons* of this method and choosing a way to model how tasks should be suspended.
> Rust's Futures model a task as a [State Machine](https://en.wikipedia.org/wiki/Finite-state_machine) 
> where a suspension point represents a `state.`


1. We need functions that are stoppable/freezable. Which can be implemented as state machines.
    - We are not using OS threads, or green-threads in this example. We are going with the 3rd option listed above.
    - In other words: we should turn our `async` functions, and divide it into states for each waiting/blocking
    point. [`freezable`](https://github.com/ozgunozerk/asynction/tree/main/freezable) is the most simple and
    through example of this imho. But here is another great example: [https://cfsamson.github.io/books-futures-explained/4_generators_async_await.htm](https://cfsamson.github.io/books-futures-explained/4_generators_async_await.html)
    - to achieve this, we can’t expect our users to write their own states for each function.
    Because we don’t write state machines for `async` programming. In reality, our `async` code indeed
    turns into state machines behind the scenes. Who does that? The compiler! I won’t write a compiler
    for this project, but I wrote a macro which will turn your weakling function into a giga-chad state
    machine. The code for the macro itself is ugly due to the innate nature of macro concept, but you don’t
    have to understand it. Act like it is the compiler that doing the work for you. If you are curious,
    here is the [link to the macro](https://github.com/ozgunozerk/asynction/tree/main/freezable-macro)
    - Bonus point: using this `freezable` concept, we can actually implement our own `generators`
    and `iterators`. We don’t need the fancy and magical `yield` keyword. In fact, this is how
    the `yield` keyword is implemented ;)
2. these freezable functions will put themselves to sleep (`freeze` themselves) at some specific points
(network requests, read/writes, inputs from user, etc.). We need a mechanism to awake (`unfreeze`) them.
Recall that the `async` concept is useful especially for I/O related tasks. So, the `async` functions we
will write, will most probably wait on some low-level I/O tasks. And should only be awaken when the
related I/O resource is ready.
    - If we tell the OS that we are interested in specific events, it can notify us [https://cfsamson.github.io/book-exploring-async-basics/4_interrupts_firmware_io.html](https://cfsamson.github.io/book-exploring-async-basics/4_interrupts_firmware_io.html)
        - What happens in a very brief summary is: your I/O request is relayed to the related component
        (for example, if you requested to fetch a website, the request is relayed to your network card).
        - The network card has a microcontroller in it, so it probably does some polling to check if there
        is any answer present from the server. When there is, it notifies the OS. And then OS interrupts the
        CPU with a message: “this resource is now ready”.
        - If you can imagine, this whole OS part is another rabbit hole. If we want to implement our own
        functions that can communicate with OS, we will have to dive into specific signal/flags that each
        OS use.
        - On top of that, each operating system has a different strategy for this notification system
        (for example: Epoll for linux, Kqueue for BSD(macOS), and IOCP for Windows). Each will require a
        different implementation on our side.
        - This is all too low-level, and implementing this is whole another project
        (`[mio](https://github.com/tokio-rs/mio)`). I don’t think doing that will help demystify
        the `async` concept, and we will be diverging from our main point. If you insist on learning
        the details of OS communication, read the above link and all the OS related chapters in there,
        and then you might dive into `mio` project.
    - coming back to our topic: we need a mechanism for telling the OS that we will be interested in
    some events
    - and another one for listening the signals that OS sending us
    - since we won’t be using any 3rd party libraries in this project, and also don’t write our
    own `mio`. So what should we do? Let’s recap: we should tell the OS that we are interested in a
    specific event, and that event will happen at some time in the future. And the OS will later
    notify us about this. We won’t have actual I/O resources in our case. We just want to show that we
    can stop a function, and with a signal from the OS, we can continue on this function. So we can do
    this instead:
        - spawn a thread, that will be the simulation of the OS
        - send our task’s ID (in place of some I/O resource subscription) to this thread (the OS)
        - and the OS will just wait for some random time, and then notify us that the requested resource
        (ID for our case) is ready.
3. We covered the communication part with the OS. But how will the rest work? Who will awake (`unfreeze`)
the tasks? One way to implement this is:
    - have a thread (executor) that will run the these async tasks. Btw, this does not have to be an
    extra thread, it can be the main thread as well
    - and have another thread that will listen to the signals sent by the OS, and somehow notify the
    executor about this, so that executor will know which task is available for further progress
        - this is a great short summary I believe ([quoting](https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/appendix-1/reactor-executor-pattern)):

          > In Rust a library like mio will be what drives the Reactor part. In Rust we have `Futures`
          > which pass on a `Waker` to the Reactor. Instead of communicating directly with the Executor through
          > a channel, the Reactor wil call `Waker.wake()` on the relevant `Waker` once an event is ready.

        - quoting from ****What does the “wake” phase require?**** part of this article: [https://boats.gitlab.io/blog/post/wakers-i/](https://boats.gitlab.io/blog/post/wakers-i/)

            > The final phase is the phase in which the wakers really do all of their work.
            > Once the event we’re waiting on has happened, the event source calls the wake method.
            > The wake method is implemented by each executor, and contains the logic for setting up this
            > future to be polled again by the executor. It turns out there are several ways to implement
            > this, and we’d like to be able to support all of them.
            >
            > - **Using an &‘static AtomicBool:** In this implementation, the executor can only run one task
            > at a time. When it is time to wake that task, a global flag is tripped, and then the task
            > will be polled again via a side-channel. This implementation does not make sense for most
            >use cases, but it is actually being used by some users on embedded platforms with extremely
            > minimal resources. The waker is represented as a reference to the global flag.
            > - **Using Task IDs:** In this implementation, the executor stores a global set of tasks that
            > it is current polling in some sort of associative map. When it is time to wake a task,
            > the executor is given the ID for that task in order to tell it which task is ready to be polled.
            > The waker is represented as this task ID (in effect, the waker’s data is a `usize`).
            > - **Using reference counting:** In this implementation (which has become the predominant
            > implementation), the executor is just one or more queue of tasks that it will fetch from
            > and poll as soon as they’re ready. The waker is itself a reference counted pointer to a
            > particular task, and when it is time to wake, it puts itself onto the executor’s queue.
            > In this case, the waker is represented as a reference counted pointer.

    - since this is all too complicated, we will use a simpler approach:
        - our `Executor` will send the names/ID's of the I/O resources we are waiting on to our `Reactor`
        - our `Reactor` will listen to the OS notifications for these I/O resources
        - if any notification arrives, our `Reactor` will then notify the `Executor` about these events.
        - remember that, this is an oversimplification!

</details>

---

<br>
<br>

We won't be diving into assembly or machine code, but we will not use any 3rd party library,
and implement every magical thing from scratch, including:
- the types/traits:
  - `Futures`,
  - `Generators`,
  - `Iterators`,
- the keywords:
  - `async`,
  - `yield`
  - `await`.
- the concepts:
  - `Executor`
  - `Reactor`
  - `Waker`



## The Roadmap:
1. Implement `Freezable`:
   1. The most important ability of the `async` world is: being able to stop a function, and continue from where we left off.
   Async functions are able to do this, via `Generators`. And `Generators` are able to do this via the `yield` keyword.
   So we should implement a library, that will demonstrate how we can `freeze` a function,
   and continue to it when we feel like it.
   1. We will see that, we can write a `freezable` function. But in order to make another person's function turn into a
   `freezable` function, we will have to write a code, that should write a code for us. So, the next step is:
   *write a macro, that will turn the given function into a `freezable` function*.
2. Implement a `Reactor` and an `Executor`
   1. Now, our `freezable` functions are ready to use, but in the async world,
   we are not continuing our functions when we please to.
   They are being *frozen* when the IO resource is not ready, and getting *awaken* when that resource is ready.
   1. simulate the OS with an external thread, since dealing with OS's and IO's are whole another project
   on its own, [`mio`](https://github.com/tokio-rs/mio) (details are discussed below).
   1. implement a `Reactor` that will listen to the signals of the OS, and can relay the information
   to the `Executor` about which task should be polled next.
   1. write another macro, that can poll, and continue on many `freezable` tasks in a single thread.
   This will be our `Executor`, which will try to call `unfreeze()` on the the tasks that are told
   by the `Reactor`



