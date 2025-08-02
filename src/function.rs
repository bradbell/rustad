// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD function objects
//! : [parent module](super)
//
use std::cell::RefCell;
use std::thread::LocalKey;
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::ad::GAD;
use crate::operator::GlobalOpInfoVec;
use crate::ptrait::GenericAs;
use crate::record::sealed::ThisThreadTape;
use crate::record::{NEXT_TAPE_ID, GTape};
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator;
// -----------------------------------------------------------------------
// forward_zero
/// Zero order forward mode evaluation; i.e., function values.
///
/// * Syntax :
/// ```text
///     (range_zero, var_zero) = f.forward_zero(domain_zero, trace)
///     (range_zero, var_zero) = f.ad_forward_zero(domain_zero, trace)
/// ```
/// See [GADFun::forward_zero] and
/// [GADFun::ad_forward_zero] prototypes.
///
/// * F :
/// is the floating point type used for value calculations.
///
/// * U :
/// is the floating point type used for index inm the operation sequence.
///
/// * f :
/// is this [GADFun] object.
///
/// * domain_zero :
/// specifies the domain space variable values.
///
/// * trace :
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// * range_zero :
/// The first return value is the range vector corresponding to domain_zero;
/// i.e., the function value correspdong the operation sequence.
///
/// * var_zero :
/// The second return value is the value for all the variables
/// in the operation sequence.
/// This is used as an input when computing derivatives.
///
pub fn doc_forward_zero() { }
//
/// Create the zero order forward mode member functions.
///
/// * prefix :
/// is the name of the function without the _zero on the end; i.e.,
/// forward or ad_forward.
///
/// * EvalType :
/// is the type used to evaluate zero order forward mode.
/// It is also the type of the elements of the vectors in
/// *domain_zero* , *range_zero* and *var_zero* .
/// If *prefix* is forward (ad_forward), this must be F ( GAD<F,U> ) .
///
/// See [ doc_forward_zero ]
macro_rules! forward_zero {
    ( $prefix:ident, $EvalType:ty ) => { paste::paste! {

        #[doc = concat!(
            " ADFun zero order forward using ",
            stringify!($EvalType),
            " computations; see [ doc_forward_zero ]",
        )]
        pub fn [< $prefix _zero >] (
            &self,
            domain_zero : &[$EvalType],
            trace       : bool
        ) -> ( Vec<$EvalType> , Vec<$EvalType> )
        {
            assert_eq!(
                domain_zero.len(), self.n_domain,
                "f.forward_zero: domain_zero length does not match f"
            );
            //
            let op_info_vec = &*< F as GlobalOpInfoVec<U> >::get();
            let nan_f : F          = f32::NAN.into();
            let nan_e : $EvalType  = nan_f.into();
            let mut var_zero = vec![ nan_e; self.n_var ];
            for j in 0 .. self.n_domain {
                var_zero[j] = domain_zero[j];
            }
            if trace {
                println!( "Begin Trace: forward_zero: n_var = {}", self.n_var);
                println!( "index, flag" );
                for j in 0 .. self.flag_all.len() {
                    println!( "{}, {}", j, self.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "var_index, domain_zero" );
                for j in 0 .. domain_zero.len() {
                    println!( "{}, {}", j, var_zero[j] );
                }
                println!( "var_index, var, op, arg" );
            }
            for op_index in 0 .. self.id_all.len() {
                let op_id : usize = GenericAs::gas( self.id_all[op_index] );
                let start : usize = GenericAs::gas( self.op2arg[op_index] );
                let end   : usize = GenericAs::gas( self.op2arg[op_index + 1] );
                let arg       = &self.arg_all[start .. end];
                let res       = self.n_domain + op_index;
                let forward_0 = op_info_vec[op_id].[< $prefix _0 >];
                forward_0(&mut var_zero,
                    &self.con_all, &self.flag_all, &arg, res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                            "{}, {}, {}, {:?}", res, var_zero[res], name, arg
                    );
                }
            }
            if trace {
                println!( "range_index, var_index, con_index" );
                for i in 0 .. self.range_is_var.len() {
                    let index : usize =
                        GenericAs::gas( self.range2tape_index[i] );
                    if self.range_is_var[i] {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_zero" );
            }
            let mut range_zero : Vec<$EvalType> = Vec::new();
            for i in 0 .. self.range_is_var.len() {
                let index : usize = GenericAs::gas( self.range2tape_index[i] );
                if self.range_is_var[i] {
                    range_zero.push( var_zero[index] );
                } else {
                    let constant = self.con_all[index];
                    range_zero.push( constant.into() );
                }
            }
            ( range_zero, var_zero )
        }
    } }
}
// -----------------------------------------------------------------------
// forward_one
//
/// First order forward mode evaluation; i.e., directional derivatives.
///
/// * Syntax :
/// ```text
///     range_one = f.forward_one(domain_one, var_zero, trace)
///     range_one = f.ad_forward_one(domain_one, var_zero, trace)
/// ```
/// See the [GADFun::forward_one] and
/// [GADFun::ad_forward_one) prototypes.
///
/// * F :
/// is the floating point type used for value calculations.
///
/// * U :
/// is the floating point type used for index inm the operation sequence.
///
/// * f :
/// is this [GADFun] object.
///
/// * domain_one :
/// specifies the domain space direction along which the directional
/// derivative is evaluated.
///
/// * var_zero :
/// is the value for all the variables in the operation sequence.
/// This was returned at the end of a zero order forward mode computation.
///
/// * trace :
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// * range_one :
/// The return value is the range vector corresponding to
/// domain_one and var_zero;
/// i.e., the directional derivative for the fuctioon
/// corresponding to the operation sequence.
///
pub fn doc_forward_one() { }
//
/// Create the first order forward mode member functions.
///
/// * prefix :
/// is the name of the function without the _one on the end; i.e.,
/// forward or ad_forward.
///
/// * EvalType :
/// is the type used to evaluate first order forward mode.
/// It is also the type of the elements of the vectors
/// *var_zero* , *domain_one* , and *range_one* .
/// If *prefix* is forward (ad_forward), this must be F ( GAD<F,U> ) .
///
/// See [ doc_forward_one ]
macro_rules! forward_one {
    ( $prefix:ident, $EvalType:ty ) => { paste::paste! {

        #[doc = concat!(
            " ADFun firsat order forward using ",
            stringify!($EvalType),
            " computations; see [ doc_forward_one ]",
        )]
        pub fn [< $prefix _one >] (
            &self,
            domain_one : &[$EvalType],
            var_zero   : &Vec<$EvalType>,
            trace      : bool
        ) -> Vec<$EvalType>
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
            let op_info_vec = &*< F as GlobalOpInfoVec<U> >::get();
            let nan_f : F          = f32::NAN.into();
            let nan_e : $EvalType  = nan_f.into();
            let mut var_one = vec![ nan_e; self.n_var ];
            for j in 0 .. self.n_domain {
                var_one[j] = domain_one[j];
            }
            if trace {
                println!( "Begin Trace: forward_zero: n_var = {}", self.n_var);
                println!( "index, flag" );
                for j in 0 .. self.flag_all.len() {
                    println!( "{}, {}", j, self.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "var_index, domain_zero, domain_one" );
                for j in 0 .. domain_one.len() {
                    println!( "{}, [{}, {}]", j, var_zero[j], var_one[j] );
                }
                println!( "var_index, var, op, arg" );
            }
            for op_index in 0 .. self.id_all.len() {
                let op_id : usize = GenericAs::gas( self.id_all[op_index] );
                let start : usize = GenericAs::gas( self.op2arg[op_index] );
                let end   : usize = GenericAs::gas( self.op2arg[op_index + 1] );
                let arg           = &self.arg_all[start .. end];
                let res           = self.n_domain + op_index;
                let forward_1 = op_info_vec[op_id].[< $prefix _1 >];
                forward_1(&mut var_one, var_zero, &self.con_all, &arg, res );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                        "{}, [{}, {}], {}, {:?}",
                        res, var_zero[res], var_one[res], name, arg
                    );
                }
            }
            if trace {
                println!( "range_index, var_index, con_index" );
                for i in 0 .. self.range_is_var.len() {
                    let index : usize =
                        GenericAs::gas( self.range2tape_index[i] );
                    if self.range_is_var[i] {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_one" );
            }
            let mut range_one : Vec<$EvalType> = Vec::new();
            for i in 0 .. self.range_is_var.len() {
                let index : usize = GenericAs::gas( self.range2tape_index[i] );
                range_one.push( var_one[index] );
            }
            range_one
        }
    } }
}
// -------------------------------------------------------------------
// reverse_one
//
/// First order reverse mode evaluation;
/// i.e., gradient of sum of weighted range vector.
///
/// * Syntax :
/// ```text
///     domain_one = f.reverse_one(range_one, var_zero, trace)
///     domain_one = f.reverse_one(range_one, var_zero, trace)
/// ```
/// See the [GADFun::reverse_one] and
/// [GADFun::ad_reverse_one] prototypes.
///
/// * f :
/// is this [GADFun object].
///
/// * ramge_one :
/// specifies the weights in the weighted sum.
///
/// * var_zero :
/// is the value for all the variables in the operation sequence.
/// This is returned at the end of a [doc_forward_zero]
/// computation.
///
/// * trace :
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// * domain_one :
/// The return value is the gradiemt of the weighted sum
/// with respect to the domain variables.
///
pub fn doc_reverse_one() { }
//
/// Create the first order reverse mode member functions.
///
/// * prefix :
/// is the name of the function without the _one on the end; i.e.,
/// reverse or ad_reverse.
///
/// * EvalType :
/// is the type used to evaluate first order reverse mode.
/// It is also the type of the elements of the vectors
/// *var_zero* , *range_one* and *domain_one* .
/// If *prefix* is reverse (ad_reverse), this must be F ( GAD<F,U> ) .
///
/// See [ doc_reverse_one ]
macro_rules! reverse_one {
    ( $prefix:ident, $EvalType:ty ) => { paste::paste! {

        #[doc = concat!(
            " First order reverse using ",
            stringify!($EvalType),
            " computations; see [ doc_reverse_one ]",
        )]
        pub fn [< $prefix _one >] (
            &self,
            range_one  : &[$EvalType],
            var_zero   : &Vec<$EvalType>,
            trace      : bool
        ) -> Vec<$EvalType>
        {
            assert_eq!(
                range_one.len(), self.range_is_var.len(),
                "f.reverse_one: range_one length does not match f"
            );
            assert_eq!(
                var_zero.len(), self.n_var,
                "f.reverse_one: var_zero length does not match f"
             );
            //
            let op_info_vec = &*< F as GlobalOpInfoVec<U> >::get();
            let zero_f : F         = 0f32.into();
            let zero_e : $EvalType = zero_f.into();
            let mut partial = vec![zero_e; self.n_var ];
            for j in 0 .. self.range_is_var.len() {
                let index : usize = GenericAs::gas( self.range2tape_index[j] );
                partial[index] += range_one[j];
            }
            if trace {
                println!( "Begin Trace: forward_zero: n_var = {}", self.n_var);
                println!( "index, flag" );
                for j in 0 .. self.flag_all.len() {
                    println!( "{}, {}", j, self.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "var_index, range_zero, range_one" );
                for j in 0 .. range_one.len() {
                    println!( "{}, [{}, {}]", j, var_zero[j], partial[j] );
                }
                println!( "var_index, var, op, arg" );
            }
            for op_index in ( 0 .. self.id_all.len() ).rev() {
                let op_id : usize = GenericAs::gas( self.id_all[op_index] );
                let start : usize = GenericAs::gas( self.op2arg[op_index] );
                let end   : usize = GenericAs::gas( self.op2arg[op_index + 1] );
                let arg           = &self.arg_all[start .. end];
                let res           = self.n_domain + op_index;
                let reverse_1 = op_info_vec[op_id].[< $prefix _1 >];
                reverse_1(&mut partial, var_zero, &self.con_all, &arg, res );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                        "{}, [{}, {}], {}, {:?}",
                        res, var_zero[res], partial[res], name, arg
                    );
                }
            }
            if trace {
                println!( "range_index, var_index, con_index" );
                for i in 0 .. self.range_is_var.len() {
                    let index : usize =
                        GenericAs::gas( self.range2tape_index[i] );
                    if self.range_is_var[i] {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: reverse_one" );
            }
            let mut domain_one : Vec<$EvalType> = Vec::new();
            for j in 0 .. self.n_domain {
                domain_one.push( partial[j] );
            }
            domain_one
        }
    } }
}
// -----------------------------------------------------------------------
// GADFun
//
/// This can evaluate an operation sequence function and its derivatives.
///
/// * Operation sequence :
/// An operation sequence is a single assignment representation of
/// the function; i.e., each variable is only assigned once.
///
/// * Constructor :
/// An [ad_domain] call is used to start recording an operation sequence.
/// An [ad_fun] call is used to stop recording move the operation sequence
/// to an new ADFun object.
///
/// * F :
/// is the floating point type used for value calculations when this
/// operation sequence was recorded.
///
/// * U :
/// is the unsigned integer type used for indices in the recording.
///
pub struct GADFun<F,U> {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain            : usize,
    //
    // n_var
    /// The total number of variables in the operation sequence.
    pub(crate) n_var               : usize,
    //
    // id_all
    /// This maps an operator's index in the operation sequence
    /// to its [operator::id]
    pub(crate) id_all              : Vec<u8>,
    //
    // range_is_var
    /// The length of this vector is the dimension of the range space.
    /// If range_is_var\[i\] is true (false), the i-th range space component
    /// is a variable (constant).
    pub(crate) range_is_var        : Vec<bool>,
    //
    // flag_all
    /// This contains boolean flags that are part of some operator definitions.
    pub(crate) flag_all            : Vec<bool>,
    //
    // range2tape_index
    /// The length of this vector is also the dimension of the range space.
    /// If range_is_var\[i\] is true (false), range2tape_indx\[i\] is the
    /// variable (constant) index for the i-th component of the range space.
    pub(crate) range2tape_index    : Vec<U>,
    //
    // op2arg
    /// This maps an operator's index in the operation sequence to its
    /// the index of its first argument in arg_all.
    pub(crate) op2arg              : Vec<U>,
    //
    // arg_all
    /// This contains the arguments for all the opereators in the
    /// operatioon sequence.
    pub(crate) arg_all             : Vec<U>,
    //
    // con_all
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub(crate) con_all             : Vec<F>,
}
//
// ---------------------------------------------------------------------------
impl<F,U> GADFun<F,U> {
    //
    // new
    /// This creates an empty operation sequence.
    ///
    /// To be more specific,
    /// its domain and range vectors have length zero.
    /// # Example
    /// ```
    /// use rustad::function::GADFun;
    /// let f : GADFun<f32,u32> = GADFun::new();
    /// assert_eq!( f.domain_len(), 0 );
    /// assert_eq!( f.range_len(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            n_domain         : 0,
            n_var            : 0,
            id_all           : Vec::new() ,
            range_is_var     : Vec::new() ,
            flag_all         : Vec::new() ,
            range2tape_index : Vec::new() ,
            op2arg           : Vec::new() ,
            arg_all          : Vec::new() ,
            con_all          : Vec::new() ,
        }
    }
    //
    // domain_len
    /// dimension of domain space
    pub fn domain_len(&self) -> usize { self.n_domain }
    //
    // range_len
    /// dimension of range space
    pub fn range_len(&self) -> usize { self.range_is_var.len() }
}
//
impl<F,U> GADFun<F,U>
where
    F : Copy + From<f32> + From<F> +  GlobalOpInfoVec<U> + std::fmt::Display ,
    U : Copy + 'static + std::fmt::Debug + GenericAs<usize>,
    GAD<F,U>: From<F>,
{
    //
    // forward_zero
    forward_zero!(forward, F);
    //
    // ad_forward_zero
    forward_zero!(ad_forward, GAD<F,U>);
    //
    // forward_one
    forward_one!(forward, F);
    //
    // ad_forward_one
    forward_one!(ad_forward, GAD<F,U>);
}
//
impl<F,U> GADFun<F,U>
where
    U       : Copy + 'static + GenericAs<usize> + std::fmt::Debug,
    GAD<F,U>: From<F> + std::ops::AddAssign ,
    F       : Copy + From<f32> + GlobalOpInfoVec<U> + std::ops::AddAssign +
              std::fmt::Display ,
{
    //
    // reverse_one
    reverse_one!(reverse, F);
    //
    // ad_reverse_one
    reverse_one!(ad_reverse, GAD<F,U>);
}
// ---------------------------------------------------------------------------
// ADFun::dependency
impl<F,U> GADFun<F,U>
where
    F     : GlobalOpInfoVec<U> ,
    U     : Copy + 'static + GenericAs<usize> + std::cmp::PartialEq,
    usize : GenericAs<U>,

