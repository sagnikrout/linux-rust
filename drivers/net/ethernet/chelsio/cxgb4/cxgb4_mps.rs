//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_mps.c
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
// Copyright (c) 2019 Chelsio Communications, Inc. All rights reserved.

    static int cxgb4_mps_ref_dec_by_mac(struct adapter *adap,
    const u8 *addr, const u8 *mask)
    {
    u8 bitmask[] = { 0xff, 0xff, 0xff, 0xff, 0xff, 0xff };
    struct mps_entries_ref *mps_entry, *tmp;
    let mut ret: c_int = -EINVAL;
    spin_lock_bh(&adap.mps_ref_lock);
    list_for_each_entry_safe(mps_entry, tmp, &adap.mps_ref, list) {
    if (ether_addr_equal(mps_entry.addr, addr) &&
    ether_addr_equal(mps_entry.mask, mask ? mask : bitmask)) {
    if (!refcount_dec_and_test(&mps_entry.refcnt)) {
    spin_unlock_bh(&adap.mps_ref_lock);
    return -EBUSY;
    }
    list_del(&mps_entry.list);
    kfree(mps_entry);
    ret = 0;
    break;
    }
    }
    spin_unlock_bh(&adap.mps_ref_lock);
    return ret;
    }
    static int cxgb4_mps_ref_inc(struct adapter *adap, const u8 *mac_addr,
    u16 idx, const u8 *mask)
    {
    u8 bitmask[] = { 0xff, 0xff, 0xff, 0xff, 0xff, 0xff };
    struct mps_entries_ref *mps_entry;
    let mut ret: c_int = 0;
    spin_lock_bh(&adap.mps_ref_lock);
    list_for_each_entry(mps_entry, &adap.mps_ref, list) {
    if (mps_entry.idx == idx) {
    refcount_inc(&mps_entry.refcnt);
    goto unlock;
    }
    }
    mps_entry = kzalloc_obj(*mps_entry, GFP_ATOMIC);
    if (!mps_entry) {
    ret = -ENOMEM;
    goto unlock;
    }
    ether_addr_copy(mps_entry.mask, mask ? mask : bitmask);
    ether_addr_copy(mps_entry.addr, mac_addr);
    mps_entry.idx = idx;
    refcount_set(&mps_entry.refcnt, 1);
    list_add_tail(&mps_entry.list, &adap.mps_ref);
    unlock:
    spin_unlock_bh(&adap.mps_ref_lock);
    return ret;
    }
    int cxgb4_free_mac_filt(struct adapter *adap, unsigned int viid,
    unsigned int naddr, const u8 **addr, bool sleep_ok)
    {
    int ret, i;
    for (i = 0; i < naddr; i++) {
    if (!cxgb4_mps_ref_dec_by_mac(adap, addr[i], core::ptr::null_mut())) {
    ret = t4_free_mac_filt(adap, adap.mbox, viid,
    1, &addr[i], sleep_ok);
    if (ret < 0)
    return ret;
    }
    }
// return number of filters freed
    return naddr;
    }
    int cxgb4_alloc_mac_filt(struct adapter *adap, unsigned int viid,
    bool free, unsigned int naddr, const u8 **addr,
    u16 *idx, u64 *hash, bool sleep_ok)
    {
    int ret, i;
    ret = t4_alloc_mac_filt(adap, adap.mbox, viid, free,
    naddr, addr, idx, hash, sleep_ok);
    if (ret < 0)
    return ret;
    for (i = 0; i < naddr; i++) {
    if (idx[i] != 0xffff) {
    if (cxgb4_mps_ref_inc(adap, addr[i], idx[i], core::ptr::null_mut())) {
    ret = -ENOMEM;
    goto error;
    }
    }
    }
    goto out;
    error:
    cxgb4_free_mac_filt(adap, viid, naddr, addr, sleep_ok);
    out:
// Returns a negative error number or the number of filters allocated
    return ret;
    }
    int cxgb4_update_mac_filt(struct port_info *pi, unsigned int viid,
    int *tcam_idx, const u8 *addr,
    bool persistent, u8 *smt_idx)
    {
    int ret;
    ret = cxgb4_change_mac(pi, viid, tcam_idx,
    addr, persistent, smt_idx);
    if (ret < 0)
    return ret;
    cxgb4_mps_ref_inc(pi.adapter, addr, *tcam_idx, core::ptr::null_mut());
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cxgb4_init_mps_ref_entries(adap: *mut adapter) -> c_int {
    int cxgb4_init_mps_ref_entries(struct adapter *adap)
    {
    spin_lock_init(&adap.mps_ref_lock);
    INIT_LIST_HEAD(&adap.mps_ref);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cxgb4_free_mps_ref_entries(adap: *mut adapter) {
    void cxgb4_free_mps_ref_entries(struct adapter *adap)
    {
    struct mps_entries_ref *mps_entry, *tmp;
    if (list_empty(&adap.mps_ref))
    return;
    spin_lock(&adap.mps_ref_lock);
    list_for_each_entry_safe(mps_entry, tmp, &adap.mps_ref, list) {
    list_del(&mps_entry.list);
    kfree(mps_entry);
    }
    spin_unlock(&adap.mps_ref_lock);
    }
