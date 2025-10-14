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
use crate::op::info::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
// -----------------------------------------------------------------------
// prototype
fn prototype(fn_name : &str, v_str : &str) -> String {
    let result = String::new();
    let result = result +
        "rustad_src_"  + fn_name + "(\n" +
        "   domain      : &Vec<&"    + v_str  + ">,\n" +
        "   range       : &mut Vec<" + v_str  + ">,\n" +
        "   message     : &mut String,\n" +
        ")\n";
    result
}
//
// -----------------------------------------------------------------------
// rust_src
impl<V> ADfn<V>
where
    V : ToString + GlobalOpInfoVec ,
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
        // op_info_vec
        let op_info_vec = &*<V as GlobalOpInfoVec>::get();
        //
        // v_str
        let v_str   = type_name::<V>();
        //
        // prototype
        let mut src = prototype(fn_name, &v_str);
        //
        // begin function body
        src = src + "{\n";
        //
        // check message
        src = src +
            "   // check message\n" +
            "   if message.len() != 0 {\n" +
            "       message = \"On input: message.len() != 0\";\n" +
            "   }\n";
        //
        // check range
        src = src +
            "   // check range\n" +
            "   if range.len() != 0 {\n" +
            "       message = \"On input: range.len() != 0\";\n" +
            "   }\n";
        //
        // check domain
        let expect = self.n_domain.to_string();
        src = src +
            "   // check domain\n" +
            "   if domain.len() != " + &expect  + " {\n" +
            "       message = \"domain length != " + &expect + "\";\n" +
            "   }\n";
        //
        // con
        let n_con = self.con_all.len().to_string();
        src = src +
            "   // con\n" +
            "   let con : Vec<" + v_str + "> = " +
                    "Vec::with_capacity(" + &n_con + ");\n";
        for c in self.con_all.iter() {
            src = src +
                "   con.push(" + &( c.to_string() ) + " as " + v_str + ");\n";
        }
        //
        // var
        // Note that rust_src does not include the domain in the var vector
        assert!( self.n_domain <= self.n_var );
        let n_var = ( self.n_var - self.n_domain ).to_string();
        src = src +
            "   //\n" +
            "   // var\n" +
            "   let var : Vec<" + v_str + "> = " +
                    "Vec::with_capacity(" + &n_var + ");\n";
        //
        // var
        for op_index in 0 .. self.id_all.len() {
            let op_id = self.id_all[op_index] as usize;
            let start = self.op2arg[op_index] as usize;
            let end   = self.op2arg[op_index + 1] as usize;
            let arg   = &self.arg_all[start .. end];
            let res   = self.n_domain + op_index;
            let rust_src  = op_info_vec[op_id].rust_src;
            src = src + "   " +
                &rust_src(self.n_domain, &self.flag_all, &arg, res) + "\n";
        }
        //
        // end function body
        src = src + "}\n";
        //
        src
        }
    }
