// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub module compiles and links to a dll library routines.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
///
use crate::adfn::rust_src::RustSrcFn;
///
// get_lib
/// Compile and link to a dll library.
///
/// * Syntax :
/// ```text
///     lib = dll_lib.get_lib(src_file, lib_file, replace_lib)
/// ```
///
/// * src_file :
/// is the name of the file that contains the source for the dll library.
/// This file need not exist if *lib_file* exists and *replace_lib* is false.
///
/// * lib_file :
/// is the name of the file that contains the dll library.
/// If this file does not exist, it will be created.
///
/// * replace_lib :
/// If this is true,
/// a new version of the dll library will be created even if it already exists.
///
/// * lib :
/// an object that can be used to call any of the public in the
/// library that are declared starting with:
/// ```text
///     #[no_mangle]
///     pub fn
/// ```
///
pub fn get_lib(
    src_file      : &str,
    lib_file      : &str,
    replace_lib   : bool,
) -> libloading::Library {
    //
    // lib_path
    let lib_path = std::path::Path::new(lib_file);
    if replace_lib {
        if lib_path.is_file() {
            let result = std::fs::remove_file(lib_file);
            if result.is_err() {
                panic!("dll_lib::get_lib: Cannot remove old library");
            }
        }
    }
    if ! lib_path.is_file() {
        //
        // cmd
        let mut cmd = String::from("rustc");
        cmd  = cmd + " " + src_file;
        cmd = cmd + " --crate-type dylib";
        cmd = cmd + " -o " + lib_file;
        //
        // lib_path
        let result = std::process::Command::new("bash")
            .arg( "-c" )
            .arg( &cmd )
            .output();
        if result.is_err() {
            panic!("dll_lib::get_lib: Cannot create library");
        }
        let stderr = result.unwrap().stderr;
        let stderr = String::from_utf8( stderr ).unwrap();
        if stderr != "" {
            eprint!("\ndll_lib::get_lib: can't compile and link library\n\n");
            eprint!("{}", stderr);
            panic!();
        }
    }
    //
    // lib
    let lib : libloading::Library;
    unsafe {
        let result = libloading::Library::new( lib_file );
        if result.is_err() {
            panic!("dll_lib::get_lib: Cannot load library");
        }
        lib  = result.unwrap();
    }
    lib
}
//
// RustSrcLink
/// This type is used for function like objects in a dll library.
///
/// If rust_src_fn is a `RustSrcLink<V>` object, it acts like a
/// [RustSrcFn] function.
pub type RustSrcLink<'a, V> = libloading::Symbol<'a, RustSrcFn<V> >;
//
// get_rust_src_fn
/// Get a link to an [RustSrcFn] function.
///
/// * lib :
/// is a library returned by the [get_lib] function.
///
/// * fn_name :
/// is the name of the function without it's leading `rust_src_` .
///
pub fn get_rust_src_fn<'a, V>(
    lib     : &'a libloading::Library,
    fn_name : &'a str,
) -> RustSrcLink<'a, V> {
    //
    // full_name
    let full_name = String::from("rust_src_") + fn_name;
    let full_name = full_name.as_bytes();
    //
    // rust_src_fn
    let rust_src_fn : RustSrcLink<V>;
    unsafe {
        let result = lib.get(full_name);
        if result.is_err() {
            let full_name = String::from("rust_src_") + fn_name;
            panic!("dll_lib::get_rust_src_fn: can't find {} in lib", full_name);
        }
        rust_src_fn = result.unwrap();
    }
    rust_src_fn
}
