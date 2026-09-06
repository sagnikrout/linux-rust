//! Automatically rewritten from C to Rust
//! Source: drivers/net/ppp/pppox.c
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
// -*- linux-c -*-
// Linux PPP over X/Ethernet (PPPoX/PPPoE) Sockets
//
// PPPoX --- Generic PPP encapsulation socket family
// PPPoE --- PPP over Ethernet (RFC 2516)
//
// Version:	0.5.2
//
// Author:	Michal Ostrowski <mostrows@speakeasy.net>
//
// 051000 :	Initialization cleanup
//
// License:
//

    static const struct pppox_proto *pppox_protos[PX_MAX_PROTO + 1];
#[no_mangle]
pub unsafe extern "C" fn register_pppox_proto(proto_num: c_int, pp: *const pppox_proto) -> c_int {
    int register_pppox_proto(int proto_num, const struct pppox_proto *pp)
    {
    if (proto_num < 0 || proto_num > PX_MAX_PROTO)
    return -EINVAL;
    if (pppox_protos[proto_num])
    return -EALREADY;
    pppox_protos[proto_num] = pp;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_pppox_proto(proto_num: c_int) {
    void unregister_pppox_proto(int proto_num)
    {
    if (proto_num >= 0 && proto_num <= PX_MAX_PROTO)
    pppox_protos[proto_num] = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pppox_unbind_sock(sk: *mut sock) {
    void pppox_unbind_sock(struct sock *sk)
    {
// Clear connection to ppp device, if attached.
    if (sk.sk_state & (PPPOX_BOUND | PPPOX_CONNECTED)) {
    ppp_unregister_channel(&pppox_sk(sk).chan);
    sk.sk_state = PPPOX_DEAD;
    }
    }
    EXPORT_SYMBOL(register_pppox_proto);
    EXPORT_SYMBOL(unregister_pppox_proto);
    EXPORT_SYMBOL(pppox_unbind_sock);
#[no_mangle]
pub unsafe extern "C" fn pppox_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    int pppox_ioctl(struct socket *sock, unsigned int cmd, unsigned long arg)
    {
    struct sock *sk = sock.sk;
    struct pppox_sock *po = pppox_sk(sk);
    int rc;
    lock_sock(sk);
    switch (cmd) {
    case PPPIOCGCHAN: {
    struct sk_buff *skb;
    int index;
    rc = -ENOTCONN;
    if (!(sk.sk_state & PPPOX_CONNECTED))
    break;
    rc = -EINVAL;
    index = ppp_channel_index(&po.chan);
    if (put_user(index , (int __user *) arg))
    break;
    rc = 0;
// PPPIOCGCHAN historically marks the userspace handoff to
// generic PPP; pppd then attaches the returned channel to
// /dev/ppp.
//
    sk.sk_state |= PPPOX_BOUND;
// Let lockless receive paths finish queueing against the old
// state.
//
    synchronize_net();
// Drain packets queued before the handoff because a bound
// socket is no longer readable.
//
    while ((skb = skb_dequeue(&sk.sk_receive_queue))) {
    skb_orphan(skb);
    ppp_input(&po.chan, skb);
    }
    break;
    }
    default:
    rc = pppox_protos[sk.sk_protocol].ioctl ?
    pppox_protos[sk.sk_protocol].ioctl(sock, cmd, arg) : -ENOTTY;
    }
    release_sock(sk);
    return rc;
    }
    EXPORT_SYMBOL(pppox_ioctl);

#[no_mangle]
pub unsafe extern "C" fn pppox_compat_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    int pppox_compat_ioctl(struct socket *sock, unsigned int cmd, unsigned long arg)
    {
    return pppox_ioctl(sock, cmd, (unsigned long)compat_ptr(arg));
    }
    EXPORT_SYMBOL(pppox_compat_ioctl);

    static int pppox_create(struct net *net, struct socket *sock, int protocol,
    int kern)
    {
    let mut rc: c_int = -EPROTOTYPE;
    if (protocol < 0 || protocol > PX_MAX_PROTO)
    goto out;
    rc = -EPROTONOSUPPORT;
    if (!pppox_protos[protocol])
    request_module("net-pf-%d-proto-%d", PF_PPPOX, protocol);
    if (!pppox_protos[protocol] ||
    !try_module_get(pppox_protos[protocol].owner))
    goto out;
    rc = pppox_protos[protocol].create(net, sock, kern);
    module_put(pppox_protos[protocol].owner);
    out:
    return rc;
    }
    static const struct net_proto_family pppox_proto_family = {
    .family	= PF_PPPOX,
    .create	= pppox_create,
    .owner	= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn pppox_init() -> int __init {
    static int __init pppox_init(void)
    {
    return sock_register(&pppox_proto_family);
    }
#[no_mangle]
unsafe extern "C" fn pppox_exit() -> void __exit {
    static void __exit pppox_exit(void)
    {
    sock_unregister(PF_PPPOX);
    }
    module_init(pppox_init);
    module_exit(pppox_exit);
    MODULE_AUTHOR("Michal Ostrowski <mostrows@speakeasy.net>");
    MODULE_DESCRIPTION("PPP over Ethernet driver (generic socket layer)");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_NETPROTO(PF_PPPOX);
