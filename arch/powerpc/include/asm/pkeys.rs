//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pkeys.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// PowerPC Memory Protection Keys management
//
// Copyright 2017, Ram Pai, IBM Corporation.
//

// Override any generic PKEY permission defines
pub const PKEY_DISABLE_EXECUTE: c_uint = 0x4;

// Reserved keys are never allocated.
extern "C" {
    pub fn __mm_pkey_is_allocated(_arg: mm, _arg: pkey) -> return;
}
//
// Returns a positive, 5-bit key on success, or -1 on failure.
// Relies on the mmap_lock to protect against concurrency in mm_pkey_alloc() and
// mm_pkey_free().
//
// Note: this is the one and only place we make sure that the pkey is
// valid as far as the hardware is concerned. The rest of the kernel
// trusts that only good, valid pkeys come out of here.
//
// Are we out of pkeys? We must handle this specially because ffz()
// behavior is undefined if there are no zeros.
//
// Try to dedicate one of the protection keys to be used as an
// execute-only protection key.
//
extern "C" {
    pub fn execute_only_pkey(mm: *mut mm_struct) -> c_int;
}
//
// Is this an mprotect_pkey() call? If so, never override the value that
// came from the user.
//
extern "C" {
    pub fn __arch_override_mprotect_pkey(_arg: vma, _arg: prot, _arg: pkey) -> return;
}
extern "C" {
    pub fn __arch_set_user_pkey_access(pkey: c_int, init_val: c_ulong) -> c_int;
}
//
// userspace should not change pkey-0 permissions.
// pkey-0 is associated with every page in the kernel.
// If userspace denies any permission on pkey-0, the
// kernel cannot operate.
//
extern "C" {
    pub fn __arch_set_user_pkey_access(_arg: pkey, _arg: init_val) -> return;
}
extern "C" {
    pub fn mmu_has_feature(_arg: MMU_FTR_PKEY) -> return;
}
extern "C" {
    pub fn pkey_mm_init(mm: *mut mm_struct);
}
