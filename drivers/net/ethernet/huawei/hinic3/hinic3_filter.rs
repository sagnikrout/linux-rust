//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_filter.c
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

#[no_mangle]
unsafe extern "C" fn hinic3_filter_addr_sync(netdev: *mut net_device, addr: *mut u8) -> c_int {
    static int hinic3_filter_addr_sync(struct net_device *netdev, u8 *addr)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    return hinic3_set_mac(nic_dev.hwdev, addr, 0,
    hinic3_global_func_id(nic_dev.hwdev));
    }
#[no_mangle]
unsafe extern "C" fn hinic3_filter_addr_unsync(netdev: *mut net_device, addr: *mut u8) -> c_int {
    static int hinic3_filter_addr_unsync(struct net_device *netdev, u8 *addr)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
// The addr is in use
    if (ether_addr_equal(addr, netdev.dev_addr))
    return 0;
    return hinic3_del_mac(nic_dev.hwdev, addr, 0,
    hinic3_global_func_id(nic_dev.hwdev));
    }
#[no_mangle]
pub unsafe extern "C" fn hinic3_clean_mac_list_filter(netdev: *mut net_device) {
    void hinic3_clean_mac_list_filter(struct net_device *netdev)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *f;
    list_for_each_entry_safe(f, ftmp, &nic_dev.uc_filter_list, list) {
    if (f.state == HINIC3_MAC_HW_SYNCED)
    hinic3_filter_addr_unsync(netdev, f.addr);
    list_del(&f.list);
    kfree(f);
    }
    list_for_each_entry_safe(f, ftmp, &nic_dev.mc_filter_list, list) {
    if (f.state == HINIC3_MAC_HW_SYNCED)
    hinic3_filter_addr_unsync(netdev, f.addr);
    list_del(&f.list);
    kfree(f);
    }
    }
    static struct hinic3_mac_filter *
    hinic3_find_mac(const struct list_head *filter_list, u8 *addr)
    {
    struct hinic3_mac_filter *f;
    list_for_each_entry(f, filter_list, list) {
    if (ether_addr_equal(addr, f.addr))
    return f;
    }
    return core::ptr::null_mut();
    }
    static void hinic3_add_filter(struct net_device *netdev,
    struct list_head *mac_filter_list,
    u8 *addr)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    struct hinic3_mac_filter *f;
    f = kzalloc_obj(*f, GFP_ATOMIC);
    if (!f)
    return;
    ether_addr_copy(f.addr, addr);
    INIT_LIST_HEAD(&f.list);
    list_add_tail(&f.list, mac_filter_list);
    f.state = HINIC3_MAC_WAIT_HW_SYNC;
    set_bit(HINIC3_MAC_FILTER_CHANGED, &nic_dev.flags);
    }
    static void hinic3_del_filter(struct net_device *netdev,
    struct hinic3_mac_filter *f)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    set_bit(HINIC3_MAC_FILTER_CHANGED, &nic_dev.flags);
    if (f.state == HINIC3_MAC_WAIT_HW_SYNC) {
// have not added to hw, delete it directly
    list_del(&f.list);
    kfree(f);
    return;
    }
    f.state = HINIC3_MAC_WAIT_HW_UNSYNC;
    }
    static struct hinic3_mac_filter *
    hinic3_mac_filter_entry_clone(const struct hinic3_mac_filter *src)
    {
    struct hinic3_mac_filter *f;
    f = kzalloc_obj(*f, GFP_ATOMIC);
    if (!f)
    return core::ptr::null_mut();
// f = *src;
    INIT_LIST_HEAD(&f.list);
    return f;
    }
    static void hinic3_undo_del_filter_entries(struct list_head *filter_list,
    const struct list_head *from)
    {
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *f;
    list_for_each_entry_safe(f, ftmp, from, list) {
    if (hinic3_find_mac(filter_list, f.addr))
    continue;
    if (f.state == HINIC3_MAC_HW_UNSYNCED)
    f.state = HINIC3_MAC_WAIT_HW_UNSYNC;
    list_move_tail(&f.list, filter_list);
    }
    }
    static void hinic3_undo_add_filter_entries(struct list_head *filter_list,
    const struct list_head *from)
    {
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *tmp;
    struct hinic3_mac_filter *f;
    list_for_each_entry_safe(f, ftmp, from, list) {
    tmp = hinic3_find_mac(filter_list, f.addr);
    if (tmp && tmp.state == HINIC3_MAC_HW_SYNCING)
    tmp.state = HINIC3_MAC_WAIT_HW_SYNC;
    }
    }
