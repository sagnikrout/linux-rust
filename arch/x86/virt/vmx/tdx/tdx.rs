//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/virt/vmx/tdx/tdx.h
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
// This file contains both macros and data structures defined by the TDX
// architecture and Linux defined software data structures and functions.
// The two should not be mixed together for better readability.  The
// architectural definitions come first.
//
// TDX module SEAMCALL leaf functions
//
pub const TDH_VP_ENTER: c_int = 0;
pub const TDH_MNG_ADDCX: c_int = 1;
pub const TDH_MEM_PAGE_ADD: c_int = 2;
pub const TDH_MEM_SEPT_ADD: c_int = 3;
pub const TDH_VP_ADDCX: c_int = 4;
pub const TDH_MEM_PAGE_AUG: c_int = 6;
pub const TDH_MEM_RANGE_BLOCK: c_int = 7;
pub const TDH_MNG_KEY_CONFIG: c_int = 8;
pub const TDH_MNG_CREATE: c_int = 9;
pub const TDH_MNG_RD: c_int = 11;
pub const TDH_MR_EXTEND: c_int = 16;
pub const TDH_MR_FINALIZE: c_int = 17;
pub const TDH_VP_FLUSH: c_int = 18;
pub const TDH_MNG_VPFLUSHDONE: c_int = 19;
pub const TDH_VP_CREATE: c_int = 10;
pub const TDH_MNG_KEY_FREEID: c_int = 20;
pub const TDH_MNG_INIT: c_int = 21;
pub const TDH_VP_INIT: c_int = 22;
pub const TDH_PHYMEM_PAGE_RDMD: c_int = 24;
pub const TDH_VP_RD: c_int = 26;
pub const TDH_PHYMEM_PAGE_RECLAIM: c_int = 28;
pub const TDH_MEM_PAGE_REMOVE: c_int = 29;
pub const TDH_SYS_KEY_CONFIG: c_int = 31;
pub const TDH_SYS_INIT: c_int = 33;
pub const TDH_SYS_RD: c_int = 34;
pub const TDH_SYS_LP_INIT: c_int = 35;
pub const TDH_SYS_TDMR_INIT: c_int = 36;
pub const TDH_MEM_TRACK: c_int = 38;
pub const TDH_PHYMEM_CACHE_WB: c_int = 40;
pub const TDH_PHYMEM_PAGE_WBINVD: c_int = 41;
pub const TDH_VP_WR: c_int = 43;
pub const TDH_SYS_CONFIG: c_int = 45;
pub const TDH_SYS_SHUTDOWN: c_int = 52;
pub const TDH_SYS_UPDATE: c_int = 53;
pub const TDH_SYS_DISABLE: c_int = 69;
//
// SEAMCALL leaf:
//
// Bit 15:0	Leaf number
// Bit 23:16	Version number
//
pub const TDX_VERSION_SHIFT: c_int = 16;
// TDX page types
pub const PT_NDA: c_uint = 0x0;
pub const PT_RSVD: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdmr_reserved_area {
    pub offset: u64,
    pub size: u64,
    pub __packed: },
pub const TDMR_INFO_ALIGNMENT: c_int = 512;
pub const TDMR_INFO_PA_ARRAY_ALIGNMENT: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdmr_info {
    pub base: u64,
    pub size: u64,
    pub pamt_1g_base: u64,
    pub pamt_1g_size: u64,
    pub pamt_2m_base: u64,
    pub pamt_2m_size: u64,
    pub pamt_4k_base: u64,
    pub pamt_4k_size: u64,
//
// The actual number of reserved areas depends on the value of
// field MD_FIELD_ID_MAX_RESERVED_PER_TDMR in the TDX module
// global metadata.
//
    pub reserved_areas): DECLARE_FLEX_ARRAY(struct tdmr_reserved_area,,
    pub __aligned(TDMR_INFO_ALIGNMENT): } __packed,
//
// Do not put any hardware-defined TDX structure representations below
// this comment!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_memblock {
    pub list: list_head,
    pub start_pfn: c_ulong,
    pub end_pfn: c_ulong,
    pub nid: c_int,
}

// Warn if kernel has less than TDMR_NR_WARN TDMRs after allocation
pub const TDMR_NR_WARN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdmr_info_list {
    pub /: *mut *mut *mut void tdmrs; / Flexible array to hold 'tdmr_info's,
    pub /: *mut *mut int nr_consumed_tdmrs; / How many 'tdmr_info's are in use,
// Metadata for finding target 'tdmr_info' and freeing @tdmrs
    pub /: *mut *mut int tdmr_sz; / Size of one 'tdmr_info',
    pub /: *mut *mut int max_tdmrs; / How many 'tdmr_info's are allocated,
}

extern "C" {
    pub fn tdx_module_shutdown() -> c_int;
}
extern "C" {
    pub fn tdx_module_run_update() -> c_int;
}
