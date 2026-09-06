//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpufreq/powernow-k8.h
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
// (c) 2003-2006 Advanced Micro Devices, Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powernow_k8_data {
    pub cpu: c_uint,
    pub /: *mut *mut u32 numps; / number of p-states,
    pub /: *mut *mut u32 batps; / number of p-states supported on battery,
// these values are constant when the PSB is used to determine
// vid/fid pairings, but are modified during the ->target() call
// when ACPI is used
    pub /: *mut *mut u32 rvo; / ramp voltage offset,
    pub /: *mut *mut u32 irt; / isochronous relief time,
    pub /: *mut *mut u32 vidmvs; / usable value calculated from mvs,
    pub /: *mut *mut u32 vstable; / voltage stabilization time, units 20 us,
    pub /: *mut *mut u32 plllock; / pll lock time, units 1 us,
    pub /: *mut *mut u32 exttype; / extended interface = 1,
// keep track of the current fid / vid or pstate
    pub currvid: u32,
    pub currfid: u32,
// the powernow_table includes all frequency and vid/fid pairings:
// fid are the lower 8 bits of the index, vid are the upper 8 bits.
// frequency is in kHz
    pub powernow_table: *mut cpufreq_frequency_table,
// the acpi table needs to be kept. it's only available if ACPI was
// used to determine valid frequency/vid/fid states
    pub acpi_data: acpi_processor_performance,
// we need to keep track of associated cores, but let cpufreq
// handle hotplug events - so just point at cpufreq pol->cpus
// structure
    pub available_cores: *mut cpumask,
}

// processor's cpuid instruction support

pub const CPUID_XFAM: c_uint = 0x0ff00000	/* extended family */;
pub const CPUID_XFAM_K8: c_int = 0;
pub const CPUID_XMOD: c_uint = 0x000f0000	/* extended model */;
pub const CPUID_XMOD_REV_MASK: c_uint = 0x000c0000;
pub const CPUID_XFAM_10H: c_uint = 0x00100000	/* family 0x10 */;
pub const CPUID_USE_XFAM_XMOD: c_uint = 0x00000f00;
pub const CPUID_GET_MAX_CAPABILITIES: c_uint = 0x80000000;
pub const CPUID_FREQ_VOLT_CAPABILITIES: c_uint = 0x80000007;
pub const P_STATE_TRANSITION_CAPABLE: c_int = 6;
// Model Specific Registers for p-state transitions. MSRs are 64-bit. For
// writes (wrmsr - opcode 0f 30), the register number is placed in ecx, and
// the value to write is placed in edx:eax. For reads (rdmsr - opcode 0f 32),
// the register number is placed in ecx, and the data is returned in edx:eax.
pub const MSR_FIDVID_CTL: c_uint = 0xc0010041;
pub const MSR_FIDVID_STATUS: c_uint = 0xc0010042;
// Field definitions within the FID VID Low Control MSR :
pub const MSR_C_LO_INIT_FID_VID: c_uint = 0x00010000;
pub const MSR_C_LO_NEW_VID: c_uint = 0x00003f00;
pub const MSR_C_LO_NEW_FID: c_uint = 0x0000003f;
pub const MSR_C_LO_VID_SHIFT: c_int = 8;
// Field definitions within the FID VID High Control MSR :
pub const MSR_C_HI_STP_GNT_TO: c_uint = 0x000fffff;
// Field definitions within the FID VID Low Status MSR :
pub const MSR_S_LO_CHANGE_PENDING: c_uint = 0x80000000   /* cleared when completed */;
pub const MSR_S_LO_MAX_RAMP_VID: c_uint = 0x3f000000;
pub const MSR_S_LO_MAX_FID: c_uint = 0x003f0000;
pub const MSR_S_LO_START_FID: c_uint = 0x00003f00;
pub const MSR_S_LO_CURRENT_FID: c_uint = 0x0000003f;
// Field definitions within the FID VID High Status MSR :
pub const MSR_S_HI_MIN_WORKING_VID: c_uint = 0x3f000000;
pub const MSR_S_HI_MAX_WORKING_VID: c_uint = 0x003f0000;
pub const MSR_S_HI_START_VID: c_uint = 0x00003f00;
pub const MSR_S_HI_CURRENT_VID: c_uint = 0x0000003f;
pub const MSR_C_HI_STP_GNT_BENIGN: c_uint = 0x00000001;
//
// There are restrictions frequencies have to follow:
// - only 1 entry in the low fid table ( <=1.4GHz )
// - lowest entry in the high fid table must be >= 2 * the entry in the
// low fid table
// - lowest entry in the high fid table must be a <= 200MHz + 2 * the entry
// in the low fid table
// - the parts can only step at <= 200 MHz intervals, odd fid values are
// supported in revision G and later revisions.
// - lowest frequency must be >= interprocessor hypertransport link speed
// (only applies to MP systems obviously)
//
// fids (frequency identifiers) are arranged in 2 tables - lo and hi

