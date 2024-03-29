# Freezable

**A demonstration of how one can implement a function that is Freezable, and can continue from where it is left off.**
**This library's aim is to show the desugared version of a such function (uncover the code that is generated for the compiler).**

*Bear in mind the following:* this library aims to inspect what happens behind the scenes, and focuses at the `desugared version`,
it will not be ergonomic to implement the `Freezable` trait directly.

*In the `freezable-macro` crate, I will provide the macro for converting any function to its `freezable`*
*version via a procedural macro. So, it will make more sense to use that macro for your functions,*
*instead of implementing the `Freezable` trait on your own.*

*Note that, there may be minimal differences with the desugared version and the macro (`freezable`) generated code.*
*The desugared code in here will still be 99% accurate, the details will be probably related to*
*technical difficulties coming along with macro implementation.*
*Remember that the main purpose of this desugrad code is to show what the generated code should look like :)*

### Why you should care?

Because this is the building block of all async computation, generators, and iterators, and the implementation
of the keyword `yield`.

*This is where all the magic is happening!*

In a single-threaded environment, if we want to get out of a function in the midst of its computation,
our only option is to `return`. But, when we `return` from a function,
we lose all the internal computation related to that function (since the functions stack vanishes when we `return` from it).

To be able to ***freeze*** a function, and come back to it whenever we please to (iterators, generators, async computation),
we need to somehow store the state of the function, and make it not do all the computation from the beginning
when we call it again.

To be able to do this, we are treating our ***freezable*** function as a `StateMachine`, and storing its `State` in an external
data structure (`enum` in this case).

Enjoy the code!
