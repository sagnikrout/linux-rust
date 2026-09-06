//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/idle.h
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

// this_cpu_ptr(&idle_entry_purr_snap) = mfspr(SPRN_PURR);
// this_cpu_ptr(&idle_entry_spurr_snap) = mfspr(SPRN_SPURR);
// idle_spurr_cycles_ptr += mfspr(SPRN_SPURR) - in_spurr;
//
// Indicate to the HV that we are idle. Now would be
// a good time to find other work to dispatch.
//
// If we are reading from an idle context, update the
// idle-purr cycles corresponding to the last idle period.
// Since the idle context is not yet over, take a fresh
// snapshot of the idle-purr.
//
extern "C" {
    pub fn be64_to_cpu(_arg: get_lppaca()->wait_state_cycles) -> return;
}
//
// If we are reading from an idle context, update the
// idle-spurr cycles corresponding to the last idle period.
// Since the idle context is not yet over, take a fresh
// snapshot of the idle-spurr.
//

