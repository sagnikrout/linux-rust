//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/security_features.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Security related feature bit definitions.
//
// Copyright 2018, Michael Ellerman, IBM Corporation.
//
// These are bit flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stf_barrier_type {
    STF_BARRIER_NONE	= 0x1,
    STF_BARRIER_FALLBACK	= 0x2,
    STF_BARRIER_EIEIO	= 0x4,
    STF_BARRIER_SYNC_ORI	= 0x8,
}

extern "C" {
    pub fn setup_stf_barrier();
}
extern "C" {
    pub fn do_stf_barrier_fixups(types: stf_barrier_type);
}
extern "C" {
    pub fn setup_count_cache_flush();
}

extern "C" {
    pub fn stf_barrier_type_get() -> stf_barrier_type;
}

// Features indicating support for Spectre/Meltdown mitigations
// The L1-D cache can be flushed with ori r30,r30,0
pub const SEC_FTR_L1D_FLUSH_ORI30: c_uint = 0x0000000000000001ull;
// The L1-D cache can be flushed with mtspr 882,r0 (aka SPRN_TRIG2)
pub const SEC_FTR_L1D_FLUSH_TRIG2: c_uint = 0x0000000000000002ull;
// ori r31,r31,0 acts as a speculation barrier
pub const SEC_FTR_SPEC_BAR_ORI31: c_uint = 0x0000000000000004ull;
// Speculation past bctr is disabled
pub const SEC_FTR_BCCTRL_SERIALISED: c_uint = 0x0000000000000008ull;
// Entries in L1-D are private to a SMT thread
pub const SEC_FTR_L1D_THREAD_PRIV: c_uint = 0x0000000000000010ull;
// Indirect branch prediction cache disabled
pub const SEC_FTR_COUNT_CACHE_DISABLED: c_uint = 0x0000000000000020ull;
// bcctr 2,0,0 triggers a hardware assisted count cache flush
pub const SEC_FTR_BCCTR_FLUSH_ASSIST: c_uint = 0x0000000000000800ull;
// bcctr 2,0,0 triggers a hardware assisted link stack flush
pub const SEC_FTR_BCCTR_LINK_FLUSH_ASSIST: c_uint = 0x0000000000002000ull;
// Features indicating need for Spectre/Meltdown mitigations
// The L1-D cache should be flushed on MSR[HV] 1->0 transition (hypervisor to guest)
pub const SEC_FTR_L1D_FLUSH_HV: c_uint = 0x0000000000000040ull;
// The L1-D cache should be flushed on MSR[PR] 0->1 transition (kernel to userspace)
pub const SEC_FTR_L1D_FLUSH_PR: c_uint = 0x0000000000000080ull;
// A speculation barrier should be used for bounds checks (Spectre variant 1)
pub const SEC_FTR_BNDS_CHK_SPEC_BAR: c_uint = 0x0000000000000100ull;
// Firmware configuration indicates user favours security over performance
pub const SEC_FTR_FAVOUR_SECURITY: c_uint = 0x0000000000000200ull;
// Software required to flush count cache on context switch
pub const SEC_FTR_FLUSH_COUNT_CACHE: c_uint = 0x0000000000000400ull;
// Software required to flush link stack on context switch
pub const SEC_FTR_FLUSH_LINK_STACK: c_uint = 0x0000000000001000ull;
// The L1-D cache should be flushed when entering the kernel
pub const SEC_FTR_L1D_FLUSH_ENTRY: c_uint = 0x0000000000004000ull;
// The L1-D cache should be flushed after user accesses from the kernel
pub const SEC_FTR_L1D_FLUSH_UACCESS: c_uint = 0x0000000000008000ull;
// The STF flush should be executed on privilege state switch
pub const SEC_FTR_STF_BARRIER: c_uint = 0x0000000000010000ull;
// Features enabled by default

