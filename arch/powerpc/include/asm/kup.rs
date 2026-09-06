//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kup.h
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
pub const KUAP_READ: c_int = 1;
pub const KUAP_WRITE: c_int = 2;

extern "C" {
    pub fn kuap_is_disabled() -> static __always_inline bool;
}

extern "C" {
    pub fn setup_kup();
}
extern "C" {
    pub fn setup_kuep(disabled: bool);
}

extern "C" {
    pub fn setup_kuap(disabled: bool);
}

//
// book3s/64/kup-radix.h defines these functions for the !KUAP case to flush
// the L1D cache after user accesses. Only include the empty stubs for other
// platforms.
//

extern "C" {
    pub fn __bad_kuap_fault(_arg: regs, _arg: address, _arg: is_write) -> return;
}

extern "C" {
    pub fn __kuap_get_and_assert_locked() -> return;
}

