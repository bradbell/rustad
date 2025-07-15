// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD an automatic differentiation floating point type
//! : [parent module](super)
//
use crate::{Index, Float};
//
#[cfg(doc)]
use crate::function::{ADFun, ad_domain};
//
#[cfg(doc)]
use crate::ad_tape::this_thread_tape;
// ---------------------------------------------------------------------------
// GAD
//
/// Generic AD, acts like F but in addition can record
/// a function evaluation.
///
/// The recording is used to create an [ADFun] object.
///
/// * F : is the floating point type used for computations.
///
/// * U : is the unsigned integer type used for indices in the tape.
///
/// * variable :
/// An AD object is a variable if it one of the [ad_domain] variables
/// or its value depends on the value of the domain variable.
///
/// * constant :
/// If an AD object is not a variable it is referred to as a constant.
///
#[derive(Copy, Clone, Debug)]
pub struct GAD<F, U> {
    //
    // tape_id
    ///
    /// An AD object is a variable if the following two conditions hold:
    /// 1. This threads tape is currently recording.
    /// 2. This threads tape and the AD object have the same *tape_id* .
    pub(crate) tape_id   : U,
    //
    // var_index
    /// If this AD object is a variable, *var_index* is its index in the tape.
    pub(crate) var_index : U,
    //
    // value
    /// is the value of this AD variable or constant.
    pub(crate) value     : F,
}
//
// AD
/// AD is a specific GAD type.
pub type AD = GAD<Float, Index>;
// -------------------------------------------------------------------------
// ad_from_value!
//
/// Convert from a value to a GAD<F, U> type
///
/// * f1 : is the GAD floating point type F.
/// * u2 : is the GAD tape index type U.
/// * t3 : is the type being converted to GAD<F, U>.
///
/// # Example
/// ```
/// // ad_from_value!(f32, u32, f32) makes the following work:
/// use rustad::ad::GAD;
/// let value  : f32          = 3.0;
/// let avalue : GAD<f32,u32> = GAD::from(value);
/// assert_eq!(avalue.to_value(), value);
/// ```
///
macro_rules! ad_from_value { ($f1:ident , $u2:ident , $t3:ident) => {
        impl From<$t3> for GAD<$f1, $u2> {
        #[doc = concat!(
            " Convert from ", stringify!($t3),
            " to an GAD\\<", stringify!($f1),
            ", ", stringify!($u2),  "\\> constant"
        ) ]
        fn from(fvalue : $t3) -> Self { Self
            {tape_id: 0 as $u2, var_index: 0 as $u2, value: fvalue as $f1, }
        }
    }
} }
//
ad_from_value!(f32, u32, f32);
ad_from_value!(f32, u64, f32);
ad_from_value!(f32, u32, f64);
ad_from_value!(f32, u64, f64);
ad_from_value!(f32, u32, isize);
ad_from_value!(f32, u64, isize);
//
ad_from_value!(f64, u32, f32);
ad_from_value!(f64, u64, f32);
ad_from_value!(f64, u32, f64);
ad_from_value!(f64, u64, f64);
ad_from_value!(f64, u32, isize);
ad_from_value!(f64, u64, isize);
// -------------------------------------------------------------------------
// GAD<F, U>::to_value
//
/// Convert from a GAD to its value
///
/// # Example
/// ```
/// use rustad::ad::GAD;
/// let ax : GAD<f64, u32> = GAD::from(4.0);
/// let x = ax.to_value();
/// assert_eq!(x, 4.0);
/// ```
impl<F : Copy, U> GAD<F, U> {
    /// Extract value from a  AD object, variable information is lost
    pub fn to_value(&self) -> F { self.value }
}
// -------------------------------------------------------------------------
// Display
//
/// Display only shows the value and ignores the variable information.
///
/// # Example
/// ```
/// use rustad::ad::GAD;
/// let ax : GAD<f64, u32> = GAD::from(3);
/// let s = format!( "{ax}" );
/// assert_eq!(s, "3");
///```
impl<F : std::fmt::Display, U> std::fmt::Display for GAD<F, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// -------------------------------------------------------------------------
// PartialEq
//
/// Two GAD object are equal if their  values are equal.
///
///
/// # Example
/// ```
/// use rustad::ad::GAD;
/// let ax : GAD<f32, u64> = GAD::from(3.0);
/// let ay : GAD<f32, u64> = GAD::from(3);
/// assert_eq!(ax, ay);
///```
impl<F : std::cmp::PartialEq, U> PartialEq for GAD<F, U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
// ---------------------------------------------------------------------------
// binary_ad_operatror_case
//
// If you try to make the cases in this macro below generic,
// you get a message saying that $f1 must be covered
// because it is not a local type.
macro_rules! binary_ad_operator_case{
    ($f1:ident, $u2:ident, $t3:ident, $o4:tt) => { paste::paste! {
        #[doc =
            "see [doc_binary_ad_operator](crate::ad::doc_binary_ad_operator)"
        ]
        impl std::ops::$t3< GAD<$f1,$u2> > for $f1
        where
        GAD<$f1,$u2> : std::ops::$t3<Output = GAD<$f1,$u2> > ,
        {   type Output = GAD<$f1,$u2>;
            //
#[ doc = concat!(
        " compute GAD<", stringify!($f1), ", ", stringify!($u2), "> ",
        stringify!($o4), " ", stringify!($f1)
            ) ]
            fn [< $t3:lower >] (self, rhs : GAD<$f1,$u2>) -> GAD<$f1,$u2> {
                GAD::from(self) $o4 rhs
            }
        }
    }
    } }
