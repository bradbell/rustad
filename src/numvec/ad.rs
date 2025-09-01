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
/// * Syntax :
/// ```text
///        az = &ax Op &ay
///        az = &ay Op &y
/// ```
///
/// * V : see [doc_generic_v]
///
/// * Op : is the source code token for this binary operator;
/// i.e., `+` , `-` , `*` , or `/` .
///
/// * ax : left hand side `AD<V>` object
/// * ay : right hand side `AD<V>` object
/// * az : result `AD<V>` object
///
/// * x  : left hand side *V* object
/// * y  : right hand side *V* object
///
/// # Example
///```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
///
/// let ax  = ad_from_value( 3.0f32 );
/// let y   = 4.0f32;
/// let az  = &ax * &y;
/// assert_eq!( az.to_value(), 12.0f32 );
///
/// let x  = 3.0f32;
/// let ay = ad_from_value(4.0f32 );
/// let az  = &x * &ay;
/// assert_eq!( az.to_value(), 12.0f32 );
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
//
/// * V : see [doc_generic_v]
/// * Name : is the operator name; i.e., Add, Sub, Mul, or Div.
/// * Op : is the operator token; i.e., +, -, *, or /.
///
/// see [doc_ad_binary_op]
macro_rules! ad_binary_op { ($Name:ident, $Op:tt) => { paste::paste! {
    // -----------------------------------------------------------------------
    fn [< record_ $Name:lower _vv >]<V> (
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
    //
    #[doc = concat!(
        "& `AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_binary_op]"
    )]
    impl<V> std::ops::$Name< &AD<V> > for &AD<V>
    where
        for<'a> &'a V: std::ops::$Name<&'a V, Output=V>,
        V    : Clone + crate::numvec::ThisThreadTapePublic ,
    {   type Output = AD<V>;
        //
        fn [< $Name:lower >](self , rhs : &AD<V> ) -> AD<V>
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
                    [< record_ $Name:lower _vv >] ( tape, &self, &rhs )
            );
            //
            // result
            AD::new(new_tape_id, new_var_index, new_value)
        }
    }
    // -----------------------------------------------------------------------
    fn [< record_ $Name:lower _vc >]<V> (
        tape: &mut Tape<V> ,
        lhs:       &AD<V>  ,
        rhs:       &V      ,
    ) -> (usize, usize)
    where
        V : Clone ,
    {
        let mut new_tape_id   = 0;
        let mut new_var_index = 0;
        if tape.recording {
            let var_lhs    = lhs.tape_id == tape.tape_id;
            if var_lhs {
                new_tape_id   = tape.tape_id;
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op2arg.push( tape.arg_all.len() as Tindex );
                tape.id_all.push( id::[< $Name:upper _VC_OP >] );
                tape.arg_all.push( lhs.var_index as Tindex );
                tape.arg_all.push( tape.con_all.len() as Tindex );
                tape.con_all.push( rhs.clone() );
            }
        }
        (new_tape_id, new_var_index)
    }
    //
    #[doc = concat!(
        "& `AD<V>` ", stringify!($Op), " & V`",
        "; see [doc_ad_binary_op]"
    )]
    impl<'a, V> std::ops::$Name< &'a V> for &'a AD<V>
    where
        &'a V: std::ops::$Name<&'a V, Output=V>,
        V    : Clone + crate::numvec::ThisThreadTapePublic ,
    {   type Output = AD<V>;
        //
        fn [< $Name:lower >](self : &'a AD<V> , rhs : &'a V ) -> AD<V>
        {
            // new_value
            let new_value     = &self.value  $Op &rhs;
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // new_tape_id, new_var_index
            let (new_tape_id, new_var_index) =
                local_key.with_borrow_mut( |tape|
                    [< record_ $Name:lower _vc >] ( tape, &self, &rhs )
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
/// Syntax :
/// ```text
///     ax Op &ay
///     ax Op &y
/// ```
///
/// * V : see [doc_generic_v]
///
/// * Op : is the source code token for this binary operator;
/// i.e., `+=` , `-=` , `*=` , or `/=` .
///
/// * ax : left hand side `AD<V>` object.
/// * ay : right hand size `AD<V>` object
/// * y  : right hand size *V* object
///
/// # Example
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
///
/// let mut ax   = ad_from_value( 3.0f64 );
/// let y        = 4.0f64;
/// ax          -= &y;
/// assert_eq!( ax.to_value(), -1.0f64 );
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
/// * V : see [doc_generic_v]
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
     fn [< record_ $Name:lower _assign_vv >]<V> (
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
    //
    #[doc = concat!(
        "`AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_compound_op]"
    )]
    impl<V> std::ops::[< $Name Assign >] < &AD<V> > for AD<V>
    where
        V: Clone +
            for<'a> std::ops::[< $Name Assign >] <&'a V> +
            crate::numvec::ThisThreadTapePublic  ,
    {   //
        fn [< $Name:lower _assign >] (&mut self, rhs : &AD<V> )
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
                [< record_ $Name:lower _assign_vv >] ( tape, self, rhs )
            );
        }
    }
    // ------------------------------------------------------------------------
     fn [< record_ $Name:lower _assign_vc >]<V> (
         tape: &mut Tape<V> ,
         lhs:  &mut AD<V>   ,
         rhs:  &    V       )
     where
        V : Clone,
     {
         if tape.recording {
             let var_lhs    = lhs.tape_id == tape.tape_id;
             if var_lhs {
                 tape.op2arg.push( tape.arg_all.len() as Tindex );
                 tape.id_all.push( id::[< $Name:upper _VC_OP >] );
                 tape.arg_all.push( lhs.var_index as Tindex);
                 tape.arg_all.push( tape.con_all.len() as Tindex );
                 tape.con_all.push( rhs.clone() );
                 //
                 lhs.var_index = tape.n_var;
                 tape.n_var   += 1;
             }
         }
     }
    //
    #[doc = concat!(
        "`AD<V>` ", stringify!($Op), " & V; see [doc_ad_compound_op]"
    )]
    impl<V> std::ops::[< $Name Assign >] <&V> for AD<V>
    where
        V: Clone +
            for<'a> std::ops::[< $Name Assign >] <&'a V> +
            crate::numvec::ThisThreadTapePublic  ,
    {   //
        fn [< $Name:lower _assign >] (&mut self, rhs : &V)
        {   //
            // self.value
            self.value $Op &rhs;
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // tape, self.tape_id, self.var_index
            local_key.with_borrow_mut( |tape|
                [< record_ $Name:lower _assign_vc >] ( tape, self, rhs )
            );
        }
    }
} } }
//
ad_compound_op!(Add, +=);
ad_compound_op!(Sub, -=);
ad_compound_op!(Mul, *=);
ad_compound_op!(Div, /=);
// ---------------------------------------------------------------------------
// record_value_op_ad!
//
/// Create function that records
/// one binary operation where lhs is *V* and rhs is `AD<V>` .
///
/// * Name         : is the operator name; i.e., Add, Sub, Mul, or Div.
///
/// * Op           : is the operator token; i.e., +, -, *, or /.
///
/// * Function Name: `record_value_` *name* `_ad` where *name* is
///  a lower case version of Name.
///
macro_rules! record_value_op_ad{ ($Name:ident, $Op:tt) => { paste::paste! {
    #[doc = concat!( "record one ", stringify!($Name),
        " where lhs is a value and rhs is a variable"
    ) ]
    pub(crate) fn [< record_value_ $Name:lower _ad >]<V> (
        tape: &mut Tape<V> ,
        lhs:       &V      ,
        rhs:       &AD<V>  ,
    ) -> (usize, usize)
    where
        V : Clone ,
    {
        let mut new_tape_id   = 0;
        let mut new_var_index = 0;
        if tape.recording {
            let var_rhs    = rhs.tape_id == tape.tape_id;
            if var_rhs {
                new_tape_id   = tape.tape_id;
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op2arg.push( tape.arg_all.len() as Tindex );
                tape.id_all.push( id::[< $Name:upper _CV_OP >] );
                tape.arg_all.push( tape.con_all.len() as Tindex );
                tape.con_all.push( lhs.clone() );
                tape.arg_all.push( rhs.var_index as Tindex );
            }
        }
        (new_tape_id, new_var_index)
    }
} } }
record_value_op_ad!(Add, +=);
record_value_op_ad!(Sub, -=);
record_value_op_ad!(Mul, *=);
record_value_op_ad!(Div, /=);
// ---------------------------------------------------------------------------
// impl_value_op_ad!
//
// If you try to make this implementation generic w.r.t V,
// you get a message saying that f32 and f64 must be covered
// because they are not local types.
//
/// Implement one binary `AD<V>` operator where lhs is a *V* object.
///
/// * V : see [doc_generic_v]
/// * Name : is the operator name; i.e., Add, Sub, Mul, or Div.
/// * Op : is the operator token; i.e., +, -, *, or /.
///
/// If *V* is the only argument to this macro, it will invoke itself
/// with *Op* equal to +, -, *, / and the corresponding *Name* .
///
/// see [doc_ad_binary_op]
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::thread::LocalKey;
///     use std::cell::RefCell;
///     use crate::numvec::ad::AD;
/// ```
macro_rules! impl_value_op_ad{
    ($V:ty)                      => {
        crate::numvec::ad::impl_value_op_ad!($V, Add, +);
        crate::numvec::ad::impl_value_op_ad!($V, Sub, -);
        crate::numvec::ad::impl_value_op_ad!($V, Mul, *);
        crate::numvec::ad::impl_value_op_ad!($V, Div, /);
    };
    ($V:ty, $Name:ident, $Op:tt) => { paste::paste! {
        #[doc =
        "see [doc_ad_binary_op](crate::numvec::ad::doc_ad_binary_op)"
        ]
        impl<'a> std::ops::$Name< &'a AD<$V> > for &'a $V
        where
        {   type Output = AD<$V>;
            //
            #[ doc = concat!(
                "compute & `", stringify!($V), "` ",
                stringify!($Op), " & `AD<", stringify!($f1), ">` "
            ) ]
            fn [< $Name:lower >]
                (self : &'a $V, rhs : &'a AD<$V>
            ) -> AD<$V> {
                //
                // new_value
                let new_value = self $Op &rhs.value;
                //
                // local_key
                let local_key : &LocalKey<
                    RefCell< crate::numvec::tape::Tape<$V> >
                > = crate::numvec::tape::sealed::ThisThreadTape::get();
                //
                // new_tape_id, new_var_index
                let (new_tape_id, new_var_index) =
                    local_key.with_borrow_mut( |tape|
                        crate::numvec::ad::[< record_value_ $Name:lower _ad >]
                            ( tape, &self, &rhs )
                );
                //
                // result
                AD::new(new_tape_id, new_var_index, new_value)
            }
        }
    } }
}
pub(crate) use impl_value_op_ad;
