//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/otx2_dcbnl.c
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
// Marvell RVU Ethernet driver
//
// Copyright (C) 2021 Marvell.
//

#[no_mangle]
unsafe extern "C" fn otx2_check_pfc_config(pfvf: *mut otx2_nic) -> c_int {
    static int otx2_check_pfc_config(struct otx2_nic *pfvf)
    {
    let mut tx_queues: u8 = pfvf.hw.tx_queues, prio;
    let mut pfc_en: u8 = pfvf.pfc_en;
    for (prio = 0; prio < NIX_PF_PFC_PRIO_MAX; prio++) {
    if ((pfc_en & (1 << prio)) &&
    prio > tx_queues - 1) {
    dev_warn(pfvf.dev,
    "Increase number of tx queues from %d to %d to support PFC.\n",
    tx_queues, prio + 1);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_pfc_txschq_config(pfvf: *mut otx2_nic) -> c_int {
    int otx2_pfc_txschq_config(struct otx2_nic *pfvf)
    {
    u8 pfc_en, pfc_bit_set;
    int prio, lvl, err;
    pfc_en = pfvf.pfc_en;
    for (prio = 0; prio < NIX_PF_PFC_PRIO_MAX; prio++) {
    pfc_bit_set = pfc_en & (1 << prio);
// Either PFC bit is not set
// or tx scheduler is not allocated for the priority
//
    if (!pfc_bit_set || !pfvf.pfc_alloc_status[prio])
    continue;
// configure the scheduler for the tls
    for (lvl = 0; lvl < NIX_TXSCH_LVL_CNT; lvl++) {
    err = otx2_txschq_config(pfvf, lvl, prio, true);
    if (err) {
    dev_err(pfvf.dev,
    "%s configure PFC tx schq for lvl:%d, prio:%d failed!\n",
    __func__, lvl, prio);
    return err;
    }
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(otx2_pfc_txschq_config);
#[no_mangle]
unsafe extern "C" fn otx2_pfc_txschq_alloc_one(pfvf: *mut otx2_nic, prio: u8) -> c_int {
    static int otx2_pfc_txschq_alloc_one(struct otx2_nic *pfvf, u8 prio)
    {
    struct nix_txsch_alloc_req *req;
    struct nix_txsch_alloc_rsp *rsp;
    int lvl, rc;
// Get memory to put this msg
    req = otx2_mbox_alloc_msg_nix_txsch_alloc(&pfvf.mbox);
    if (!req)
    return -ENOMEM;
// Request one schq per level upto max level as configured
// link config level. These rest of the scheduler can be
// same as hw.txschq_list.
//
    for (lvl = 0; lvl <= pfvf.hw.txschq_link_cfg_lvl; lvl++)
    req.schq[lvl] = 1;
    rc = otx2_sync_mbox_msg(&pfvf.mbox);
    if (rc)
    return rc;
    rsp = (struct nix_txsch_alloc_rsp *)
    otx2_mbox_get_rsp(&pfvf.mbox.mbox, 0, &req.hdr);
    if (IS_ERR(rsp))
    return PTR_ERR(rsp);
// Setup transmit scheduler list
    for (lvl = 0; lvl <= pfvf.hw.txschq_link_cfg_lvl; lvl++) {
    if (!rsp.schq[lvl])
    return -ENOSPC;
    pfvf.pfc_schq_list[lvl][prio] = rsp.schq_list[lvl][0];
    }
// Set the Tx schedulers for rest of the levels same as
// hw.txschq_list as those will be common for all.
//
    for (; lvl < NIX_TXSCH_LVL_CNT; lvl++)
    pfvf.pfc_schq_list[lvl][prio] = pfvf.hw.txschq_list[lvl][0];
    pfvf.pfc_alloc_status[prio] = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_pfc_txschq_alloc(pfvf: *mut otx2_nic) -> c_int {
    int otx2_pfc_txschq_alloc(struct otx2_nic *pfvf)
    {
    let mut pfc_en: u8 = pfvf.pfc_en;
    u8 pfc_bit_set;
    int err, prio;
    for (prio = 0; prio < NIX_PF_PFC_PRIO_MAX; prio++) {
    pfc_bit_set = pfc_en & (1 << prio);
    if (!pfc_bit_set || pfvf.pfc_alloc_status[prio])
    continue;
// Add new scheduler to the priority
    err = otx2_pfc_txschq_alloc_one(pfvf, prio);
    if (err) {
    dev_err(pfvf.dev, "%s failed to allocate PFC TX schedulers\n", __func__);
    return err;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(otx2_pfc_txschq_alloc);
#[no_mangle]
unsafe extern "C" fn otx2_pfc_txschq_stop_one(pfvf: *mut otx2_nic, prio: u8) -> c_int {
    static int otx2_pfc_txschq_stop_one(struct otx2_nic *pfvf, u8 prio)
    {
    int lvl;
// free PFC TLx nodes
    for (lvl = 0; lvl <= pfvf.hw.txschq_link_cfg_lvl; lvl++)
    otx2_txschq_free_one(pfvf, lvl,
    pfvf.pfc_schq_list[lvl][prio]);
    pfvf.pfc_alloc_status[prio] = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn otx2_pfc_update_sq_smq_mapping(pfvf: *mut otx2_nic, prio: c_int) -> c_int {
    static int otx2_pfc_update_sq_smq_mapping(struct otx2_nic *pfvf, int prio)
    {
    struct nix_cn10k_aq_enq_req *cn10k_sq_aq;
    struct net_device *dev = pfvf.netdev;
    let mut if_up: bool = netif_running(dev);
    struct nix_aq_enq_req *sq_aq;
    if (if_up) {
    if (pfvf.pfc_alloc_status[prio])
    netif_tx_stop_all_queues(pfvf.netdev);
    else
    netif_tx_stop_queue(netdev_get_tx_queue(dev, prio));
    }
    if (test_bit(CN10K_LMTST, &pfvf.hw.cap_flag)) {
    cn10k_sq_aq = otx2_mbox_alloc_msg_nix_cn10k_aq_enq(&pfvf.mbox);
    if (!cn10k_sq_aq)
    return -ENOMEM;
// Fill AQ info
    cn10k_sq_aq.qidx = prio;
    cn10k_sq_aq.ctype = NIX_AQ_CTYPE_SQ;
    cn10k_sq_aq.op = NIX_AQ_INSTOP_WRITE;
// Fill fields to update
    cn10k_sq_aq.sq.ena = 1;
    cn10k_sq_aq.sq_mask.ena = 1;
    cn10k_sq_aq.sq_mask.smq = GENMASK(9, 0);
    cn10k_sq_aq.sq.smq = otx2_get_smq_idx(pfvf, prio);
    } else {
    sq_aq = otx2_mbox_alloc_msg_nix_aq_enq(&pfvf.mbox);
    if (!sq_aq)
    return -ENOMEM;
// Fill AQ info
    sq_aq.qidx = prio;
    sq_aq.ctype = NIX_AQ_CTYPE_SQ;
    sq_aq.op = NIX_AQ_INSTOP_WRITE;
// Fill fields to update
    sq_aq.sq.ena = 1;
    sq_aq.sq_mask.ena = 1;
    sq_aq.sq_mask.smq = GENMASK(8, 0);
    sq_aq.sq.smq = otx2_get_smq_idx(pfvf, prio);
    }
    otx2_sync_mbox_msg(&pfvf.mbox);
    if (if_up) {
    if (pfvf.pfc_alloc_status[prio])
    netif_tx_start_all_queues(pfvf.netdev);
    else
    netif_tx_start_queue(netdev_get_tx_queue(dev, prio));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_pfc_txschq_update(pfvf: *mut otx2_nic) -> c_int {
    int otx2_pfc_txschq_update(struct otx2_nic *pfvf)
    {
    let mut if_up: bool = netif_running(pfvf.netdev);
    let mut pfc_en: u8 = pfvf.pfc_en, pfc_bit_set;
    struct mbox *mbox = &pfvf.mbox;
    int err, prio;
    mutex_lock(&mbox.lock);
    for (prio = 0; prio < NIX_PF_PFC_PRIO_MAX; prio++) {
    pfc_bit_set = pfc_en & (1 << prio);
// tx scheduler was created but user wants to disable now
    if (!pfc_bit_set && pfvf.pfc_alloc_status[prio]) {
    mutex_unlock(&mbox.lock);
    if (if_up)
    netif_tx_stop_all_queues(pfvf.netdev);
    otx2_smq_flush(pfvf, pfvf.pfc_schq_list[NIX_TXSCH_LVL_SMQ][prio]);
    if (if_up)
    netif_tx_start_all_queues(pfvf.netdev);
// delete the schq
    err = otx2_pfc_txschq_stop_one(pfvf, prio);
    if (err) {
    dev_err(pfvf.dev,
    "%s failed to stop PFC tx schedulers for priority: %d\n",
    __func__, prio);
    return err;
    }
    mutex_lock(&mbox.lock);
    goto update_sq_smq_map;
    }
// Either PFC bit is not set
// or Tx scheduler is already mapped for the priority
//
    if (!pfc_bit_set || pfvf.pfc_alloc_status[prio])
    continue;
// Add new scheduler to the priority
    err = otx2_pfc_txschq_alloc_one(pfvf, prio);
    if (err) {
    mutex_unlock(&mbox.lock);
    dev_err(pfvf.dev,
    "%s failed to allocate PFC tx schedulers for priority: %d\n",
    __func__, prio);
    return err;
    }
    update_sq_smq_map:
    err = otx2_pfc_update_sq_smq_mapping(pfvf, prio);
    if (err) {
    mutex_unlock(&mbox.lock);
    dev_err(pfvf.dev, "%s failed PFC Tx schq sq:%d mapping", __func__, prio);
    return err;
    }
    }
    err = otx2_pfc_txschq_config(pfvf);
    mutex_unlock(&mbox.lock);
    if (err)
    return err;
    return 0;
    }
    EXPORT_SYMBOL(otx2_pfc_txschq_update);
#[no_mangle]
pub unsafe extern "C" fn otx2_pfc_txschq_stop(pfvf: *mut otx2_nic) -> c_int {
    int otx2_pfc_txschq_stop(struct otx2_nic *pfvf)
    {
    u8 pfc_en, pfc_bit_set;
    int prio, err;
    pfc_en = pfvf.pfc_en;
    for (prio = 0; prio < NIX_PF_PFC_PRIO_MAX; prio++) {
    pfc_bit_set = pfc_en & (1 << prio);
    if (!pfc_bit_set || !pfvf.pfc_alloc_status[prio])
    continue;
// Delete the existing scheduler
    err = otx2_pfc_txschq_stop_one(pfvf, prio);
    if (err) {
    dev_err(pfvf.dev, "%s failed to stop PFC TX schedulers\n", __func__);
    return err;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(otx2_pfc_txschq_stop);
#[no_mangle]
pub unsafe extern "C" fn otx2_config_priority_flow_ctrl(pfvf: *mut otx2_nic) -> c_int {
    int otx2_config_priority_flow_ctrl(struct otx2_nic *pfvf)
    {
    struct cgx_pfc_cfg *req;
    struct cgx_pfc_rsp *rsp;
    let mut err: c_int = 0;
    if (is_otx2_lbkvf(pfvf.pdev))
    return 0;
    mutex_lock(&pfvf.mbox.lock);
    req = otx2_mbox_alloc_msg_cgx_prio_flow_ctrl_cfg(&pfvf.mbox);
    if (!req) {
    err = -ENOMEM;
    goto unlock;
    }
    if (pfvf.pfc_en) {
    req.rx_pause = true;
    req.tx_pause = true;
    } else {
    req.rx_pause = false;
    req.tx_pause = false;
    }
    req.pfc_en = pfvf.pfc_en;
    if (!otx2_sync_mbox_msg(&pfvf.mbox)) {
    rsp = (struct cgx_pfc_rsp *)
    otx2_mbox_get_rsp(&pfvf.mbox.mbox, 0, &req.hdr);
    if (IS_ERR(rsp)) {
    err = PTR_ERR(rsp);
    goto unlock;
    }
    if (req.rx_pause != rsp.rx_pause || req.tx_pause != rsp.tx_pause) {
    dev_warn(pfvf.dev,
    "Failed to config PFC\n");
    err = -EPERM;
    }
    }
    unlock:
    mutex_unlock(&pfvf.mbox.lock);
    return err;
    }
    EXPORT_SYMBOL(otx2_config_priority_flow_ctrl);
    void otx2_update_bpid_in_rqctx(struct otx2_nic *pfvf, int vlan_prio, int qidx,
    bool pfc_enable)
    {
    let mut if_up: bool = netif_running(pfvf.netdev);
    struct npa_aq_enq_req *npa_aq;
    struct nix_aq_enq_req *aq;
    let mut err: c_int = 0;
    if (pfvf.queue_to_pfc_map[qidx] && pfc_enable) {
    dev_warn(pfvf.dev,
    "PFC enable not permitted as Priority %d already mapped to Queue %d\n",
    pfvf.queue_to_pfc_map[qidx], qidx);
    return;
    }
    if (if_up) {
    netif_tx_stop_all_queues(pfvf.netdev);
    netif_carrier_off(pfvf.netdev);
    }
    pfvf.queue_to_pfc_map[qidx] = vlan_prio;
    aq = otx2_mbox_alloc_msg_nix_aq_enq(&pfvf.mbox);
    if (!aq) {
    err = -ENOMEM;
    goto out;
    }
    aq.cq.bpid = pfvf.bpid[vlan_prio];
    aq.cq_mask.bpid = GENMASK(8, 0);
// Fill AQ info
    aq.qidx = qidx;
    aq.ctype = NIX_AQ_CTYPE_CQ;
    aq.op = NIX_AQ_INSTOP_WRITE;
    otx2_sync_mbox_msg(&pfvf.mbox);
    npa_aq = otx2_mbox_alloc_msg_npa_aq_enq(&pfvf.mbox);
    if (!npa_aq) {
    err = -ENOMEM;
    goto out;
    }
    npa_aq.aura.nix0_bpid = pfvf.bpid[vlan_prio];
    npa_aq.aura_mask.nix0_bpid = GENMASK(8, 0);
// Fill NPA AQ info
    npa_aq.aura_id = qidx;
    npa_aq.ctype = NPA_AQ_CTYPE_AURA;
    npa_aq.op = NPA_AQ_INSTOP_WRITE;
    otx2_sync_mbox_msg(&pfvf.mbox);
    out:
    if (if_up) {
    netif_carrier_on(pfvf.netdev);
    netif_tx_start_all_queues(pfvf.netdev);
    }
    if (err)
    dev_warn(pfvf.dev,
    "Updating BPIDs in CQ and Aura contexts of RQ%d failed with err %d\n",
    qidx, err);
    }
    EXPORT_SYMBOL(otx2_update_bpid_in_rqctx);
#[no_mangle]
unsafe extern "C" fn otx2_dcbnl_ieee_getpfc(dev: *mut net_device, pfc: *mut ieee_pfc) -> c_int {
    static int otx2_dcbnl_ieee_getpfc(struct net_device *dev, struct ieee_pfc *pfc)
    {
    struct otx2_nic *pfvf = netdev_priv(dev);
    pfc.pfc_cap = IEEE_8021QAZ_MAX_TCS;
    pfc.pfc_en = pfvf.pfc_en;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn otx2_dcbnl_ieee_setpfc(dev: *mut net_device, pfc: *mut ieee_pfc) -> c_int {
    static int otx2_dcbnl_ieee_setpfc(struct net_device *dev, struct ieee_pfc *pfc)
    {
    struct otx2_nic *pfvf = netdev_priv(dev);
    u8 old_pfc_en;
    int err;
    old_pfc_en = pfvf.pfc_en;
    pfvf.pfc_en = pfc.pfc_en;
    if (pfvf.hw.tx_queues >= NIX_PF_PFC_PRIO_MAX)
    goto process_pfc;
// Check if the PFC configuration can be
// supported by the tx queue configuration
//
    err = otx2_check_pfc_config(pfvf);
    if (err) {
    pfvf.pfc_en = old_pfc_en;
    return err;
    }
    process_pfc:
    err = otx2_config_priority_flow_ctrl(pfvf);
    if (err) {
    pfvf.pfc_en = old_pfc_en;
    return err;
    }
// Default disable backpressure on NIX-CPT
    otx2_nix_cpt_config_bp(pfvf, false);
// Request Per channel Bpids
    if (pfc.pfc_en)
    otx2_nix_config_bp(pfvf, true);
    err = otx2_pfc_txschq_update(pfvf);
    if (err) {
    if (pfc.pfc_en)
    otx2_nix_config_bp(pfvf, false);
    otx2_pfc_txschq_stop(pfvf);
    pfvf.pfc_en = old_pfc_en;
    otx2_config_priority_flow_ctrl(pfvf);
    dev_err(pfvf.dev, "%s failed to update TX schedulers\n", __func__);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn otx2_dcbnl_getdcbx(dev: *mut net_device __always_unused) -> u8 {
    static u8 otx2_dcbnl_getdcbx(struct net_device __always_unused *dev)
    {
    return DCB_CAP_DCBX_HOST | DCB_CAP_DCBX_VER_IEEE;
    }
#[no_mangle]
unsafe extern "C" fn otx2_dcbnl_setdcbx(dev: *mut net_device __always_unused, mode: u8) -> u8 {
    static u8 otx2_dcbnl_setdcbx(struct net_device __always_unused *dev, u8 mode)
    {
    return (mode != (DCB_CAP_DCBX_HOST | DCB_CAP_DCBX_VER_IEEE)) ? 1 : 0;
    }
    static const struct dcbnl_rtnl_ops otx2_dcbnl_ops = {
    .ieee_getpfc	= otx2_dcbnl_ieee_getpfc,
    .ieee_setpfc	= otx2_dcbnl_ieee_setpfc,
    .getdcbx	= otx2_dcbnl_getdcbx,
    .setdcbx	= otx2_dcbnl_setdcbx,
    };
#[no_mangle]
pub unsafe extern "C" fn otx2_dcbnl_set_ops(dev: *mut net_device) -> c_int {
    int otx2_dcbnl_set_ops(struct net_device *dev)
    {
    struct otx2_nic *pfvf = netdev_priv(dev);
    pfvf.queue_to_pfc_map = devm_kzalloc(pfvf.dev, pfvf.hw.rx_queues,
    GFP_KERNEL);
    if (!pfvf.queue_to_pfc_map)
    return -ENOMEM;
    dev.dcbnl_ops = &otx2_dcbnl_ops;
    return 0;
    }
    EXPORT_SYMBOL(otx2_dcbnl_set_ops);