{
    // dependency
    /// Compute dependency pattern for the operation sequence in this GADFun.
    ///
    /// * Syntax :
    /// ```text
    ///     pattern = f.dependency(trace)
    /// ```
    ///
    /// * F :
    /// is the floating point type used for value calculations.
    ///
    /// * U :
    /// is the floating point type used for index inm the operation sequence.
    ///
    /// * f :
    /// is this [GADFun] object.
    ///
    /// * trace :
    /// If trace is true, a trace of the dependency calculation
    /// is printed on standard output.
    /// Note that in the trace, the cases where *var_index* is less
    /// that the number of domain variables will end up in the pattern
    /// with the corresponding row.
    ///
    /// * pattern :
    /// The the return value *pattern* is vector of (row, column) pairs.
    /// Each row (column) is non-negative and
    /// less than the range (domain) dimension for the function.
    /// If a pair (i, j) does not appear, the range component
    /// with index i does not depend on the domain component with index j.
    /// Note that this can be used as a sparsity pattern for the Jacobian
    /// of the function.
    ///```
    /// use rustad::function;
    /// use rustad::ad::GAD;
    /// type AD = GAD<f32, u64>;
    /// let x  : Vec<f32> = vec![1.0, 2.0, 3.0];
    /// let ax : Vec<AD>  = function::ad_domain(&x);
    /// let mut ay : Vec<AD> = Vec::new();
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
    pub fn dependency(&self, trace : bool) -> Vec<(U, U)>
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
        let n_var_u : U = GenericAs::gas(n_var);
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
            let mut var_index : usize = GenericAs::gas( range2tape_index[row] );
            if trace {
                println!( "row {} var_index {}", row, var_index );
            }
            //
            // var_index_stack
            // use resize instead of new stack to reduce memory allocation
            let zero_u : U = GenericAs::gas(0);
            var_index_stack.resize(0, zero_u);
            var_index_stack.push( GenericAs::gas( var_index) );
            while var_index_stack.len() > 0 {
                //
                // var_index
                let var_index_u = var_index_stack.pop().unwrap();
                var_index = GenericAs::gas( var_index_u );
                //
                let row_u : U = GenericAs::gas(row);
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
                            GenericAs::gas( self.id_all[op_index] );
                        let op_info          = &op_info_vec[op_id];
                        let arg_var_index_fn = op_info.arg_var_index;
                        //
                        // arg
                        let begin : usize = GenericAs::gas( op2arg[op_index] );
                        let end : usize = GenericAs::gas(op2arg[op_index + 1]);
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
            println!( "n_dependency = {}", result.len() );
        }
        result
    }
}
// ----------------------------------------------------------------------------
// ad_domain
//
/// This starts recording a new operation sequence.
///
/// * Recording :
/// There must not currently be a recording in process on the current thread
/// when ad_domain is called. The recording is stopped when [ad_fun] is called.
///
/// * F : is the floating point type used for values calculations.
/// * U : is the unsigned integer type used for tape indices.
///
/// * domain :
/// This vector determines the number of domain (independent) variables
/// and their value during the recording.
///
/// * ad_domain :
/// The return is a vector of variables
/// with the same length and values as domain.
/// Dependencies with respect to these variables will be recorded on
/// the tape for this thread.
///
pub fn ad_domain<F,U>( domain : &[F] ) -> Vec< GAD<F,U> >
where
    F     : Copy + Sized + 'static + ThisThreadTape<U> ,
    U     : Sized + 'static ,
    usize : Sized + 'static + GenericAs<U> ,
{
    //
    // new_tape_id
    let new_tape_id : usize;
    {   let mut next_tape_id = NEXT_TAPE_ID.lock().unwrap();
        //
        // The rest of this block has a lock, so it is fast and can't fail.
        new_tape_id   = *next_tape_id;
        *next_tape_id = new_tape_id + 1;
    }
    let local_key : &LocalKey< RefCell< GTape<F,U> > > =
        < F as ThisThreadTape<U> >::get();
    local_key.with_borrow_mut( |tape| {
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
    let mut result : Vec< GAD<F,U> > = Vec::new();
    for j in 0 .. domain.len() {
        result.push(  GAD {
            tape_id   : GenericAs::gas( new_tape_id ),
            var_index : GenericAs::gas( j ),
            value     : domain[j],
        } );
    }
    result
}
// ----------------------------------------------------------------------------
// ad_fun
//
/// Stops a recordng and moves it to an ADFun object.
///
/// * Recording :
/// There must currently be a recording in process on the current thread
/// ( started by [ad_domain] ).
///
/// * F : is the floating point type used for values calculations.
/// * U : is the unsigned integer type used for tape indices.
///
/// * ad_range :
/// This is an AD vector of range space variables.
///
/// * ad_fun :
/// The return value is an ADFun containing the sequence of operations
/// that computed ad_range as a function of [ad_domain].
/// It can compute the range space variables and derivarives
/// as a function of the domain space variables.
///
/// * Assumptions :
/// The following assumptions are checked for the tape for this thread:
///     1. tape.arg_all.len() <= U::Max
///     2. tape.tape_id       <= U::Max
///
pub fn ad_fun<F,U>( ad_range : &[ GAD<F,U> ] ) -> GADFun<F,U>
where
    F     : Copy + Sized + 'static + ThisThreadTape<U> ,
    U     : Copy + Sized + 'static + TryFrom<usize> + GenericAs<usize> ,
    usize : GenericAs<U> ,
{
    let mut result : GADFun<F,U> = GADFun::new();
    let local_key : &LocalKey< RefCell< GTape<F,U> > > =
        < F as ThisThreadTape<U> >::get();
    let tape_id : usize = local_key.with_borrow_mut( |tape| {
        //
        // tape.recording
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        //
        // check assumptions
        assert_eq!( tape.n_var , tape.n_domain + tape.id_all.len() );
        assert_eq!( tape.op2arg.len() , tape.id_all.len() );
        match U::try_from( tape.arg_all.len() ) {
            Err(_) => panic!( "tape.arg_all.len() > U::MAX" ),
            Ok(_)  => (),
        }
        match U::try_from( tape.tape_id ) {
            Err(_) => panic!( "tape.tape_id > U::MAX" ),
            Ok(_)  => (),
        }
        //
        // tape.op2arg
        // end marker for arguments to the last operation
        tape.op2arg.push( GenericAs::gas( tape.arg_all.len() ) );
        //
        std::mem::swap( &mut result.n_domain,      &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.id_all,        &mut tape.id_all );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_all,       &mut tape.arg_all );
        std::mem::swap( &mut result.con_all,       &mut tape.con_all );
        std::mem::swap( &mut result.flag_all,      &mut tape.flag_all );
        tape.tape_id
    } );
    //
    // range_is_var, range2tape_index
    for i in 0 .. ad_range.len() {
        if GenericAs::gas( ad_range[i].tape_id ) == tape_id {
            result.range_is_var.push( true );
            result.range2tape_index.push( ad_range[i].var_index );
        } else {
            result.range_is_var.push( false );
            result.range2tape_index.push(
                GenericAs::gas( result.con_all.len()  )
            );
            result.con_all.push( ad_range[i].value );
        }
    }
    result
}
