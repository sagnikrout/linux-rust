//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_dup_netdev.c
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
// Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_dup_netdev {
    pub sreg_dev: u8,
}

    static void nft_dup_netdev_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_dup_netdev *priv = nft_expr_priv(expr);
    let mut oif: c_int = regs.data[priv.sreg_dev];
    nf_dup_netdev_egress(pkt, oif);
    }
    static const struct nla_policy nft_dup_netdev_policy[NFTA_DUP_MAX + 1] = {
    [NFTA_DUP_SREG_DEV]	= { .type = NLA_U32 },
    };
    static int nft_dup_netdev_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_dup_netdev *priv = nft_expr_priv(expr);
    if (tb[NFTA_DUP_SREG_DEV] == core::ptr::null_mut())
    return -EINVAL;
    return nft_parse_register_load(ctx, tb[NFTA_DUP_SREG_DEV], &priv.sreg_dev,
    sizeof(int));
    }
    static int nft_dup_netdev_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    struct nft_dup_netdev *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_DUP_SREG_DEV, priv.sreg_dev))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static int nft_dup_netdev_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    const struct nft_expr *expr)
    {
    const struct nft_dup_netdev *priv = nft_expr_priv(expr);
    let mut oif: c_int = ctx.regs[priv.sreg_dev].data.data[0];
    return nft_fwd_dup_netdev_offload(ctx, flow, FLOW_ACTION_MIRRED, oif);
    }
#[no_mangle]
unsafe extern "C" fn nft_dup_netdev_offload_action(expr: *const nft_expr) -> bool {
    static bool nft_dup_netdev_offload_action(const struct nft_expr *expr)
    {
    return true;
    }
    static struct nft_expr_type nft_dup_netdev_type;
    static const struct nft_expr_ops nft_dup_netdev_ops = {
    .type		= &nft_dup_netdev_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_dup_netdev)),
    .eval		= nft_dup_netdev_eval,
    .init		= nft_dup_netdev_init,
    .dump		= nft_dup_netdev_dump,
    .offload	= nft_dup_netdev_offload,
    .offload_action	= nft_dup_netdev_offload_action,
    };
    static struct nft_expr_type nft_dup_netdev_type __read_mostly = {
    .family		= NFPROTO_NETDEV,
    .name		= "dup",
    .ops		= &nft_dup_netdev_ops,
    .policy		= nft_dup_netdev_policy,
    .maxattr	= NFTA_DUP_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_dup_netdev_module_init() -> int __init {
    static int __init nft_dup_netdev_module_init(void)
    {
    return nft_register_expr(&nft_dup_netdev_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_dup_netdev_module_exit() -> void __exit {
    static void __exit nft_dup_netdev_module_exit(void)
    {
    nft_unregister_expr(&nft_dup_netdev_type);
    }
    module_init(nft_dup_netdev_module_init);
    module_exit(nft_dup_netdev_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
    MODULE_ALIAS_NFT_AF_EXPR(5, "dup");
    MODULE_DESCRIPTION("nftables netdev packet duplication support");
