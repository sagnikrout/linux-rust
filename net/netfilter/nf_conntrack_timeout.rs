//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_timeout.c
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
// (C) 2012 by Pablo Neira Ayuso <pablo@netfilter.org>
// (C) 2012 by Vyatta Inc. <http://www.vyatta.com>
//

    const struct nf_ct_timeout_hooks __rcu *nf_ct_timeout_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nf_ct_timeout_hook);
// nf_ct_iterate_cleanup() holds the conntrack lock.
#[no_mangle]
unsafe extern "C" fn untimeout(ct: *mut nf_conn, timeout: *mut c_void) -> c_int {
    static int untimeout(struct nf_conn *ct, void *timeout)
    {
    struct nf_conn_timeout *timeout_ext = nf_ct_timeout_find(ct);
    if (timeout_ext) {
    struct nf_ct_timeout *t;
    rcu_read_lock();
    t = rcu_dereference(timeout_ext.timeout);
    if (!t) {
    rcu_read_unlock();
    return 0;
    }
    if (!timeout || t == timeout) {
    RCU_INIT_POINTER(timeout_ext.timeout, core::ptr::null_mut());
// No race with nf_conntrack_free() which is called
// only after the conntrack has been removed from
// the hashes.
//
    if (refcount_dec_and_test(&t.refcnt))
    kfree_rcu(t, rcu);
    }
    rcu_read_unlock();
    }
// We are not intended to delete this conntrack.
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nf_ct_untimeout(net: *mut net, timeout: *mut nf_ct_timeout) {
    void nf_ct_untimeout(struct net *net, struct nf_ct_timeout *timeout)
    {
    struct nf_ct_iter_data iter_data = {
    .net	= net,
    .data	= timeout,
    };
    nf_ct_iterate_cleanup_net(untimeout, &iter_data);
    }
    EXPORT_SYMBOL_GPL(nf_ct_untimeout);
#[no_mangle]
unsafe extern "C" fn __nf_ct_timeout_put(timeout: *mut nf_ct_timeout) {
    static void __nf_ct_timeout_put(struct nf_ct_timeout *timeout)
    {
    const struct nf_ct_timeout_hooks *h = rcu_dereference(nf_ct_timeout_hook);
    if (h)
    h.timeout_put(timeout);
    }
    int nf_ct_set_timeout(struct net *net, struct nf_conn *ct,
    u8 l3num, u8 l4num, const char *timeout_name)
    {
    const struct nf_ct_timeout_hooks *h;
    struct nf_ct_timeout *timeout;
    struct nf_conn_timeout *timeout_ext;
    const char *errmsg = core::ptr::null_mut();
    let mut ret: c_int = 0;
    WARN_ON_ONCE(!nf_ct_is_template(ct));
    rcu_read_lock();
    h = rcu_dereference(nf_ct_timeout_hook);
    if (!h) {
    ret = -ENOENT;
    errmsg = "Timeout policy base is empty";
    goto out;
    }
    timeout = h.timeout_find_get(net, timeout_name);
    if (!timeout) {
    ret = -ENOENT;
    pr_info_ratelimited("No such timeout policy \"%s\"\n",
    timeout_name);
    goto out;
    }
    if (timeout.l3num != l3num) {
    ret = -EINVAL;
    pr_info_ratelimited("Timeout policy `%s' can only be used by "
    "L%d protocol number %d\n",
    timeout_name, 3, timeout.l3num);
    goto err_put_timeout;
    }
// Make sure the timeout policy matches any existing protocol tracker,
// otherwise default to generic.
//
    if (timeout.l4proto.l4proto != l4num) {
    ret = -EINVAL;
    pr_info_ratelimited("Timeout policy `%s' can only be used by "
    "L%d protocol number %d\n",
    timeout_name, 4, timeout.l4proto.l4proto);
    goto err_put_timeout;
    }
    timeout_ext = nf_ct_timeout_ext_add(ct, timeout, GFP_ATOMIC);
    if (!timeout_ext) {
    ret = -ENOMEM;
    goto err_put_timeout;
    }
    rcu_read_unlock();
    return ret;
    err_put_timeout:
    __nf_ct_timeout_put(timeout);
    out:
    rcu_read_unlock();
    if (errmsg)
    pr_info_ratelimited("%s\n", errmsg);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nf_ct_set_timeout);
#[no_mangle]
pub unsafe extern "C" fn nf_ct_destroy_timeout(ct: *mut nf_conn) {
    void nf_ct_destroy_timeout(struct nf_conn *ct)
    {
    struct nf_conn_timeout *timeout_ext;
    const struct nf_ct_timeout_hooks *h;
    WARN_ON_ONCE(!nf_ct_is_template(ct));
    rcu_read_lock();
    h = rcu_dereference(nf_ct_timeout_hook);
    if (h) {
    timeout_ext = nf_ct_timeout_find(ct);
    if (timeout_ext) {
    struct nf_ct_timeout *t;
    t = rcu_dereference(timeout_ext.timeout);
    if (t)
    h.timeout_put(t);
    RCU_INIT_POINTER(timeout_ext.timeout, core::ptr::null_mut());
    if (t && refcount_dec_and_test(&t.refcnt))
    kfree_rcu(t, rcu);
    }
    }
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(nf_ct_destroy_timeout);
