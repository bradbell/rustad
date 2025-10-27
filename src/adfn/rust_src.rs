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
use crate::ADType;
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
        "#[no_mangle]\n" +
        "pub fn rust_src_"  + fn_name + "(\n" +
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
    V : ToString + From<f32> +  GlobalOpInfoVec ,
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
    /// The actual function name will be `rust_src_` followed by *fn_name* .
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
    ///     a vector containing the references to the domain variable values;
    ///     i.e., the independent variables.
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
            "       let msg  = \"On input: message.len() != 0\";\n" +
            "       *message = String::from(msg);\n" +
            "       return;\n" +
            "   }\n";
        //
        // check range
        src = src +
            "   // check range\n" +
            "   if range.len() != 0 {\n" +
            "       let msg  = \"On input: range.len() != 0\";\n" +
            "       *message = String::from(msg);\n" +
            "       return;\n" +
            "   }\n";
        //
        // check domain
        let expect = self.var.n_dom.to_string();
        src = src +
            "   // check domain\n" +
            "   if domain.len() != " + &expect  + " {\n" +
            "       let msg  = \"domain length != " + &expect + "\";\n" +
            "       *message = String::from(msg);\n" +
            "       return;\n" +
            "   }\n";
        //
        // V
        src = src +
            "   //\n" +
            "   // V\n" +
            "   type V = " + v_str + ";\n";
        //
        // nan
        src = src +
            "   //\n" +
            "   // nan\n" +
            "   let nan = V::from( f32::NAN );\n";
        //
        // con
        if 0 < self.cop.len() {
            let n_con = self.cop.len().to_string();
            src = src +
                "   // con\n" +
                "   let mut con : Vec<V> = " + "vec![nan; " + &n_con + "];\n";
            for i in 0 .. self.cop.len() {
                let i_str = i.to_string();
                let c_str = self.cop[i].to_string();
                src = src +
                    "   con[" + &i_str + "] = " + &c_str + " as V;\n";
            }
        }
        //
        // dep
        let n_dep = self.var.n_dep.to_string();
        src = src +
            "   //\n" +
            "   // dep\n" +
            "   // vector of dependent variables\n" +
            "   let mut dep : Vec<V> = vec![nan; " + &n_dep + "];\n";
        //
        // dep
        for op_index in 0 .. self.var.id_seq.len() {
            let op_id    = self.var.id_seq[op_index] as usize;
            let start    = self.var.arg_seq[op_index] as usize;
            let end      = self.var.arg_seq[op_index + 1] as usize;
            let arg      = &self.var.arg_all[start .. end];
            let res      = self.var.n_dom + op_index;
            let rust_src = op_info_vec[op_id].rust_src;
            let not_used = V::from( f32::NAN );
            src = src + &rust_src(
                    not_used, self.var.n_dom, &self.var.flag, &arg, res
                );
        }
        //
        // range
        let n_range = self.range2ad_type.len();
        src = src +
            "   //\n" +
            "   // range\n" +
            "   range.reserve(" + &n_range.to_string() + ");\n";
        for i in 0 .. n_range {
            let index = self.range2index[i] as usize;
            if self.range2ad_type[i] == ADType::Variable {
                if index < self.var.n_dom {
                    let i_str = index.to_string();
                    src = src +
                        "   range.push( domain[" + &i_str + "] );\n";
                } else {
                    let i_str = (index - self.var.n_dom).to_string();
                    src = src +
                        "   range.push( dep[" + &i_str + "] );\n";
                    }
            } else {
                let i_str = index.to_string();
                 src = src +
                    "   range.push( con[" + &i_str + "] );\n";
            }
        }
        //
        // end function body
        src = src + "}\n";
        //
        src
        }
    }
