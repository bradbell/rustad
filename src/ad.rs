// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD an automatic differentiation floating point type: [parent module](super)
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
/// Syntax
/// <pre>
///     let avalue : GAD&lt;F, U&gt; = GAD::from(value)
/// </pre>
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
impl<F : Clone, U> GAD<F, U> {
    /// Extract value from a  AD object, variable information is lost
    pub fn to_value(&self) -> F { self.value.clone() }
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
//
// binary_ad_operator!
//
/// Binary AD operators
///
/// | Left      | Operator | Right     |
/// |-----------|----------|-----------|
/// | AD        | +, *     | AD        |
/// | F         | +, *     | AD        |
/// | AD        | +, *     | F         |
///
pub fn doc_binary_ad_operator() { }
//
/// This macro implements the the following binary operations:
///
/// | Left        | Operator| Right       |
/// |-------------|---------|-------------|
/// | AD          | op      | AD          |
/// | f1          | op      | AD          |
/// | AD          | op      | f1          |
///
///
/// This include storing the operation in the tape for this thread and AD type.
///
/// * Traig
/// is the std::ops trait for this operator; e.g., Add .
///
/// * op
/// is the token for this operator; e.g., + .
///
macro_rules! binary_ad_operator { ($Trait:ident, $op:tt) => {paste::paste! {
    //
    #[ doc = " see [doc_binary_ad_operator]" ]
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
    impl<F,U> std::ops::$Trait< GAD<F,U> > for GAD<F,U>
    where
    F     : Copy + std::ops::$Trait<Output = F>  + ThisThreadTape<U> ,
    U     : 'static + GenericAs<usize> + Copy ,
    usize : GenericAs<U>
    {   type Output = Self;
        //
        #[ doc = concat!(" compute AD ", stringify!($op), " AD") ]
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
// binary_ad_assign_op
/// This macro implements the following binary assignment operations:
/// <pre>
///     AD op= AD
///     AD op= Float
/// </pre>
/// This include storing the operation in the [this_thread_tape] .
///
/// # Name
/// is the std::ops trait for this operator without the Assign;
/// e.g., Add .
///
/// # symbol
/// is the token for this operator; e.g., += .
///
macro_rules! binary_ad_assign_op { ($Name:ident, $symbol:tt) => {paste::paste! {
    //
    #[ doc = concat!(" record an ", stringify!($Name), "Assign operation ") ]
    fn [< record_ $Name:lower _assign>]
    (tape: &mut Tape, lhs: &mut AD, rhs: &AD) {
        if tape.recording {
            let var_lhs    = lhs.tape_id as usize == tape.tape_id;
            let var_rhs    = rhs.tape_id as usize == tape.tape_id;
            if var_lhs || var_rhs {
                tape.op2arg.push( tape.arg_all.len() as Index);
                if var_lhs && var_rhs {
                    tape.id_all.push( [< $Name:upper _VV_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( rhs.var_index );
                } else if var_lhs {
                    tape.id_all.push( [< $Name:upper _VC_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( tape.con_all.len() as Index);
                    tape.con_all.push( rhs.value );
                } else {
                    tape.id_all.push( [< $Name:upper _CV_OP >] );
                    tape.arg_all.push( tape.con_all.len() as Index);
                    tape.con_all.push( lhs.value );
                    tape.arg_all.push( rhs.var_index );
                }
                lhs.tape_id   = tape.tape_id as Index;
                lhs.var_index = tape.n_var as Index;
                tape.n_var   += 1;
            }
        }
    }
    //
    impl std::ops::[< $Name Assign >]<AD> for AD {
        #[ doc = concat!(" compute AD ", stringify!($symbol), " AD") ]
        fn [< $Name:lower _assign >] (&mut self, rhs : AD) {
            let local_key : &LocalKey< RefCell< GTape<Float, Index> > > =
                    this_thread_tape();
            local_key.with_borrow_mut(
                |tape| [< record_ $Name:lower _assign >] (tape, self, &rhs)
            );
            let _ = self.value $symbol rhs.value;
        }
    }
    //
    impl std::ops::[< $Name Assign >] <Float> for AD {
        #[ doc = concat!(" compute AD ", stringify!($symbol), " Float") ]
        fn [< $Name:lower _assign >] (&mut self, rhs : Float) {
            let _ = *self $symbol AD::from(rhs);
        }
    }
} } }
//
// make this macro visible in the entire crate
pub(crate) use binary_ad_assign_op;
// -------------------------------------------------------------------------
// advec
/// Create a vector with AD elements: [source module](crate::ad)
///```
/// use rustad::{Float, AD, advec};
/// fn check(avec : &Vec<AD> ) {
///     assert_eq!( avec.len() , 3 );
///     assert_eq!( avec[2], AD::from(3.0) );
/// }
/// let avec = advec![ 1f32, 2f64, 3isize ];
/// check(&avec);
/// ```
#[macro_export]
macro_rules! advec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push( rustad::ad::AD::from( $x ) );
            )*
            temp_vec
        }
    };
}
