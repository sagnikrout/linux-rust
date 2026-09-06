//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_probe_tmpl.h
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
// Traceprobe fetch helper inlines
//
// (u8 *)buf = (u8)val;
// (u16 *)buf = (u16)val;
// (u32 *)buf = (u32)val;
// TBD: 32bit signed
// (u64 *)buf = (u64)val;
// (unsigned long *)buf = val;
// (u8 *)buf <<= code->lshift;
// (u8 *)buf >>= code->rshift;
// (u16 *)buf <<= code->lshift;
// (u16 *)buf >>= code->rshift;
// (u32 *)buf <<= code->lshift;
// (u32 *)buf >>= code->rshift;
// (u64 *)buf <<= code->lshift;
// (u64 *)buf >>= code->rshift;
//
// These functions must be defined for each callsite.
// Return consumed dynamic data size (>= 0), or error (< 0).
// If dest is NULL, don't store result and return required dynamic data size.
//
extern "C" {
    pub fn fetch_store_strlen(addr: c_ulong) -> static nokprobe_inline int;
}
extern "C" {
    pub fn fetch_store_strlen_user(addr: c_ulong) -> static nokprobe_inline int;
}
//
// Fetch a null-terminated symbol string + offset. Caller MUST set *(u32 *)buf
// with max length and relative data location.
//
extern "C" {
    pub fn sprint_symbol(_arg: __dest, _arg: addr) -> return;
}
// common part of process_fetch_insn
// val = code->immediate;
// val = (unsigned long)current->comm;
// val = (unsigned long)code->data;
// val = (unsigned long)current;
// From the 2nd stage, routine is same
// 2nd stage: dereference memory if needed
// 3rd stage: store value to buffer
// 4th stage: modify stored value if needed
// the last stage: Loop on array
// (u32 *)dest = update_data_loc(loc, ret);
// Sum up total data length for dynamic arrays (strings)
// Store the value of each argument
// Point the dynamic data area if needed
// dl = make_data_loc(maxlen, dyndata - base);
