//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/page-states.c
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
// Copyright IBM Corp. 2008
//
// Guest page hinting for unused pages.
//
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

    int __bootdata_preserved(cmma_flag);
    EXPORT_SYMBOL(cmma_flag);
#[no_mangle]
pub unsafe extern "C" fn arch_free_page(page: *mut page, order: c_int) {
    void arch_free_page(struct page *page, int order)
    {
    if (!cmma_flag)
    return;
    __set_page_unused(page_to_virt(page), 1UL << order);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_alloc_page(page: *mut page, order: c_int) {
    void arch_alloc_page(struct page *page, int order)
    {
    if (!cmma_flag)
    return;
    if (cmma_flag < 2)
    __set_page_stable_dat(page_to_virt(page), 1UL << order);
    else
    __set_page_stable_nodat(page_to_virt(page), 1UL << order);
    }
