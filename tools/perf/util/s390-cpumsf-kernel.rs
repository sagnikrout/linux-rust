//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/s390-cpumsf-kernel.h
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
// Auxtrace support for s390 CPU measurement sampling facility
//
// Copyright IBM Corp. 2018
// Author(s): Hendrik Brueckner <brueckner@linux.ibm.com>
// Thomas Richter <tmricht@linux.ibm.com>
//

pub const S390_CPUMSF_DIAG_DEF_FIRST: c_uint = 0x8001	/* Diagnostic entry lowest id */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_basic_entry {
    pub /: *mut *mut unsigned int def:16; / 0-15 Data Entry Format,
    pub /: *mut *mut unsigned int R:4; / 16-19 reserved,
    pub /: *mut *mut unsigned int U:4; / 20-23 Number of unique instruct.,
    pub /: *mut *mut unsigned int z:2; / zeros,
    pub /: *mut *mut unsigned int T:1; / 26 PSW DAT mode,
    pub /: *mut *mut unsigned int W:1; / 27 PSW wait state,
    pub /: *mut *mut unsigned int P:1; / 28 PSW Problem state,
    pub /: *mut *mut unsigned int AS:2; / 29-30 PSW address-space control,
    pub /: *mut *mut unsigned int I:1; / 31 entry valid or invalid,
    pub /: *mut *mut unsigned int CL:2; / 32-33 Configuration Level,
    pub int:14: unsigned,
    pub /: *mut *mut unsigned int prim_asn:16; / primary ASN,
    pub /: *mut *mut unsigned long long ia; / Instruction Address,
    pub /: *mut *mut unsigned long long gpp; / Guest Program Parameter,
    pub /: *mut *mut unsigned long long hpp; / Host Program Parameter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_diag_entry {
    pub /: *mut *mut unsigned int def:16; / 0-15 Data Entry Format,
    pub /: *mut *mut unsigned int R:15; / 16-19 and 20-30 reserved,
    pub /: *mut *mut unsigned int I:1; / 31 entry valid or invalid,
    pub /: *mut *mut u8 data[]; / Machine-dependent sample data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_combined_entry {
    pub /: *mut *mut hws_basic_entry basic; / Basic-sampling data entry,
    pub /: *mut *mut hws_diag_entry diag; / Diagnostic-sampling data entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_trailer_entry {
    pub /: *mut *mut unsigned int f:1; / 0 - Block Full Indicator,
    pub /: *mut *mut unsigned int a:1; / 1 - Alert request control,
    pub /: *mut *mut unsigned int t:1; / 2 - Timestamp format,
    pub /: *mut *mut unsigned int:29; / 3 - 31: Reserved,
    pub /: *mut *mut unsigned int bsdes:16; / 32-47: size of basic SDE,
    pub /: *mut *mut unsigned int dsdes:16; / 48-63: size of diagnostic SDE,
}
