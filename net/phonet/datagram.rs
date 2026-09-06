//! Automatically rewritten from C to Rust
//! Source: net/phonet/datagram.c
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
// File: datagram.c
//
// Datagram (ISI) Phonet sockets
//
// Copyright (C) 2008 Nokia Corporation.
//
// Authors: Sakari Ailus <sakari.ailus@nokia.com>
// Rémi Denis-Courmont
//

    static int pn_backlog_rcv(struct sock *sk, struct sk_buff *skb);
// associated socket ceases to exist
#[no_mangle]
unsafe extern "C" fn pn_sock_close(sk: *mut sock, timeout: c_long) {
    static void pn_sock_close(struct sock *sk, long timeout)
    {
    sk_common_release(sk);
    }
#[no_mangle]
unsafe extern "C" fn pn_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int {
    static int pn_ioctl(struct sock *sk, int cmd, int *karg)
    {
    struct sk_buff *skb;
    switch (cmd) {
    case SIOCINQ:
    spin_lock_bh(&sk.sk_receive_queue.lock);
    skb = skb_peek(&sk.sk_receive_queue);
// karg = skb ? skb->len : 0;
    spin_unlock_bh(&sk.sk_receive_queue.lock);
    return 0;
    case SIOCPNADDRESOURCE:
    case SIOCPNDELRESOURCE: {
    let mut res: u32 = *karg;
    if (res >= 256)
    return -EINVAL;
    if (cmd == SIOCPNADDRESOURCE)
    return pn_sock_bind_res(sk, res);
    else
    return pn_sock_unbind_res(sk, res);
    }
    }
    return -ENOIOCTLCMD;
    }
// Destroy socket. All references are gone.
#[no_mangle]
unsafe extern "C" fn pn_destruct(sk: *mut sock) {
    static void pn_destruct(struct sock *sk)
    {
    skb_queue_purge(&sk.sk_receive_queue);
    }
#[no_mangle]
unsafe extern "C" fn pn_init(sk: *mut sock) -> c_int {
    static int pn_init(struct sock *sk)
    {
    sk.sk_destruct = pn_destruct;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pn_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int {
    static int pn_sendmsg(struct sock *sk, struct msghdr *msg, size_t len)
    {
    DECLARE_SOCKADDR(struct sockaddr_pn *, target, msg.msg_name);
    struct sk_buff *skb;
    int err;
    if (msg.msg_flags & ~(MSG_DONTWAIT|MSG_EOR|MSG_NOSIGNAL|
    MSG_CMSG_COMPAT))
    return -EOPNOTSUPP;
    if (target == core::ptr::null_mut())
    return -EDESTADDRREQ;
    if (msg.msg_namelen < sizeof(struct sockaddr_pn))
    return -EINVAL;
    if (target.spn_family != AF_PHONET)
    return -EAFNOSUPPORT;
    skb = sock_alloc_send_skb(sk, MAX_PHONET_HEADER + len,
    msg.msg_flags & MSG_DONTWAIT, &err);
    if (skb == core::ptr::null_mut())
    return err;
    skb_reserve(skb, MAX_PHONET_HEADER);
    err = memcpy_from_msg((void *)skb_put(skb, len), msg, len);
    if (err < 0) {
    kfree_skb(skb);
    return err;
    }
//
// Fill in the Phonet header and
// finally pass the packet forwards.
//
    err = pn_skb_send(sk, skb, target);
// If ok, return len.
    return (err >= 0) ? len : err;
    }
    static int pn_recvmsg(struct sock *sk, struct msghdr *msg, size_t len,
    int flags)
    {
    struct sk_buff *skb = core::ptr::null_mut();
    struct sockaddr_pn sa;
    let mut rval: c_int = -EOPNOTSUPP;
    int copylen;
    if (flags & ~(MSG_PEEK|MSG_TRUNC|MSG_DONTWAIT|MSG_NOSIGNAL|
    MSG_CMSG_COMPAT))
    goto out_nofree;
    skb = skb_recv_datagram(sk, flags, &rval);
    if (skb == core::ptr::null_mut())
    goto out_nofree;
    pn_skb_get_src_sockaddr(skb, &sa);
    copylen = skb.len;
    if (len < copylen) {
    msg.msg_flags |= MSG_TRUNC;
    copylen = len;
    }
    rval = skb_copy_datagram_msg(skb, 0, msg, copylen);
    if (rval) {
    rval = -EFAULT;
    goto out;
    }
    rval = (flags & MSG_TRUNC) ? skb.len : copylen;
    if (msg.msg_name != core::ptr::null_mut()) {
    __sockaddr_check_size(sizeof(sa));
    memcpy(msg.msg_name, &sa, sizeof(sa));
    msg.msg_namelen = sizeof(sa);
    }
    out:
    skb_free_datagram(sk, skb);
    out_nofree:
    return rval;
    }
// Queue an skb for a sock.
#[no_mangle]
unsafe extern "C" fn pn_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int pn_backlog_rcv(struct sock *sk, struct sk_buff *skb)
    {
    let mut err: c_int = sock_queue_rcv_skb(sk, skb);
    if (err < 0)
    kfree_skb(skb);
    return err ? NET_RX_DROP : NET_RX_SUCCESS;
    }
// Module registration
    static struct proto pn_proto = {
    .close		= pn_sock_close,
    .ioctl		= pn_ioctl,
    .init		= pn_init,
    .sendmsg	= pn_sendmsg,
    .recvmsg	= pn_recvmsg,
    .backlog_rcv	= pn_backlog_rcv,
    .hash		= pn_sock_hash,
    .unhash		= pn_sock_unhash,
    .get_port	= pn_sock_get_port,
    .obj_size	= sizeof(struct pn_sock),
    .owner		= THIS_MODULE,
    .name		= "PHONET",
    };
    static const struct phonet_protocol pn_dgram_proto = {
    .ops		= &phonet_dgram_ops,
    .prot		= &pn_proto,
    .sock_type	= SOCK_DGRAM,
    };
#[no_mangle]
pub unsafe extern "C" fn isi_register() -> int __init {
    int __init isi_register(void)
    {
    return phonet_proto_register(PN_PROTO_PHONET, &pn_dgram_proto);
    }
#[no_mangle]
pub unsafe extern "C" fn isi_unregister() -> void __exit {
    void __exit isi_unregister(void)
    {
    phonet_proto_unregister(PN_PROTO_PHONET, &pn_dgram_proto);
    }
