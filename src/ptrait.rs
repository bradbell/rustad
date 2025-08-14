// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! Define traits that are used by the public rustad API.
//!
//! Link to [parent module](super)
// ----------------------------------------------------------------------------
// use
//
use crate::record::sealed::ThisThreadTape;
use crate::checkpoint::sealed::CheckpointAll;
//
// ----------------------------------------------------------------------------
// Sealed traits
// https://rust-lang.github.io/api-guidelines/future-proofing.html
//
/*
// It has not been necessary to use GenericAsPublic (so far).
// GenericAsPublic
use crate::gas::sealed::GenericAs;
/// This is the public interface to a sealed trait
pub trait GenericAsPublic<D> : GenericAs<D>
{ }
impl<S,D> GenericAsPublic<D> for S
where
    S : GenericAs<D> ,
{ }
*/
//
// ThisThreadTapePublic
/// This is the public interface to a sealed trait
pub trait ThisThreadTapePublic<U> : ThisThreadTape<U>
where
    U : Sized + 'static ,
{}
impl<F,U> ThisThreadTapePublic<U> for F
where
    F : ThisThreadTape<U> ,
    U : Sized + 'static ,
{}
//
// CheckpointAllPublic
/// This is the public interface to a sealed trait
pub trait CheckpointAllPublic<U> : CheckpointAll<U>
where
    U : Sized + 'static ,
{}
impl<F,U> CheckpointAllPublic<U> for F
where
    F : CheckpointAll<U> ,
    U : Sized + 'static ,
{}
