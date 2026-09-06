//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/dcr.c
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
// (c) Copyright 2006 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//

    unsigned int dcr_resource_start(const struct device_node *np,
    unsigned int index)
    {
    unsigned int ds;
    const u32 *dr = of_get_property(np, "dcr-reg", &ds);
    if (dr == core::ptr::null_mut() || ds & 1 || index >= (ds / 8))
    return 0;
    return dr[index * 2];
    }
    EXPORT_SYMBOL_GPL(dcr_resource_start);
#[no_mangle]
pub unsafe extern "C" fn dcr_resource_len(np: *const device_node, index: c_uint) -> c_uint {
    unsigned int dcr_resource_len(const struct device_node *np, unsigned int index)
    {
    unsigned int ds;
    const u32 *dr = of_get_property(np, "dcr-reg", &ds);
    if (dr == core::ptr::null_mut() || ds & 1 || index >= (ds / 8))
    return 0;
    return dr[index * 2 + 1];
    }
    EXPORT_SYMBOL_GPL(dcr_resource_len);
    DEFINE_SPINLOCK(dcr_ind_lock);
    EXPORT_SYMBOL_GPL(dcr_ind_lock);
