// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! ADFun objects: [parent module](super)
//
use crate::{Index, Float, AD};
use crate::operator::OP_INFO_VEC;
use crate::ad_tape::{THIS_THREAD_TAPE, NEXT_TAPE_ID};
//
#[cfg(doc)]
use crate::operator;
//
// -----------------------------------------------------------------------
// forward_zero
/// Zero order forward mode evaluation: [source module](crate::function)
///
/// # Documentation for the functions created by forward_zero!
///
/// ## Syntax
/// <pre>
///     (range_zero, var_zero) = f.forward_zero(domain_zero, trace)
///     (range_zero, var_zero) = f.ad_forward_zero(domain_zero, trace)
/// </pre>
/// See [Float][ADFun::forward_zero] and
/// [AD](ADFun::ad_forward_zero) prototypes.
///
/// ## f
/// is is this ADFun object.
///
/// ## domain_zero
/// specifies the domain space variable values.
///
/// ## trace
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// ## range_zero
/// The first return value is the range vector corresponding to domain_zero;
/// i.e., the function value correspdong the operation sequence.
///
/// ## var_zero
/// The second return value is the value for all the variables
/// in the operation sequence. This is needed to compute derivatives.
///
/// # Documentation for forward_zero!
/// This macro is not intended to be used outside the rustad crate.
/// It only has the following use cases:
/// <pre>
///     forward_zero!(Float);
///     forward_zero!(AD);
/// </pre>
#[macro_export]
macro_rules! forward_zero {
    (Float) => { forward_zero!(forward, Float); };
    (AD)    => { forward_zero!(ad_forward, AD); };
    //
    ( $prefix:ident, $float_type:ident ) => { paste::paste! {

        #[doc = concat!(
            " Zero order forward using ",
            stringify!($float_type),
            " see [ forward_zero! ] for documentation",
        )]
        pub fn [< $prefix _zero >] (
            &self,
            domain_zero : &[$float_type],
            trace       : bool
        ) -> ( Vec<$float_type> , Vec<$float_type> )
        {
            assert_eq!(
                domain_zero.len(), self.n_domain,
                "f.forward_zero: domain_zero length does not match f"
            );
            //
            let op_info_vec = &*OP_INFO_VEC;
            let nan          = $float_type::from( Float::NAN );
            let mut var_zero = vec![ nan; self.n_var ];
            for j in 0 .. self.n_domain {
                var_zero[j] = domain_zero[j];
            }
            if trace {
                println!( "Begin Trace: forward_zero" );
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "index, domain_zero" );
                for j in 0 .. domain_zero.len() {
                    println!( "{}, {}", j, var_zero[j] );
                }
                println!( "res. name, arg,. var_zero" );
            }
            for op_index in 0 .. self.id_all.len() {
                let op_id     = self.id_all[op_index];
                let start     = self.op2arg[op_index];
                let end       = self.op2arg[op_index + 1];
                let arg       = &self.arg_all[start .. end];
                let res       = self.n_domain + op_index;
                let forward_0 = op_info_vec[op_id].[< $prefix _0 >];
                forward_0(&mut var_zero,
                    &self.con_all, &self.flag_all, &arg, res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                        "{}, {}, {:?}, {}", res, name, arg, var_zero[res]
                    );
                }
            }
            if trace {
                println!( "End Trace: forward_zero" );
            }
            let mut range_zero : Vec<$float_type> = Vec::new();
            for i in 0 .. self.range_index.len() {
                range_zero.push( var_zero[ self.range_index[i] ] );
            }
            ( range_zero, var_zero )
        }

    } }
}
// -----------------------------------------------------------------------
// forward_one
/// First order forward mode evaluation; [source module](crate::function)
///
/// # Documentation for the functions created by forward_zero!
///
/// ## Syntax
/// <pre>
///     range_one = f.forward_one(domain_one, var_zero, trace)
///     range_one = f.ad_forward_one(domain_one, var_zero, trace)
/// </pre>
/// See [Float][ADFun::forward_one] and
/// [AD](ADFun::ad_forward_one) prototypes.
///
/// ## f
/// is is this [ADFun] object.
///
/// # domain_one
/// specifies the directional derivative for domain space variables.
///
/// ## var_zero
/// is the value for all the variables in the operation sequence.
/// This is returned at the end of a [forward_zero](ADFun::forward_zero)
/// computation.
///
/// ## trace
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// ## range_one
/// The return value is the range vector corresponding to
/// domain_one and var_zero;
/// i.e., the directional derivative for the fuctioon
/// corresponding to the operation sequence.
///
/// # Documentation for forward_one!
/// This macro is not intended to be used outside the rustad crate.
/// It only has the following use cases:
/// <pre>
///     forward_one!(Float);
///     forward_one!(AD);
/// </pre>
#[macro_export]
macro_rules! forward_one {
    (Float) => { forward_one!(forward, Float); };
    (AD)    => { forward_one!(ad_forward, AD); };
    //
    ( $prefix:ident, $float_type:ident ) => { paste::paste! {

        #[doc = concat!(
            " First order forward using ",
            stringify!($float_type),
            " see [ forward_one! ] for documentation",
        )]
        pub fn [< $prefix _one >] (
            &self,
            domain_one : &[$float_type],
            var_zero   : &Vec<$float_type>,
            trace      : bool
        ) -> Vec<$float_type>
        {
            assert_eq!(
                domain_one.len(), self.n_domain,
                "f.forward_one: domain_one length does not match f"
            );
            assert_eq!(
                var_zero.len(), self.n_var,
                "f.forward_one: var_zero length does not match f"
             );
            //
            let op_info_vec = &*OP_INFO_VEC;
            let nan          = $float_type::from( Float::NAN );
            let mut var_one = vec![ nan; self.n_var ];
            for j in 0 .. self.n_domain {
                var_one[j] = domain_one[j];
            }
            if trace {
                println!( "Begin Trace: forward_one" );
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "index, domain_zero, domain_one" );
                for j in 0 .. domain_one.len() {
                    println!( "{}, [{}, {}]", j, var_zero[j], var_one[j] );
                }
                println!( "res, name, arg, var_zero[res]. var_one[res]" );
            }
            for op_index in 0 .. self.id_all.len() {
                let op_id     = self.id_all[op_index];
                let start     = self.op2arg[op_index];
                let end       = self.op2arg[op_index + 1];
                let arg       = &self.arg_all[start .. end];
                let res       = self.n_domain + op_index;
                let forward_1 = op_info_vec[op_id].[< $prefix _1 >];
                forward_1(&mut var_one, var_zero, &self.con_all, &arg, res );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                        "{}, {}, {:?}, [{}, {}]",
                        res, name, arg, var_zero[res], var_one[res]
                    );
                }
            }
            if trace {
                println!( "End Trace: forward_one" );
            }
            let mut range_one : Vec<$float_type> = Vec::new();
            for i in 0 .. self.range_index.len() {
                range_one.push( var_one[ self.range_index[i] ] );
            }
            range_one
        }
    } }
}
// -------------------------------------------------------------------
// reverse_one
/// First order reverse mode evaluation: [source module](crate::function)
///
/// # Documentation for the functions created by reverse_one!
///
/// ## Syntax
/// <pre>
///     domain_one = f.reverse_one(range_one, var_zero, trace)
///     domain_one = f.reverse_one(range_one, var_zero, trace)
/// </pre>
/// See [Float][ADFun::reverse_one] and
/// [AD](ADFun::ad_reverse_one) prototypes.
///
/// ## f
/// is is this ADFun object.
///
/// ## ramge_one
/// specifies the partials of as scalar function of range variables.
///
/// ## var_zero
/// is the value for all the variables in the operation sequence.
/// This is returned at the end of a [forward_zero](ADFun::forward_zero)
/// computation.
///
/// ## trace
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// ## domain_one
/// The return value is the partials of the scalar function
/// with respect to the domain variables.
/// in the operation sequence. This is needed to compute derivatives.
///
/// # Documentation for reverse_one!
/// This macro is not intended to be used outside the rustad crate.
/// It only has the following use cases:
/// <pre>
///     reverse_one!(Float);
///     reverse_one!(AD);
/// </pre>
#[macro_export]
macro_rules! reverse_one {
    (Float) => { reverse_one!(reverse, Float); };
    (AD)    => { reverse_one!(ad_reverse, AD); };
    //
    ( $prefix:ident, $float_type:ident ) => { paste::paste! {

        #[doc = concat!(
            " First order reverse using ",
            stringify!($float_type),
            " see [ reverse_one! ] for documentation",
        )]
        pub fn [< $prefix _one >] (
            &self,
            range_one  : &[$float_type],
            var_zero   : &Vec<$float_type>,
            trace      : bool
        ) -> Vec<$float_type>
        {
            assert_eq!(
                range_one.len(), self.range_index.len(),
                "f.reverse_one: range_one length does not match f"
            );
            assert_eq!(
                var_zero.len(), self.n_var,
                "f.reverse_one: var_zero length does not match f"
             );
            //
            let op_info_vec = &*OP_INFO_VEC;
            let zero        = $float_type::from( Float::from(0.0) );
            let mut partial = vec![zero; self.n_var ];
            for j in 0 .. self.range_index.len() {
                // 2DO: change this to += ones it is implemented for AD
                partial[ self.range_index[j] ] =
                    partial[ self.range_index[j] ] + range_one[j];
            }
            if trace {
                println!( "Begin Trace: reverse_one" );
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "index, range_zero, range_one" );
                for j in 0 .. range_one.len() {
                    println!( "{}, [{}, {}]", j, var_zero[j], partial[j] );
                }
                println!( "res, name, arg, var_zero[res]. partial[res]" );
            }
            for op_index in ( 0 .. self.id_all.len() ).rev() {
                let op_id     = self.id_all[op_index];
                let start     = self.op2arg[op_index];
                let end       = self.op2arg[op_index + 1];
                let arg       = &self.arg_all[start .. end];
                let res       = self.n_domain + op_index;
                let reverse_1 = op_info_vec[op_id].[< $prefix _1 >];
                reverse_1(&mut partial, var_zero, &self.con_all, &arg, res );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                        "{}, {}, {:?}, [{}, {}]",
                        res, name, arg, var_zero[res], partial[res]
                    );
                }
            }
            if trace {
                println!( "End Trace: reverse_one" );
            }
            let mut domain_one : Vec<$float_type> = Vec::new();
            for j in 0 .. self.n_domain {
                domain_one.push( partial[j] );
            }
            domain_one
        }
    } }
}
// -----------------------------------------------------------------------
//
// ADFun
/// This object can evaluate an operation sequence amd its derivatives.
///
/// # Operation sequence
/// An operation sequence is a single assignment representation of
/// the function; i.e., a variable is only assigned once.
///
/// # Constructor
/// An [ad_domain] call is used to start a recording an operation sequence.
/// An [ad_fun] call is used to stop recording move the operation sequence
/// to an new ADFun object.
pub struct ADFun {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain       : Index,
    //
    // n_var
    /// The total number of variables in the operation sequence.
    pub(crate) n_var          : Index,
    //
    // range_index
    /// The variable index for each of the range variables in this function.
    /// The dimension of its range spase is range_index.len().
    pub(crate) range_index    : Vec<Index>,
    //
    // id_all
    /// This maps an operator's index in the operation sequence
    /// to its [operator::id]
    pub(crate) id_all         : Vec<Index>,
    //
    // op2arg
    /// This maps an operator's index in the operation sequence to its
    /// the index of its first argument in arg_all.
    pub(crate) op2arg         : Vec<Index>,
    //
    // arg_all
    /// This contains the arguments for all the opereators in the
    /// operatioon sequence.
    pub(crate) arg_all        : Vec<Index>,
    //
    // con_all
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub(crate) con_all        : Vec<Float>,
    //
    // flag_all
    /// This contains boolean flags that are part of some operator definitions.
    pub(crate) flag_all       : Vec<bool>,
}
// ---------------------------------------------------------------------------
impl ADFun {
    //
    // new
    /// This creates an empty operation sequence; i.e,
    /// its domain and range vectors have length zero.
    /// # Example
    /// ```
    /// let f = rustad::function::ADFun::new();
    /// assert_eq!( f.domain_len(), 0 );
    /// assert_eq!( f.range_len(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            n_domain      : 0,
            n_var         : 0,
            id_all        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_all       : Vec::new() ,
            con_all       : Vec::new() ,
            flag_all      : Vec::new() ,
            range_index   : Vec::new() ,
        }
    }
    //
    // domain_len
    /// dimension of domain space
    pub fn domain_len(&self) -> Index { self.n_domain }
    //
    // range_len
    /// dimension of range space
    pub fn range_len(&self) -> Index { self.range_index.len() }
    //
    // forward_zero
    forward_zero!(Float);
    //
    // ad_forward_zero
    forward_zero!(AD);
    //
    // forward_one
    forward_one!(Float);
    //
    // ad_forward_one
    forward_one!(AD);
    //
    // reverse_one
    reverse_one!(Float);
    //
    // ad_reverse_one
    reverse_one!(AD);
    //
    // -----------------------------------------------------------------------
    // dependency
    /// Computes the dependency pattern for the function in this ADFun.
    ///
    /// <pre>
    ///     pattern = dependency(trace)
    /// </pre>
    ///
    /// # trace
    /// If trace is true, a trace of the dependency calculation
    /// is printed on standard output.
    /// Note that in the trace, the cases where *var_index* is less
    /// that the number of domain variables will end up in the pattern
    /// with the corresponding row.
    ///
    /// # pattermn
    /// The the return value *pattern* is vector of (row, column) pairs.
    /// Each row (column) is non-negative and
    /// less than the range (domain) dimension for the function.
    /// If a pair (i, j) does not appear, the range component
    /// with index i does not depend on the domain component with index j.
    /// Note that this can be used as a sparsity pattern for the Jacobian
    /// of the function.
    ///```
    /// use rustad::{Float, AD, function};
    /// let x       : Vec<Float> = vec![1.0, 2.0, 3.0];
    /// let ax      = function::ad_domain(&x);
    /// let mut ay  : Vec<AD> = Vec::new();
    /// for j in 0 .. x.len() {
    ///     ay.push( ax[j] * ax[j] );
    /// }
    /// let f           = function::ad_fun(&ay);
    /// let trace       = false;
    /// let mut pattern = f.dependency(trace);
    /// pattern.sort_by( |x, y| x.partial_cmp(y).unwrap() );
    /// assert_eq!( pattern.len(), 3 );
    /// assert_eq!( pattern[0], (0,0) );
    /// assert_eq!( pattern[1], (1,1) );
    /// assert_eq!( pattern[2], (2,2) );
    ///```
    pub fn dependency(&self, trace : bool) -> Vec<(Index, Index)>
    {   //
        // op_info_vec
        let op_info_vec = &*OP_INFO_VEC;
        //
        // n_domain, n_var, flag_all, arg_all, op2arg, n_range
        let n_domain     = self.n_domain;
        let n_var        = self.n_var;
        let flag_all     = &self.flag_all;
        let arg_all      = &self.arg_all;
        let op2arg       = &self.op2arg;
        let range_index  = &self.range_index;
        let n_range      = range_index.len();
        //
        // done
        let mut done : Vec<Index> = vec![n_var; n_var];
        //
        // result, arg_var_index, var_index_stack
        let mut result          : Vec<(Index, Index)> = Vec::new();
        let mut arg_var_index   : Vec<Index> = Vec::new();
        let mut var_index_stack : Vec<Index> = Vec::new();
        //
        if trace {
            println!( "n_domain = {}, n_range = {}", n_domain, n_range );
        }
        //
        // row
        // determine the variables that range index row depends on
        for row in 0 .. n_range {
            //
            // var_index
            let mut var_index = self.range_index[row];
            if trace {
                println!( "row {} var_index {}", row, var_index );
            }
            //
            // var_index_stack
            // use resize instead of new stack to reduce memory allocation
            var_index_stack.resize(0, 0);
            var_index_stack.push( var_index );
            while var_index_stack.len() > 0 {
                //
                // var_index
                var_index = var_index_stack.pop().unwrap();
                //
                if done[var_index] != row {
                    done[var_index] = row;
                    if trace {
                        println!( "    var_index = {}", var_index );
                    }
                    if var_index < n_domain {
                        //
                        // result
                        // var_index is a domain variable index
                        result.push( (row, var_index) );
                    } else {
                        //
                        // op_index
                        // the operator that creates this variable
                        let op_index         = var_index - n_domain;
                        //
                        // arv_var_index_fn
                        let op_id            = self.id_all[op_index];
                        let op_info          = &op_info_vec[op_id];
                        let arg_var_index_fn = op_info.arg_var_index;
                        //
                        // arg
                        let begin = op2arg[op_index];
                        let end   = op2arg[op_index + 1];
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
        }
        if trace {
            println!( "n_dependency = {}", result.len() );
        }
        result
    }
    // -----------------------------------------------------------------------
}
//
// ad_domain
/// Calling `ad_domain` starts a new recording ([ad_fun] stops the recording).
///
/// # Recording
/// There must not currently be a recording in process on the current thread.
///
/// # domain
/// This vector determines the number of domain (independent) variables
/// and their value during the recording.
///
/// # ad_domain
/// The return is a vector of variables
/// with the same length and values as domain.
/// Dependencies with respect to these variables will be recorded on
/// the tape for this thread.
pub fn ad_domain( domain : &[Float] ) -> Vec<AD> {
    //
    // new_tape_id
    let new_tape_id : Index;
    {   let mut next_tape_id = NEXT_TAPE_ID.lock().unwrap();
        //
        // The rest of this block has a lock, so it is fast and can't fail.
        new_tape_id   = *next_tape_id;
        *next_tape_id = new_tape_id + 1;
    }
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        assert_ne!( new_tape_id, 0);
        assert!( ! tape.recording , "indepndent: tape is already recording");
        assert_eq!( tape.id_all.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_all.len(), 0 );
        assert_eq!( tape.con_all.len(), 0 );
        assert_eq!( tape.flag_all.len(), 0 );
        tape.tape_id        = new_tape_id;
        tape.recording      = true;
        tape.n_domain       = domain.len();
        tape.n_var          = domain.len();
        //
    } );
    let mut result : Vec<AD> = Vec::new();
    for j in 0 .. domain.len() {
        result.push(
             AD { tape_id : new_tape_id, var_index : j, value : domain[j] }
        );
    }
    result
}
//
// ad_fun
/// Calling `ad_fun` stops a recordng and moves it to an ADFun object
/// ([ad_domain] starts a recording).
///
/// # Recording
/// There must currently be a recording in process on the current thread.
///
/// # ad_range
/// This is an AD vector of range space variables.
///
/// # ad_fun
/// The return value is an ADFun containing the sequence of operations
/// that computed ad_range as a function of [ad_domain].
/// It can compute the range space variables and derivarives
/// as a function of the domain space variables.
pub fn ad_fun( ad_range : &[AD] ) -> ADFun {
    let mut result = ADFun::new();
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        //
        // tape.recording
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        //
        // tape.op2arg
        // end marker for arguments to the last operation
        tape.op2arg.push( tape.arg_all.len() );
        //
        std::mem::swap( &mut result.n_domain, &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.id_all,        &mut tape.id_all );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_all,       &mut tape.arg_all );
        std::mem::swap( &mut result.con_all,       &mut tape.con_all );
        std::mem::swap( &mut result.flag_all,      &mut tape.flag_all );
    } );
    //
    // range_index
    // 2DO handle case where ad_range[i] is a constant (need to test CALL_OP).
    for i in 0 .. ad_range.len() {
        result.range_index.push( ad_range[i].var_index );
    }
    result
}
