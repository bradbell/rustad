// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_dyp method (compute dynamic parameters)
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn,
    FConst,
};
use crate::op::info::sealed::GlobalOpFnsVec;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::ConstData;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// forward_dyp
/// Compute the dependent dynamic parameters.
///
/// * Syntax :
///   ```text
///     dyp_all  = f.forward_dyp_value(dyp_dom, arg_vec)
///     dyp_all  = f.forward_dyp_ad(dyp_dom, arg_vec)
///   ```
/// * Prototype :
///   see [ADfn::forward_dyp_value] and [ADfn::forward_dyp_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * f : is an [ADfn] object.
///
/// * arg_vec :
///   is an [arg_vec](crate::doc_arg_vec) with the following possible keys:
///
///   * trace
///     The corresponding value must be true of false (default is false).
///     If it is true, a trace of forward_dyp is printed on stdout.
///
/// * dyp_dom :
///   specifies the domain space dynamic parameter values.
///
/// * dyp_all  :
///   is both the dynamic parameter sub-vectors in the following order:
///   the domain dynamic parameters followed by the dependent dynamic parameters.
///   Note that *dyp_dom* gets moved to the beginning of *dyp_all* .
///
pub fn doc_forward_dyp() { }
//
/// Create the member function that evaluates the dependent dynamic parameters.
///
/// * suffix :
///   is either `value` or `ad` ;
///
/// * V : see [doc_generic_v]
///
/// * E : see [doc_generic_e] .
///   If *suffix* is `value` , *E must be be the value type *V* .
///   If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_forward_dyp]
macro_rules! forward_dyp {
    ( $suffix:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!( " `", stringify!($E), "`",
        " evaluation of dependent dynamic parameters; see [doc_forward_dyp]" ,
        )]
        pub fn [< forward_dyp_ $suffix >] (
            &self,
            dyp_dom     : Vec<$E>         ,
            arg_vec     : &Vec<[&str; 2]> ,
        ) -> Vec<$E>
        {
            // trace
            let mut trace = false;
            for arg in arg_vec {
                match arg[0] {
                    "trace" => {
                        match arg[1] {
                            "true"  => { trace = true; },
                            "false" => { trace = false; },
                            _ => { panic!(
                            "forward_dyp arg_vec: invalid value for trace"
                            ); }
                        }
                    },
                    _ => panic!("forward_dyp arg_vec: invalid key"),
                }
            }
            //
            // dyp_dom
            assert_eq!(
                dyp_dom.len(), self.dyp.n_dom,
                "f.forward_dyp: dyp_dom vector length does not match f"
            );
            //
            // op_fns_vec
            let op_fns_vec = GlobalOpFnsVec::get();
            //
            // n_dyp
            let n_dyp = self.dyp.n_dom + self.dyp.n_dep;
            //
            // dyp_all
            let nan_e        = $E::nan();
            let mut dyp_all  = dyp_dom;
            dyp_all.resize(n_dyp, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_dyp_{}", stringify!($suffix) );
                println!( "index, bool" );
                for j in 0 .. self.dyp.bool_all.len() {
                    println!( "{}, {:?}", j, self.dyp.bool_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "dyp_index, dyp_dom" );
                for j in 0 .. self.dyp.n_dom {
                    println!( "{}, {}", j, dyp_all[j] );
                }
                println!( "index, dyp_all, op_name, arg, arg_type" );
            }
            //
            // cop, bool_all
            let cop      = &self.cop;
            let bool_all = &self.dyp.bool_all;
            //
            // dyp_all
            for op_index in 0 .. self.dyp.id_all.len() {
                let op_id    = self.dyp.id_all[op_index] as usize;
                let start    = self.dyp.arg_start[op_index] as usize;
                let end      = self.dyp.arg_start[op_index + 1] as usize;
                //
                let arg      = &self.dyp.arg_all[start .. end];
                let arg_type = &self.dyp.arg_type_all[start .. end];
                let res      = self.dyp.n_dom + op_index;
                //
                let const_data = ConstData {
                    cop, bool_all, arg, arg_type, res
                };
                //
                let forward_dyp = op_fns_vec[op_id].[< forward_dyp_ $suffix >];
                forward_dyp(
                    &mut dyp_all,
                    const_data,
                );
                if trace {
                    let name = &op_fns_vec[op_id].name;
                    println!( "{}, {}, {}, {:?}, {:?}",
                        res, dyp_all[res], name, arg, arg_type
                    );
                }
            }
            if trace {
                println!("End Trace: forward_dyp_{}", stringify!($suffix));
            }
            dyp_all
        }
    }
} }
//
impl<V> ADfn<V> where
V : Clone + std::fmt::Display + GlobalOpFnsVec + FConst + ThisThreadTape,
{   //
    // forward_dyp
    forward_dyp!( value, V );
    forward_dyp!( ad,    AD::<V> );
}

#[cfg(test)]
mod tests {
    use crate::{
        AD,
        AzFloat,
        start_recording,
        stop_recording,
    };
    //
    #[test]
    fn test_forward_dyp() {
        //
        // V
        type V = AzFloat<f32>;
        //
        // f
        let np         = 3;
        let nx         = 1;
        let p : Vec<V> = vec![ V::from(1.0); np ];
        let x : Vec<V> = vec![ V::from(1.0 ); nx ];
        let (ap, ax)   = start_recording( Some(p.clone()), x.clone());
        //
        // asum
        // The first addition adds the constants zero and so is not recorded
        let mut asum   = AD::from( V::from(0.0) );
        for j in 0 .. np {
            asum += &ap[j];
        }
        //
        // f
        let ay = vec![ &ax[0] * &asum ];
        let f  = stop_recording(ay);
        //
        // dyp_all
        let arg_vec  : Vec<[&str; 2]> = Vec::new();
        let dyp_all  = f.forward_dyp_value(p.clone(), &arg_vec);
        //
        assert_eq!( dyp_all.len(), 2 * np - 1 );
        for j in 0 .. np {
            assert_eq!( dyp_all[j], p[j] );
        }
        let mut sum = p[0];
        for j in 1 .. np {
            sum += &p[j];
            assert_eq!( dyp_all[np + j - 1], sum );
        }
    }
}
