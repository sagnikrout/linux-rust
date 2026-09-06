//! Automatically rewritten from C to Rust
//! Source: net/core/netdev_work.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

    static void netdev_work_proc(struct work_struct *work);
// @netdev_work_lock protects:
// - @netdev_work_list
// - within the list entries (struct net_device fields):
// - work_node
// - work_tracker
// - work_pending
// - work_core_pending
//
    static LIST_HEAD(netdev_work_list);
    static DEFINE_SPINLOCK(netdev_work_lock);
    static DECLARE_WORK(netdev_work, netdev_work_proc);
    static void netdev_work_enqueue(struct net_device *dev, unsigned long events,
    unsigned long core)
    {
    if (!events && !core)
    return;
    spin_lock_bh(&netdev_work_lock);
    if (!dev_isalive(dev)) {
    spin_unlock_bh(&netdev_work_lock);
    return;
    }
    if (list_empty(&dev.work_node)) {
    list_add_tail(&dev.work_node, &netdev_work_list);
    netdev_hold(dev, &dev.work_tracker, GFP_ATOMIC);
    }
    dev.work_pending |= events;
    dev.work_core_pending |= core;
    spin_unlock_bh(&netdev_work_lock);
    schedule_work(&netdev_work);
    }
    static unsigned long
    netdev_work_dequeue(struct net_device *dev, unsigned long *pending,
    unsigned long mask)
    {
    unsigned long events;
    spin_lock_bh(&netdev_work_lock);
    events = *pending & mask;
// pending &= ~events;
    if (!list_empty(&dev.work_node) &&
    !dev.work_pending && !dev.work_core_pending) {
    list_del_init(&dev.work_node);
    netdev_put(dev, &dev.work_tracker);
    }
    spin_unlock_bh(&netdev_work_lock);
    return events;
    }
#[no_mangle]
pub unsafe extern "C" fn netdev_work_cancel_all(dev: *mut net_device) {
    void netdev_work_cancel_all(struct net_device *dev)
    {
    spin_lock_bh(&netdev_work_lock);
    dev.work_pending = 0;
    dev.work_core_pending = 0;
    if (!list_empty(&dev.work_node)) {
    list_del_init(&dev.work_node);
    netdev_put(dev, &dev.work_tracker);
    }
    spin_unlock_bh(&netdev_work_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn netdev_work_sched(dev: *mut net_device, events: c_ulong) {
    void netdev_work_sched(struct net_device *dev, unsigned long events)
    {
    netdev_work_enqueue(dev, events, 0);
    }
    EXPORT_SYMBOL(netdev_work_sched);
//
// netdev_work_cancel() - cancel selected work for a netdev
// @dev: net_device
// @mask: events to cancel
//
// Clear @mask from the device's work pending mask. If no work is left pending
// the device is dequeued and its ndo_work won't be called.
//
// No expectations on locking, but also no guarantees provided. If the caller
// wants to touch @dev afterwards (e.g. call the work that got canceled)
// they have to ensure @dev does not get freed.
//
// Returns: the subset of @mask that was actually pending, so the caller can run
// those events inline.
//
#[no_mangle]
pub unsafe extern "C" fn netdev_work_cancel(dev: *mut net_device, mask: c_ulong) -> c_ulong {
    unsigned long netdev_work_cancel(struct net_device *dev, unsigned long mask)
    {
    return netdev_work_dequeue(dev, &dev.work_pending, mask);
    }
    EXPORT_SYMBOL(netdev_work_cancel);
#[no_mangle]
pub unsafe extern "C" fn __netdev_work_core_sched(dev: *mut net_device, events: c_ulong) {
    void __netdev_work_core_sched(struct net_device *dev, unsigned long events)
    {
    netdev_work_enqueue(dev, 0, events);
    }
    unsigned long
    __netdev_work_core_cancel(struct net_device *dev, unsigned long mask)
    {
    return netdev_work_dequeue(dev, &dev.work_core_pending, mask);
    }
    static void netdev_work_run(struct net_device *dev, unsigned long events,
    unsigned long core)
    {
    if (!netif_device_present(dev))
    return;
    if (core & NETDEV_WORK_RX_MODE)
    netif_rx_mode_run(dev);
    if (events && dev.netdev_ops.ndo_work)
    dev.netdev_ops.ndo_work(dev, events);
    }
#[no_mangle]
unsafe extern "C" fn netdev_work_proc(work: *mut work_struct) {
    static void netdev_work_proc(struct work_struct *work)
    {
    rtnl_lock();
    while (true) {
    let mut events: c_ulong = 0, core = 0;
    netdevice_tracker tracker;
    struct net_device *dev;
    spin_lock_bh(&netdev_work_lock);
    if (list_empty(&netdev_work_list)) {
    spin_unlock_bh(&netdev_work_lock);
    break;
    }
    dev = list_first_entry(&netdev_work_list, struct net_device,
    work_node);
// Take a temporary reference so @dev can't be freed while we
// drop the lock to grab its ops lock; the work reference is
// only released once we claim the work below.
// The re-locking dance is to ensure that ops lock is enough
// to ensure canceling work is not racy with dequeue.
//
    netdev_hold(dev, &tracker, GFP_ATOMIC);
    spin_unlock_bh(&netdev_work_lock);
    netdev_lock_ops(dev);
    spin_lock_bh(&netdev_work_lock);
    if (!list_empty(&dev.work_node)) {
    list_del_init(&dev.work_node);
    core = dev.work_core_pending;
    dev.work_core_pending = 0;
    events = dev.work_pending;
    dev.work_pending = 0;
// We took another ref above
    netdev_put(dev, &dev.work_tracker);
    if (!dev_isalive(dev))
    core = events = 0;
    }
    spin_unlock_bh(&netdev_work_lock);
    netdev_work_run(dev, events, core);
    netdev_unlock_ops(dev);
    netdev_put(dev, &tracker);
    }
    rtnl_unlock();
    }
