//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/amd/ibs.h
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
// From PPR Vol 1 for AMD Family 19h Model 01h B1
// 55898 Rev 0.35 - Feb 5, 2021
//

// IBS_OP_DATA2 DataSrc
pub const IBS_DATA_SRC_LOC_CACHE: c_int = 2;
pub const IBS_DATA_SRC_DRAM: c_int = 3;
pub const IBS_DATA_SRC_REM_CACHE: c_int = 4;
pub const IBS_DATA_SRC_IO: c_int = 7;
// IBS_OP_DATA2 DataSrc Extension
pub const IBS_DATA_SRC_EXT_LOC_CACHE: c_int = 1;
pub const IBS_DATA_SRC_EXT_NEAR_CCX_CACHE: c_int = 2;
pub const IBS_DATA_SRC_EXT_DRAM: c_int = 3;
pub const IBS_DATA_SRC_EXT_FAR_CCX_CACHE: c_int = 5;
pub const IBS_DATA_SRC_EXT_PMEM: c_int = 6;
pub const IBS_DATA_SRC_EXT_IO: c_int = 7;
pub const IBS_DATA_SRC_EXT_EXT_MEM: c_int = 8;
pub const IBS_DATA_SRC_EXT_PEER_AGENT_MEM: c_int = 12;
//
// IBS Hardware MSRs
//
// MSR 0xc0011030: IBS Fetch Control
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_fetch_ctl {
    pub val: __u64,
// (needs IbsPhyAddrValid)
// (needs IbsFetchComp)
    pub /: *mut *mut reserved:2; / 62-63: reserved,
}

// MSR 0xc0011033: IBS Execution Control
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_ctl {
    pub val: __u64,
    pub /: *mut *mut ldlat_en:1; / 63: Load Latency enabled,
}

// MSR 0xc0011035: IBS Op Data 1
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_data {
    pub val: __u64,
    pub /: *mut *mut reserved2:23; / 41-63: reserved,
}

// MSR 0xc0011036: IBS Op Data 2
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_data2 {
    pub val: __u64,
    pub /: *mut *mut reserved1:54; / 10-63: reserved,
}

// MSR 0xc0011037: IBS Op Data 3
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_data3 {
    pub val: __u64,
    pub /: *mut *mut tlb_refill_lat:16; / 48-63: L1 TLB refill latency,
}

// MSR 0xc001103c: IBS Fetch Control Extended
#[repr(C)]
#[derive(Copy, Clone)]
pub union ic_ibs_extd_ctl {
    pub val: __u64,
    pub /: *mut *mut reserved:48; / 16-63: reserved,
}

//
// IBS driver related
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_ibs_data {
    pub size: u32,
    pub /: *mut *mut u32 data[0]; / data buffer starts here,
    pub caps: u32,
}
