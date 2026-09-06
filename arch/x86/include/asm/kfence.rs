//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kfence.h
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
// x86 KFENCE support.
//
// Copyright (C) 2020, Google LLC.
//

// Force 4K pages for __kfence_pool.
// Protect the given page and flush TLB.
//
// protect requires making the page not-present.  If the PTE is
// already in the right state, there's nothing to do.
//
// Otherwise, flip the Present bit, taking care to avoid writing an
// L1TF-vulnerable PTE (not present, without the high address bits
// set).
//
// If the page was protected (non-present) and we're making it
// present, there is no need to flush the TLB at all.
//
// We need to avoid IPIs, as we may get KFENCE allocations or faults
// with interrupts disabled. Therefore, the below is best-effort, and
// does not flush TLBs on all CPUs. We can tolerate some inaccuracy;
// lazy fault handling takes care of faults after the page is PRESENT.
//
// Flush this CPU's TLB, assuming whoever did the allocation/free is
// likely to continue running on this CPU.
//

