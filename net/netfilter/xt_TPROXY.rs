//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_TPROXY.c
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
// Copyright (c) 2006-2010 BalaBit IT Ltd.
// Author: Balazs Scheidler, Krisztian Kovacs
//

pub const XT_TPROXY_HAVE_IPV6: c_int = 1;

    static unsigned int
    tproxy_tg4(struct net *net, struct sk_buff *skb, __be32 laddr, __be16 lport,
    u_int32_t mark_mask, u_int32_t mark_value)
    {
    const struct iphdr *iph = ip_hdr(skb);
    struct udphdr _hdr, *hp;
    struct sock *sk;
    hp = skb_header_pointer(skb, ip_hdrlen(skb), sizeof(_hdr), &_hdr);
    if (hp == core::ptr::null_mut())
    return NF_DROP;
// check if there's an ongoing connection on the packet
// addresses, this happens if the redirect already happened
// and the current packet belongs to an already established
// connection
    sk = nf_tproxy_get_sock_v4(net, skb, iph.protocol,
    iph.saddr, iph.daddr,
    hp.source, hp.dest,
    skb.dev, NF_TPROXY_LOOKUP_ESTABLISHED);
    laddr = nf_tproxy_laddr4(skb, laddr, iph.daddr);
    if (!lport)
    lport = hp.dest;
// UDP has no TCP_TIME_WAIT state, so we never enter here
    if (sk && sk.sk_state == TCP_TIME_WAIT)
// reopening a TIME_WAIT connection needs special handling
    sk = nf_tproxy_handle_time_wait4(net, skb, laddr, lport, sk);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !sk) -> else {
    else if (!sk)
// no, there's no established connection, check if
// there's a listener on the redirected addr/port
    sk = nf_tproxy_get_sock_v4(net, skb, iph.protocol,
    iph.saddr, laddr,
    hp.source, lport,
    skb.dev, NF_TPROXY_LOOKUP_LISTENER);
// NOTE: assign_sock consumes our sk reference
    if (sk && nf_tproxy_sk_is_transparent(sk)) {
// This should be in a separate target, but we don't do multiple
    targets on the same rule yet */
    skb.mark = (skb.mark & ~mark_mask) ^ mark_value;
    nf_tproxy_assign_sock(skb, sk);
    return NF_ACCEPT;
    }
    return NF_DROP;
    }
    static unsigned int
    tproxy_tg4_v0(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_tproxy_target_info *tgi = par.targinfo;
    if (par.fragoff)
    return NF_DROP;
    return tproxy_tg4(xt_net(par), skb, tgi.laddr, tgi.lport,
    tgi.mark_mask, tgi.mark_value);
    }
    static unsigned int
    tproxy_tg4_v1(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_tproxy_target_info_v1 *tgi = par.targinfo;
    if (par.fragoff)
    return NF_DROP;
    return tproxy_tg4(xt_net(par), skb, tgi.laddr.ip, tgi.lport,
    tgi.mark_mask, tgi.mark_value);
    }

    static unsigned int
    tproxy_tg6_v1(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ipv6hdr *iph = ipv6_hdr(skb);
    const struct xt_tproxy_target_info_v1 *tgi = par.targinfo;
    let mut fragoff: c_ushort = 0;
    struct udphdr _hdr, *hp;
    struct sock *sk;
    const struct in6_addr *laddr;
    __be16 lport;
    let mut thoff: c_int = 0;
    int tproto;
    tproto = ipv6_find_hdr(skb, &thoff, -1, &fragoff, core::ptr::null_mut());
    if (tproto < 0 || fragoff)
    return NF_DROP;
    hp = skb_header_pointer(skb, thoff, sizeof(_hdr), &_hdr);
    if (!hp)
    return NF_DROP;
// check if there's an ongoing connection on the packet
// addresses, this happens if the redirect already happened
// and the current packet belongs to an already established
// connection
    sk = nf_tproxy_get_sock_v6(xt_net(par), skb, thoff, tproto,
    &iph.saddr, &iph.daddr,
    hp.source, hp.dest,
    xt_in(par), NF_TPROXY_LOOKUP_ESTABLISHED);
    laddr = nf_tproxy_laddr6(skb, &tgi.laddr.in6, &iph.daddr);
    lport = tgi.lport ? tgi.lport : hp.dest;
// UDP has no TCP_TIME_WAIT state, so we never enter here
    if (sk && sk.sk_state == TCP_TIME_WAIT) {
    const struct xt_tproxy_target_info_v1 *tgi = par.targinfo;
// reopening a TIME_WAIT connection needs special handling
    sk = nf_tproxy_handle_time_wait6(skb, tproto, thoff,
    xt_net(par),
    &tgi.laddr.in6,
    tgi.lport,
    sk);
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !sk) -> else {
    else if (!sk)
// no there's no established connection, check if
// there's a listener on the redirected addr/port
    sk = nf_tproxy_get_sock_v6(xt_net(par), skb, thoff,
    tproto, &iph.saddr, laddr,
    hp.source, lport,
    xt_in(par), NF_TPROXY_LOOKUP_LISTENER);
// NOTE: assign_sock consumes our sk reference
    if (sk && nf_tproxy_sk_is_transparent(sk)) {
// This should be in a separate target, but we don't do multiple
    targets on the same rule yet */
    skb.mark = (skb.mark & ~tgi.mark_mask) ^ tgi.mark_value;
    nf_tproxy_assign_sock(skb, sk);
    return NF_ACCEPT;
    }
    return NF_DROP;
    }
#[no_mangle]
unsafe extern "C" fn tproxy_tg6_check(par: *const xt_tgchk_param) -> c_int {
    static int tproxy_tg6_check(const struct xt_tgchk_param *par)
    {
    const struct ip6t_ip6 *i = par.entryinfo;
    int err;
    err = nf_defrag_ipv6_enable(par.net);
    if (err)
    return err;
    if ((i.proto == IPPROTO_TCP || i.proto == IPPROTO_UDP) &&
    !(i.invflags & IP6T_INV_PROTO))
    return 0;
    pr_info_ratelimited("Can be used only with -p tcp or -p udp\n");
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tproxy_tg6_destroy(par: *const xt_tgdtor_param) {
    static void tproxy_tg6_destroy(const struct xt_tgdtor_param *par)
    {
    nf_defrag_ipv6_disable(par.net);
    }

#[no_mangle]
unsafe extern "C" fn tproxy_tg4_check(par: *const xt_tgchk_param) -> c_int {
    static int tproxy_tg4_check(const struct xt_tgchk_param *par)
    {
    const struct ipt_ip *i = par.entryinfo;
    int err;
    err = nf_defrag_ipv4_enable(par.net);
    if (err)
    return err;
    if ((i.proto == IPPROTO_TCP || i.proto == IPPROTO_UDP)
    && !(i.invflags & IPT_INV_PROTO))
    return 0;
    pr_info_ratelimited("Can be used only with -p tcp or -p udp\n");
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tproxy_tg4_destroy(par: *const xt_tgdtor_param) {
    static void tproxy_tg4_destroy(const struct xt_tgdtor_param *par)
    {
    nf_defrag_ipv4_disable(par.net);
    }
    static struct xt_target tproxy_tg_reg[] __read_mostly = {
    {
    .name		= "TPROXY",
    .family		= NFPROTO_IPV4,
    .table		= "mangle",
    .target		= tproxy_tg4_v0,
    .revision	= 0,
    .targetsize	= sizeof(struct xt_tproxy_target_info),
    .checkentry	= tproxy_tg4_check,
    .destroy	= tproxy_tg4_destroy,
    .hooks		= 1 << NF_INET_PRE_ROUTING,
    .me		= THIS_MODULE,
    },
    {
    .name		= "TPROXY",
    .family		= NFPROTO_IPV4,
    .table		= "mangle",
    .target		= tproxy_tg4_v1,
    .revision	= 1,
    .targetsize	= sizeof(struct xt_tproxy_target_info_v1),
    .checkentry	= tproxy_tg4_check,
    .destroy	= tproxy_tg4_destroy,
    .hooks		= 1 << NF_INET_PRE_ROUTING,
    .me		= THIS_MODULE,
    },

    {
    .name		= "TPROXY",
    .family		= NFPROTO_IPV6,
    .table		= "mangle",
    .target		= tproxy_tg6_v1,
    .revision	= 1,
    .targetsize	= sizeof(struct xt_tproxy_target_info_v1),
    .checkentry	= tproxy_tg6_check,
    .destroy	= tproxy_tg6_destroy,
    .hooks		= 1 << NF_INET_PRE_ROUTING,
    .me		= THIS_MODULE,
    },

    };
#[no_mangle]
unsafe extern "C" fn tproxy_tg_init() -> int __init {
    static int __init tproxy_tg_init(void)
    {
    return xt_register_targets(tproxy_tg_reg, ARRAY_SIZE(tproxy_tg_reg));
    }
#[no_mangle]
unsafe extern "C" fn tproxy_tg_exit() -> void __exit {
    static void __exit tproxy_tg_exit(void)
    {
    xt_unregister_targets(tproxy_tg_reg, ARRAY_SIZE(tproxy_tg_reg));
    }
    module_init(tproxy_tg_init);
    module_exit(tproxy_tg_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Balazs Scheidler, Krisztian Kovacs");
    MODULE_DESCRIPTION("Netfilter transparent proxy (TPROXY) target module.");
    MODULE_ALIAS("ipt_TPROXY");
    MODULE_ALIAS("ip6t_TPROXY");
