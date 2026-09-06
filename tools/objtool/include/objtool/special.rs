//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/special.h
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
pub struct special_alt {
    pub list: list_head,
    pub group: bool,
    pub jump_or_nop: bool,
    pub key_addend: u8,
    pub orig_sec: *mut section,
    pub orig_off: c_ulong,
    pub new_sec: *mut section,
    pub new_off: c_ulong,
    pub /: *mut *mut unsigned int orig_len, new_len, feature; / group only,
}

extern "C" {
    pub fn special_get_alts(elf: *mut elf, alts: *mut list_head) -> c_int;
}
extern "C" {
    pub fn arch_handle_alternative(alt: *mut special_alt);
}
//
// Should the reloc at @offset -- the "new" (replacement) field of a special
// section group entry -- be ignored?  The meaning of a zero-length replacement
// is arch specific, so the arch decides.
//
extern "C" {
    pub fn arch_alt_ignore_new_reloc(sec: *mut section, offset: c_ulong) -> bool;
}
