//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/cros_ec_chardev.h
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
// ChromeOS EC device interface.
//
// Copyright (C) 2014 Google, Inc.
//

//
// struct cros_ec_readmem - Struct used to read mapped memory.
// @offset: Within EC_LPC_ADDR_MEMMAP region.
// @bytes: Number of bytes to read. Zero means "read a string" (including '\0')
// At most only EC_MEMMAP_SIZE bytes can be read.
// @buffer: Where to store the result. The ioctl returns the number of bytes
// read or negative on error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_readmem {
    pub offset: u32,
    pub bytes: u32,
    pub buffer: [u8; EC_MEMMAP_SIZE],
}

pub const CROS_EC_DEV_IOC: c_uint = 0xEC;

