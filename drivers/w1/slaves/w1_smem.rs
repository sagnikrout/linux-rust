//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_smem.c
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
// w1_smem.c
//
// Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

pub const W1_FAMILY_SMEM_01: c_uint = 0x01;
pub const W1_FAMILY_SMEM_81: c_uint = 0x81;
    static struct w1_family w1_smem_family_01 = {
    .fid = W1_FAMILY_SMEM_01,
    };
    static struct w1_family w1_smem_family_81 = {
    .fid = W1_FAMILY_SMEM_81,
    };
#[no_mangle]
unsafe extern "C" fn w1_smem_init() -> int __init {
    static int __init w1_smem_init(void)
    {
    int err;
    err = w1_register_family(&w1_smem_family_01);
    if (err)
    return err;
    err = w1_register_family(&w1_smem_family_81);
    if (err) {
    w1_unregister_family(&w1_smem_family_01);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_smem_fini() -> void __exit {
    static void __exit w1_smem_fini(void)
    {
    w1_unregister_family(&w1_smem_family_01);
    w1_unregister_family(&w1_smem_family_81);
    }
    module_init(w1_smem_init);
    module_exit(w1_smem_fini);
    MODULE_AUTHOR("Evgeniy Polyakov <zbr@ioremap.net>");
    MODULE_DESCRIPTION("Driver for 1-wire Dallas network protocol, 64bit memory family.");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("w1-family-" __stringify(W1_FAMILY_SMEM_01));
    MODULE_ALIAS("w1-family-" __stringify(W1_FAMILY_SMEM_81));
