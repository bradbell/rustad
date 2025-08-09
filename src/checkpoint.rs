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
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::function::GADFun;
use crate::gad::GAD;
use crate::gas::as_from;
use crate::gas::sealed::GenericAs;
use crate::operator::GlobalOpInfoVec;
use crate::operator::id::{CALL_OP, CALL_RES_OP};
use crate::ptrait::ThisThreadCheckpointAllPublic;
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
pub fn store_checkpoint<F,U>(
    fun:  GADFun<F,U>,
    name: &String)
where
    F     : GlobalOpInfoVec<U> + ThisThreadCheckpointAllPublic<U> ,
    U     : Copy + 'static + GenericAs<usize> + std::cmp::PartialEq,
    usize : GenericAs<U>,
{
    //
    // This thread's checkpoint information for GAD<F,U>
    let local_key = < F as sealed::ThisThreadCheckpointAll<U> >::get();
    local_key.with_borrow_mut( |all| {
        assert!(
            ! all.map.contains_key(name),
            "store_checkpoint: name {name} was used before on this thread"
        );
        let index           = all.vec.len();
        let trace           = false;
        let pattern         = fun.sub_sparsity(trace);
        let checkpoint_info = OneCheckpointInfo {
            fun_index  : index,
            name       : name.clone(),
            adfun      : fun,
            dependency : pattern,
        };
        all.vec.push( checkpoint_info );
        all.map.insert(name.clone(), index);
    } );
}
//
// use_checkpoint
/// Makes a call, by name, to a checkpoint function.
///
/// ```text
///     ad_range = use_checkpoint(&name, &ad_comain, trace)
/// ```
///
/// If the tape for this thread is recording, include the call
/// as a checkpoint in the tape.
///
/// * F, U : see [doc_generic_f_and_u]
///
/// * name :
/// The name that was used to store the checkpoint function.
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
/// use rustad::gad::GAD;
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
/// let name    = "f".to_string();
/// store_checkpoint(f, &name);
/// //
/// // g
/// // g(u) = f( u0, u0 + u1, u1)
/// //      = [ u0 + u0 + u1 , (u0 + u1) * u1 ]
/// let  u : Vec<F>  = vec![ 4.0, 5.0];
/// let au : Vec<AD> = rustad::ad_domain(&u);
/// let ax = vec![ au[0], au[0] + au[1], au[1] ];
/// let ay = use_checkpoint(&name, &ax, trace);
/// let g  = rustad::ad_fun(&ay);
/// //
/// // w
/// // w = g(u)
/// let (w, _)  = g.forward_zero(&u, trace);
/// assert_eq!( w[0], u[0] + u[0] + u[1] );
/// assert_eq!( w[1], (u[0] + u[1]) * u[1] );
/// ```
pub fn use_checkpoint<F,U>(
    name      : &String,
    ad_domain : &Vec< GAD<F,U> >,
    trace     : bool,
) -> Vec< GAD<F,U> >
where
    U:        'static + Copy + GenericAs<usize> + std::fmt::Debug,
    usize:    GenericAs<U> ,
    GAD<F,U>: From<F>,
    F:        Copy +
              From<f32> +
              std::fmt::Display +
              sealed::ThisThreadCheckpointAll<U> +
              GlobalOpInfoVec<U> +
              ThisThreadTape<U>,
{   //
    // ad_range
    let local_key = < F as sealed::ThisThreadCheckpointAll<U> >::get();
    let ad_range  = local_key.with_borrow( |all| {
        let option_fun_index = all.map.get(name);
        if option_fun_index == None {
            panic!("use_checkpoint: \
                    name {name} has not been stored as a checkpoint."
            );
        }
        let fun_index        = *option_fun_index.unwrap();
        let check_point_info = &all.vec[fun_index];
        assert_eq!( fun_index, check_point_info.fun_index );
        let local_key : &LocalKey< RefCell< GTape<F,U> > > =
            < F as ThisThreadTape<U> >::get();
        let ad_range_zero = local_key.with_borrow_mut( |tape|
            use_checkpoint_info(tape, check_point_info, ad_domain, trace)
        );
        ad_range_zero
    } );
    ad_range
}
//
// OneCheckpointInfo
/// Information used to splice a checkpoint function call into a recording;
/// see [doc_generic_f_and_u].
pub(crate) struct OneCheckpointInfo<F,U> {
    //
    // fun_index
    /// is the index of this checkpoint function in the vector of all
    /// checkpoint functions.
    pub fun_index    : usize,
    //
    // name
    /// ia a name, that is meaningful to the user, used to identify
    /// this checkpoint function.
    pub name         : String,
    //
    // adfun
    /// ia the [GADFun] object that is used to evaluate this
    // checkpoint function
    /// and its derivative.
    pub adfun        : GADFun<F,U>,
    //
    // dependency
    /// is the dependency pattern as vector of pairs of non-negative integers.
    /// If (i,j) is not in dependency, then the i-th component of the range
    /// does not depend on the j-th component of the domain.
    pub dependency   : Vec< [U; 2] >,
}
//
// sealed::AllCheckpointInfo
// sealed::ThisThreadCheckpointAll
pub (crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    #[cfg(doc)]
    use super::doc_generic_f_and_u;
    //
    use std::thread::LocalKey;
    use std::cell::RefCell;
    use super::OneCheckpointInfo;
    //
    // AllCheckpointInfo
    /// Information for all the checkpoints; see [doc_generic_f_and_u].
    pub struct AllCheckpointInfo<F,U> {
       pub (crate) vec : Vec< OneCheckpointInfo<F,U> > ,
       pub (crate) map : std::collections::HashMap<String, usize> ,
    }
    impl<F,U> AllCheckpointInfo<F,U> {
       pub fn new() -> Self {
          Self {
             vec : Vec::new() ,
             map : std::collections::HashMap::new() ,
          }
       }
    }
    //
    // ThisThreadCheckpointAll
    /// ```text
    ///     < F as sealed::ThisThreadCheckpointAll >::get()
    /// ```
    /// returns a reference to this tape's GAD<F,U> checkpoint information;
    /// see [doc_generic_f_and_u].
    ///
    /// 2DO: Perhaps it would be better if this were global
    /// instead of tape local.
    ///
    pub trait ThisThreadCheckpointAll<U>
    where
        Self : Sized + 'static ,
        U    : Sized + 'static ,
    {
        fn get() -> &'static LocalKey< RefCell< AllCheckpointInfo<Self, U> > >;
    }
}
//
// impl_this_thread_checkpoint!
/// Implement ThisThreadCheckpointAll
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
    impl sealed::ThisThreadCheckpointAll<$u2> for $f1 {
        fn get() ->
        &'static LocalKey< RefCell< sealed::AllCheckpointInfo<$f1, $u2> > > {
            thread_local! {
                pub(crate) static THIS_THREAD_CHECKPOINT_ALL :
                    RefCell< sealed::AllCheckpointInfo<$f1, $u2> > =
                        RefCell::new( sealed::AllCheckpointInfo::new() );

            }
            &THIS_THREAD_CHECKPOINT_ALL
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
    // name, adfun, dependency
    let fun_index  = check_point_info.fun_index;
    let name       = &check_point_info.name;
    let adfun      = &check_point_info.adfun;
    let dependency = &check_point_info.dependency;
    if adfun.domain_len() != ad_domain.len() {
        panic!( "use_chckpoint: ad_domain.len() = {} \
                is not equal to {name}.domain_len() = {}",
                ad_domain.len(), adfun.domain_len()
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
        tape.arg_all.push( as_from(fun_index) );           // arg[0]
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
