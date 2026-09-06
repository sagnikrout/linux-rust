//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/usercopy_64.c
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
// User address space access functions.
//
// Copyright 1997 Andi Kleen <ak@muc.de>
// Copyright 1997 Linus Torvalds
// Copyright 2002 Andi Kleen <ak@suse.de>
//

//
// Zero Userspace
//

//
// clean_cache_range - write back a cache range with CLWB
// @addr:	virtual start address
// @size:	number of bytes to write back
//
// Write back a cache range using the CLWB (cache line write back)
// instruction. Note that @size is internally rounded up to be cache
// line size aligned.
//
#[no_mangle]
unsafe extern "C" fn clean_cache_range(addr: *mut c_void, size: usize) {
    static void clean_cache_range(void *addr, size_t size)
    {
    let mut x86_clflush_size: u16 = boot_cpu_data.x86_clflush_size;
    let mut clflush_mask: c_ulong = x86_clflush_size - 1;
    void *vend = addr + size;
    void *p;
    for (p = (void *)((unsigned long)addr & ~clflush_mask);
    p < vend; p += x86_clflush_size)
    clwb(p);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_wb_cache_pmem(addr: *mut c_void, size: usize) {
    void arch_wb_cache_pmem(void *addr, size_t size)
    {
    clean_cache_range(addr, size);
    }
    EXPORT_SYMBOL_GPL(arch_wb_cache_pmem);
#[no_mangle]
pub unsafe extern "C" fn copy_user_flushcache(dst: *mut c_void, src: *const void __user, size: usize) -> usize {
    size_t copy_user_flushcache(void *dst, const void __user *src, size_t size)
    {
    unsigned long flushed, dest = (unsigned long) dst;
    unsigned long rc;
    src = masked_user_access_begin(src);
    rc = copy_to_nontemporal(dst, ( const void *)src, size);
    user_access_end();
//
// copy_to_nontemporal() uses non-temporal stores for the bulk
// of the transfer, but we need to manually flush if the
// transfer is unaligned. A cached memory copy is used when
// destination or size is not naturally aligned. That is:
// - Require 8-byte alignment when size is 8 bytes or larger.
// - Require 4-byte alignment when size is 4 bytes.
//
    if (size < 8) {
    if (!IS_ALIGNED(dest, 4) || size != 4)
    clean_cache_range(dst, size);
    } else {
    if (!IS_ALIGNED(dest, 8)) {
    dest = ALIGN(dest, boot_cpu_data.x86_clflush_size);
    clean_cache_range(dst, 1);
    }
    flushed = dest - (unsigned long) dst;
    if (size > flushed && !IS_ALIGNED(size - flushed, 8))
    clean_cache_range(dst + size - 1, 1);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn __memcpy_flushcache(_dst: *mut c_void, _src: *const c_void, size: usize) {
    void __memcpy_flushcache(void *_dst, const void *_src, size_t size)
    {
    let mut dest: c_ulong = (unsigned long) _dst;
    let mut source: c_ulong = (unsigned long) _src;
// cache copy and flush to align dest
    if (!IS_ALIGNED(dest, 8)) {
    let mut len: usize = min_t(size_t, size, ALIGN(dest, 8) - dest);
    memcpy((void *) dest, (void *) source, len);
    clean_cache_range((void *) dest, len);
    dest += len;
    source += len;
    size -= len;
    if (!size)
    return;
    }
// 4x8 movnti loop
    while (size >= 32) {
    asm("movq    (%0), %%r8\n"
    "movq   8(%0), %%r9\n"
    "movq  16(%0), %%r10\n"
    "movq  24(%0), %%r11\n"
    "movnti  %%r8,   (%1)\n"
    "movnti  %%r9,  8(%1)\n"
    "movnti %%r10, 16(%1)\n"
    "movnti %%r11, 24(%1)\n"
    :: "r" (source), "r" (dest)
    : "memory", "r8", "r9", "r10", "r11");
    dest += 32;
    source += 32;
    size -= 32;
    }
// 1x8 movnti loop
    while (size >= 8) {
    asm("movq    (%0), %%r8\n"
    "movnti  %%r8,   (%1)\n"
    :: "r" (source), "r" (dest)
    : "memory", "r8");
    dest += 8;
    source += 8;
    size -= 8;
    }
// 1x4 movnti loop
    while (size >= 4) {
    asm("movl    (%0), %%r8d\n"
    "movnti  %%r8d,   (%1)\n"
    :: "r" (source), "r" (dest)
    : "memory", "r8");
    dest += 4;
    source += 4;
    size -= 4;
    }
// cache copy for remaining bytes
    if (size) {
    memcpy((void *) dest, (void *) source, size);
    clean_cache_range((void *) dest, size);
    }
    }
    EXPORT_SYMBOL_GPL(__memcpy_flushcache);
