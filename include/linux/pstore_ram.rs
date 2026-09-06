//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pstore_ram.h
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
// Copyright (C) 2010 Marco Stornelli <marco.stornelli@gmail.com>
// Copyright (C) 2011 Kees Cook <keescook@chromium.org>
// Copyright (C) 2011 Google, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct persistent_ram_ecc_info {
    pub block_size: c_int,
    pub ecc_size: c_int,
    pub symsize: c_int,
    pub poly: c_int,
    pub par: *mut u16,
}

//
// Ramoops platform data
// @mem_size	memory size for ramoops
// @mem_address	physical memory address to contain ramoops
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ramoops_platform_data {
    pub mem_size: c_ulong,
    pub mem_address: phys_addr_t,
    pub mem_type: c_uint,
    pub record_size: c_ulong,
    pub console_size: c_ulong,
    pub ftrace_size: c_ulong,
    pub pmsg_size: c_ulong,
    pub max_reason: c_int,
    pub flags: u32,
    pub ecc_info: persistent_ram_ecc_info,
}
