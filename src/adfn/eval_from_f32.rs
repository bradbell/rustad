//! Define the eval_from_f32 macro.
//!
//! Link to [parent module](super)
//
#[cfg(doc)]
use crate::doc_generic_v;
//
// eval_from_f32!
/// Convert from an f32 value to the evaluation type.
///
/// ```syntax
///     let eval_value = eval_from_f32(suffix, V, f32_value)
/// ```
///
/// * suffix : If this is `value`, the evaluation type is V.
/// Otherwise suffix must be `ad` and the evaluation type is `AD<V>`
///
/// * V : This is V in [doc_generic_v]. Note that This type supports
/// `V::from(f32_value)` .
///
/// * f32_value : This is the f32 value being converted.
///
/// * eval_value : This is the converted value; If suffix is value,
/// it has type V. Otherwise, it has type `AD<V>` .
///
///
macro_rules! eval_from_f32 {
    //
    (value, $V:ident, $f32_value:expr) => {{
        $V::from( $f32_value )
    }};
    (ad,    $V:ident, $f32_value:expr) => {{
        crate::ad::ad_from_value( $V::from( $f32_value ) )
    }};
}
pub(crate) use eval_from_f32;
