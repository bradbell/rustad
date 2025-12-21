// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] reverse_depend method.
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
use crate::adfn::optimize;
//
// -----------------------------------------------------------------------
//
// ADfn::reverse_depend
impl<V> ADfn<V>
where
    V               : AtomInfoVec + GlobalOpInfoVec,
    AtomCallback<V> : Clone,
{   //
    // reverse_depend
    /// Determine [optimize::Depend] for this [ADfn].
    pub(crate) fn reverse_depend(&self, trace : bool) -> optimize::Depend {
        //
        // atom_depend, cop_depend, dyp_depend, var_depend
        // work space used to avoid reallocationg vectors
        let mut atom_depend : Vec<usize>  = Vec::new();
        let mut cop_depend  : Vec<IndexT> = Vec::new();
        let mut dyp_depend  : Vec<IndexT> = Vec::new();
        let mut var_depend  : Vec<IndexT> = Vec::new();
        //
        // n_cop, n_dyp, n_var, rng_ad_type, rng_index
        let n_cop         = self.cop_len();
        let n_dyp         = self.dyp_len();
        let n_var         = self.var_len();
        let rng_ad_type   = &self.rng_ad_type;
        let rng_index     = &self.rng_index;
        //
        // depend
        let mut depend = optimize::Depend {
            cop : vec![false; n_cop ],
            dyp : vec![false; n_dyp ],
            var : vec![false; n_var ],
        };
        //
        if trace {
            println!( "Begin Trace: reverse_depend" );
            println!(
                "n_cop = {}, n_dyp = {}, n_var = {}", n_cop, n_dyp, n_var
            );
            println!( "rng_index, type_index, type" );
        }
        //
        // depend
        for i in 0 .. self.rng_ad_type.len() {
            let index = rng_index[i] as usize;
            match rng_ad_type[i] {
                ADType::ConstantP => { depend.cop[index] = true; },
                ADType::DynamicP  => { depend.dyp[index] = true; },
                ADType::Variable  => { depend.var[index] = true; },
                _ => panic!( "reverse_depend: expected an AD type."),
            }
            //
            if trace {
                println!( "{}, {}, {:?}", i, index, &rng_ad_type[i])
            }
        }
        if trace {
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
            let op_seq      : &OpSequence;
            let res_type    : ADType;
            if i_op_seq == 0 {
                op_seq      = &self.var;
                res_type    = ADType::Variable;
            } else {
                op_seq      = &self.dyp;
                res_type    = ADType::DynamicP;
            }
            if trace {
                println!("Begin reverse {:?}", res_type );
                println!( "res, res_type, name, arg, arg_type" )
            }
            //
            // n_dep, flag_all
            let n_dom    = op_seq.n_dom;
            let n_dep    = op_seq.n_dep;
            let flag_all = &op_seq.flag_all;
            //
            // op_index, res, res_depend
            for op_index in (0 .. n_dep).rev() {
                let res        = n_dom + op_index;
                let res_depend = if i_op_seq == 0 {
                    depend.var[res]
                } else {
                    depend.dyp[res]
                };
                if res_depend {
                    //
                    // op_id, arg, arg_type
                    let op_id     = op_seq.id_all[op_index];
                    let start     = op_seq.arg_start[op_index] as usize;
                    let end       = op_seq.arg_start[op_index + 1] as usize;
                    let arg       = &op_seq.arg_all[start .. end];
                    let arg_type  = &op_seq.arg_type_all[start .. end];
                    //
                    if op_id == CALL_OP || op_id == CALL_RES_OP {
                        cop_depend.clear();
                        dyp_depend.clear();
                        var_depend.clear();
                        call_depend::<V>(
                            &mut atom_depend,
                            &mut cop_depend,
                            &mut dyp_depend,
                            &mut var_depend,
                            op_seq,
                            op_index
                        );
                        for dep_index in var_depend.iter() {
                            depend.var[*dep_index as usize] = true;
                        }
                        for dep_index in dyp_depend.iter() {
                            depend.dyp[*dep_index as usize] = true;
                        }
                        for dep_index in cop_depend.iter() {
                            depend.cop[*dep_index as usize] = true;
                        }
                    } else {
                        let reverse_depend =
                            op_info_vec[op_id as usize].reverse_depend;
                        reverse_depend(
                            &mut depend,
                            &flag_all,
                            &arg,
                            &arg_type,
                            res,
                            res_type.clone(),
                        );
                    }
                    if trace {
                        let name = &op_info_vec[op_id as usize].name;
                        println!(
                            "{}, {:?}, {}, {:?}, {:?}",
                            res, res_type, name, arg, arg_type
                        )
                    }
                }
            }
        }
        if trace {
            for res in (0 .. depend.cop.len()).rev() {
                if depend.cop[res] {
                    println!( "{}, ConstantP", res);
                }
            }
            println!( "End Trace: reverse_depend" );
        }
        depend
    }
}
//
#[cfg(test)]
mod tests {
    use crate::{
        AD,
        start_recording_dyp_var,
        stop_recording,
        AzFloat,
        ad_from_value,
        IndexT,
        AtomCallback,
        register_atom,
        call_atom,
    };
    //
    // V
    type V = AzFloat<f64>;
    //
    // eye_forward_fun_value
    fn eye_forward_fun_value(
        _use_range : &[bool]   ,
        domain     : &[&V]     ,
        _call_info : IndexT    ,
        _trace      : bool     ,
    ) -> Result< Vec<V>, String >
    {   // range
        let mut range : Vec<V> = Vec::with_capacity( domain.len() );
        for i in 0 .. domain.len() {
            range.push( domain[i].clone() );
        }
        Ok(range)
    }
    //
    // eye_rev_depend
    fn eye_rev_depend(
        depend       : &mut Vec<usize> ,
        rng_index    : usize           ,
        n_dom        : usize           ,
        _call_info   : IndexT          ,
        _trace       : bool            ,
    ) -> String
    {   assert_eq!( depend.len(), 0 );
        assert!( rng_index < n_dom );
        depend.push(rng_index);
        //
        let error_msg = String::new();
        error_msg
    }
    //
    // register_eye
    fn register_eye() -> IndexT {
        //
        // eye_callback
        let eye_callback = AtomCallback{
            name               : &"eye",
            rev_depend         : Some(eye_rev_depend),
            forward_fun_value  : Some(eye_forward_fun_value) ,
            //
            forward_fun_ad     : None,
            forward_der_value  : None,
            forward_der_ad     : None,
            reverse_der_value  : None,
            reverse_der_ad     : None,
        };
        // eye_atom_id
        let eye_atom_id = register_atom( eye_callback );
        eye_atom_id
    }
    /*
    domain values
    p[0]                        used     dynamic  0
    p[1]                    not used     dynamic  1
    x[0]                        used     variable 0
    x[1]                    not used     variable 1
    dependent values
    q[0] = p[0] + p[1]          used     dynamic  2
    q[1] = p[0] * p[1]      not used     dynamic  3
    y[0] = x[0] + q[0]          used     variable 2
    y[1] = x[0] * q[1]      not used     variable 3
    z[0] = 5
    z[1] = q[0]                 used     dynamic  2
    z[2] = y[0]                 used     variable 2
    // call operator creates new dependents even when equal.
    w[0] = 5
    w[1] = z[1]                 used     dynamic  4
    w[2] = z[2]                 used     variable 4
    */
    #[test]
    fn test_reverse_depend() {
        //
        // trace
        let trace = false;
        //
        // eye_atom_id, call_info
        let eye_atom_id = register_eye();
        let call_info   = 0;
        //
        // f
        let np   = 2;
        let nx   = 2;
        let p    = vec![V::from(1.0); np ];
        let x    = vec![V::from(1.0); nx ];
        let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
        //
        // aq
        let mut aq  : Vec< AD<V> > = Vec::new();
        aq.push( &ap[0] + &ap[0] );  // dynamic with index np
        aq.push( &ap[1] * &ap[1] );  // dynamic with index np + 1
        //
        // ay
        let mut ay  : Vec< AD<V> > = Vec::new();
        ay.push( &ax[0] + &aq[0] );  // variable with index nx
        ay.push( &ax[1] * &aq[1] );  // variable with index nx + 1
        //
        // az
        let mut az  : Vec< AD<V> > = Vec::new();
        az.push( ad_from_value( V::from( 5.0 ) ) );  // constant with index 0
        az.push( aq[0].clone() );  // az[1] = dynamic with index np
        az.push( ay[0].clone() );  // az[2] = variable with index nx
        //
        // aw
        // aw[0] = last constant
        // aw[1] = dynamic with index np + 2 depoends on dynamic with index np
        // aw[2] = variable with index nx + 2 depends on variable with index nx
        let nw = az.len();
        let aw = call_atom(nw, az, eye_atom_id, call_info, trace);
        let f  = stop_recording(aw);
        //
        let p_both      = f.forward_dyp_value(p, trace);
        f.forward_var_value(&p_both, x, trace);
        //
        // depend
        let depend = f.reverse_depend(trace);
        //
        // depend.cop
        // TODO: There are four constants, but should only be two;
        // The nan at index zero and the five in z[0].
        assert_eq!( depend.cop, [false, false, false, true] );
        //
        // depend.dyp
        assert_eq!( depend.dyp, [true, false, true, false, true] );
        //
        // depend.var
        assert_eq!( depend.var, [true, false, true, false, true] );
    }
}
