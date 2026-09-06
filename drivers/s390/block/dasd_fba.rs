//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/block/dasd_fba.h
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2000
//
// Maximum number of blocks to be chained
//
pub const DASD_FBA_MAX_BLOCKS: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DE_fba_data {
    pub /: *mut *mut unsigned char perm:2; / Permissions on this extent,
    pub /: *mut *mut unsigned char zero:2; / Must be zero,
    pub /: *mut *mut unsigned char da:1; / usually zero,
    pub /: *mut *mut unsigned char diag:1; / allow diagnose,
    pub /: *mut *mut unsigned char zero2:2; / zero,
// C attribute field omitted
    pub /: *mut *mut __u8 zero; / Must be zero,
    pub /: *mut *mut __u16 blk_size; / Blocksize,
    pub /: *mut *mut __u32 ext_loc; / Extent locator,
    pub /: *mut *mut __u32 ext_beg; / logical number of block 0 in extent,
    pub /: *mut *mut __u32 ext_end; / logocal number of last block in extent,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LO_fba_data {
    pub zero:4: c_uchar,
    pub cmd:4: c_uchar,
// C attribute field omitted
    pub auxiliary: __u8,
    pub blk_ct: __u16,
    pub blk_nr: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_fba_characteristics {
    pub c: __u8,
    pub reserved:1: c_uchar,
    pub overrunnable:1: c_uchar,
    pub burst_byte:1: c_uchar,
    pub data_chain:1: c_uchar,
    pub zeros:4: c_uchar,
// C attribute field omitted
// C attribute field omitted
    pub c: __u8,
    pub zero0:1: c_uchar,
    pub removable:1: c_uchar,
    pub shared:1: c_uchar,
    pub zero1:1: c_uchar,
    pub mam:1: c_uchar,
    pub zeros:3: c_uchar,
// C attribute field omitted
// C attribute field omitted
    pub dev_class: __u8,
    pub unit_type: __u8,
    pub blk_size: __u16,
    pub blk_per_cycl: __u32,
    pub blk_per_bound: __u32,
    pub blk_bdsa: __u32,
    pub reserved0: __u32,
    pub reserved1: __u16,
    pub blk_ce: __u16,
    pub reserved2: __u32,
    pub reserved3: __u16,
// C attribute field omitted
