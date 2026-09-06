//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/dma-noncoherent.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2023 Renesas Electronics Corp.
//

//
// struct riscv_nonstd_cache_ops - Structure for non-standard CMO function pointers
//
// @wback: Function pointer for cache writeback
// @inv: Function pointer for invalidating cache
// @wback_inv: Function pointer for flushing the cache (writeback + invalidating)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_nonstd_cache_ops {
    pub size): *mut *mut void (wback)(phys_addr_t paddr, size_t,
    pub size): *mut *mut void (inv)(phys_addr_t paddr, size_t,
    pub size): *mut *mut void (wback_inv)(phys_addr_t paddr, size_t,
}

extern "C" {
    pub fn riscv_noncoherent_register_cache_ops(ops: *const riscv_nonstd_cache_ops);
}
