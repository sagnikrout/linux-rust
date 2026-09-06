//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/bus.h
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
// linux/drivers/mmc/core/bus.h
//
// Copyright (C) 2003 Russell King, All Rights Reserved.
// Copyright 2007 Pierre Ossman
//

extern "C" {
    pub fn mmc_add_card(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_remove_card(card: *mut mmc_card);
}
extern "C" {
    pub fn mmc_register_bus() -> c_int;
}
extern "C" {
    pub fn mmc_unregister_bus();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_driver {
    pub drv: device_driver,
    pub card): *mut *mut int (probe)(struct mmc_card,
    pub card): *mut *mut void (remove)(struct mmc_card,
    pub card): *mut *mut void (shutdown)(struct mmc_card,
}

extern "C" {
    pub fn mmc_register_driver(drv: *mut mmc_driver) -> c_int;
}
extern "C" {
    pub fn mmc_unregister_driver(drv: *mut mmc_driver);
}
