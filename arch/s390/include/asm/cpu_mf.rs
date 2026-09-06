//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/cpu_mf.h
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
// CPU-measurement facilities
//
// Copyright IBM Corp. 2012, 2018
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
// Jan Glauber <jang@linux.vnet.ibm.com>
//

pub const CPU_MF_SF_RIBM_NOTAV: c_uint = 0x1		/* Sampling unavailable */;
// CPU measurement facility support
extern "C" {
    pub fn test_facility(test_facility(67: 40) &&) -> return;
}
extern "C" {
    pub fn test_facility(test_facility(68: 40) &&) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpumf_ctr_info {
    pub cfvn: u16,
    pub auth_ctl: u16,
    pub enable_ctl: u16,
    pub act_ctl: u16,
    pub max_cpu: u16,
    pub csvn: u16,
    pub max_cg: u16,
    pub reserved1: u16,
    pub reserved2: [u32; 12],
    pub __packed: },
// QUERY SAMPLING INFORMATION block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_qsi_info_block {
    pub /: *mut *mut unsigned int b0_13:14; / 0-13: zeros,
    pub /: *mut *mut unsigned int as:1; / 14: basic-sampling authorization,
    pub /: *mut *mut unsigned int ad:1; / 15: diag-sampling authorization,
    pub /: *mut *mut unsigned int b16_21:6; / 16-21: zeros,
    pub /: *mut *mut unsigned int es:1; / 22: basic-sampling enable control,
    pub /: *mut *mut unsigned int ed:1; / 23: diag-sampling enable control,
    pub /: *mut *mut unsigned int b24_29:6; / 24-29: zeros,
    pub /: *mut *mut unsigned int cs:1; / 30: basic-sampling activation control,
    pub /: *mut *mut unsigned int cd:1; / 31: diag-sampling activation control,
    pub /: *mut *mut unsigned int bsdes:16; / 4-5: size of basic sampling entry,
    pub /: *mut *mut unsigned int dsdes:16; / 6-7: size of diagnostic sampling entry,
    pub /: *mut *mut unsigned long min_sampl_rate; / 8-15: minimum sampling interval,
    pub interval*/: *mut *mut unsigned long max_sampl_rate; / 16-23: maximum sampling,
    pub /: *mut *mut unsigned long tear; / 24-31: TEAR contents,
    pub /: *mut *mut unsigned long dear; / 32-39: DEAR contents,
    pub /: *mut *mut unsigned int rsvrd0:24; / 40-42: reserved,
    pub /: *mut *mut unsigned int ribm:8; / 43: Reserved by IBM,
    pub /: *mut *mut unsigned int cpu_speed; / 44-47: CPU speed,
    pub /: *mut *mut unsigned long long rsvrd1; / 48-55: reserved,
    pub /: *mut *mut unsigned long long rsvrd2; / 56-63: reserved,
    pub __packed: },
// SET SAMPLING CONTROLS request block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_lsctl_request_block {
    pub /: *mut *mut unsigned int s:1; / 0: maximum buffer indicator,
    pub use*/: *mut *mut unsigned int h:1; / 1: part. level reserved for VM,
    pub /: *mut *mut unsigned long long b2_53:52;/ 2-53: zeros,
    pub /: *mut *mut unsigned int es:1; / 54: basic-sampling enable control,
    pub /: *mut *mut unsigned int ed:1; / 55: diag-sampling enable control,
    pub /: *mut *mut unsigned int b56_61:6; / 56-61: - zeros,
    pub /: *mut *mut unsigned int cs:1; / 62: basic-sampling activation control,
    pub /: *mut *mut unsigned int cd:1; / 63: diag-sampling activation control,
    pub /: *mut *mut unsigned long interval; / 8-15: sampling interval,
    pub /: *mut *mut unsigned long tear; / 16-23: TEAR contents,
    pub /: *mut *mut unsigned long dear; / 24-31: DEAR contents,
// 32-63:
    pub /: *mut *mut unsigned long rsvrd1; / reserved,
    pub /: *mut *mut unsigned long rsvrd2; / reserved,
    pub /: *mut *mut unsigned long rsvrd3; / reserved,
    pub /: *mut *mut unsigned long rsvrd4; / reserved,
    pub __packed: },
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
    pub /: *mut *mut unsigned int H:1; / 34 Host Indicator,
    pub /: *mut *mut unsigned int LS:1; / 35 Limited Sampling,
    pub int:12: unsigned,
    pub /: *mut *mut unsigned int prim_asn:16; / primary ASN,
    pub /: *mut *mut unsigned long long ia; / Instruction Address,
    pub /: *mut *mut unsigned long long gpp; / Guest Program Parameter,
    pub /: *mut *mut unsigned long long hpp; / Host Program Parameter,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_diag_entry {
    pub /: *mut *mut unsigned int def:16; / 0-15 Data Entry Format,
    pub /: *mut *mut unsigned int R:15; / 16-19 and 20-30 reserved,
    pub /: *mut *mut unsigned int I:1; / 31 entry valid or invalid,
    pub /: *mut *mut u8 data[]; / Machine-dependent sample data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_combined_entry {
    pub /: *mut *mut hws_basic_entry basic; / Basic-sampling data entry,
    pub /: *mut *mut hws_diag_entry diag; / Diagnostic-sampling data entry,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union hws_trailer_header {
    pub /: *mut *mut unsigned int f:1; / 0 - Block Full Indicator,
    pub /: *mut *mut unsigned int a:1; / 1 - Alert request control,
    pub /: *mut *mut unsigned int t:1; / 2 - Timestamp format,
    pub /: *mut *mut unsigned int :29; / 3 - 31: Reserved,
    pub /: *mut *mut unsigned int bsdes:16; / 32-47: size of basic SDE,
    pub /: *mut *mut unsigned int dsdes:16; / 48-63: size of diagnostic SDE,
    pub /: *mut *mut unsigned long long overflow; / 64 - Overflow Count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_trailer_entry {
    pub /: *mut *mut hws_trailer_header header; / 0 - 15 Flags + Overflow Count,
    pub /: *mut *mut unsigned char timestamp[16]; / 16 - 31 timestamp,
    pub /: *mut *mut unsigned long long reserved1; / 32 -Reserved,
    pub /: *mut *mut unsigned long long reserved2; /,
    pub /: *mut *mut unsigned int clock_base:1; / in progusage2,
    pub progusage1:63: c_ulonglong,
    pub progusage2: c_ulonglong,
}

// Load program parameter
extern "C" {
    pub fn volatile("memory": "lpp 0(%0)\n" :: "a" (pp) :) -> asm;
}
// Query counter information
// Load CPU-counter-set controls
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
// Extract CPU counter
// content = _content;
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
// Extract CPU counter
// val = content;
// Store CPU counter multiple for a particular counter set
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stcctm_ctr_set {
    EXTENDED = 0,
    BASIC = 1,
    PROBLEM_STATE = 2,
    CRYPTO_ACTIVITY = 3,
    MT_DIAG = 5,
    MT_DIAG_CLEARING = 9,	/* clears loss-of-MT-ctr-data alert */
}

//
// If cc == 2, less than RANGE counters are stored, but it's not easy
// to tell how many. Always unpoison the whole range for simplicity.
//
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
// Query sampling information
// Load sampling controls
