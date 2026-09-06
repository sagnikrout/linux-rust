//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/umid.c
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
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// Changed by set_umid_arg
    static int umid_inited;
#[no_mangle]
unsafe extern "C" fn set_umid_arg(name: *mut c_char, add: *mut c_int) -> int __init {
    static int __init set_umid_arg(char *name, int *add)
    {
    int err;
    if (umid_inited) {
    os_warn("umid already set\n");
    return 0;
    }
// add = 0;
    err = set_umid(name);
    if (err == -EEXIST)
    os_warn("umid '%s' already in use\n", name);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !err) -> else {
    else if (!err)
    umid_inited = 1;
    return 0;
    }
    __uml_setup("umid=", set_umid_arg,
    "umid=<name>\n"
    "    This is used to assign a unique identity to this UML machine and\n"
    "    is used for naming the pid file and management console socket.\n\n"
    );
