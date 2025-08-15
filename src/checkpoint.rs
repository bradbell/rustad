// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module can be used to checkpoint a section of AD computation
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// use
//
use std::thread::LocalKey;
use std::cell::RefCell;
use std::sync::LazyLock;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::function::GADFun;
use crate::gad::GAD;
use crate::gas::as_from;
use crate::gas::sealed::GenericAs;
use crate::operator::GlobalOpInfoVec;
use crate::operator::id::{CALL_OP, CALL_RES_OP};
use crate::ptrait::CheckpointAllPublic;
use crate::record::GTape;
use crate::record::sealed::ThisThreadTape;
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::doc_generic_f_and_u;
// ---------------------------------------------------------------------------
//
// store_checkpoint
/// Converts a [GADFun] object to a checkpoint functions for this thread.
///
/// * F, U : see [doc_generic_f_and_u]
///
/// * fun :
/// The ADFun object that it converted to a checkpoint function.
///
/// * name :
/// The name the user chooses for this checkpoint function.
/// This name must not appear in a previous `store_checkpoint` call
/// on this thread.
///
/// * Example : see the example in [use_checkpoint]
///
pub fn store_checkpoint<F,U>(fun:  GADFun<F,U>) -> usize
where
    F     : GlobalOpInfoVec<U> + CheckpointAllPublic<U> ,
    U     : Copy + 'static + GenericAs<usize> + std::cmp::PartialEq,
    usize : GenericAs<U>,
{
    //
    // pattern
    // do this calculation outside of the lock
    let trace          = false;
    let pattern        = fun.sub_sparsity(trace);
    //
    // try_write
    let lazy_lock      = < F as sealed::CheckpointAll<U> >::get();
    let rw_lock        = &*lazy_lock;
    let mut try_write  = rw_lock.try_write();
    let mut count      = 0;
    while try_write.is_err() && count < 30 {
        sleep( Duration::from_millis(100) );
        count     += 1;
        try_write  = rw_lock.try_write();
    }
    // ----------------------------------------------------------------------
    // Begin: lock out read and other writes
    // ----------------------------------------------------------------------
    if try_write.is_err() { panic!(
        "store_checkpoint: timed out while waiting for a write lock"
    ) };
    //
    // all, checkpoint_id
    let mut all = try_write.unwrap();
    let checkpoint_id   = all.vec.len();
    let checkpoint_info = OneCheckpointInfo {
        checkpoint_id  : checkpoint_id,
        adfun          : fun,
        dependency     : pattern,
    };
    all.vec.push( checkpoint_info );
    //
    checkpoint_id
    // ----------------------------------------------------------------------
    // End lock
    // ----------------------------------------------------------------------
}
//
// use_checkpoint
/// Makes a call, by name, to a checkpoint function.
///
/// * Syntax :
/// ```text
///     ad_range = use_checkpoint(checkpoint_id, &ad_comain, trace)
/// ```
///
/// If the tape for this thread is recording, include the call
/// as a checkpoint in the tape.
///
/// * F, U : see [doc_generic_f_and_u]
///
/// * checkpoint_id :
/// the checkpoint_id returned by a previous call to [store_checkpoint] ;
///
/// * ad_domain :
/// The value of the domain variables for the function bning called.
///
/// * trace :
/// If this is true (false), evaluation of the
/// checkpoint function corresponding is traced.
///
/// * ad_range :
/// The range variable values that correspond to the
/// domain variable values.
///
/// # Example
/// ```
/// use rustad::GAD;
/// use rustad::store_checkpoint;
/// use rustad::use_checkpoint;
/// //
/// // F, AD
/// type F  = f32;
/// type U  = u64;
/// type AD = GAD<F,U>;
/// //
/// // trace
/// let trace   = false;
/// //
/// // f
/// // f(x) = [x0 + x1, x1 * x2]
/// let  x : Vec<F>  = vec![ 1.0, 2.0, 3.0 ];
/// let ax : Vec<AD> = rustad::ad_domain(&x);
/// let ay = vec![ ax[0] + ax[1], ax[1] * ax[2] ];
/// let f  = rustad::ad_fun(&ay);
/// //
/// // f
/// // store as a checkpoint function
/// let checkpoint_id = store_checkpoint(f);
/// //
/// // g
/// // g(u) = f( u0, u0 + u1, u1)
/// //      = [ u0 + u0 + u1 , (u0 + u1) * u1 ]
/// let  u : Vec<F>  = vec![ 4.0, 5.0];
/// let au : Vec<AD> = rustad::ad_domain(&u);
/// let ax = vec![ au[0], au[0] + au[1], au[1] ];
/// let ay = use_checkpoint(checkpoint_id, &ax, trace);
/// let g  = rustad::ad_fun(&ay);
/// //
/// // w
/// // w = g(u)
/// let (w, _)  = g.forward_zero(&u, trace);
/// assert_eq!( w[0], u[0] + u[0] + u[1] );
/// assert_eq!( w[1], (u[0] + u[1]) * u[1] );
/// ```
pub fn use_checkpoint<F,U>(
    checkpoint_id : usize ,
    ad_domain     : &Vec< GAD<F,U> >,
    trace         : bool,
) -> Vec< GAD<F,U> >
where
    U:        'static + Copy + GenericAs<usize> + std::fmt::Debug,
    usize:    GenericAs<U> ,
    GAD<F,U>: From<F>,
    F:        Copy +
              From<f32> +
              std::fmt::Display +
              sealed::CheckpointAll<U> +
              GlobalOpInfoVec<U> +
              ThisThreadTape<U>,
{   //
    // ad_range
    let lazy_lock     = < F as sealed::CheckpointAll<U> >::get();
    let rw_lock       = &*lazy_lock;
    let mut try_read  = rw_lock.try_read();
    let mut count     = 0;
    while try_read.is_err() && count < 30 {
        sleep( Duration::from_millis(100) );
        count     += 1;
        try_read  = rw_lock.try_read();
    }
    if try_read.is_err() { panic!(
        "use_checkpoint: timeout while waiting for read lock"
    ) };
    // ----------------------------------------------------------------------
    // Begin: lock out writes
    // ----------------------------------------------------------------------
    let all = try_read.unwrap();
    let check_point_info = &all.vec[checkpoint_id];
    assert_eq!( checkpoint_id, check_point_info.checkpoint_id );
    let local_key : &LocalKey< RefCell< GTape<F,U> > > =
        < F as ThisThreadTape<U> >::get();
    let ad_range = local_key.with_borrow_mut( |tape|
        use_checkpoint_info(tape, check_point_info, ad_domain, trace)
    );
    ad_range
    // ----------------------------------------------------------------------
    // End: lock out writes
    // ----------------------------------------------------------------------
}
//
// OneCheckpointInfo
/// Information used to splice a checkpoint function call into a recording;
/// see [doc_generic_f_and_u].
pub(crate) struct OneCheckpointInfo<F,U> {
    //
    // checkpoint_id
    /// is the index of this checkpoint function in the vector of all
    /// checkpoint functions.
    pub checkpoint_id    : usize,
    //
    // adfun
    /// ia the [GADFun] object that is used to evaluate this
    // checkpoint function
    /// and its derivative.
    pub adfun            : GADFun<F,U>,
    //
    // dependency
    /// is the dependency pattern as vector of pairs of non-negative integers.
    /// If (i,j) is not in dependency, then the i-th component of the range
    /// does not depend on the j-th component of the domain.
    pub dependency       : Vec< [U; 2] >,
}
//
// sealed::AllCheckpointInfo
// sealed::CheckpointAll
pub (crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    #[cfg(doc)]
    use super::doc_generic_f_and_u;
    //
    use std::sync::LazyLock;
    use std::sync::RwLock;
    use super::OneCheckpointInfo;
    //
    // AllCheckpointInfo
    /// Information for all the checkpoints; see [doc_generic_f_and_u].
    pub struct AllCheckpointInfo<F,U> {
       pub (crate) vec : Vec< OneCheckpointInfo<F,U> > ,
    }
    impl<F,U> AllCheckpointInfo<F,U> {
       pub fn new() -> Self {
          Self {
             vec : Vec::new() ,
          }
       }
    }
    //
    // CheckpointAll
    /// ```text
    ///     < F as sealed::CheckpointAll >::get()
    /// ```
    /// returns a reference to this tape's GAD<F,U> checkpoint information;
    /// see [doc_generic_f_and_u].
    ///
    /// 2DO: Perhaps it would be better if this were global
    /// instead of tape local.
    ///
    pub trait CheckpointAll<U>
    where
        Self : Sized + 'static ,
        U    : Sized + 'static ,
    {
        fn get() -> &'static LazyLock< RwLock< AllCheckpointInfo<Self, U> > >;
    }
}
//
// impl_this_thread_checkpoint!
/// Implement CheckpointAll
/// for all possible values of F,U; see [doc_generic_f_and_u] .
///
/// * f1 : is the floating point type used for values calculations.
/// * u2 : is the unsigned integer type used for tape indices.
///
macro_rules! impl_this_thread_checkpoint{ ($f1:ident, $u2:ident) => {
    #[doc = concat!(
        "This threads tape for recording ",
        "GAD<" , stringify!($f1), ", ", stringify!($u2), "> operations"
    ) ]
    impl sealed::CheckpointAll<$u2> for $f1 {
        fn get() ->
        &'static LazyLock< RwLock< sealed::AllCheckpointInfo<$f1, $u2> > > {
            pub(crate) static CHECKPOINT_ALL :
                LazyLock< RwLock< sealed::AllCheckpointInfo<$f1, $u2> > >  =
                        LazyLock::new(|| RwLock::new(
                             sealed::AllCheckpointInfo::new()
            ) );
            &CHECKPOINT_ALL
        }
    }
} }
impl_this_thread_checkpoint!(f32, u32);
impl_this_thread_checkpoint!(f32, u64);
impl_this_thread_checkpoint!(f64, u32);
impl_this_thread_checkpoint!(f64, u64);
//
// use_checkpoint_info
/// Make a call, by OneCheckpointInfo, to a checkpoint function.
///
/// If the tape for this thread is recording, include the call
/// as a checkpoint in the tape.
///
/// * F, U : see [doc_generic_f_and_u]
///
/// * tape :
/// The tape that records operations on this thread.
///
/// * check_point_info :
/// The information for this checkpoint.
///
/// * ad_domain :
/// The value of the checkpoint function domain variables.
///
/// * trace :
/// If this is true (false), evaluation of the
/// checkpoint function corresponding to *ad_domain* is traced.
///
/// * return :
/// The values of the range variables that correspond to the
/// domain variable values.
fn use_checkpoint_info<F,U>(
    tape             : &mut GTape<F,U>,
    check_point_info : &OneCheckpointInfo<F,U>,
    ad_domain        : &Vec< GAD<F,U> >,
    trace            : bool,
) -> Vec< GAD<F,U> >
where
    F:         Copy + From<f32> + GlobalOpInfoVec<U> + std::fmt::Display,
    U:         'static + Copy + GenericAs<usize> + std::fmt::Debug,
    GAD<F, U>: From<F>,
    usize:     GenericAs<U>,
{
    //
    // adfun, dependency
    let checkpoint_id  = check_point_info.checkpoint_id;
    let adfun          = &check_point_info.adfun;
    let dependency     = &check_point_info.dependency;
    if adfun.domain_len() != ad_domain.len() {
        panic!( "use_chckpoint: ad_domain.len() = {} \
                is not equal to domain size for checkpoint_id = {}",
                ad_domain.len(), checkpoint_id
        );
    }
    //
    // call_n_arg, call_n_res
    let call_n_arg = adfun.domain_len();
    let call_n_res = adfun.range_len();
    //
    // domain_zero
    let mut domain_zero : Vec<F> = Vec::new();
    for j in 0 .. call_n_arg {
        domain_zero.push( ad_domain[j].value );
    }
    //
    // range_zero
    let (range_zero, _var_zero) = adfun.forward_zero(&domain_zero, trace);
    //
    // ad_range
    let mut ad_range : Vec< GAD<F,U> > = Vec::new();
    for i in 0 .. call_n_res {
        ad_range.push( GAD {
            tape_id:    as_from(0),
            var_index:  as_from(0),
            value:      range_zero[i],
        } );
    }
    //
    //
    if tape.recording {
        //
        // is_var_domain
        let mut is_var_domain : Vec<bool> = Vec::new();
        for j in 0 .. call_n_arg { is_var_domain.push(
            tape.tape_id == as_from( ad_domain[j].tape_id )
        ); }
        //
        // is_var_range
        let mut is_var_range = vec![false; call_n_res];
        for k in 0 .. dependency.len() {
            let [i,j] = dependency[k];
            if is_var_domain[ as_from(j) ] {
                is_var_range[ as_from(i) ] = true;
            }
        }
        //
        // tape.id_all, tape.op2arg
        tape.id_all.push( CALL_OP );
        tape.op2arg.push( as_from( tape.arg_all.len() ) );
        //
        // tape.arg_all, tape.con_all
        tape.arg_all.push( as_from(checkpoint_id) );           // arg[0]
        tape.arg_all.push( as_from(call_n_arg) );          // arg[1]
        tape.arg_all.push( as_from(call_n_res) );          // arg[2]
        tape.arg_all.push( as_from( tape.flag_all.len() ) ); // arg[3]
        for j in 0 .. call_n_arg {
            let index = if is_var_domain[j] {
                as_from( ad_domain[j].var_index )
            } else {
                let con_index = tape.con_all.len();
                tape.con_all.push( ad_domain[j].value );
                con_index
            };
            tape.arg_all.push( as_from(index) ); // arg[4+j]
        }
        //
        // tape.flag_all
        for j in 0 .. call_n_arg {
            tape.flag_all.push( is_var_domain[j] );
        }
        for i in 0 .. call_n_res {
            tape.flag_all.push( is_var_range[i] );
        }
        //
        // ad_range, n_var_res
        let mut n_var_res = 0;
        for i in 0 .. call_n_res {
            if is_var_range[i] {
                ad_range[i].tape_id   = as_from(tape.tape_id);
                ad_range[i].var_index = as_from(tape.n_var + n_var_res);
                n_var_res += 1;
            }
        }
        assert_ne!( n_var_res, 0);
        //
        // tape.n_var
        tape.n_var += n_var_res;
        //
        // tape.id_all, tape.op2arg
        for _i in 0 .. (n_var_res - 1) {
            tape.id_all.push( CALL_RES_OP );
            tape.op2arg.push( as_from( tape.arg_all.len() ) );
        }
    }
    ad_range
}
