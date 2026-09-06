//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_chain_route.c
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

    static unsigned int nf_route_table_hook4(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    const struct iphdr *iph;
    struct nft_pktinfo pkt;
    __be32 saddr, daddr;
    unsigned int ret;
    u32 mark;
    int err;
    u8 tos;
    nft_set_pktinfo(&pkt, skb, state);
    nft_set_pktinfo_ipv4(&pkt);
    mark = skb.mark;
    iph = ip_hdr(skb);
    saddr = iph.saddr;
    daddr = iph.daddr;
    tos = iph.tos;
    ret = nft_do_chain(&pkt, priv);
    if (ret == NF_ACCEPT) {
    iph = ip_hdr(skb);
    if (iph.saddr != saddr ||
    iph.daddr != daddr ||
    skb.mark != mark ||
    iph.tos != tos) {
    err = ip_route_me_harder(state.net, state.sk, skb, RTN_UNSPEC);
    if (err < 0)
    ret = NF_DROP_ERR(err);
    }
    }
    return ret;
    }
    static const struct nft_chain_type nft_chain_route_ipv4 = {
    .name		= "route",
    .type		= NFT_CHAIN_T_ROUTE,
    .family		= NFPROTO_IPV4,
    .hook_mask	= (1 << NF_INET_LOCAL_OUT),
    .hooks		= {
    [NF_INET_LOCAL_OUT]	= nf_route_table_hook4,
    },
    };

    static unsigned int nf_route_table_hook6(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct in6_addr saddr, daddr;
    struct nft_pktinfo pkt;
    u32 mark, flowlabel;
    unsigned int ret;
    u8 hop_limit;
    int err;
    nft_set_pktinfo(&pkt, skb, state);
    nft_set_pktinfo_ipv6(&pkt);
// save source/dest address, mark, hoplimit, flowlabel, priority
    memcpy(&saddr, &ipv6_hdr(skb).saddr, sizeof(saddr));
    memcpy(&daddr, &ipv6_hdr(skb).daddr, sizeof(daddr));
    mark = skb.mark;
    hop_limit = ipv6_hdr(skb).hop_limit;
// flowlabel and prio (includes version, which shouldn't change either)
    flowlabel = *((u32 *)ipv6_hdr(skb));
    ret = nft_do_chain(&pkt, priv);
    if (ret == NF_ACCEPT &&
    (memcmp(&ipv6_hdr(skb).saddr, &saddr, sizeof(saddr)) ||
    memcmp(&ipv6_hdr(skb).daddr, &daddr, sizeof(daddr)) ||
    skb.mark != mark ||
    ipv6_hdr(skb).hop_limit != hop_limit ||
    flowlabel != *((u32 *)ipv6_hdr(skb)))) {
    err = nf_ip6_route_me_harder(state.net, state.sk, skb);
    if (err < 0)
    ret = NF_DROP_ERR(err);
    }
    return ret;
    }
    static const struct nft_chain_type nft_chain_route_ipv6 = {
    .name		= "route",
    .type		= NFT_CHAIN_T_ROUTE,
    .family		= NFPROTO_IPV6,
    .hook_mask	= (1 << NF_INET_LOCAL_OUT),
    .hooks		= {
    [NF_INET_LOCAL_OUT]	= nf_route_table_hook6,
    },
    };

    static unsigned int nf_route_table_inet(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    switch (state.pf) {
    case NFPROTO_IPV4:
    return nf_route_table_hook4(priv, skb, state);
    case NFPROTO_IPV6:
    return nf_route_table_hook6(priv, skb, state);
    default:
    nft_set_pktinfo(&pkt, skb, state);
    break;
    }
    return nft_do_chain(&pkt, priv);
    }
    static const struct nft_chain_type nft_chain_route_inet = {
    .name		= "route",
    .type		= NFT_CHAIN_T_ROUTE,
    .family		= NFPROTO_INET,
    .hook_mask	= (1 << NF_INET_LOCAL_OUT),
    .hooks		= {
    [NF_INET_LOCAL_OUT]	= nf_route_table_inet,
    },
    };

#[no_mangle]
pub unsafe extern "C" fn nft_chain_route_init() -> void __init {
    void __init nft_chain_route_init(void)
    {

    nft_register_chain_type(&nft_chain_route_ipv6);

    nft_register_chain_type(&nft_chain_route_ipv4);

    nft_register_chain_type(&nft_chain_route_inet);

    }
#[no_mangle]
pub unsafe extern "C" fn nft_chain_route_fini() -> void __exit {
    void __exit nft_chain_route_fini(void)
    {

    nft_unregister_chain_type(&nft_chain_route_ipv6);

    nft_unregister_chain_type(&nft_chain_route_ipv4);

    nft_unregister_chain_type(&nft_chain_route_inet);

    }
