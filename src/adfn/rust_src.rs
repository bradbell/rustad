// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the ADfn [ADfn::rust_src] method
//! (rust source code for function value).
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
        "rustad_src_"  + fn_name + "(\n" +
        "   domain      : &Vec<&"    + type_name::<V>()  + ">,\n" +
        "   range       : &mut Vec<" + type_name::<V>()  + ">,\n" +
        "   message     : &mut String,\n" +
        ")\n";
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
    /// * return
    /// The return string contains the source code for the following function:
    /// <br/> `rust_src_` *fn_name* (
    /// <br/> `    domain      : &Vec<&V>,`
    /// <br/> `    range       : &mut Vec<V>,`
    /// <br/> `    message     : &mut String,`
    /// <br/> )
    ///
    ///     * domain :
    ///     is a vector containing the references to the domain variable values.
    ///
    ///     * range :
    ///     This vector must be empty on input.
    ///     Upon return it contains the range variable values
    ///     corresponding to the domain variable values.
    ///
    ///     * message :
    ///     This string must be empty on input.
    ///     If is empty upon return, no error was detected.
    ///     Otherwise it contains an error message.
    ///
    pub fn rust_src(&self, fn_name : &str) -> String {
        //
        // src
        let mut src    = prototype::<V>(fn_name) + "{\n";
        //
        // src
        src = src +
            "   // check message\n" +
            "   if message.len() != 0 {\n" +
            "       message = \"On input: message.len() != 0\";\n" +
            "   }\n";
        //
        // src
        src = src +
            "   // check range\n" +
            "   if range.len() != 0 {\n" +
            "       message = \"On input: range.len() != 0\";\n" +
            "   }\n";
        //
        // src
        let expect = self.domain_len().to_string();
        src = src +
            "   // check domain\n" +
            "   if domain.len() != " + &expect  + " {\n" +
            "       message = \"domain length != " + &expect + "\";\n" +
            "   }\n";
        //
        // src
        src = src +
            "}\n";
        //
        src
    }
}
