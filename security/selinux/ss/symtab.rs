//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/symtab.h
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
// A symbol table (symtab) maintains associations between symbol
// strings and datum values.  The type of the datum values
// is arbitrary.  The symbol table type is implemented
// using the hash table type (hashtab).
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symtab {
    pub /: *mut *mut hashtab table; / hash table (keyed on a string),
    pub /: *mut *mut u32 nprim; / number of primary names in table,
}

extern "C" {
    pub fn symtab_init(s: *mut symtab, size: u32) -> c_int;
}
extern "C" {
    pub fn symtab_insert(s: *mut symtab, name: *mut c_char, datum: *mut c_void) -> c_int;
}
