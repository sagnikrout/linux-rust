//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/google/coreboot_table.h
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
// coreboot_table.h
//
// Internal header for coreboot table access.
//
// Copyright 2014 Gerd Hoffmann <kraxel@redhat.com>
// Copyright 2017 Google Inc.
// Copyright 2017 Samuel Holland <samuel@sholland.org>
//

// A device, additionally with information from coreboot.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coreboot_device {
    pub dev: device,
    pub entry: coreboot_table_entry,
    pub cbmem_ref: lb_cbmem_ref,
    pub cbmem_entry: lb_cbmem_entry,
    pub framebuffer: lb_framebuffer,
    pub raw): DECLARE_FLEX_ARRAY(u8,,
}

extern "C" {
    pub fn container_of(_arg: dev, coreboot_device: struct, _arg: dev) -> return;
}
// A driver for handling devices described in coreboot tables.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coreboot_driver {
    pub ): *mut *mut int (probe)(struct coreboot_device,
    pub ): *mut *mut void (remove)(struct coreboot_device,
    pub drv: device_driver,
    pub id_table: *const coreboot_device_id,
}

// use a macro to avoid include chaining to get THIS_MODULE

// Register a driver that uses the data from a coreboot table.
// Unregister a driver that uses the data from a coreboot table.
extern "C" {
    pub fn coreboot_driver_unregister(driver: *mut coreboot_driver);
}
// module_coreboot_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

