//! Automatically rewritten from C Header to Rust Module
//! Source: tools/virtio/linux/kernel.h
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

// Macro flag: #define CONFIG_SMP

// generic data direction definitions
pub const READ: c_int = 0;
pub const WRITE: c_int = 1;
pub type dma_addr_t = c_ulonglong;
pub type __kernel_size_t = usize;
pub type __wsum = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page {
    pub dummy: c_ulonglong,
}

// Physical == Virtual

// Page address: Virtual / 4K

extern "C" {
    pub fn malloc(_arg: s) -> return;
}
extern "C" {
    pub fn kmalloc(s: *mut *mut n, _arg: gfp) -> return;
}

extern "C" {
    pub fn kmalloc(_arg: s, _arg: gfp) -> return;
}
extern "C" {
    pub fn realloc(_arg: p, _arg: s) -> return;
}

extern "C" {
    pub fn krealloc(_arg: p, _arg: bytes, _arg: gfp) -> return;
}

