# A Rust Automatic Differentiation Package

## Plan
This package is intended to include the features in
[CppAD](https://github.com/coin-or/CppAD) in a way that 
is easier to use and has source code that is easier to understand and maintain.

## Operations Implemented

1.  Addition and subtraction: We have held off on other simple numerical
    operations while we focus on more complicated features.

2.  Forward and reverse sparsity calculations.

3.  Checkpointing.

4.  Generic code that is floating point type and indexing type independent.

5.  Using derivative calculations in the definition of other functions
    that can be differentiated.

## User Documentation
This package is does not yet have a stable API. 
You can see to current plan by execution the following:

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
