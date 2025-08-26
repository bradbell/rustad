// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD a generic automatic differentiation floating point type
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
/// The type used for indices in the tape and function objects
type Tindex = u32;
// ---------------------------------------------------------------------------
// AD
//
/// AD acts like V but in addition can record a function evaluation.
///
/// * V : is the floating point type used for value calculations.
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
    /// 2DO: Change to pub(crate) when this gets used.
    pub tape_id   : Tindex,
    //
    // var_index
    /// If this AD object is a variable, *var_index* is its index in the tape.
    /// 2DO: Change to pub(crate) when this gets used.
    pub var_index : Tindex,
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
        new_tape_id: Tindex, new_var_index: Tindex, new_value: V )-> Self {
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
///```
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
    let tape_id   = 0 as Tindex;
    let var_index = 0 as Tindex;
    AD::new(tape_id, var_index, value)
}
