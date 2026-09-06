//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_reject_inet.c
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
// Copyright (c) 2014 Patrick McHardy <kaber@trash.net>
//

    static void nft_reject_inet_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_reject *priv = nft_expr_priv(expr);
    switch (nft_pf(pkt)) {
    case NFPROTO_IPV4:
    switch (priv.type) {
    case NFT_REJECT_ICMP_UNREACH:
    nf_send_unreach(pkt.skb, priv.icmp_code,
    nft_hook(pkt));
    break;
    case NFT_REJECT_TCP_RST:
    nf_send_reset(nft_net(pkt), nft_sk(pkt),
    pkt.skb, nft_hook(pkt));
    break;
    case NFT_REJECT_ICMPX_UNREACH:
    nf_send_unreach(pkt.skb,
    nft_reject_icmp_code(priv.icmp_code),
    nft_hook(pkt));
    break;
    }
    break;
    case NFPROTO_IPV6:
    switch (priv.type) {
    case NFT_REJECT_ICMP_UNREACH:
    nf_send_unreach6(nft_net(pkt), pkt.skb,
    priv.icmp_code, nft_hook(pkt));
    break;
    case NFT_REJECT_TCP_RST:
    nf_send_reset6(nft_net(pkt), nft_sk(pkt),
    pkt.skb, nft_hook(pkt));
    break;
    case NFT_REJECT_ICMPX_UNREACH:
    nf_send_unreach6(nft_net(pkt), pkt.skb,
    nft_reject_icmpv6_code(priv.icmp_code),
    nft_hook(pkt));
    break;
    }
    break;
    }
    regs.verdict.code = NF_DROP;
    }
    static int nft_reject_inet_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    return nft_chain_validate_hooks(ctx.chain,
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_FORWARD) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_INGRESS));
    }
    static struct nft_expr_type nft_reject_inet_type;
    static const struct nft_expr_ops nft_reject_inet_ops = {
    .type		= &nft_reject_inet_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_reject)),
    .eval		= nft_reject_inet_eval,
    .init		= nft_reject_init,
    .dump		= nft_reject_dump,
    .validate	= nft_reject_inet_validate,
    };
    static struct nft_expr_type nft_reject_inet_type __read_mostly = {
    .family		= NFPROTO_INET,
    .name		= "reject",
    .ops		= &nft_reject_inet_ops,
    .policy		= nft_reject_policy,
    .maxattr	= NFTA_REJECT_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_reject_inet_module_init() -> int __init {
    static int __init nft_reject_inet_module_init(void)
    {
    return nft_register_expr(&nft_reject_inet_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_reject_inet_module_exit() -> void __exit {
    static void __exit nft_reject_inet_module_exit(void)
    {
    nft_unregister_expr(&nft_reject_inet_type);
    }
    module_init(nft_reject_inet_module_init);
    module_exit(nft_reject_inet_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_ALIAS_NFT_AF_EXPR(1, "reject");
    MODULE_DESCRIPTION("Netfilter nftables reject inet support");
