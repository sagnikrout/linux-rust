//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/crash_dump_64.c
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
// Memory preserving reboot related code.
//
// Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
// Copyright (C) IBM Corporation, 2004. All rights reserved
//

    static ssize_t __copy_oldmem_page(struct iov_iter *iter, unsigned long pfn,
    size_t csize, unsigned long offset,
    bool encrypted)
    {
    void  *vaddr;
    if (!csize)
    return 0;
    if (encrypted)
    vaddr = ( void *)ioremap_encrypted(pfn << PAGE_SHIFT, PAGE_SIZE);
    else
    vaddr = ( void *)ioremap_cache(pfn << PAGE_SHIFT, PAGE_SIZE);
    if (!vaddr)
    return -ENOMEM;
    csize = copy_to_iter(vaddr + offset, csize, iter);
    iounmap((void __iomem *)vaddr);
    return csize;
    }
    ssize_t copy_oldmem_page(struct iov_iter *iter, unsigned long pfn, size_t csize,
    unsigned long offset)
    {
    return __copy_oldmem_page(iter, pfn, csize, offset, false);
    }
//
// copy_oldmem_page_encrypted - same as copy_oldmem_page() above but ioremap the
// memory with the encryption mask set to accommodate kdump on SME-enabled
// machines.
//
    ssize_t copy_oldmem_page_encrypted(struct iov_iter *iter, unsigned long pfn,
    size_t csize, unsigned long offset)
    {
    return __copy_oldmem_page(iter, pfn, csize, offset, true);
    }
#[no_mangle]
pub unsafe extern "C" fn elfcorehdr_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> isize {
    ssize_t elfcorehdr_read(char *buf, size_t count, u64 *ppos)
    {
    let mut kvec: kvec = { .iov_base = buf, .iov_len = count };
    struct iov_iter iter;
    iov_iter_kvec(&iter, ITER_DEST, &kvec, 1, count);
    return read_from_oldmem(&iter, count, ppos,
    cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT));
    }
