// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement [GADFun] methods that compute function values and derivatives.
//!
//! Link to [parent module](super)
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::function::GADFun;
use crate::gad::GAD;
use crate::gas::as_from;
use crate::gas::sealed::GenericAs;
use crate::operator::GlobalOpInfoVec;
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::doc_generic_f_and_u;
//
#[cfg(doc)]
use crate::operator;
// -----------------------------------------------------------------------
// forward_zero
/// Zero order forward mode evaluation; i.e., function values.
///
/// * Syntax :
/// ```text
///     (range_zero, var_zero) = f.forward_zero(&domain_zero, trace)
///     (range_zero, var_zero) = f.ad_forward_zero(&domain_zero, trace)
/// ```
/// See [GADFun::forward_zero] and
/// [GADFun::ad_forward_zero] prototypes.
///
/// * F, U : see [doc_generic_f_and_u]
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
/// # Example
/// ```
/// type F      = f32;
/// type AD     = rustad::GAD<F, u32>;
/// //
/// // f
/// let x        : Vec<F> = vec![ 2.0, 2.0, 2.0 ];
/// let ax                = rustad::ad_domain(&x);
/// let mut asum : AD     = AD::from(0.0);
/// for j in 0 .. ax.len() {
///     asum += ax[j];
/// }
/// let ay = vec![ asum ];
/// let f  = rustad::ad_fun(&ay);
/// //
/// // y
/// let trace           = false;
/// let x      : Vec<F> = vec![ 1.0, 2.0, 3.0 ];
/// let (y, v)          = f.forward_zero(&x, trace);
/// //
/// assert_eq!( y[0] , (1 + 2 + 3) as F );
/// ```
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
                let op_id : usize = as_from( self.id_all[op_index] );
                let start : usize = as_from( self.op2arg[op_index] );
                let end   : usize = as_from( self.op2arg[op_index + 1] );
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
                        as_from( self.range2tape_index[i] );
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
                let index : usize = as_from( self.range2tape_index[i] );
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
///     range_one = f.forward_one(&domain_one, &var_zero, trace)
///     range_one = f.ad_forward_one(&domain_one, &var_zero, trace)
/// ```
/// See the [GADFun::forward_one] and
/// [GADFun::ad_forward_one) prototypes.
///
/// * F, U : see [doc_generic_f_and_u]
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
/// # Float Example
/// ```
/// type F      = f64;
/// type AD     = rustad::GAD<F, u32>;
/// //
/// // nx
/// let nx = 3;
///
/// // f
/// let x        : Vec<F> = vec![ 2.0; nx ];
/// let ax                = rustad::ad_domain(&x);
/// let mut asum : AD     = AD::from(0.0);
/// for j in 0 .. nx {
///     asum += ax[j] * ax[j];
/// }
/// let ay = vec![ asum ];
/// let f  = rustad::ad_fun(&ay);
/// //
/// // trace, x0, v0
/// let trace             = false;
/// let x0       : Vec<F> = vec![ 1.0, 2.0, 3.0 ];
/// let (y0, v0)          = f.forward_zero(&x0, trace);
/// //
/// // dy[0] = df/dx[j]
/// for j in 0 .. nx {
///     let mut dx : Vec<F> = vec![ 0.0; nx ];
///     dx[j]   = 1.0 as F;
///     let dy  = f.forward_one(&dx, &v0, trace);
///     assert_eq!( dy[0] ,  2.0 * x0[j] );
/// }
/// ```
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
                let op_id : usize = as_from( self.id_all[op_index] );
                let start : usize = as_from( self.op2arg[op_index] );
                let end   : usize = as_from( self.op2arg[op_index + 1] );
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
                        as_from( self.range2tape_index[i] );
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
                let index : usize = as_from( self.range2tape_index[i] );
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
/// * F, U : see [doc_generic_f_and_u]
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
                let index : usize = as_from( self.range2tape_index[j] );
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
                let op_id : usize = as_from( self.id_all[op_index] );
                let start : usize = as_from( self.op2arg[op_index] );
                let end   : usize = as_from( self.op2arg[op_index + 1] );
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
                        as_from( self.range2tape_index[i] );
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
