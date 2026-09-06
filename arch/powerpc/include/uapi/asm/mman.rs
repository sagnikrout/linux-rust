//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/mman.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const PROT_SAO: c_uint = 0x10		/* Strong Access Ordering */;

pub const MAP_NORESERVE: c_uint = 0x40            /* don't reserve swap pages */;
pub const MAP_LOCKED: c_uint = 0x80;
pub const MAP_GROWSDOWN: c_uint = 0x0100		/* stack-like segment */;
pub const MAP_DENYWRITE: c_uint = 0x0800		/* ETXTBSY */;
pub const MAP_EXECUTABLE: c_uint = 0x1000		/* mark it as an executable */;
pub const MCL_CURRENT: c_uint = 0x2000          /* lock all currently mapped pages */;
pub const MCL_FUTURE: c_uint = 0x4000          /* lock all additions to address space */;
pub const MCL_ONFAULT: c_uint = 0x8000		/* lock all pages that are faulted in */;
// Override any generic PKEY permission defines
pub const PKEY_DISABLE_EXECUTE: c_uint = 0x4;

