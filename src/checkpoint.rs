// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the checkpoint utilities.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// use
use std::sync::RwLock;
//
use crate::{
    IndexT,
    ADfn,
    GlobalCheckpointVecPublic,
};
//
#[cfg(doc)]
use crate::doc_generic_v;
// ---------------------------------------------------------------------------
// TODO: Change to pub(crate) after general purpose examples/checkpoint.rs
// code moves to this file.
pub mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    use std::sync::RwLock;
    //
    use crate::ADfn;
    //
    #[cfg(doc)]
    use crate::doc_generic_v;
    //
    // GlobalCheckpointVec
    pub trait GlobalCheckpointVec
    where
        Self : Sized + 'static,
    {   /// Returns a reference to the map from checkpoint_id to ADfn objects.
        ///
        /// ```text
        ///     let rw_lock = GlobalCheckpointVec::get();
        /// ```
        ///
        /// * Self : must be a value type V in [doc_generic_v]
        ///
        /// * rw_lock :
        /// is a read-write lock object [std::sync::RwLock]
        ///
        /// * write_lock :
        /// ``` text
        ///     let write_lock     = rw_lock.write();
        ///     let checkpoint_vec = write_lock.unwrap();
        /// ```
        ///
        /// * read_lock :
        /// ``` text
        ///     let read_lock      = rw_lock.read();
        ///     let checkpoint_vec = read_lock.unwrap();
        /// ```
        ///
        /// * checkpont_vec :
        /// checkpont_vec\[checkpoint_id\] is the [ADfn] corresponding to
        /// checkpoint_id.
        ///
        fn get() -> &'static RwLock< Vec< ADfn<Self> > >;
    }
}
//
// impl_global_checkpoint_vec!
/// Implement the global checkpoint vector for value type V
///
/// * V : see [doc_generic_v]
///
/// This macro must be executed once for any type *V*  where
/// `AD<V>` is used. The rustad package automatically executes it
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::sync::RwLock;
/// ```
macro_rules! impl_global_checkpoint_vec{ ($V:ty) => {
    #[doc = concat!(
        "The global Checkpoint vector for value type `", stringify!($V), "`"
    ) ]
    impl crate::checkpoint::sealed::GlobalCheckpointVec for $V {
        fn get() -> &'static
        RwLock< Vec< crate::ADfn<$V> > > {
            pub(crate) static CHECKPOINT_VEC :
                RwLock< Vec< crate::ADfn<$V> > > =
                    RwLock::new( Vec::new() );
            &CHECKPOINT_VEC
        }
    }
} }
pub(crate) use impl_global_checkpoint_vec;
// ----------------------------------------------------------------------------
// register_checkpoint
/// Move a function object to the global chekpoint vector.
///
/// * Syntax :
/// ```text
///     checkpoint_id = register_checkpoint(ad_fn)
/// ```
///
/// * V : see [doc_generic_v]
///
/// * ad_fn :
/// is the ad_fn that is being moved to the global checkpoint vector.
///
/// * checkpoint_id :
/// is the index that is used to identify this checkpoint function.
///
pub fn register_checkpoint<V>( ad_fn : ADfn<V> ) -> IndexT
where
    V : GlobalCheckpointVecPublic ,
{   //
    // rwlock
    let rw_lock : &RwLock< Vec< ADfn<V> > > =
        sealed::GlobalCheckpointVec::get();
    //
    // checkpoint_id
    let checkpoint_id  : IndexT;
    let id_too_large   : bool;
    {   //
        // write_lock
        let write_lock = rw_lock.write();
        assert!( write_lock.is_ok() );
        //
        let mut checkpoint_vec = write_lock.unwrap();
        let id_usize           = checkpoint_vec.len();
        id_too_large           = (IndexT::MAX as usize) < id_usize;
        checkpoint_id          = checkpoint_vec.len() as IndexT;
        checkpoint_vec.push( ad_fn );
    }
    assert!( ! id_too_large );
    checkpoint_id
}
// ----------------------------------------------------------------------------
