//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/libarena/spmc.h
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

pub const SPMC_ARR_BASESZ: c_int = 128;
pub const SPMC_ARR_ORDERS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spmc_arr {
    pub data: *mut u64 __arena,
    pub order: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spmc {
    pub cur: *mut volatile struct spmc_arr __arena,
    pub top: volatile u64,
    pub bottom: volatile u64,
    pub arr: [spmc_arr; SPMC_ARR_ORDERS],
}

extern "C" {
    pub fn spmc_owned_add(spmc: *mut spmc __arena, val: u64) -> c_int;
}
extern "C" {
    pub fn spmc_owned_remove(spmc: *mut spmc __arena, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn spmc_steal(spmc: *mut spmc __arena, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn spmc_destroy(spmc: *mut spmc __arena) -> c_int;
}
