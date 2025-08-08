// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module implements [GADFun] methods that use forward mode
//! : [parent module](super)
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::function::GADFun;
use crate::gas::as_from;
use crate::gas::sealed::GenericAs;
use crate::operator::GlobalOpInfoVec;
use crate::vec_set::VecSet;
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::doc_generic_f_and_u;
//
impl<F,U> GADFun<F,U>
where
    F     : GlobalOpInfoVec<U> ,
    U     : 'static + Copy + GenericAs<usize> + std::fmt::Debug ,
    usize : GenericAs<U>,
{
    // for_sparsity
    /// Use the forward mode to compute a Jacobian sparsity pattern.
    ///
    /// * Syntax :
    /// ```text
    ///     pattern = f.for_sparsity(trace)
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
    /// let mut pattern = f.for_sparsity(trace);
    /// pattern.sort();
    /// assert_eq!( pattern.len(), 3 );
    /// assert_eq!( pattern[0], (0,0) );
    /// assert_eq!( pattern[1], (1,1) );
    /// assert_eq!( pattern[2], (2,2) );
    ///```
    pub fn for_sparsity(&self, trace : bool) -> Vec<(U, U)>
    {   //
        // op_info_vec
        let op_info_vec = &*< F as GlobalOpInfoVec<U> >::get();
        //
        // n_domain, n_var, flag_all, arg_all, op2arg,
        // range_is_var, range2tape_index, n_range
        let n_domain          = self.n_domain;
        let id_all            = &self.id_all;
        let flag_all          = &self.flag_all;
        let arg_all           = &self.arg_all;
        let op2arg            = &self.op2arg;
        let range_is_var      = &self.range_is_var;
        let range2tape_index  = &self.range2tape_index;
        let n_range           = range_is_var.len();
        //
        // result, arg_var_index, arg_var_usize
        let mut result          : Vec<(U, U)> = Vec::new();
        let mut arg_var_index   : Vec<U>      = Vec::new();
        let mut arg_var_usize   : Vec<usize>  = Vec::new();
        let mut set_vec         : VecSet      = VecSet::new();
        //
        // set_vec.get(id_set) for id_set = 0 .. n_domain
        for id_set in 0 .. n_domain {
            set_vec.singleton( id_set );
        }
        //
        if trace {
            println!(
                "Begin Trace: for_sparisty: n_domain = {}, n_range = {}",
                 n_domain, n_range
            );
            println!("var_index, operator, var_arguments, set_result");
        }
        //
        // op_index
        for op_index in 0 .. id_all.len() {
            //
            // op_info, arg_var_index_fn
            let op_id    : usize = as_from( id_all[op_index] );
            let op_info          = &op_info_vec[op_id];
            let arg_var_index_fn = op_info.arg_var_index;
            //
            // arg
            let begin : usize = as_from( op2arg[op_index] );
            let end   : usize = as_from(op2arg[op_index + 1]);
            let arg           = &arg_all[begin .. end];
            //
            // arg_var_index
            // the variables that are arguments to this operator
            arg_var_index_fn(&mut arg_var_index, &flag_all, arg);
            //
            // arg_var_usize
            let n_arg = arg_var_index.len();
            arg_var_usize.resize(n_arg, 0);
            for i in 0 .. n_arg {
                arg_var_usize[i] = as_from( arg_var_index[i] );
            }
            //
            // var_index
            // this is the operation sequence variable index that
            // we are computing the dependency for.
            let var_index = n_domain + op_index;
            //
            // set_vec.get(var_index)
            let set_id = set_vec.union( &arg_var_usize );
            assert_eq!( var_index,  set_id );
            //
            if trace {
                let name = &op_info_vec[op_id].name;
                let set  = set_vec.get(var_index);
                println!(
                    "{}, {}, {:?}, {:?}",
                    var_index, name, arg_var_index,  set
                );
            }
        }
        for i in 0 .. n_range {
            let row_var_index : usize = as_from( range2tape_index[i] );
            let set  = set_vec.get(row_var_index);
            for j in 0 .. set.len() {
                let row_u : U = as_from( i );
                let col_u : U = as_from( set[j] );
                result.push( (row_u, col_u) );
            }
        }
        if trace {
            println!( "n_pattern = {}", result.len() );
        }
        result
    } // end: pub fn for_sparsity(&self, trace : bool) -> Vec<(U, U)>
} // end: impl<F,U> GADFun<F,U>
