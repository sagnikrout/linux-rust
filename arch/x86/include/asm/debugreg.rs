//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/debugreg.h
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
// Define bits that are always set to 1 in DR7, only bit 10 is
// architecturally reserved to '1'.
//
// This is also the init/reset value for DR7.
//
pub const DR7_FIXED_1: c_uint = 0x00000400;

//
// These special macros can be used to get or set a debugging register
//

//
// Use "asm volatile" for DR7 reads to forbid re-ordering them
// with other code.
//
// This is needed because a DR7 access can cause a #VC exception
// when running under SEV-ES. Taking a #VC exception is not a
// safe thing to do just anywhere in the entry code and
// re-ordering might place the access into an unsafe location.
//
// This happened in the NMI handler, where the DR7 read was
// re-ordered to happen before the call to sev_es_ist_enter(),
// causing stack recursion.
//
extern "C" {
    pub fn volatile(%%db7: "mov, (val): %0" : "=r") -> asm;
}
//
// Use "asm volatile" for DR7 writes to forbid re-ordering them
// with other code.
//
// While is didn't happen with a DR7 write (see the DR7 read
// comment above which explains where it happened), add the
// "asm volatile" here too to avoid similar problems in the
// future.
//
extern "C" {
    pub fn volatile(%0: "mov, (value): %%db7" ::"r") -> asm;
}
// Reset the control register for HW Breakpoint
// Zero-out the individual HW breakpoint address registers
extern "C" {
    pub fn hw_breakpoint_restore();
}
// Architecturally set bit
//
// Ensure the compiler doesn't lower the above statements into
// the critical section; disabling breakpoints late would not
// be good.
//
// Ensure the compiler doesn't raise this statement into
// the critical section; enabling breakpoints early would
// not be good.
//

extern "C" {
    pub fn amd_set_dr_addr_mask(mask: c_ulong, dr: c_uint);
}
extern "C" {
    pub fn amd_get_dr_addr_mask(dr: c_uint) -> c_ulong;
}

