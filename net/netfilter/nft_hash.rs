//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_hash.c
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
// Copyright (c) 2016 Laura Garcia <nevola@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_jhash {
    pub sreg: u8,
    pub dreg: u8,
    pub len: u8,
    pub autogen_seed:1: bool,
    pub modulus: u32,
    pub seed: u32,
    pub offset: u32,
}

    static void nft_jhash_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_jhash *priv = nft_expr_priv(expr);
    const void *data = &regs.data[priv.sreg];
    u32 h;
    h = reciprocal_scale(jhash(data, priv.len, priv.seed),
    priv.modulus);
    regs.data[priv.dreg] = h + priv.offset;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_symhash {
    pub dreg: u8,
    pub modulus: u32,
    pub offset: u32,
}

    static void nft_symhash_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_symhash *priv = nft_expr_priv(expr);
    struct sk_buff *skb = pkt.skb;
    u32 h;
    h = reciprocal_scale(__skb_get_hash_symmetric_net(nft_net(pkt), skb),
    priv.modulus);
    regs.data[priv.dreg] = h + priv.offset;
    }
    static const struct nla_policy nft_hash_policy[NFTA_HASH_MAX + 1] = {
    [NFTA_HASH_SREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_HASH_DREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_HASH_LEN]		= NLA_POLICY_MAX(NLA_BE32, 255),
    [NFTA_HASH_MODULUS]	= { .type = NLA_U32 },
    [NFTA_HASH_SEED]	= { .type = NLA_U32 },
    [NFTA_HASH_OFFSET]	= { .type = NLA_U32 },
    [NFTA_HASH_TYPE]	= { .type = NLA_U32 },
    };
    static int nft_jhash_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_jhash *priv = nft_expr_priv(expr);
    u32 len;
    int err;
    if (!tb[NFTA_HASH_SREG] ||
    !tb[NFTA_HASH_DREG] ||
    !tb[NFTA_HASH_LEN]  ||
    !tb[NFTA_HASH_MODULUS])
    return -EINVAL;
    if (tb[NFTA_HASH_OFFSET])
    priv.offset = ntohl(nla_get_be32(tb[NFTA_HASH_OFFSET]));
    err = nft_parse_u32_check(tb[NFTA_HASH_LEN], U8_MAX, &len);
    if (err < 0)
    return err;
    if (len == 0)
    return -ERANGE;
    priv.len = len;
    err = nft_parse_register_load(ctx, tb[NFTA_HASH_SREG], &priv.sreg, len);
    if (err < 0)
    return err;
    priv.modulus = ntohl(nla_get_be32(tb[NFTA_HASH_MODULUS]));
    if (priv.modulus < 1)
    return -ERANGE;
    if (priv.offset + priv.modulus - 1 < priv.offset)
    return -EOVERFLOW;
    if (tb[NFTA_HASH_SEED]) {
    priv.seed = ntohl(nla_get_be32(tb[NFTA_HASH_SEED]));
    } else {
    priv.autogen_seed = true;
    get_random_bytes(&priv.seed, sizeof(priv.seed));
    }
    return nft_parse_register_store(ctx, tb[NFTA_HASH_DREG], &priv.dreg,
    core::ptr::null_mut(), NFT_DATA_VALUE, sizeof(u32));
    }
    static int nft_symhash_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_symhash *priv = nft_expr_priv(expr);
    if (!tb[NFTA_HASH_DREG]    ||
    !tb[NFTA_HASH_MODULUS])
    return -EINVAL;
    if (tb[NFTA_HASH_OFFSET])
    priv.offset = ntohl(nla_get_be32(tb[NFTA_HASH_OFFSET]));
    priv.modulus = ntohl(nla_get_be32(tb[NFTA_HASH_MODULUS]));
    if (priv.modulus < 1)
    return -ERANGE;
    if (priv.offset + priv.modulus - 1 < priv.offset)
    return -EOVERFLOW;
    return nft_parse_register_store(ctx, tb[NFTA_HASH_DREG],
    &priv.dreg, core::ptr::null_mut(), NFT_DATA_VALUE,
    sizeof(u32));
    }
    static int nft_jhash_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_jhash *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_HASH_SREG, priv.sreg))
    goto nla_put_failure;
    if (nft_dump_register(skb, NFTA_HASH_DREG, priv.dreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_HASH_LEN, htonl(priv.len)))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_HASH_MODULUS, htonl(priv.modulus)))
    goto nla_put_failure;
    if (!priv.autogen_seed &&
    nla_put_be32(skb, NFTA_HASH_SEED, htonl(priv.seed)))
    goto nla_put_failure;
    if (priv.offset != 0)
    if (nla_put_be32(skb, NFTA_HASH_OFFSET, htonl(priv.offset)))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_HASH_TYPE, htonl(NFT_HASH_JENKINS)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static int nft_symhash_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_symhash *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_HASH_DREG, priv.dreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_HASH_MODULUS, htonl(priv.modulus)))
    goto nla_put_failure;
    if (priv.offset != 0)
    if (nla_put_be32(skb, NFTA_HASH_OFFSET, htonl(priv.offset)))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_HASH_TYPE, htonl(NFT_HASH_SYM)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static struct nft_expr_type nft_hash_type;
    static const struct nft_expr_ops nft_jhash_ops = {
    .type		= &nft_hash_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_jhash)),
    .eval		= nft_jhash_eval,
    .init		= nft_jhash_init,
    .dump		= nft_jhash_dump,
    };
    static const struct nft_expr_ops nft_symhash_ops = {
    .type		= &nft_hash_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_symhash)),
    .eval		= nft_symhash_eval,
    .init		= nft_symhash_init,
    .dump		= nft_symhash_dump,
    };
    static const struct nft_expr_ops *
    nft_hash_select_ops(const struct nft_ctx *ctx,
    const struct nlattr * const tb[])
    {
    u32 type;
    if (!tb[NFTA_HASH_TYPE])
    return &nft_jhash_ops;
    type = ntohl(nla_get_be32(tb[NFTA_HASH_TYPE]));
    switch (type) {
    case NFT_HASH_SYM:
    return &nft_symhash_ops;
    case NFT_HASH_JENKINS:
    return &nft_jhash_ops;
    default:
    break;
    }
    return ERR_PTR(-EOPNOTSUPP);
    }
    static struct nft_expr_type nft_hash_type __read_mostly = {
    .name		= "hash",
    .select_ops	= nft_hash_select_ops,
    .policy		= nft_hash_policy,
    .maxattr	= NFTA_HASH_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_hash_module_init() -> int __init {
    static int __init nft_hash_module_init(void)
    {
    return nft_register_expr(&nft_hash_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_hash_module_exit() -> void __exit {
    static void __exit nft_hash_module_exit(void)
    {
    nft_unregister_expr(&nft_hash_type);
    }
    module_init(nft_hash_module_init);
    module_exit(nft_hash_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Laura Garcia <nevola@gmail.com>");
    MODULE_ALIAS_NFT_EXPR("hash");
    MODULE_DESCRIPTION("Netfilter nftables hash module");
