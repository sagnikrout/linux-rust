//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/crash_dump_32.c
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

#[no_mangle]
pub unsafe extern "C" fn is_crashed_pfn_valid(pfn: c_ulong) -> bool {
    static inline bool is_crashed_pfn_valid(unsigned long pfn)
    {

//
// non-PAE kdump kernel executed from a PAE one will crop high pte
// bits and poke unwanted space counting again from address 0, we
// don't want that. pte must fit into unsigned long. In fact the
// test checks high 12 bits for being zero (pfn will be shifted left
// by PAGE_SHIFT).
//
    return pte_pfn(pfn_pte(pfn, __pgprot(0))) == pfn;

    return true;

    }
    ssize_t copy_oldmem_page(struct iov_iter *iter, unsigned long pfn, size_t csize,
    unsigned long offset)
    {
    void  *vaddr;
    if (!csize)
    return 0;
    if (!is_crashed_pfn_valid(pfn))
    return -EFAULT;
    vaddr = kmap_local_pfn(pfn);
    csize = copy_to_iter(vaddr + offset, csize, iter);
    kunmap_local(vaddr);
    return csize;
    }
