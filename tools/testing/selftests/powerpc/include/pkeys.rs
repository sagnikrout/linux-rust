//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/include/pkeys.h
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
// Copyright 2020, Sandipan Das, IBM Corp.
//

//
// Older versions of libc use the Intel-specific access rights.
// Hence, override the definitions as they might be incorrect.
//

pub const PKEY_DISABLE_ACCESS: c_uint = 0x3;

pub const PKEY_DISABLE_WRITE: c_uint = 0x2;

pub const PKEY_DISABLE_EXECUTE: c_uint = 0x4;

pub const PKEY_UNRESTRICTED: c_uint = 0x0;
// Older versions of libc do not define this

pub const SEGV_PKUERR: c_int = 4;

pub const SI_PKEY_OFFSET: c_uint = 0x20;
pub const __NR_pkey_mprotect: c_int = 386;
pub const __NR_pkey_alloc: c_int = 384;
pub const __NR_pkey_free: c_int = 385;

pub const NT_PPC_PKEY: c_uint = 0x110;

pub const PKEY_BITS_PER_PKEY: c_int = 2;
pub const NR_PKEYS: c_int = 32;

pub const AMR_BITS_PER_PKEY: c_int = 2;

extern "C" {
    pub fn mfspr(_arg: SPRN_AMR) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_pkey_mprotect, _arg: addr, _arg: len, _arg: prot, _arg: pkey) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_pkey_alloc, _arg: flags, _arg: rights) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_pkey_free, _arg: pkey) -> return;
}
// Protection keys are currently supported on Hash MMU only
// Check if the system call is supported
//
// In older versions of libc, siginfo_t does not have si_pkey as
// a member.
//

