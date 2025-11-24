// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] optimize method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn,
    AtomCallback,
    IndexT,
};
use crate::op::{
    info::sealed::GlobalOpInfoVec,
    call::call_depend,
    id::CALL_OP,
    id::CALL_RES_OP,
};
use crate::ad::ADType;
use crate::tape::OpSequence;
use crate::atom::sealed::AtomInfoVec;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// OptimizeDepend
/// Which constants, dynamic parameters, and variables the
/// range for an [ADfn] depends on.
///
/// TODO: change to private when reverse_depend gets changes to private.
pub struct OptimizeDepend {
    // cop
    /// Constant parameters dependency; length [ADfn::cop_len].
    pub(crate) cop : Vec<bool> ,
    //
    // dyp
    /// Dynamic parameters dependency; length [ADfn::dyp_len].
    pub(crate) dyp : Vec<bool> ,
    //
    // var
    /// Variable dependency; length [ADfn::var_len].
    pub(crate) var : Vec<bool> ,
}
//
// ADfn::reverse_depend
impl<V> ADfn<V>
where
    V               : AtomInfoVec + GlobalOpInfoVec,
    AtomCallback<V> : Clone,
{   //
    // reverse_depend
    /// Determine [OptimizeDepend] for this [ADfn].
    /// TODO: change to private when this gets used by a public function.
    pub fn reverse_depend(&self, trace : bool) -> OptimizeDepend {
        //
        // atom_depend, dyp_depend, var_depend
        // work space used to avoid reallocationg vectors
        let mut atom_depend : Vec<usize>  = Vec::new();
        let mut dyp_depend  : Vec<IndexT> = Vec::new();
        let mut var_depend  : Vec<IndexT> = Vec::new();
        //
        // n_cop, n_dyp, n_var, range_ad_type, range_index
        let n_cop         = self.cop_len();
        let n_dyp         = self.dyp_len();
        let n_var         = self.var_len();
        let range_ad_type = &self.range_ad_type;
        let range_index   = &self.range_index;
        //
        // depend
        let mut depend = OptimizeDepend {
            cop : vec![false; n_cop ],
            dyp : vec![false; n_dyp ],
            var : vec![false; n_var ],
        };
        //
        if trace {
            println!( "Begin Trace: reverse_depend" );
            println!( "n_cop = {}, n_dyp = {}, n_var = {}", n_cop, n_dyp, n_var);
            println!( "range_index, ADType" );
        }
        //
        // depend
        for i in 0 .. self.range_ad_type.len() {
            let index = range_index[i] as usize;
            match range_ad_type[i] {
                ADType::ConstantP => { depend.cop[index] = true; },
                ADType::DynamicP  => { depend.dyp[index] = true; },
                ADType::Variable  => { depend.var[index] = true; },
                _ => panic!( "reverse_depend: expected an AD type."),
            }
            //
            if trace {
                println!( "{}, {:?}", index, &range_ad_type[i])
            }
        }
        if trace {
            println!( "res, res_type, name, arg, arg_type" )
        }
        //
        // op_info_vec
        let op_info_vec = &*<V as GlobalOpInfoVec>::get();
        //
        //
        // i_op_seq
        for i_op_seq in 0 .. 2 {
            //
            // op_seq
            let op_seq   : &OpSequence;
            let res_type : ADType;
            if i_op_seq == 0 {
                op_seq    = &self.var;
                res_type  = ADType::Variable;
            } else {
                op_seq    = &self.dyp;
                res_type  = ADType::DynamicP;
            }
            //
            // n_dep, flag_all
            let n_dom    = op_seq.n_dom;
            let n_dep    = op_seq.n_dep;
            let flag_all = &op_seq.flag_all;
            //
            // op_index, op_id
            for op_index in (0 .. n_dep).rev() {
                let op_id     = op_seq.id_seq[op_index];
                //
                if op_id == CALL_OP || op_id == CALL_RES_OP {
                    dyp_depend.clear();
                    var_depend.clear();
                    call_depend::<V>(
                        &mut atom_depend,
                        &mut dyp_depend,
                        &mut var_depend,
                        &self.var,
                        op_index
                     );
                    for dep_index in var_depend.iter() {
                        depend.var[*dep_index as usize] = true;
                    }
                    for dep_index in dyp_depend.iter() {
                        depend.dyp[*dep_index as usize] = true;
                    }
                } else {
                    let op_id     = op_id as usize;
                    let start     = op_seq.arg_seq[op_index] as usize;
                    let end       = op_seq.arg_seq[op_index + 1] as usize;
                    let arg       = &op_seq.arg_all[start .. end];
                    let arg_type  = &op_seq.arg_type_all[start .. end];
                    let res       = n_dom + op_index;
                    let reverse_depend = op_info_vec[op_id].reverse_depend;
                    reverse_depend(
                        &mut depend,
                        &flag_all,
                        &arg,
                        &arg_type,
                        res,
                        res_type.clone(),
                    );
                    if trace {
                        let name = &op_info_vec[op_id].name;
                        println!(
                            "{}, {:?}, {}, {:?}, {:?}",
                            res, res_type, name, arg, arg_type
                        )
                    }
                }
            }
        }
        if trace {
            println!( "depend.cop = {:?}", depend.cop );
            println!( "depend.dyp = {:?}", depend.dyp );
            println!( "depend.var = {:?}", depend.var );
        }
        depend
    }
}
//
#[test]
fn test_reverse_depend() {
    use crate::{
        AD,
        start_recording_dyp_var,
        stop_recording,
        AzFloat,
        ad_from_value,
    };
    //
    // trace
    let trace = false;
    //
    // V
    type V = AzFloat<f64>;
    //
    // f
    let np   = 2;
    let nx   = 2;
    let p    = vec![V::from(1.0); np ];
    let x    = vec![V::from(1.0); nx ];
    let (ap, ax) = start_recording_dyp_var(p, x);
    //
    // aq
    let mut aq  : Vec< AD<V> > = Vec::new();
    aq.push( &ap[0] + &ap[0] );  // will be used
    aq.push( &ap[1] * &ap[1] );  // will not be used
    //
    // ay
    let mut ay  : Vec< AD<V> > = Vec::new();
    ay.push( &ax[0] + &aq[0] );  // will be used
    ay.push( &ax[1] * &aq[1] );  // will not be used
    //
    // f
    let mut az  : Vec< AD<V> > = Vec::new();
    az.push( ad_from_value( V::from( 5.0 ) ) ); // this constant is used
    az.push( aq[0].clone() );  // aq[0] and ap[0] are used
    az.push( ay[0].clone() );  // ay[0] and ax[0] are used
    let f = stop_recording(az);
    //
    // depend
    let depend = f.reverse_depend(trace);
    //
    // depend
    assert_eq!( depend.cop, vec![ true ] );
    assert_eq!( depend.dyp, vec![ true, false, true, false ] );
    assert_eq!( depend.var, vec![ true, false, true, false ] );
}
