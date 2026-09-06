//! Automatically rewritten from C to Rust
//! Source: arch/riscv/mm/pmem.c
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
// Copyright (C) 2022 Ventana Micro Systems Inc.
//

#[no_mangle]
pub unsafe extern "C" fn arch_wb_cache_pmem(addr: *mut c_void, size: usize) {
    void arch_wb_cache_pmem(void *addr, size_t size)
    {

    if (unlikely(noncoherent_cache_ops.wback)) {
    noncoherent_cache_ops.wback(virt_to_phys(addr), size);
    return;
    }

    ALT_CMO_OP(CLEAN, addr, size, riscv_cbom_block_size);
    }
    EXPORT_SYMBOL_GPL(arch_wb_cache_pmem);
#[no_mangle]
pub unsafe extern "C" fn arch_invalidate_pmem(addr: *mut c_void, size: usize) {
    void arch_invalidate_pmem(void *addr, size_t size)
    {

    if (unlikely(noncoherent_cache_ops.inv)) {
    noncoherent_cache_ops.inv(virt_to_phys(addr), size);
    return;
    }

    ALT_CMO_OP(INVAL, addr, size, riscv_cbom_block_size);
    }
    EXPORT_SYMBOL_GPL(arch_invalidate_pmem);
