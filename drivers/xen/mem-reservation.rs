//! Automatically rewritten from C to Rust
//! Source: drivers/xen/mem-reservation.c
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
// Xen memory reservation utilities.
//
// Copyright (c) 2003, B Dragovic
// Copyright (c) 2003-2004, M Williamson, K Fraser
// Copyright (c) 2005 Dan M. Smith, IBM Corporation
// Copyright (c) 2010 Daniel Kiper
// Copyright (c) 2018 Oleksandr Andrushchenko, EPAM Systems Inc.
//

    let mut xen_scrub_pages: bool __read_mostly = IS_ENABLED(CONFIG_XEN_SCRUB_PAGES_DEFAULT);
    core_param(xen_scrub_pages, xen_scrub_pages, bool, 0);
//
// Use one extent per PAGE_SIZE to avoid to break down the page into
// multiple frame.
//

    void __xenmem_reservation_va_mapping_update(unsigned long count,
    struct page **pages,
    xen_pfn_t *frames)
    {
    int i;
    for (i = 0; i < count; i++) {
    struct page *page = pages[i];
    let mut pfn: c_ulong = page_to_pfn(page);
    int ret;
    BUG_ON(!page);
//
// We don't support PV MMU when Linux and Xen is using
// different page granularity.
//
    BUILD_BUG_ON(XEN_PAGE_SIZE != PAGE_SIZE);
    set_phys_to_machine(pfn, frames[i]);
    ret = HYPERVISOR_update_va_mapping(
    (unsigned long)__va(pfn << PAGE_SHIFT),
    mfn_pte(frames[i], PAGE_KERNEL), 0);
    BUG_ON(ret);
    }
    }
    EXPORT_SYMBOL_GPL(__xenmem_reservation_va_mapping_update);
    void __xenmem_reservation_va_mapping_reset(unsigned long count,
    struct page **pages)
    {
    int i;
    for (i = 0; i < count; i++) {
    struct page *page = pages[i];
    let mut pfn: c_ulong = page_to_pfn(page);
    int ret;
//
// We don't support PV MMU when Linux and Xen are using
// different page granularity.
//
    BUILD_BUG_ON(XEN_PAGE_SIZE != PAGE_SIZE);
    ret = HYPERVISOR_update_va_mapping(
    (unsigned long)__va(pfn << PAGE_SHIFT),
    __pte_ma(0), 0);
    BUG_ON(ret);
    __set_phys_to_machine(pfn, INVALID_P2M_ENTRY);
    }
    }
    EXPORT_SYMBOL_GPL(__xenmem_reservation_va_mapping_reset);

// @frames is an array of PFNs
#[no_mangle]
pub unsafe extern "C" fn xenmem_reservation_increase(count: c_int, frames: *mut xen_pfn_t) -> c_int {
    int xenmem_reservation_increase(int count, xen_pfn_t *frames)
    {
    struct xen_memory_reservation reservation = {
    .address_bits = 0,
    .extent_order = EXTENT_ORDER,
    .domid        = DOMID_SELF
    };
// XENMEM_populate_physmap requires a PFN based on Xen granularity.
    set_xen_guest_handle(reservation.extent_start, frames);
    reservation.nr_extents = count;
    return HYPERVISOR_memory_op(XENMEM_populate_physmap, &reservation);
    }
    EXPORT_SYMBOL_GPL(xenmem_reservation_increase);
// @frames is an array of GFNs
#[no_mangle]
pub unsafe extern "C" fn xenmem_reservation_decrease(count: c_int, frames: *mut xen_pfn_t) -> c_int {
    int xenmem_reservation_decrease(int count, xen_pfn_t *frames)
    {
    struct xen_memory_reservation reservation = {
    .address_bits = 0,
    .extent_order = EXTENT_ORDER,
    .domid        = DOMID_SELF
    };
// XENMEM_decrease_reservation requires a GFN
    set_xen_guest_handle(reservation.extent_start, frames);
    reservation.nr_extents = count;
    return HYPERVISOR_memory_op(XENMEM_decrease_reservation, &reservation);
    }
    EXPORT_SYMBOL_GPL(xenmem_reservation_decrease);
