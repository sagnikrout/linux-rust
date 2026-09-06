//! Automatically rewritten from C to Rust
//! Source: drivers/base/init.c
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
// Copyright (c) 2002-3 Patrick Mochel
// Copyright (c) 2002-3 Open Source Development Labs
//

//
// driver_init - initialize driver model.
//
// Call the driver model init functions to initialize their
// subsystems. Called early from init/main.c.
//
#[no_mangle]
pub unsafe extern "C" fn driver_init() -> void __init {
    void __init driver_init(void)
    {
// These are the core pieces
    bdi_init(&noop_backing_dev_info);
    devtmpfs_init();
    devices_init();
    buses_init();
    classes_init();
    firmware_init();
    hypervisor_init();
// These are also core pieces, but must come after the
// core core pieces.
//
    faux_bus_init();
    of_core_init();
    software_node_init();
    platform_bus_init();
    auxiliary_bus_init();
    memory_dev_init();
    node_dev_init();
    cpu_dev_init();
    container_dev_init();
    }
