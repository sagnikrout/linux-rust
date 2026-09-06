//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_socket.c
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
// Transparent proxy support for Linux/iptables
//
// Copyright (C) 2007-2008 BalaBit IT Ltd.
// Author: Krisztian Kovacs
//

// "socket" match based redirection (no specific rule)
// ===================================================
//
// There are connections with dynamic endpoints (e.g. FTP data
// connection) that the user is unable to add explicit rules
// for. These are taken care of by a generic "socket" rule. It is
// assumed that the proxy application is trusted to open such
// connections without explicit iptables rule (except of course the
// generic 'socket' rule). In this case the following sockets are
// matched in preference order:
//
// - match: if there's a fully established connection matching the
// _packet_ tuple
//
// - match: if there's a non-zero bound listener (possibly with a
// non-local address) We don't accept zero-bound listeners, since
// then local services could intercept traffic going through the
// box.
//
    static bool
    socket_match(const struct sk_buff *skb, struct xt_action_param *par,
    const struct xt_socket_mtinfo1 *info)
    {
    struct sk_buff *pskb = (struct sk_buff *)skb;
    struct sock *sk = skb.sk;
    if (sk && !net_eq(xt_net(par), sock_net(sk)))
    sk = core::ptr::null_mut();
    if (!sk)
    sk = nf_sk_lookup_slow_v4(xt_net(par), skb, xt_in(par));
    if (sk) {
    bool wildcard;
    let mut transparent: bool = true;
// Ignore sockets listening on INADDR_ANY,
// unless XT_SOCKET_NOWILDCARD is set
//
    wildcard = (!(info.flags & XT_SOCKET_NOWILDCARD) &&
    sk_fullsock(sk) &&
    inet_sk(sk).inet_rcv_saddr == 0);
// Ignore non-transparent sockets,
// if XT_SOCKET_TRANSPARENT is used
//
    if (info.flags & XT_SOCKET_TRANSPARENT)
    transparent = inet_sk_transparent(sk);
    if (info.flags & XT_SOCKET_RESTORESKMARK && !wildcard &&
    transparent && sk_fullsock(sk))
    pskb.mark = READ_ONCE(sk.sk_mark);
    if (sk != skb.sk)
    sock_gen_put(sk);
    if (wildcard || !transparent)
    sk = core::ptr::null_mut();
    }
    return sk != core::ptr::null_mut();
    }
    static bool
    socket_mt4_v0(const struct sk_buff *skb, struct xt_action_param *par)
    {
    static struct xt_socket_mtinfo1 xt_info_v0 = {
    .flags = 0,
    };
    return socket_match(skb, par, &xt_info_v0);
    }
    static bool
    socket_mt4_v1_v2_v3(const struct sk_buff *skb, struct xt_action_param *par)
    {
    return socket_match(skb, par, par.matchinfo);
    }

    static bool
    socket_mt6_v1_v2_v3(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_socket_mtinfo1 *info = (struct xt_socket_mtinfo1 *) par.matchinfo;
    struct sk_buff *pskb = (struct sk_buff *)skb;
    struct sock *sk = skb.sk;
    if (sk && !net_eq(xt_net(par), sock_net(sk)))
    sk = core::ptr::null_mut();
    if (!sk)
    sk = nf_sk_lookup_slow_v6(xt_net(par), skb, xt_in(par));
    if (sk) {
    bool wildcard;
    let mut transparent: bool = true;
// Ignore sockets listening on INADDR_ANY
// unless XT_SOCKET_NOWILDCARD is set
//
    wildcard = (!(info.flags & XT_SOCKET_NOWILDCARD) &&
    sk_fullsock(sk) &&
    ipv6_addr_any(&sk.sk_v6_rcv_saddr));
// Ignore non-transparent sockets,
// if XT_SOCKET_TRANSPARENT is used
//
    if (info.flags & XT_SOCKET_TRANSPARENT)
    transparent = inet_sk_transparent(sk);
    if (info.flags & XT_SOCKET_RESTORESKMARK && !wildcard &&
    transparent && sk_fullsock(sk))
    pskb.mark = READ_ONCE(sk.sk_mark);
    if (sk != skb.sk)
    sock_gen_put(sk);
    if (wildcard || !transparent)
    sk = core::ptr::null_mut();
    }
    return sk != core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn socket_mt_enable_defrag(net: *mut net, family: c_int) -> c_int {
    static int socket_mt_enable_defrag(struct net *net, int family)
    {
    switch (family) {
    case NFPROTO_IPV4:
    return nf_defrag_ipv4_enable(net);

    case NFPROTO_IPV6:
    return nf_defrag_ipv6_enable(net);

    }
    WARN_ONCE(1, "Unknown family %d\n", family);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socket_mt_v1_check(par: *const xt_mtchk_param) -> c_int {
    static int socket_mt_v1_check(const struct xt_mtchk_param *par)
    {
    const struct xt_socket_mtinfo1 *info = (struct xt_socket_mtinfo1 *) par.matchinfo;
    if (info.flags & ~XT_SOCKET_FLAGS_V1) {
    pr_info_ratelimited("unknown flags 0x%x\n",
    info.flags & ~XT_SOCKET_FLAGS_V1);
    return -EINVAL;
    }
    return socket_mt_enable_defrag(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn socket_mt_v2_check(par: *const xt_mtchk_param) -> c_int {
    static int socket_mt_v2_check(const struct xt_mtchk_param *par)
    {
    const struct xt_socket_mtinfo2 *info = (struct xt_socket_mtinfo2 *) par.matchinfo;
    if (info.flags & ~XT_SOCKET_FLAGS_V2) {
    pr_info_ratelimited("unknown flags 0x%x\n",
    info.flags & ~XT_SOCKET_FLAGS_V2);
    return -EINVAL;
    }
    return socket_mt_enable_defrag(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn socket_mt_v3_check(par: *const xt_mtchk_param) -> c_int {
    static int socket_mt_v3_check(const struct xt_mtchk_param *par)
    {
    const struct xt_socket_mtinfo3 *info =
    (struct xt_socket_mtinfo3 *)par.matchinfo;
    if (info.flags & ~XT_SOCKET_FLAGS_V3) {
    pr_info_ratelimited("unknown flags 0x%x\n",
    info.flags & ~XT_SOCKET_FLAGS_V3);
    return -EINVAL;
    }
    return socket_mt_enable_defrag(par.net, par.family);
    }
#[no_mangle]
unsafe extern "C" fn socket_mt_destroy(par: *const xt_mtdtor_param) {
    static void socket_mt_destroy(const struct xt_mtdtor_param *par)
    {
    if (par.family == NFPROTO_IPV4)
    nf_defrag_ipv4_disable(par.net);

#[no_mangle]
pub unsafe extern "C" fn if(NFPROTO_IPV6: par->family ==) -> else {
    else if (par.family == NFPROTO_IPV6)
    nf_defrag_ipv6_disable(par.net);

    }
    static struct xt_match socket_mt_reg[] __read_mostly = {
    {
    .name		= "socket",
    .revision	= 0,
    .family		= NFPROTO_IPV4,
    .match		= socket_mt4_v0,
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },
    {
    .name		= "socket",
    .revision	= 1,
    .family		= NFPROTO_IPV4,
    .match		= socket_mt4_v1_v2_v3,
    .destroy	= socket_mt_destroy,
    .checkentry	= socket_mt_v1_check,
    .matchsize	= sizeof(struct xt_socket_mtinfo1),
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },

    {
    .name		= "socket",
    .revision	= 1,
    .family		= NFPROTO_IPV6,
    .match		= socket_mt6_v1_v2_v3,
    .checkentry	= socket_mt_v1_check,
    .matchsize	= sizeof(struct xt_socket_mtinfo1),
    .destroy	= socket_mt_destroy,
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },

    {
    .name		= "socket",
    .revision	= 2,
    .family		= NFPROTO_IPV4,
    .match		= socket_mt4_v1_v2_v3,
    .checkentry	= socket_mt_v2_check,
    .destroy	= socket_mt_destroy,
    .matchsize	= sizeof(struct xt_socket_mtinfo1),
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },

    {
    .name		= "socket",
    .revision	= 2,
    .family		= NFPROTO_IPV6,
    .match		= socket_mt6_v1_v2_v3,
    .checkentry	= socket_mt_v2_check,
    .destroy	= socket_mt_destroy,
    .matchsize	= sizeof(struct xt_socket_mtinfo1),
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },

    {
    .name		= "socket",
    .revision	= 3,
    .family		= NFPROTO_IPV4,
    .match		= socket_mt4_v1_v2_v3,
    .checkentry	= socket_mt_v3_check,
    .destroy	= socket_mt_destroy,
    .matchsize	= sizeof(struct xt_socket_mtinfo1),
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },

    {
    .name		= "socket",
    .revision	= 3,
    .family		= NFPROTO_IPV6,
    .match		= socket_mt6_v1_v2_v3,
    .checkentry	= socket_mt_v3_check,
    .destroy	= socket_mt_destroy,
    .matchsize	= sizeof(struct xt_socket_mtinfo1),
    .hooks		= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN),
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn socket_mt_init() -> int __init {
    static int __init socket_mt_init(void)
    {
    return xt_register_matches(socket_mt_reg, ARRAY_SIZE(socket_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn socket_mt_exit() -> void __exit {
    static void __exit socket_mt_exit(void)
    {
    xt_unregister_matches(socket_mt_reg, ARRAY_SIZE(socket_mt_reg));
    }
    module_init(socket_mt_init);
    module_exit(socket_mt_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Krisztian Kovacs, Balazs Scheidler");
    MODULE_DESCRIPTION("x_tables socket match module");
    MODULE_ALIAS("ipt_socket");
    MODULE_ALIAS("ip6t_socket");
