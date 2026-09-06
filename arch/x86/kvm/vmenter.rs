//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmenter.h
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
// SPEC_CTRL handling: if the guest's SPEC_CTRL value differs from the
// host's, write the MSR.  This is kept out-of-line so that the common
// case does not have to jump.
//
// IMPORTANT: To avoid RSB underflow attacks and any other nastiness,
// there must not be any returns or indirect branches between this code
// and vmentry.
//

// Same for after vmexit.
//
// Load the value that the guest had written into MSR_IA32_SPEC_CTRL,
// if it was not intercepted during guest execution.
//
// Now restore the host value of the MSR if different from the guest's.

//
// For legacy IBRS, the IBRS bit always needs to be written after
// transitioning from a less privileged predictor mode, regardless of
// whether the guest/host values differ.
//

