//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/genksyms/genksyms.h
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
// Generate kernel symbol version hashes.
//
pub const MODUTILS_GENKSYMS_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_type {
    SYM_NORMAL, SYM_TYPEDEF, SYM_ENUM, SYM_STRUCT, SYM_UNION,
    SYM_ENUM_CONST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_status {
    STATUS_UNCHANGED, STATUS_DEFINED, STATUS_MODIFIED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct string_list {
    pub next: *mut string_list,
    pub tag: symbol_type,
    pub in_source_file: c_int,
    pub string: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol {
    pub hnode: hlist_node,
    pub name: *mut c_char,
    pub type: symbol_type,
    pub defn: *mut string_list,
    pub expansion_trail: *mut symbol,
    pub visited: *mut symbol,
    pub is_extern: c_int,
    pub is_declared: c_int,
    pub status: symbol_status,
    pub is_override: c_int,
}

extern "C" {
    pub fn export_symbol(: *const c_char);
}
extern "C" {
    pub fn free_node(list: *mut string_list);
}
extern "C" {
    pub fn free_list(s: *mut string_list, e: *mut string_list);
}
extern "C" {
    pub fn yylex() -> c_int;
}
extern "C" {
    pub fn yyparse() -> c_int;
}
extern "C" {
    pub fn error_with_pos(: *const c_char, ((format(printf: ...) __attribute__, _arg: 1, _arg: 2)));
}
// ----------------------------------------------------------------------

