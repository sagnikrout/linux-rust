//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/dsos.h
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
// Collection of DSOs as an array for iteration speed, but sorted for O(n)
// lookup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsos {
    pub lock: rw_semaphore,
    pub dsos: *mut dso,
    pub cnt: c_uint,
    pub allocated: c_uint,
    pub sorted: bool,
}

extern "C" {
    pub fn dsos__init(dsos: *mut dsos);
}
extern "C" {
    pub fn dsos__exit(dsos: *mut dsos);
}
extern "C" {
    pub fn __dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int;
}
extern "C" {
    pub fn dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int;
}
extern "C" {
    pub fn dsos__read_build_ids(dsos: *mut dsos, with_hits: bool) -> bool;
}
extern "C" {
    pub fn dsos__fprintf(dsos: *mut dsos, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn dsos__hit_all(dsos: *mut dsos) -> c_int;
}
extern "C" {
    pub fn dsos__for_each_dso(dsos: *mut dsos, dso: *mut *mut int (cb)(struct dso, data): *mut c_void, data: *mut c_void) -> c_int;
}
