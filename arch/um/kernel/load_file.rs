//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/load_file.c
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

#[no_mangle]
unsafe extern "C" fn __uml_load_file(filename: *const c_char, buf: *mut c_void, size: c_int) -> int __init {
    static int __init __uml_load_file(const char *filename, void *buf, int size)
    {
    int fd, n;
    fd = os_open_file(filename, of_read(OPENFLAGS()), 0);
    if (fd < 0) {
    printk(KERN_ERR "Opening '%s' failed - err = %d\n", filename,
    -fd);
    return -1;
    }
    n = os_read_file(fd, buf, size);
    if (n != size) {
    printk(KERN_ERR "Read of %d bytes from '%s' failed, "
    "err = %d\n", size,
    filename, -n);
    return -1;
    }
    os_close_file(fd);
    return 0;
    }
    void *uml_load_file(const char *filename, unsigned long long *size)
    {
    void *area;
    int err;
// size = 0;
    if (!filename)
    return core::ptr::null_mut();
    err = os_file_size(filename, size);
    if (err)
    return core::ptr::null_mut();
    if (*size == 0) {
    printk(KERN_ERR "\"%s\" is empty\n", filename);
    return core::ptr::null_mut();
    }
    area = memblock_alloc_or_panic(*size, SMP_CACHE_BYTES);
    if (__uml_load_file(filename, area, *size)) {
    memblock_free(area, *size);
    return core::ptr::null_mut();
    }
    return area;
    }
