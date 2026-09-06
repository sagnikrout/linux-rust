//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_ulp.c
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
// Pluggable TCP upper layer protocol support.
//
// Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
// Copyright (c) 2016-2017, Dave Watson <davejwatson@fb.com>. All rights reserved.
//

    static DEFINE_SPINLOCK(tcp_ulp_list_lock);
    static LIST_HEAD(tcp_ulp_list);
// Simple linear search, don't expect many entries!
    static struct tcp_ulp_ops *tcp_ulp_find(const char *name)
    {
    struct tcp_ulp_ops *e;
    list_for_each_entry_rcu(e, &tcp_ulp_list, list,
    lockdep_is_held(&tcp_ulp_list_lock)) {
    if (strcmp(e.name, name) == 0)
    return e;
    }
    return core::ptr::null_mut();
    }
    static const struct tcp_ulp_ops *__tcp_ulp_find_autoload(const char *name)
    {
    const struct tcp_ulp_ops *ulp = core::ptr::null_mut();
    rcu_read_lock();
    ulp = tcp_ulp_find(name);

    if (!ulp && capable(CAP_NET_ADMIN)) {
    rcu_read_unlock();
    request_module("tcp-ulp-%s", name);
    rcu_read_lock();
    ulp = tcp_ulp_find(name);
    }

    if (!ulp || !try_module_get(ulp.owner))
    ulp = core::ptr::null_mut();
    rcu_read_unlock();
    return ulp;
    }
// Attach new upper layer protocol to the list
// of available protocols.
//
#[no_mangle]
pub unsafe extern "C" fn tcp_register_ulp(ulp: *mut tcp_ulp_ops) -> c_int {
    int tcp_register_ulp(struct tcp_ulp_ops *ulp)
    {
    let mut ret: c_int = 0;
    spin_lock(&tcp_ulp_list_lock);
    if (tcp_ulp_find(ulp.name))
    ret = -EEXIST;
    else
    list_add_tail_rcu(&ulp.list, &tcp_ulp_list);
    spin_unlock(&tcp_ulp_list_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(tcp_register_ulp);
#[no_mangle]
pub unsafe extern "C" fn tcp_unregister_ulp(ulp: *mut tcp_ulp_ops) {
    void tcp_unregister_ulp(struct tcp_ulp_ops *ulp)
    {
    spin_lock(&tcp_ulp_list_lock);
    list_del_rcu(&ulp.list);
    spin_unlock(&tcp_ulp_list_lock);
    synchronize_rcu();
    }
    EXPORT_SYMBOL_GPL(tcp_unregister_ulp);
// Build string with list of available upper layer protocl values
#[no_mangle]
pub unsafe extern "C" fn tcp_get_available_ulp(buf: *mut c_char, maxlen: usize) {
    void tcp_get_available_ulp(char *buf, size_t maxlen)
    {
    struct tcp_ulp_ops *ulp_ops;
    let mut offs: usize = 0;
// buf = '\0';
    rcu_read_lock();
    list_for_each_entry_rcu(ulp_ops, &tcp_ulp_list, list) {
    offs += snprintf(buf + offs, maxlen - offs,
    "%s%s",
    offs == 0 ? "" : " ", ulp_ops.name);
    if (WARN_ON_ONCE(offs >= maxlen))
    break;
    }
    rcu_read_unlock();
    }
    void tcp_update_ulp(struct sock *sk, struct proto *proto,
    void (*write_space)(struct sock *sk))
    {
    struct inet_connection_sock *icsk = inet_csk(sk);
    if (icsk.icsk_ulp_ops.update)
    icsk.icsk_ulp_ops.update(sk, proto, write_space);
    }
#[no_mangle]
pub unsafe extern "C" fn tcp_cleanup_ulp(sk: *mut sock) {
    void tcp_cleanup_ulp(struct sock *sk)
    {
    struct inet_connection_sock *icsk = inet_csk(sk);
// No sock_owned_by_me() check here as at the time the
// stack calls this function, the socket is dead and
// about to be destroyed.
//
    if (!icsk.icsk_ulp_ops)
    return;
    if (icsk.icsk_ulp_ops.release)
    icsk.icsk_ulp_ops.release(sk);
    module_put(icsk.icsk_ulp_ops.owner);
    icsk.icsk_ulp_ops = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __tcp_set_ulp(sk: *mut sock, ulp_ops: *const tcp_ulp_ops) -> c_int {
    static int __tcp_set_ulp(struct sock *sk, const struct tcp_ulp_ops *ulp_ops)
    {
    struct inet_connection_sock *icsk = inet_csk(sk);
    int err;
    err = -EEXIST;
    if (icsk.icsk_ulp_ops)
    goto out_err;
    if (sk.sk_socket)
    clear_bit(SOCK_SUPPORT_ZC, &sk.sk_socket.flags);
    err = -ENOTCONN;
    if (!ulp_ops.clone && sk.sk_state == TCP_LISTEN)
    goto out_err;
    err = ulp_ops.init(sk);
    if (err)
    goto out_err;
    icsk.icsk_ulp_ops = ulp_ops;
    return 0;
    out_err:
    module_put(ulp_ops.owner);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn tcp_set_ulp(sk: *mut sock, name: *const c_char) -> c_int {
    int tcp_set_ulp(struct sock *sk, const char *name)
    {
    const struct tcp_ulp_ops *ulp_ops;
    sock_owned_by_me(sk);
    ulp_ops = __tcp_ulp_find_autoload(name);
    if (!ulp_ops)
    return -ENOENT;
    return __tcp_set_ulp(sk, ulp_ops);
    }
