//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/qinfo.h
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

// lpddr_private describes lpddr flash chip in memory map
// @ManufactId - Chip Manufacture ID
// @DevId - Chip Device ID
// @qinfo - pointer to qinfo records describing the chip
// @numchips - number of chips including virual RWW partitions
// @chipshift - Chip/partition size 2^chipshift
// @chips - per-chip data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr_private {
    pub ManufactId: u16,
    pub DevId: u16,
    pub qinfo: *mut qinfo_chip,
    pub numchips: c_int,
    pub chipshift: c_ulong,
    pub __counted_by(numchips): flchip chips[],
}

// qinfo_query_info structure contains request information for
// each qinfo record
// @major - major number of qinfo record
// @major - minor number of qinfo record
// @id_str - descriptive string to access the record
// @desc - detailed description for the qinfo record
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qinfo_query_info {
    pub major: u8,
    pub minor: u8,
    pub id_str: *mut c_char,
    pub desc: *mut c_char,
}

//
// qinfo_chip structure contains necessary qinfo records data
// @DevSizeShift - Device size 2^n bytes
// @BufSizeShift - Program buffer size 2^n bytes
// @TotalBlocksNum - Total number of blocks
// @UniformBlockSizeShift - Uniform block size 2^UniformBlockSizeShift bytes
// @HWPartsNum - Number of hardware partitions
// @SuspEraseSupp - Suspend erase supported
// @SingleWordProgTime - Single word program 2^SingleWordProgTime u-sec
// @ProgBufferTime - Program buffer write 2^ProgBufferTime u-sec
// @BlockEraseTime - Block erase 2^BlockEraseTime m-sec
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qinfo_chip {
// General device info
    pub DevSizeShift: u16,
    pub BufSizeShift: u16,
// Erase block information
    pub TotalBlocksNum: u16,
    pub UniformBlockSizeShift: u16,
// Partition information
    pub HWPartsNum: u16,
// Optional features
    pub SuspEraseSupp: u16,
// Operation typical time
    pub SingleWordProgTime: u16,
    pub ProgBufferTime: u16,
    pub BlockEraseTime: u16,
}

// defines for fixup usage
pub const LPDDR_MFR_ANY: c_uint = 0xffff;
pub const LPDDR_ID_ANY: c_uint = 0xffff;
pub const NUMONYX_MFGR_ID: c_uint = 0x0089;
pub const R18_DEVICE_ID_1G: c_uint = 0x893c;

