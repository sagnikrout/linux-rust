//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/include/reg.h
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
// Copyright 2014, Michael Ellerman, IBM Corp.
//

pub const SPRN_MMCR2: c_int = 769;
pub const SPRN_MMCRA: c_int = 770;
pub const SPRN_MMCR0: c_int = 779;
pub const MMCR0_PMAO: c_uint = 0x00000080;
pub const MMCR0_PMAE: c_uint = 0x04000000;
pub const MMCR0_FC: c_uint = 0x80000000;
pub const SPRN_EBBHR: c_int = 804;
pub const SPRN_EBBRR: c_int = 805;

pub const BESCR_PMEO: c_uint = 0x1     /* PMU Event-based exception Occurred */;

pub const SPRN_PMC1: c_int = 771;
pub const SPRN_PMC2: c_int = 772;
pub const SPRN_PMC3: c_int = 773;
pub const SPRN_PMC4: c_int = 774;
pub const SPRN_PMC5: c_int = 775;
pub const SPRN_PMC6: c_int = 776;
pub const SPRN_SIAR: c_int = 780;
pub const SPRN_SDAR: c_int = 781;
pub const SPRN_SIER: c_int = 768;

pub const SPRN_TEXASR: c_uint = 0x82    /* Transaction Exception and Status Register */;
pub const SPRN_TFIAR: c_uint = 0x81    /* Transaction Failure Inst Addr    */;
pub const SPRN_TFHAR: c_uint = 0x80    /* Transaction Failure Handler Addr */;
pub const SPRN_TAR: c_uint = 0x32f	/* Target Address Register */;

pub const SPRN_PVR: c_uint = 0x11F;

pub const SPRN_DSCR_PRIV: c_uint = 0x11	/* Privilege State DSCR */;
pub const SPRN_DSCR: c_uint = 0x03	/* Data Stream Control Register */;

// TEXASR register bits
pub const TEXASR_FC: c_uint = 0xFE00000000000000;
pub const TEXASR_FP: c_uint = 0x0100000000000000;
pub const TEXASR_DA: c_uint = 0x0080000000000000;
pub const TEXASR_NO: c_uint = 0x0040000000000000;
pub const TEXASR_FO: c_uint = 0x0020000000000000;
pub const TEXASR_SIC: c_uint = 0x0010000000000000;
pub const TEXASR_NTC: c_uint = 0x0008000000000000;
pub const TEXASR_TC: c_uint = 0x0004000000000000;
pub const TEXASR_TIC: c_uint = 0x0002000000000000;
pub const TEXASR_IC: c_uint = 0x0001000000000000;
pub const TEXASR_IFC: c_uint = 0x0000800000000000;
pub const TEXASR_ABT: c_uint = 0x0000000100000000;
pub const TEXASR_SPD: c_uint = 0x0000000080000000;
pub const TEXASR_HV: c_uint = 0x0000000020000000;
pub const TEXASR_PR: c_uint = 0x0000000010000000;
pub const TEXASR_FS: c_uint = 0x0000000008000000;
pub const TEXASR_TE: c_uint = 0x0000000004000000;
pub const TEXASR_ROT: c_uint = 0x0000000002000000;
// MSR register bits

// macro to check TM MSR bits

// Vector Instructions

extern "C" {
    pub fn store_gpr(addr: *mut c_ulong);
}
extern "C" {
    pub fn load_gpr(addr: *mut c_ulong);
}
extern "C" {
    pub fn store_fpr(addr: *mut double);
}

