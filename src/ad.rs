// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD an automatic differentiation floating point type
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
/// or its value depends on the value of a domain variable.
///
/// # constant
/// If an AD object is not a variable it is referred to as a constant.
#[derive(Copy, Clone)]
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
//
/// Converting from a Float to an AD creates a constamt with the same value
impl From<Float> for AD {
    /// Convert a Float to an AD constant
    fn from(this_value : Float) -> Self {
        Self {
            tape_id   : 0,
            var_index : 0,
            value     : this_value,
        }
    }
}
//
/// Converting from an AD to a Float
pub fn float_from_ad(ad : AD) -> Float {
    ad.value
}
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
