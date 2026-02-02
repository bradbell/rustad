// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] sparse Jacobian methods.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn,
    FloatCore,
    SparsityPattern,
};
use crate::op::info::sealed::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
#[cfg(doc)]
//
// -----------------------------------------------------------------------
// sparse_jac
/// Sparse Jacobian evaluation using forward mode.
///
/// * Syntax :
///   ```text
///     jacobian = f.for_sparse_jac_value(
///         dyp_both, &var_both, &sub_pattern, &color_vec, trace
///     )
///     jacobian = f.for_sparse_jac_ad(
///         dyp_both, &var_both, &sub_pattern, &color_vec, trace
///     )
///   ```
///
/// * Prototype :
///   see [ADfn::for_sparse_jac_value] and  [ADfn::for_sparse_jac_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
/// * f : is an [ADfn] object.
///
/// * dyp_both :
///   If there are no dynamic parameters in f, this should be None
///   or the empty vector.
///   Otherwise it is the dynamic parameter sub-vectors in the following order:
///   domain dynamic parameters followed by dependent dynamic parameters.
///   This is normally computed by
///   [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) .
///
/// * var_both :
///   is both the variable sub-vectors in the following order:
///   the domain variables followed by the dependent variables.
///   This is normally computed by
///   [forward_var](crate::adfn::forward_var::doc_forward_var) .
///
/// * sub_pattern :
///   This is a subset of the sparsity for the Jacobian of f.
///   All of the column (row) indices must be less than
///   the variable domain dimension (range dimension) for this ADfn object.
///
/// * color_vec :
///   This is a coloring for the Jacobian matrix for f evaluated on the
///   subset specified by *sub_pattern* .
///
/// * trace :
///   if true, a trace of the calculations is printed on stdout.
///
/// * jacobian :
///   The return is the Jacobian on the subset sparsity pattern.
///   To be specific, it has the same length as *sub_pattern* and for each k,
///   `jacobian[k]` is the Jacobian at row index `sub_pattern[k][0]`
///   and column index `sub_pattern[k][1]` .
pub fn doc_for_sparse_jac() {}
//
/// Create the for_sparse_jac functions
///
/// * suffix : is either `value` or `ad` ;
/// * V      : see [doc_generic_v]
/// * E      : see [doc_generic_e] .
///
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_for_sparse_jac]
macro_rules! for_sparse_jac {
    ($suffix:ident, $E:ty) => {paste::paste! {
        #[doc = concat!(
            "`", stringify!($E), "` evaluation of of sparse Jacobians; ",
            "see [doc_for_sparse_jac]",
        )]
        pub fn [< for_sparse_jac_ $suffix >] (
            &self,
            dyp_both     : Option< &Vec<$E> >  ,
            var_both     : &Vec<$E>            ,
            sub_pattern  : &SparsityPattern    ,
            color_vec    : &[usize]            ,
            trace        : bool                ,
        ) -> Vec<$E>
        {   //
            // n
            let n = self.var_dom_len();
            debug_assert!( n == color_vec.len() );
            //
            // n_color
            let n_color =
                color_vec.iter().filter(|&k| k < &n ).max().unwrap() + 1;
            //
            // zero_e, one_e
            let zero_e : $E = FloatCore::zero();
            let one_e  : $E = FloatCore::one();
            //
            // order
            let mut order : Vec<usize> = (0 .. sub_pattern.len()).collect();
            order.sort_by_key( |&ell| color_vec[ sub_pattern[ell][1] ] );
            //
            // index
            let mut index = 0;
            //
            // jacobian
            let mut jacobian = vec![zero_e.clone(); sub_pattern.len()];
            //
            if trace {
                println!("Begin Trace: for_sparse_jac: n = {}", n);
                println!("color_vec = {:?}", color_vec);
            }
            //
            // color
            for color in 0 .. n_color {
                if trace {
                    println!( "color = {}", color);
                }
                //
                // dom_der
                let mut dom_der : Vec<$E> = Vec::with_capacity(n);
                for j in 0 .. n {
                    if color_vec[j] == color {
                        dom_der.push( one_e.clone() );
                    } else {
                        dom_der.push( zero_e.clone() );
                    }
                }
                // range_der
                let range_der = self. [< forward_der_ $suffix >](
                    dyp_both, &var_both, dom_der, trace
                );
                //
                let [mut i, mut j] = sub_pattern[ order[index] ];
                while index < sub_pattern.len() && color_vec[j] == color {
                    // TODO: figure out how to do this without a clone
                    jacobian[ order[index] ] = range_der[i].clone();
                    index                   += 1;
                    if index < sub_pattern.len() {
                        [i, j] = sub_pattern[ order[index] ];
                    }
                }
            }
            debug_assert!( index == sub_pattern.len() );
            if trace {
                println!("End Trace: for_sparse_jac");
            }
            //
            jacobian
        }
    }
}}
//
impl<V> ADfn<V>
where
    V : Clone + std::fmt::Display + GlobalOpInfoVec + FloatCore,
{   //
    // for_sparse_jac
    for_sparse_jac!( value, V );
    for_sparse_jac!( ad,    AD::<V> );
}
