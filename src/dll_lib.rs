// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This module compiles and links to a dll library routines.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
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
                panic!("load_rust_dll_lib: Cannot remove old library");
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
            panic!("load_rust_dll_lib: Cannot create library");
        }
        let stderr = result.unwrap().stderr;
        let stderr = String::from_utf8( stderr ).unwrap();
        if stderr != "" {
            eprint!("\nload_rust_dll_lib: can't compile and link library\n\n");
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
            panic!("load_rust_dll_lib: Cannot load library");
        }
        lib  = result.unwrap();
    }
    lib
}
