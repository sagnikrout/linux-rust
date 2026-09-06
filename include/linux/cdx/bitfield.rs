//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cdx/bitfield.h
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
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

// Lowest bit numbers and widths
pub const CDX_DWORD_LBN: c_int = 0;
pub const CDX_DWORD_WIDTH: c_int = 32;
// Specified attribute (e.g. LBN) of the specified field

// Low bit number of the specified field

// Bit width of the specified field

// High bit number of the specified field

// A doubleword (i.e. 4 byte) datatype - little-endian in HW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_dword {
    pub cdx_u32: __le32,
}

// Value expanders for printk

//
// Extract bit field portion [low,high) from the 32-bit little-endian
// element which contains bits [min,max)
//

//
// Creates the portion of the named bit field that lies within the
// range [min,max).
//

//
// Creates the portion of the named bit fields that lie within the
// range [min,max).
//

// Populate a dword field with various numbers of arguments

