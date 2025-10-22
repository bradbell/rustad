// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
Example converting an ADfn, that has an atomic function, call to source code.

sumsq_forward_zero;
z = g(x) = x[0] * x[0] + x[1] * x[1] + ...
*/
use rustad::{
    register_atom,
    AtomEval,
    IndexT,
    start_recording,
    stop_recording,
    call_atom,
    get_lib,
    RustSrcFn,
    get_rust_src_fn,
};
//
// V
type V = f64;
//
// sumsq_forward_depend
fn sumsq_forward_depend(
    is_var_domain  : &Vec<bool> ,
    _call_info     : IndexT     ,
    _trace         : bool       ,
) -> Vec<bool>
{
    let mut is_var_range = false;
    for j in 0 .. is_var_domain.len() {
        is_var_range = is_var_range || is_var_domain[j];
    }
    vec![ is_var_range ]
}
//
// register_sumsq_atom
fn register_sumsq_atom()-> IndexT {
    //
    // sumsq_atom_eval
    let sumsq_atom_eval = AtomEval {
        name                 : &"sumsq",
        forward_depend       :  sumsq_forward_depend,
        //
        forward_zero_value   :  sumsq_forward_zero_value,
        forward_zero_ad      :  None,
        //
        forward_one_value    :  None,
        forward_one_ad       :  None,
        //
        reverse_one_value    :  None,
        reverse_one_ad       :  None,
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_atom_eval );
    sumsq_atom_id
}
//
// BEGIN atom_src
// sumsq_forward_zero_value
pub fn sumsq_forward_zero_value(
    domain_zero  : &Vec<&V>    ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // var_zero, sumsq_zero
    let mut sumsq_zero = 0 as V;
    for j in 0 .. domain_zero.len() {
        sumsq_zero += &( domain_zero[j] * domain_zero[j] );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_zero_value");
        print!("domain_zero = [ ");
        for j in 0 .. domain_zero.len() {
                print!("{}, ", domain_zero[j]);
        }
        println!("]");
        println!("sumsq_zero = {}", sumsq_zero);
        println!("End Trace: sumsq_forward_zero_value");
    }
    vec![ sumsq_zero ]
}
// END atom_src
// -------------------------------------------------------------------------
// main
// -------------------------------------------------------------------------
fn main() {
    let sumsq_atom_id  = register_sumsq_atom();
    let call_info      = 0;
    let trace          = false;
    //
    // v_str
    let v_str   = std::any::type_name::<V>();
    //
    // i_str
    let i_str   = std::any::type_name::<IndexT>();
    //
    // nx
    let nx = 3;
    //
    // f
    let x   : Vec<V> = vec![ 1.0; nx];
    let ax           = start_recording(x);
    let ay           = call_atom(ax, sumsq_atom_id, call_info, trace);
    let f            = stop_recording(ay);
    //
    // this_src
    let this_file = file!();
    let this_src  = std::fs::read_to_string(this_file).unwrap();
    //
    // atom_src
    let start    = this_src.find("// BEGIN atom_src\n").unwrap();
    let end      = this_src.find("// END atom_src\n").unwrap();
    let atom_src = String::from( &this_src[start .. end] );
    let atom_src = atom_src.replace("// BEGIN atom_src\n", "//\n");
    //
    // atom_src
    let atom_src = atom_src.replace("sumsq_forward_zero_value", "atom_sumsq");
    let atom_src = atom_src.replace("pub fn", "fn");
    let atom_src = atom_src.replace("IndexT", i_str);
    //
    let atom_src = atom_src.replace("<V>", "<v_str>");
    let atom_src = atom_src.replace("<&V>", "<&v_str>");
    let atom_src = atom_src.replace("as V", "as v_str");
    let atom_src = atom_src.replace("v_str", v_str);
    //
    // rust_src
    let fn_name   = "sumsq";
    let rust_src  = f.rust_src(fn_name);
    //
    // src_file
    let src_file  = "tmp/example_atom_src.rs";
    let src       = atom_src + &rust_src;
    let result = std::fs::write(src_file, src);
    if result.is_err() {
        panic!( "Cannot write {src_file}"  );
    }
    //
    // lib
    let lib_file    = "tmp/example_atom_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_file, lib_file, replace_lib);
    //
    // sumsq_fn
    let sumsq_fn : RustSrcFn<V> = get_rust_src_fn(&lib, &fn_name);
    //
    // x, y, msg
    let x         : Vec<V>  = vec![ 2.0 as V; nx ];
    let mut y     : Vec<V>  = Vec::new();
    let mut msg             = String::new();
    let mut x_ref : Vec<&V> = Vec::new();
    for xj in x.iter() {
        x_ref.push( &xj )
    }
    sumsq_fn(&x_ref, &mut y, &mut msg);
    //
    assert_eq!( y.len(), 1 );
    assert_eq!( y[0], (nx as V) * 4.0 );
}
