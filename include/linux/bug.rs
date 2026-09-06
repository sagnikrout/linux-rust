//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bug.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bug_trap_type {
    BUG_TRAP_TYPE_NONE = 0,
    BUG_TRAP_TYPE_WARN = 1,
    BUG_TRAP_TYPE_BUG = 2,
}

extern "C" {
    pub fn report_bug(bug_addr: c_ulong, regs: *mut pt_regs) -> bug_trap_type;
}
extern "C" {
    pub fn report_bug_entry(bug: *mut bug_entry, regs: *mut pt_regs) -> bug_trap_type;
}
// These are defined by the architecture
extern "C" {
    pub fn is_valid_bugaddr(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn generic_bug_clear_once();
}

// file = NULL;
// line = 0;

extern "C" {
    pub fn mem_dump_obj(object: *mut c_void);
}

//
// Since detected data corruption should stop operation on the affected
// structures. Return value must be checked and sanely acted on by caller.
//

