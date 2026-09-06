//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/extmem.h
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
// definitions for external memory segment support
// Copyright IBM Corp. 2003
//
// DCSS segment is defined as a contiguous range of pages using DEFSEG command.
// The range start and end is a page number with a value less than or equal to
// 0x7ffffff (see CP Commands and Utilities Reference).
//

// possible values for segment type as returned by segment_info
pub const SEG_TYPE_SW: c_int = 0;
pub const SEG_TYPE_EW: c_int = 1;
pub const SEG_TYPE_SR: c_int = 2;
pub const SEG_TYPE_ER: c_int = 3;
pub const SEG_TYPE_SN: c_int = 4;
pub const SEG_TYPE_EN: c_int = 5;
pub const SEG_TYPE_SC: c_int = 6;
pub const SEG_TYPE_EWEN: c_int = 7;
pub const SEGMENT_SHARED: c_int = 0;
pub const SEGMENT_EXCLUSIVE: c_int = 1;
extern "C" {
    pub fn segment_load(name: *mut c_char, segtype: c_int, addr: *mut c_ulong, length: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn segment_unload(name: *mut c_char);
}
extern "C" {
    pub fn segment_save(name: *mut c_char);
}
extern "C" {
    pub fn segment_type(name: *mut *mut c_char) -> c_int;
}
extern "C" {
    pub fn segment_modify_shared(name: *mut c_char, do_nonshared: c_int) -> c_int;
}
extern "C" {
    pub fn segment_warning(rc: c_int, seg_name: *mut c_char);
}

