//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/physmem_info.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum physmem_info_source {
    MEM_DETECT_NONE = 0,
    MEM_DETECT_SCLP_STOR_INFO,
    MEM_DETECT_DIAG260,
    MEM_DETECT_DIAG500_STOR_LIMIT,
    MEM_DETECT_SCLP_READ_INFO,
    MEM_DETECT_BIN_SEARCH
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct physmem_range {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reserved_range_type {
    RR_DECOMPRESSOR,
    RR_INITRD,
    RR_VMLINUX,
    RR_AMODE31,
    RR_IPLREPORT,
    RR_CERT_COMP_LIST,
    RR_MEM_DETECT_EXT,
    RR_VMEM,
    RR_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reserved_range {
    pub start: c_ulong,
    pub end: c_ulong,
    pub chain: *mut reserved_range,
}

//
// Storage element id is defined as 1 byte (up to 256 storage elements).
// In practise only storage element id 0 and 1 are used).
// According to architecture one storage element could have as much as
// 1020 subincrements. 255 physmem_ranges are embedded in physmem_info.
// If more physmem_ranges are required, a block of memory from already
// known physmem_range is taken (online_extended points to it).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct physmem_info {
    pub range_count: u32,
    pub info_source: u8,
    pub usable: c_ulong,
    pub reserved: [reserved_range; RR_MAX],
    pub online: [physmem_range; MEM_INLINED_ENTRIES],
    pub online_extended: *mut physmem_range,
}

extern "C" {
    pub fn add_physmem_online_range(start: u64, end: u64);
}
// start = 0;
// end = 0;
// start = (unsigned long)physmem_info.online[n].start;
// end = (unsigned long)physmem_info.online[n].end;
// start = (unsigned long)physmem_info.online_extended[n - MEM_INLINED_ENTRIES].start;
// end = (unsigned long)physmem_info.online_extended[n - MEM_INLINED_ENTRIES].end;
// end = physmem_info.usable;
//
// for_each_physmem_usable_range - early online memory range iterator
// @i: an integer used as loop variable
// @p_start: ptr to unsigned long for start address of the range
// @p_end: ptr to unsigned long for end address of the range
//
// Walks over detected online memory ranges below usable limit.
//

// Walks over all detected online memory ranges disregarding usable limit.

// p_start = range ? range->start : 0, *p_end = range ? range->end : 0)
extern "C" {
    pub fn __va(_arg: range->chain) -> return;
}

// p_start = range ? range->start : 0, *p_end = range ? range->end : 0;	\
// p_start = range ? range->start : 0, *p_end = range ? range->end : 0)
// addr = physmem_info.reserved[type].start;
// size = physmem_info.reserved[type].end - physmem_info.reserved[type].start;

