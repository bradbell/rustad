// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] rust_src method (rust source code for function value).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use std::any::type_name;
use crate::ADfn;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
// -----------------------------------------------------------------------
// prototype
fn prototype<V>(fn_name : &str) -> String {
    let result = String::new();
    let result = result
        + "unsafe extern \"C\" "  + fn_name + "(\n"
        + "    domain      : *" + type_name::<V>()  + ",\n"
        + "    domain_len  : usize,\n"
        + "    range       : *mut " + type_name::<V>()  + ",\n"
        + "    range_len   : usize,\n"
        + "    message     : *mut u8,\n"
        + "    message_len : usize,\n"
        + ") -> usize\n";
    result
}
//
// -----------------------------------------------------------------------
// rust_src
/// Under Construction:
/// Rust source code for zero order forward mode evaluation; i.e.,
/// function value.
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * f :
/// is an [ADfn] object.
///
/// * fn_name :
/// is the name of the rust function created by this operation.
///
/// ```text
/// unsafe extern "C" fn_name(
///     domain      : *V           ,
///     domain_len  : usize        ,
///     range       : *mut V       ,
///     range_len   : usize        ,
///     message     : *mut u8      ,
///     message_len : usize        ,
/// ) -> usize;
/// ```
///
/// * domain :
/// is a raw pointer to the vector containing the value of the domain variables.
///
/// * domain_len :
/// is the length of the domain vector. The domain pointer must be valid
/// for all indices less than this length.
///
/// * range :
/// is a raw pointer to the vector where the range results will be stored.
///
/// * range_len :
/// is the length of the range vector. The range pointer must be valid
/// for all indices less than this length.
///
/// * message :
/// is a raw pointer to the vector where a messages
/// about the evaluation of fn_name is stored.
///
/// * message_len :
/// is the length of the message vector. The message pointer must be valid
/// for all indices less than this length.
/// This must be greater than zero and
/// error messages are  truncated to this length.
///
/// * return :
/// The fn_name return value is the length of the message.
/// If it is zero, there was no error and the range vector has been
/// set properly.
///
///
impl<V> ADfn<V>
{
    pub fn rust_src(&self, f_name : &str) -> String {
        let result = prototype::<V>(f_name);
        result
    }
}
