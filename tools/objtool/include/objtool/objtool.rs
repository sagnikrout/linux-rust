//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/objtool.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2020 Matt Helsley <mhelsley@vmware.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_state {
    pub clean: bool,
    pub targets: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct objtool_file {
    pub elf: *mut elf,
    pub 20): DECLARE_HASHTABLE(insn_hash,,
    pub retpoline_call_list: list_head,
    pub return_thunk_list: list_head,
    pub static_call_list: list_head,
    pub mcount_loc_list: list_head,
    pub endbr_list: list_head,
    pub call_list: list_head,
    pub klp: bool ignore_unreachables, hints, rodata,,
    pub nr_endbr: c_uint,
    pub nr_endbr_int: c_uint,
    pub jl_long: unsigned long jl_short,,
    pub jl_nop_long: unsigned long jl_nop_short,,
    pub pv_ops: *mut pv_state,
}

extern "C" {
    pub fn init_signal_handler() -> c_int;
}
extern "C" {
    pub fn objtool_pv_add(file: *mut objtool_file, idx: c_int, func: *mut symbol) -> c_int;
}
extern "C" {
    pub fn check(file: *mut objtool_file) -> c_int;
}
extern "C" {
    pub fn orc_dump(objname: *const c_char) -> c_int;
}
extern "C" {
    pub fn orc_create(file: *mut objtool_file) -> c_int;
}
