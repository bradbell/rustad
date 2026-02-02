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
/// Sparse Jacobian evaluation using reverse mode.
///
/// * Syntax :
///   ```text
///     jacobian = f.rev_sparse_jac_value(
///         dyp_both, &var_both, &sub_pattern, &color_vec, trace
///     )
///     jacobian = f.rev_sparse_jac_ad(
///         dyp_both, &var_both, &sub_pattern, &color_vec, trace
///     )
///   ```
///
/// * Prototype :
///   see [ADfn::rev_sparse_jac_value] and  [ADfn::rev_sparse_jac_ad]
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
///   This is a subset of the sparsity pattern for
///   the transpose of the Jacobain of f.
///   All of the column (row) indices must be less than
///   the range dimension (variable domain dimension) for this ADfn object.
///
/// * color_vec :
///   This is a coloring correspoding to the transpose of the Jacobian matrix 
///   for f evalued on the subset specified by *sub_pattern*.
///
/// * trace :
///   if true, a trace of the calculations is printed on stdout.
///
/// * jacobian :
///   The return is the transpose of the Jacobian on the 
///   subset sparsity pattern.
///   To be specific, it has the same length as *sub_pattern* and for each k,
///   `jacobian[k]` is the Jacobian at row index `sub_pattern[k][1]`
///   and column index `sub_pattern[k][2]` .
pub fn doc_rev_sparse_jac() {}
//
/// Create the rev_sparse_jac functions
///
/// * suffix : is either `value` or `ad` ;
/// * V      : see [doc_generic_v]
/// * E      : see [doc_generic_e] .
///
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_rev_sparse_jac]
macro_rules! rev_sparse_jac {
    ($suffix:ident, $V:ident, $E:ty) => {paste::paste! {
        #[doc = concat!(
            "`", stringify!($E), "` evaluation of of sparse Jacobians; ",
            "see [doc_rev_sparse_jac]",
        )]
        pub fn [< rev_sparse_jac_ $suffix >] (
            &self,
            dyp_both     : Option< &Vec<$E> >  ,
            var_both     : &Vec<$E>            ,
            sub_pattern  : &SparsityPattern    ,
            color_vec    : &[usize]            ,
            trace        : bool                ,
        ) -> Vec<$E>
        {   //
            // n
            let m = self.rng_len();
            debug_assert!( m == color_vec.len() );
            //
            // n_color
            let n_color =
                color_vec.iter().filter(|&k| k < &m ).max().unwrap() + 1;
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
            // color
            for color in 0 .. n_color {
                //
                // dom_der
                let mut range_der : Vec<$E> = Vec::with_capacity(m);
                for i in 0 .. m {
                    if color_vec[i] == color {
                        range_der.push( one_e.clone() );
                    } else {
                        range_der.push( zero_e.clone() );
                    }
                }
                // dom_der
                let dom_der = self. [< reverse_der_ $suffix >](
                    dyp_both, &var_both, range_der, trace
                );
                //
                let [mut j, mut i] = sub_pattern[ order[index] ];
                while index < sub_pattern.len() && color_vec[i] == color {
                    // TODO: figure out how to do this without a clone
                    jacobian[ order[index] ] = dom_der[j].clone();
                    index                   += 1;
                    if index < sub_pattern.len() {
                        [j, i] = sub_pattern[ order[index] ];
                    }
                }
            }
            debug_assert!( index == sub_pattern.len() );
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
    // rev_sparse_jac
    rev_sparse_jac!( value, V, V );
    rev_sparse_jac!( ad,    V, AD::<V> );
}
