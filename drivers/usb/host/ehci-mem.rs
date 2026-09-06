//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-mem.c
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
// Copyright (c) 2001 by David Brownell
//
// this file is part of ehci-hcd.c
// -------------------------------------------------------------------------
//
// There's basically three types of memory:
// - data used only by the HCD ... kmalloc is fine
// - async and periodic schedules, shared by HC and HCD ... these
// need to use dma_pool or dma_alloc_coherent
// - driver buffers, read/written by HC ... single shot DMA mapped
//
// There's also "register" data (e.g. PCI or SOC), which is memory mapped.
// No memory seen by this driver is pageable.
//
// -------------------------------------------------------------------------
// Allocate the key transfer structures from the previously allocated pool
    static inline void ehci_qtd_init(struct ehci_hcd *ehci, struct ehci_qtd *qtd,
    dma_addr_t dma)
    {
    memset (qtd, 0, sizeof *qtd);
    qtd.qtd_dma = dma;
    qtd.hw_token = cpu_to_hc32(ehci, QTD_STS_HALT);
    qtd.hw_next = EHCI_LIST_END(ehci);
    qtd.hw_alt_next = EHCI_LIST_END(ehci);
    INIT_LIST_HEAD (&qtd.qtd_list);
    }
    static struct ehci_qtd *ehci_qtd_alloc (struct ehci_hcd *ehci, gfp_t flags)
    {
    struct ehci_qtd		*qtd;
    dma_addr_t		dma;
    qtd = dma_pool_alloc (ehci.qtd_pool, flags, &dma);
    if (qtd != core::ptr::null_mut()) {
    ehci_qtd_init(ehci, qtd, dma);
    }
    return qtd;
    }
