//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/fixmap.h
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
// fixmap.h: compile-time virtual memory allocation
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 1998 Ingo Molnar
//
// Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
// x86_32 and x86_64 integration by Gustavo F. Padovan, February 2009
// Break out common bits to asm-generic by Mark Salter, November 2013
//

//
// 'index to address' translation. If anyone tries to use the idx
// directly without translation, we catch the bug with a NULL-deference
// kernel oops. Illegal ranges of incoming indices are caught too.
//
extern "C" {
    pub fn __fix_to_virt(_arg: idx) -> return;
}
extern "C" {
    pub fn __virt_to_fix(_arg: vaddr) -> return;
}
//
// Provide some reasonable defaults for page flags.
// Not all architectures use all of these different types and some
// architectures use different names.
//

// Return a pointer with offset calculated

//
// Some hardware wants to get fixmapped without caching.
//

//
// Some fixmaps are for IO
//

