# A Rust Automatic Differentiation Library

- [Objective](#objective)
- [Operations Implemented](#operations-implemented)
- [Under Construction](#under-construction)
- [Wish List](#wish-list)
- [User Documentation](#user-documentation)
- [Developer Documentation](#developer-documentation)
- [Testing](#testing)
- [Contact Us](#contact-us)

## Objective
This package is intended to include most of the features in
[CppAD](https://cppad.readthedocs.io/latest/) in a way that :

1.  It is easy to use.
2.  Its source code is easy to understand and maintain.
3.  It works well with may threads.
4.  It supports machine learning algorithms.



## Operations Implemented

1.  Addition and subtraction: We have held off on other simple numerical
    operations while we focus on more complicated features.

2.  Forward and reverse mode derivative calculations.

3.  Forward and reverse mode sparsity calculations.

4.  Generic code that is is the same for different floating point types.
    These types include numerical vectors that act element wise.

5.  Derivative calculations can be used in the definition of new functions
    (that can be differentiated). 
    This is called AD evaluation of the original functions.

6.  Atomic functions and Checkpointing. Atomic function have been extended
    so that they stay atomic when used in functions that are AD evaluated.

## Under Construction

1.  Generate compile and link source code for derivative calculations.

## Goals Before Stable API

1.  Subtraction, multiplication and all the standard math functions.
    
2.  Optimizing the operation sequence.

5.  Add dynamic parameters; i.e., function arguments that can change value
    but act as constants when differentiating.

## Wish List

1.  Generate llvm, similar to the source code generation and use it
    to speed up evaluation of ForwardZero, ForwardOne, and ReverseOne
    for values.

2.  In the case of NumVec operations, Generate GPU code and use it
    to speed up evaluation of ForwardZero, ForwardOne, and ReverseOne
    for values.

## User Documentation
This package does not yet have a stable API. 
More work needs to be done to separate implementation details
from the API.
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
