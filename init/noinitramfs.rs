//! Automatically rewritten from C to Rust
//! Source: init/noinitramfs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// init/noinitramfs.c
//
// Copyright (C) 2006, NXP Semiconductors, All Rights Reserved
// Author: Jean-Paul Saman <jean-paul.saman@nxp.com>
//

//
// Create a simple rootfs that is similar to the default initramfs
//
#[no_mangle]
unsafe extern "C" fn default_rootfs() -> int __init {
    static int __init default_rootfs(void)
    {
    int err;
    usermodehelper_enable();
    err = init_mkdir("/dev", 0755);
    if (err < 0)
    goto out;
    err = init_mknod("/dev/console", S_IFCHR | S_IRUSR | S_IWUSR,
    new_encode_dev(MKDEV(5, 1)));
    if (err < 0)
    goto out;
    err = init_mkdir("/root", 0700);
    if (err < 0)
    goto out;
    return 0;
    out:
    printk(KERN_WARNING "Failed to create a rootfs\n");
    return err;
    }
    rootfs_initcall(default_rootfs);
