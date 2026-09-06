//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_chain_nat.c
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

    static unsigned int nft_nat_do_chain(void *priv, struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nft_pktinfo pkt;
    nft_set_pktinfo(&pkt, skb, state);
    switch (state.pf) {

    case NFPROTO_IPV4:
    nft_set_pktinfo_ipv4(&pkt);
    break;

    case NFPROTO_IPV6:
    nft_set_pktinfo_ipv6(&pkt);
    break;

    default:
    break;
    }
    return nft_do_chain(&pkt, priv);
    }

    static const struct nft_chain_type nft_chain_nat_ipv4 = {
    .name		= "nat",
    .type		= NFT_CHAIN_T_NAT,
    .family		= NFPROTO_IPV4,
    .owner		= THIS_MODULE,
    .hook_mask	= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_LOCAL_IN),
    .hooks		= {
    [NF_INET_PRE_ROUTING]	= nft_nat_do_chain,
    [NF_INET_POST_ROUTING]	= nft_nat_do_chain,
    [NF_INET_LOCAL_OUT]	= nft_nat_do_chain,
    [NF_INET_LOCAL_IN]	= nft_nat_do_chain,
    },
    .ops_register = nf_nat_ipv4_register_fn,
    .ops_unregister = nf_nat_ipv4_unregister_fn,
    };

    static const struct nft_chain_type nft_chain_nat_ipv6 = {
    .name		= "nat",
    .type		= NFT_CHAIN_T_NAT,
    .family		= NFPROTO_IPV6,
    .owner		= THIS_MODULE,
    .hook_mask	= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_LOCAL_IN),
    .hooks		= {
    [NF_INET_PRE_ROUTING]	= nft_nat_do_chain,
    [NF_INET_POST_ROUTING]	= nft_nat_do_chain,
    [NF_INET_LOCAL_OUT]	= nft_nat_do_chain,
    [NF_INET_LOCAL_IN]	= nft_nat_do_chain,
    },
    .ops_register		= nf_nat_ipv6_register_fn,
    .ops_unregister		= nf_nat_ipv6_unregister_fn,
    };

#[no_mangle]
unsafe extern "C" fn nft_nat_inet_reg(net: *mut net, ops: *const nf_hook_ops) -> c_int {
    static int nft_nat_inet_reg(struct net *net, const struct nf_hook_ops *ops)
    {
    return nf_nat_inet_register_fn(net, ops);
    }
#[no_mangle]
unsafe extern "C" fn nft_nat_inet_unreg(net: *mut net, ops: *const nf_hook_ops) {
    static void nft_nat_inet_unreg(struct net *net, const struct nf_hook_ops *ops)
    {
    nf_nat_inet_unregister_fn(net, ops);
    }
    static const struct nft_chain_type nft_chain_nat_inet = {
    .name		= "nat",
    .type		= NFT_CHAIN_T_NAT,
    .family		= NFPROTO_INET,
    .owner		= THIS_MODULE,
    .hook_mask	= (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING),
    .hooks		= {
    [NF_INET_PRE_ROUTING]	= nft_nat_do_chain,
    [NF_INET_LOCAL_IN]	= nft_nat_do_chain,
    [NF_INET_LOCAL_OUT]	= nft_nat_do_chain,
    [NF_INET_POST_ROUTING]	= nft_nat_do_chain,
    },
    .ops_register		= nft_nat_inet_reg,
    .ops_unregister		= nft_nat_inet_unreg,
    };

#[no_mangle]
unsafe extern "C" fn nft_chain_nat_init() -> int __init {
    static int __init nft_chain_nat_init(void)
    {

    nft_register_chain_type(&nft_chain_nat_ipv6);

    nft_register_chain_type(&nft_chain_nat_ipv4);

    nft_register_chain_type(&nft_chain_nat_inet);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nft_chain_nat_exit() -> void __exit {
    static void __exit nft_chain_nat_exit(void)
    {

    nft_unregister_chain_type(&nft_chain_nat_ipv4);

    nft_unregister_chain_type(&nft_chain_nat_ipv6);

    nft_unregister_chain_type(&nft_chain_nat_inet);

    }
    module_init(nft_chain_nat_init);
    module_exit(nft_chain_nat_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("nftables network address translation support");

    MODULE_ALIAS_NFT_CHAIN(AF_INET, "nat");

    MODULE_ALIAS_NFT_CHAIN(AF_INET6, "nat");

    MODULE_ALIAS_NFT_CHAIN(1, "nat");	/* NFPROTO_INET */
