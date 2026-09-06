//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/set_memory.h
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
pub const SET_MEMORY_RO: c_int = 0;
pub const SET_MEMORY_RW: c_int = 1;
pub const SET_MEMORY_NX: c_int = 2;
pub const SET_MEMORY_X: c_int = 3;

pub const SET_MEMORY_ROX: c_int = 6;
extern "C" {
    pub fn change_memory_attr(addr: c_ulong, numpages: c_int, action: c_long) -> c_int;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_RO) -> return;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_RW) -> return;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_NX) -> return;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_X) -> return;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_NP) -> return;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_P) -> return;
}
extern "C" {
    pub fn change_memory_attr(_arg: addr, _arg: numpages, _arg: SET_MEMORY_ROX) -> return;
}