#[no_mangle]
pub unsafe extern "C" fn ehci_qtd_free(ehci: *mut ehci_hcd, qtd: *mut ehci_qtd) {
    static inline void ehci_qtd_free (struct ehci_hcd *ehci, struct ehci_qtd *qtd)
    {
    dma_pool_free (ehci.qtd_pool, qtd, qtd.qtd_dma);
    }
#[no_mangle]
unsafe extern "C" fn qh_destroy(ehci: *mut ehci_hcd, qh: *mut ehci_qh) {
    static void qh_destroy(struct ehci_hcd *ehci, struct ehci_qh *qh)
    {
// clean qtds first, and know this is not linked
    if (!list_empty (&qh.qtd_list) || qh.qh_next.ptr) {
    ehci_dbg (ehci, "unused qh not empty!\n");
    BUG ();
    }
    if (qh.dummy)
    ehci_qtd_free (ehci, qh.dummy);
    dma_pool_free(ehci.qh_pool, qh.hw, qh.qh_dma);
    kfree(qh);
    }
    static struct ehci_qh *ehci_qh_alloc (struct ehci_hcd *ehci, gfp_t flags)
    {
    struct ehci_qh		*qh;
    dma_addr_t		dma;
    qh = kzalloc_obj(*qh, GFP_ATOMIC);
    if (!qh)
    goto done;
    qh.hw = (struct ehci_qh_hw *)
    dma_pool_zalloc(ehci.qh_pool, flags, &dma);
    if (!qh.hw)
    goto fail;
    qh.qh_dma = dma;
// INIT_LIST_HEAD (&qh->qh_list);
    INIT_LIST_HEAD (&qh.qtd_list);
    INIT_LIST_HEAD(&qh.unlink_node);
// dummy td enables safe urb queuing
    qh.dummy = ehci_qtd_alloc (ehci, flags);
    if (qh.dummy == core::ptr::null_mut()) {
    ehci_dbg (ehci, "no dummy td\n");
    goto fail1;
    }
    done:
    return qh;
    fail1:
    dma_pool_free(ehci.qh_pool, qh.hw, qh.qh_dma);
    fail:
    kfree(qh);
    return core::ptr::null_mut();
    }
// -------------------------------------------------------------------------
// The queue heads and transfer descriptors are managed from pools tied
// to each of the "per device" structures.
// This is the initialisation and cleanup code.
//
#[no_mangle]
unsafe extern "C" fn ehci_mem_cleanup(ehci: *mut ehci_hcd) {
    static void ehci_mem_cleanup (struct ehci_hcd *ehci)
    {
    if (ehci.async)
    qh_destroy(ehci, ehci.async);
    ehci.async = core::ptr::null_mut();
    if (ehci.dummy)
    qh_destroy(ehci, ehci.dummy);
    ehci.dummy = core::ptr::null_mut();
// DMA consistent memory and pools
    dma_pool_destroy(ehci.qtd_pool);
    ehci.qtd_pool = core::ptr::null_mut();
    dma_pool_destroy(ehci.qh_pool);
    ehci.qh_pool = core::ptr::null_mut();
    dma_pool_destroy(ehci.itd_pool);
    ehci.itd_pool = core::ptr::null_mut();
    dma_pool_destroy(ehci.sitd_pool);
    ehci.sitd_pool = core::ptr::null_mut();
    if (ehci.periodic)
    dma_free_coherent(ehci_to_hcd(ehci).self.sysdev,
    ehci.periodic_size * sizeof (u32),
    ehci.periodic, ehci.periodic_dma);
    ehci.periodic = core::ptr::null_mut();
// shadow periodic table
    kfree(ehci.pshadow);
    ehci.pshadow = core::ptr::null_mut();
    }
// remember to add cleanup code (above) if you add anything here
#[no_mangle]
unsafe extern "C" fn ehci_mem_init(ehci: *mut ehci_hcd, flags: gfp_t) -> c_int {
    static int ehci_mem_init (struct ehci_hcd *ehci, gfp_t flags)
    {
    int i;
// QTDs for control/bulk/intr transfers
    ehci.qtd_pool = dma_pool_create ("ehci_qtd",
    ehci_to_hcd(ehci).self.sysdev,
    sizeof (struct ehci_qtd),
    32 /* byte alignment (for hw parts) */,
    4096 /* can't cross 4K */);
    if (!ehci.qtd_pool) {
    goto fail;
    }
// QHs for control/bulk/intr transfers
    ehci.qh_pool = dma_pool_create ("ehci_qh",
    ehci_to_hcd(ehci).self.sysdev,
    sizeof(struct ehci_qh_hw),
    32 /* byte alignment (for hw parts) */,
    4096 /* can't cross 4K */);
    if (!ehci.qh_pool) {
    goto fail;
    }
    ehci.async = ehci_qh_alloc (ehci, flags);
    if (!ehci.async) {
    goto fail;
    }
// ITD for high speed ISO transfers
    ehci.itd_pool = dma_pool_create ("ehci_itd",
    ehci_to_hcd(ehci).self.sysdev,
    sizeof (struct ehci_itd),
    32 /* byte alignment (for hw parts) */,
    4096 /* can't cross 4K */);
    if (!ehci.itd_pool) {
    goto fail;
    }
// SITD for full/low speed split ISO transfers
    ehci.sitd_pool = dma_pool_create ("ehci_sitd",
    ehci_to_hcd(ehci).self.sysdev,
    sizeof (struct ehci_sitd),
    32 /* byte alignment (for hw parts) */,
    4096 /* can't cross 4K */);
    if (!ehci.sitd_pool) {
    goto fail;
    }
// Hardware periodic table
    ehci.periodic = (__le32 *)
    dma_alloc_coherent(ehci_to_hcd(ehci).self.sysdev,
    ehci.periodic_size * sizeof(__le32),
    &ehci.periodic_dma, flags);
    if (ehci.periodic == core::ptr::null_mut()) {
    goto fail;
    }
    if (ehci.use_dummy_qh) {
    struct ehci_qh_hw	*hw;
    ehci.dummy = ehci_qh_alloc(ehci, flags);
    if (!ehci.dummy)
    goto fail;
    hw = ehci.dummy.hw;
    hw.hw_next = EHCI_LIST_END(ehci);
    hw.hw_qtd_next = EHCI_LIST_END(ehci);
    hw.hw_alt_next = EHCI_LIST_END(ehci);
    ehci.dummy.hw = hw;
    for (i = 0; i < ehci.periodic_size; i++)
    ehci.periodic[i] = cpu_to_hc32(ehci,
    ehci.dummy.qh_dma);
    } else {
    for (i = 0; i < ehci.periodic_size; i++)
    ehci.periodic[i] = EHCI_LIST_END(ehci);
    }
// software shadow of hardware table
    ehci.pshadow = kcalloc(ehci.periodic_size, sizeof(void *), flags);
    if (ehci.pshadow != core::ptr::null_mut())
    return 0;
    fail:
    ehci_dbg (ehci, "couldn't init memory\n");
    ehci_mem_cleanup (ehci);
    return -ENOMEM;
    }
