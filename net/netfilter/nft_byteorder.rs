//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_byteorder.c
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
// Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
//
// Development of this code funded by Astaro AG (http://www.astaro.com/)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_byteorder {
    pub sreg: u8,
    pub dreg: u8,
    pub op:8: enum nft_byteorder_ops,
    pub size: u8,
}

    void nft_byteorder_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_byteorder *priv = nft_expr_priv(expr);
    const u32 *src = &regs.data[priv.sreg];
    u32 *dst = &regs.data[priv.dreg];
    switch (priv.size) {
    case 8: {
    u64 *dst64 = (void *)dst;
    u64 src64;
    switch (priv.op) {
    case NFT_BYTEORDER_NTOH:
    src64 = nft_reg_load64(src);
    nft_reg_store64(dst64, be64_to_cpu(( __be64)src64));
    break;
    case NFT_BYTEORDER_HTON:
    src64 = ( __u64)cpu_to_be64(nft_reg_load64(src));
    nft_reg_store64(dst64, src64);
    break;
    }
    break;
    }
    case 4:
    switch (priv.op) {
    case NFT_BYTEORDER_NTOH:
// dst = ntohl(( __be32)*src);
    break;
    case NFT_BYTEORDER_HTON:
// dst = ( __u32)htonl(*src);
    break;
    }
    break;
    case 2:
    switch (priv.op) {
    case NFT_BYTEORDER_NTOH:
    nft_reg_store16(dst, ntohs(nft_reg_load_be16(src)));
    break;
    case NFT_BYTEORDER_HTON:
    nft_reg_store_be16(dst, htons(nft_reg_load16(src)));
    break;
    }
    break;
    }
    }
    static const struct nla_policy nft_byteorder_policy[NFTA_BYTEORDER_MAX + 1] = {
    [NFTA_BYTEORDER_SREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_BYTEORDER_DREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_BYTEORDER_OP]	= NLA_POLICY_MAX(NLA_BE32, 255),
    [NFTA_BYTEORDER_LEN]	= NLA_POLICY_MAX(NLA_BE32, 255),
    [NFTA_BYTEORDER_SIZE]	= NLA_POLICY_MAX(NLA_BE32, 255),
    };
    static int nft_byteorder_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_byteorder *priv = nft_expr_priv(expr);
    u32 size, len;
    int err;
    if (tb[NFTA_BYTEORDER_SREG] == core::ptr::null_mut() ||
    tb[NFTA_BYTEORDER_DREG] == core::ptr::null_mut() ||
    tb[NFTA_BYTEORDER_LEN] == core::ptr::null_mut() ||
    tb[NFTA_BYTEORDER_SIZE] == core::ptr::null_mut() ||
    tb[NFTA_BYTEORDER_OP] == core::ptr::null_mut())
    return -EINVAL;
    priv.op = ntohl(nla_get_be32(tb[NFTA_BYTEORDER_OP]));
    switch (priv.op) {
    case NFT_BYTEORDER_NTOH:
    case NFT_BYTEORDER_HTON:
    break;
    default:
    return -EINVAL;
    }
    err = nft_parse_u32_check(tb[NFTA_BYTEORDER_SIZE], U8_MAX, &size);
    if (err < 0)
    return err;
    priv.size = size;
    switch (priv.size) {
    case 2:
    case 4:
    case 8:
    break;
    default:
    return -EINVAL;
    }
    err = nft_parse_u32_check(tb[NFTA_BYTEORDER_LEN], U8_MAX, &len);
    if (err < 0)
    return err;
// no longer support multi-reg conversions
    if (len != size)
    return -EOPNOTSUPP;
    err = nft_parse_register_load(ctx, tb[NFTA_BYTEORDER_SREG], &priv.sreg,
    len);
    if (err < 0)
    return err;
    err = nft_parse_register_store(ctx, tb[NFTA_BYTEORDER_DREG],
    &priv.dreg, core::ptr::null_mut(), NFT_DATA_VALUE,
    len);
    if (err < 0)
    return err;
    if (nft_reg_overlap(priv.sreg, priv.dreg, len))
    return -EINVAL;
    return 0;
    }
    static int nft_byteorder_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_byteorder *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_BYTEORDER_SREG, priv.sreg))
    goto nla_put_failure;
    if (nft_dump_register(skb, NFTA_BYTEORDER_DREG, priv.dreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_BYTEORDER_OP, htonl(priv.op)))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_BYTEORDER_SIZE, htonl(priv.size)))
    goto nla_put_failure;
// compatibility for old userspace which permitted size != len
    if (nla_put_be32(skb, NFTA_BYTEORDER_LEN, htonl(priv.size)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static const struct nft_expr_ops nft_byteorder_ops = {
    .type		= &nft_byteorder_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_byteorder)),
    .eval		= nft_byteorder_eval,
    .init		= nft_byteorder_init,
    .dump		= nft_byteorder_dump,
    };
    struct nft_expr_type nft_byteorder_type __read_mostly = {
    .name		= "byteorder",
    .ops		= &nft_byteorder_ops,
    .policy		= nft_byteorder_policy,
    .maxattr	= NFTA_BYTEORDER_MAX,
    .owner		= THIS_MODULE,
    };
