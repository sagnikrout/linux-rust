//! Automatically rewritten from C to Rust
//! Source: net/ipv4/xfrm4_tunnel.c
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
// xfrm4_tunnel.c: Generic IP tunnel transformer.
//
// Copyright (C) 2003 David S. Miller (davem@redhat.com)
//

#[no_mangle]
unsafe extern "C" fn ipip_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int ipip_output(struct xfrm_state *x, struct sk_buff *skb)
    {
    skb_push(skb, -skb_network_offset(skb));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipip_xfrm_rcv(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int ipip_xfrm_rcv(struct xfrm_state *x, struct sk_buff *skb)
    {
    return ip_hdr(skb).protocol;
    }
#[no_mangle]
unsafe extern "C" fn ipip_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int {
    static int ipip_init_state(struct xfrm_state *x, struct netlink_ext_ack *extack)
    {
    if (x.props.mode != XFRM_MODE_TUNNEL) {
    NL_SET_ERR_MSG(extack, "IPv4 tunnel can only be used with tunnel mode");
    return -EINVAL;
    }
    if (x.encap) {
    NL_SET_ERR_MSG(extack, "IPv4 tunnel is not compatible with encapsulation");
    return -EINVAL;
    }
    x.props.header_len = sizeof(struct iphdr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipip_destroy(x: *mut xfrm_state) {
    static void ipip_destroy(struct xfrm_state *x)
    {
    }
    static const struct xfrm_type ipip_type = {
    .owner		= THIS_MODULE,
    .proto	     	= IPPROTO_IPIP,
    .init_state	= ipip_init_state,
    .destructor	= ipip_destroy,
    .input		= ipip_xfrm_rcv,
    .output		= ipip_output
    };
#[no_mangle]
unsafe extern "C" fn xfrm_tunnel_rcv(skb: *mut sk_buff) -> c_int {
    static int xfrm_tunnel_rcv(struct sk_buff *skb)
    {
    return xfrm4_rcv_spi(skb, IPPROTO_IPIP, ip_hdr(skb).saddr);
    }
#[no_mangle]
unsafe extern "C" fn xfrm_tunnel_err(skb: *mut sk_buff, info: u32) -> c_int {
    static int xfrm_tunnel_err(struct sk_buff *skb, u32 info)
    {
    return -ENOENT;
    }
    static struct xfrm_tunnel xfrm_tunnel_handler __read_mostly = {
    .handler	=	xfrm_tunnel_rcv,
    .err_handler	=	xfrm_tunnel_err,
    .priority	=	4,
    };

    static struct xfrm_tunnel xfrm64_tunnel_handler __read_mostly = {
    .handler	=	xfrm_tunnel_rcv,
    .err_handler	=	xfrm_tunnel_err,
    .priority	=	3,
    };

#[no_mangle]
unsafe extern "C" fn ipip_init() -> int __init {
    static int __init ipip_init(void)
    {
    if (xfrm_register_type(&ipip_type, AF_INET) < 0) {
    pr_info("%s: can't add xfrm type\n", __func__);
    return -EAGAIN;
    }
    if (xfrm4_tunnel_register(&xfrm_tunnel_handler, AF_INET)) {
    pr_info("%s: can't add xfrm handler for AF_INET\n", __func__);
    xfrm_unregister_type(&ipip_type, AF_INET);
    return -EAGAIN;
    }

    if (xfrm4_tunnel_register(&xfrm64_tunnel_handler, AF_INET6)) {
    pr_info("%s: can't add xfrm handler for AF_INET6\n", __func__);
    xfrm4_tunnel_deregister(&xfrm_tunnel_handler, AF_INET);
    xfrm_unregister_type(&ipip_type, AF_INET);
    return -EAGAIN;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipip_fini() -> void __exit {
    static void __exit ipip_fini(void)
    {

    if (xfrm4_tunnel_deregister(&xfrm64_tunnel_handler, AF_INET6))
    pr_info("%s: can't remove xfrm handler for AF_INET6\n",
    __func__);

    if (xfrm4_tunnel_deregister(&xfrm_tunnel_handler, AF_INET))
    pr_info("%s: can't remove xfrm handler for AF_INET\n",
    __func__);
    xfrm_unregister_type(&ipip_type, AF_INET);
    }
    module_init(ipip_init);
    module_exit(ipip_fini);
    MODULE_DESCRIPTION("IPv4 XFRM tunnel driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_XFRM_TYPE(AF_INET, XFRM_PROTO_IPIP);
