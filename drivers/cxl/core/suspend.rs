//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/suspend.c
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
// Copyright(c) 2022 Intel Corporation. All rights reserved.

    static atomic_t mem_active;
#[no_mangle]
pub unsafe extern "C" fn cxl_mem_active() -> bool {
    bool cxl_mem_active(void)
    {
    return atomic_read(&mem_active) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_mem_active_inc() {
    void cxl_mem_active_inc(void)
    {
    atomic_inc(&mem_active);
    }
    EXPORT_SYMBOL_NS_GPL(cxl_mem_active_inc, "CXL");
#[no_mangle]
pub unsafe extern "C" fn cxl_mem_active_dec() {
    void cxl_mem_active_dec(void)
    {
    atomic_dec(&mem_active);
    }
    EXPORT_SYMBOL_NS_GPL(cxl_mem_active_dec, "CXL");
