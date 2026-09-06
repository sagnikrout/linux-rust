//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_fib_netdev.c
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
// Copyright (c) 2017 Pablo M. Bermudo Garay <pablombg@gmail.com>
//
// This code is based on net/netfilter/nft_fib_inet.c, written by
// Florian Westphal <fw@strlen.de>.
//

    static void nft_fib_netdev_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_fib *priv = nft_expr_priv(expr);
    switch (ntohs(pkt.skb.protocol)) {
    case ETH_P_IP:
    switch (priv.result) {
    case NFT_FIB_RESULT_OIF:
    case NFT_FIB_RESULT_OIFNAME:
    return nft_fib4_eval(expr, regs, pkt);
    case NFT_FIB_RESULT_ADDRTYPE:
    return nft_fib4_eval_type(expr, regs, pkt);
    }
    break;
    case ETH_P_IPV6:
    if (!ipv6_mod_enabled())
    break;
    switch (priv.result) {
    case NFT_FIB_RESULT_OIF:
    case NFT_FIB_RESULT_OIFNAME:
    return nft_fib6_eval(expr, regs, pkt);
    case NFT_FIB_RESULT_ADDRTYPE:
    return nft_fib6_eval_type(expr, regs, pkt);
    }
    break;
    }
    regs.verdict.code = NFT_BREAK;
    }
    static int nft_fib_netdev_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    const struct nft_fib *priv = nft_expr_priv(expr);
    unsigned int hooks;
    switch (priv.result) {
    case NFT_FIB_RESULT_OIF:
    case NFT_FIB_RESULT_OIFNAME:
    hooks = (1 << NF_NETDEV_INGRESS);
    break;
    case NFT_FIB_RESULT_ADDRTYPE:
    if (priv.flags & NFTA_FIB_F_IIF)
    hooks = (1 << NF_NETDEV_INGRESS);
#[no_mangle]
pub unsafe extern "C" fn if(NFTA_FIB_F_OIF: priv->flags &) -> else {
    else if (priv.flags & NFTA_FIB_F_OIF)
    hooks = (1 << NF_NETDEV_EGRESS);
    else
    hooks = (1 << NF_NETDEV_INGRESS) |
    (1 << NF_NETDEV_EGRESS);
    break;
    default:
    return -EINVAL;
    }
    return nft_chain_validate_hooks(ctx.chain, hooks);
    }
    static struct nft_expr_type nft_fib_netdev_type;
    static const struct nft_expr_ops nft_fib_netdev_ops = {
    .type		= &nft_fib_netdev_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_fib)),
    .eval		= nft_fib_netdev_eval,
    .init		= nft_fib_init,
    .dump		= nft_fib_dump,
    .validate	= nft_fib_netdev_validate,
    };
    static struct nft_expr_type nft_fib_netdev_type __read_mostly = {
    .family		= NFPROTO_NETDEV,
    .name		= "fib",
    .ops		= &nft_fib_netdev_ops,
    .policy		= nft_fib_policy,
    .maxattr	= NFTA_FIB_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_fib_netdev_module_init() -> int __init {
    static int __init nft_fib_netdev_module_init(void)
    {
    return nft_register_expr(&nft_fib_netdev_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_fib_netdev_module_exit() -> void __exit {
    static void __exit nft_fib_netdev_module_exit(void)
    {
    nft_unregister_expr(&nft_fib_netdev_type);
    }
    module_init(nft_fib_netdev_module_init);
    module_exit(nft_fib_netdev_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pablo M. Bermudo Garay <pablombg@gmail.com>");
    MODULE_ALIAS_NFT_AF_EXPR(5, "fib");
    MODULE_DESCRIPTION("nftables netdev fib lookups support");
