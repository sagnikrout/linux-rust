//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/map_symbol.h
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
pub const __PERF_MAP_SYMBOL: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_symbol {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_map_symbol {
    pub ms: map_symbol,
    pub addr: u64,
    pub al_addr: u64,
    pub al_level: c_char,
    pub phys_addr: u64,
    pub data_page_size: u64,
}

extern "C" {
    pub fn map_symbol__exit(ms: *mut map_symbol);
}
extern "C" {
    pub fn addr_map_symbol__exit(ams: *mut addr_map_symbol);
}
extern "C" {
    pub fn map_symbol__copy(dst: *mut map_symbol, src: *mut map_symbol);
}
extern "C" {
    pub fn addr_map_symbol__copy(dst: *mut addr_map_symbol, src: *mut addr_map_symbol);
}
