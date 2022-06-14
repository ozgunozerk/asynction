# Freezable

**A demonstration of how one can implement a function that is Freezable, and can continue from where it is left off.**

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
