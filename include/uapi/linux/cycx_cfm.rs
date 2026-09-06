//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cycx_cfm.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// cycx_cfm.h	Cyclom 2X WAN Link Driver.
// Definitions for the Cyclom 2X Firmware Module (CFM).
//
// Author:	Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//
// Copyright:	(c) 1998-2003 Arnaldo Carvalho de Melo
//
// Based on sdlasfm.h by Gene Kozin <74604.152@compuserve.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
// ============================================================================
// 1998/08/08	acme		Initial version.
//
// Defines
pub const CFM_VERSION: c_int = 2;

// min/max
pub const CFM_IMAGE_SIZE: c_uint = 0x20000	/* max size of CYCX code image file */;

pub const CFM_LOAD_BUFSZ: c_uint = 0x400	/* buffer size for reset code (buffer_load) */;
// Firmware Commands
pub const GEN_POWER_ON: c_uint = 0x1280;
pub const GEN_SET_SEG: c_uint = 0x1401	/* boot segment setting. */;
pub const GEN_BOOT_DAT: c_uint = 0x1402	/* boot data. */;
pub const GEN_START: c_uint = 0x1403	/* board start. */;
pub const GEN_DEFPAR: c_uint = 0x1404	/* buffer length for boot. */;
// Adapter Types
pub const CYCX_2X: c_int = 2;
// for now only the 2X is supported, no plans to support 8X or 16X
pub const CYCX_8X: c_int = 8;
pub const CYCX_16X: c_int = 16;
pub const CFID_X25_2X: c_int = 5200;
//
// struct cycx_fw_info - firmware module information.
// @codeid - firmware ID
// @version - firmware version number
// @adapter - compatible adapter types
// @memsize - minimum memory size
// @reserved - reserved
// @startoffs - entry point offset
// @winoffs - dual-port memory window offset
// @codeoffs - code load offset
// @codesize - code size
// @dataoffs - configuration data load offset
// @datasize - configuration data size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cycx_fw_info {
    pub codeid: c_ushort,
    pub version: c_ushort,
    pub adapter: [c_ushort; CFM_MAX_CYCX],
    pub memsize: c_ulong,
    pub reserved: [c_ushort; 2],
    pub startoffs: c_ushort,
    pub winoffs: c_ushort,
    pub codeoffs: c_ushort,
    pub codesize: c_ulong,
    pub dataoffs: c_ushort,
    pub datasize: c_ulong,
}

//
// struct cycx_firmware - CYCX firmware file structure
// @signature - CFM file signature
// @version - file format version
// @checksum - info + image
// @reserved - reserved
// @descr - description string
// @info - firmware module info
// @image - code image (variable size)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cycx_firmware {
    pub signature: [c_char; 80],
    pub version: c_ushort,
    pub checksum: c_ushort,
    pub reserved: [c_ushort; 6],
    pub descr: [c_char; CFM_DESCR_LEN],
    pub info: cycx_fw_info,
    pub image: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cycx_fw_header {
    pub reset_size: c_ulong,
    pub data_size: c_ulong,
    pub code_size: c_ulong,
}
