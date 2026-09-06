//! Automatically rewritten from C to Rust
//! Source: net/ipv4/ipcomp.c
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
// IP Payload Compression Protocol (IPComp) - RFC3173.
//
// Copyright (c) 2003 James Morris <jmorris@intercode.com.au>
//
// Todo:
// - Tunable compression parameters.
// - Compression stats.
// - Adaptive compression.
//

#[no_mangle]
unsafe extern "C" fn ipcomp4_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int ipcomp4_err(struct sk_buff *skb, u32 info)
    {
    struct net *net = dev_net(skb.dev);
    __be32 spi;
    const struct iphdr *iph = (const struct iphdr *)skb.data;
    struct ip_comp_hdr *ipch = (struct ip_comp_hdr *)(skb.data+(iph.ihl<<2));
    struct xfrm_state *x;
    switch (icmp_hdr(skb).type) {
    case ICMP_DEST_UNREACH:
    if (icmp_hdr(skb).code != ICMP_FRAG_NEEDED)
    return 0;
    break;
    case ICMP_REDIRECT:
    break;
    default:
    return 0;
    }
    spi = htonl(ntohs(ipch.cpi));
    x = xfrm_state_lookup(net, skb.mark, (const xfrm_address_t *)&iph.daddr,
    spi, IPPROTO_COMP, AF_INET);
    if (!x)
    return 0;
    if (icmp_hdr(skb).type == ICMP_DEST_UNREACH)
    ipv4_update_pmtu(skb, net, info, 0, IPPROTO_COMP);
    else
    ipv4_redirect(skb, net, 0, IPPROTO_COMP);
    xfrm_state_put(x);
    return 0;
    }
// We always hold one tunnel user reference to indicate a tunnel
    static struct lock_class_key xfrm_state_lock_key;
    static struct xfrm_state *ipcomp_tunnel_create(struct xfrm_state *x)
    {
    struct net *net = xs_net(x);
    struct xfrm_state *t;
    t = xfrm_state_alloc(net);
    if (!t)
    goto out;
    lockdep_set_class(&t.lock, &xfrm_state_lock_key);
    t.id.proto = IPPROTO_IPIP;
    t.id.spi = x.props.saddr.a4;
    t.id.daddr.a4 = x.id.daddr.a4;
    memcpy(&t.sel, &x.sel, sizeof(t.sel));
    t.props.family = AF_INET;
    t.props.mode = x.props.mode;
    t.props.saddr.a4 = x.props.saddr.a4;
    t.props.flags = x.props.flags;
    t.props.extra_flags = x.props.extra_flags;
    memcpy(&t.mark, &x.mark, sizeof(t.mark));
    t.if_id = x.if_id;
    if (xfrm_init_state(t, core::ptr::null_mut()))
    goto error;
    atomic_set(&t.tunnel_users, 1);
    out:
    return t;
    error:
    t.km.state = XFRM_STATE_DEAD;
    xfrm_state_put(t);
    t = core::ptr::null_mut();
    goto out;
    }
//
// Must be protected by xfrm_cfg_mutex.  State and tunnel user references are
// always incremented on success.
//
#[no_mangle]
unsafe extern "C" fn ipcomp_tunnel_attach(x: *mut xfrm_state) -> c_int {
    static int ipcomp_tunnel_attach(struct xfrm_state *x)
    {
    struct net *net = xs_net(x);
    let mut err: c_int = 0;
    struct xfrm_state *t;
    let mut mark: u32 = x.mark.v & x.mark.m;
    t = xfrm_state_lookup(net, mark, (xfrm_address_t *)&x.id.daddr.a4,
    x.props.saddr.a4, IPPROTO_IPIP, AF_INET);
    if (!t) {
    t = ipcomp_tunnel_create(x);
    if (!t) {
    err = -EINVAL;
    goto out;
    }
    xfrm_state_insert(t);
    xfrm_state_hold(t);
    }
    x.tunnel = t;
    atomic_inc(&t.tunnel_users);
    out:
    return err;
    }
    static int ipcomp4_init_state(struct xfrm_state *x,
    struct netlink_ext_ack *extack)
    {
    let mut err: c_int = -EINVAL;
    x.props.header_len = 0;
    switch (x.props.mode) {
    case XFRM_MODE_TRANSPORT:
    break;
    case XFRM_MODE_TUNNEL:
    x.props.header_len += sizeof(struct iphdr);
    break;
    default:
    NL_SET_ERR_MSG(extack, "Unsupported XFRM mode for IPcomp");
    goto out;
    }
    err = ipcomp_init_state(x, extack);
    if (err)
    goto out;
    if (x.props.mode == XFRM_MODE_TUNNEL) {
    err = ipcomp_tunnel_attach(x);
    if (err) {
    NL_SET_ERR_MSG(extack, "Kernel error: failed to initialize the associated state");
    goto out;
    }
    }
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ipcomp4_rcv_cb(skb: *mut sk_buff, err: c_int) -> c_int {
    static int ipcomp4_rcv_cb(struct sk_buff *skb, int err)
    {
    return 0;
    }
    static const struct xfrm_type ipcomp_type = {
    .owner		= THIS_MODULE,
    .proto	     	= IPPROTO_COMP,
    .init_state	= ipcomp4_init_state,
    .destructor	= ipcomp_destroy,
    .input		= ipcomp_input,
    .output		= ipcomp_output
    };
    static struct xfrm4_protocol ipcomp4_protocol = {
    .handler	=	xfrm4_rcv,
    .input_handler	=	xfrm_input,
    .cb_handler	=	ipcomp4_rcv_cb,
    .err_handler	=	ipcomp4_err,
    .priority	=	0,
    };
#[no_mangle]
unsafe extern "C" fn ipcomp4_init() -> int __init {
    static int __init ipcomp4_init(void)
    {
    if (xfrm_register_type(&ipcomp_type, AF_INET) < 0) {
    pr_info("%s: can't add xfrm type\n", __func__);
    return -EAGAIN;
    }
    if (xfrm4_protocol_register(&ipcomp4_protocol, IPPROTO_COMP) < 0) {
    pr_info("%s: can't add protocol\n", __func__);
    xfrm_unregister_type(&ipcomp_type, AF_INET);
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipcomp4_fini() -> void __exit {
    static void __exit ipcomp4_fini(void)
    {
    if (xfrm4_protocol_deregister(&ipcomp4_protocol, IPPROTO_COMP) < 0)
    pr_info("%s: can't remove protocol\n", __func__);
    xfrm_unregister_type(&ipcomp_type, AF_INET);
    }
    module_init(ipcomp4_init);
    module_exit(ipcomp4_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IP Payload Compression Protocol (IPComp/IPv4) - RFC3173");
    MODULE_AUTHOR("James Morris <jmorris@intercode.com.au>");
    MODULE_ALIAS_XFRM_TYPE(AF_INET, XFRM_PROTO_COMP);