#[no_mangle]
unsafe extern "C" fn hinic3_cleanup_filter_list(head: *const list_head) {
    static void hinic3_cleanup_filter_list(const struct list_head *head)
    {
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *f;
    list_for_each_entry_safe(f, ftmp, head, list) {
    list_del(&f.list);
    kfree(f);
    }
    }
    static int hinic3_mac_filter_sync_hw(struct net_device *netdev,
    struct list_head *del_list,
    struct list_head *add_list,
    int *add_count)
    {
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *f;
    int err;
    if (!list_empty(del_list)) {
    list_for_each_entry_safe(f, ftmp, del_list, list) {
// ignore errors when deleting mac
    hinic3_filter_addr_unsync(netdev, f.addr);
    list_del(&f.list);
    kfree(f);
    }
    }
    if (!list_empty(add_list)) {
    list_for_each_entry_safe(f, ftmp, add_list, list) {
    if (f.state != HINIC3_MAC_HW_SYNCING)
    continue;
    err = hinic3_filter_addr_sync(netdev, f.addr);
    if (err) {
    netdev_err(netdev, "Failed to add mac\n");
    return err;
    }
    f.state = HINIC3_MAC_HW_SYNCED;
    (*add_count)++;
    }
    }
    return 0;
    }
    static int hinic3_mac_filter_sync(struct net_device *netdev,
    struct list_head *mac_filter_list, bool uc)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    struct list_head tmp_del_list, tmp_add_list;
    struct hinic3_mac_filter *fclone;
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *f;
    let mut err: c_int = 0, add_count = 0;
    INIT_LIST_HEAD(&tmp_del_list);
    INIT_LIST_HEAD(&tmp_add_list);
    list_for_each_entry_safe(f, ftmp, mac_filter_list, list) {
    if (f.state != HINIC3_MAC_WAIT_HW_UNSYNC)
    continue;
    f.state = HINIC3_MAC_HW_UNSYNCED;
    list_move_tail(&f.list, &tmp_del_list);
    }
    list_for_each_entry_safe(f, ftmp, mac_filter_list, list) {
    if (f.state != HINIC3_MAC_WAIT_HW_SYNC)
    continue;
    fclone = hinic3_mac_filter_entry_clone(f);
    if (!fclone) {
    hinic3_undo_del_filter_entries(mac_filter_list,
    &tmp_del_list);
    hinic3_undo_add_filter_entries(mac_filter_list,
    &tmp_add_list);
    netdev_err(netdev,
    "Failed to clone mac_filter_entry\n");
    err = -ENOMEM;
    goto cleanup_tmp_filter_list;
    }
    f.state = HINIC3_MAC_HW_SYNCING;
    list_add_tail(&fclone.list, &tmp_add_list);
    }
    err = hinic3_mac_filter_sync_hw(netdev, &tmp_del_list,
    &tmp_add_list, &add_count);
    if (err) {
// there were errors, delete all mac in hw
    hinic3_undo_add_filter_entries(mac_filter_list, &tmp_add_list);
    add_count = 0;
// VF does not support promiscuous mode,
// don't delete any other uc mac.
//
    if (!HINIC3_IS_VF(nic_dev.hwdev) || !uc) {
    list_for_each_entry_safe(f, ftmp, mac_filter_list,
    list) {
    if (f.state != HINIC3_MAC_HW_SYNCED)
    continue;
    fclone = hinic3_mac_filter_entry_clone(f);
    if (!fclone)
    break;
    f.state = HINIC3_MAC_WAIT_HW_SYNC;
    list_add_tail(&fclone.list, &tmp_del_list);
    }
    }
    hinic3_mac_filter_sync_hw(netdev, &tmp_del_list,
    &tmp_add_list, &add_count);
    }
    cleanup_tmp_filter_list:
    hinic3_cleanup_filter_list(&tmp_del_list);
    hinic3_cleanup_filter_list(&tmp_add_list);
    return err ? err : add_count;
    }
