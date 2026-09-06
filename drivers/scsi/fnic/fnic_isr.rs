//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/fnic/fnic_isr.c
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn fnic_isr_legacy(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fnic_isr_legacy(int irq, void *data)
    {
    struct fnic *fnic = data;
    u32 pba;
    let mut work_done: c_ulong = 0;
    pba = vnic_intr_legacy_pba(fnic.legacy_pba);
    if (!pba)
    return IRQ_NONE;
    fnic.fnic_stats.misc_stats.last_isr_time = jiffies;
    atomic64_inc(&fnic.fnic_stats.misc_stats.isr_count);
    if (pba & (1 << FNIC_INTX_NOTIFY)) {
    vnic_intr_return_all_credits(&fnic.intr[FNIC_INTX_NOTIFY]);
    fnic_handle_link_event(fnic);
    }
    if (pba & (1 << FNIC_INTX_ERR)) {
    vnic_intr_return_all_credits(&fnic.intr[FNIC_INTX_ERR]);
    fnic_log_q_error(fnic);
    }
    if (pba & (1 << FNIC_INTX_DUMMY)) {
    atomic64_inc(&fnic.fnic_stats.misc_stats.intx_dummy);
    vnic_intr_return_all_credits(&fnic.intr[FNIC_INTX_DUMMY]);
    }
    if (pba & (1 << FNIC_INTX_WQ_RQ_COPYWQ)) {
    work_done += fnic_wq_copy_cmpl_handler(fnic, io_completions, FNIC_MQ_CQ_INDEX);
    work_done += fnic_wq_cmpl_handler(fnic, -1);
    work_done += fnic_rq_cmpl_handler(fnic, -1);
    vnic_intr_return_credits(&fnic.intr[FNIC_INTX_WQ_RQ_COPYWQ],
    work_done,
    1 /* unmask intr */,
    1 /* reset intr timer */);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fnic_isr_msi(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fnic_isr_msi(int irq, void *data)
    {
    struct fnic *fnic = data;
    let mut work_done: c_ulong = 0;
    fnic.fnic_stats.misc_stats.last_isr_time = jiffies;
    atomic64_inc(&fnic.fnic_stats.misc_stats.isr_count);
    work_done += fnic_wq_copy_cmpl_handler(fnic, io_completions, FNIC_MQ_CQ_INDEX);
    work_done += fnic_wq_cmpl_handler(fnic, -1);
    work_done += fnic_rq_cmpl_handler(fnic, -1);
    vnic_intr_return_credits(&fnic.intr[0],
    work_done,
    1 /* unmask intr */,
    1 /* reset intr timer */);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fnic_isr_msix_rq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fnic_isr_msix_rq(int irq, void *data)
    {
    struct fnic *fnic = data;
    let mut rq_work_done: c_ulong = 0;
    fnic.fnic_stats.misc_stats.last_isr_time = jiffies;
    atomic64_inc(&fnic.fnic_stats.misc_stats.isr_count);
    rq_work_done = fnic_rq_cmpl_handler(fnic, -1);
    vnic_intr_return_credits(&fnic.intr[FNIC_MSIX_RQ],
    rq_work_done,
    1 /* unmask intr */,
    1 /* reset intr timer */);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fnic_isr_msix_wq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fnic_isr_msix_wq(int irq, void *data)
    {
    struct fnic *fnic = data;
    let mut wq_work_done: c_ulong = 0;
    fnic.fnic_stats.misc_stats.last_isr_time = jiffies;
    atomic64_inc(&fnic.fnic_stats.misc_stats.isr_count);
    wq_work_done = fnic_wq_cmpl_handler(fnic, -1);
    vnic_intr_return_credits(&fnic.intr[FNIC_MSIX_WQ],
    wq_work_done,
    1 /* unmask intr */,
    1 /* reset intr timer */);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fnic_isr_msix_wq_copy(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fnic_isr_msix_wq_copy(int irq, void *data)
    {
    struct fnic *fnic = data;
    let mut wq_copy_work_done: c_ulong = 0;
    int i;
    fnic.fnic_stats.misc_stats.last_isr_time = jiffies;
    atomic64_inc(&fnic.fnic_stats.misc_stats.isr_count);
    i = irq - fnic.msix[0].irq_num;
    if (i >= fnic.wq_copy_count + fnic.copy_wq_base ||
    i < 0 || fnic.msix[i].irq_num != irq) {
    for (i = fnic.copy_wq_base; i < fnic.wq_copy_count + fnic.copy_wq_base ; i++) {
    if (fnic.msix[i].irq_num == irq)
    break;
    }
    }
    wq_copy_work_done = fnic_wq_copy_cmpl_handler(fnic, io_completions, i);
    vnic_intr_return_credits(&fnic.intr[i],
    wq_copy_work_done,
    1 /* unmask intr */,
    1 /* reset intr timer */);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fnic_isr_msix_err_notify(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fnic_isr_msix_err_notify(int irq, void *data)
    {
    struct fnic *fnic = data;
    fnic.fnic_stats.misc_stats.last_isr_time = jiffies;
    atomic64_inc(&fnic.fnic_stats.misc_stats.isr_count);
    vnic_intr_return_all_credits(&fnic.intr[fnic.err_intr_offset]);
    fnic_log_q_error(fnic);
    fnic_handle_link_event(fnic);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn fnic_free_intr(fnic: *mut fnic) {
    void fnic_free_intr(struct fnic *fnic)
    {
    int i;
    switch (vnic_dev_get_intr_mode(fnic.vdev)) {
    case VNIC_DEV_INTR_MODE_INTX:
    case VNIC_DEV_INTR_MODE_MSI:
    free_irq(pci_irq_vector(fnic.pdev, 0), fnic);
    break;
    case VNIC_DEV_INTR_MODE_MSIX:
    for (i = 0; i < ARRAY_SIZE(fnic.msix); i++)
    if (fnic.msix[i].requested)
    free_irq(pci_irq_vector(fnic.pdev, i),
    fnic.msix[i].devid);
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fnic_request_intr(fnic: *mut fnic) -> c_int {
    int fnic_request_intr(struct fnic *fnic)
    {
    let mut err: c_int = 0;
    int i;
    switch (vnic_dev_get_intr_mode(fnic.vdev)) {
    case VNIC_DEV_INTR_MODE_INTX:
    err = request_irq(pci_irq_vector(fnic.pdev, 0),
    &fnic_isr_legacy, IRQF_SHARED, DRV_NAME, fnic);
    break;
    case VNIC_DEV_INTR_MODE_MSI:
    err = request_irq(pci_irq_vector(fnic.pdev, 0), &fnic_isr_msi,
    0, fnic.name, fnic);
    break;
    case VNIC_DEV_INTR_MODE_MSIX:
    sprintf(fnic.msix[FNIC_MSIX_RQ].devname,
    "%.11s-fcs-rq", fnic.name);
    fnic.msix[FNIC_MSIX_RQ].isr = fnic_isr_msix_rq;
    fnic.msix[FNIC_MSIX_RQ].devid = fnic;
    sprintf(fnic.msix[FNIC_MSIX_WQ].devname,
    "%.11s-fcs-wq", fnic.name);
    fnic.msix[FNIC_MSIX_WQ].isr = fnic_isr_msix_wq;
    fnic.msix[FNIC_MSIX_WQ].devid = fnic;
    for (i = fnic.copy_wq_base; i < fnic.wq_copy_count + fnic.copy_wq_base; i++) {
    sprintf(fnic.msix[i].devname,
    "%.11s-scsi-wq-%d", fnic.name, i-FNIC_MSIX_WQ_COPY);
    fnic.msix[i].isr = fnic_isr_msix_wq_copy;
    fnic.msix[i].devid = fnic;
    }
    sprintf(fnic.msix[fnic.err_intr_offset].devname,
    "%.11s-err-notify", fnic.name);
    fnic.msix[fnic.err_intr_offset].isr =
    fnic_isr_msix_err_notify;
    fnic.msix[fnic.err_intr_offset].devid = fnic;
    for (i = 0; i < fnic.intr_count; i++) {
    fnic.msix[i].irq_num = pci_irq_vector(fnic.pdev, i);
    err = request_irq(fnic.msix[i].irq_num,
    fnic.msix[i].isr, 0,
    fnic.msix[i].devname,
    fnic.msix[i].devid);
    if (err) {
    FNIC_ISR_DBG(KERN_ERR, fnic,
    "request_irq failed with error: %d\n",
    err);
    fnic_free_intr(fnic);
    break;
    }
    fnic.msix[i].requested = 1;
    }
    break;
    default:
    break;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn fnic_set_intr_mode_msix(fnic: *mut fnic) -> c_int {
    int fnic_set_intr_mode_msix(struct fnic *fnic)
    {
    let mut n: c_uint = ARRAY_SIZE(fnic.rq);
    let mut m: c_uint = ARRAY_SIZE(fnic.wq);
    let mut o: c_uint = ARRAY_SIZE(fnic.hw_copy_wq);
    unsigned int min_irqs = n + m + 1 + 1; /*rq, raw wq, wq, err*/
//
// We need n RQs, m WQs, o Copy WQs, n+m+o CQs, and n+m+o+1 INTRs
// (last INTR is used for WQ/RQ errors and notification area)
//
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "rq-array size: %d wq-array size: %d copy-wq array size: %d\n",
    n, m, o);
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "rq_count: %d raw_wq_count: %d wq_copy_count: %d cq_count: %d\n",
    fnic.rq_count, fnic.raw_wq_count,
    fnic.wq_copy_count, fnic.cq_count);
    if (fnic.rq_count <= n && fnic.raw_wq_count <= m &&
    fnic.wq_copy_count <= o) {
    let mut vec_count: c_int = 0;
    let mut vecs: c_int = fnic.rq_count + fnic.raw_wq_count + fnic.wq_copy_count + 1;
    vec_count = pci_alloc_irq_vectors(fnic.pdev, min_irqs, vecs,
    PCI_IRQ_MSIX | PCI_IRQ_AFFINITY);
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "allocated %d MSI-X vectors\n",
    vec_count);
    if (vec_count > 0) {
    if (vec_count < vecs) {
    FNIC_ISR_DBG(KERN_ERR, fnic,
    "interrupts number mismatch: vec_count: %d vecs: %d\n",
    vec_count, vecs);
    if (vec_count < min_irqs) {
    FNIC_ISR_DBG(KERN_ERR, fnic,
    "no interrupts for copy wq\n");
    return 1;
    }
    }
    fnic.rq_count = n;
    fnic.raw_wq_count = m;
    fnic.copy_wq_base = fnic.rq_count + fnic.raw_wq_count;
    fnic.wq_copy_count = vec_count - n - m - 1;
    fnic.wq_count = fnic.raw_wq_count + fnic.wq_copy_count;
    if (fnic.cq_count != vec_count - 1) {
    FNIC_ISR_DBG(KERN_ERR, fnic,
    "CQ count: %d does not match MSI-X vector count: %d\n",
    fnic.cq_count, vec_count);
    fnic.cq_count = vec_count - 1;
    }
    fnic.intr_count = vec_count;
    fnic.err_intr_offset = fnic.rq_count + fnic.wq_count;
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "rq_count: %d raw_wq_count: %d copy_wq_base: %d\n",
    fnic.rq_count,
    fnic.raw_wq_count, fnic.copy_wq_base);
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "wq_copy_count: %d wq_count: %d cq_count: %d\n",
    fnic.wq_copy_count,
    fnic.wq_count, fnic.cq_count);
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "intr_count: %d err_intr_offset: %u\n",
    fnic.intr_count,
    fnic.err_intr_offset);
    vnic_dev_set_intr_mode(fnic.vdev, VNIC_DEV_INTR_MODE_MSIX);
    FNIC_ISR_DBG(KERN_INFO, fnic,
    "fnic using MSI-X\n");
    return 0;
    }
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn fnic_set_intr_mode(fnic: *mut fnic) -> c_int {
    int fnic_set_intr_mode(struct fnic *fnic)
    {
    let mut ret_status: c_int = 0;
//
// Set interrupt mode (INTx, MSI, MSI-X) depending
// system capabilities.
//
// Try MSI-X first
//
    ret_status = fnic_set_intr_mode_msix(fnic);
    if (ret_status == 0)
    return ret_status;
//
// Next try MSI
// We need 1 RQ, 1 WQ, 1 WQ_COPY, 3 CQs, and 1 INTR
//
    if (fnic.rq_count >= 1 &&
    fnic.raw_wq_count >= 1 &&
    fnic.wq_copy_count >= 1 &&
    fnic.cq_count >= 3 &&
    fnic.intr_count >= 1 &&
    pci_alloc_irq_vectors(fnic.pdev, 1, 1, PCI_IRQ_MSI) == 1) {
    fnic.rq_count = 1;
    fnic.raw_wq_count = 1;
    fnic.wq_copy_count = 1;
    fnic.wq_count = 2;
    fnic.cq_count = 3;
    fnic.intr_count = 1;
    fnic.err_intr_offset = 0;
    FNIC_ISR_DBG(KERN_DEBUG, fnic,
    "Using MSI Interrupts\n");
    vnic_dev_set_intr_mode(fnic.vdev, VNIC_DEV_INTR_MODE_MSI);
    return 0;
    }
//
// Next try INTx
// We need 1 RQ, 1 WQ, 1 WQ_COPY, 3 CQs, and 3 INTRs
// 1 INTR is used for all 3 queues, 1 INTR for queue errors
// 1 INTR for notification area
//
    if (fnic.rq_count >= 1 &&
    fnic.raw_wq_count >= 1 &&
    fnic.wq_copy_count >= 1 &&
    fnic.cq_count >= 3 &&
    fnic.intr_count >= 3) {
    fnic.rq_count = 1;
    fnic.raw_wq_count = 1;
    fnic.wq_copy_count = 1;
    fnic.cq_count = 3;
    fnic.intr_count = 3;
    FNIC_ISR_DBG(KERN_DEBUG, fnic,
    "Using Legacy Interrupts\n");
    vnic_dev_set_intr_mode(fnic.vdev, VNIC_DEV_INTR_MODE_INTX);
    return 0;
    }
    vnic_dev_set_intr_mode(fnic.vdev, VNIC_DEV_INTR_MODE_UNKNOWN);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn fnic_clear_intr_mode(fnic: *mut fnic) {
    void fnic_clear_intr_mode(struct fnic *fnic)
    {
    pci_free_irq_vectors(fnic.pdev);
    vnic_dev_set_intr_mode(fnic.vdev, VNIC_DEV_INTR_MODE_INTX);
    }