pub const HI_VCOFREQ_TABLE_BOTTOM: c_int = 1600;

pub const MAX_FID: c_uint = 0x2a	/* Spec only gives FID values as far as 5 GHz */;
pub const LEAST_VID: c_uint = 0x3e	/* Lowest (numerically highest) useful vid value */;

pub const MAX_FREQ: c_int = 5000;
pub const INVALID_FID_MASK: c_uint = 0xffffffc0  /* not a valid fid if these bits are set */;
pub const INVALID_VID_MASK: c_uint = 0xffffffc0  /* not a valid vid if these bits are set */;
pub const VID_OFF: c_uint = 0x3f;

//
// Most values of interest are encoded in a single field of the _PSS
// entries: the "control" value.
//
pub const IRT_SHIFT: c_int = 30;
pub const RVO_SHIFT: c_int = 28;
pub const EXT_TYPE_SHIFT: c_int = 27;
pub const PLL_L_SHIFT: c_int = 20;
pub const MVS_SHIFT: c_int = 18;
pub const VST_SHIFT: c_int = 11;
pub const VID_SHIFT: c_int = 6;
pub const IRT_MASK: c_int = 3;
pub const RVO_MASK: c_int = 3;
pub const EXT_TYPE_MASK: c_int = 1;
pub const PLL_L_MASK: c_uint = 0x7f;
pub const MVS_MASK: c_int = 3;
pub const VST_MASK: c_uint = 0x7f;
pub const VID_MASK: c_uint = 0x1f;
pub const FID_MASK: c_uint = 0x1f;
pub const EXT_VID_MASK: c_uint = 0x3f;
pub const EXT_FID_MASK: c_uint = 0x3f;
//
// Version 1.4 of the PSB table. This table is constructed by BIOS and is
// to tell the OS's power management driver which VIDs and FIDs are
// supported by this particular processor.
// If the data in the PSB / PST is wrong, then this driver will program the
// wrong values into hardware, which is very likely to lead to a crash.
//

pub const PSB_ID_STRING_LEN: c_int = 10;
pub const PSB_VERSION_1_4: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_s {
    pub signature: [u8; 10],
    pub tableversion: u8,
    pub flags1: u8,
    pub vstable: u16,
    pub flags2: u8,
    pub num_tables: u8,
    pub cpuid: u32,
    pub plllocktime: u8,
    pub maxfid: u8,
    pub maxvid: u8,
    pub numps: u8,
}

// Pairs of fid/vid values are appended to the version 1.4 PSB table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pst_s {
    pub fid: u8,
    pub vid: u8,
}

extern "C" {
    pub fn core_voltage_post_transition(data: *mut powernow_k8_data, reqvid: u32) -> static int;
}
extern "C" {
    pub fn core_frequency_transition(data: *mut powernow_k8_data, reqfid: u32) -> static int;
}
extern "C" {
    pub fn powernow_k8_acpi_pst_values(data: *mut powernow_k8_data, index: c_uint) -> static void;
}
extern "C" {
    pub fn fill_powernow_table_fidvid(data: *mut powernow_k8_data, powernow_table: *mut cpufreq_frequency_table) -> static int;
}
