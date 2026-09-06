//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/svm.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Secure VM platform
//
// Copyright 2018 IBM Corporation
// Author: Anshuman Khandual <khandual@linux.vnet.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn init_svm() -> int __init {
    static int __init init_svm(void)
    {
    if (!is_secure_guest())
    return 0;
// Don't release the SWIOTLB buffer.
    ppc_swiotlb_enable = 1;
//
// Since the guest memory is inaccessible to the host, devices always
// need to use the SWIOTLB buffer for DMA even if dma_capable() says
// otherwise.
//
    ppc_swiotlb_flags |= SWIOTLB_ANY;
// Share the SWIOTLB buffer with the host.
    swiotlb_update_mem_attributes();
    return 0;
    }
    machine_early_initcall(pseries, init_svm);
#[no_mangle]
pub unsafe extern "C" fn set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_encrypted(unsigned long addr, int numpages)
    {
    if (!cc_platform_has(CC_ATTR_MEM_ENCRYPT))
    return 0;
    if (!PAGE_ALIGNED(addr))
    return -EINVAL;
    uv_unshare_page(PHYS_PFN(__pa(addr)), numpages);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_decrypted(unsigned long addr, int numpages)
    {
    if (!cc_platform_has(CC_ATTR_MEM_ENCRYPT))
    return 0;
    if (!PAGE_ALIGNED(addr))
    return -EINVAL;
    uv_share_page(PHYS_PFN(__pa(addr)), numpages);
    return 0;
    }
// There's one dispatch log per CPU.

    static struct page *dtl_page_store[NR_DTL_PAGE];
    static long dtl_nr_pages;
#[no_mangle]
unsafe extern "C" fn is_dtl_page_shared(page: *mut page) -> bool {
    static bool is_dtl_page_shared(struct page *page)
    {
    long i;
    for (i = 0; i < dtl_nr_pages; i++)
    if (dtl_page_store[i] == page)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn dtl_cache_ctor(addr: *mut c_void) {
    void dtl_cache_ctor(void *addr)
    {
    let mut pfn: c_ulong = PHYS_PFN(__pa(addr));
    struct page *page = pfn_to_page(pfn);
    if (!is_dtl_page_shared(page)) {
    dtl_page_store[dtl_nr_pages] = page;
    dtl_nr_pages++;
    WARN_ON(dtl_nr_pages >= NR_DTL_PAGE);
    uv_share_page(pfn, 1);
    }
    }
