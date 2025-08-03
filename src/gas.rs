// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightext: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! Generic as function for converting types like `as` would.
//! : [parent module](super)
///
// sealed::GenericAs
pub (crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    pub trait GenericAs<D> {
        fn from_(self : Self) -> D;
    }
    //
    /// Implement from_ returning S as D for one (S, D) type pair.
    ///
    /// * S : is the source type
    /// * D : is the destination type
    ///
    macro_rules! generic_as{ ($S:ident, $D:ident) => {
        impl GenericAs<$D> for $S {
            fn from_(self : Self) -> $D {
                self as $D
            }
        }
    } }
    //
    generic_as!(usize, u64);
    generic_as!(usize, u32);
    generic_as!(usize, u16);
    generic_as!(usize, u8);
    //
    generic_as!(u64, usize);
    generic_as!(u32, usize);
    generic_as!(u16, usize);
    generic_as!(u8, usize);
}
//
// as_from
/// Conversion from type S to type D using as.
///
/// Conversion to and from usize is implement for the following types:
/// u8, u16, u32, u64.
///
pub (crate) fn as_from<S,D>(s : S) -> D
where
    S : sealed::GenericAs<D> ,
{   sealed::GenericAs::from_(s)
}
//
#[cfg(test)]
mod examples {

    #[test]
    fn as_from_example() {
        use crate::gas::as_from;
        let five    : usize = 5;
        let convert : u32   = as_from(five);
        assert_eq!( 5u32, convert );
    }
}
