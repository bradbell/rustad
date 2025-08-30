// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module defines the automatic differentiation class `AD<V>`.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::numvec::tape::Tindex;
use crate::numvec::tape::Tape;
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::op::id;
// ---------------------------------------------------------------------------
/// Documentation for the rustad generic type parameter V.
///
/// The generic parameter *V* , in ``AD`` < *V* > and other generic types ,
/// is the type used for calculating values.
/// It does not have dependency information that represents
/// how each value is related to the domain variables (independent variables).
pub fn doc_generic_v() {}
//
// AD
//
/// AD acts like V but in addition can record a function evaluation.
///
/// * V : see [doc_generic_v]
///
/// * variable :
/// An AD object is a variable if it one of the domain variables
/// or its value depends on the value of the domain variables.
///
/// * constant :
/// If an AD object is not a variable it is referred to as a constant.
///
#[derive(Clone, Debug)]
pub struct AD<V> {
    //
    // tape_id
    ///
    /// An AD object is a variable if the following two conditions hold:
    /// 1. This threads tape is currently recording.
    /// 2. This threads tape and the AD object have the same *tape_id* .
    /// TODO: Change to pub(crate) when this gets used.
    pub tape_id   : usize,
    //
    // var_index
    /// If this AD object is a variable, *var_index* is its index in the tape.
    /// TODO: Change to pub(crate) when this gets used.
    pub var_index : usize,
    //
    // value
    /// is the value of this AD variable or constant.
    pub(crate) value     : V,
}
//
// new
impl<V> AD<V> {
    //
    /// Create an arbitrary new AD object.
    ///
    /// * new_tape_id : is the [AD::tape_id] for the new object.
    ///
    /// * new_var_index : is the [AD::var_index] for the new object.
    ///
    /// * new_value : is the [AD::value}} for the new object.
    pub(crate) fn new(
        new_tape_id: usize, new_var_index: usize, new_value: V )-> Self {
        Self {
            tape_id   : new_tape_id,
            var_index : new_var_index,
            value     : new_value,
        }
    }
}
//
// to_value
impl<V> AD<V> {
    //
    /// Convert an AD object to a value
    /// (its the variable information is lost).
    ///
    /// **See Also** : example in [ad_from_value]
    ///
    /// # Example using NumVec
    /// ```
    /// use rustad::numvec::AD;
    /// use rustad::numvec::NumVec;
    /// use rustad::numvec::ad_from_value;
    /// let v   : Vec<f64>    = vec![ 2.0, 3.0 ];
    /// let nv                = NumVec::new(v);
    /// let av                = ad_from_value(nv);
    /// let nv                = av.to_value();
    /// assert_eq!( nv.vec[0], 2.0 );
    /// assert_eq!( nv.vec[1], 3.0 );
    /// ```
    pub fn to_value(self) -> V {
        self.value
    }
}
// ---------------------------------------------------------------------------
// Display
//
/// Display only shows the value and ignores the variable information.
///
/// # Example
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
/// let x  : f64  = 5.0;
/// let ax        = ad_from_value(x);
/// let s         = format!( "{ax}" );
/// assert_eq!(s, "5");
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
/// use rustad::numvec::NumVec;
/// let x  : Vec<f64>  = vec![ 5.0, 6.0 ];
/// let x_nv           = NumVec::new(x);
/// let ax             = ad_from_value(x_nv);
/// let s              = format!( "{ax}" );
/// assert_eq!(s, "[ 5, 6, ]");
/// ```
impl<V : std::fmt::Display> std::fmt::Display for AD<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// ---------------------------------------------------------------------------
// ad_from_value
/// Convert a value to an AD object with no variable information;
/// i.e., constant.
///
/// **See Also** : example in [AD::to_value]
///
/// # Example
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
/// let x  : f32  = 3.0;
/// let ax        = ad_from_value(x);
/// assert_eq!( ax.to_value(), 3.0 );
/// ```
pub fn ad_from_value<V> ( value : V ) ->AD<V> {
    let tape_id   = 0;
    let var_index = 0;
    AD::new(tape_id, var_index, value)
}
// ---------------------------------------------------------------------------
/// Binary `AD<V>` operators.
///
/// V : see [doc_generic_v]
///
/// Op : is the source code token for this binary operator;
/// i.e., `+` , `-` , `*` , or `/` .
///
/// Prototype:
/// <br/>
/// & `AD<V>` *Op* & `AD<V>`
///
/// # Example
///```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
///
/// let a  = ad_from_value( 3.0f32 );
/// let b  = ad_from_value( 4.0f32 );
/// let c = &a * &b;
/// assert_eq!( c.to_value(), 12.0f32 );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
/// use rustad::numvec::NumVec;
///
/// let x     : Vec<f64> = vec![ 1.0, 4.0 ];
/// let y     : Vec<f64> = vec![ 2.0, 2.0 ];
/// let x_nv             = NumVec::new(x);
/// let y_nv             = NumVec::new(y);
/// let ax               = ad_from_value(x_nv);
/// let ay               = ad_from_value(y_nv);
/// let az               = &ax / &ay;
/// assert_eq!( az.to_value().vec, vec![0.5f64, 2.0f64] );
/// ```
pub fn doc_ad_binary_op() { }
//
/// Add one binary operator to the `AD<V>` class;
///
/// * Name : is the operator name; i.e., Add, Sub, Mul, or Div.
///
/// * Op : is the operator token; i.e., +, -, *, or /.
///
/// see [doc_ad_binary_op]
macro_rules! ad_binary_op { ($Name:ident, $Op:tt) => { paste::paste! {
    // -----------------------------------------------------------------------
    fn [< record_ $Name:lower >]<V> (
        tape: &mut Tape<V> ,
        lhs:       &AD<V>  ,
        rhs:       &AD<V>  ,
    ) -> (usize, usize)
    where
        V : Clone ,
    {
        let mut new_tape_id   = 0;
        let mut new_var_index = 0;
        if tape.recording {
            let var_lhs    = lhs.tape_id == tape.tape_id;
            let var_rhs    = rhs.tape_id == tape.tape_id;
            if var_lhs || var_rhs {
                new_tape_id   = tape.tape_id;
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op2arg.push( tape.arg_all.len() as Tindex );
                if var_lhs && var_rhs {
                    tape.id_all.push( id::[< $Name:upper _VV_OP >] );
                    tape.arg_all.push( lhs.var_index as Tindex );
                    tape.arg_all.push( rhs.var_index as Tindex );
                } else if var_lhs {
                    tape.id_all.push( id::[< $Name:upper _VC_OP >] );
                    tape.arg_all.push( lhs.var_index as Tindex );
                    tape.arg_all.push( tape.con_all.len() as Tindex );
                    tape.con_all.push( rhs.value.clone() );
                } else {
                    tape.id_all.push( id::[< $Name:upper _CV_OP >] );
                    tape.arg_all.push( tape.con_all.len() as Tindex );
                    tape.con_all.push( lhs.value.clone() );
                    tape.arg_all.push( rhs.var_index as Tindex );
                }
            }
        }
        ( new_tape_id, new_var_index )
    }
    // -----------------------------------------------------------------------
    #[doc = concat!(
        "& `AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_binary_op]"
    )]
    impl<'a, V> std::ops::$Name< &'a AD<V> > for &'a AD<V>
    where
        &'a V: std::ops::$Name<&'a V, Output=V>,
        V    : Clone + crate::numvec::ThisThreadTapePublic ,
    {   type Output = AD<V>;
        //
        fn [< $Name:lower >](self : &'a AD<V> , rhs : &'a AD<V> ) -> AD<V>
        {
            // new_value
            let new_value     = &self.value  $Op &rhs.value;
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // new_tape_id, new_var_index
            let (new_tape_id, new_var_index) =
                local_key.with_borrow_mut( |tape|
                    [< record_ $Name:lower >] ( tape, &self, &rhs )
            );
            //
            // result
            AD::new(new_tape_id, new_var_index, new_value)
        }
    }
} } }
//
ad_binary_op!(Add, +);
ad_binary_op!(Sub, -);
ad_binary_op!(Mul, *);
ad_binary_op!(Div, /);
// ---------------------------------------------------------------------------
/// Compound Assignment `AD<V>` operators.
///
/// V : see [doc_generic_v]
///
/// Op : is the source code token for this binary operator;
/// i.e., `+=` , `-=` , `*=` , or `/=` .
///
/// Prototype:
/// <br/>
/// & `AD<V>` *Op* & `AD<V>`
///
/// # Example
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
///
/// let mut a   = ad_from_value( 3.0f64 );
/// let b       = ad_from_value( 4.0f64 );
/// a          -= &b;
/// assert_eq!( a.to_value(), -1.0f64 );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
/// use rustad::numvec::NumVec;
///
/// let x     : Vec<f32>  = vec![ 1.0, 4.0 ];
/// let y     : Vec<f32>  = vec![ 2.0, 2.0 ];
/// let x_nv              = NumVec::new(x);
/// let y_nv              = NumVec::new(y);
/// let mut ax            = ad_from_value(x_nv);
/// let ay                = ad_from_value(y_nv);
/// ax                   *= &ay;
/// assert_eq!( ax.to_value().vec, vec![2.0f32, 8.0f32] );
/// ```
pub fn doc_ad_compound_op() { }
//
/// Add one compound assignment operator to the `AD<V>` class;
///
/// * Name : is the operator name with Assign at end;
/// i.e., Add, Sub, Mul, or Div.
///
/// * Op : is the operator token; i.e., +=, -=, *=, or /= .
///
/// see [doc_ad_compound_op]
//
/// see [doc_ad_compound_op]
macro_rules! ad_compound_op { ($Name:ident, $Op:tt) => { paste::paste! {
    // ------------------------------------------------------------------------
     fn [< record_ $Name:lower _assign >]<V> (
         tape: &mut Tape<V> ,
         lhs:  &mut AD<V>   ,
         rhs:  &    AD<V>   )
     where
        V : Clone,
     {
         if tape.recording {
             let var_lhs    = lhs.tape_id == tape.tape_id;
             let var_rhs    = rhs.tape_id == tape.tape_id;
             if var_lhs || var_rhs {
                 tape.op2arg.push( tape.arg_all.len() as Tindex );
                 if var_lhs && var_rhs {
                     tape.id_all.push( id::[< $Name:upper _VV_OP >] );
                     tape.arg_all.push( lhs.var_index as Tindex);
                     tape.arg_all.push( rhs.var_index as Tindex);
                 } else if var_lhs {
                     tape.id_all.push( id::[< $Name:upper _VC_OP >] );
                     tape.arg_all.push( lhs.var_index as Tindex);
                     tape.arg_all.push( tape.con_all.len() as Tindex );
                     tape.con_all.push( rhs.value.clone() );
                 } else {
                     tape.id_all.push( id::[< $Name:upper _CV_OP >] );
                     tape.arg_all.push( tape.con_all.len() as Tindex );
                     tape.con_all.push( lhs.value.clone() );
                     tape.arg_all.push( rhs.var_index as Tindex);
                 }
                 lhs.tape_id   = tape.tape_id;
                 lhs.var_index = tape.n_var;
                 tape.n_var   += 1;
             }
         }
     }
    // ------------------------------------------------------------------------
    #[doc = concat!(
        "`AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_compound_op]"
    )]
    impl<'a, V> std::ops::[< $Name Assign >] < &'a AD<V> > for AD<V>
    where
        V: Clone +
            std::ops::[< $Name Assign >] <&'a V> +
            crate::numvec::ThisThreadTapePublic  ,
    {   //
        fn [< $Name:lower _assign >] (&mut self, rhs : &'a AD<V> )
        {   //
            // self.value
            self.value $Op &rhs.value;
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // tape, self.tape_id, self.var_index
            local_key.with_borrow_mut( |tape|
                [< record_ $Name:lower _assign >] ( tape, self, rhs )
            );
        }
    }
} } }
//
ad_compound_op!(Add, +=);
ad_compound_op!(Sub, -=);
ad_compound_op!(Mul, *=);
ad_compound_op!(Div, /=);
