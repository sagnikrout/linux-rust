//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/pkeys.h
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
// Copyright (C) 2023 Arm Ltd.
//
// Based on arch/x86/include/asm/pkeys.h
//

pub const arch_max_pkey(): c_int = 8;
extern "C" {
    pub fn arch_set_user_pkey_access(pkey: c_int, init_val: c_ulong) -> c_int;
}
extern "C" {
    pub fn system_supports_poe() -> return;
}
extern "C" {
    pub fn vma_pkey(_arg: vma) -> return;
}
// Execute-only mappings are handled by EPAN/FEAT_PAN3.

//
// "Allocated" pkeys are those that have been returned
// from pkey_alloc() or pkey 0 which is allocated
// implicitly when the mm is created.
//
extern "C" {
    pub fn mm_pkey_allocation_map(pkey: mm) & (1U <<) -> return;
}
//
// Returns a positive, 3-bit key on success, or -1 on failure.
//
// Note: this is the one and only place we make sure
// that the pkey is valid as far as the hardware is
// concerned.  The rest of the kernel trusts that
// only good, valid pkeys come out of here.
//
// Are we out of pkeys?  We must handle this specially
// because ffz() behavior is undefined if there are no
// zeros.
//
