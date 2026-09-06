//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/firmware.c
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
// Extracted from cputable.c
//
// Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
//
// Modifications for ppc64:
// Copyright (C) 2003 Dave Engebretsen <engebret@us.ibm.com>
// Copyright (C) 2005 Stephen Rothwell, IBM Corporation
//

    unsigned long powerpc_firmware_features __read_mostly;
    EXPORT_SYMBOL_GPL(powerpc_firmware_features);

    DEFINE_STATIC_KEY_FALSE(kvm_guest);
    EXPORT_SYMBOL_GPL(kvm_guest);
#[no_mangle]
pub unsafe extern "C" fn check_kvm_guest() -> int __init {
    int __init check_kvm_guest(void)
    {
    struct device_node *hyper_node;
    hyper_node = of_find_node_by_path("/hypervisor");
    if (!hyper_node)
    return 0;
    if (of_device_is_compatible(hyper_node, "linux,kvm"))
    static_branch_enable(&kvm_guest);
    of_node_put(hyper_node);
    return 0;
    }
    core_initcall(check_kvm_guest); // before kvm_guest_init()
