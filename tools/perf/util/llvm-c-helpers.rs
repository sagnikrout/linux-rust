//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/llvm-c-helpers.h
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
pub const __PERF_LLVM_C_HELPERS: c_int = 1;
//
// Helpers to call into LLVM C++ code from C, for the parts that do not have
// C APIs.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llvm_a2l_frame {
    pub filename: *mut *mut c_char,
    pub funcname: *mut *mut c_char,
    pub line: c_uint,
}

//
// Implement addr2line() using libLLVM. LLVM is a C++ API, and
// many of the linux/ headers cannot be included in a C++ compile unit,
// so we need to make a little bridge code here. llvm_addr2line() will
// convert the inline frame information from LLVM's internal structures
// and put them into a flat array given in inline_frames. The caller
// is then responsible for taking that array and convert it into perf's
// regular inline frame structures (which depend on e.g. struct list_head).
//
// If the address could not be resolved, or an error occurred (e.g. OOM),
// returns 0. Otherwise, returns the number of inline frames (which means 1
// if the address was not part of an inlined function). If unwind_inlines
// is set and the return code is nonzero, inline_frames will be set to
// a newly allocated array with that length. The caller is then responsible
// for freeing both the strings and the array itself.
//
// Simple symbolizers for addresses; will convert something like
// 0x12345 to "func+0x123". Will return NULL if no symbol was found.
//
// The returned value must be freed by the caller, with free().
//

