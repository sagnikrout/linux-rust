//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/perf_event_server.h
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
// Performance event support - PowerPC classic/server specific definitions.
//
// Copyright 2008-2009 Paul Mackerras, IBM Corporation.
//

// Update perf_event_print_debug() if this changes
pub const MAX_HWEVENTS: c_int = 8;
pub const MAX_EVENT_ALTERNATIVES: c_int = 8;
pub const MAX_LIMITED_HWCOUNTERS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmcr_regs {
    pub mmcr0: c_ulong,
    pub mmcr1: c_ulong,
    pub mmcr2: c_ulong,
    pub mmcra: c_ulong,
    pub mmcr3: c_ulong,
}

//
// This struct provides the constants and functions needed to
// describe the PMU on a particular POWER-family CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_pmu {
    pub name: *const c_char,
    pub n_counter: c_int,
    pub max_alternatives: c_int,
    pub add_fields: c_ulong,
    pub test_adder: c_ulong,
    pub flags): *mut *mut perf_event pevents[], u32,
    pub event_config1): *mut *mut unsigned long valp, u64,
    pub alt[]): u64,
    pub regs): *mut u32 flags, struct pt_regs,
    pub type): *mut *mut *mut void (get_mem_weight)(u64 weight, u64,
    pub group_constraint_mask: c_ulong,
    pub group_constraint_val: c_ulong,
    pub branch_sample_type): *mut *mut u64 (bhrb_filter_map)(u64,
    pub pmu_bhrb_filter): *mut *mut void (config_bhrb)(u64,
    pub mmcr): *mut *mut void (disable_pmc)(unsigned int pmc, struct mmcr_regs,
    pub event_id): *mut *mut int (limited_pmc_event)(u64,
    pub flags: u32,
    pub attr_groups: *const attribute_group,
    pub n_generic: c_int,
    pub generic_events: *mut c_int,
    pub n_blacklist_ev: c_int,
    pub blacklist_ev: *mut c_int,
// BHRB entries in the PMU
    pub bhrb_nr: c_int,
//
// set this flag with `PERF_PMU_CAP_EXTENDED_REGS` if
// the pmu supports extended perf regs capability
//
    pub capabilities: c_int,
//
// Function to check event code for values which are
// reserved. Function takes struct perf_event as input,
// since event code could be spread in attr.config
//
    pub ev): *mut *mut int (check_attr_config)(struct perf_event,
}

//
// Values for power_pmu.flags
//
pub const PPMU_LIMITED_PMC5_6: c_uint = 0x00000001 /* PMC5/6 have limited function */;
pub const PPMU_ALT_SIPR: c_uint = 0x00000002 /* uses alternate posn for SIPR/HV */;
pub const PPMU_NO_SIPR: c_uint = 0x00000004 /* no SIPR/HV in MMCRA at all */;
pub const PPMU_NO_CONT_SAMPLING: c_uint = 0x00000008 /* no continuous sampling */;
pub const PPMU_SIAR_VALID: c_uint = 0x00000010 /* Processor has SIAR Valid bit */;
pub const PPMU_HAS_SSLOT: c_uint = 0x00000020 /* Has sampled slot in MMCRA */;
pub const PPMU_HAS_SIER: c_uint = 0x00000040 /* Has SIER */;
pub const PPMU_ARCH_207S: c_uint = 0x00000080 /* PMC is architecture v2.07S */;
pub const PPMU_NO_SIAR: c_uint = 0x00000100 /* Do not use SIAR */;
pub const PPMU_ARCH_31: c_uint = 0x00000200 /* Has MMCR3, SIER2 and SIER3 */;
pub const PPMU_P10_DD1: c_uint = 0x00000400 /* Is power10 DD1 processor version */;
pub const PPMU_P10: c_uint = 0x00000800 /* For power10 pmu */;
pub const PPMU_HAS_ATTR_CONFIG1: c_uint = 0x00001000 /* Using config1 attribute */;
//
// Values for flags to get_alternatives()
//

extern "C" {
    pub fn register_power_pmu(pmu: *mut power_pmu) -> int __init;
}
extern "C" {
    pub fn perf_arch_misc_flags(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn read_bhrb(n: c_int) -> unsigned long int;
}
//
// Only override the default definitions in include/linux/perf_event.h
// if we have hardware PMU support.
//

//
// The power_pmu.get_constraint function returns a 32/64-bit value and
// a 32/64-bit mask that express the constraints between this event_id and
// other events.
//
// The value and mask are divided up into (non-overlapping) bitfields
// of three different types:
//
// Select field: this expresses the constraint that some set of bits
// in MMCR* needs to be set to a specific value for this event_id.  For a
// select field, the mask contains 1s in every bit of the field, and
// the value contains a unique value for each possible setting of the
// MMCR* bits.  The constraint checking code will ensure that two events
// that set the same field in their masks have the same value in their
// value dwords.
//
// Add field: this expresses the constraint that there can be at most
// N events in a particular class.  A field of k bits can be used for
// N <= 2^(k-1) - 1.  The mask has the most significant bit of the field
// set (and the other bits 0), and the value has only the least significant
// bit of the field set.  In addition, the 'add_fields' and 'test_adder'
// in the struct power_pmu for this processor come into play.  The
// add_fields value contains 1 in the LSB of the field, and the
// test_adder contains 2^(k-1) - 1 - N in the field.
//
// NAND field: this expresses the constraint that you may not have events
// in all of a set of classes.  (For example, on PPC970, you can't select
// events from the FPU, ISU and IDU simultaneously, although any two are
// possible.)  For N classes, the field is N+1 bits wide, and each class
// is assigned one bit from the least-significant N bits.  The mask has
// only the most-significant bit set, and the value has only the bit
// for the event_id's class set.  The test_adder has the least significant
// bit set in the field.
//
// If an event_id is not subject to the constraint expressed by a particular
// field, then it will have 0 in both the mask and value for that field.
//
// EVENT_VAR() is same as PMU_EVENT_VAR with a suffix.
//
// Having a suffix allows us to have aliases in sysfs - eg: the generic
// event 'cpu-cycles' can have two entries in sysfs: 'cpu-cycles' and
// 'PM_CYC' where the latter is the name by which the event is known in
// POWER CPU specification.
//
// Similarly, some hardware and cache events use the same event code. Eg.
// on POWER8, both "cache-references" and "L1-dcache-loads" events refer
// to the same event, PM_LD_REF_L1.  The suffix, allows us to have two
// sysfs objects for the same event and thus two entries/aliases in sysfs.
//

