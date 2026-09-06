//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/common.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2020 Intel Corporation.
//

//
// This file contains defines, structures, etc. that are used
// to communicate between kernel and user code.
//
// version of protocol header (known to chip also). In the long run,
// we should be able to generate and accept a range of version numbers;
// for now we only accept one, and it's compiled in.
//
pub const IPS_PROTO_VERSION: c_int = 2;
//
// These are compile time constants that you may want to enable or disable
// if you are trying to debug problems with code or performance.
// HFI1_VERBOSE_TRACING define as 1 if you want additional tracing in
// fast path code
// HFI1_TRACE_REGWRITES define as 1 if you want register writes to be
// traced in fast path code
// _HFI1_TRACING define as 0 if you want to remove all tracing in a
// compilation unit
//
// driver/hw feature set bitmask
pub const HFI1_CAP_USER_SHIFT: c_int = 24;

// locked flag - if set, only HFI1_CAP_WRITABLE_MASK bits can be set
pub const HFI1_CAP_LOCKED_SHIFT: c_int = 63;
pub const HFI1_CAP_LOCKED_MASK: c_uint = 0x1ULL;

// extra bits used between kernel and user processes

//
// The set of capability bits that can be changed after initial load
// This set is the same for kernel and user contexts. However, for
// user contexts, the set can be further filtered by using the
// HFI1_CAP_RESERVED_MASK bits.
//

//
// A set of capability bits that are "global" and are not allowed to be
// set in the user bitmask.
//

//
// Set of capabilities that need to be enabled for kernel context in
// order to be allowed for user contexts, as well.
//

// Default enabled capabilities (both kernel and user)

//
// A bitmask of kernel/global capabilities that should be communicated
// to user level processes.
//

//
// The next set of defines are for packet headers, and chip register
// and memory bits that are visible to and/or used by user-mode software.
//
// Receive Header Flags
//
pub const RHF_PKT_LEN_SHIFT: c_int = 0;
pub const RHF_PKT_LEN_MASK: c_uint = 0xfffull;

pub const RHF_RCV_TYPE_SHIFT: c_int = 12;
pub const RHF_RCV_TYPE_MASK: c_uint = 0x7ull;

pub const RHF_USE_EGR_BFR_SHIFT: c_int = 15;
pub const RHF_USE_EGR_BFR_MASK: c_uint = 0x1ull;

pub const RHF_EGR_INDEX_SHIFT: c_int = 16;
pub const RHF_EGR_INDEX_MASK: c_uint = 0x7ffull;

pub const RHF_DC_INFO_SHIFT: c_int = 27;
pub const RHF_DC_INFO_MASK: c_uint = 0x1ull;

pub const RHF_RCV_SEQ_SHIFT: c_int = 28;
pub const RHF_RCV_SEQ_MASK: c_uint = 0xfull;

pub const RHF_EGR_OFFSET_SHIFT: c_int = 32;
pub const RHF_EGR_OFFSET_MASK: c_uint = 0xfffull;

pub const RHF_HDRQ_OFFSET_SHIFT: c_int = 44;
pub const RHF_HDRQ_OFFSET_MASK: c_uint = 0x1ffull;

pub const RHF_RCV_TYPE_ERR_SHIFT: c_int = 56;
pub const RHF_RCV_TYPE_ERR_MASK: c_uint = 0x7ul;

pub const RHF_ERROR_SMASK: c_uint = 0xffe0000000000000ull		/* bits 63:53 */;
// RHF receive types
pub const RHF_RCV_TYPE_EXPECTED: c_int = 0;
pub const RHF_RCV_TYPE_EAGER: c_int = 1;

pub const RHF_RCV_TYPE_ERROR: c_int = 3;
pub const RHF_RCV_TYPE_BYPASS: c_int = 4;
pub const RHF_RCV_TYPE_INVALID5: c_int = 5;
pub const RHF_RCV_TYPE_INVALID6: c_int = 6;
pub const RHF_RCV_TYPE_INVALID7: c_int = 7;
// RHF receive type error - expected packet errors
pub const RHF_RTE_EXPECTED_FLOW_SEQ_ERR: c_uint = 0x2;
pub const RHF_RTE_EXPECTED_FLOW_GEN_ERR: c_uint = 0x4;
// RHF receive type error - eager packet errors
pub const RHF_RTE_EAGER_NO_ERR: c_uint = 0x0;
// RHF receive type error - IB packet errors
pub const RHF_RTE_IB_NO_ERR: c_uint = 0x0;
// RHF receive type error - error packet errors
pub const RHF_RTE_ERROR_NO_ERR: c_uint = 0x0;
pub const RHF_RTE_ERROR_OP_CODE_ERR: c_uint = 0x1;
pub const RHF_RTE_ERROR_KHDR_MIN_LEN_ERR: c_uint = 0x2;
pub const RHF_RTE_ERROR_KHDR_HCRC_ERR: c_uint = 0x3;
pub const RHF_RTE_ERROR_KHDR_KVER_ERR: c_uint = 0x4;
pub const RHF_RTE_ERROR_CONTEXT_ERR: c_uint = 0x5;
pub const RHF_RTE_ERROR_KHDR_TID_ERR: c_uint = 0x6;
// RHF receive type error - bypass packet errors
pub const RHF_RTE_BYPASS_NO_ERR: c_uint = 0x0;
// MAX RcvSEQ
pub const RHF_MAX_SEQ: c_int = 13;
// IB - LRH header constants
pub const HFI1_LRH_GRH: c_uint = 0x0003      /* 1. word of IB LRH - next header: GRH */;
pub const HFI1_LRH_BTH: c_uint = 0x0002      /* 1. word of IB LRH - next header: BTH */;
// misc.
pub const SC15_PACKET: c_uint = 0xF;
pub const SIZE_OF_CRC: c_int = 1;
pub const SIZE_OF_LT: c_int = 1;

pub const LIM_MGMT_P_KEY: c_uint = 0x7FFF;
pub const FULL_MGMT_P_KEY: c_uint = 0xFFFF;

pub const HFI1_PSM_IOC_BASE_SEQ: c_uint = 0x0;
// Number of BTH.PSN bits used for sequence number in expected rcvs
pub const HFI1_KDETH_BTH_SEQ_SHIFT: c_int = 11;

extern "C" {
    pub fn __le64_to_cpu()rbuf): *mut *mut ((__le64) -> return;
}
// return size is in bytes, not DWORDs
// returned offset is in DWORDS
