//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/tools/vdso2c.h
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
// This file is included twice from vdso2c.c.  It generates code for 32-bit
// and 64-bit vDSOs.  We need both for 64-bit builds, since 32-bit vDSOs
// are built for 32-bit userspace.
//
// Extract a section from the input data into a standalone blob.  Used to
// capture kernel-only data that needs to persist indefinitely, e.g. the
// exception fixup tables, but only in the kernel, i.e. the section can
// be stripped from the final vDSO image.
//
// alt_sec = NULL, *extable_sec = NULL;
// Walk the segment table.
// Walk the dynamic table
// Walk the section table
// Walk the symbol table
//
// Careful: we use negative addresses, but
// st_value is unsigned, so we rely
// on syms[k] being a signed type of the
// correct width.
//
