//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/um/shared/sysdep/faultinfo_32.h
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


//
// Copyright (C) 2004 Fujitsu Siemens Computers GmbH
// Author: Bodo Stroesser <bstroesser@fujitsu-siemens.com>
// Licensed under the GPL
//
// this structure contains the full arch-specific faultinfo
// from the traps.
// On i386, ptrace_faultinfo unfortunately doesn't provide
// all the info, since trap_no is missing.
// All common elements are defined at the same position in
// both structures, thus making it easy to copy the
// contents without knowledge about the structure elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct faultinfo {
    pub /: *mut *mut int error_code; / in ptrace_faultinfo misleadingly called is_write,
    pub /: *mut *mut unsigned long cr2; / in ptrace_faultinfo called addr,
    pub /: *mut *mut int trap_no; / missing in ptrace_faultinfo,
}

// This is Page Fault

pub const PTRACE_FULL_FAULTINFO: c_int = 0;

