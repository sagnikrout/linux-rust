//! Automatically rewritten from C to Rust
//! Source: net/l3mdev/l3mdev.c
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
//
// net/l3mdev/l3mdev.c - L3 master device implementation
// Copyright (c) 2015 Cumulus Networks
// Copyright (c) 2015 David Ahern <dsa@cumulusnetworks.com>
//

    static DEFINE_SPINLOCK(l3mdev_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l3mdev_handler {
    pub dev_lookup: lookup_by_table_id_t,
}

    static struct l3mdev_handler l3mdev_handlers[L3MDEV_TYPE_MAX + 1];
#[no_mangle]
unsafe extern "C" fn l3mdev_check_type(l3type: enum l3mdev_type) -> c_int {
    static int l3mdev_check_type(enum l3mdev_type l3type)
    {
    if (l3type <= L3MDEV_TYPE_UNSPEC || l3type > L3MDEV_TYPE_MAX)
    return -EINVAL;
    return 0;
    }
    int l3mdev_table_lookup_register(enum l3mdev_type l3type,
    lookup_by_table_id_t fn)
    {
    struct l3mdev_handler *hdlr;
    int res;
    res = l3mdev_check_type(l3type);
    if (res)
    return res;
    hdlr = &l3mdev_handlers[l3type];
    spin_lock(&l3mdev_lock);
    if (hdlr.dev_lookup) {
    res = -EBUSY;
    goto unlock;
    }
    hdlr.dev_lookup = fn;
    res = 0;
    unlock:
    spin_unlock(&l3mdev_lock);
    return res;
    }
    EXPORT_SYMBOL_GPL(l3mdev_table_lookup_register);
    void l3mdev_table_lookup_unregister(enum l3mdev_type l3type,
    lookup_by_table_id_t fn)
    {
    struct l3mdev_handler *hdlr;
    if (l3mdev_check_type(l3type))
    return;
    hdlr = &l3mdev_handlers[l3type];
    spin_lock(&l3mdev_lock);
    if (hdlr.dev_lookup == fn)
    hdlr.dev_lookup = core::ptr::null_mut();
    spin_unlock(&l3mdev_lock);
    }
    EXPORT_SYMBOL_GPL(l3mdev_table_lookup_unregister);
    int l3mdev_ifindex_lookup_by_table_id(enum l3mdev_type l3type,
    struct net *net, u32 table_id)
    {
    lookup_by_table_id_t lookup;
    struct l3mdev_handler *hdlr;
    let mut ifindex: c_int = -EINVAL;
    int res;
    res = l3mdev_check_type(l3type);
    if (res)
    return res;
    hdlr = &l3mdev_handlers[l3type];
    spin_lock(&l3mdev_lock);
    lookup = hdlr.dev_lookup;
    if (!lookup)
    goto unlock;
    ifindex = lookup(net, table_id);
    unlock:
    spin_unlock(&l3mdev_lock);
    return ifindex;
    }
    EXPORT_SYMBOL_GPL(l3mdev_ifindex_lookup_by_table_id);
//
// l3mdev_master_ifindex_rcu - get index of L3 master device
// @dev: targeted interface
//
#[no_mangle]
pub unsafe extern "C" fn l3mdev_master_ifindex_rcu(dev: *const net_device) -> c_int {
    int l3mdev_master_ifindex_rcu(const struct net_device *dev)
    {
    let mut ifindex: c_int = 0;
    if (!dev)
    return 0;
    if (netif_is_l3_master(dev)) {
    ifindex = dev.ifindex;
    } else if (netif_is_l3_slave(dev)) {
    struct net_device *master;
    struct net_device *_dev = (struct net_device *)dev;
// netdev_master_upper_dev_get_rcu calls
// list_first_or_null_rcu to walk the upper dev list.
// list_first_or_null_rcu does not handle a const arg. We aren't
// making changes, just want the master device from that list so
// typecast to remove the const
//
    master = netdev_master_upper_dev_get_rcu(_dev);
    if (master)
    ifindex = master.ifindex;
    }
    return ifindex;
    }
    EXPORT_SYMBOL_GPL(l3mdev_master_ifindex_rcu);
//
// l3mdev_master_upper_ifindex_by_index_rcu - get index of upper l3 master
// device
// @net: network namespace for device index lookup
// @ifindex: targeted interface
//
#[no_mangle]
pub unsafe extern "C" fn l3mdev_master_upper_ifindex_by_index_rcu(net: *mut net, ifindex: c_int) -> c_int {
    int l3mdev_master_upper_ifindex_by_index_rcu(struct net *net, int ifindex)
    {
    struct net_device *dev;
    dev = dev_get_by_index_rcu(net, ifindex);
    while (dev && !netif_is_l3_master(dev))
    dev = netdev_master_upper_dev_get_rcu(dev);
    return dev ? dev.ifindex : 0;
    }
    EXPORT_SYMBOL_GPL(l3mdev_master_upper_ifindex_by_index_rcu);
//
// l3mdev_fib_table_rcu - get FIB table id associated with an L3
// master interface
// @dev: targeted interface
//
#[no_mangle]
pub unsafe extern "C" fn l3mdev_fib_table_rcu(dev: *const net_device) -> u32 {
    u32 l3mdev_fib_table_rcu(const struct net_device *dev)
    {
    let mut tb_id: u32 = 0;
    if (!dev)
    return 0;
    if (netif_is_l3_master(dev)) {
    if (dev.l3mdev_ops.l3mdev_fib_table)
    tb_id = dev.l3mdev_ops.l3mdev_fib_table(dev);
    } else if (netif_is_l3_slave(dev)) {
// Users of netdev_master_upper_dev_get_rcu need non-const,
// but current inet_*type functions take a const
//
    struct net_device *_dev = (struct net_device *) dev;
    const struct net_device *master;
    master = netdev_master_upper_dev_get_rcu(_dev);
    if (master &&
    master.l3mdev_ops.l3mdev_fib_table)
    tb_id = master.l3mdev_ops.l3mdev_fib_table(master);
    }
    return tb_id;
    }
    EXPORT_SYMBOL_GPL(l3mdev_fib_table_rcu);
#[no_mangle]
pub unsafe extern "C" fn l3mdev_fib_table_by_index(net: *mut net, ifindex: c_int) -> u32 {
    u32 l3mdev_fib_table_by_index(struct net *net, int ifindex)
    {
    struct net_device *dev;
    let mut tb_id: u32 = 0;
    if (!ifindex)
    return 0;
    rcu_read_lock();
    dev = dev_get_by_index_rcu(net, ifindex);
    if (dev)
    tb_id = l3mdev_fib_table_rcu(dev);
    rcu_read_unlock();
    return tb_id;
    }
    EXPORT_SYMBOL_GPL(l3mdev_fib_table_by_index);
//
// l3mdev_link_scope_lookup - IPv6 route lookup based on flow for link
// local and multicast addresses
// @net: network namespace for device index lookup
// @fl6: IPv6 flow struct for lookup
// This function does not hold refcnt on the returned dst.
// Caller must hold rcu_read_lock().
//
    struct dst_entry *l3mdev_link_scope_lookup(struct net *net,
    struct flowi6 *fl6)
    {
    struct dst_entry *dst = core::ptr::null_mut();
    struct net_device *dev;
    WARN_ON_ONCE(!rcu_read_lock_held());
    if (fl6.flowi6_oif) {
    dev = dev_get_by_index_rcu(net, fl6.flowi6_oif);
    if (dev && netif_is_l3_slave(dev))
    dev = netdev_master_upper_dev_get_rcu(dev);
    if (dev && netif_is_l3_master(dev) &&
    dev.l3mdev_ops.l3mdev_link_scope_lookup)
    dst = dev.l3mdev_ops.l3mdev_link_scope_lookup(dev, fl6);
    }
    return dst;
    }
    EXPORT_SYMBOL_GPL(l3mdev_link_scope_lookup);
//
// l3mdev_fib_rule_match - Determine if flowi references an
// L3 master device
// @net: network namespace for device index lookup
// @fl:  flow struct
// @arg: store the table the rule matched with here
//
    int l3mdev_fib_rule_match(struct net *net, struct flowi *fl,
    struct fib_lookup_arg *arg)
    {
    struct net_device *dev;
    let mut rc: c_int = 0;
// update flow ensures flowi_l3mdev is set when relevant
    if (!fl.flowi_l3mdev)
    return 0;
    rcu_read_lock();
    dev = dev_get_by_index_rcu(net, fl.flowi_l3mdev);
    if (dev && netif_is_l3_master(dev) &&
    dev.l3mdev_ops.l3mdev_fib_table) {
    arg.table = dev.l3mdev_ops.l3mdev_fib_table(dev);
    rc = 1;
    }
    rcu_read_unlock();
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn l3mdev_update_flow(net: *mut net, fl: *mut flowi) {
    void l3mdev_update_flow(struct net *net, struct flowi *fl)
    {
    struct net_device *dev;
    rcu_read_lock();
    if (fl.flowi_oif) {
    dev = dev_get_by_index_rcu(net, fl.flowi_oif);
    if (dev) {
    if (!fl.flowi_l3mdev) {
    fl.flowi_l3mdev = l3mdev_master_ifindex_rcu(dev);
    fl.flowi_flags |= FLOWI_FLAG_L3MDEV_OIF;
    }
// oif set to L3mdev directs lookup to its table;
// reset to avoid oif match in fib_lookup
//
    if (netif_is_l3_master(dev))
    fl.flowi_oif = 0;
    goto out;
    }
    }
    if (fl.flowi_iif > LOOPBACK_IFINDEX && !fl.flowi_l3mdev) {
    dev = dev_get_by_index_rcu(net, fl.flowi_iif);
    if (dev)
    fl.flowi_l3mdev = l3mdev_master_ifindex_rcu(dev);
    }
    out:
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(l3mdev_update_flow);
