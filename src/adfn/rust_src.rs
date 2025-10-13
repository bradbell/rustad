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
    let result = result +
        "unsafe extern \"C\" rustad_src_"  + fn_name + "(\n" +
        "   domain      : *" + type_name::<V>()  + ",\n" +
        "   domain_len  : usize,\n" +
        "   range       : *mut " + type_name::<V>()  + ",\n" +
        "   range_len   : usize,\n" +
        "   message     : *mut u8,\n" +
        "   message_len : usize,\n" +
        ") -> usize\n";
    result
}
//
// -----------------------------------------------------------------------
// rust_src
impl<V> ADfn<V>
{
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
    /// The actual function name will be `rust_src` followed by *fn_name* .
    ///
    /// ```text
    /// unsafe extern "C" rust_src_fn_name(
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
    /// is a raw pointer to a vector containing the domain variable values.
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
    pub fn rust_src(&self, f_name : &str) -> String {
        //
        // src
        let mut src    = prototype::<V>(f_name) + "{\n";
        //
        // src
        let expect = self.domain_len().to_string();
        src = src +
            "   // error_msg: check domain_len\n" +
            "   let mut error_msg = String::new()\n" +
            "   if domain_len != " + &expect  + " {\n" +
            "       error_msg = \"domain_len != " + &expect + "\";\n" +
            "   }\n";
        //
        // src
        let expect = self.range_len().to_string();
        src = src +
            "   // error_msg: check range_len\n" +
            "   let mut error_msg = String::new()\n" +
            "   if range_len != " + &expect  + " {\n" +
            "       error_msg = \"range_len != " + &expect + "\";\n" +
            "   }\n";
        //
        // src
        src = src +
        "   // message\n" +
        "   let error_msg   = error_msg.as_bytes();\n" +
        "   let error_len   = std::cmp::min(error_msg.len(), message_len);\n" +
        "   for i in 0 .. error_len {\n" +
        "        message[i] = error_msg[i];\n" +
        "   }\n";
        //
        // src
        src = src +
            "   // return\n" +
            "   error_len\n" +
        "}\n";
        //
        src
    }
}
