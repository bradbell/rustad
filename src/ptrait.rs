// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! Define traits that are used by the public rustad API.
//! : [parent module](super)
//
// ----------------------------------------------------------------------------
// Sealed traits
// https://rust-lang.github.io/api-guidelines/future-proofing.html
//
/*
// It has not been necessary to use GenericAsPublic (so far).
// GenericAsPublic
use crate::gas::sealed::GenericAs;
pub trait GenericAsPublic<D> : GenericAs<D>
{ }
impl<S,D> GenericAsPublic<D> for S
where
    S : GenericAs<D> ,
{ }
*/
//
// ThisThreadTapePublic
use crate::record::sealed::ThisThreadTape;
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
// ThisThreadCheckpointAllPublic
use crate::checkpoint::sealed::ThisThreadCheckpointAll;
pub trait ThisThreadCheckpointAllPublic<U> : ThisThreadCheckpointAll<U>
where
    U : Sized + 'static ,
{}
impl<F,U> ThisThreadCheckpointAllPublic<U> for F
where
    F : ThisThreadCheckpointAll<U> ,
    U : Sized + 'static ,
{}
