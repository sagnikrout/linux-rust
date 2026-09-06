//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/intel-pt-decoder/intel-pt-pkt-decoder.h
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
// intel_pt_pkt_decoder.h: Intel Processor Trace support
// Copyright (c) 2013-2014, Intel Corporation.
//

// Macro flag: #define INCLUDE__INTEL_PT_PKT_DECODER_H__

pub const INTEL_PT_PKT_DESC_MAX: c_int = 256;

pub const INTEL_PT_PSB_LEN: c_int = 16;
pub const INTEL_PT_PKT_MAX_SZ: c_int = 16;
pub const INTEL_PT_VMX_NR_FLAG: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_pkt_type {
    INTEL_PT_BAD,
    INTEL_PT_PAD,
    INTEL_PT_TNT,
    INTEL_PT_TIP_PGD,
    INTEL_PT_TIP_PGE,
    INTEL_PT_TSC,
    INTEL_PT_TMA,
    INTEL_PT_MODE_EXEC,
    INTEL_PT_MODE_TSX,
    INTEL_PT_MTC,
    INTEL_PT_TIP,
    INTEL_PT_FUP,
    INTEL_PT_CYC,
    INTEL_PT_VMCS,
    INTEL_PT_PSB,
    INTEL_PT_PSBEND,
    INTEL_PT_CBR,
    INTEL_PT_TRACESTOP,
    INTEL_PT_PIP,
    INTEL_PT_OVF,
    INTEL_PT_MNT,
    INTEL_PT_PTWRITE,
    INTEL_PT_PTWRITE_IP,
    INTEL_PT_EXSTOP,
    INTEL_PT_EXSTOP_IP,
    INTEL_PT_MWAIT,
    INTEL_PT_PWRE,
    INTEL_PT_PWRX,
    INTEL_PT_BBP,
    INTEL_PT_BIP,
    INTEL_PT_BEP,
    INTEL_PT_BEP_IP,
    INTEL_PT_CFE,
    INTEL_PT_CFE_IP,
    INTEL_PT_EVD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_pkt {
    pub type: intel_pt_pkt_type,
    pub count: c_int,
    pub payload: u64,
}

//
// Decoding of BIP packets conflicts with single-byte TNT packets. Since BIP
// packets only occur in the context of a block (i.e. between BBP and BEP), that
// context must be recorded and passed to the packet decoder.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_pkt_ctx {
    INTEL_PT_NO_CTX,	/* BIP packets are invalid */
    INTEL_PT_BLK_4_CTX,	/* 4-byte BIP packets */
    INTEL_PT_BLK_8_CTX,	/* 8-byte BIP packets */
}

extern "C" {
    pub fn intel_pt_pkt_desc(packet: *const intel_pt_pkt, buf: *mut c_char, len: usize) -> c_int;
}
