//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/intel-pt-decoder/intel-pt-insn-decoder.h
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
// intel_pt_insn_decoder.h: Intel Processor Trace support
// Copyright (c) 2013-2014, Intel Corporation.
//

// Macro flag: #define INCLUDE__INTEL_PT_INSN_DECODER_H__

pub const INTEL_PT_INSN_DESC_MAX: c_int = 32;
pub const INTEL_PT_INSN_BUF_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_insn_op {
    INTEL_PT_OP_OTHER,
    INTEL_PT_OP_CALL,
    INTEL_PT_OP_RET,
    INTEL_PT_OP_JCC,
    INTEL_PT_OP_JMP,
    INTEL_PT_OP_LOOP,
    INTEL_PT_OP_IRET,
    INTEL_PT_OP_INT,
    INTEL_PT_OP_SYSCALL,
    INTEL_PT_OP_SYSRET,
    INTEL_PT_OP_VMENTRY,
    INTEL_PT_OP_ERETS,
    INTEL_PT_OP_ERETU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_insn_branch {
    INTEL_PT_BR_NO_BRANCH,
    INTEL_PT_BR_INDIRECT,
    INTEL_PT_BR_CONDITIONAL,
    INTEL_PT_BR_UNCONDITIONAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_insn {
    pub op: intel_pt_insn_op,
    pub branch: intel_pt_insn_branch,
    pub emulated_ptwrite: bool,
    pub length: c_int,
    pub rel: i32,
    pub buf: [c_uchar; INTEL_PT_INSN_BUF_SZ],
}

extern "C" {
    pub fn intel_pt_insn_type(op: intel_pt_insn_op) -> c_int;
}
