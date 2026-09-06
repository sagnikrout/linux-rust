//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/cio_debugfs.c
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
// S/390 common I/O debugfs interface
//
// Copyright IBM Corp. 2021
// Author(s): Vineeth Vijayan <vneethv@linux.ibm.com>
//

    struct dentry *cio_debugfs_dir;
// Create the debugfs directory for CIO under the arch_debugfs_dir
// i.e /sys/kernel/debug/s390/cio
//
#[no_mangle]
unsafe extern "C" fn cio_debugfs_init() -> int __init {
    static int __init cio_debugfs_init(void)
    {
    cio_debugfs_dir = debugfs_create_dir("cio", arch_debugfs_dir);
    return 0;
    }
    subsys_initcall(cio_debugfs_init);
