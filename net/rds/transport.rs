//! Automatically rewritten from C to Rust
//! Source: net/rds/transport.c
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


//
// Copyright (c) 2006, 2017 Oracle and/or its affiliates. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    static char * const rds_trans_modules[] = {
    [RDS_TRANS_IB] = "rds_rdma",
    [RDS_TRANS_GAP] = core::ptr::null_mut(),
    [RDS_TRANS_TCP] = "rds_tcp",
    };
    static struct rds_transport *transports[RDS_TRANS_COUNT];
    static DECLARE_RWSEM(rds_trans_sem);
#[no_mangle]
pub unsafe extern "C" fn rds_trans_register(trans: *mut rds_transport) {
    void rds_trans_register(struct rds_transport *trans)
    {
    BUG_ON(strlen(trans.t_name) + 1 > TRANSNAMSIZ);
    down_write(&rds_trans_sem);
    if (transports[trans.t_type])
    printk(KERN_ERR "RDS Transport type %d already registered\n",
    trans.t_type);
    else {
    transports[trans.t_type] = trans;
    printk(KERN_INFO "Registered RDS/%s transport\n", trans.t_name);
    }
    up_write(&rds_trans_sem);
    }
    EXPORT_SYMBOL_GPL(rds_trans_register);
#[no_mangle]
pub unsafe extern "C" fn rds_trans_unregister(trans: *mut rds_transport) {
    void rds_trans_unregister(struct rds_transport *trans)
    {
    down_write(&rds_trans_sem);
    transports[trans.t_type] = core::ptr::null_mut();
    printk(KERN_INFO "Unregistered RDS/%s transport\n", trans.t_name);
    up_write(&rds_trans_sem);
    }
    EXPORT_SYMBOL_GPL(rds_trans_unregister);
#[no_mangle]
pub unsafe extern "C" fn rds_trans_put(trans: *mut rds_transport) {
    void rds_trans_put(struct rds_transport *trans)
    {
    if (trans)
    module_put(trans.t_owner);
    }
    struct rds_transport *rds_trans_get_preferred(struct net *net,
    const struct in6_addr *addr,
    __u32 scope_id)
    {
    struct rds_transport *ret = core::ptr::null_mut();
    struct rds_transport *trans;
    unsigned int i;
    if (ipv6_addr_v4mapped(addr)) {
    if (*(u_int8_t *)&addr.s6_addr32[3] == IN_LOOPBACKNET)
    return &rds_loop_transport;
    } else if (ipv6_addr_loopback(addr)) {
    return &rds_loop_transport;
    }
    down_read(&rds_trans_sem);
    for (i = 0; i < RDS_TRANS_COUNT; i++) {
    trans = transports[i];
    if (trans && (trans.laddr_check(net, addr, scope_id) == 0) &&
    (!trans.t_owner || try_module_get(trans.t_owner))) {
    ret = trans;
    break;
    }
    }
    up_read(&rds_trans_sem);
    return ret;
    }
    struct rds_transport *rds_trans_get(int t_type)
    {
    struct rds_transport *ret = core::ptr::null_mut();
    struct rds_transport *trans;
    down_read(&rds_trans_sem);
    trans = transports[t_type];
    if (!trans) {
    up_read(&rds_trans_sem);
    if (rds_trans_modules[t_type])
    request_module(rds_trans_modules[t_type]);
    down_read(&rds_trans_sem);
    trans = transports[t_type];
    }
    if (trans && trans.t_type == t_type &&
    (!trans.t_owner || try_module_get(trans.t_owner)))
    ret = trans;
    up_read(&rds_trans_sem);
    return ret;
    }
//
// This returns the number of stats entries in the snapshot and only
// copies them using the iter if there is enough space for them.  The
// caller passes in the global stats so that we can size and copy while
// holding the lock.
//
    unsigned int rds_trans_stats_info_copy(struct rds_info_iterator *iter,
    unsigned int avail)
    {
    struct rds_transport *trans;
    let mut total: c_uint = 0;
    unsigned int part;
    int i;
    rds_info_iter_unmap(iter);
    down_read(&rds_trans_sem);
    for (i = 0; i < RDS_TRANS_COUNT; i++) {
    trans = transports[i];
    if (!trans || !trans.stats_info_copy)
    continue;
    part = trans.stats_info_copy(iter, avail);
    avail -= min(avail, part);
    total += part;
    }
    up_read(&rds_trans_sem);
    return total;
    }
