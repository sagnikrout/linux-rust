//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/subcmd/help.h
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
#[derive(Copy, Clone)]
pub struct cmdnames {
    pub alloc: usize,
    pub cnt: usize,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdname {
    pub /: *mut *mut size_t len; / also used for similarity index in help.c,
    pub name: [c_char; ],
    pub names: *mut },
}

extern "C" {
    pub fn add_cmdname(cmds: *mut cmdnames, name: *const c_char, len: usize);
}
extern "C" {
    pub fn clean_cmdnames(cmds: *mut cmdnames);
}
extern "C" {
    pub fn cmdname_compare(a: *const c_void, b: *const c_void) -> c_int;
}
extern "C" {
    pub fn uniq(cmds: *mut cmdnames);
}
// Here we require that excludes is a sorted list.
extern "C" {
    pub fn exclude_cmds(cmds: *mut cmdnames, excludes: *mut cmdnames);
}
extern "C" {
    pub fn is_in_cmdlist(c: *mut cmdnames, s: *const c_char) -> c_int;
}
