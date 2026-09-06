//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_sockopt.c
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

// Sockopts only registered and called from user context, so
    net locking would be overkill.  Also, [gs]etsockopt calls may
    sleep. */
    static DEFINE_MUTEX(nf_sockopt_mutex);
    static LIST_HEAD(nf_sockopts);
// Do exclusive ranges overlap?
#[no_mangle]
pub unsafe extern "C" fn overlap(min1: c_int, max1: c_int, min2: c_int, max2: c_int) -> c_int {
    static inline int overlap(int min1, int max1, int min2, int max2)
    {
    return max1 > min2 && min1 < max2;
    }
// Functions to register sockopt ranges (exclusive).
#[no_mangle]
pub unsafe extern "C" fn nf_register_sockopt(reg: *mut nf_sockopt_ops) -> c_int {
    int nf_register_sockopt(struct nf_sockopt_ops *reg)
    {
    struct nf_sockopt_ops *ops;
    let mut ret: c_int = 0;
    mutex_lock(&nf_sockopt_mutex);
    list_for_each_entry(ops, &nf_sockopts, list) {
    if (ops.pf == reg.pf
    && (overlap(ops.set_optmin, ops.set_optmax,
    reg.set_optmin, reg.set_optmax)
    || overlap(ops.get_optmin, ops.get_optmax,
    reg.get_optmin, reg.get_optmax))) {
    pr_debug("nf_sock overlap: %u-%u/%u-%u v %u-%u/%u-%u\n",
    ops.set_optmin, ops.set_optmax,
    ops.get_optmin, ops.get_optmax,
    reg.set_optmin, reg.set_optmax,
    reg.get_optmin, reg.get_optmax);
    ret = -EBUSY;
    goto out;
    }
    }
    list_add(&reg.list, &nf_sockopts);
    out:
    mutex_unlock(&nf_sockopt_mutex);
    return ret;
    }
    EXPORT_SYMBOL(nf_register_sockopt);
#[no_mangle]
pub unsafe extern "C" fn nf_unregister_sockopt(reg: *mut nf_sockopt_ops) {
    void nf_unregister_sockopt(struct nf_sockopt_ops *reg)
    {
    mutex_lock(&nf_sockopt_mutex);
    list_del(&reg.list);
    mutex_unlock(&nf_sockopt_mutex);
    }
    EXPORT_SYMBOL(nf_unregister_sockopt);
    static struct nf_sockopt_ops *nf_sockopt_find(struct sock *sk, u_int8_t pf,
    int val, int get)
    {
    struct nf_sockopt_ops *ops;
    mutex_lock(&nf_sockopt_mutex);
    list_for_each_entry(ops, &nf_sockopts, list) {
    if (ops.pf == pf) {
    if (!try_module_get(ops.owner))
    goto out_nosup;
    if (get) {
    if (val >= ops.get_optmin &&
    val < ops.get_optmax)
    goto out;
    } else {
    if (val >= ops.set_optmin &&
    val < ops.set_optmax)
    goto out;
    }
    module_put(ops.owner);
    }
    }
    out_nosup:
    ops = ERR_PTR(-ENOPROTOOPT);
    out:
    mutex_unlock(&nf_sockopt_mutex);
    return ops;
    }
    int nf_setsockopt(struct sock *sk, u_int8_t pf, int val, sockptr_t opt,
    unsigned int len)
    {
    struct nf_sockopt_ops *ops;
    int ret;
    ops = nf_sockopt_find(sk, pf, val, 0);
    if (IS_ERR(ops))
    return PTR_ERR(ops);
    ret = ops.set(sk, val, opt, len);
    module_put(ops.owner);
    return ret;
    }
    EXPORT_SYMBOL(nf_setsockopt);
    int nf_getsockopt(struct sock *sk, u_int8_t pf, int val, char __user *opt,
    int *len)
    {
    struct nf_sockopt_ops *ops;
    int ret;
    ops = nf_sockopt_find(sk, pf, val, 1);
    if (IS_ERR(ops))
    return PTR_ERR(ops);
    ret = ops.get(sk, val, opt, len);
    module_put(ops.owner);
    return ret;
    }
    EXPORT_SYMBOL(nf_getsockopt);
