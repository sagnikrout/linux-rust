//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/vDSO/parse_vdso.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// To use this vDSO parser, first call one of the vdso_init_* functions.
// If you've already parsed auxv, then pass the value of AT_SYSINFO_EHDR
// to vdso_init_from_sysinfo_ehdr.  Otherwise pass auxv to vdso_init_from_auxv.
// Then call vdso_sym for each symbol you want.  For example, to look up
// gettimeofday on x86_64, use:
//
// <some pointer> = vdso_sym("LINUX_2.6", "gettimeofday");
// or
// <some pointer> = vdso_sym("LINUX_2.6", "__vdso_gettimeofday");
//
// vdso_sym will return 0 if the symbol doesn't exist or if the init function
// failed or was not called.  vdso_sym is a little slow, so its return value
// should be cached.
//
// vdso_sym is threadsafe; the init functions are not.
//
// These are the prototypes:
//
extern "C" {
    pub fn vdso_init_from_sysinfo_ehdr(base: uintptr_t);
}