pub(crate) use binary_ad_operator_case;
// ---------------------------------------------------------------------------
// binary_ad_operator!
//
/// Binary GAD<F,U> operators
///
/// * F : is the floating point type used for value calculations.
/// * U : is the unsigned integer type used for tape indices.
///
/// | Left      | Operator | Right     |
/// |-----------|----------|-----------|
/// | GAD<F,U>  | +, *     | GAD<F,U>  |
/// | F         | +, *     | GAD<F,U>  |
/// | GAD<F,U>  | +, *     | F         |
///
/// # Example
/// ```
/// use rustad::ad::GAD;
/// let ax : GAD<f32, u32> = GAD::from(3.0);
/// let ay : GAD<f32, u32> = GAD::from(4.0);
/// let az = ax + ay;
/// assert_eq!(GAD::from(7.0), az);
///```
///
pub fn doc_binary_ad_operator() { }
//
/// This macro implements the the GAD<F, U> binary operators.
/// This include storing the operation in the tape for this thread and AD type.
/// See [doc_binary_ad_operator].
///
/// * Trait :
/// is the std::ops trait for this operator; e.g., Add .
///
/// * op :
/// is the token for this operator; e.g., + .
///
macro_rules! binary_ad_operator { ($Trait:ident, $op:tt) => {paste::paste! {
    //
    fn [< record_ $Trait:lower >]<F,U> (
        tape: &mut GTape<F,U> ,
        lhs: &GAD<F,U>        ,
        rhs: &GAD<F,U>        ,
    ) -> (U, U)
    where
        F     : Copy ,
        U     : GenericAs<usize> + Copy ,
        usize : GenericAs<U>
    {
        let mut new_tape_id   = 0;
        let mut new_var_index = 0;
        if tape.recording {
            let var_lhs    = GenericAs::gas(lhs.tape_id) == tape.tape_id;
            let var_rhs    = GenericAs::gas(rhs.tape_id) == tape.tape_id;
            if var_lhs || var_rhs {
                new_tape_id   = tape.tape_id;
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op2arg.push( GenericAs::gas(tape.arg_all.len()) );
                if var_lhs && var_rhs {
                    tape.id_all.push( [< $Trait:upper _VV_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( rhs.var_index );
                } else if var_lhs {
                    tape.id_all.push( [< $Trait:upper _VC_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( GenericAs::gas(tape.con_all.len()) );
                    tape.con_all.push( rhs.value );
                } else {
                    tape.id_all.push( [< $Trait:upper _CV_OP >] );
                    tape.arg_all.push( GenericAs::gas(tape.con_all.len()) );
                    tape.con_all.push( lhs.value );
                    tape.arg_all.push( rhs.var_index );
                }
            }
        }
        ( GenericAs::gas(new_tape_id), GenericAs::gas(new_var_index) )
    }
    //
    #[doc = "see [doc_binary_ad_operator](crate::ad::doc_binary_ad_operator)"]
    impl<F,U> std::ops::$Trait< GAD<F,U> > for GAD<F,U>
    where
    F     : Copy + std::ops::$Trait<Output = F>  + ThisThreadTape<U> ,
    U     : 'static + GenericAs<usize> + Copy ,
    usize : GenericAs<U>
    {   type Output = Self;
        //
        fn [< $Trait:lower >] (self, rhs : Self) -> Self {
            let new_value = self.value $op rhs.value;
            let local_key :
                &LocalKey< RefCell< GTape<F, U> > > = this_thread_tape();
            let ( new_tape_id, new_var_index) =
            local_key.with_borrow_mut(
                |tape| [< record_ $Trait:lower >] (tape, &self, &rhs)
            );
            Self {
                tape_id   : new_tape_id,
                var_index : new_var_index,
                value     : new_value,
            }
        }
    }
    //
    #[doc = "see [doc_binary_ad_operator](crate::ad::doc_binary_ad_operator)"]
    impl<F,U> std::ops::$Trait<F> for GAD<F,U>
    where
    GAD<F,U> : From<F> ,
    F        : Copy + std::ops::$Trait<Output = F>  + ThisThreadTape<U> ,
    U        : 'static + GenericAs<usize> + Copy ,
    usize    : GenericAs<U>
    {   type Output = Self;
        //
        #[ doc = concat!(" compute AD ", stringify!($op), " Float") ]
        fn [< $Trait:lower >] (self, rhs : F) -> Self {
            self $op GAD::from(rhs)
        }
    }
    //
    crate::ad::binary_ad_operator_case!(f32, u32, $Trait, $op);
    crate::ad::binary_ad_operator_case!(f32, u64, $Trait, $op);
    crate::ad::binary_ad_operator_case!(f64, u32, $Trait, $op);
    crate::ad::binary_ad_operator_case!(f64, u64, $Trait, $op);
} } }
//
pub(crate) use binary_ad_operator;
// ---------------------------------------------------------------------------
// binary_ad_assign_op!
//
/// Compound Assignment GAD<F,U> operators
///
/// | Left      | Operator | Right     |
/// |-----------|----------|-----------|
/// | GAD<F,U>  | +=, *=   | GAD<F,U>  |
/// | GAD<F,U>  | +=, *=   | F         |
///
/// # Example
/// ```
/// use rustad::ad::GAD;
/// let ax     : GAD<f32, u64> = GAD::from(3.0);
/// let mut ay : GAD<f32, u64> = GAD::from(4.0);
/// ay += ax;
/// assert_eq!(GAD::from(7.0), ay);
///```
pub fn doc_binary_ad_assign_op() { }
//
/// This macro implements the the GAD<F, U> compound assignment operators.
/// This include storing the operation in the tape for this thread and AD type.
/// See [doc_binary_ad_assign_op].
///
/// * Name :
/// is the operator name for this compound assignment; e.g., Add .
///
/// * op :
/// is the token for this compound assignment; e.g., += .
///
macro_rules! binary_ad_assign_op {
    ($Name:ident, $op:tt) => {paste::paste! {
        crate::ad::binary_ad_assign_op!( $Name, $op, [< $Name Assign >] );
    } };
    ($Name:ident, $op:tt, $Trait:ident) => {paste::paste! {
        //
        fn [< record_ $Trait:snake >]<F,U> (
            tape: &mut GTape<F,U> ,
            lhs: &mut GAD<F,U>    ,
            rhs: &GAD<F,U>        )
        where
            F     : Copy,
            U     : GenericAs<usize> + Copy,
            usize : GenericAs<U> ,
        {
            if tape.recording {
                let var_lhs    = GenericAs::gas(lhs.tape_id) == tape.tape_id;
                let var_rhs    = GenericAs::gas(rhs.tape_id) == tape.tape_id;
                if var_lhs || var_rhs {
                    tape.op2arg.push( GenericAs::gas(tape.arg_all.len()) );
                    if var_lhs && var_rhs {
                        tape.id_all.push( [< $Name:upper _VV_OP >] );
                        tape.arg_all.push( lhs.var_index );
                        tape.arg_all.push( rhs.var_index );
                    } else if var_lhs {
                        tape.id_all.push( [< $Name:upper _VC_OP >] );
                        tape.arg_all.push( lhs.var_index );
                        tape.arg_all.push( GenericAs::gas(tape.con_all.len()) );
                        tape.con_all.push( rhs.value );
                    } else {
                        tape.id_all.push( [< $Name:upper _CV_OP >] );
                        tape.arg_all.push( GenericAs::gas(tape.con_all.len()) );
                        tape.con_all.push( lhs.value );
                        tape.arg_all.push( rhs.var_index );
                    }
                    lhs.tape_id   = GenericAs::gas(tape.tape_id);
                    lhs.var_index = GenericAs::gas(tape.n_var);
                    tape.n_var   += 1;
                }
            }
        }
        //
        impl<F,U> std::ops::$Trait<GAD<F,U>> for GAD<F,U>
        where
            GAD<F,U> : From<F> ,
            F        : ThisThreadTape<U> + std::ops::$Trait + Copy,
            U        : 'static + GenericAs<usize> + Copy,
            usize    : GenericAs<U> ,
        {
            fn [< $Trait:snake >] (&mut self, rhs : GAD<F,U>) {
                let local_key : &LocalKey< RefCell< GTape<F,U> > > =
                        this_thread_tape();
                local_key.with_borrow_mut(
                    |tape| [< record_ $Trait:snake >] (tape, self, &rhs)
                );
                let _ = self.value $op rhs.value;
            }
        }
        //
        impl<F,U> std::ops::$Trait<F> for GAD<F,U>
        where
            GAD<F,U> : From<F> ,
            F        : ThisThreadTape<U> + std::ops::$Trait + Copy,
            U        : 'static + GenericAs<usize> + Copy,
            usize    : GenericAs<U> ,
        {
            fn [< $Trait:snake >] (&mut self, rhs : F) {
                let _ = *self $op GAD::from(rhs);
            }
        }
    } };
}
//
// make this macro visible in the entire crate
pub(crate) use binary_ad_assign_op;
// -------------------------------------------------------------------------
// gadvec!
//
/// Create a vector with GAD<F,U> elements
///
/// * F :
/// is the floating point type used for value calculations.
///
/// * U :
/// is the unsigned integer type used for indices in the tape.
///
/// * E :
/// is one of the elements. It must have a type that can be converted
/// using GAD::from.
///
///```
/// use rustad::ad::GAD;
/// let avec = rustad::gadvec![ f64, u64, 1f32, 2f64, 3isize ];
/// assert_eq!( avec.len(), 3);
/// assert_eq!( avec[0], GAD::from(1) );
/// assert_eq!( avec[1], GAD::from(2) );
/// assert_eq!( avec[2], GAD::from(3) );
/// ```
#[macro_export]
macro_rules! gadvec {
    ( $F:ident,  $U:ident,  $( $E:expr ),* ) => {
        {
            let mut avec : Vec< rustad::ad::GAD<$F,$U> > = Vec::new();
            $(
                avec.push( rustad::ad::GAD::from( $E ) );
            )*
            avec
        }
    };
}
