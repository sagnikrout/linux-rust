//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/acrn.h
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
// This CPUID returns feature bitmaps in EAX.
// Guest VM uses this to detect the appropriate feature bit.
//
pub const ACRN_CPUID_FEATURES: c_uint = 0x40000001;
// Bit 0 indicates whether guest VM is privileged

//
// Timing Information.
// This leaf returns the current TSC frequency in kHz.
//
// EAX: (Virtual) TSC frequency in kHz.
// EBX, ECX, EDX: RESERVED (reserved fields are set to zero).
//
pub const ACRN_CPUID_TIMING_INFO: c_uint = 0x40000010;
extern "C" {
    pub fn acrn_setup_intr_handler((*handler)(void): *mut c_void);
}
extern "C" {
    pub fn acrn_remove_intr_handler();
}
extern "C" {
    pub fn cpuid_base_hypervisor(_arg: "ACRNACRNACRN", _arg: 0) -> return;
}
extern "C" {
    pub fn cpuid_eax(_arg: ACRN_CPUID_TIMING_INFO) -> return;
}
//
// Hypercalls for ACRN
//
// - VMCALL instruction is used to implement ACRN hypercalls.
// - ACRN hypercall ABI:
// - Hypercall number is passed in R8 register.
// - Up to 2 arguments are passed in RDI, RSI.
// - Return value will be placed in RAX.
//
// Because GCC doesn't support R8 register as direct register constraints, use
// supported constraint as input with a explicit MOV to R8 in beginning of asm.
//
