//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_fdb.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_fdb_event_work {
    pub work: work_struct,
    pub fdb_info: switchdev_notifier_fdb_info,
    pub dev: *mut net_device,
    pub orig_dev: *mut net_device,
    pub lan966x: *mut lan966x,
    pub event: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_fdb_entry {
    pub list: list_head,
    pub __aligned(2): unsigned char mac[ETH_ALEN],
    pub vid: u16,
    pub references: u32,
}

    static struct lan966x_fdb_entry *
    lan966x_fdb_find_entry(struct lan966x *lan966x,
    struct switchdev_notifier_fdb_info *fdb_info)
    {
    struct lan966x_fdb_entry *fdb_entry;
    list_for_each_entry(fdb_entry, &lan966x.fdb_entries, list) {
    if (fdb_entry.vid == fdb_info.vid &&
    ether_addr_equal(fdb_entry.mac, fdb_info.addr))
    return fdb_entry;
    }
    return core::ptr::null_mut();
    }
    static void lan966x_fdb_add_entry(struct lan966x *lan966x,
    struct switchdev_notifier_fdb_info *fdb_info)
    {
    struct lan966x_fdb_entry *fdb_entry;
    fdb_entry = lan966x_fdb_find_entry(lan966x, fdb_info);
    if (fdb_entry) {
    fdb_entry.references++;
    return;
    }
    fdb_entry = kzalloc_obj(*fdb_entry);
    if (!fdb_entry)
    return;
    ether_addr_copy(fdb_entry.mac, fdb_info.addr);
    fdb_entry.vid = fdb_info.vid;
    fdb_entry.references = 1;
    list_add_tail(&fdb_entry.list, &lan966x.fdb_entries);
    }
    static bool lan966x_fdb_del_entry(struct lan966x *lan966x,
    struct switchdev_notifier_fdb_info *fdb_info)
    {
    struct lan966x_fdb_entry *fdb_entry, *tmp;
    list_for_each_entry_safe(fdb_entry, tmp, &lan966x.fdb_entries,
    list) {
    if (fdb_entry.vid == fdb_info.vid &&
    ether_addr_equal(fdb_entry.mac, fdb_info.addr)) {
    fdb_entry.references--;
    if (!fdb_entry.references) {
    list_del(&fdb_entry.list);
    kfree(fdb_entry);
    return true;
    }
    break;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_fdb_write_entries(lan966x: *mut lan966x, vid: u16) {
    void lan966x_fdb_write_entries(struct lan966x *lan966x, u16 vid)
    {
    struct lan966x_fdb_entry *fdb_entry;
    list_for_each_entry(fdb_entry, &lan966x.fdb_entries, list) {
    if (fdb_entry.vid != vid)
    continue;
    lan966x_mac_cpu_learn(lan966x, fdb_entry.mac, fdb_entry.vid);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_fdb_erase_entries(lan966x: *mut lan966x, vid: u16) {
    void lan966x_fdb_erase_entries(struct lan966x *lan966x, u16 vid)
    {
    struct lan966x_fdb_entry *fdb_entry;
    list_for_each_entry(fdb_entry, &lan966x.fdb_entries, list) {
    if (fdb_entry.vid != vid)
    continue;
    lan966x_mac_cpu_forget(lan966x, fdb_entry.mac, fdb_entry.vid);
    }
    }
#[no_mangle]
unsafe extern "C" fn lan966x_fdb_purge_entries(lan966x: *mut lan966x) {
    static void lan966x_fdb_purge_entries(struct lan966x *lan966x)
    {
    struct lan966x_fdb_entry *fdb_entry, *tmp;
    list_for_each_entry_safe(fdb_entry, tmp, &lan966x.fdb_entries, list) {
    list_del(&fdb_entry.list);
    kfree(fdb_entry);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_fdb_init(lan966x: *mut lan966x) -> c_int {
    int lan966x_fdb_init(struct lan966x *lan966x)
    {
    INIT_LIST_HEAD(&lan966x.fdb_entries);
    lan966x.fdb_work = alloc_ordered_workqueue("lan966x_order", 0);
    if (!lan966x.fdb_work)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_fdb_deinit(lan966x: *mut lan966x) {
    void lan966x_fdb_deinit(struct lan966x *lan966x)
    {
    destroy_workqueue(lan966x.fdb_work);
    lan966x_fdb_purge_entries(lan966x);
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_fdb_flush_workqueue(lan966x: *mut lan966x) {
    void lan966x_fdb_flush_workqueue(struct lan966x *lan966x)
    {
    flush_workqueue(lan966x.fdb_work);
    }
#[no_mangle]
unsafe extern "C" fn lan966x_fdb_port_event_work(fdb_work: *mut lan966x_fdb_event_work) {
    static void lan966x_fdb_port_event_work(struct lan966x_fdb_event_work *fdb_work)
    {
    struct switchdev_notifier_fdb_info *fdb_info;
    struct lan966x_port *port;
    struct lan966x *lan966x;
    lan966x = fdb_work.lan966x;
    port = netdev_priv(fdb_work.orig_dev);
    fdb_info = &fdb_work.fdb_info;
    switch (fdb_work.event) {
    case SWITCHDEV_FDB_ADD_TO_DEVICE:
    if (!fdb_info.added_by_user)
    break;
    lan966x_mac_add_entry(lan966x, port, fdb_info.addr,
    fdb_info.vid);
    break;
    case SWITCHDEV_FDB_DEL_TO_DEVICE:
    if (!fdb_info.added_by_user)
    break;
    lan966x_mac_del_entry(lan966x, fdb_info.addr,
    fdb_info.vid);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn lan966x_fdb_bridge_event_work(fdb_work: *mut lan966x_fdb_event_work) {
    static void lan966x_fdb_bridge_event_work(struct lan966x_fdb_event_work *fdb_work)
    {
    struct switchdev_notifier_fdb_info *fdb_info;
    struct lan966x *lan966x;
    int ret;
    lan966x = fdb_work.lan966x;
    fdb_info = &fdb_work.fdb_info;
// In case the bridge is called
    switch (fdb_work.event) {
    case SWITCHDEV_FDB_ADD_TO_DEVICE:
// If there is no front port in this vlan, there is no
// point to copy the frame to CPU because it would be
// just dropped at later point. So add it only if
// there is a port but it is required to store the fdb
// entry for later point when a port actually gets in
// the vlan.
//
    lan966x_fdb_add_entry(lan966x, fdb_info);
    if (!lan966x_vlan_cpu_member_cpu_vlan_mask(lan966x,
    fdb_info.vid))
    break;
    lan966x_mac_cpu_learn(lan966x, fdb_info.addr,
    fdb_info.vid);
    break;
    case SWITCHDEV_FDB_DEL_TO_DEVICE:
    ret = lan966x_fdb_del_entry(lan966x, fdb_info);
    if (!lan966x_vlan_cpu_member_cpu_vlan_mask(lan966x,
    fdb_info.vid))
    break;
    if (ret)
    lan966x_mac_cpu_forget(lan966x, fdb_info.addr,
    fdb_info.vid);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn lan966x_fdb_lag_event_work(fdb_work: *mut lan966x_fdb_event_work) {
    static void lan966x_fdb_lag_event_work(struct lan966x_fdb_event_work *fdb_work)
    {
    struct switchdev_notifier_fdb_info *fdb_info;
    struct lan966x_port *port;
    struct lan966x *lan966x;
    if (!lan966x_lag_first_port(fdb_work.orig_dev, fdb_work.dev))
    return;
    lan966x = fdb_work.lan966x;
    port = netdev_priv(fdb_work.dev);
    fdb_info = &fdb_work.fdb_info;
    switch (fdb_work.event) {
    case SWITCHDEV_FDB_ADD_TO_DEVICE:
    if (!fdb_info.added_by_user)
    break;
    lan966x_mac_add_entry(lan966x, port, fdb_info.addr,
    fdb_info.vid);
    break;
    case SWITCHDEV_FDB_DEL_TO_DEVICE:
    if (!fdb_info.added_by_user)
    break;
    lan966x_mac_del_entry(lan966x, fdb_info.addr, fdb_info.vid);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn lan966x_fdb_event_work(work: *mut work_struct) {
    static void lan966x_fdb_event_work(struct work_struct *work)
    {
    struct lan966x_fdb_event_work *fdb_work =
    container_of(work, struct lan966x_fdb_event_work, work);
    if (lan966x_netdevice_check(fdb_work.orig_dev))
    lan966x_fdb_port_event_work(fdb_work);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: netif_is_bridge_master(fdb_work->orig_dev)) -> else {
    else if (netif_is_bridge_master(fdb_work.orig_dev))
    lan966x_fdb_bridge_event_work(fdb_work);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: netif_is_lag_master(fdb_work->orig_dev)) -> else {
    else if (netif_is_lag_master(fdb_work.orig_dev))
    lan966x_fdb_lag_event_work(fdb_work);
    kfree(fdb_work.fdb_info.addr);
    kfree(fdb_work);
    }
    int lan966x_handle_fdb(struct net_device *dev,
    struct net_device *orig_dev,
    unsigned long event, const void *ctx,
    const struct switchdev_notifier_fdb_info *fdb_info)
    {
    struct lan966x_port *port = netdev_priv(dev);
    struct lan966x *lan966x = port.lan966x;
    struct lan966x_fdb_event_work *fdb_work;
    if (ctx && ctx != port)
    return 0;
    switch (event) {
    case SWITCHDEV_FDB_ADD_TO_DEVICE:
    case SWITCHDEV_FDB_DEL_TO_DEVICE:
    if (lan966x_netdevice_check(orig_dev) &&
    !fdb_info.added_by_user)
    break;
    fdb_work = kzalloc_obj(*fdb_work, GFP_ATOMIC);
    if (!fdb_work)
    return -ENOMEM;
    fdb_work.dev = dev;
    fdb_work.orig_dev = orig_dev;
    fdb_work.lan966x = lan966x;
    fdb_work.event = event;
    INIT_WORK(&fdb_work.work, lan966x_fdb_event_work);
    memcpy(&fdb_work.fdb_info, fdb_info, sizeof(fdb_work.fdb_info));
    fdb_work.fdb_info.addr = kzalloc(ETH_ALEN, GFP_ATOMIC);
    if (!fdb_work.fdb_info.addr)
    goto err_addr_alloc;
    ether_addr_copy((u8 *)fdb_work.fdb_info.addr, fdb_info.addr);
    queue_work(lan966x.fdb_work, &fdb_work.work);
    break;
    }
    return 0;
    err_addr_alloc:
    kfree(fdb_work);
    return -ENOMEM;
    }
