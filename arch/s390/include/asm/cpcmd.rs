//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/cpcmd.h
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
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
// Christian Borntraeger (cborntra@de.ibm.com),
//
// the lowlevel function for cpcmd
//
extern "C" {
    pub fn __cpcmd(cmd: *const c_char, response: *mut c_char, rlen: c_int, response_code: *mut c_int) -> c_int;
}
//
// cpcmd is the in-kernel interface for issuing CP commands
//
// cmd:		null-terminated command string, max 240 characters
// response:	response buffer for VM's textual response
// rlen:	size of the response buffer, cpcmd will not exceed this size
// but will cap the output, if its too large. Everything that
// did not fit into the buffer will be silently dropped
// response_code: return pointer for VM's error code
// return value: the size of the response. The caller can check if the buffer
// was large enough by comparing the return value and rlen
// NOTE: If the response buffer is not in real storage, cpcmd can sleep
//
extern "C" {
    pub fn cpcmd(cmd: *const c_char, response: *mut c_char, rlen: c_int, response_code: *mut c_int) -> c_int;
}
