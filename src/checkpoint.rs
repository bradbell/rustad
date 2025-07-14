// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module can be used to checkpoint a section of AD computation:
//! //! ADFun objects:
//!
//! # Example
//! ```
//! use rustad::Float;
//! use rustad::function;
//! use rustad::checkpoint::{store_checkpoint, use_checkpoint};
//! //
//! // trace
//! let trace   = false;
//! //
//! // f
//! // f(x) = [x0 + x1, x1 * x2]
//! let  x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
//! let ax      = function::ad_domain(&x);
//! let ay      = vec![ ax[0] + ax[1], ax[1] * ax[2] ];
//! let f       = function::ad_fun(&ay);
//! //
//! // f
//! // store as a checkpoint function
//! let name    = "f".to_string();
//! store_checkpoint(f, &name);
//! //
//! // g
//! // g(u) = f( u0, u0 + u1, u1)
//! //      = [ u0 + u0 + u1 , (u0 + u1) * u1 ]
//! let  u : Vec<Float>  = vec![ 4.0, 5.0];
//! let au      = function::ad_domain(&u);
//! let ax      = vec![ au[0], au[0] + au[1], au[1] ];
//! let ay      = use_checkpoint(&name, &ax, trace);
//! let g       = function::ad_fun(&ay);
//! //
//! // w
//! // w = g(u)
//! let (w, _)  = g.forward_zero(&u, trace);
//! assert_eq!( w[0], u[0] + u[0] + u[1] );
//! assert_eq!( w[1], (u[0] + u[1]) * u[1] );
//! ```
//
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::{Index, Float};
use crate::function::ADFun;
use crate::ad::AD;
use crate::ad_tape::{Tape, GTape, this_thread_tape};
use crate::operator::id::{CALL_OP, CALL_RES_OP};
//
// CheckpointInfo
/// Information used to splice a checkpoint function call into a recording.
pub(crate) struct CheckpointInfo {
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
    /// ia the [ADFun] object that is used to evaluate this chekcpoing unciton
    /// and its derivative.
    pub adfun        : ADFun,
    //
    // dependency
    /// is the dependency pattern as vector of pairs of non-negative integers.
    /// If (i,j) is not in dependency, then the i-th component of the range
    /// does not depend on the j-th component of the domain.
    pub dependency   : Vec<(Index, Index)>,
}
//
thread_local! {
    //
    // THIS_THREAD_CHECKOINT_VEC
    /// thread local storage holding a vector of CheckpointInfo objects.
    pub(crate) static THIS_THREAD_CHECKPOINT_VEC:
        std::cell::RefCell< Vec<CheckpointInfo> > =
            std::cell::RefCell::new( Vec::new() );
    //
    // THIS_THREAD_CHECKPOINT_MAP
    /// thread local storage that maps names to index in
    /// THIS_THREAD_CHECKPONT_VEC
    pub(crate) static THIS_THREAD_CHECKPOINT_MAP:
        std::cell::RefCell< std::collections::HashMap<String, usize> > =
            std::cell::RefCell::new( std::collections::HashMap::new() );
}
//
// store_checkpoint
/// Converts an ADFun object to a checkpoint functions for this thread.
///
/// * fun :
/// The ADFun object that it converted to a checkpoint function.
///
/// * name :
/// The name the user chooses for this checkpoint function.
/// This name must not appear in a previous `store_checkpoint` call
/// on this thread.
pub fn store_checkpoint(
    fun:  ADFun,
    name: &String) {
    //
    // fun_index, THIS_THREAD_CHECKPONT_VEC
    let fun_index = THIS_THREAD_CHECKPOINT_VEC.with_borrow_mut( |vec| {
        let index           = vec.len();
        let trace           = false;
        let pattern         = fun.dependency(trace);
        let checkpoint_info = CheckpointInfo {
            fun_index  : index,
            name       : name.clone(),
            adfun      : fun,
            dependency : pattern,
        };
        vec.push( checkpoint_info );
        index
    } );
    //
    // THIS_THREAD_CHECKPOINT_MAP
    THIS_THREAD_CHECKPOINT_MAP.with_borrow_mut( |map| {
        assert!(
            ! map.contains_key(name),
            "store_checkpoint: name {name} was used before on this thread"
        );
        map.insert(name.clone(), fun_index);
    } );
}
//
// use_checkpoint
/// Makes a call, by name, to a checkpoint function.
///
/// If the tape for this thread is recording, include the call
/// as a checkpoint in the tape.
///
/// * name :
/// The name that was used to store the checkpoint function.
///
/// * ad_domain :
/// The value of the domain variables for the function bning called.
///
/// * trace :
/// If this is true (false), a Float evaluation of the
/// checkpoint function corresponding to *ad_domain* is traced.
///
/// * return :
/// The values of the range variables that correspond to the
/// domain variable values.
pub fn use_checkpoint(
    name      : &String,
    ad_domain : &Vec<AD>,
    trace     : bool,
) -> Vec<AD> {
    //
    // fun_index
    let fun_index = THIS_THREAD_CHECKPOINT_MAP.with_borrow( |map| {
        let option_fun_index = map.get(name);
        if option_fun_index == None {
            panic!("use_checkpoint: \
                    name {name} has not been stored as a checkpoint."
            );
        }
        *option_fun_index.unwrap()
    } );
    //
    // checkpoint_info
    let ad_range = THIS_THREAD_CHECKPOINT_VEC.with_borrow( |vec| {
        let check_point_info = &vec[fun_index];
        assert_eq!( fun_index, check_point_info.fun_index );
        let local_key : &LocalKey< RefCell< GTape<Float, Index> > > =
            this_thread_tape();
        let ad_range_zero = local_key.with_borrow_mut( |tape|
            use_checkpoint_info(tape, check_point_info, ad_domain, trace)
        );
        ad_range_zero
    } );
    ad_range
}
//
// use_checkpoint_info
/// Make a call, by CheckpointInfo, to a checkpoint function.
///
/// If the tape for this thread is recording, include the call
/// as a checkpoint in the tape.
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
/// If this is true (false), a Float evaluation of the
/// checkpoint function corresponding to *ad_domain* is traced.
///
/// * return :
/// The values of the range variables that correspond to the
/// domain variable values.
fn use_checkpoint_info(
    tape             : &mut Tape,
    check_point_info : &CheckpointInfo,
    ad_domain        : &Vec<AD>,
    trace            : bool,
) -> Vec<AD> {
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
    let mut domain_zero : Vec<Float> = Vec::new();
    for j in 0 .. call_n_arg {
        domain_zero.push( ad_domain[j].value );
    }
    //
    // range_zero
    let (range_zero, _var_zero) = adfun.forward_zero(&domain_zero, trace);
    //
    // ad_range
    let mut ad_range : Vec<AD> = Vec::new();
    for i in 0 .. call_n_res {
        ad_range.push( AD {tape_id: 0, var_index: 0, value: range_zero[i]} );
    }
    //
    //
    if tape.recording {
        //
        // is_var_domain
        let mut is_var_domain : Vec<bool> = Vec::new();
        for j in 0 .. call_n_arg {
            is_var_domain.push(tape.tape_id == ad_domain[j].tape_id as usize);
        }
        //
        // is_var_range
        let mut is_var_range = vec![false; call_n_res];
        for k in 0 .. dependency.len() {
            let (i,j) = dependency[k];
            if is_var_domain[j as usize] {
                is_var_range[i as usize] = true;
            }
        }
        //
        // tape.id_all, tape.op2arg
        tape.id_all.push( CALL_OP );
        tape.op2arg.push( tape.arg_all.len() as Index );
        //
        // tape.arg_all, tape.con_all
        tape.arg_all.push( fun_index as Index );           // arg[0]
        tape.arg_all.push( call_n_arg as Index );          // arg[1]
        tape.arg_all.push( call_n_res as Index );          // arg[2]
        tape.arg_all.push( tape.flag_all.len() as Index ); // arg[3]
        for j in 0 .. call_n_arg {
            let index = if is_var_domain[j] {
                ad_domain[j].var_index as usize
            } else {
                let con_index = tape.con_all.len();
                tape.con_all.push( ad_domain[j].value );
                con_index
            };
            tape.arg_all.push( index as Index ); // arg[4+j]
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
                ad_range[i].tape_id   = tape.tape_id as Index;
                ad_range[i].var_index = (tape.n_var + n_var_res) as Index;
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
            tape.op2arg.push( tape.arg_all.len() as Index );
        }
    }
    ad_range
}
