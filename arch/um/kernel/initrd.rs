//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/initrd.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// Changed by uml_initrd_setup, which is a setup
    let mut __initdata: *mut static char initrd = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn read_initrd() -> int __init {
    int __init read_initrd(void)
    {
    unsigned long long size;
    void *area;
    if (!initrd)
    return 0;
    area = uml_load_file(initrd, &size);
    if (!area)
    return 0;
    initrd_start = (unsigned long) area;
    initrd_end = initrd_start + size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uml_initrd_setup(line: *mut c_char, add: *mut c_int) -> int __init {
    static int __init uml_initrd_setup(char *line, int *add)
    {
// add = 0;
    initrd = line;
    return 0;
    }
    __uml_setup("initrd=", uml_initrd_setup,
    "initrd=<initrd image>\n"
    "    This is used to boot UML from an initrd image.  The argument is the\n"
    "    name of the file containing the image.\n\n"
    );
