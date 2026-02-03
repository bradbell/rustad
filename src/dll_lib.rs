// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub module compiles and links to a dll library routines.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
use crate::adfn::rust_src::RustSrcFn;
// ----------------------------------------------------------------------------
// write_src_file
fn write_src_file(src_dir : &str, name : &str, source : &str) {
    let src_file  = src_dir.to_string() + "/" + name;
    let result    = std::fs::write(src_file.clone(), source);
    if result.is_err() {
        panic!( "Cannot write {src_file}"  );
    }
}
// ----------------------------------------------------------------------------
// create_src_dir
/// Create a get_lib source code directory
///
/// * src_dir  :
///   is the directory we are creating. If it already exists, any files
///   there are left in place except for:
///   lib.rs, core.rs, cmp_as.rs, az_float.rs.
///
/// * lib_src :
///   is an in memory representation of the data that is written to the
///   file *src_dir* `/lib.rs` .
///
/// * Other Files :
///   core.rs, cmp_as.rs, az_float.rs
///   are copies of the corresponding rustad files in src/float.
///
pub fn create_src_dir(
    src_dir  :  &str ,
    lib_src  :  &str ,
) {
    let result    = std::fs::create_dir_all(src_dir);
    if result.is_err() { panic!(
        "dll_lib::create_src_dir: Cannot create the directory {}", src_dir
    ); }
    write_src_file(src_dir, "lib.rs", lib_src);
    write_src_file(src_dir, "core.rs",     crate::CORE_RS);
    write_src_file(src_dir, "cmp_as.rs",   crate::CMP_AS_RS);
    write_src_file(src_dir, "az_float.rs", crate::AZ_FLOAT_RS);
    write_src_file(src_dir, "num_vec.rs",  crate::NUM_VEC_RS);
}

// ----------------------------------------------------------------------------
// get_lib
/// Compile and link to a dll library.
///
/// * Syntax :
///   ```text
///     lib = dll_lib.get_lib(src_dir, lib_file, replace_lib)
///   ```
///
/// * src_dir :
///   is the name of the directory that contains the source for the dll library.
///   If *lib_file* exists and *replace_lib* is false, *src_dir* need not exist.
///   If it does exist, the top level source code file for the library must be
///   *src_dir* `/lib.rs` .
///
/// * lib_file :
///   is the name of the file that contains the dll library.
///   If this file does not exist, it will be created.
///
/// * replace_lib :
///   If this is true,
///   a new version of the dll library will be created even if it already exists.
///
/// * lib :
///   an object that can be used to call any of the public in the
///   library that are declared starting with:
///   ```text
///     #[no_mangle]
///     pub fn
///   ```
///
pub fn get_lib(
    src_dir       : &str,
    lib_file      : &str,
    replace_lib   : bool,
) -> libloading::Library {
    //
    // lib_path
    let lib_path = std::path::Path::new(lib_file);
    if replace_lib && lib_path.is_file() {
        let result = std::fs::remove_file(lib_file);
        if result.is_err() {
            panic!("dll_lib::get_lib: Cannot remove old library");
        }
    }
    if ! lib_path.is_file() {
        //
        // src_file
        let src_file = src_dir.to_string() + "/lib.rs";
        let src_path = std::path::Path::new(&src_file);
        if ! src_path.is_file() { panic!(
            "dll_lib::get_lib: Cannot find lib.rs in src_dir = {}", src_dir
        ); }
        //
        // cmd
        let mut cmd = String::from("rustc");
        cmd  = cmd + " " + &src_file;
        cmd += " --crate-type dylib";
        cmd  = cmd + " -o " + lib_file;
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
        if ! stderr.is_empty() {
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
// ----------------------------------------------------------------------------
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
///   is a library returned by the [get_lib] function.
///
/// * fn_name :
///   is the name of the function without it's leading `rust_src_` .
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
