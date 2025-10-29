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
This package is intended to include most of the features in
[CppAD](https://cppad.readthedocs.io/latest/) in a way that :

1.  It is easy to use.
2.  Its source code is easy to understand and maintain.
3.  It works well with may threads.
4.  It supports machine learning algorithms.

## Releases

1.  0.0.0 : 2025.10.18 :
    A preliminary release created so people can find this readme file.

## Operations Implemented

1.  Addition and multiplication: We have held off on other simple numerical
    operations while we focus on more complicated features.

2.  Forward and reverse mode derivatives with optional tracing
    of the computation.

3.  Forward and reverse mode sparsity patterns with optional tracing
    of the computation.

4.  Generic code that is is the same for different floating point types.
    These types include numerical vectors that act element wise.

5.  Derivative calculations can be used in the definition of new functions
    (that can be differentiated). 
    This is called AD evaluation of the original functions.

6.  Atomic functions and Checkpointing. Atomic function have been extended
    so that they stay atomic when used in functions that are AD evaluated.

7.  Generate compile and link source code for derivative calculations.

## Under Construction
Add dynamic parameters; i.e., function arguments that can change value
but act as constants when differentiating.


## Goals Before Stable API

1.  Subtraction, division and all the standard math functions.
    
2.  Reduce tape size both during recording and by
    optimizing the operation sequence. For example:
    multiplication by the constants zero and one and addition by zero does not need to be recorded.

3.  Forward and reverse sparse derivative calculations
    (sparsity patterns are already implemented). 

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
