//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_last.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_last {
    pub jiffies: c_ulong,
    pub set: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_last_priv {
    pub last: *mut nft_last,
}

    static const struct nla_policy nft_last_policy[NFTA_LAST_MAX + 1] = {
    [NFTA_LAST_SET] = { .type = NLA_U32 },
    [NFTA_LAST_MSECS] = { .type = NLA_U64 },
    };
    static int nft_last_init(const struct nft_ctx *ctx, const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_last_priv *priv = nft_expr_priv(expr);
    struct nft_last *last;
    u64 last_jiffies;
    int err;
    last = kzalloc_obj(*last, GFP_KERNEL_ACCOUNT);
    if (!last)
    return -ENOMEM;
    if (tb[NFTA_LAST_SET])
    last.set = ntohl(nla_get_be32(tb[NFTA_LAST_SET]));
    if (last.set && tb[NFTA_LAST_MSECS]) {
    err = nf_msecs_to_jiffies64(tb[NFTA_LAST_MSECS], &last_jiffies);
    if (err < 0)
    goto err;
    last.jiffies = jiffies - (unsigned long)last_jiffies;
    }
    priv.last = last;
    return 0;
    err:
    kfree(last);
    return err;
    }
    static void nft_last_eval(const struct nft_expr *expr,
    struct nft_regs *regs, const struct nft_pktinfo *pkt)
    {
    struct nft_last_priv *priv = nft_expr_priv(expr);
    struct nft_last *last = priv.last;
    if (READ_ONCE(last.jiffies) != jiffies)
    WRITE_ONCE(last.jiffies, jiffies);
    if (READ_ONCE(last.set) == 0)
    WRITE_ONCE(last.set, 1);
    }
    static int nft_last_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    struct nft_last_priv *priv = nft_expr_priv(expr);
    struct nft_last *last = priv.last;
    let mut last_jiffies: c_ulong = READ_ONCE(last.jiffies);
    let mut last_set: u32 = READ_ONCE(last.set);
    __be64 msecs;
    if (time_before(jiffies, last_jiffies)) {
    WRITE_ONCE(last.set, 0);
    last_set = 0;
    }
    if (last_set)
    msecs = nf_jiffies64_to_msecs(jiffies - last_jiffies);
    else
    msecs = 0;
    if (nla_put_be32(skb, NFTA_LAST_SET, htonl(last_set)) ||
    nla_put_be64(skb, NFTA_LAST_MSECS, msecs, NFTA_LAST_PAD))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static void nft_last_destroy(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_last_priv *priv = nft_expr_priv(expr);
    kfree(priv.last);
    }
#[no_mangle]
unsafe extern "C" fn nft_last_clone(dst: *mut nft_expr, src: *const nft_expr, gfp: gfp_t) -> c_int {
    static int nft_last_clone(struct nft_expr *dst, const struct nft_expr *src, gfp_t gfp)
    {
    struct nft_last_priv *priv_dst = nft_expr_priv(dst);
    struct nft_last_priv *priv_src = nft_expr_priv(src);
    priv_dst.last = kzalloc_obj(*priv_dst.last, gfp);
    if (!priv_dst.last)
    return -ENOMEM;
    priv_dst.last.set = priv_src.last.set;
    priv_dst.last.jiffies = priv_src.last.jiffies;
    return 0;
    }
    static const struct nft_expr_ops nft_last_ops = {
    .type		= &nft_last_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_last_priv)),
    .eval		= nft_last_eval,
    .init		= nft_last_init,
    .destroy	= nft_last_destroy,
    .clone		= nft_last_clone,
    .dump		= nft_last_dump,
    };
    struct nft_expr_type nft_last_type __read_mostly = {
    .name		= "last",
    .ops		= &nft_last_ops,
    .policy		= nft_last_policy,
    .maxattr	= NFTA_LAST_MAX,
    .flags		= NFT_EXPR_STATEFUL,
    .owner		= THIS_MODULE,
    };
