// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module can be used to checkpoint a section of AD computation.
//
use crate::{Index, Float};
use crate::function::ADFun;
use crate::ad::AD;
use crate::ad_tape::{Tape, THIS_THREAD_TAPE};
use crate::operator::id::{CALL_OP};
//
pub(crate) struct CheckpointInfo {
    pub fun_index    : Index,
    pub name         : String,
    pub adfun        : ADFun,
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
    /// thread local storage mapping names to index in THIS_THREAD_ADFUN_VEC
    pub(crate) static THIS_THREAD_CHECKPOINT_MAP:
        std::cell::RefCell< std::collections::HashMap<String, Index> > =
            std::cell::RefCell::new( std::collections::HashMap::new() );
}
//
// store_checkpoint
/// Stores checkpoint functions for this thread.
pub fn store_checkpoint(fun: ADFun, name: String) {
    //
    // fun_index, THIS_THREAD_ADFUN_VEC
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
            ! map.contains_key(&name),
            "store_checkpoint: name {name} was used before on this thread"
        );
        map.insert(name, fun_index);
    } );
}
//
// use_checkpoint
/// Make a call (by name) to a checkpoint function and possibly record it.
pub fn use_checkpoint(name : &String, ad_domain : &Vec<AD>) -> Vec<AD> {
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
        let ad_range_zero = THIS_THREAD_TAPE.with_borrow_mut( |tape|
            use_checkpoint_info(tape, check_point_info, ad_domain)
        );
        ad_range_zero
    } );
    ad_range
}
//
// use_checkpoint_info
/// Make a call (by info) to a checkpoint function and possibly record it.
fn use_checkpoint_info(
    tape : &mut Tape, check_point_info : &CheckpointInfo, ad_domain : &Vec<AD>
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
    let trace = false;
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
            is_var_domain.push( tape.tape_id == ad_domain[j].tape_id );
        }
        //
        // is_var_range
        let mut is_var_range = vec![false; call_n_res];
        for k in 0 .. dependency.len() {
            let (i,j) = dependency[k];
            if is_var_domain[j] {
                is_var_range[i] = true;
            }
        }
        //
        // tape.id_all, tape.op2arg
        tape.id_all.push( CALL_OP );
        tape.op2arg.push( tape.arg_all.len() );
        //
        // tape.arg_all, tape.con_all
        tape.arg_all.push( fun_index );           // arg[0]
        tape.arg_all.push( call_n_arg );          // arg[1]
        tape.arg_all.push( call_n_res );          // arg[2]
        tape.arg_all.push( tape.flag_all.len() ); // arg[3]
        for j in 0 .. call_n_arg {
            if is_var_domain[j] {
                ad_range[j].tape_id   = tape.tape_id;
                ad_range[j].var_index = tape.n_var;
                tape.arg_all.push( tape.n_var );         // arg[4 + j]
                tape.n_var += 1;
            } else {
                tape.arg_all.push( tape.con_all.len() ); // arg[4 + j]
                tape.con_all.push( ad_domain[j].value );
            }
        }
        //
        // tape.flag_all
        for j in 0 .. call_n_arg {
            tape.flag_all.push( is_var_domain[j] );
        }
        for i in 0 .. call_n_res {
            tape.flag_all.push( is_var_range[i] );
        }
    }
    ad_range
}
