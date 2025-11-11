// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub(crate) module defines the hashing methods used by rustad.
//!
//! Link to [parent module](super)
//!
// -------------------------------------------------------------------------
//
//
// TypeHash
//
/// Computes the rustad hash for a type using rustc_hash::FxHasher.
pub trait TypeHash
{   fn type_hash(&self) -> u64;
}
//
// impl_type_hash
/// Implement the TypeHash trait for the value types S and NumVec `<S>`
///
/// * S : is f32 or f64
///
macro_rules! impl_value_type_hash {
    ( NumVec< $S:ident > ) => { impl crate::hash::TypeHash for NumVec< $S >
        where
            for<'a> &'a $S : ordered_float::PrimitiveFloat,
            ordered_float::OrderedFloat<$S> : std::hash::Hash ,
        {
            fn type_hash(&self) -> u64
            {   use std::hash::{Hash, Hasher};
                let mut state = rustc_hash::FxHasher::default();
                if self.vec.len() == 1 {
                    let element = ordered_float::OrderedFloat( &(self.s) );
                    element.hash(state);
                } else {
                    for element in self.vec.iter() {
                        let element = ordered_float::OrderedFloat( &(self.s) );
                        element.hash(state);
                    }
                }
                state.finish()
            }
        }
    };
    ( $S:ident ) => { impl crate::hash::TypeHash for $S
        where
            for<'a> &'a $S : ordered_float::PrimitiveFloat,
            ordered_float::OrderedFloat<$S> : std::hash::Hash ,
        {   fn type_hash (&self) -> u64
            {   use std::hash::{Hash, Hasher};
                let mut state = rustc_hash::FxHasher::default();
                let element = ordered_float::OrderedFloat( self );
                element.hash(&mut state);
                state.finish()
            }
        }
    };
}
pub(crate) use impl_value_type_hash;
