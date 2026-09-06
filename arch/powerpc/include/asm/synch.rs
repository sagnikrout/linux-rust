//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/synch.h
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

extern "C" {
    pub fn __volatile__("memory": "mbar" : : :) -> __asm__;
}
extern "C" {
    pub fn __volatile__("memory": "eieio" : : :) -> __asm__;
}
extern "C" {
    pub fn __volatile__("memory": "isync" : : :) -> __asm__;
}
extern "C" {
    pub fn volatile(:"memory": "ptesync": :) -> asm;
}
//
// POWER9, POWER10 need a cp_abort after tlbiel to ensure the copy is
// invalidated correctly. If this is not done, the paste can take data
// from the physical address that was translated at copy time.
//
// POWER9 in practice does not need this, because address spaces with
// accelerators mapped will use tlbie (which does invalidate the copy)
// to invalidate translations. It's not possible to limit POWER10 this
// way due to local copy-paste.
//
// POWER12 does not need it.
//

// Macro flag: #define PPC_ACQUIRE_BARRIER
// Macro flag: #define PPC_RELEASE_BARRIER
// Macro flag: #define PPC_ATOMIC_ENTRY_BARRIER
// Macro flag: #define PPC_ATOMIC_EXIT_BARRIER

