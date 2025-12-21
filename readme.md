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

1.  The API is easy to use.
2.  Its representation of functions using tapes is easy to manipulate.
    This makes its source code easy to understand and helps developers
    of other AD packages.
3.  The package works well with many threads.
4.  The package has special types that supports machine learning algorithms.

## Releases

1.  0.0.0 : 2025.10.18 :
    A preliminary release created so people can find this readme file.

## Features Implemented

1.  Addition and multiplication: We have held off on other simple numerical
    operations while we focus on more complicated features.

2.  Forward and reverse mode derivatives with optional tracing
    of the computation.

3.  Forward and reverse mode sparsity patterns with optional tracing
    of the computation.

4.  Generic code that is is the same for different floating point types.
    These types include numerical vectors that act element wise
    (for machine learning) .

5.  Derivative calculations can be used in the definition of new functions
    (that can be differentiated). 
    This is called AD evaluation of the derivatives.

6.  Atomic functions and Checkpointing. Atomic function have been extended
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

## Goals Before Stable API

1.  Subtraction, division and all the standard math functions.

2.  Conditional Expressions.
    
3.  Forward and reverse sparse derivative calculations
    (sparsity patterns are already implemented). 

4.  Checkpointing with arbitrary order of differentiation.

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
