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
use crate::ad_tape::THIS_THREAD_TAPE;
//
/// AD acts like the Float. It also can record functions and store
/// them in [ADFun] objects.
///
/// # variable
/// An AD object is a variable if it one of the [ad_domain] variables
/// or its value depends on the value of the domain variable.
///
/// # constant
/// If an AD object is not a variable it is referred to as a constant.
#[derive(Copy, Clone, Debug)]
pub struct AD {
    //
    // tape_id
    ///
    /// An AD object is a variable if the following two conditions hold:
    /// 1. [THIS_THREAD_TAPE] is currently recording.
    /// 2. This threads tape and the AD object have the same *tape_id* .
    pub(crate) tape_id   : Index,
    //
    // var_index
    /// If this AD object is a variable, var_index is its index in the tape.
    pub(crate) var_index : Index,
    //
    // value
    /// This is the value of this AD variable or constant.
    pub(crate) value     : Float,
}
// -------------------------------------------------------------------------
impl AD {
    /// Extract value from an object (dependencies are lost)
    pub fn to_float(&self) -> Float { self.value }
}
//
// -------------------------------------------------------------------------
impl From<f64> for AD {
    /// Convert from f64 to an AD constant
    fn from(this_value : f64) -> AD {
        AD {tape_id: 0, var_index: 0, value: Float::from(this_value), }
    }
}
impl From<f32> for AD {
    /// Convert from f32 to an AD constant
    fn from(this_value : f32) -> AD {
        AD {tape_id: 0, var_index: 0, value: Float::from(this_value), }
    }
}
/// This maacro implements conversion from integer types to AD
macro_rules! impl_ad_from_integer {
    ($integer:tt) => { paste::paste!{
        impl From< [< i $integer >] > for AD {
            #[doc = concat!(
                " Convert from i", stringify!($integer), " to an AD constant"
            ) ]
            fn from(from_value : [< i $integer >] ) -> AD {
                let float_value = from_value as Float;
                AD {tape_id: 0, var_index: 0, value: float_value, }
            }
        }
        impl From< [< u $integer >] > for AD {
            #[doc = concat!(
                " Convert from u", stringify!($integer), " to an AD constant"
            ) ]
            fn from(from_value : [< u $integer >] ) -> AD {
                let float_value = from_value as Float;
                AD {tape_id: 0, var_index: 0, value: float_value, }
            }
        }
    } }
}
impl_ad_from_integer!(8);
impl_ad_from_integer!(16);
impl_ad_from_integer!(32);
impl_ad_from_integer!(64);
impl_ad_from_integer!(128);
// -------------------------------------------------------------------------
//
/// Display will only show the value and ignore the variable information.
///
/// # Example
/// ```
/// use rustad::{AD, Float};
/// let x = rustad::AD::from( rustad::Float::from(3) );
/// let s = format!( "{x}" );
/// assert_eq!(s, "3");
///```
impl std::fmt::Display for AD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // only display value
        write!(f, "{}", self.value)
    }
}
//
// PartialEq
/// Two AD object are equal if their Float values are equal.
/// ```
/// use rustad::{AD, Float};
/// assert_eq!( AD::from( Float::from(3.0) ), AD::from( Float::from(3) ) );
///```
impl PartialEq for AD {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
// ---------------------------------------------------------------------------
// binary_AD_operator
/// This macro implements the the following binary operations:
/// <pre>
///     AD    op AD
///     Float op AD
///     AD    op Float
/// </pre>
/// This include storing the operation in the [THIS_THREAD_TAPE] .
///
/// # Trait
/// is the std::ops trait for this operator; e.g., Add .
///
/// # op
/// is the token for this operator; e.g., + .
///
macro_rules! binary_ad_operator { ($Trait:ident, $op:tt) => {paste::paste! {
    //
    #[ doc = concat!(" record an ", stringify!($Trait), " operation ") ]
    fn [< record_ $Trait:lower >] (tape: &mut Tape, lhs: &AD, rhs: &AD) ->
    (Index, Index) {
        let mut new_tape_id   = 0;
        let mut new_var_index = 0;
        if tape.recording {
            let var_lhs    = lhs.tape_id == tape.tape_id;
            let var_rhs    = rhs.tape_id == tape.tape_id;
            if var_lhs || var_rhs {
                new_tape_id   = tape.tape_id;
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op2arg.push( tape.arg_all.len() );
                if var_lhs && var_rhs {
                    tape.id_all.push( [< $Trait:upper _VV_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( rhs.var_index );
                } else if var_lhs {
                    tape.id_all.push( [< $Trait:upper _VC_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( rhs.value );
                } else {
                    tape.id_all.push( [< $Trait:upper _CV_OP >] );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( lhs.value );
                    tape.arg_all.push( rhs.var_index );
                }
            }
        }
        (new_tape_id, new_var_index)
    }
    //
    impl std::ops::$Trait<AD> for AD {
        type Output = AD;
        //
        #[ doc = concat!(" compute AD ", stringify!($op), " AD") ]
        fn [< $Trait:lower >] (self, rhs : AD) -> AD {
            let new_value = self.value $op rhs.value;
            let ( new_tape_id, new_var_index) =
            THIS_THREAD_TAPE.with_borrow_mut(
                |tape| [< record_ $Trait:lower >] (tape, &self, &rhs)
            );
            AD {
                tape_id   : new_tape_id,
                var_index : new_var_index,
                value     : new_value,
            }
        }
    }
    //
    impl std::ops::$Trait<AD> for Float {
        type Output = AD;
        //
        #[ doc = concat!(" compute Float ", stringify!($op), " AD") ]
        fn [< $Trait:lower >] (self, rhs : AD) -> AD {
            AD::from(self) $op rhs
        }
    }
    //
    impl std::ops::$Trait<Float> for AD {
        type Output = AD;
        //
        #[ doc = concat!(" compute AD ", stringify!($op), " Float") ]
        fn [< $Trait:lower >] (self, rhs : Float) -> AD {
            self $op AD::from(rhs)
        }
    }
} } }
//
pub(crate) use binary_ad_operator;
// ---------------------------------------------------------------------------
// binary_ad_assign_op
/// This macro implements the the following binary assignment operations:
/// <pre>
///     AD op= AD
///     AD op= Float
/// </pre>
/// This include storing the operation in the [THIS_THREAD_TAPE] .
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
            let var_lhs    = lhs.tape_id == tape.tape_id;
            let var_rhs    = rhs.tape_id == tape.tape_id;
            if var_lhs || var_rhs {
                tape.op2arg.push( tape.arg_all.len() );
                if var_lhs && var_rhs {
                    tape.id_all.push( [< $Name:upper _VV_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( rhs.var_index );
                } else if var_lhs {
                    tape.id_all.push( [< $Name:upper _VC_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( rhs.value );
                } else {
                    tape.id_all.push( [< $Name:upper _CV_OP >] );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( lhs.value );
                    tape.arg_all.push( rhs.var_index );
                }
                lhs.tape_id   = tape.tape_id;
                lhs.var_index = tape.n_var;
                tape.n_var   += 1;
            }
        }
    }
    //
    impl std::ops::[< $Name Assign >]<AD> for AD {
        #[ doc = concat!(" compute AD ", stringify!($symbol), " AD") ]
        fn [< $Name:lower _assign >] (&mut self, rhs : AD) {
            THIS_THREAD_TAPE.with_borrow_mut(
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
///     assert_eq!( avec.len() , 4 );
///     assert_eq!( avec[3], AD::from(4) );
/// }
/// let avec = advec![ 1f32, 2f64, 3u128, 4i8 ];
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
