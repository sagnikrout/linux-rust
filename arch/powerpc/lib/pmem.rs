//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/lib/pmem.c
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
// Copyright(c) 2017 IBM Corporation. All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn __clean_pmem_range(start: c_ulong, stop: c_ulong) {
    static inline void __clean_pmem_range(unsigned long start, unsigned long stop)
    {
    let mut shift: c_ulong = l1_dcache_shift();
    let mut bytes: c_ulong = l1_dcache_bytes();
    void *addr = (void *)(start & ~(bytes - 1));
    let mut size: c_ulong = stop - (unsigned long)addr + (bytes - 1);
    unsigned long i;
    for (i = 0; i < size >> shift; i++, addr += bytes)
    asm volatile(PPC_DCBSTPS(%0, %1): :"i"(0), "r"(addr): "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn __flush_pmem_range(start: c_ulong, stop: c_ulong) {
    static inline void __flush_pmem_range(unsigned long start, unsigned long stop)
    {
    let mut shift: c_ulong = l1_dcache_shift();
    let mut bytes: c_ulong = l1_dcache_bytes();
    void *addr = (void *)(start & ~(bytes - 1));
    let mut size: c_ulong = stop - (unsigned long)addr + (bytes - 1);
    unsigned long i;
    for (i = 0; i < size >> shift; i++, addr += bytes)
    asm volatile(PPC_DCBFPS(%0, %1): :"i"(0), "r"(addr): "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn clean_pmem_range(start: c_ulong, stop: c_ulong) {
    static inline void clean_pmem_range(unsigned long start, unsigned long stop)
    {
    if (cpu_has_feature(CPU_FTR_ARCH_207S))
    return __clean_pmem_range(start, stop);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_pmem_range(start: c_ulong, stop: c_ulong) {
    static inline void flush_pmem_range(unsigned long start, unsigned long stop)
    {
    if (cpu_has_feature(CPU_FTR_ARCH_207S))
    return __flush_pmem_range(start, stop);
    }
//
// CONFIG_ARCH_HAS_PMEM_API symbols
//
#[no_mangle]
pub unsafe extern "C" fn arch_wb_cache_pmem(addr: *mut c_void, size: usize) {
    void arch_wb_cache_pmem(void *addr, size_t size)
    {
    let mut start: c_ulong = (unsigned long) addr;
    clean_pmem_range(start, start + size);
    }
    EXPORT_SYMBOL_GPL(arch_wb_cache_pmem);
#[no_mangle]
pub unsafe extern "C" fn arch_invalidate_pmem(addr: *mut c_void, size: usize) {
    void arch_invalidate_pmem(void *addr, size_t size)
    {
    let mut start: c_ulong = (unsigned long) addr;
    flush_pmem_range(start, start + size);
    }
    EXPORT_SYMBOL_GPL(arch_invalidate_pmem);
//
// CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE symbols
//
    size_t copy_from_user_flushcache(void *dest, const void __user *src,
    size_t size)
    {
    unsigned long not_copied, start = (unsigned long) dest;
    src = mask_user_address(src);
    not_copied = __copy_from_user(dest, src, size);
    clean_pmem_range(start, start + size);
    return not_copied;
    }
#[no_mangle]
pub unsafe extern "C" fn memcpy_flushcache(dest: *mut c_void, src: *const c_void, size: usize) {
    void memcpy_flushcache(void *dest, const void *src, size_t size)
    {
    let mut start: c_ulong = (unsigned long) dest;
    memcpy(dest, src, size);
    clean_pmem_range(start, start + size);
    }
    EXPORT_SYMBOL(memcpy_flushcache);
