# A Rust Automatic Differentiation Library

- [Objective](#objective)
- [Releases](#releases)
- [Operations Implemented](#operations-implemented)
- [Under Construction](#under-construction)
- [Wish List](#wish-list)
- [User Documentation](#user-documentation)
- [Developer Documentation](#developer-documentation)
- [Testing](#testing)
- [Contact Us](#contact-us)

## Objective
This package is intended to include (and extend) most of the features in
[CppAD](https://cppad.readthedocs.io/latest/) in a way that :

1.  It's API is easy to use.
2.  It's representation of functions is easy to understand and manipulate.
    This makes the package very flexible and helps developers of other AD packages.
3.  It works well with many threads; e.g,
    AD function objects do not have state
    and hence can be shared by many threads.
4.  It has special types that supports machine learning algorithms; i.e,
    element wise vector operations.

## Releases

1.  0.0.0 : 2025.10.18 :
    A preliminary release created so people can find this readme file.

## Features Implemented

1.  Addition, subtraction and multiplication: 
    We have held off on other simple numerical
    operations while we focus on more complicated features.

2.  Forward and reverse mode derivatives with optional tracing
    of the computation.

3.  Forward and reverse mode sparsity patterns with optional tracing
    of the computation.

4.  Generic code that is is the same for different floating point types.
    These types include numeric vectors that act element wise
    (for machine learning) .

5.  Derivative calculations can be used in the definition of new functions
    (that can be differentiated). 
    This is called AD evaluation of the derivatives.

6.  Atomic functions and Checkpointing. These have been extended
    (from the CppAD implementations)
    so that they stay atomic when used in functions that are AD evaluated.

7.  Generate compile and link source code for derivative calculations.

8.  Dynamic parameters; i.e., function arguments that are treated as constant
    during differeniation.

9.  Absolute zero multiplication; i.e., zero times nan is also zero.
    This is important when computing derivatives where some of the components
    are nan, but they are not used.

10. Optimization: Reduce tape size both during recording; e.g.,
    multiplication by the constants zero and one does not need to be recorded.
    Reduce size and avoid recomputaiton in AD function objects by detecting
    equivalent constants, dynamic parameters, and variables..

11. Conditional Expressions allow for if then else values to be recorded
    in an AD function object. Rustad implements this using comparison operators
    that have numerical results (instead of boolean results).
    For example, the following pseudo syntax would represent the absolute
    value of x: 

        y = (x >= 0) * x + (x < 0) * -x

    Note that this has a natural extension to numeric vectors where it
    acts element wise.

12. Forward, reverse, and subgraph sparse derivative calculations

## Goals Before Stable API

We are currently implementing the standard math functions.
    
## Wish List

1.  Generate llvm, similar to the source code generation and use it
    to speed up evaluation of function values and derivatives.

2.  In the case of NumVec operations, Generate GPU code and use it
    to speed up evaluation of function values and derivatives.

3.  Abs-normal form.

## User Documentation
This package does not yet have a stable API. 
You can see to current user documentation by executing the following:

    cargo doc
    firefox target/doc/rustad/index.html

## Developer Documentation
You can see to current developer documentation by executing the following:

    cargo doc --document-private-items
    firefox target/doc/rustad/index.html

## Testing
The following command should build and run all of the tests on Unix
(including MacOS):

    bin/check_all.sh

The following command will just run the speed test:

    cargo run --release

## Contact Us

1.  [Contributing](https://github.com/bradbell/rustad/discussions/categories/contribute)
2.  [Help](https://github.com/bradbell/rustad/discussions/categories/q-a)
