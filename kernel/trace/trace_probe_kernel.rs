//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_probe_kernel.h
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
// This depends on trace_probe.h, but can not include it due to
// the way trace_probe_tmpl.h is used by trace_kprobe.c and trace_eprobe.c.
// Which means that any other user must include trace_probe.h before including
// this file.
//
// Return the length of string -- including null terminal byte
extern "C" {
    pub fn strnlen_user_nofault(_arg: uaddr, _arg: MAX_STRING_SIZE) -> return;
}
// Return the length of string -- including null terminal byte

extern "C" {
    pub fn fetch_store_strlen_user(_arg: addr) -> return;
}

// (u32 *)dest = make_data_loc(ret, __dest - base);
//
// Fetch a null-terminated string from user. Caller MUST set *(u32 *)buf
// with max length and relative data location.
//
// Fetch a null-terminated string. Caller MUST set *(u32 *)buf with max
// length and relative data location.
//

extern "C" {
    pub fn fetch_store_string_user(_arg: addr, _arg: dest, _arg: base) -> return;
}

//
// Try to get string again, since the string can be changed while
// probing.
//
extern "C" {
    pub fn copy_from_user_nofault(_arg: dest, _arg: uaddr, _arg: size) -> return;
}

extern "C" {
    pub fn probe_mem_read_user(_arg: dest, _arg: src, _arg: size) -> return;
}

extern "C" {
    pub fn copy_from_kernel_nofault(_arg: dest, _arg: src, _arg: size) -> return;
}
