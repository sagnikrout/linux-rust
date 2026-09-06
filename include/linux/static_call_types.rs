//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/static_call_types.h
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
// Flags in the low bits of static_call_site::key.
//

//
// The static call site table needs to be created by external tooling (objtool
// or a compiler plugin).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_call_site {
    pub addr: i32,
    pub key: i32,
}

extern "C" {
    pub fn typeof(STATIC_CALL_TRAMP(name: func)) -> extern;
}

//
// __ADDRESSABLE() is used to ensure the key symbol doesn't get stripped from
// the symbol table so that objtool can reference it when it generates the
// .static_call_sites section.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_call_key {
    pub func: *mut c_void,
// bit 0: 0 = mods, 1 = sites
    pub type: c_ulong,
    pub mods: *mut static_call_mod,
    pub sites: *mut static_call_site,
}

// Macro flag: #define __STATIC_CALL_ADDRESSABLE(name)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_call_key {
    pub func: *mut c_void,
}

// Macro flag: #define __STATIC_CALL_MOD_ADDRESSABLE(name)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_call_key {
    pub func: *mut c_void,
}

