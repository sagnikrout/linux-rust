//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/builtin.h
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
// Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opts {
// actions:
    pub checksum: bool,
    pub disas: *const c_char,
    pub dump_orc: bool,
    pub hack_jump_label: bool,
    pub hack_noinstr: bool,
    pub hack_skylake: bool,
    pub ibt: bool,
    pub klp_symids: bool,
    pub mcount: bool,
    pub noabs: bool,
    pub noinstr: bool,
    pub orc: bool,
    pub prefix: c_int,
    pub retpoline: bool,
    pub rethunk: bool,
    pub unret: bool,
    pub sls: bool,
    pub stackval: bool,
    pub static_call: bool,
    pub uaccess: bool,
// options:
    pub backtrace: bool,
    pub backup: bool,
    pub cfi: bool,
    pub debug_checksum: *const c_char,
    pub dryrun: bool,
    pub fineibt: bool,
    pub link: bool,
    pub mnop: bool,
    pub module: bool,
    pub no_unreachable: bool,
    pub output: *const c_char,
    pub sec_address: bool,
    pub stats: bool,
    pub trace: *const c_char,
    pub verbose: bool,
    pub werror: bool,
    pub wide: bool,
}

extern "C" {
    pub fn cmd_parse_options(argc: c_int, argv: *const c_char, usage[]: *const *const c_char) -> c_int;
}
extern "C" {
    pub fn objtool_run(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn make_backup() -> c_int;
}
extern "C" {
    pub fn cmd_klp(argc: c_int, argv: *const c_char) -> c_int;
}
