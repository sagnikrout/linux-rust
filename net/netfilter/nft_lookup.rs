//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_lookup.c
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
// Copyright (c) 2009 Patrick McHardy <kaber@trash.net>
//
// Development of this code funded by Astaro AG (http://www.astaro.com/)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_lookup {
    pub set: *mut nft_set,
    pub sreg: u8,
    pub dreg: u8,
    pub dreg_set: bool,
    pub invert: bool,
    pub binding: nft_set_binding,
}

    static const struct nft_set_ext *
    __nft_set_do_lookup(const struct net *net, const struct nft_set *set,
    const u32 *key)
    {

    if (set.ops == &nft_set_hash_fast_type.ops)
    return nft_hash_lookup_fast(net, set, key);
    if (set.ops == &nft_set_hash_type.ops)
    return nft_hash_lookup(net, set, key);
    if (set.ops == &nft_set_rhash_type.ops)
    return nft_rhash_lookup(net, set, key);
    if (set.ops == &nft_set_bitmap_type.ops)
    return nft_bitmap_lookup(net, set, key);
    if (set.ops == &nft_set_pipapo_type.ops)
    return nft_pipapo_lookup(net, set, key);

    if (set.ops == &nft_set_pipapo_avx2_type.ops)
    return nft_pipapo_avx2_lookup(net, set, key);

    if (set.ops == &nft_set_rbtree_type.ops)
    return nft_rbtree_lookup(net, set, key);
    DEBUG_NET_WARN_ON_ONCE(1);

    return set.ops.lookup(net, set, key);
    }
