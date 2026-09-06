//! Automatically rewritten from C to Rust
//! Source: lib/argv_split.c
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

#[no_mangle]
unsafe extern "C" fn count_argc(str: *const c_char) -> c_int {
    static int count_argc(const char *str)
    {
    let mut count: c_int = 0;
    bool was_space;
    for (was_space = true; *str; str++) {
    if (isspace(*str)) {
    was_space = true;
    } else if (was_space) {
    was_space = false;
    count++;
    }
    }
    return count;
    }
//
// argv_free - free an argv
// @argv: the argument vector to be freed
//
// Frees an argv and the strings it points to.
//
#[no_mangle]
pub unsafe extern "C" fn argv_free(argv: *mut c_char) {
    void argv_free(char **argv)
    {
    argv--;
    kfree(argv[0]);
    kfree(argv);
    }
    EXPORT_SYMBOL(argv_free);
//
// argv_split - split a string at whitespace, returning an argv
// @gfp: the GFP mask used to allocate memory
// @str: the string to be split
// @argcp: returned argument count
//
// Returns: an array of pointers to strings which are split out from
// @str.  This is performed by strictly splitting on white-space; no
// quote processing is performed.  Multiple whitespace characters are
// considered to be a single argument separator.  The returned array
// is always NULL-terminated.  Returns NULL on memory allocation
// failure.
//
// The source string at `str' may be undergoing concurrent alteration via
// userspace sysctl activity (at least).  The argv_split() implementation
// attempts to handle this gracefully by taking a local copy to work on.
//
    char **argv_split(gfp_t gfp, const char *str, int *argcp)
    {
    char *argv_str;
    bool was_space;
    char **argv, **argv_ret;
    int argc;
    argv_str = kstrndup(str, KMALLOC_MAX_SIZE - 1, gfp);
    if (!argv_str)
    return core::ptr::null_mut();
    argc = count_argc(argv_str);
    argv = kmalloc_array(argc + 2, sizeof(*argv), gfp);
    if (!argv) {
    kfree(argv_str);
    return core::ptr::null_mut();
    }
// argv = argv_str;
    argv_ret = ++argv;
    for (was_space = true; *argv_str; argv_str++) {
    if (isspace(*argv_str)) {
    was_space = true;
// argv_str = 0;
    } else if (was_space) {
    was_space = false;
// argv++ = argv_str;
    }
    }
// argv = NULL;
    if (argcp)
// argcp = argc;
    return argv_ret;
    }
    EXPORT_SYMBOL(argv_split);
