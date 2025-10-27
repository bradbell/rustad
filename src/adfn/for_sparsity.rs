// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Implements [ADfn] forward sparsity method.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
//
use crate::vec_set::VecSet;
use crate::ADfn;
use crate::ADType;
use crate::IndexT;
use crate::op::info::GlobalOpInfoVec;
use crate::op::info::OpInfo;
//
#[cfg(doc)]
use crate::doc_generic_v;
// ----------------------------------------------------------------------------
// ADfn::for_sparsity
impl<V> ADfn<V>
where
    V : GlobalOpInfoVec ,
{
    /// Use the forward mode to compute a Jacobian sparsity pattern.
    ///
    /// See Also : [ADfn::sub_sparsity]
    ///
    /// * Syntax :
    /// ```text
    ///     pattern = f.for_sparsity(trace)
    /// ```
    ///
    /// * V : see [doc_generic_v]
    ///
    /// * f :
    /// is this [ADfn] object. The sparsity pattern is for the Jacobian
    /// of the function defined by the operation sequence stored in f.
    ///
    /// * trace :
    /// If trace is true, a trace of the sparsoty calculation
    /// is printed on standard output.
    ///
    /// * pattern :
    /// The the return value *pattern* is vector of [row, column] pairs.
    /// Each row (column) is less than the range (domain)
    /// dimension for the function.
    /// If a pair [i, j] does not appear, the range component
    /// with index i does not depend on the domain component with index j.
    ///
    /// * dependency :
    /// This is a dependency pattern. For example,
    /// if an range variable was equal to the
    /// Heaviside function of a domain variable,
    /// the corresponding pair would be in the sparisty pattern even though
    /// the corresponding derivative is always zero.
    ///
    /// # Example
    /// ```
    /// use rustad::AD;
    /// use rustad::ad_from_value;
    /// use rustad::start_recording;
    /// use rustad::stop_recording;
    /// //
    /// // V
    /// type V = f32;
    /// //
    /// // nx
    /// let nx = 4;
    /// //
    /// let x      : Vec<V>       = vec![2.0; nx];
    /// let ax                    = start_recording(x);
    /// let mut ay : Vec< AD<V> > = Vec::new();
    /// ay.push( ad_from_value( V::from(5.0) ) ); // ay[0] is a constant
    /// for j in 1 .. nx {
    ///     ay.push( &ax[j] * &ax[j] );      // ay[j] is a variable
    /// }
    /// let f           = stop_recording(ay);
    /// let trace       = false;
    /// let mut pattern = f.for_sparsity(trace);
    /// pattern.sort();
    /// assert_eq!( pattern.len(), nx - 1 );
    /// for j in 1 .. nx {
    ///     assert_eq!( pattern[j-1], [j,j] );
    /// }
    /// ```
    ///
    pub fn for_sparsity(&self, trace : bool) -> Vec< [usize; 2] >
    {   //
        // op_info_vec
        let op_info_vec : &Vec< OpInfo<V> >  = &*GlobalOpInfoVec::get();
        //
        // n_domain, n_var, flag_all, arg_all, op2arg,
        // range_ad_type, range_index, n_range
        let n_domain          = self.var.n_dom;
        let id_all            = &self.var.id_seq;
        let flag_all          = &self.var.flag;
        let arg_all           = &self.var.arg_all;
        let op2arg            = &self.var.arg_seq;
        let range_ad_type     = &self.range_ad_type;
        let range_index       = &self.range_index;
        let n_range           = range_ad_type.len();
        //
        // result, arg_var_index, arg_var_usize, set_vec
        let mut result          : Vec< [usize; 2] > = Vec::new();
        let mut arg_var_index   : Vec<IndexT>       = Vec::new();
        let mut arg_var_usize   : Vec<usize>        = Vec::new();
        let mut set_vec         : VecSet            = VecSet::new();
        //
        // set_vec.get(id_set) for id_set = 0 .. n_domain
        for id_set in 0 .. n_domain {
            set_vec.singleton( id_set );
        }
        //
        if trace {
            let mut range_var_index : Vec<IndexT> = Vec::new();
            for i in 0 .. range_index.len() {
                if range_ad_type[i] == ADType::Variable {
                        range_var_index.push(  range_index[i] );
                }
            }
            println!( "Begin Trace: for_sparisty: n_domain = {}", n_domain);
            println!("range_var_index = {:?}", range_var_index);
            println!("var_index, op_name, var_arguments, set_result");
        }
        //
        // op_index
        for op_index in 0 .. id_all.len() {
            //
            // op_info, arg_var_index_fn
            let op_id            = id_all[op_index] as usize;
            let op_info          = &op_info_vec[op_id];
            let arg_var_index_fn = op_info.arg_var_index;
            //
            // arg
            let begin = op2arg[op_index] as usize;
            let end   = op2arg[op_index + 1] as usize;
            let arg   = &arg_all[begin .. end];
            //
            // arg_var_index
            // the variables that are arguments to this operator
            arg_var_index_fn(&mut arg_var_index, &flag_all, arg);
            //
            // arg_var_usize
            let n_arg = arg_var_index.len();
            arg_var_usize.resize(n_arg, 0);
            for i in 0 .. n_arg {
                arg_var_usize[i] = arg_var_index[i] as usize;
            }
            //
            // var_index
            // variable index that we are computing the dependency for.
            let var_index = n_domain + op_index;
            //
            // set_vec.get(var_index)
            let set_id = set_vec.union( &arg_var_usize );
            debug_assert!( var_index ==  set_id );
            //
            if trace {
                let op_name = &op_info_vec[op_id].name;
                let set     = set_vec.get(var_index);
                println!(
                    "{}, {}, {:?}, {:?}",
                    var_index, op_name, arg_var_index,  set
                );
            }
        }
        for i in 0 .. n_range { if range_ad_type[i] == ADType::Variable {
            let row_var_index = range_index[i] as usize;
            let set           = set_vec.get(row_var_index);
            for j in 0 .. set.len() {
                let row =  i as usize;
                let col = set[j] as usize;
                result.push( [row, col] );
            }
        } }
        if trace {
            println!( "n_pattern = {}", result.len() );
        }
        result
    }
}
