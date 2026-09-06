//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/intel/prq.c
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
// Copyright (C) 2015 Intel Corporation
//
// Originally split from drivers/iommu/intel/svm.c
//

// Page request queue descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_req_dsc {
    union {
    struct {
    pub type:8: u64,
    pub pasid_present:1: u64,
    pub rsvd:7: u64,
    pub rid:16: u64,
    pub pasid:20: u64,
    pub exe_req:1: u64,
    pub pm_req:1: u64,
    pub rsvd2:10: u64,
}

    u64 qw_0;
    };
    union {
    struct {
    u64 rd_req:1;
    u64 wr_req:1;
    u64 lpig:1;
    u64 prg_index:9;
    u64 addr:52;
    };
    u64 qw_1;
    };
    u64 qw_2;
    u64 qw_3;
    };
//
// intel_iommu_drain_pasid_prq - Drain page requests and responses for a pasid
// @dev: target device
// @pasid: pasid for draining
//
// Drain all pending page requests and responses related to @pasid in both
// software and hardware. This is supposed to be called after the device
// driver has stopped DMA, the pasid entry has been cleared, and both IOTLB
// and DevTLB have been invalidated.
//
// It waits until all pending page requests for @pasid in the page fault
// queue are completed by the prq handling thread. Then follow the steps
// described in VT-d spec CH7.10 to drain all page requests and page
// responses pending in the hardware.
//
#[no_mangle]
pub unsafe extern "C" fn intel_iommu_drain_pasid_prq(dev: *mut device, pasid: u32) {
    void intel_iommu_drain_pasid_prq(struct device *dev, u32 pasid)
    {
    struct device_domain_info *info;
    struct dmar_domain *domain;
    struct intel_iommu *iommu;
    struct qi_desc desc[3];
    int head, tail;
    u16 sid, did;
    info = dev_iommu_priv_get(dev);
    if (!info.iopf_refcount)
    return;
    iommu = info.iommu;
    domain = info.domain;
    sid = PCI_DEVID(info.bus, info.devfn);
    did = domain ? domain_id_iommu(domain, iommu) : FLPT_DEFAULT_DID;
//
// Check and wait until all pending page requests in the queue are
// handled by the prq handling thread.
//
    prq_retry:
    reinit_completion(&iommu.prq_complete);
    tail = readq(iommu.reg + DMAR_PQT_REG) & PRQ_RING_MASK;
    head = readq(iommu.reg + DMAR_PQH_REG) & PRQ_RING_MASK;
    while (head != tail) {
    struct page_req_dsc *req;
    req = &iommu.prq[head / sizeof(*req)];
    if (req.rid != sid ||
    (req.pasid_present && pasid != req.pasid) ||
    (!req.pasid_present && pasid != IOMMU_NO_PASID)) {
    head = (head + sizeof(*req)) & PRQ_RING_MASK;
    continue;
    }
    wait_for_completion(&iommu.prq_complete);
    goto prq_retry;
    }
    iopf_queue_flush_dev(dev);
//
// Perform steps described in VT-d spec CH7.10 to drain page
// requests and responses in hardware.
//
    memset(desc, 0, sizeof(desc));
    desc[0].qw0 = QI_IWD_STATUS_DATA(QI_DONE) |
    QI_IWD_FENCE |
    QI_IWD_TYPE;
    if (pasid == IOMMU_NO_PASID) {
    qi_desc_iotlb(iommu, did, 0, 0, DMA_TLB_DSI_FLUSH, &desc[1]);
    qi_desc_dev_iotlb(sid, info.pfsid, info.ats_qdep, 0,
    MAX_AGAW_PFN_WIDTH, &desc[2]);
    } else {
    qi_desc_piotlb_all(did, pasid, &desc[1]);
    qi_desc_dev_iotlb_pasid(sid, info.pfsid, pasid, info.ats_qdep,
    0, MAX_AGAW_PFN_WIDTH, &desc[2]);
    }
    qi_retry:
    reinit_completion(&iommu.prq_complete);
    qi_submit_sync(iommu, desc, 3, QI_OPT_WAIT_DRAIN);
    if (readl(iommu.reg + DMAR_PRS_REG) & DMA_PRS_PRO) {
    wait_for_completion(&iommu.prq_complete);
    goto qi_retry;
    }
    }
#[no_mangle]
unsafe extern "C" fn is_canonical_address(addr: u64) -> bool {
    static bool is_canonical_address(u64 addr)
    {
    let mut shift: c_int = 64 - (__VIRTUAL_MASK_SHIFT + 1);
    let mut saddr: c_long = (long)addr;
    return (((saddr << shift) >> shift) == saddr);
    }
    static void handle_bad_prq_event(struct intel_iommu *iommu,
    struct page_req_dsc *req, int result)
    {
    let mut desc: qi_desc = { };
    pr_err("%s: Invalid page request: %08llx %08llx\n",
    iommu.name, ((unsigned long long *)req)[0],
    ((unsigned long long *)req)[1]);
    if (!req.lpig)
    return;
    desc.qw0 = QI_PGRP_PASID(req.pasid) |
    QI_PGRP_DID(req.rid) |
    QI_PGRP_PASID_P(req.pasid_present) |
    QI_PGRP_RESP_CODE(result) |
    QI_PGRP_RESP_TYPE;
    desc.qw1 = QI_PGRP_IDX(req.prg_index);
    qi_submit_sync(iommu, &desc, 1, 0);
    }
#[no_mangle]
unsafe extern "C" fn prq_to_iommu_prot(req: *mut page_req_dsc) -> c_int {
    static int prq_to_iommu_prot(struct page_req_dsc *req)
    {
    let mut prot: c_int = 0;
    if (req.rd_req)
    prot |= IOMMU_FAULT_PERM_READ;
    if (req.wr_req)
    prot |= IOMMU_FAULT_PERM_WRITE;
    if (req.exe_req)
    prot |= IOMMU_FAULT_PERM_EXEC;
    if (req.pm_req)
    prot |= IOMMU_FAULT_PERM_PRIV;
    return prot;
    }
    static void intel_prq_report(struct intel_iommu *iommu, struct device *dev,
    struct page_req_dsc *desc)
    {
    let mut event: iopf_fault = { };
// Fill in event data for device specific processing
    event.fault.type = IOMMU_FAULT_PAGE_REQ;
    event.fault.prm.addr = (u64)desc.addr << VTD_PAGE_SHIFT;
    event.fault.prm.pasid = desc.pasid;
    event.fault.prm.grpid = desc.prg_index;
    event.fault.prm.perm = prq_to_iommu_prot(desc);
    if (desc.lpig)
    event.fault.prm.flags |= IOMMU_FAULT_PAGE_REQUEST_LAST_PAGE;
    if (desc.pasid_present) {
    event.fault.prm.flags |= IOMMU_FAULT_PAGE_REQUEST_PASID_VALID;
    event.fault.prm.flags |= IOMMU_FAULT_PAGE_RESPONSE_NEEDS_PASID;
    }
    iommu_report_device_fault(dev, &event);
    }
#[no_mangle]
unsafe extern "C" fn prq_event_thread(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t prq_event_thread(int irq, void *d)
    {
    struct intel_iommu *iommu = d;
    struct page_req_dsc *req;
    int head, tail, handled;
    struct device *dev;
    u64 address;
//
// Clear PPR bit before reading head/tail registers, to ensure that
// we get a new interrupt if needed.
//
    writel(DMA_PRS_PPR, iommu.reg + DMAR_PRS_REG);
    tail = readq(iommu.reg + DMAR_PQT_REG) & PRQ_RING_MASK;
    head = readq(iommu.reg + DMAR_PQH_REG) & PRQ_RING_MASK;
    handled = (head != tail);
    while (head != tail) {
    req = &iommu.prq[head / sizeof(*req)];
    address = (u64)req.addr << VTD_PAGE_SHIFT;
    if (unlikely(!is_canonical_address(address))) {
    pr_err("IOMMU: %s: Address is not canonical\n",
    iommu.name);
    bad_req:
    handle_bad_prq_event(iommu, req, QI_RESP_INVALID);
    goto prq_advance;
    }
    if (unlikely(req.pm_req && (req.rd_req || req.wr_req))) {
    pr_err("IOMMU: %s: Page request in Privilege Mode\n",
    iommu.name);
    goto bad_req;
    }
    if (unlikely(req.exe_req && req.rd_req)) {
    pr_err("IOMMU: %s: Execution request not supported\n",
    iommu.name);
    goto bad_req;
    }
// Drop Stop Marker message. No need for a response.
    if (unlikely(req.lpig && !req.rd_req && !req.wr_req))
    goto prq_advance;
//
// If prq is to be handled outside iommu driver via receiver of
// the fault notifiers, we skip the page response here.
//
    mutex_lock(&iommu.iopf_lock);
    dev = device_rbtree_find(iommu, req.rid);
    if (!dev) {
    mutex_unlock(&iommu.iopf_lock);
    goto bad_req;
    }
    intel_prq_report(iommu, dev, req);
    trace_prq_report(iommu, dev, req.qw_0, req.qw_1,
    req.qw_2, req.qw_3,
    iommu.prq_seq_number++);
    mutex_unlock(&iommu.iopf_lock);
    prq_advance:
    head = (head + sizeof(*req)) & PRQ_RING_MASK;
    }
    writeq(tail, iommu.reg + DMAR_PQH_REG);
//
// Clear the page request overflow bit and wake up all threads that
// are waiting for the completion of this handling.
//
    if (readl(iommu.reg + DMAR_PRS_REG) & DMA_PRS_PRO) {
    pr_info_ratelimited("IOMMU: %s: PRQ overflow detected\n",
    iommu.name);
    head = readq(iommu.reg + DMAR_PQH_REG) & PRQ_RING_MASK;
    tail = readq(iommu.reg + DMAR_PQT_REG) & PRQ_RING_MASK;
    if (head == tail) {
    iopf_queue_discard_partial(iommu.iopf_queue);
    writel(DMA_PRS_PRO, iommu.reg + DMAR_PRS_REG);
    pr_info_ratelimited("IOMMU: %s: PRQ overflow cleared",
    iommu.name);
    }
    }
    if (!completion_done(&iommu.prq_complete))
    complete(&iommu.prq_complete);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_iommu_enable_prq(iommu: *mut intel_iommu) -> c_int {
    int intel_iommu_enable_prq(struct intel_iommu *iommu)
    {
    struct iopf_queue *iopfq;
    int irq, ret;
    iommu.prq =
    iommu_alloc_pages_node_sz(iommu.node, GFP_KERNEL, PRQ_SIZE);
    if (!iommu.prq) {
    pr_warn("IOMMU: %s: Failed to allocate page request queue\n",
    iommu.name);
    return -ENOMEM;
    }
    irq = dmar_alloc_hwirq(IOMMU_IRQ_ID_OFFSET_PRQ + iommu.seq_id, iommu.node, iommu);
    if (irq <= 0) {
    pr_err("IOMMU: %s: Failed to create IRQ vector for page request queue\n",
    iommu.name);
    ret = -EINVAL;
    goto free_prq;
    }
    iommu.pr_irq = irq;
    snprintf(iommu.iopfq_name, sizeof(iommu.iopfq_name),
    "dmar%d-iopfq", iommu.seq_id);
    iopfq = iopf_queue_alloc(iommu.iopfq_name);
    if (!iopfq) {
    pr_err("IOMMU: %s: Failed to allocate iopf queue\n", iommu.name);
    ret = -ENOMEM;
    goto free_hwirq;
    }
    iommu.iopf_queue = iopfq;
    snprintf(iommu.prq_name, sizeof(iommu.prq_name), "dmar%d-prq", iommu.seq_id);
    ret = request_threaded_irq(irq, core::ptr::null_mut(), prq_event_thread, IRQF_ONESHOT,
    iommu.prq_name, iommu);
    if (ret) {
    pr_err("IOMMU: %s: Failed to request IRQ for page request queue\n",
    iommu.name);
    goto free_iopfq;
    }
    writeq(0ULL, iommu.reg + DMAR_PQH_REG);
    writeq(0ULL, iommu.reg + DMAR_PQT_REG);
    writeq(virt_to_phys(iommu.prq) | PRQ_ORDER, iommu.reg + DMAR_PQA_REG);
    init_completion(&iommu.prq_complete);
    return 0;
    free_iopfq:
    iopf_queue_free(iommu.iopf_queue);
    iommu.iopf_queue = core::ptr::null_mut();
    free_hwirq:
    dmar_free_hwirq(irq);
    iommu.pr_irq = 0;
    free_prq:
    iommu_free_pages(iommu.prq);
    iommu.prq = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_iommu_finish_prq(iommu: *mut intel_iommu) -> c_int {
    int intel_iommu_finish_prq(struct intel_iommu *iommu)
    {
    writeq(0ULL, iommu.reg + DMAR_PQH_REG);
    writeq(0ULL, iommu.reg + DMAR_PQT_REG);
    writeq(0ULL, iommu.reg + DMAR_PQA_REG);
    if (iommu.pr_irq) {
    free_irq(iommu.pr_irq, iommu);
    dmar_free_hwirq(iommu.pr_irq);
    iommu.pr_irq = 0;
    }
    if (iommu.iopf_queue) {
    iopf_queue_free(iommu.iopf_queue);
    iommu.iopf_queue = core::ptr::null_mut();
    }
    iommu_free_pages(iommu.prq);
    iommu.prq = core::ptr::null_mut();
    return 0;
    }
    void intel_iommu_page_response(struct device *dev, struct iopf_fault *evt,
    struct iommu_page_response *msg)
    {
    struct device_domain_info *info = dev_iommu_priv_get(dev);
    struct intel_iommu *iommu = info.iommu;
    let mut bus: u8 = info.bus, devfn = info.devfn;
    struct iommu_fault_page_request *prm;
    struct qi_desc desc;
    bool pasid_present;
    u16 sid;
    prm = &evt.fault.prm;
    sid = PCI_DEVID(bus, devfn);
    pasid_present = prm.flags & IOMMU_FAULT_PAGE_REQUEST_PASID_VALID;
    desc.qw0 = QI_PGRP_PASID(prm.pasid) | QI_PGRP_DID(sid) |
    QI_PGRP_PASID_P(pasid_present) |
    QI_PGRP_RESP_CODE(msg.code) |
    QI_PGRP_RESP_TYPE;
    desc.qw1 = QI_PGRP_IDX(prm.grpid);
    desc.qw2 = 0;
    desc.qw3 = 0;
    qi_submit_sync(iommu, &desc, 1, 0);
    }
