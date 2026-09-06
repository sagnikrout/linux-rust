//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/sm_common.h
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
// Copyright © 2009 - Maxim Levitsky
// Common routines & support for SmartMedia/xD format
//

// Full oob structure as written on the flash
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm_oob {
    pub reserved: u32,
    pub data_status: u8,
    pub block_status: u8,
    pub lba_copy1: [u8; 2],
    pub ecc2: [u8; 3],
    pub lba_copy2: [u8; 2],
    pub ecc1: [u8; 3],
    pub __packed: },
// one sector is always 512 bytes, but it can consist of two nand pages
pub const SM_SECTOR_SIZE: c_int = 512;
// oob area is also 16 bytes, but might be from two pages
pub const SM_OOB_SIZE: c_int = 16;
// This is maximum zone size, and all devices that have more that one zone
pub const SM_MAX_ZONE_SIZE: c_int = 1024;
// support for small page nand
pub const SM_SMALL_PAGE: c_int = 256;
pub const SM_SMALL_OOB_SIZE: c_int = 8;
    pub smartmedia): *mut *mut int sm_register_device(struct mtd_info mtd, int,
    pub 5: return hweight16(oob->data_status) >=,
    pub 7: return hweight16(oob->block_status) >=,
    pub }: 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
// First test for erased block
    pub 1: return,
    pub 0: return,
