//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/hmcdrv_mod.c
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
// HMC Drive DVD Module
//
// Copyright IBM Corp. 2013
// Author(s): Ralf Hoppe (rhoppe@de.ibm.com)
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Copyright 2013 IBM Corporation");
    MODULE_DESCRIPTION("HMC drive DVD access");
//
// module parameter 'cachesize'
//
    let mut hmcdrv_mod_cachesize: static size_t = HMCDRV_CACHE_SIZE_DFLT;
    module_param_named(cachesize, hmcdrv_mod_cachesize, ulong, S_IRUGO);
//
// hmcdrv_mod_init() - module init function
//
#[no_mangle]
unsafe extern "C" fn hmcdrv_mod_init() -> int __init {
    static int __init hmcdrv_mod_init(void)
    {
    int rc = hmcdrv_ftp_probe(); /* perform w/o cache */
    if (rc)
    return rc;
    rc = hmcdrv_cache_startup(hmcdrv_mod_cachesize);
    if (rc)
    return rc;
    rc = hmcdrv_dev_init();
    if (rc)
    hmcdrv_cache_shutdown();
    return rc;
    }
//
// hmcdrv_mod_exit() - module exit function
//
#[no_mangle]
unsafe extern "C" fn hmcdrv_mod_exit() -> void __exit {
    static void __exit hmcdrv_mod_exit(void)
    {
    hmcdrv_dev_exit();
    hmcdrv_cache_shutdown();
    }
    module_init(hmcdrv_mod_init);
    module_exit(hmcdrv_mod_exit);
