// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] dead_code method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    SimpleFloat,
    ADfn,
    IndexT,
};
use crate::adfn::optimize::Depend;
use crate::adfn::optimize::Old2New;
use crate::tape::Tape;
use crate::tape::OpSequence;
use crate::op::binary::is_binary_op;
use crate::ad::ADType;
use crate::op::id::{
    CALL_OP,
    CALL_RES_OP
};
use crate::op::call::{
    BEGIN_DOM,
    BEGIN_FLAG,
    NUMBER_RNG,
};
// -----------------------------------------------------------------------
// set_old2new
fn set_old2new(
    old2new       : &mut Old2New,
    i_op_seq      : usize ,
    old_index     : usize ,
    new_index     : usize ,
    trace         : bool  ,
) {
    if i_op_seq == 0 {
        old2new.dyp[old_index] = new_index as IndexT;
    } else {
        old2new.var[old_index] = new_index as IndexT;
    }
    if trace {
        println!( "{}, {}", old_index, new_index );
    }
}
// -----------------------------------------------------------------------
// get_old2new(
fn get_old2new(
    old2new   : &Old2New ,
    ad_type   : &ADType   ,
    old_index : usize     ,
) -> Option<IndexT> {
    let option : Option<IndexT>;
    match ad_type {
        ADType::ConstantP => {
            let new_index = old2new.cop[old_index];
            let index     = new_index as usize;
            if index < old2new.cop.len() + 1 {
                option = Some(new_index);
            } else {
                option = None;
            }
        },
        ADType::DynamicP  => {
            let new_index = old2new.dyp[old_index];
            let index     = new_index as usize;
            if index < old2new.dyp.len() {
                option = Some(new_index);
            } else {
                option = None;
            }
        },
        ADType::Variable => {
            let new_index = old2new.var[old_index];
            let index     = new_index as usize;
            if index < old2new.var.len() {
                option = Some(new_index);
            } else {
                option = None;
            }
        },
        _  => {
            panic!("dead_code: unexpected argument type {:?}", ad_type)
        },
    }
    option
}
// -----------------------------------------------------------------------
// new_binary_op
fn new_binary_op(
    old2new      : &mut Old2New   ,
    i_op_seq     : usize           ,
    op_id        : u8              ,
    arg          : &[IndexT]       ,
    arg_type     : &[ADType]       ,
    old_op_index : usize           ,
    old_op_seq   : &OpSequence     ,
    new_op_seq   : &mut OpSequence ,
    trace        : bool            ,
) {
    assert_eq!( arg.len(), 2);
    //
    // new_op_index
    let new_op_index = new_op_seq.id_all.len();
    //
    // new_op_seq: id_all, arg_start, n_dep
    new_op_seq.n_dep += 1;
    new_op_seq.id_all.push( op_id );
    new_op_seq.arg_start.push( new_op_seq.arg_all.len() as IndexT );
    //
    // new_op_seq: arg_all, arg_type_all
    for i_arg in 0 .. 2 {
        let arg_type_i = arg_type[i_arg].clone();
        let old_index = arg[i_arg] as usize;
        let option    = get_old2new( &old2new, &arg_type_i, old_index );
        let new_index = option.unwrap();
        new_op_seq.arg_all.push( new_index );
        new_op_seq.arg_type_all.push( arg_type_i );
    }
    // old2new
    let new_index    = new_op_index + new_op_seq.n_dom;
    let old_index    = old_op_index + old_op_seq.n_dom;
    set_old2new(old2new, i_op_seq, old_index, new_index, trace);
    //
}
// -----------------------------------------------------------------------
fn new_call_op(
    old2new          : &mut Old2New    ,
    old_rng_is_dep   : &[bool]          ,
    new_flag         : &[bool]          ,
    i_op_seq         : usize            ,
    arg              : &[IndexT]        ,
    arg_type         : &[ADType]        ,
    old_op_index     : usize            ,
    new_op_seq       : &mut OpSequence  ,
    trace            : bool             ,
) {
    //
    //
    // new_arg, new_arg_type
    let mut new_arg = arg.to_vec();
    new_arg[BEGIN_DOM-1]  = new_op_seq.flag_all.len() as IndexT;
    let mut new_arg_type  = arg_type.to_vec();
    //
    // new_arg, new_arg_type
    let n_dom = new_op_seq.n_dom;
    for i_dom in 0 .. n_dom {
        let old_index  = arg[BEGIN_DOM + i_dom] as usize;
        let ad_type    = &arg_type[BEGIN_DOM + i_dom];
        let option  = get_old2new(&old2new, ad_type, old_index);
        if option.is_none() {
            // A call argument will get optimized out if it is not
            // necessary to obtain the call results that are used.
            new_arg[BEGIN_DOM + i_dom]      = 0; // nan
            new_arg_type[BEGIN_DOM + i_dom] = ADType::ConstantP;
        } else {
            let new_index              = option.unwrap();
            new_arg[BEGIN_DOM + i_dom] = new_index;
        }
    }
    //
    // new_op_index
    let new_op_index = new_op_seq.id_all.len();
    //
    // new_op_seq: n_dep, id_all, arg_start
    new_op_seq.n_dep += 1;
    new_op_seq.id_all.push( CALL_OP );
    new_op_seq.arg_start.push( new_op_seq.arg_all.len() as IndexT );
    //
    // new_op_seq: arg_all, arg_all_type, arg_type_all, flag_all
    let n_arg  = new_arg.len();
    let n_flag = new_flag.len();
    new_op_seq.arg_all.extend_from_slice( &new_arg[0 .. n_arg] );
    new_op_seq.arg_type_all.extend_from_slice( &new_arg_type[0 .. n_arg] );
    new_op_seq.flag_all.extend_from_slice( &new_flag[0 .. n_flag] );
    //
    // old2new, CALL_RES_OP operators
    let new_rng_is_dep = &new_flag[1 .. n_flag];
    let n_rng          = new_rng_is_dep.len();
    assert_eq!( n_rng, old_rng_is_dep.len() );
    let mut old_i_dep = 0;
    let mut new_i_dep = 0;
    for i_rng in 0 .. n_rng {
        if old_rng_is_dep[i_rng] {
            if new_rng_is_dep[i_rng] {
                //
                let old_index = old_op_index + n_dom + old_i_dep;
                let new_index = new_op_index + n_dom + new_i_dep;
                set_old2new(old2new, i_op_seq, old_index, new_index, trace);
                //
                if 0 < new_i_dep  {
                    //
                    // new_op_seq: n_dep, id_all, arg_start, arg_all, arg_type_all
                    new_op_seq.n_dep += 1;
                    new_op_seq.id_all.push( CALL_RES_OP );
                    new_op_seq.arg_start.push(new_op_seq.arg_all.len() as IndexT);
                    new_op_seq.arg_all.push( new_i_dep as IndexT );
                    new_op_seq.arg_type_all.push( ADType::Empty );
                }
                new_i_dep += 1;
            }
            old_i_dep +=1;
        }
    }
}
// -----------------------------------------------------------------------
//
// ADfn::dead_code
impl<V> ADfn<V>
where
    V : Clone,
{   //
    // dead_code
    /// Determine a new tape, with dead code removed,
    /// and the map from ADfn indices to tape indices.
    ///
    /// * Syntax :
    /// ```text
    ///     (tape, old2new) = f.dead_code(depend, trace)
    /// ```
    ///
    /// * f :
    /// is an [ADfn] object.
    ///
    /// * depend :
    /// is the [Depend] structure for *f* .
    ///
    /// * trace :
    /// if true, a trace is printed on standard output.
    ///
    /// * tape :
    /// is a [Tape] corresponding to the optimized version of *f* .
    ///
    /// * old2new  :
    /// is the [Old2New] structure that maps indices in *f* to
    /// indices in *tape* .
    ///
    pub(crate) fn dead_code(&self, depend : &Depend, trace : bool,
    ) -> ( Tape<V>, Old2New)
    where
        V : Clone + SimpleFloat + PartialEq ,
    {
        //
        // self.cop[0]
        let nan :  V  = SimpleFloat::nan();
        assert!( nan == self.cop[0] );
        //
        // tape
        let mut tape : Tape<V> = Tape::new();
        //
        // n_cop, n_dyp, n_var
        let n_cop = self.cop_len();
        let n_dyp = self.dyp_len();
        let n_var = self.var_len();
        //
        // old2new
        // Initialize as an invalid value.
        let mut old2new = Old2New{
                cop : vec![ n_cop as IndexT; n_cop] ,
                dyp : vec![ n_dyp as IndexT; n_dyp ] ,
                var : vec![ n_var as IndexT; n_var ] ,
        };
        //
        if trace {
            println!( "Begin Trace: dead_code");
            println!(
                "n_cop = {}, n_dyp = {}, n_var = {}", n_cop, n_dyp, n_var
            );
            if 0 < n_cop {
                println!( "cop: old_index, new_index" );
            }
        }
        //
        // tape.cop
        //
        let n_cop = self.cop_len();
        for old_index in 0 .. n_cop {
            if depend.cop[old_index] || old_index == 0 {
                let value               = self.cop[old_index].clone();
                let new_index           = tape.cop.len();
                old2new.cop[old_index] = new_index as IndexT;
                tape.cop.push( value );
                println!( "{}, {}", old_index, new_index );
            }
        }
        //
        // i_op_seq
        let start_op_seq = if n_dyp > 0 { 0 } else { 1 };
        for i_op_seq in start_op_seq .. 2 {
            let old_depend      : &Vec<bool>;
            let old_op_seq      : &OpSequence;
            let mut new_op_seq  : &mut OpSequence;
            if i_op_seq == 0 {
                old_depend = &depend.dyp;
                old_op_seq = &self.dyp;
                new_op_seq = &mut tape.dyp;
                if trace {
                    println!( "dyp: old_index, new_index" );
                }
            } else {
                old_depend = &depend.var;
                old_op_seq = &self.var;
                new_op_seq = &mut tape.var;
                if trace {
                    println!( "var: old_index, new_index" );
                }
            }
            //
            // new_op_seq.n_dom, old2new.dyp
            let n_dom        = old_op_seq.n_dom;
            new_op_seq.n_dom = n_dom;
            for old_index in 0 .. n_dom { set_old2new(
                &mut old2new, i_op_seq, old_index, old_index, trace
            ); }
            //
            // old_op_index, first_op
            let mut old_op_index = 0;
            while old_op_index < old_op_seq.n_dep {
                //
                // old_res, op_id
                let old_res = old_op_index + old_op_seq.n_dom;
                let op_id   = old_op_seq.id_all[old_op_index];
                //
                // arg, arg_type
                let start =
                    old_op_seq.arg_start[old_op_index] as usize;
                let end   =
                    old_op_seq.arg_start[old_op_index + 1] as usize;
                let arg       = &old_op_seq.arg_all[start .. end];
                let arg_type  = &old_op_seq.arg_type_all[start .. end];
                //
                if is_binary_op(op_id) {
                    if old_depend[old_res] {
                        //
                        // old2new, new_op_seq
                        new_binary_op(
                            &mut old2new,
                            i_op_seq,
                            op_id,
                            arg,
                            arg_type,
                            old_op_index,
                            old_op_seq,
                            &mut new_op_seq,
                            trace,
                        );
                    }
                    old_op_index += 1;
                } else { if op_id == CALL_OP {
                    let flag_all       = &old_op_seq.flag_all;
                    let n_rng          = arg[NUMBER_RNG] as usize;
                    let start          = arg[BEGIN_FLAG] as usize;
                    let end            = start + n_rng + 1;
                    let trace_this_op  = flag_all[start];
                    let old_rng_is_dep = &flag_all[start+1 .. end];
                    //
                    // old_n_dep, new_n_dep, new_rng_is_dep
                    let mut old_n_dep      = 0;
                    let mut new_any_depend = false;
                    let mut new_flag = vec![false; n_rng + 1];
                    new_flag[0]      = trace_this_op;
                    for i_rng in 0 .. n_rng {
                        if old_rng_is_dep[i_rng] {
                            if old_depend[old_res + old_n_dep] {
                                // This i_rng will be a dependent in new_op_seq
                                new_flag[i_rng + 1] = true;
                                new_any_depend      = true;
                            }
                            old_n_dep += 1;
                        }
                    }
                    if new_any_depend {
                        //
                        // old2new, new_op_seq
                        new_call_op(
                            &mut old2new,
                            &old_rng_is_dep,
                            &new_flag,
                            i_op_seq,
                            arg,
                            arg_type,
                            old_op_index,
                            &mut new_op_seq,
                            trace,
                        );
                    }
                    old_op_index += old_n_dep;
                } else {
                    panic!( "dead_code: op_id = {}", op_id );
                }
            } // while old_op_index <
        } // for i_op_seq in 0 .. 2
    }
    if trace {
        println!( "End Trace: dead_code" );
        }
        return (tape, old2new);
    } // fn dead_code
} // impl
