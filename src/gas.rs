// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Define the GenericAs trait wutih a generic as function
//! [parent module](super)
//
/// Generic as function for converting types like `as` would.
/// So far implemented for the following types:
///
/// * usize <-> u32
///
/// # Example
/// ```
/// use rustad::gas::GenericAs;
/// fn use_gas<T> (f : usize) -> T where usize : GenericAs<T> {
///     GenericAs::gas(f)
/// }
/// let five    : usize = 5;
/// let convert : u32   = use_gas(five);
/// assert_eq!( 5u32, convert );
/// ```
///
pub trait GenericAs<T> {
    fn gas(self : Self) -> T;
}
//
macro_rules! generic_as{ ($F:ident, $T:ident) => {
    impl GenericAs<$T> for $F {
        fn gas(self : Self) -> $T {
            self as $T
        }
    }
} }
//
generic_as!(usize, u32);
generic_as!(u32, usize);
//
generic_as!(usize, u64);
generic_as!(u64, usize);
//
