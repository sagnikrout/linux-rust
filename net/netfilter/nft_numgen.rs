//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_numgen.c
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
pub struct nft_ng_inc {
    pub dreg: u8,
    pub modulus: u32,
    pub counter: *mut core::sync::atomic::AtomicI32,
    pub offset: u32,
}

#[no_mangle]
unsafe extern "C" fn nft_ng_inc_gen(priv: *mut nft_ng_inc) -> u32 {
    static u32 nft_ng_inc_gen(struct nft_ng_inc *priv)
    {
    u32 nval, oval;
    do {
    oval = atomic_read(priv.counter);
    nval = (oval + 1 < priv.modulus) ? oval + 1 : 0;
    } while (atomic_cmpxchg(priv.counter, oval, nval) != oval);
    return nval + priv.offset;
    }
    static void nft_ng_inc_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_ng_inc *priv = nft_expr_priv(expr);
    regs.data[priv.dreg] = nft_ng_inc_gen(priv);
    }
    static const struct nla_policy nft_ng_policy[NFTA_NG_MAX + 1] = {
    [NFTA_NG_DREG]		= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_NG_MODULUS]	= { .type = NLA_U32 },
    [NFTA_NG_TYPE]		= { .type = NLA_U32 },
    [NFTA_NG_OFFSET]	= { .type = NLA_U32 },
    };
    static int nft_ng_inc_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_ng_inc *priv = nft_expr_priv(expr);
    int err;
    if (tb[NFTA_NG_OFFSET])
    priv.offset = ntohl(nla_get_be32(tb[NFTA_NG_OFFSET]));
    priv.modulus = ntohl(nla_get_be32(tb[NFTA_NG_MODULUS]));
    if (priv.modulus == 0)
    return -ERANGE;
    if (priv.offset + priv.modulus - 1 < priv.offset)
    return -EOVERFLOW;
    priv.counter = kmalloc_obj(*priv.counter, GFP_KERNEL_ACCOUNT);
    if (!priv.counter)
    return -ENOMEM;
    atomic_set(priv.counter, priv.modulus - 1);
    err = nft_parse_register_store(ctx, tb[NFTA_NG_DREG], &priv.dreg,
    core::ptr::null_mut(), NFT_DATA_VALUE, sizeof(u32));
    if (err < 0)
    goto err;
    return 0;
    err:
    kfree(priv.counter);
    return err;
    }
    static int nft_ng_dump(struct sk_buff *skb, enum nft_registers dreg,
    u32 modulus, enum nft_ng_types type, u32 offset)
    {
    if (nft_dump_register(skb, NFTA_NG_DREG, dreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_NG_MODULUS, htonl(modulus)))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_NG_TYPE, htonl(type)))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_NG_OFFSET, htonl(offset)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static int nft_ng_inc_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_ng_inc *priv = nft_expr_priv(expr);
    return nft_ng_dump(skb, priv.dreg, priv.modulus, NFT_NG_INCREMENTAL,
    priv.offset);
    }
    static void nft_ng_inc_destroy(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    const struct nft_ng_inc *priv = nft_expr_priv(expr);
    kfree(priv.counter);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_ng_random {
    pub dreg: u8,
    pub modulus: u32,
    pub offset: u32,
}

#[no_mangle]
unsafe extern "C" fn nft_ng_random_gen(priv: *const nft_ng_random) -> u32 {
    static u32 nft_ng_random_gen(const struct nft_ng_random *priv)
    {
    return reciprocal_scale(get_random_u32(), priv.modulus) + priv.offset;
    }
    static void nft_ng_random_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_ng_random *priv = nft_expr_priv(expr);
    regs.data[priv.dreg] = nft_ng_random_gen(priv);
    }
    static int nft_ng_random_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_ng_random *priv = nft_expr_priv(expr);
    if (tb[NFTA_NG_OFFSET])
    priv.offset = ntohl(nla_get_be32(tb[NFTA_NG_OFFSET]));
    priv.modulus = ntohl(nla_get_be32(tb[NFTA_NG_MODULUS]));
    if (priv.modulus == 0)
    return -ERANGE;
    if (priv.offset + priv.modulus - 1 < priv.offset)
    return -EOVERFLOW;
    return nft_parse_register_store(ctx, tb[NFTA_NG_DREG], &priv.dreg,
    core::ptr::null_mut(), NFT_DATA_VALUE, sizeof(u32));
    }
    static int nft_ng_random_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_ng_random *priv = nft_expr_priv(expr);
    return nft_ng_dump(skb, priv.dreg, priv.modulus, NFT_NG_RANDOM,
    priv.offset);
    }
    static struct nft_expr_type nft_ng_type;
    static const struct nft_expr_ops nft_ng_inc_ops = {
    .type		= &nft_ng_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_ng_inc)),
    .eval		= nft_ng_inc_eval,
    .init		= nft_ng_inc_init,
    .destroy	= nft_ng_inc_destroy,
    .dump		= nft_ng_inc_dump,
    };
    static const struct nft_expr_ops nft_ng_random_ops = {
    .type		= &nft_ng_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_ng_random)),
    .eval		= nft_ng_random_eval,
    .init		= nft_ng_random_init,
    .dump		= nft_ng_random_dump,
    };
    static const struct nft_expr_ops *
    nft_ng_select_ops(const struct nft_ctx *ctx, const struct nlattr * const tb[])
    {
    u32 type;
    if (!tb[NFTA_NG_DREG]	 ||
    !tb[NFTA_NG_MODULUS] ||
    !tb[NFTA_NG_TYPE])
    return ERR_PTR(-EINVAL);
    type = ntohl(nla_get_be32(tb[NFTA_NG_TYPE]));
    switch (type) {
    case NFT_NG_INCREMENTAL:
    return &nft_ng_inc_ops;
    case NFT_NG_RANDOM:
    return &nft_ng_random_ops;
    }
    return ERR_PTR(-EINVAL);
    }
    static struct nft_expr_type nft_ng_type __read_mostly = {
    .name		= "numgen",
    .select_ops	= nft_ng_select_ops,
    .policy		= nft_ng_policy,
    .maxattr	= NFTA_NG_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_ng_module_init() -> int __init {
    static int __init nft_ng_module_init(void)
    {
    return nft_register_expr(&nft_ng_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_ng_module_exit() -> void __exit {
    static void __exit nft_ng_module_exit(void)
    {
    nft_unregister_expr(&nft_ng_type);
    }
    module_init(nft_ng_module_init);
    module_exit(nft_ng_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Laura Garcia <nevola@gmail.com>");
    MODULE_ALIAS_NFT_EXPR("numgen");
    MODULE_DESCRIPTION("nftables number generator module");
