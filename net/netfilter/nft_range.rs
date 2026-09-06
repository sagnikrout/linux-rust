//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_range.c
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
// Copyright (c) 2016 Pablo Neira Ayuso <pablo@netfilter.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_range_expr {
    pub data_from: nft_data,
    pub data_to: nft_data,
    pub sreg: u8,
    pub len: u8,
    pub op:8: enum nft_range_ops,
}

    void nft_range_eval(const struct nft_expr *expr,
    struct nft_regs *regs, const struct nft_pktinfo *pkt)
    {
    const struct nft_range_expr *priv = nft_expr_priv(expr);
    int d1, d2;
    d1 = memcmp(&regs.data[priv.sreg], &priv.data_from, priv.len);
    d2 = memcmp(&regs.data[priv.sreg], &priv.data_to, priv.len);
    switch (priv.op) {
    case NFT_RANGE_EQ:
    if (d1 < 0 || d2 > 0)
    regs.verdict.code = NFT_BREAK;
    break;
    case NFT_RANGE_NEQ:
    if (d1 >= 0 && d2 <= 0)
    regs.verdict.code = NFT_BREAK;
    break;
    }
    }
    static const struct nla_policy nft_range_policy[NFTA_RANGE_MAX + 1] = {
    [NFTA_RANGE_SREG]		= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_RANGE_OP]			= NLA_POLICY_MAX(NLA_BE32, 255),
    [NFTA_RANGE_FROM_DATA]		= { .type = NLA_NESTED },
    [NFTA_RANGE_TO_DATA]		= { .type = NLA_NESTED },
    };
    static int nft_range_init(const struct nft_ctx *ctx, const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_range_expr *priv = nft_expr_priv(expr);
    struct nft_data_desc desc_from = {
    .type	= NFT_DATA_VALUE,
    .size	= sizeof(priv.data_from),
    };
    struct nft_data_desc desc_to = {
    .type	= NFT_DATA_VALUE,
    .size	= sizeof(priv.data_to),
    };
    int err;
    u32 op;
    if (!tb[NFTA_RANGE_SREG]      ||
    !tb[NFTA_RANGE_OP]	      ||
    !tb[NFTA_RANGE_FROM_DATA] ||
    !tb[NFTA_RANGE_TO_DATA])
    return -EINVAL;
    err = nft_data_init(core::ptr::null_mut(), &priv.data_from, &desc_from,
    tb[NFTA_RANGE_FROM_DATA]);
    if (err < 0)
    return err;
    err = nft_data_init(core::ptr::null_mut(), &priv.data_to, &desc_to,
    tb[NFTA_RANGE_TO_DATA]);
    if (err < 0)
    goto err1;
    if (desc_from.len != desc_to.len) {
    err = -EINVAL;
    goto err2;
    }
    err = nft_parse_register_load(ctx, tb[NFTA_RANGE_SREG], &priv.sreg,
    desc_from.len);
    if (err < 0)
    goto err2;
    err = nft_parse_u32_check(tb[NFTA_RANGE_OP], U8_MAX, &op);
    if (err < 0)
    goto err2;
    switch (op) {
    case NFT_RANGE_EQ:
    case NFT_RANGE_NEQ:
    break;
    default:
    err = -EINVAL;
    goto err2;
    }
    priv.op  = op;
    priv.len = desc_from.len;
    return 0;
    err2:
    nft_data_release(&priv.data_to, desc_to.type);
    err1:
    nft_data_release(&priv.data_from, desc_from.type);
    return err;
    }
    static int nft_range_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_range_expr *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_RANGE_SREG, priv.sreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_RANGE_OP, htonl(priv.op)))
    goto nla_put_failure;
    if (nft_data_dump(skb, NFTA_RANGE_FROM_DATA, &priv.data_from,
    NFT_DATA_VALUE, priv.len) < 0 ||
    nft_data_dump(skb, NFTA_RANGE_TO_DATA, &priv.data_to,
    NFT_DATA_VALUE, priv.len) < 0)
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static const struct nft_expr_ops nft_range_ops = {
    .type		= &nft_range_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_range_expr)),
    .eval		= nft_range_eval,
    .init		= nft_range_init,
    .dump		= nft_range_dump,
    };
    struct nft_expr_type nft_range_type __read_mostly = {
    .name		= "range",
    .ops		= &nft_range_ops,
    .policy		= nft_range_policy,
    .maxattr	= NFTA_RANGE_MAX,
    .owner		= THIS_MODULE,
    };