#[no_mangle]
unsafe extern "C" fn nft_base_seq(net: *const net) -> c_uint {
    static unsigned int nft_base_seq(const struct net *net)
    {
// pairs with smp_store_release() in nf_tables_commit()
    return smp_load_acquire(&net.nft.base_seq);
    }
#[no_mangle]
unsafe extern "C" fn nft_lookup_should_retry(net: *const net, seq: c_uint) -> bool {
    static bool nft_lookup_should_retry(const struct net *net, unsigned int seq)
    {
    return unlikely(seq != nft_base_seq(net));
    }
    const struct nft_set_ext *
    nft_set_do_lookup(const struct net *net, const struct nft_set *set,
    const u32 *key)
    {
    const struct nft_set_ext *ext;
    unsigned int base_seq;
    do {
    base_seq = nft_base_seq(net);
    ext = __nft_set_do_lookup(net, set, key);
    if (ext)
    break;
// No match?  There is a small chance that lookup was
// performed in the old generation, but nf_tables_commit()
// already unlinked a (matching) element.
//
// We need to repeat the lookup to make sure that we didn't
// miss a matching element in the new generation.
//
    } while (nft_lookup_should_retry(net, base_seq));
    return ext;
    }
    EXPORT_SYMBOL_GPL(nft_set_do_lookup);
    void nft_lookup_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_lookup *priv = nft_expr_priv(expr);
    const struct nft_set *set = priv.set;
    const struct net *net = nft_net(pkt);
    const struct nft_set_ext *ext;
    bool found;
    ext = nft_set_do_lookup(net, set, &regs.data[priv.sreg]);
    if (!ext)
    ext = nft_set_catchall_lookup(net, set);
    found = !!ext ^ priv.invert;
    if (!found) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    if (ext) {
    if (priv.dreg_set)
    nft_data_copy(&regs.data[priv.dreg],
    nft_set_ext_data(ext), set.dlen);
    nft_set_elem_update_expr(ext, regs, pkt);
    }
    }
    static const struct nla_policy nft_lookup_policy[NFTA_LOOKUP_MAX + 1] = {
    [NFTA_LOOKUP_SET]	= { .type = NLA_STRING,
    .len = NFT_SET_MAXNAMELEN - 1 },
    [NFTA_LOOKUP_SET_ID]	= { .type = NLA_U32 },
    [NFTA_LOOKUP_SREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_LOOKUP_DREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_LOOKUP_FLAGS]	=
    NLA_POLICY_MASK(NLA_BE32, NFT_LOOKUP_F_INV),
    };
    static int nft_lookup_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_lookup *priv = nft_expr_priv(expr);
    let mut genmask: u8 = nft_genmask_next(ctx.net);
    struct nft_set *set;
    u32 flags;
    int err;
    if (tb[NFTA_LOOKUP_SET] == core::ptr::null_mut() ||
    tb[NFTA_LOOKUP_SREG] == core::ptr::null_mut())
    return -EINVAL;
    set = nft_set_lookup_global(ctx.net, ctx.table, tb[NFTA_LOOKUP_SET],
    tb[NFTA_LOOKUP_SET_ID], genmask);
    if (IS_ERR(set))
    return PTR_ERR(set);
    err = nft_parse_register_load(ctx, tb[NFTA_LOOKUP_SREG], &priv.sreg,
    set.klen);
    if (err < 0)
    return err;
    if (tb[NFTA_LOOKUP_FLAGS]) {
    flags = ntohl(nla_get_be32(tb[NFTA_LOOKUP_FLAGS]));
    if (flags & NFT_LOOKUP_F_INV)
    priv.invert = true;
    }
    if (tb[NFTA_LOOKUP_DREG] != core::ptr::null_mut()) {
    if (priv.invert)
    return -EINVAL;
    if (!(set.flags & NFT_SET_MAP))
    return -EINVAL;
    err = nft_parse_register_store(ctx, tb[NFTA_LOOKUP_DREG],
    &priv.dreg, core::ptr::null_mut(),
    nft_set_datatype(set),
    set.dlen);
    if (err < 0)
    return err;
    priv.dreg_set = true;
    } else if (set.flags & NFT_SET_MAP) {
// Map given, but user asks for lookup only (i.e. to
// ignore value assoicated with key).
//
// This makes no sense for anonymous maps since they are
// scoped to the rule, but for named sets this can be useful.
//
    if (set.flags & NFT_SET_ANONYMOUS)
    return -EINVAL;
    }
    priv.binding.flags = set.flags & NFT_SET_MAP;
    err = nf_tables_bind_set(ctx, set, &priv.binding);
    if (err < 0)
    return err;
    priv.set = set;
    return 0;
    }
    static void nft_lookup_deactivate(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    enum nft_trans_phase phase)
    {
    struct nft_lookup *priv = nft_expr_priv(expr);
    nf_tables_deactivate_set(ctx, priv.set, &priv.binding, phase);
    }
    static void nft_lookup_activate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_lookup *priv = nft_expr_priv(expr);
    nf_tables_activate_set(ctx, priv.set);
    }
    static void nft_lookup_destroy(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_lookup *priv = nft_expr_priv(expr);
    nf_tables_destroy_set(ctx, priv.set);
    }
    static int nft_lookup_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_lookup *priv = nft_expr_priv(expr);
    let mut flags: u32 = priv.invert ? NFT_LOOKUP_F_INV : 0;
    if (nla_put_string(skb, NFTA_LOOKUP_SET, priv.set.name))
    goto nla_put_failure;
    if (nft_dump_register(skb, NFTA_LOOKUP_SREG, priv.sreg))
    goto nla_put_failure;
    if (priv.dreg_set)
    if (nft_dump_register(skb, NFTA_LOOKUP_DREG, priv.dreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_LOOKUP_FLAGS, htonl(flags)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static int nft_lookup_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    const struct nft_lookup *priv = nft_expr_priv(expr);
    struct nft_set_iter iter = {
    .genmask	= nft_genmask_next(ctx.net),
    .type		= NFT_ITER_UPDATE,
    .fn		= nft_setelem_validate,
    };
    if (!(priv.set.flags & NFT_SET_MAP) ||
    priv.set.dtype != NFT_DATA_VERDICT)
    return 0;
    priv.set.ops.walk(ctx, priv.set, &iter);
    if (!iter.err)
    iter.err = nft_set_catchall_validate(ctx, priv.set);
    if (iter.err < 0)
    return iter.err;
    return 0;
    }
    static const struct nft_expr_ops nft_lookup_ops = {
    .type		= &nft_lookup_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_lookup)),
    .eval		= nft_lookup_eval,
    .init		= nft_lookup_init,
    .activate	= nft_lookup_activate,
    .deactivate	= nft_lookup_deactivate,
    .destroy	= nft_lookup_destroy,
    .dump		= nft_lookup_dump,
    .validate	= nft_lookup_validate,
    };
    struct nft_expr_type nft_lookup_type __read_mostly = {
    .name		= "lookup",
    .ops		= &nft_lookup_ops,
    .policy		= nft_lookup_policy,
    .maxattr	= NFTA_LOOKUP_MAX,
    .owner		= THIS_MODULE,
    };
