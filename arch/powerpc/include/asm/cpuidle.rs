//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cpuidle.h
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

// Thread state used in powernv idle state management
pub const PNV_THREAD_RUNNING: c_int = 0;
pub const PNV_THREAD_NAP: c_int = 1;
pub const PNV_THREAD_SLEEP: c_int = 2;
pub const PNV_THREAD_WINKLE: c_int = 3;
//
// Core state used in powernv idle for POWER8.
//
// The lock bit synchronizes updates to the state, as well as parts of the
// sleep/wake code (see kernel/idle_book3s.S).
//
// Bottom 8 bits track the idle state of each thread. Bit is cleared before
// the thread executes an idle instruction (nap/sleep/winkle).
//
// Then there is winkle tracking. A core does not lose complete state
// until every thread is in winkle. So the winkle count field counts the
// number of threads in winkle (small window of false positives is okay
// around the sleep/wake, so long as there are no false negatives).
//
// When the winkle count reaches 8 (the COUNT_ALL_BIT becomes set), then
// the THREAD_WINKLE_BITS are set, which indicate which threads have not
// yet woken from the winkle state.
//
pub const NR_PNV_CORE_IDLE_LOCK_BIT: c_int = 28;

pub const PNV_CORE_IDLE_WINKLE_COUNT_SHIFT: c_int = 16;
pub const PNV_CORE_IDLE_WINKLE_COUNT: c_uint = 0x00010000;
pub const PNV_CORE_IDLE_WINKLE_COUNT_BITS: c_uint = 0x000F0000;
pub const PNV_CORE_IDLE_THREAD_WINKLE_BITS_SHIFT: c_int = 8;
pub const PNV_CORE_IDLE_THREAD_WINKLE_BITS: c_uint = 0x0000FF00;
pub const PNV_CORE_IDLE_THREAD_BITS: c_uint = 0x000000FF;
//
// ============================ NOTE =================================
// The older firmware populates only the RL field in the psscr_val and
// sets the psscr_mask to 0xf. On such a firmware, the kernel sets the
// remaining PSSCR fields to default values as follows:
//
// - ESL and EC bits are to 1. So wakeup from any stop state will be
// at vector 0x100.
//
// - MTL and PSLL are set to the maximum allowed value as per the ISA,
// i.e. 15.
//
// - The Transition Rate, TR is set to the Maximum value 3.
//

pub const PSSCR_EC_SHIFT: c_int = 20;
pub const PSSCR_ESL_SHIFT: c_int = 21;

pub const PNV_IDLE_NAME_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_idle_states_t {
    pub name: [c_char; PNV_IDLE_NAME_LEN],
    pub latency_ns: u32,
    pub residency_ns: u32,
    pub psscr_val: u64,
    pub psscr_mask: u64,
    pub flags: u32,
    pub valid: bool,
}

extern "C" {
    pub fn pnv_cpu_offline(cpu: c_uint) -> c_ulong;
}
extern "C" {
    pub fn validate_psscr_val_mask(psscr_val: *mut u64, psscr_mask: *mut u64, flags: u32) -> int __init;
}

