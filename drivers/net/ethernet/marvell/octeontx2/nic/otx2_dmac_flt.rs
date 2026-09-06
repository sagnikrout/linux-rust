//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/otx2_dmac_flt.c
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

    static int otx2_dmacflt_do_add(struct otx2_nic *pf, const u8 *mac,
    u32 *dmac_index)
    {
    struct cgx_mac_addr_add_req *req;
    struct cgx_mac_addr_add_rsp *rsp;
    int err;
    mutex_lock(&pf.mbox.lock);
    req = otx2_mbox_alloc_msg_cgx_mac_addr_add(&pf.mbox);
    if (!req) {
    mutex_unlock(&pf.mbox.lock);
    return -ENOMEM;
    }
    ether_addr_copy(req.mac_addr, mac);
    err = otx2_sync_mbox_msg(&pf.mbox);
    if (!err) {
    rsp = (struct cgx_mac_addr_add_rsp *)
    otx2_mbox_get_rsp(&pf.mbox.mbox, 0, &req.hdr);
    if (IS_ERR(rsp)) {
    mutex_unlock(&pf.mbox.lock);
    return PTR_ERR(rsp);
    }
// dmac_index = rsp->index;
    }
    mutex_unlock(&pf.mbox.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn otx2_dmacflt_add_pfmac(pf: *mut otx2_nic, dmac_index: *mut u32) -> c_int {
    static int otx2_dmacflt_add_pfmac(struct otx2_nic *pf, u32 *dmac_index)
    {
    struct cgx_mac_addr_set_or_get *req;
    struct cgx_mac_addr_set_or_get *rsp;
    int err;
    mutex_lock(&pf.mbox.lock);
    req = otx2_mbox_alloc_msg_cgx_mac_addr_set(&pf.mbox);
    if (!req) {
    mutex_unlock(&pf.mbox.lock);
    return -ENOMEM;
    }
    req.index = *dmac_index;
    ether_addr_copy(req.mac_addr, pf.netdev.dev_addr);
    err = otx2_sync_mbox_msg(&pf.mbox);
    if (err)
    goto out;
    rsp = (struct cgx_mac_addr_set_or_get *)
    otx2_mbox_get_rsp(&pf.mbox.mbox, 0, &req.hdr);
    if (IS_ERR_OR_NULL(rsp)) {
    err = -EINVAL;
    goto out;
    }
// dmac_index = rsp->index;
    out:
    mutex_unlock(&pf.mbox.lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_dmacflt_add(pf: *mut otx2_nic, mac: *const u8, bit_pos: u32) -> c_int {
    int otx2_dmacflt_add(struct otx2_nic *pf, const u8 *mac, u32 bit_pos)
    {
    u32 *dmacindex;
// Store dmacindex returned by CGX/RPM driver which will
// be used for macaddr update/remove
//
    dmacindex = &pf.flow_cfg.bmap_to_dmacindex[bit_pos];
    if (ether_addr_equal(mac, pf.netdev.dev_addr))
    return otx2_dmacflt_add_pfmac(pf, dmacindex);
    else
    return otx2_dmacflt_do_add(pf, mac, dmacindex);
    }
    static int otx2_dmacflt_do_remove(struct otx2_nic *pfvf, const u8 *mac,
    u32 dmac_index)
    {
    struct cgx_mac_addr_del_req *req;
    int err;
    mutex_lock(&pfvf.mbox.lock);
    req = otx2_mbox_alloc_msg_cgx_mac_addr_del(&pfvf.mbox);
    if (!req) {
    mutex_unlock(&pfvf.mbox.lock);
    return -ENOMEM;
    }
    req.index = dmac_index;
    err = otx2_sync_mbox_msg(&pfvf.mbox);
    mutex_unlock(&pfvf.mbox.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn otx2_dmacflt_remove_pfmac(pf: *mut otx2_nic, dmac_index: u32) -> c_int {
    static int otx2_dmacflt_remove_pfmac(struct otx2_nic *pf, u32 dmac_index)
    {
    struct cgx_mac_addr_reset_req *req;
    int err;
    mutex_lock(&pf.mbox.lock);
    req = otx2_mbox_alloc_msg_cgx_mac_addr_reset(&pf.mbox);
    if (!req) {
    mutex_unlock(&pf.mbox.lock);
    return -ENOMEM;
    }
    req.index = dmac_index;
    err = otx2_sync_mbox_msg(&pf.mbox);
    mutex_unlock(&pf.mbox.lock);
    return err;
    }
    int otx2_dmacflt_remove(struct otx2_nic *pf, const u8 *mac,
    u32 bit_pos)
    {
    let mut dmacindex: u32 = pf.flow_cfg.bmap_to_dmacindex[bit_pos];
    if (ether_addr_equal(mac, pf.netdev.dev_addr))
    return otx2_dmacflt_remove_pfmac(pf, dmacindex);
    else
    return otx2_dmacflt_do_remove(pf, mac, dmacindex);
    }
// CGX/RPM blocks support max unicast entries of 32.
// on typical configuration MAC block associated
// with 4 lmacs, each lmac will have 8 dmac entries
//
#[no_mangle]
pub unsafe extern "C" fn otx2_dmacflt_get_max_cnt(pf: *mut otx2_nic) -> c_int {
    int otx2_dmacflt_get_max_cnt(struct otx2_nic *pf)
    {
    struct cgx_max_dmac_entries_get_rsp *rsp;
    struct msg_req *msg;
    int err;
    mutex_lock(&pf.mbox.lock);
    msg = otx2_mbox_alloc_msg_cgx_mac_max_entries_get(&pf.mbox);
    if (!msg) {
    mutex_unlock(&pf.mbox.lock);
    return -ENOMEM;
    }
    err = otx2_sync_mbox_msg(&pf.mbox);
    if (err)
    goto out;
    rsp = (struct cgx_max_dmac_entries_get_rsp *)
    otx2_mbox_get_rsp(&pf.mbox.mbox, 0, &msg.hdr);
    if (IS_ERR_OR_NULL(rsp)) {
    err = -EINVAL;
    goto out;
    }
    pf.flow_cfg.dmacflt_max_flows = rsp.max_dmac_filters;
    out:
    mutex_unlock(&pf.mbox.lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_dmacflt_update(pf: *mut otx2_nic, mac: *mut u8, bit_pos: u32) -> c_int {
    int otx2_dmacflt_update(struct otx2_nic *pf, u8 *mac, u32 bit_pos)
    {
    struct cgx_mac_addr_update_req *req;
    struct cgx_mac_addr_update_rsp *rsp;
    int rc;
    mutex_lock(&pf.mbox.lock);
    req = otx2_mbox_alloc_msg_cgx_mac_addr_update(&pf.mbox);
    if (!req) {
    mutex_unlock(&pf.mbox.lock);
    return -ENOMEM;
    }
    ether_addr_copy(req.mac_addr, mac);
    req.index = pf.flow_cfg.bmap_to_dmacindex[bit_pos];
// check the response and change index
    rc = otx2_sync_mbox_msg(&pf.mbox);
    if (rc)
    goto out;
    rsp = (struct cgx_mac_addr_update_rsp *)
    otx2_mbox_get_rsp(&pf.mbox.mbox, 0, &req.hdr);
    if (IS_ERR(rsp)) {
    rc = PTR_ERR(rsp);
    goto out;
    }
    pf.flow_cfg.bmap_to_dmacindex[bit_pos] = rsp.index;
    out:
    mutex_unlock(&pf.mbox.lock);
    return rc;
    }
