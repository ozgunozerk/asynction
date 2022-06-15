# Asycntion

This repository is for revealing the magic of the `async` functions.
It tries to uncover every single secret of the `async` concept.

We won't be diving into assembly or machine code, but we will implement every magical thing from scratch, including:
- the concepts:
  - `Futures`,
  - `Generators`,
  - `Iterators`,
- and the keywords:
  - `async`,
  - `yield`
  - `await`.

**Roadmap:**
1. Implement `Freezable`:
   1. The most critical ability of the `async` world is: being able to stop a function, and continue from where we left off.
   Async functions are able to do this, via `Generators`. And `Generators` are able to do this via the `yield` keyword.
   So we should implement a library, that will demonstrate how we can `freeze` a function,
   and continue to it when we feel like it.
   2. We will see that, we can write a `freezable` function. But in order to make another person's function turn into a
   `freezable` function, we will have to write a code, that should write a code for us. So, the next step is:
   *write a macro, that will turn the given function into a `freezable` function*.
2. Implement a `Reactor` and an `Executor`
   1. Now, our `freezable` functions are ready to use, but in the async world,
   we are not continuing our functions when we please to.
   They are being *frozen* when the IO resource is not ready, and getting *awaken* when that resource is ready.
   To simulate the async world, we should write a `Reactor`, that will awake the tasks when the resource is ready,
   and an `Executor`, that will continuously try to run the `awake` tasks.
   But for now, we will simulate the IO task being ready or not, via simple sleeps on the reactor side.
   2. Improve our macro for `freezable`, such that, it will only *freeze* the function, when it encounters a IO related task.
3. Make the reactor actually communicate with the OS
   1. For this, we should implement a *non-blocking* version of the IO operations. We will start with `stdin`.
   That will also be an `freezable` function, but immediately returns `NonReady`, and freezes itself.
   2. Reactor should listen to the OS for the respective signals. Say, when the user inputs something,
   reactor should *unfreeze/awake* the respective function.



**Endgoal:**
Create a library, that will turn this code:
```rust
// This is psuedo-rust, which is the original code, that we are aiming to desugar
use filedescriptor::FileDescriptor;

async fn write_bytes_new_file() {
    let extension = ".log";
    let file_name = get_file_name_from_user().await;
    let full_name = file_name + extension;
    let file_descriptor = create_file(full_name).await;
    println!("File created!");
    return open_write_bytes(file_descriptor).await;
    println!("Write successful!");
}
```

into this code, via a macro:
```rust
// this is the pseudo-rust, that we are aiming to be the desugaring of the original code

use filedescriptor::FileDescriptor;
use StateMachine::*;

enum StateMachine<T> {
    Chunk0(),
    Chunk1(String),
    Chunk2(String),
    Chunk3(FileDescriptor),
    Chunk4(T),
}

enum AsyncResult<T> {
    Ready(T),
    NotReady,
}

fn executor<T>() {
    let mut state = StateMachine::<T>::Chunk0();
    /*
    rest...
    */
}

fn write_bytes_new_file<T>(state: &mut StateMachine<T>) -> AsyncResult<()> {
    match state {
        Chunk0() => {
            let extension = ".log";
            *state = StateMachine::Chunk1(extension.to_string());
            return AsyncResult::NotReady;
        }
        Chunk1(extension) => {
            let file_name_res: Option<String> = get_file_name_from_user();
            if let Some(file_name) = file_name_res {
                let full_name = file_name + extension;
                *state = StateMachine::Chunk2(file_name);
            }
            return AsyncResult::NotReady;
        }
        Chunk2(full_name) => {
            let file_descriptor = create_file(full_name);
            if file_descriptor.is_some() {
                println!("File created!");
                *state = StateMachine::Chunk3(file_descriptor);
            }
            return AsyncResult::NotReady;
        }
        Chunk3(file_descriptor) => {
            let result = open_write_bytes(file_descriptor);
            if result.is_some() {
                println!("Write successful!");
                *state = StateMachine::Chunk4(())
            }
            return AsyncResult::NotReady;
        }
        Chunk4(result) => return AsyncResult::Ready(result),
    }
}
```
also, implement the undefined functions there :)