#[no_mangle]
unsafe extern "C" fn hinic3_mac_filter_sync_all(netdev: *mut net_device) {
    static void hinic3_mac_filter_sync_all(struct net_device *netdev)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    int add_count;
    if (test_bit(HINIC3_MAC_FILTER_CHANGED, &nic_dev.flags)) {
    clear_bit(HINIC3_MAC_FILTER_CHANGED, &nic_dev.flags);
    add_count = hinic3_mac_filter_sync(netdev,
    &nic_dev.uc_filter_list,
    true);
    if (add_count < 0 &&
    hinic3_test_support(nic_dev, HINIC3_NIC_F_PROMISC))
    set_bit(HINIC3_PROMISC_FORCE_ON,
    &nic_dev.rx_mod_state);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: add_count) -> else {
    else if (add_count)
    clear_bit(HINIC3_PROMISC_FORCE_ON,
    &nic_dev.rx_mod_state);
    add_count = hinic3_mac_filter_sync(netdev,
    &nic_dev.mc_filter_list,
    false);
    if (add_count < 0 &&
    hinic3_test_support(nic_dev, HINIC3_NIC_F_ALLMULTI))
    set_bit(HINIC3_ALLMULTI_FORCE_ON,
    &nic_dev.rx_mod_state);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: add_count) -> else {
    else if (add_count)
    clear_bit(HINIC3_ALLMULTI_FORCE_ON,
    &nic_dev.rx_mod_state);
    }
    }

    (L2NIC_RX_MODE_UC | L2NIC_RX_MODE_MC | L2NIC_RX_MODE_BC)
    static void hinic3_update_mac_filter(struct net_device *netdev,
    const struct netdev_hw_addr_list *src_list,
    struct list_head *filter_list)
    {
    struct hinic3_mac_filter *filter;
    struct hinic3_mac_filter *ftmp;
    struct hinic3_mac_filter *f;
    struct netdev_hw_addr *ha;
// add addr if not already in the filter list
    netif_addr_lock_bh(netdev);
    netdev_hw_addr_list_for_each(ha, src_list) {
    filter = hinic3_find_mac(filter_list, ha.addr);
    if (!filter)
    hinic3_add_filter(netdev, filter_list, ha.addr);
#[no_mangle]
pub unsafe extern "C" fn if(HINIC3_MAC_WAIT_HW_UNSYNC: filter->state ==) -> else {
    else if (filter.state == HINIC3_MAC_WAIT_HW_UNSYNC)
    filter.state = HINIC3_MAC_HW_SYNCED;
    }
    netif_addr_unlock_bh(netdev);
// delete addr if not in netdev list
    list_for_each_entry_safe(f, ftmp, filter_list, list) {
    let mut found: bool = false;
    netif_addr_lock_bh(netdev);
    netdev_hw_addr_list_for_each(ha, src_list)
    if (ether_addr_equal(ha.addr, f.addr)) {
    found = true;
    break;
    }
    netif_addr_unlock_bh(netdev);
    if (found)
    continue;
    hinic3_del_filter(netdev, f);
    }
    }
    static void hinic3_sync_rx_mode_to_hw(struct net_device *netdev, int promisc_en,
    int allmulti_en)
    {
    struct hinic3_nic_dev *nic_dev = netdev_priv(netdev);
    let mut rx_mode: u32 = HINIC3_DEFAULT_RX_MODE;
    int err;
    rx_mode |= (promisc_en ? L2NIC_RX_MODE_PROMISC : 0);
    rx_mode |= (allmulti_en ? L2NIC_RX_MODE_MC_ALL : 0);
    if (promisc_en != test_bit(HINIC3_HW_PROMISC_ON,
    &nic_dev.rx_mod_state))
    netdev_dbg(netdev, "%s promisc mode\n",
    promisc_en ? "Enter" : "Left");
    if (allmulti_en !=
    test_bit(HINIC3_HW_ALLMULTI_ON, &nic_dev.rx_mod_state))
    netdev_dbg(netdev, "%s all_multi mode\n",
    allmulti_en ? "Enter" : "Left");
    err = hinic3_set_rx_mode(nic_dev.hwdev, rx_mode);
    if (err) {
    netdev_err(netdev, "Failed to set rx_mode\n");
    return;
    }
    promisc_en ? set_bit(HINIC3_HW_PROMISC_ON, &nic_dev.rx_mod_state) :
    clear_bit(HINIC3_HW_PROMISC_ON, &nic_dev.rx_mod_state);
    allmulti_en ? set_bit(HINIC3_HW_ALLMULTI_ON, &nic_dev.rx_mod_state) :
    clear_bit(HINIC3_HW_ALLMULTI_ON, &nic_dev.rx_mod_state);
    }
#[no_mangle]
pub unsafe extern "C" fn hinic3_set_rx_mode_work(work: *mut work_struct) {
    void hinic3_set_rx_mode_work(struct work_struct *work)
    {
    let mut promisc_en: c_int = 0, allmulti_en = 0;
    struct hinic3_nic_dev *nic_dev;
    struct net_device *netdev;
    nic_dev = container_of(work, struct hinic3_nic_dev, rx_mode_work);
    netdev = nic_dev.netdev;
    if (test_and_clear_bit(HINIC3_UPDATE_MAC_FILTER, &nic_dev.flags)) {
    hinic3_update_mac_filter(netdev, &netdev.uc,
    &nic_dev.uc_filter_list);
    hinic3_update_mac_filter(netdev, &netdev.mc,
    &nic_dev.mc_filter_list);
    }
    hinic3_mac_filter_sync_all(netdev);
    if (hinic3_test_support(nic_dev, HINIC3_NIC_F_PROMISC))
    promisc_en = !!(netdev.flags & IFF_PROMISC) ||
    test_bit(HINIC3_PROMISC_FORCE_ON,
    &nic_dev.rx_mod_state);
    if (hinic3_test_support(nic_dev, HINIC3_NIC_F_ALLMULTI))
    allmulti_en = !!(netdev.flags & IFF_ALLMULTI) ||
    test_bit(HINIC3_ALLMULTI_FORCE_ON,
    &nic_dev.rx_mod_state);
    if (promisc_en != test_bit(HINIC3_HW_PROMISC_ON,
    &nic_dev.rx_mod_state) ||
    allmulti_en != test_bit(HINIC3_HW_ALLMULTI_ON,
    &nic_dev.rx_mod_state))
    hinic3_sync_rx_mode_to_hw(netdev, promisc_en, allmulti_en);
    }
