//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ocelot.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright 2022 Innovative Advantage Inc.

//
// Don't use _get_and_ioremap_resource() here, since that will invoke
// prints of "invalid resource" which will simply add confusion.
//
extern "C" {
    pub fn ERR_CAST(_arg: regs) -> return;
}
extern "C" {
    pub fn devm_regmap_init_mmio(_arg: dev, _arg: regs, _arg: config) -> return;
}
//
// Fall back to using REG and getting the resource from the parent
// device, which is possible in an MFD configuration
//
extern "C" {
    pub fn dev_get_regmap(_arg: dev->parent, _arg: res->name) -> return;
}
