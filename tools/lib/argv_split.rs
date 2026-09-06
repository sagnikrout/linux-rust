//! Automatically rewritten from C to Rust
//! Source: tools/lib/argv_split.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// Helper function for splitting a string into an argv-like array.
//

    static const char *skip_arg(const char *cp)
    {
    while (*cp && !isspace(*cp))
    cp++;
    return cp;
    }
#[no_mangle]
unsafe extern "C" fn count_argc(str: *const c_char) -> c_int {
    static int count_argc(const char *str)
    {
    let mut count: c_int = 0;
    while (*str) {
    str = skip_spaces(str);
    if (*str) {
    count++;
    str = skip_arg(str);
    }
    }
    return count;
    }
//
// argv_free - free an argv
// @argv - the argument vector to be freed
//
// Frees an argv and the strings it points to.
//
#[no_mangle]
pub unsafe extern "C" fn argv_free(argv: *mut c_char) {
    void argv_free(char **argv)
    {
    char **p;
    for (p = argv; *p; p++) {
    free(*p);
// p = NULL;
    }
    free(argv);
    }
//
// argv_split - split a string at whitespace, returning an argv
// @str: the string to be split
// @argcp: returned argument count
//
// Returns an array of pointers to strings which are split out from
// @str.  This is performed by strictly splitting on white-space; no
// quote processing is performed.  Multiple whitespace characters are
// considered to be a single argument separator.  The returned array
// is always NULL-terminated.  Returns NULL on memory allocation
// failure.
//
    char **argv_split(const char *str, int *argcp)
    {
    let mut argc: c_int = count_argc(str);
    char **argv = calloc(argc + 1, sizeof(*argv));
    char **argvp;
    if (argv == core::ptr::null_mut())
    goto out;
    if (argcp)
// argcp = argc;
    argvp = argv;
    while (*str) {
    str = skip_spaces(str);
    if (*str) {
    const char *p = str;
    char *t;
    str = skip_arg(str);
    t = strndup(p, str-p);
    if (t == core::ptr::null_mut())
    goto fail;
// argvp++ = t;
    }
    }
// argvp = NULL;
    out:
    return argv;
    fail:
    argv_free(argv);
    return core::ptr::null_mut();
    }
