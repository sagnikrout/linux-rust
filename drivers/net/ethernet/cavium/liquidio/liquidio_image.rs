//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/liquidio_image.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//

pub const LIO_MAX_FIRMWARE_VERSION_LEN: c_int = 16;
pub const LIO_MAX_BOOTCMD_LEN: c_int = 1024;
pub const LIO_MAX_IMAGES: c_int = 16;
pub const LIO_NIC_MAGIC: c_uint = 0x434E4943     /* "CNIC" */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_firmware_desc {
    pub addr: __be64,
    pub len: __be32,
    pub /: *mut *mut __be32 crc32; / crc32 of image,
}

// Following the header is a list of 64-bit aligned binary images,
// as described by the desc field.
// Numeric fields are in network byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_firmware_file_header {
    pub magic: __be32,
    pub version: [c_char; LIO_MAX_FIRMWARE_VERSION_LEN],
    pub bootcmd: [c_char; LIO_MAX_BOOTCMD_LEN],
    pub num_images: __be32,
    pub desc: [octeon_firmware_desc; LIO_MAX_IMAGES],
    pub pad: __be32,
    pub /: *mut *mut __be32 crc32; / header checksum,
}
