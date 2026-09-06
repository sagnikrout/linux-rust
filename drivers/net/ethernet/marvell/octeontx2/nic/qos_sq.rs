//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/qos_sq.c
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
// Marvell RVU Physical Function ethernet driver
//
// Copyright (C) 2023 Marvell.
//

pub const OTX2_QOS_MAX_LEAF_NODES: c_int = 16;
#[no_mangle]
unsafe extern "C" fn otx2_qos_aura_pool_free(pfvf: *mut otx2_nic, pool_id: c_int) {
    static void otx2_qos_aura_pool_free(struct otx2_nic *pfvf, int pool_id)
    {
    struct otx2_pool *pool;
    if (!pfvf.qset.pool)
    return;
    pool = &pfvf.qset.pool[pool_id];
    qmem_free(pfvf.dev, pool.stack);
    qmem_free(pfvf.dev, pool.fc_addr);
    pool.stack = core::ptr::null_mut();
    pool.fc_addr = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn otx2_qos_sq_aura_pool_init(pfvf: *mut otx2_nic, qidx: c_int) -> c_int {
    static int otx2_qos_sq_aura_pool_init(struct otx2_nic *pfvf, int qidx)
    {
    struct otx2_qset *qset = &pfvf.qset;
    int pool_id, stack_pages, num_sqbs;
    struct otx2_hw *hw = &pfvf.hw;
    struct otx2_snd_queue *sq;
    struct otx2_pool *pool;
    dma_addr_t bufptr;
    int err, ptr;
    u64 iova, pa;
// Calculate number of SQBs needed.
//
// For a 128byte SQE, and 4K size SQB, 31 SQEs will fit in one SQB.
// Last SQE is used for pointing to next SQB.
//
    num_sqbs = (hw.sqb_size / 128) - 1;
    num_sqbs = (qset.sqe_cnt + num_sqbs) / num_sqbs;
// Get no of stack pages needed
    stack_pages =
    (num_sqbs + hw.stack_pg_ptrs - 1) / hw.stack_pg_ptrs;
    pool_id = otx2_get_pool_idx(pfvf, AURA_NIX_SQ, qidx);
    pool = &pfvf.qset.pool[pool_id];
// Initialize aura context
    err = otx2_aura_init(pfvf, pool_id, pool_id, num_sqbs);
    if (err)
    return err;
// Initialize pool context
    err = otx2_pool_init(pfvf, pool_id, stack_pages,
    num_sqbs, hw.sqb_size, AURA_NIX_SQ);
    if (err)
    goto aura_free;
// Flush accumulated messages
    err = otx2_sync_mbox_msg(&pfvf.mbox);
    if (err)
    goto pool_free;
// Allocate pointers and free them to aura/pool
    sq = &qset.sq[qidx];
    sq.sqb_count = 0;
    sq.sqb_ptrs = kcalloc(num_sqbs, sizeof(*sq.sqb_ptrs), GFP_KERNEL);
    if (!sq.sqb_ptrs) {
    err = -ENOMEM;
    goto pool_free;
    }
    for (ptr = 0; ptr < num_sqbs; ptr++) {
    err = otx2_alloc_rbuf(pfvf, pool, &bufptr, pool_id, ptr);
    if (err)
    goto sqb_free;
    pfvf.hw_ops.aura_freeptr(pfvf, pool_id, bufptr);
    sq.sqb_ptrs[sq.sqb_count++] = (u64)bufptr;
    }
    return 0;
    sqb_free:
    while (ptr--) {
    if (!sq.sqb_ptrs[ptr])
    continue;
    iova = sq.sqb_ptrs[ptr];
    pa = otx2_iova_to_phys(pfvf.iommu_domain, iova);
    dma_unmap_page_attrs(pfvf.dev, iova, hw.sqb_size,
    DMA_FROM_DEVICE,
    DMA_ATTR_SKIP_CPU_SYNC);
    put_page(virt_to_page(phys_to_virt(pa)));
    otx2_aura_allocptr(pfvf, pool_id);
    }
    sq.sqb_count = 0;
    kfree(sq.sqb_ptrs);
    pool_free:
    qmem_free(pfvf.dev, pool.stack);
    aura_free:
    qmem_free(pfvf.dev, pool.fc_addr);
    otx2_mbox_reset(&pfvf.mbox.mbox, 0);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn otx2_qos_sq_free_sqbs(pfvf: *mut otx2_nic, qidx: c_int) {
    static void otx2_qos_sq_free_sqbs(struct otx2_nic *pfvf, int qidx)
    {
    struct otx2_qset *qset = &pfvf.qset;
    struct otx2_hw *hw = &pfvf.hw;
    struct otx2_snd_queue *sq;
    u64 iova, pa;
    int sqb;
    sq = &qset.sq[qidx];
    if (!sq.sqb_ptrs)
    return;
    for (sqb = 0; sqb < sq.sqb_count; sqb++) {
    if (!sq.sqb_ptrs[sqb])
    continue;
    iova = sq.sqb_ptrs[sqb];
    pa = otx2_iova_to_phys(pfvf.iommu_domain, iova);
    dma_unmap_page_attrs(pfvf.dev, iova, hw.sqb_size,
    DMA_FROM_DEVICE,
    DMA_ATTR_SKIP_CPU_SYNC);
    put_page(virt_to_page(phys_to_virt(pa)));
    }
    sq.sqb_count = 0;
    sq = &qset.sq[qidx];
    qmem_free(pfvf.dev, sq.sqe);
    qmem_free(pfvf.dev, sq.tso_hdrs);
    kfree(sq.sg);
    kfree(sq.sqb_ptrs);
    qmem_free(pfvf.dev, sq.timestamps);
    memset((void *)sq, 0, sizeof(*sq));
    }
// send queue id
#[no_mangle]
unsafe extern "C" fn otx2_qos_sqb_flush(pfvf: *mut otx2_nic, qidx: c_int) {
    static void otx2_qos_sqb_flush(struct otx2_nic *pfvf, int qidx)
    {
    int sqe_tail, sqe_head;
    void __iomem *ptr;
    u64 incr, val;
    ptr = otx2_get_regaddr(pfvf, NIX_LF_SQ_OP_STATUS);
    incr = (u64)qidx << 32;
    val = otx2_atomic64_add(incr, ptr);
    sqe_head = (val >> 20) & 0x3F;
    sqe_tail = (val >> 28) & 0x3F;
    if (sqe_head != sqe_tail)
    usleep_range(50, 60);
    }
#[no_mangle]
unsafe extern "C" fn otx2_qos_ctx_disable(pfvf: *mut otx2_nic, qidx: u16, aura_id: c_int) -> c_int {
    static int otx2_qos_ctx_disable(struct otx2_nic *pfvf, u16 qidx, int aura_id)
    {
    struct nix_cn10k_aq_enq_req *cn10k_sq_aq;
    struct npa_aq_enq_req *aura_aq;
    struct npa_aq_enq_req *pool_aq;
    struct nix_aq_enq_req *sq_aq;
    if (test_bit(CN10K_LMTST, &pfvf.hw.cap_flag)) {
    cn10k_sq_aq = otx2_mbox_alloc_msg_nix_cn10k_aq_enq(&pfvf.mbox);
    if (!cn10k_sq_aq)
    return -ENOMEM;
    cn10k_sq_aq.qidx = qidx;
    cn10k_sq_aq.sq.ena = 0;
    cn10k_sq_aq.sq_mask.ena = 1;
    cn10k_sq_aq.ctype = NIX_AQ_CTYPE_SQ;
    cn10k_sq_aq.op = NIX_AQ_INSTOP_WRITE;
    } else {
    sq_aq = otx2_mbox_alloc_msg_nix_aq_enq(&pfvf.mbox);
    if (!sq_aq)
    return -ENOMEM;
    sq_aq.qidx = qidx;
    sq_aq.sq.ena = 0;
    sq_aq.sq_mask.ena = 1;
    sq_aq.ctype = NIX_AQ_CTYPE_SQ;
    sq_aq.op = NIX_AQ_INSTOP_WRITE;
    }
    aura_aq = otx2_mbox_alloc_msg_npa_aq_enq(&pfvf.mbox);
    if (!aura_aq) {
    otx2_mbox_reset(&pfvf.mbox.mbox, 0);
    return -ENOMEM;
    }
    aura_aq.aura_id = aura_id;
    aura_aq.aura.ena = 0;
    aura_aq.aura_mask.ena = 1;
    aura_aq.ctype = NPA_AQ_CTYPE_AURA;
    aura_aq.op = NPA_AQ_INSTOP_WRITE;
    pool_aq = otx2_mbox_alloc_msg_npa_aq_enq(&pfvf.mbox);
    if (!pool_aq) {
    otx2_mbox_reset(&pfvf.mbox.mbox, 0);
    return -ENOMEM;
    }
    pool_aq.aura_id = aura_id;
    pool_aq.pool.ena = 0;
    pool_aq.pool_mask.ena = 1;
    pool_aq.ctype = NPA_AQ_CTYPE_POOL;
    pool_aq.op = NPA_AQ_INSTOP_WRITE;
    return otx2_sync_mbox_msg(&pfvf.mbox);
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_qos_get_qid(pfvf: *mut otx2_nic) -> c_int {
    int otx2_qos_get_qid(struct otx2_nic *pfvf)
    {
    int qidx;
    qidx = find_first_zero_bit(pfvf.qos.qos_sq_bmap,
    pfvf.hw.tc_tx_queues);
    let mut qidx: return = = pfvf.hw.tc_tx_queues ? -ENOSPC : qidx;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_qos_free_qid(pfvf: *mut otx2_nic, qidx: c_int) {
    void otx2_qos_free_qid(struct otx2_nic *pfvf, int qidx)
    {
    clear_bit(qidx, pfvf.qos.qos_sq_bmap);
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_qos_enable_sq(pfvf: *mut otx2_nic, qidx: c_int) -> c_int {
    int otx2_qos_enable_sq(struct otx2_nic *pfvf, int qidx)
    {
    struct otx2_hw *hw = &pfvf.hw;
    int pool_id, sq_idx, err;
    if (pfvf.flags & OTX2_FLAG_INTF_DOWN)
    return -EPERM;
    sq_idx = hw.non_qos_queues + qidx;
    mutex_lock(&pfvf.mbox.lock);
    err = otx2_qos_sq_aura_pool_init(pfvf, sq_idx);
    if (err)
    goto out;
    pool_id = otx2_get_pool_idx(pfvf, AURA_NIX_SQ, sq_idx);
    err = otx2_sq_init(pfvf, sq_idx, pool_id);
    if (err)
    goto out;
    out:
    mutex_unlock(&pfvf.mbox.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn otx2_qos_nix_npa_ndc_sync(pfvf: *mut otx2_nic) -> c_int {
    static int otx2_qos_nix_npa_ndc_sync(struct otx2_nic *pfvf)
    {
    struct ndc_sync_op *req;
    int rc;
    mutex_lock(&pfvf.mbox.lock);
    req = otx2_mbox_alloc_msg_ndc_sync_op(&pfvf.mbox);
    if (!req) {
    mutex_unlock(&pfvf.mbox.lock);
    return -ENOMEM;
    }
    req.nix_lf_tx_sync = true;
    req.npa_lf_sync = true;
    rc = otx2_sync_mbox_msg(&pfvf.mbox);
    mutex_unlock(&pfvf.mbox.lock);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_qos_disable_sq(pfvf: *mut otx2_nic, qidx: c_int) {
    void otx2_qos_disable_sq(struct otx2_nic *pfvf, int qidx)
    {
    struct otx2_qset *qset = &pfvf.qset;
    struct otx2_hw *hw = &pfvf.hw;
    struct otx2_snd_queue *sq;
    struct otx2_cq_queue *cq;
    int pool_id, sq_idx;
    sq_idx = hw.non_qos_queues + qidx;
// If the DOWN flag is set SQs are already freed
    if (pfvf.flags & OTX2_FLAG_INTF_DOWN)
    return;
    sq = &pfvf.qset.sq[sq_idx];
    if (!sq.sqb_ptrs)
    return;
    if (sq_idx < hw.non_qos_queues ||
    sq_idx >= otx2_get_total_tx_queues(pfvf)) {
    netdev_err(pfvf.netdev, "Send Queue is not a QoS queue\n");
    return;
    }
    cq = &qset.cq[pfvf.hw.rx_queues + sq_idx];
    pool_id = otx2_get_pool_idx(pfvf, AURA_NIX_SQ, sq_idx);
    otx2_qos_sqb_flush(pfvf, sq_idx);
    otx2_smq_flush(pfvf, otx2_get_smq_idx(pfvf, sq_idx));
// NIX/NPA NDC sync
    otx2_qos_nix_npa_ndc_sync(pfvf);
    otx2_cleanup_tx_cqes(pfvf, cq);
    mutex_lock(&pfvf.mbox.lock);
    otx2_qos_ctx_disable(pfvf, sq_idx, pool_id);
    mutex_unlock(&pfvf.mbox.lock);
    otx2_qos_sq_free_sqbs(pfvf, sq_idx);
    otx2_qos_aura_pool_free(pfvf, pool_id);
    }
