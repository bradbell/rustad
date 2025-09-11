# A Rust Automatic Differentiation Package

## Goals
This package is intended to include many of the features in
[CppAD](https://github.com/coin-or/CppAD) in a way that :

1. It is easy to use.
2. Its source code is easy to understand and maintain.
3. It works well with may threads.
4. It supports machine learning algorithms.

## Operations Implemented

1.  Addition and subtraction: We have held off on other simple numerical
    operations while we focus on more complicated features.

2.  Forward and reverse mode derivative calculations.

3.  Forward and reverse mode sparsity calculations.

4.  Atomic functions and Checkpointing.

5.  Generic code that is is the same for different floating point types.
    These types include numerical vectors act element wise.

6.  Derivative calculations can be used in the definition of other functions
    that can be differentiated.

## User Documentation
This package is does not yet have a stable API. 
More work needs to be done to separate the implementation details
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
