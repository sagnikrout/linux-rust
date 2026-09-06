//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/return_address.c
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
// This code come from arch/arm64/kernel/return_address.c
//
// Copyright (C) 2023 SiFive.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct return_address_data {
    pub level: c_uint,
    pub addr: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn save_return_addr(d: *mut c_void, pc: c_ulong) -> bool {
    static bool save_return_addr(void *d, unsigned long pc)
    {
    struct return_address_data *data = d;
    if (!data.level) {
    data.addr = (void *)pc;
    return false;
    }
    --data.level;
    return true;
    }
    NOKPROBE_SYMBOL(save_return_addr);
    noinline void *return_address(unsigned int level)
    {
    struct return_address_data data;
    data.level = level + 3;
    data.addr = core::ptr::null_mut();
    arch_stack_walk(save_return_addr, &data, current, core::ptr::null_mut());
    if (!data.level)
    return data.addr;
    else
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(return_address);
    NOKPROBE_SYMBOL(return_address);
