//! Automatically rewritten from C to Rust
//! Source: security/selinux/netport.c
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
// Network port table
//
// SELinux must keep a mapping of network ports to labels/SIDs.  This
// mapping is maintained as part of the normal policy but a fast cache is
// needed to reduce the lookup overhead.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// This code is heavily based on the "netif" concept originally developed by
// James Morris <jmorris@redhat.com>
// (see security/selinux/netif.c for more information)
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2008
//

pub const SEL_NETPORT_HASH_SIZE: c_int = 256;
pub const SEL_NETPORT_HASH_BKT_LIMIT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sel_netport_bkt {
    pub size: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sel_netport {
    pub psec: netport_security_struct,
    pub list: list_head,
    pub rcu: rcu_head,
}

    static DEFINE_SPINLOCK(sel_netport_lock);
    static struct sel_netport_bkt sel_netport_hash[SEL_NETPORT_HASH_SIZE];
//
// sel_netport_hashfn - Hashing function for the port table
// @pnum: port number
//
// Description:
// This is the hashing function for the port table, it returns the bucket
// number for the given port.
//
#[no_mangle]
unsafe extern "C" fn sel_netport_hashfn(pnum: u16) -> c_uint {
    static unsigned int sel_netport_hashfn(u16 pnum)
    {
    return (pnum & (SEL_NETPORT_HASH_SIZE - 1));
    }
//
// sel_netport_find - Search for a port record
// @protocol: protocol
// @pnum: port
//
// Description:
// Search the network port table and return the matching record.  If an entry
// can not be found in the table return NULL.
//
    static struct sel_netport *sel_netport_find(u8 protocol, u16 pnum)
    {
    unsigned int idx;
    struct sel_netport *port;
    idx = sel_netport_hashfn(pnum);
    list_for_each_entry_rcu(port, &sel_netport_hash[idx].list, list)
    if (port.psec.port == pnum && port.psec.protocol == protocol)
    return port;
    return core::ptr::null_mut();
    }
//
// sel_netport_insert - Insert a new port into the table
// @port: the new port record
//
// Description:
// Add a new port record to the network address hash table.
//
#[no_mangle]
unsafe extern "C" fn sel_netport_insert(port: *mut sel_netport) {
    static void sel_netport_insert(struct sel_netport *port)
    {
    unsigned int idx;
// we need to impose a limit on the growth of the hash table so check
// this bucket to make sure it is within the specified bounds
    idx = sel_netport_hashfn(port.psec.port);
    list_add_rcu(&port.list, &sel_netport_hash[idx].list);
    if (sel_netport_hash[idx].size == SEL_NETPORT_HASH_BKT_LIMIT) {
    struct sel_netport *tail;
    tail = list_entry(
    rcu_dereference_protected(
    list_tail_rcu(&sel_netport_hash[idx].list),
    lockdep_is_held(&sel_netport_lock)),
    struct sel_netport, list);
    list_del_rcu(&tail.list);
    kfree_rcu(tail, rcu);
    } else
    sel_netport_hash[idx].size++;
    }
//
// sel_netport_sid_slow - Lookup the SID of a network address using the policy
// @protocol: protocol
// @pnum: port
// @sid: port SID
//
// Description:
// This function determines the SID of a network port by querying the security
// policy.  The result is added to the network port table to speedup future
// queries.  Returns zero on success, negative values on failure.
//
#[no_mangle]
unsafe extern "C" fn sel_netport_sid_slow(protocol: u8, pnum: u16, sid: *mut u32) -> c_int {
    static int sel_netport_sid_slow(u8 protocol, u16 pnum, u32 *sid)
    {
    int ret;
    struct sel_netport *port;
    struct sel_netport *new;
    spin_lock_bh(&sel_netport_lock);
    port = sel_netport_find(protocol, pnum);
    if (port != core::ptr::null_mut()) {
// sid = port->psec.sid;
    spin_unlock_bh(&sel_netport_lock);
    return 0;
    }
    ret = security_port_sid(protocol, pnum, sid);
    if (ret != 0)
    goto out;
// If this memory allocation fails still return 0. The SID
// is valid, it just won't be added to the cache.
//
    new = kmalloc_obj(*new, GFP_ATOMIC);
    if (new) {
    new.psec.port = pnum;
    new.psec.protocol = protocol;
    new.psec.sid = *sid;
    sel_netport_insert(new);
    }
    out:
    spin_unlock_bh(&sel_netport_lock);
    if (unlikely(ret))
    pr_warn("SELinux: failure in %s(), unable to determine network port label\n",
    __func__);
    return ret;
    }
//
// sel_netport_sid - Lookup the SID of a network port
// @protocol: protocol
// @pnum: port
// @sid: port SID
//
// Description:
// This function determines the SID of a network port using the fastest method
// possible.  First the port table is queried, but if an entry can't be found
// then the policy is queried and the result is added to the table to speedup
// future queries.  Returns zero on success, negative values on failure.
//
#[no_mangle]
pub unsafe extern "C" fn sel_netport_sid(protocol: u8, pnum: u16, sid: *mut u32) -> c_int {
    int sel_netport_sid(u8 protocol, u16 pnum, u32 *sid)
    {
    struct sel_netport *port;
    rcu_read_lock();
    port = sel_netport_find(protocol, pnum);
    if (likely(port != core::ptr::null_mut())) {
// sid = port->psec.sid;
    rcu_read_unlock();
    return 0;
    }
    rcu_read_unlock();
    return sel_netport_sid_slow(protocol, pnum, sid);
    }
//
// sel_netport_flush - Flush the entire network port table
//
// Description:
// Remove all entries from the network address table.
//
#[no_mangle]
pub unsafe extern "C" fn sel_netport_flush() {
    void sel_netport_flush(void)
    {
    unsigned int idx;
    struct sel_netport *port, *port_tmp;
    spin_lock_bh(&sel_netport_lock);
    for (idx = 0; idx < SEL_NETPORT_HASH_SIZE; idx++) {
    list_for_each_entry_safe(port, port_tmp,
    &sel_netport_hash[idx].list, list) {
    list_del_rcu(&port.list);
    kfree_rcu(port, rcu);
    }
    sel_netport_hash[idx].size = 0;
    }
    spin_unlock_bh(&sel_netport_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn sel_netport_init() -> int __init {
    int __init sel_netport_init(void)
    {
    int iter;
    if (!selinux_enabled_boot)
    return 0;
    for (iter = 0; iter < SEL_NETPORT_HASH_SIZE; iter++) {
    INIT_LIST_HEAD(&sel_netport_hash[iter].list);
    sel_netport_hash[iter].size = 0;
    }
    return 0;
    }
