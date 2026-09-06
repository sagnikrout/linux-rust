//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/sharpsl.h
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
// SharpSL NAND support
//
// Copyright (C) 2008 Dmitry Baryshkov
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sharpsl_nand_platform_data {
    pub badblock_pattern: *mut nand_bbt_descr,
    pub ecc_layout: *const mtd_ooblayout_ops,
    pub partitions: *mut mtd_partition,
    pub nr_partitions: c_uint,
    pub part_parsers: *const *const c_char,
}
