//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/arm-spe-decoder/arm-spe-pkt-decoder.h
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
// Arm Statistical Profiling Extensions (SPE) support
// Copyright (c) 2017-2018, Arm Ltd.
//

// Macro flag: #define INCLUDE__ARM_SPE_PKT_DECODER_H__

pub const ARM_SPE_PKT_DESC_MAX: c_int = 512;

pub const ARM_SPE_PKT_MAX_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_pkt_type {
    ARM_SPE_BAD,
    ARM_SPE_PAD,
    ARM_SPE_END,
    ARM_SPE_TIMESTAMP,
    ARM_SPE_ADDRESS,
    ARM_SPE_COUNTER,
    ARM_SPE_CONTEXT,
    ARM_SPE_OP_TYPE,
    ARM_SPE_EVENTS,
    ARM_SPE_DATA_SOURCE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_spe_pkt {
    pub type: arm_spe_pkt_type,
    pub index: c_uchar,
    pub payload: u64,
    pub midr: u64,
}

// Short header (HEADER0) and extended header (HEADER1)
pub const SPE_HEADER0_PAD: c_uint = 0x0;
pub const SPE_HEADER0_END: c_uint = 0x1;
pub const SPE_HEADER0_TIMESTAMP: c_uint = 0x71;
// Mask for event & data source

pub const SPE_HEADER0_EVENTS: c_uint = 0x42;
pub const SPE_HEADER0_SOURCE: c_uint = 0x43;
// Mask for context & operation

pub const SPE_HEADER0_CONTEXT: c_uint = 0x64;
pub const SPE_HEADER0_OP_TYPE: c_uint = 0x48;
// Mask for extended format
pub const SPE_HEADER0_EXTENDED: c_uint = 0x20;
// Mask for address & counter

pub const SPE_HEADER0_ADDRESS: c_uint = 0xb0;
pub const SPE_HEADER0_COUNTER: c_uint = 0x98;
pub const SPE_HEADER1_ALIGNMENT: c_uint = 0x0;

// Address packet header
pub const SPE_ADDR_PKT_HDR_INDEX_INS: c_uint = 0x0;
pub const SPE_ADDR_PKT_HDR_INDEX_BRANCH: c_uint = 0x1;
pub const SPE_ADDR_PKT_HDR_INDEX_DATA_VIRT: c_uint = 0x2;
pub const SPE_ADDR_PKT_HDR_INDEX_DATA_PHYS: c_uint = 0x3;
pub const SPE_ADDR_PKT_HDR_INDEX_PREV_BRANCH: c_uint = 0x4;
// Address packet payload
pub const SPE_ADDR_PKT_ADDR_BYTE7_SHIFT: c_int = 56;

pub const SPE_ADDR_PKT_EL0: c_int = 0;
pub const SPE_ADDR_PKT_EL1: c_int = 1;
pub const SPE_ADDR_PKT_EL2: c_int = 2;
pub const SPE_ADDR_PKT_EL3: c_int = 3;
// Context packet header

// Counter packet header
pub const SPE_CNT_PKT_HDR_INDEX_TOTAL_LAT: c_uint = 0x0;
pub const SPE_CNT_PKT_HDR_INDEX_ISSUE_LAT: c_uint = 0x1;
pub const SPE_CNT_PKT_HDR_INDEX_TRANS_LAT: c_uint = 0x2;
// Event packet payload
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_events {
    EV_EXCEPTION_GEN	= 0,
    EV_RETIRED		= 1,
    EV_L1D_ACCESS		= 2,
    EV_L1D_REFILL		= 3,
    EV_TLB_ACCESS		= 4,
    EV_TLB_WALK		= 5,
    EV_NOT_TAKEN		= 6,
    EV_MISPRED		= 7,
    EV_LLC_ACCESS		= 8,
    EV_LLC_MISS		= 9,
    EV_REMOTE_ACCESS	= 10,
    EV_ALIGNMENT		= 11,
    EV_TRANSACTIONAL	= 16,
    EV_PARTIAL_PREDICATE	= 17,
    EV_EMPTY_PREDICATE	= 18,
    EV_L2D_ACCESS		= 19,
    EV_L2D_MISS		= 20,
    EV_CACHE_DATA_MODIFIED	= 21,
    EV_RECENTLY_FETCHED	= 22,
    EV_DATA_SNOOPED		= 23,
    EV_STREAMING_SVE_MODE	= 24,
    EV_SMCU			= 25,
}

// Operation packet header

pub const SPE_OP_PKT_HDR_CLASS_OTHER: c_uint = 0x0;
pub const SPE_OP_PKT_HDR_CLASS_LD_ST_ATOMIC: c_uint = 0x1;
pub const SPE_OP_PKT_HDR_CLASS_BR_ERET: c_uint = 0x2;

//
// SME effective vector length or tile size (ETS) is stored in byte 0
// bits [6:4,2]; the length is rounded up to a power of two and use 128
// as one step, so ETS calculation is:
//
// 128 * (2 ^ bits [6:4,2]) = 32 << (bits [6:4,2])
//

//
// SVE effective vector length (EVL) is stored in byte 0 bits [6:4];
// the length is rounded up to a power of two and use 32 as one step,
// so EVL calculation is:
//
// 32 * (2 ^ bits [6:4]) = 32 << (bits [6:4])
//

extern "C" {
    pub fn arm_spe_pkt_desc(packet: *const arm_spe_pkt, buf: *mut c_char, len: usize) -> c_int;
}
