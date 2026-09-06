//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kallsyms.h
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
// Rewritten and vastly simplified by Rusty Russell for in-kernel
// module loader:
// Copyright 2002 Rusty Russell <rusty@rustcorp.com.au> IBM Corporation
//

pub const KSYM_NAME_LEN: c_int = 512;

extern "C" {
    pub fn in_gate_area_no_mm(_arg: addr) -> return;
}
extern "C" {
    pub fn in_gate_area_no_mm(_arg: addr) -> return;
}
extern "C" {
    pub fn is_kernel(_arg: addr) -> return;
}
extern "C" {
    pub fn is_kernel_text(is_kernel_inittext(addr: addr) ||) -> return;
}

// How and when do we show kallsyms values?
extern "C" {
    pub fn kallsyms_show_value(cred: *const cred) -> bool;
}

extern "C" {
    pub fn kallsyms_sym_address(idx: c_int) -> c_ulong;
}
// Lookup the address for a symbol. Returns 0 if not found.
extern "C" {
    pub fn kallsyms_lookup_name(name: *const c_char) -> c_ulong;
}
// Lookup an address.  modname is set to NULL if it's in the kernel.
// Look up a kernel symbol and return it in a text buffer.
extern "C" {
    pub fn sprint_symbol(buffer: *mut c_char, address: c_ulong) -> c_int;
}
extern "C" {
    pub fn sprint_symbol_build_id(buffer: *mut c_char, address: c_ulong) -> c_int;
}
extern "C" {
    pub fn sprint_symbol_no_offset(buffer: *mut c_char, address: c_ulong) -> c_int;
}
extern "C" {
    pub fn sprint_backtrace(buffer: *mut c_char, address: c_ulong) -> c_int;
}
extern "C" {
    pub fn sprint_backtrace_build_id(buffer: *mut c_char, address: c_ulong) -> c_int;
}
extern "C" {
    pub fn lookup_symbol_name(addr: c_ulong, symname: *mut c_char) -> c_int;
}

// buffer = '\0';

