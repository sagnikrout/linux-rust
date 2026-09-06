//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-mem.c
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


// SPDX-License-Identifier: GPL-1.0+
//
// OHCI HCD (Host Controller Driver) for USB.
//
// (C) Copyright 1999 Roman Weissgaerber <weissg@vienna.at>
// (C) Copyright 2000-2002 David Brownell <dbrownell@users.sourceforge.net>
//
// This file is licenced under the GPL.
//
// -------------------------------------------------------------------------
//
// OHCI deals with three types of memory:
// - data used only by the HCD ... kmalloc is fine
// - async and periodic schedules, shared by HC and HCD ... these
// need to use dma_pool or dma_alloc_coherent
// - driver buffers, read/written by HC ... the hcd glue or the
// device driver provides us with dma addresses
//
// There's also "register" data, which is memory mapped.
// No memory seen by this driver (or any HCD) may be paged out.
//
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn ohci_hcd_init(ohci: *mut ohci_hcd) {
    static void ohci_hcd_init (struct ohci_hcd *ohci)
    {
    ohci.next_statechange = jiffies;
    spin_lock_init (&ohci.lock);
    INIT_LIST_HEAD (&ohci.pending);
    INIT_LIST_HEAD(&ohci.eds_in_use);
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn ohci_mem_init(ohci: *mut ohci_hcd) -> c_int {
    static int ohci_mem_init (struct ohci_hcd *ohci)
    {
//
// HCs with local memory allocate from localmem_pool so there's
// no need to create the below dma pools.
//
    if (ohci_to_hcd(ohci).localmem_pool)
    return 0;
    ohci.td_cache = dma_pool_create ("ohci_td",
    ohci_to_hcd(ohci).self.controller,
    sizeof (struct td),
    32 /* byte alignment */,
    0 /* no page-crossing issues */);
    if (!ohci.td_cache)
    return -ENOMEM;
    ohci.ed_cache = dma_pool_create ("ohci_ed",
    ohci_to_hcd(ohci).self.controller,
    sizeof (struct ed),
    16 /* byte alignment */,
    0 /* no page-crossing issues */);
    if (!ohci.ed_cache) {
    dma_pool_destroy (ohci.td_cache);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ohci_mem_cleanup(ohci: *mut ohci_hcd) {
    static void ohci_mem_cleanup (struct ohci_hcd *ohci)
    {
    dma_pool_destroy(ohci.td_cache);
    ohci.td_cache = core::ptr::null_mut();
    dma_pool_destroy(ohci.ed_cache);
    ohci.ed_cache = core::ptr::null_mut();
    }
// -------------------------------------------------------------------------
// ohci "done list" processing needs this mapping
    static inline struct td *
    dma_to_td (struct ohci_hcd *hc, dma_addr_t td_dma)
    {
    struct td *td;
    td_dma &= TD_MASK;
    td = hc.td_hash [TD_HASH_FUNC(td_dma)];
    while (td && td.td_dma != td_dma)
    td = td.td_hash;
    return td;
    }
// TDs ...
    static struct td *
    td_alloc (struct ohci_hcd *hc, gfp_t mem_flags)
    {
    dma_addr_t	dma;
    struct td	*td;
    struct usb_hcd	*hcd = ohci_to_hcd(hc);
    if (hcd.localmem_pool)
    td = gen_pool_dma_zalloc_align(hcd.localmem_pool,
    sizeof(*td), &dma, 32);
    else
    td = dma_pool_zalloc(hc.td_cache, mem_flags, &dma);
    if (td) {
// in case hc fetches it, make it look dead
    td.hwNextTD = cpu_to_hc32 (hc, dma);
    td.td_dma = dma;
// hashed in td_fill
    }
    return td;
    }
    static void
    td_free (struct ohci_hcd *hc, struct td *td)
    {
    struct td	**prev = &hc.td_hash [TD_HASH_FUNC (td.td_dma)];
    struct usb_hcd	*hcd = ohci_to_hcd(hc);
    while (*prev && *prev != td)
    prev = &(*prev).td_hash;
    if (*prev)
// prev = td->td_hash;
#[no_mangle]
pub unsafe extern "C" fn if(cpu_to_hc32(hc: (td->hwINFO &, 0: TD_DONE)) !=) -> else {
    else if ((td.hwINFO & cpu_to_hc32(hc, TD_DONE)) != 0)
    ohci_dbg (hc, "no hash for td %p\n", td);
    if (hcd.localmem_pool)
    gen_pool_free(hcd.localmem_pool, (unsigned long)td,
    sizeof(*td));
    else
    dma_pool_free(hc.td_cache, td, td.td_dma);
    }
// -------------------------------------------------------------------------
// EDs ...
    static struct ed *
    ed_alloc (struct ohci_hcd *hc, gfp_t mem_flags)
    {
    dma_addr_t	dma;
    struct ed	*ed;
    struct usb_hcd	*hcd = ohci_to_hcd(hc);
    if (hcd.localmem_pool)
    ed = gen_pool_dma_zalloc_align(hcd.localmem_pool,
    sizeof(*ed), &dma, 16);
    else
    ed = dma_pool_zalloc(hc.ed_cache, mem_flags, &dma);
    if (ed) {
    INIT_LIST_HEAD (&ed.td_list);
    ed.dma = dma;
    }
    return ed;
    }
    static void
    ed_free (struct ohci_hcd *hc, struct ed *ed)
    {
    struct usb_hcd	*hcd = ohci_to_hcd(hc);
    if (hcd.localmem_pool)
    gen_pool_free(hcd.localmem_pool, (unsigned long)ed,
    sizeof(*ed));
    else
    dma_pool_free(hc.ed_cache, ed, ed.dma);
    }
