//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/crash_dump.c
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
// Routines for doing kexec-based kdump
//
// Copyright (C) 2017 Linaro Limited
// Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
//

    ssize_t copy_oldmem_page(struct iov_iter *iter, unsigned long pfn,
    size_t csize, unsigned long offset)
    {
    void *vaddr;
    if (!csize)
    return 0;
    vaddr = memremap(__pfn_to_phys(pfn), PAGE_SIZE, MEMREMAP_WB);
    if (!vaddr)
    return -ENOMEM;
    csize = copy_to_iter(vaddr + offset, csize, iter);
    memunmap(vaddr);
    return csize;
    }
//
// elfcorehdr_read - read from ELF core header
// @buf: buffer where the data is placed
// @count: number of bytes to read
// @ppos: address in the memory
//
// This function reads @count bytes from elf core header which exists
// on crash dump kernel's memory.
//
#[no_mangle]
pub unsafe extern "C" fn elfcorehdr_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> isize {
    ssize_t elfcorehdr_read(char *buf, size_t count, u64 *ppos)
    {
    memcpy(buf, phys_to_virt((phys_addr_t)*ppos), count);
// ppos += count;
    return count;
    }
