//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cputable.h
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

// This structure can grow, it's real size is used by head.S code
// via the mkdefs mechanism.
//
extern "C" {
    pub fn void(offset: *mut *mut cpu_setup_t)(unsigned long, spec: *mut *mut cpu_spec) -> typedef;
}
extern "C" {
    pub fn void(_arg: *mut cpu_restore_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum powerpc_pmc_type {
    PPC_PMC_DEFAULT = 0,
    PPC_PMC_IBM = 1,
    PPC_PMC_PA6T = 2,
    PPC_PMC_G4 = 3,
}

extern "C" {
    pub fn machine_check_generic(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_4xx(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_440A(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_e500mc(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_e500(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_47x(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_8xx(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn machine_check_83xx(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn cpu_down_flush_e500v2();
}
extern "C" {
    pub fn cpu_down_flush_e500mc();
}
extern "C" {
    pub fn cpu_down_flush_e5500();
}
extern "C" {
    pub fn cpu_down_flush_e6500();
}
// NOTE WELL: Update identify_cpu() if fields are added or removed!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_spec {
// CPU is matched via (PVR & pvr_mask) == pvr_value
    pub pvr_mask: c_uint,
    pub pvr_value: c_uint,
    pub cpu_name: *mut c_char,
    pub /: *mut *mut unsigned long cpu_features; / Kernel features,
    pub /: *mut *mut unsigned int cpu_user_features; / Userland features,
    pub /: *mut *mut unsigned int cpu_user_features2; / Userland features v2,
    pub /: *mut *mut unsigned int mmu_features; / MMU features,
// cache line sizes
    pub icache_bsize: c_uint,
    pub dcache_bsize: c_uint,
// flush caches inside the current cpu
    pub (*cpu_down_flush)(void): *mut c_void,
// number of performance monitor counters
    pub num_pmcs: c_uint,
    pub pmc_type: powerpc_pmc_type,
// this is called to initialize various CPU bits like L1 cache,
// BHT, SPD, etc... from head.S before branching to identify_machine
//
    pub cpu_setup: cpu_setup_t,
// Used to restore cpu setup on secondary processors and at resume
    pub cpu_restore: cpu_restore_t,
// Name of processor class, for the ELF AT_PLATFORM entry
    pub platform: *mut c_char,
// Processor specific machine check handling. Return negative
// if the error is fatal, 1 if it was fully recovered and 0 to
// pass up (not CPU originated)
    pub regs): *mut *mut int (machine_check)(struct pt_regs,
//
// Processor specific early machine check handler which is
// called in real mode to handle SLB and TLB errors.
//
    pub regs): *mut *mut long (machine_check_early)(struct pt_regs,
}

extern "C" {
    pub fn set_cur_cpu_spec(s: *mut cpu_spec);
}
extern "C" {
    pub fn identify_cpu_name(pvr: c_uint);
}

extern "C" {
    pub fn cpu_feature_keys_init();
}

// CPU kernel features
// Definitions for features that we have on both 32-bit and 64-bit chips

// ASM_CONST(0x00000020) Free

// Definitions for features that only exist on 32-bit chips

// Define these to 0 for the sake of tests in common code

//
// Definitions for the 64-bit processor unique features;
// on 32-bit, make the names available but defined to be 0.
//

pub const LONG_ASM_CONST(x): c_int = 0;

// LONG_ASM_CONST(0x0000000400000000) Free

// We only set the altivec features if the kernel was compiled with altivec
// support
//

pub const CPU_FTR_ALTIVEC_COMP: c_int = 0;
pub const PPC_FEATURE_HAS_ALTIVEC_COMP: c_int = 0;

// We only set the VSX features if the kernel was compiled with VSX
// support
//

pub const CPU_FTR_VSX_COMP: c_int = 0;
pub const PPC_FEATURE_HAS_VSX_COMP: c_int = 0;

// We only set the spe features if the kernel was compiled with spe
// support
//

pub const CPU_FTR_SPE_COMP: c_int = 0;
pub const PPC_FEATURE_HAS_SPE_COMP: c_int = 0;
pub const PPC_FEATURE_HAS_EFP_SINGLE_COMP: c_int = 0;
pub const PPC_FEATURE_HAS_EFP_DOUBLE_COMP: c_int = 0;

// We only set the TM feature if the kernel was compiled with TM supprt

pub const CPU_FTR_TM_COMP: c_int = 0;
pub const PPC_FEATURE2_HTM_COMP: c_int = 0;
pub const PPC_FEATURE2_HTM_NOSC_COMP: c_int = 0;

// We need to mark all pages as being coherent if we're SMP or we have a
// 74[45]x and an MPC107 host bridge. Also 83xx and PowerQUICC II
// require it for PCI "streaming/prefetch" to work properly.
// This is also required by 52xx family.
//

pub const CPU_FTR_COMMON: c_int = 0;

// The powersave features NAP & DOZE seems to confuse BDI when
//

pub const CPU_FTR_MAYBE_CAN_DOZE: c_int = 0;
pub const CPU_FTR_MAYBE_CAN_NAP: c_int = 0;

//
// e5500/e6500 erratum A-006958 is a timebase bug that can use the
// same workaround as CPU_FTR_CELL_TB_BUG.
//

// 64-bit CPUs

// pseries may disable DBELL with ibm,pi-features

//
// Maximum number of hw breakpoint supported on powerpc. Number of
// breakpoints supported by actual hw might be less than this, which
// is decided at run time in nr_wp_slots().
//
pub const HBP_NUM_MAX: c_int = 2;

