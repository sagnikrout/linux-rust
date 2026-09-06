//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/host1x/context_bus.c
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
// Copyright (c) 2021, NVIDIA Corporation.
//

    const struct bus_type host1x_context_device_bus_type = {
    .name = "host1x-context",
    };
    EXPORT_SYMBOL_GPL(host1x_context_device_bus_type);
#[no_mangle]
unsafe extern "C" fn host1x_context_device_bus_init() -> int __init {
    static int __init host1x_context_device_bus_init(void)
    {
    int err;
    err = bus_register(&host1x_context_device_bus_type);
    if (err < 0) {
    pr_err("bus type registration failed: %d\n", err);
    return err;
    }
    return 0;
    }
    postcore_initcall(host1x_context_device_bus_init);
