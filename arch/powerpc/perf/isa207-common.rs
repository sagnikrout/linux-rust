//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/isa207-common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2009 Paul Mackerras, IBM Corporation.
// Copyright 2013 Michael Ellerman, IBM Corporation.
// Copyright 2016 Madhavan Srinivasan, IBM Corporation.
//

pub const EVENT_BHRB_SHIFT: c_int = 62;

pub const EVENT_IFM_SHIFT: c_int = 60;

pub const EVENT_THR_CMP_MASK: c_uint = 0x3ff;

pub const EVENT_THR_CTL_MASK: c_uint = 0xffull;

pub const EVENT_THR_SEL_MASK: c_uint = 0x7;

pub const EVENT_THRESH_MASK: c_uint = 0x1fffffull;

pub const EVENT_SAMPLE_MASK: c_uint = 0x1f;

pub const EVENT_CACHE_SEL_MASK: c_uint = 0xf;

pub const EVENT_PMC_MASK: c_uint = 0xf;

pub const EVENT_UNIT_MASK: c_uint = 0xf;

pub const EVENT_COMBINE_MASK: c_uint = 0x1;

pub const EVENT_MARKED_MASK: c_uint = 0x1;

pub const EVENT_PSEL_MASK: c_uint = 0xff	/* PMCxSEL value */;
// Bits defined by Linux

// Contants to support power9 raw encoding format

pub const p9_EVENT_COMBINE_MASK: c_uint = 0x3ull;

pub const p9_SDAR_MODE_SHIFT: c_int = 50;
pub const p9_SDAR_MODE_MASK: c_uint = 0x3ull;

// Contants to support power10 raw encoding format
pub const p10_SDAR_MODE_SHIFT: c_int = 22;
pub const p10_SDAR_MODE_MASK: c_uint = 0x3ull;

pub const p10_EVENT_L2L3_SEL_MASK: c_uint = 0x1f;
pub const p10_L2L3_SEL_SHIFT: c_int = 3;
pub const p10_L2L3_EVENT_SHIFT: c_int = 40;
pub const p10_EVENT_THRESH_MASK: c_uint = 0xffffull;
pub const p10_EVENT_CACHE_SEL_MASK: c_uint = 0x3ull;
pub const p10_EVENT_MMCR3_MASK: c_uint = 0x7fffull;
pub const p10_EVENT_MMCR3_SHIFT: c_int = 45;
pub const p10_EVENT_RADIX_SCOPE_QUAL_SHIFT: c_int = 9;
pub const p10_EVENT_RADIX_SCOPE_QUAL_MASK: c_uint = 0x1;
pub const p10_MMCR1_RADIX_SCOPE_QUAL_SHIFT: c_int = 45;
// Event Threshold Compare bit constant for power10 in config1 attribute
pub const p10_EVENT_THR_CMP_SHIFT: c_int = 0;
pub const p10_EVENT_THR_CMP_MASK: c_uint = 0x3FFFFull;

//
// Layout of constraint bits:
//
// 60        56        52        48        44        40        36        32
// | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - |
// [   fab_match   ]         [       thresh_cmp      ] [   thresh_ctl    ] [   ]
// |                                  |
// [  thresh_cmp bits for p10]           thresh_sel -
//
// 28        24        20        16        12         8         4         0
// | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - | - - - - |
// [ ] |   [ ] |  [  sample ]   [     ]   [6] [5]   [4] [3]   [2] [1]
// |  |    |  |                  |
// BHRB IFM -*  |    |  |*radix_scope      |      Count of events for each PMC.
// EBB -*    |                     |        p1, p2, p3, p4, p5, p6.
// L1 I/D qualifier -*                     |
// nc - number of counters -
//
// The PMC fields P1..P6, and NC, are adder fields. As we accumulate constraints
// we want the low bit of each field to be added to any existing value.
//
// Everything else is a value field.
//

// We just throw all the threshold bits into the constraint

//
// For NC we are counting up to 4 events. This requires three bits, and we need
// the fifth event to overflow and set the 4th bit. To achieve that we bias the
// fields by 3 in test_adder.
//
pub const CNST_NC_SHIFT: c_int = 12;

//
// For the per-PMC fields we have two bits. The low bit is added, so if two
// events ask for the same PMC the sum will overflow, setting the high bit,
// indicating an error. So our mask sets the high bit.
//

// Our add_fields is defined as:

// Bits in MMCR1 for PowerISA v2.07

pub const MMCR1_FAB_SHIFT: c_int = 36;
pub const MMCR1_DC_IC_QUAL_MASK: c_uint = 0x3;
pub const MMCR1_DC_IC_QUAL_SHIFT: c_int = 46;
// MMCR1 Combine bits macro for power9

// Bits in MMCRA for PowerISA v2.07
pub const MMCRA_SAMP_MODE_SHIFT: c_int = 1;
pub const MMCRA_SAMP_ELIG_SHIFT: c_int = 4;
pub const MMCRA_SAMP_ELIG_MASK: c_int = 7;
pub const MMCRA_THR_CTL_SHIFT: c_int = 8;
pub const MMCRA_THR_SEL_SHIFT: c_int = 16;
pub const MMCRA_THR_CMP_SHIFT: c_int = 32;
pub const MMCRA_SDAR_MODE_SHIFT: c_int = 42;

pub const MMCRA_IFM_SHIFT: c_int = 30;
pub const MMCRA_THR_CTR_MANT_SHIFT: c_int = 19;
pub const MMCRA_THR_CTR_MANT_MASK: c_uint = 0x7Ful;

pub const MMCRA_THR_CTR_EXP_SHIFT: c_int = 27;
pub const MMCRA_THR_CTR_EXP_MASK: c_uint = 0x7ul;

pub const P10_MMCRA_THR_CTR_MANT_MASK: c_uint = 0xFFul;

// MMCRA Threshold Compare bit constant for power9
pub const p9_MMCRA_THR_CMP_SHIFT: c_int = 45;
// Bits in MMCR2 for PowerISA v2.07

pub const MAX_ALT: c_int = 2;
pub const MAX_PMU_COUNTERS: c_int = 6;
// Bits in MMCR3 for PowerISA v3.10

pub const ISA207_SIER_TYPE_SHIFT: c_int = 15;

pub const ISA207_SIER_LDST_SHIFT: c_int = 1;

pub const ISA207_SIER_DATA_SRC_SHIFT: c_int = 53;

// Bits in SIER2/SIER3 for Power10

extern "C" {
    pub fn isa207_get_constraint(event: u64, maskp: *mut c_ulong, valp: *mut c_ulong, event_config1: u64) -> c_int;
}
extern "C" {
    pub fn isa207_disable_pmc(pmc: c_uint, mmcr: *mut mmcr_regs);
}
extern "C" {
    pub fn isa207_get_mem_weight(weight: *mut u64, type: u64);
}
extern "C" {
    pub fn isa3XX_check_attr_config(ev: *mut perf_event) -> c_int;
}
