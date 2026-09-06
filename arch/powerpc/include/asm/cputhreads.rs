//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cputhreads.h
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
// Mapping of threads to cores
//
// Note: This implementation is limited to a power of 2 number of
// threads per core and the same number for each core in the system
// (though it would work if some processors had less threads as long
// as the CPU numbers are still allocated, just not brought online).
//
// However, the API allows for a different implementation in the future
// if needed, as long as you only use the functions and not the variables
// directly.
//

pub const threads_per_core: c_int = 1;
pub const threads_per_subcore: c_int = 1;
pub const threads_shift: c_int = 0;
pub const has_big_cores: c_int = 0;

extern "C" {
    pub fn cpu_core_index_of_thread(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn cpu_first_thread_of_core(core: c_int) -> c_int;
}

//
// tlb_thread_siblings are siblings which share a TLB. This is not
// architected, is not something a hypervisor could emulate and a future
// CPU may change behaviour even in compat mode, so this should only be
// used on PowerNV, and only with care.
//
extern "C" {
    pub fn cpu_first_thread_sibling(_arg: cpu) -> return;
}
extern "C" {
    pub fn cpu_last_thread_sibling(_arg: cpu) -> return;
}

extern "C" {
    pub fn mfspr(_arg: SPRN_TENSR) -> return;
}

extern "C" {
    pub fn book3e_start_thread(thread: c_int, addr: c_ulong);
}
extern "C" {
    pub fn book3e_stop_thread(thread: c_int);
}

pub const INVALID_THREAD_HWID: c_uint = 0x0fff;
