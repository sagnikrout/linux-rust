//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/isapnp.h
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
// ISA Plug & Play support
// Copyright (c) by Jaroslav Kysela <perex@suse.cz>
//

//

//

pub const DEVICE_COUNT_COMPATIBLE: c_int = 4;
pub const ISAPNP_CARD_DEVS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isapnp_card_id {
    pub /: *mut *mut unsigned long driver_data; / data private to the driver,
    pub card_device: unsigned short card_vendor,,
    pub function: unsigned short vendor,,
    pub /: *mut *mut } devs[ISAPNP_CARD_DEVS]; / logical devices,
}

// lowlevel configuration
extern "C" {
    pub fn isapnp_present() -> c_int;
}
extern "C" {
    pub fn isapnp_cfg_begin(csn: c_int, device: c_int) -> c_int;
}
extern "C" {
    pub fn isapnp_cfg_end() -> c_int;
}
extern "C" {
    pub fn isapnp_read_byte(idx: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn isapnp_write_byte(idx: c_uchar, val: c_uchar);
}

extern "C" {
    pub fn isapnp_proc_init() -> c_int;
}
extern "C" {
    pub fn isapnp_proc_done() -> c_int;
}

// compat

// lowlevel configuration

