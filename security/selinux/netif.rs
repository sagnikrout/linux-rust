//! Automatically rewritten from C to Rust
//! Source: security/selinux/netif.c
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
// Network interface table.
//
// Network interfaces (devices) do not have a security field, so we
// maintain a table associating each interface with a SID.
//
// Author: James Morris <jmorris@redhat.com>
//
// Copyright (C) 2003 Red Hat, Inc., James Morris <jmorris@redhat.com>
// Copyright (C) 2007 Hewlett-Packard Development Company, L.P.
// Paul Moore <paul@paul-moore.com>
//

pub const SEL_NETIF_HASH_SIZE: c_int = 64;
pub const SEL_NETIF_HASH_MAX: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sel_netif {
    pub list: list_head,
    pub nsec: netif_security_struct,
    pub rcu_head: rcu_head,
}

    static u32 sel_netif_total;
    static DEFINE_SPINLOCK(sel_netif_lock);
    static struct list_head sel_netif_hash[SEL_NETIF_HASH_SIZE];
//
// sel_netif_hashfn - Hashing function for the interface table
// @ns: the network namespace
// @ifindex: the network interface
//
// Description:
// This is the hashing function for the network interface table, it returns the
// bucket number for the given interface.
//
#[no_mangle]
pub unsafe extern "C" fn sel_netif_hashfn(ns: *const net, ifindex: c_int) -> u32 {
    static inline u32 sel_netif_hashfn(const struct net *ns, int ifindex)
    {
    return (((uintptr_t)ns + ifindex) & (SEL_NETIF_HASH_SIZE - 1));
    }
//
// sel_netif_find - Search for an interface record
// @ns: the network namespace
// @ifindex: the network interface
//
// Description:
// Search the network interface table and return the record matching @ifindex.
// If an entry can not be found in the table return NULL.
//
    static inline struct sel_netif *sel_netif_find(const struct net *ns,
    int ifindex)
    {
    let mut idx: u32 = sel_netif_hashfn(ns, ifindex);
    struct sel_netif *netif;
    list_for_each_entry_rcu(netif, &sel_netif_hash[idx], list)
    if (net_eq(netif.nsec.ns, ns) &&
    netif.nsec.ifindex == ifindex)
    return netif;
    return core::ptr::null_mut();
    }
//
// sel_netif_insert - Insert a new interface into the table
// @netif: the new interface record
//
// Description:
// Add a new interface record to the network interface hash table.  Returns
// zero on success, negative values on failure.
//
#[no_mangle]
unsafe extern "C" fn sel_netif_insert(netif: *mut sel_netif) -> c_int {
    static int sel_netif_insert(struct sel_netif *netif)
    {
    u32 idx;
    if (sel_netif_total >= SEL_NETIF_HASH_MAX)
    return -ENOSPC;
    idx = sel_netif_hashfn(netif.nsec.ns, netif.nsec.ifindex);
    list_add_rcu(&netif.list, &sel_netif_hash[idx]);
    sel_netif_total++;
    return 0;
    }
//
// sel_netif_destroy - Remove an interface record from the table
// @netif: the existing interface record
//
// Description:
// Remove an existing interface record from the network interface table.
//
#[no_mangle]
unsafe extern "C" fn sel_netif_destroy(netif: *mut sel_netif) {
    static void sel_netif_destroy(struct sel_netif *netif)
    {
    list_del_rcu(&netif.list);
    sel_netif_total--;
    kfree_rcu(netif, rcu_head);
    }
//
// sel_netif_sid_slow - Lookup the SID of a network interface using the policy
// @ns: the network namespace
// @ifindex: the network interface
// @sid: interface SID
//
// Description:
// This function determines the SID of a network interface by querying the
// security policy.  The result is added to the network interface table to
// speedup future queries.  Returns zero on success, negative values on
// failure.
//
#[no_mangle]
unsafe extern "C" fn sel_netif_sid_slow(ns: *mut net, ifindex: c_int, sid: *mut u32) -> c_int {
    static int sel_netif_sid_slow(struct net *ns, int ifindex, u32 *sid)
    {
    let mut ret: c_int = 0;
    struct sel_netif *netif;
    struct sel_netif *new;
    struct net_device *dev;
// NOTE: we always use init's network namespace since we don't
// currently support containers
    dev = dev_get_by_index(ns, ifindex);
    if (unlikely(dev == core::ptr::null_mut())) {
    pr_warn("SELinux: failure in %s(), invalid network interface (%d)\n",
    __func__, ifindex);
    return -ENOENT;
    }
    spin_lock_bh(&sel_netif_lock);
    netif = sel_netif_find(ns, ifindex);
    if (netif != core::ptr::null_mut()) {
// sid = netif->nsec.sid;
    goto out;
    }
    ret = security_netif_sid(dev.name, sid);
    if (ret != 0)
    goto out;
// If this memory allocation fails still return 0. The SID
// is valid, it just won't be added to the cache.
//
    new = kmalloc_obj(*new, GFP_ATOMIC);
    if (new) {
    new.nsec.ns = ns;
    new.nsec.ifindex = ifindex;
    new.nsec.sid = *sid;
    if (sel_netif_insert(new))
    kfree(new);
    }
    out:
    spin_unlock_bh(&sel_netif_lock);
    dev_put(dev);
    if (unlikely(ret))
    pr_warn("SELinux: failure in %s(), unable to determine network interface label (%d)\n",
    __func__, ifindex);
    return ret;
    }
//
// sel_netif_sid - Lookup the SID of a network interface
// @ns: the network namespace
// @ifindex: the network interface
// @sid: interface SID
//
// Description:
// This function determines the SID of a network interface using the fastest
// method possible.  First the interface table is queried, but if an entry
// can't be found then the policy is queried and the result is added to the
// table to speedup future queries.  Returns zero on success, negative values
// on failure.
//
#[no_mangle]
pub unsafe extern "C" fn sel_netif_sid(ns: *mut net, ifindex: c_int, sid: *mut u32) -> c_int {
    int sel_netif_sid(struct net *ns, int ifindex, u32 *sid)
    {
    struct sel_netif *netif;
    rcu_read_lock();
    netif = sel_netif_find(ns, ifindex);
    if (likely(netif != core::ptr::null_mut())) {
// sid = netif->nsec.sid;
    rcu_read_unlock();
    return 0;
    }
    rcu_read_unlock();
    return sel_netif_sid_slow(ns, ifindex, sid);
    }
//
// sel_netif_kill - Remove an entry from the network interface table
// @ns: the network namespace
// @ifindex: the network interface
//
// Description:
// This function removes the entry matching @ifindex from the network interface
// table if it exists.
//
#[no_mangle]
unsafe extern "C" fn sel_netif_kill(ns: *const net, ifindex: c_int) {
    static void sel_netif_kill(const struct net *ns, int ifindex)
    {
    struct sel_netif *netif;
    rcu_read_lock();
    spin_lock_bh(&sel_netif_lock);
    netif = sel_netif_find(ns, ifindex);
    if (netif)
    sel_netif_destroy(netif);
    spin_unlock_bh(&sel_netif_lock);
    rcu_read_unlock();
    }
//
// sel_netif_flush - Flush the entire network interface table
//
// Description:
// Remove all entries from the network interface table.
//
#[no_mangle]
pub unsafe extern "C" fn sel_netif_flush() {
    void sel_netif_flush(void)
    {
    int idx;
    struct sel_netif *netif;
    spin_lock_bh(&sel_netif_lock);
    for (idx = 0; idx < SEL_NETIF_HASH_SIZE; idx++)
    list_for_each_entry(netif, &sel_netif_hash[idx], list)
    sel_netif_destroy(netif);
    spin_unlock_bh(&sel_netif_lock);
    }
    static int sel_netif_netdev_notifier_handler(struct notifier_block *this,
    unsigned long event, void *ptr)
    {
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    if (event == NETDEV_DOWN)
    sel_netif_kill(dev_net(dev), dev.ifindex);
    return NOTIFY_DONE;
    }
    static struct notifier_block sel_netif_netdev_notifier = {
    .notifier_call = sel_netif_netdev_notifier_handler,
    };
#[no_mangle]
pub unsafe extern "C" fn sel_netif_init() -> int __init {
    int __init sel_netif_init(void)
    {
    int i;
    if (!selinux_enabled_boot)
    return 0;
    for (i = 0; i < SEL_NETIF_HASH_SIZE; i++)
    INIT_LIST_HEAD(&sel_netif_hash[i]);
    register_netdevice_notifier(&sel_netif_netdev_notifier);
    return 0;
    }
