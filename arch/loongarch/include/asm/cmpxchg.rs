//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/cmpxchg.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

// Mask value to the correct size.
//
// Calculate a shift & mask that correspond to the value we wish to
// exchange within the naturally aligned 4 byte integerthat includes
// it.
//
// Calculate a pointer to the naturally aligned 4 byte integer that
// includes our byte of interest, and load its value.
//
extern "C" {
    pub fn __xchg_small()ptr: *mut (volatile void, _arg: x, _arg: size) -> return;
}

extern "C" {
    pub fn __xchg_amo_asm(_arg: "amswap_db.w", )ptr: *mut (volatile u32, _arg: (u32)x) -> return;
}

extern "C" {
    pub fn __xchg_llsc_asm(_arg: "ll.w", _arg: "sc.w", )ptr: *mut (volatile u32, _arg: (u32)x) -> return;
}

extern "C" {
    pub fn __xchg_amo_asm(_arg: "amswap_db.d", )ptr: *mut (volatile u64, _arg: (u64)x) -> return;
}

extern "C" {
    pub fn __xchg_llsc_asm(_arg: "ll.d", _arg: "sc.d", )ptr: *mut (volatile u64, _arg: (u64)x) -> return;
}

// Mask inputs to the correct size.
//
// Calculate a shift & mask that correspond to the value we wish to
// compare & exchange within the naturally aligned 4 byte integer
// that includes it.
//
// Calculate a pointer to the naturally aligned 4 byte integer that
// includes our byte of interest, and load its value.
//
extern "C" {
    pub fn __cmpxchg_small(_arg: ptr, _arg: old, _arg: new, _arg: size) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __u128_halves {
    pub full: u128,
    pub low: u64,
    pub high: u64,
}

