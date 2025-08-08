// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module implements [GADFun] methods that use subgraphs
//! : [parent module](super)
//!
//! Method List :
//! 1. [GADFun::sub_sparsity]
//
use super::GADFun;
use crate::operator::GlobalOpInfoVec;
use crate::gas::sealed::GenericAs;
use crate::gas::as_from;
//
#[cfg(doc)]
use crate::doc_generic_f_and_u;
//
// ---------------------------------------------------------------------------
// ADFun::sub_sparsity
impl<F,U> GADFun<F,U>
where
    F     : GlobalOpInfoVec<U> ,
    U     : Copy + 'static + GenericAs<usize> + std::cmp::PartialEq,
    usize : GenericAs<U>,
{
    // sub_sparsity
    /// Use the subgraph method to compute a Jacobian sparsity pattern.
    ///
    /// * Syntax :
    /// ```text
    ///     pattern = f.sub_sparsity(trace)
    /// ```
    ///
    /// * F, U : see [doc_generic_f_and_u]
    ///
    /// * f :
    /// is this [GADFun] object. The sparsity pattern is for the Jacobian
    /// of the function defined by the operation sequence stored in f.
    ///
    /// * trace :
    /// If trace is true, a trace of the sparsoty calculation
    /// is printed on standard output.
    /// Note that in the trace, the cases where *var_index* is less
    /// that the number of domain variables will end up in the pattern
    /// with the corresponding row.
    ///
    /// * pattern :
    /// The the return value *pattern* is vector of (row, column) pairs.
    /// Each row (column) is less than the range (domain)
    /// dimension for the function.
    /// If a pair (i, j) does not appear, the range component
    /// with index i does not depend on the domain component with index j.
    ///
    /// * dependency :
    /// This is a dependency pattern. For example,
    /// if an range variable was equal to the
    /// Heaviside function of a domain variable,
    /// the corresponding pair would be in the sparisty pattern even though
    /// the corresponding derivative is always zero.
    ///```
    /// use rustad::function;
    /// use rustad::gad::GAD;
    /// type AD = GAD<f32, u64>;
    /// let x  : Vec<f32> = vec![1.0, 2.0, 3.0];
    /// let ax : Vec<AD>  = function::ad_domain(&x);
    /// let mut ay : Vec<AD> = Vec::new();
    /// for j in 0 .. x.len() {
    ///     ay.push( ax[j] * ax[j] );
    /// }
    /// let f           = function::ad_fun(&ay);
    /// let trace       = false;
    /// let mut pattern = f.sub_sparsity(trace);
    /// pattern.sort( );
    /// assert_eq!( pattern.len(), 3 );
    /// assert_eq!( pattern[0], (0,0) );
    /// assert_eq!( pattern[1], (1,1) );
    /// assert_eq!( pattern[2], (2,2) );
    ///```
    pub fn sub_sparsity(&self, trace : bool) -> Vec<(U, U)>
    {   //
        // op_info_vec
        let op_info_vec = &*< F as GlobalOpInfoVec<U> >::get();
        //
        // n_domain, n_var, flag_all, arg_all, op2arg,
        // range_is_var, range2tape_index, n_range
        let n_domain          = self.n_domain;
        let n_var             = self.n_var;
        let flag_all          = &self.flag_all;
        let arg_all           = &self.arg_all;
        let op2arg            = &self.op2arg;
        let range_is_var      = &self.range_is_var;
        let range2tape_index  = &self.range2tape_index;
        let n_range           = range_is_var.len();
        //
        // done
        let n_var_u : U = as_from(n_var);
        let mut done : Vec<U> = vec![n_var_u; n_var];
        //
        // result, arg_var_index, var_index_stack
        let mut result          : Vec<(U, U)> = Vec::new();
        let mut arg_var_index   : Vec<U> = Vec::new();
        let mut var_index_stack : Vec<U> = Vec::new();
        //
        if trace {
            println!( "n_domain = {}, n_range = {}", n_domain, n_range );
        }
        //
        // row
        // determine the variables that range index row depends on
        for row in 0 .. n_range { if range_is_var[row] {
            //
            // var_index
            let mut var_index : usize = as_from( range2tape_index[row] );
            if trace {
                println!( "row {} var_index {}", row, var_index );
            }
            //
            // var_index_stack
            // use resize instead of new stack to reduce memory allocation
            let zero_u : U = as_from(0);
            var_index_stack.resize(0, zero_u);
            var_index_stack.push( as_from( var_index) );
            while var_index_stack.len() > 0 {
                //
                // var_index
                let var_index_u = var_index_stack.pop().unwrap();
                var_index = as_from( var_index_u );
                //
                let row_u : U = as_from(row);
                if done[var_index] != row_u {
                    done[var_index] = row_u;
                    if trace {
                        println!( "    var_index = {}", var_index );
                    }
                    if var_index < n_domain {
                        //
                        // result
                        // var_index is a domain variable index
                        result.push( (row_u, var_index_u) );
                    } else {
                        //
                        // op_index
                        // the operator that creates this variable
                        let op_index         = var_index - n_domain;
                        //
                        // arv_var_index_fn
                        let op_id : usize =
                            as_from( self.id_all[op_index] );
                        let op_info          = &op_info_vec[op_id];
                        let arg_var_index_fn = op_info.arg_var_index;
                        //
                        // arg
                        let begin : usize = as_from( op2arg[op_index] );
                        let end : usize = as_from(op2arg[op_index + 1]);
                        let arg   = &arg_all[begin .. end];
                        //
                        // arg_var_index
                        // the variables that are arguments to this operator
                        arg_var_index_fn(&mut arg_var_index, &flag_all, arg);
                        //
                        // var_index_stack
                        for i in 0 .. arg_var_index.len() {
                            var_index_stack.push( arg_var_index[i] );
                        }
                    }
                }
            }
        } }
        if trace {
            println!( "n_pattern = {}", result.len() );
        }
        result
    }
}
