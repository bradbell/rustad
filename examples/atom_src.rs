// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
/*
Example converting an ADfn, that has an atomic function, call to source code.

sumsq_forward_fun;
z = g(x) = x[0] * x[0] + x[1] * x[1] + ...
*/
use rustad::{
    AzFloat,
    register_atom,
    AtomCallback,
    IndexT,
    start_recording,
    stop_recording,
    call_atom,
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
    create_src_dir
};
//
// V
type V = AzFloat<f64>;
//
// V_STR
const V_STR : &str = "AzFloat<f64>";
//
// sumsq_rev_depend
fn sumsq_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String {
    assert_eq!( depend.len(), 0 );
    let mut error_msg = String::new();
    if 0 < rng_index {
        error_msg += "sumsq_rev_depend: 0 < rng_index";
    } else {
        for j in 0 .. n_dom {
            depend.push( j );
        }
    }
    error_msg
}
//
// register_sumsq_atom
fn register_sumsq_atom()-> IndexT {
    //
    // sumsq_callback
    let sumsq_callback = AtomCallback {
        name                 : &"sumsq",
        rev_depend           :  Some( sumsq_rev_depend ),
        //
        forward_fun_value    :  Some( sumsq_forward_fun_value ),
        forward_fun_ad       :  None,
        //
        forward_der_value    :  None,
        forward_der_ad       :  None,
        //
        reverse_der_value    :  None,
        reverse_der_ad       :  None,
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_callback );
    sumsq_atom_id
}
//
// BEGIN atom_src
// sumsq_forward_fun_value
pub fn sumsq_forward_fun_value(
    _use_range   : &[bool]     ,
    domain       : &[&V]       ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{   //
    // sumsq_zero
    let mut sumsq_zero : V =  0.0.into();
    for j in 0 .. domain.len() {
        sumsq_zero += &( domain[j] * domain[j] );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_fun_value");
        print!("domain      = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
        }
        println!("]");
        println!("sumsq_zero = {}", sumsq_zero);
        println!("End Trace: sumsq_forward_fun_value");
    }
    Ok( vec![ sumsq_zero ] )
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
    let v_str   = V_STR;
    //
    // i_str
    let i_str   = std::any::type_name::<IndexT>();
    //
    // nx
    let nx = 3;
    //
    // f
    let x    = vec![ V::from(1.0) ; nx];
    let (_, ax) = start_recording(None, x);
    let ny   = 1;
    let ay   = call_atom(ny, ax, sumsq_atom_id, call_info, trace);
    let f    = stop_recording(ay);
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
    let atom_src = atom_src.replace("sumsq_forward_fun_value", "atom_sumsq");
    let atom_src = atom_src.replace("pub fn", "fn");
    let atom_src = atom_src.replace("IndexT", i_str);
    //
    let atom_src = atom_src.replace("<V>", "<v_str>");
    let atom_src = atom_src.replace("[&V]", "[&v_str]");
    let atom_src = atom_src.replace(": V =", ": v_str =");
    let atom_src = atom_src.replace("v_str", v_str);
    //
    // rust_src
    let fn_name   = "sumsq";
    let rust_src  = f.rust_src(fn_name);
    //
    // src_dir
    let src_dir   = "tmp/example_atom_src";
    let lib_src   = atom_src + &rust_src;
    create_src_dir(src_dir, &lib_src);
    //
    // lib
    let lib_file    = "tmp/example_atom_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_dir, lib_file, replace_lib);
    //
    // sumsq_fn
    let sumsq_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &fn_name);
    //
    // p_ref, x_ref
    let p_ref     : Vec<&V> = Vec::new();
    let x         : Vec<V>  = vec![ V::from(3.0); nx ];
    let mut x_ref : Vec<&V> = Vec::new();
    for x_j in x.iter() {
        x_ref.push( &x_j )
    }
    //
    // check result
    let result = sumsq_fn(&p_ref, &x_ref);
    let sumsq  = result.unwrap();
    //
    assert_eq!( sumsq.len(), 1 );
    assert_eq!( sumsq[0].to_inner(), (nx as f64) * 3.0 * 3.0 );
}
