//! Automatically rewritten from C to Rust
//! Source: tools/objtool/weak.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2020 Matt Helsley <mhelsley@vmware.com>
// Weak definitions necessary to compile objtool without
// some subcommands (e.g. check, orc).
//

    ({									\
    fprintf(stderr, "error: objtool: " name " not implemented\n");	\
    return ENOSYS;							\
    })
#[no_mangle]
pub unsafe extern "C" fn orc_dump(_objname: *const c_char) -> int __weak {
    int __weak orc_dump(const char *_objname)
    {
    UNSUPPORTED("ORC");
    }
#[no_mangle]
pub unsafe extern "C" fn orc_create(file: *mut objtool_file) -> int __weak {
    int __weak orc_create(struct objtool_file *file)
    {
    UNSUPPORTED("ORC");
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_klp(argc: c_int, argv: *const c_char) -> int __weak {
    int __weak cmd_klp(int argc, const char **argv)
    {
    UNSUPPORTED("klp");
    }
