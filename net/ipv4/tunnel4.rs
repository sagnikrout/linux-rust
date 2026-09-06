//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tunnel4.c
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
// tunnel4.c: Generic IP tunnel transformer.
//
// Copyright (C) 2003 David S. Miller (davem@redhat.com)
//

    static struct xfrm_tunnel __rcu *tunnel4_handlers __read_mostly;
    static struct xfrm_tunnel __rcu *tunnel64_handlers __read_mostly;
    static struct xfrm_tunnel __rcu *tunnelmpls4_handlers __read_mostly;
    static DEFINE_MUTEX(tunnel4_mutex);
    static inline struct xfrm_tunnel __rcu **fam_handlers(unsigned short family)
    {
    return (family == AF_INET) ? &tunnel4_handlers :
    (family == AF_INET6) ? &tunnel64_handlers :
    &tunnelmpls4_handlers;
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm4_tunnel_register(handler: *mut xfrm_tunnel, family: c_ushort) -> c_int {
    int xfrm4_tunnel_register(struct xfrm_tunnel *handler, unsigned short family)
    {
    struct xfrm_tunnel __rcu **pprev;
    struct xfrm_tunnel *t;
    let mut ret: c_int = -EEXIST;
    let mut priority: c_int = handler.priority;
    mutex_lock(&tunnel4_mutex);
    for (pprev = fam_handlers(family);
    (t = rcu_dereference_protected(*pprev,
    lockdep_is_held(&tunnel4_mutex))) != core::ptr::null_mut();
    pprev = &t.next) {
    if (t.priority > priority)
    break;
    if (t.priority == priority)
    goto err;
    }
    handler.next = *pprev;
    rcu_assign_pointer(*pprev, handler);
    ret = 0;
    err:
    mutex_unlock(&tunnel4_mutex);
    return ret;
    }
    EXPORT_SYMBOL(xfrm4_tunnel_register);
#[no_mangle]
pub unsafe extern "C" fn xfrm4_tunnel_deregister(handler: *mut xfrm_tunnel, family: c_ushort) -> c_int {
    int xfrm4_tunnel_deregister(struct xfrm_tunnel *handler, unsigned short family)
    {
    struct xfrm_tunnel __rcu **pprev;
    struct xfrm_tunnel *t;
    let mut ret: c_int = -ENOENT;
    mutex_lock(&tunnel4_mutex);
    for (pprev = fam_handlers(family);
    (t = rcu_dereference_protected(*pprev,
    lockdep_is_held(&tunnel4_mutex))) != core::ptr::null_mut();
    pprev = &t.next) {
    if (t == handler) {
// pprev = handler->next;
    ret = 0;
    break;
    }
    }
    mutex_unlock(&tunnel4_mutex);
    synchronize_net();
    return ret;
    }
    EXPORT_SYMBOL(xfrm4_tunnel_deregister);

    for (handler = rcu_dereference(head);		\
    handler != core::ptr::null_mut();				\
    handler = rcu_dereference(handler.next))	\
#[no_mangle]
unsafe extern "C" fn tunnel4_rcv(skb: *mut sk_buff) -> c_int {
    static int tunnel4_rcv(struct sk_buff *skb)
    {
    struct xfrm_tunnel *handler;
    if (!pskb_may_pull(skb, sizeof(struct iphdr)))
    goto drop;
    for_each_tunnel_rcu(tunnel4_handlers, handler)
    if (!handler.handler(skb))
    return 0;
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    drop:
    kfree_skb(skb);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tunnel4_rcv_cb(skb: *mut sk_buff, proto: u8, err: c_int) -> c_int {
    static int tunnel4_rcv_cb(struct sk_buff *skb, u8 proto, int err)
    {
    struct xfrm_tunnel __rcu *head;
    struct xfrm_tunnel *handler;
    int ret;
    head = (proto == IPPROTO_IPIP) ? tunnel4_handlers : tunnel64_handlers;
    for_each_tunnel_rcu(head, handler) {
    if (handler.cb_handler) {
    ret = handler.cb_handler(skb, err);
    if (ret <= 0)
    return ret;
    }
    }
    return 0;
    }
    static const struct xfrm_input_afinfo tunnel4_input_afinfo = {
    .family		=	AF_INET,
    .is_ipip	=	true,
    .callback	=	tunnel4_rcv_cb,
    };

#[no_mangle]
unsafe extern "C" fn tunnel64_rcv(skb: *mut sk_buff) -> c_int {
    static int tunnel64_rcv(struct sk_buff *skb)
    {
    struct xfrm_tunnel *handler;
    if (!pskb_may_pull(skb, sizeof(struct ipv6hdr)))
    goto drop;
    for_each_tunnel_rcu(tunnel64_handlers, handler)
    if (!handler.handler(skb))
    return 0;
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    drop:
    kfree_skb(skb);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tunnelmpls4_rcv(skb: *mut sk_buff) -> c_int {
    static int tunnelmpls4_rcv(struct sk_buff *skb)
    {
    struct xfrm_tunnel *handler;
    if (!pskb_may_pull(skb, sizeof(struct mpls_label)))
    goto drop;
    for_each_tunnel_rcu(tunnelmpls4_handlers, handler)
    if (!handler.handler(skb))
    return 0;
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, 0);
    drop:
    kfree_skb(skb);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tunnel4_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int tunnel4_err(struct sk_buff *skb, u32 info)
    {
    struct xfrm_tunnel *handler;
    for_each_tunnel_rcu(tunnel4_handlers, handler)
    if (!handler.err_handler(skb, info))
    return 0;
    return -ENOENT;
    }

#[no_mangle]
unsafe extern "C" fn tunnel64_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int tunnel64_err(struct sk_buff *skb, u32 info)
    {
    struct xfrm_tunnel *handler;
    for_each_tunnel_rcu(tunnel64_handlers, handler)
    if (!handler.err_handler(skb, info))
    return 0;
    return -ENOENT;
    }

#[no_mangle]
unsafe extern "C" fn tunnelmpls4_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int tunnelmpls4_err(struct sk_buff *skb, u32 info)
    {
    struct xfrm_tunnel *handler;
    for_each_tunnel_rcu(tunnelmpls4_handlers, handler)
    if (!handler.err_handler(skb, info))
    return 0;
    return -ENOENT;
    }

    static const struct net_protocol tunnel4_protocol = {
    .handler	=	tunnel4_rcv,
    .err_handler	=	tunnel4_err,
    .no_policy	=	1,
    };

    static const struct net_protocol tunnel64_protocol = {
    .handler	=	tunnel64_rcv,
    .err_handler	=	tunnel64_err,
    .no_policy	=	1,
    };

    static const struct net_protocol tunnelmpls4_protocol = {
    .handler	=	tunnelmpls4_rcv,
    .err_handler	=	tunnelmpls4_err,
    .no_policy	=	1,
    };

#[no_mangle]
unsafe extern "C" fn tunnel4_init() -> int __init {
    static int __init tunnel4_init(void)
    {
    if (inet_add_protocol(&tunnel4_protocol, IPPROTO_IPIP))
    goto err;

    if (inet_add_protocol(&tunnel64_protocol, IPPROTO_IPV6)) {
    inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP);
    goto err;
    }

    if (inet_add_protocol(&tunnelmpls4_protocol, IPPROTO_MPLS)) {
    inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP);

    inet_del_protocol(&tunnel64_protocol, IPPROTO_IPV6);

    goto err;
    }

    if (xfrm_input_register_afinfo(&tunnel4_input_afinfo)) {
    inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP);

    inet_del_protocol(&tunnel64_protocol, IPPROTO_IPV6);

    inet_del_protocol(&tunnelmpls4_protocol, IPPROTO_MPLS);

    goto err;
    }

    return 0;
    err:
    pr_err("%s: can't add protocol\n", __func__);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn tunnel4_fini() -> void __exit {
    static void __exit tunnel4_fini(void)
    {

    if (xfrm_input_unregister_afinfo(&tunnel4_input_afinfo))
    pr_err("tunnel4 close: can't remove input afinfo\n");

    if (inet_del_protocol(&tunnelmpls4_protocol, IPPROTO_MPLS))
    pr_err("tunnelmpls4 close: can't remove protocol\n");

    if (inet_del_protocol(&tunnel64_protocol, IPPROTO_IPV6))
    pr_err("tunnel64 close: can't remove protocol\n");

    if (inet_del_protocol(&tunnel4_protocol, IPPROTO_IPIP))
    pr_err("tunnel4 close: can't remove protocol\n");
    }
    module_init(tunnel4_init);
    module_exit(tunnel4_fini);
    MODULE_DESCRIPTION("IPv4 XFRM tunnel library");
    MODULE_LICENSE("GPL");
