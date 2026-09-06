//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_objref.c
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
// Copyright (c) 2012-2016 Pablo Neira Ayuso <pablo@netfilter.org>
//

    void nft_objref_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_object *obj = nft_objref_priv(expr);
    obj.ops.eval(obj, regs, pkt);
    }
#[no_mangle]
unsafe extern "C" fn nft_objref_validate_obj_type(ctx: *const nft_ctx, type: u32) -> c_int {
    static int nft_objref_validate_obj_type(const struct nft_ctx *ctx, u32 type)
    {
    unsigned int hooks;
    switch (type) {
    case NFT_OBJECT_SYNPROXY:
    if (ctx.family != NFPROTO_IPV4 &&
    ctx.family != NFPROTO_IPV6 &&
    ctx.family != NFPROTO_INET)
    return -EOPNOTSUPP;
    hooks = (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD);
    return nft_chain_validate_hooks(ctx.chain, hooks);
    default:
    break;
    }
    return 0;
    }
    static int nft_objref_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_object *obj = nft_objref_priv(expr);
    return nft_objref_validate_obj_type(ctx, obj.ops.type.type);
    }
    static int nft_objref_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_object *obj = nft_objref_priv(expr);
    let mut genmask: u8 = nft_genmask_next(ctx.net);
    u32 objtype;
    if (!tb[NFTA_OBJREF_IMM_NAME] ||
    !tb[NFTA_OBJREF_IMM_TYPE])
    return -EINVAL;
    objtype = ntohl(nla_get_be32(tb[NFTA_OBJREF_IMM_TYPE]));
    obj = nft_obj_lookup(ctx.net, ctx.table,
    tb[NFTA_OBJREF_IMM_NAME], objtype,
    genmask);
    if (IS_ERR(obj))
    return -ENOENT;
    if (!nft_use_inc(&obj.use))
    return -EMFILE;
    nft_objref_priv(expr) = obj;
    return 0;
    }
    static int nft_objref_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_object *obj = nft_objref_priv(expr);
    if (nla_put_string(skb, NFTA_OBJREF_IMM_NAME, obj.key.name) ||
    nla_put_be32(skb, NFTA_OBJREF_IMM_TYPE,
    htonl(obj.ops.type.type)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static void nft_objref_deactivate(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    enum nft_trans_phase phase)
    {
    struct nft_object *obj = nft_objref_priv(expr);
    if (phase == NFT_TRANS_COMMIT)
    return;
    nft_use_dec(&obj.use);
    }
    static void nft_objref_activate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_object *obj = nft_objref_priv(expr);
    nft_use_inc_restore(&obj.use);
    }
    static const struct nft_expr_ops nft_objref_ops = {
    .type		= &nft_objref_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_object *)),
    .eval		= nft_objref_eval,
    .init		= nft_objref_init,
    .activate	= nft_objref_activate,
    .deactivate	= nft_objref_deactivate,
    .dump		= nft_objref_dump,
    .validate	= nft_objref_validate,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_objref_map {
    pub set: *mut nft_set,
    pub sreg: u8,
    pub binding: nft_set_binding,
}

    void nft_objref_map_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_objref_map *priv = nft_expr_priv(expr);
    const struct nft_set *set = priv.set;
    struct net *net = nft_net(pkt);
    const struct nft_set_ext *ext;
    struct nft_object *obj;
    ext = nft_set_do_lookup(net, set, &regs.data[priv.sreg]);
    if (!ext) {
    ext = nft_set_catchall_lookup(net, set);
    if (!ext) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    }
    obj = *nft_set_ext_obj(ext);
    obj.ops.eval(obj, regs, pkt);
    }
    static int nft_objref_map_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_objref_map *priv = nft_expr_priv(expr);
    let mut genmask: u8 = nft_genmask_next(ctx.net);
    struct nft_set *set;
    int err;
    set = nft_set_lookup_global(ctx.net, ctx.table,
    tb[NFTA_OBJREF_SET_NAME],
    tb[NFTA_OBJREF_SET_ID], genmask);
    if (IS_ERR(set))
    return PTR_ERR(set);
    if (!(set.flags & NFT_SET_OBJECT))
    return -EINVAL;
    err = nft_parse_register_load(ctx, tb[NFTA_OBJREF_SET_SREG], &priv.sreg,
    set.klen);
    if (err < 0)
    return err;
    priv.binding.flags = set.flags & NFT_SET_OBJECT;
    err = nf_tables_bind_set(ctx, set, &priv.binding);
    if (err < 0)
    return err;
    priv.set = set;
    return 0;
    }
    static int nft_objref_map_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_objref_map *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_OBJREF_SET_SREG, priv.sreg) ||
    nla_put_string(skb, NFTA_OBJREF_SET_NAME, priv.set.name))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static void nft_objref_map_deactivate(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    enum nft_trans_phase phase)
    {
    struct nft_objref_map *priv = nft_expr_priv(expr);
    nf_tables_deactivate_set(ctx, priv.set, &priv.binding, phase);
    }
    static void nft_objref_map_activate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_objref_map *priv = nft_expr_priv(expr);
    nf_tables_activate_set(ctx, priv.set);
    }
    static void nft_objref_map_destroy(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_objref_map *priv = nft_expr_priv(expr);
    nf_tables_destroy_set(ctx, priv.set);
    }
    static int nft_objref_map_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    const struct nft_objref_map *priv = nft_expr_priv(expr);
    return nft_objref_validate_obj_type(ctx, priv.set.objtype);
    }
    static const struct nft_expr_ops nft_objref_map_ops = {
    .type		= &nft_objref_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_objref_map)),
    .eval		= nft_objref_map_eval,
    .init		= nft_objref_map_init,
    .activate	= nft_objref_map_activate,
    .deactivate	= nft_objref_map_deactivate,
    .destroy	= nft_objref_map_destroy,
    .dump		= nft_objref_map_dump,
    .validate	= nft_objref_map_validate,
    };
    static const struct nft_expr_ops *
    nft_objref_select_ops(const struct nft_ctx *ctx,
    const struct nlattr * const tb[])
    {
    if (tb[NFTA_OBJREF_SET_SREG] &&
    (tb[NFTA_OBJREF_SET_NAME] ||
    tb[NFTA_OBJREF_SET_ID]))
    return &nft_objref_map_ops;
    else if (tb[NFTA_OBJREF_IMM_NAME] &&
    tb[NFTA_OBJREF_IMM_TYPE])
    return &nft_objref_ops;
    return ERR_PTR(-EOPNOTSUPP);
    }
    static const struct nla_policy nft_objref_policy[NFTA_OBJREF_MAX + 1] = {
    [NFTA_OBJREF_IMM_NAME]	= { .type = NLA_STRING,
    .len = NFT_OBJ_MAXNAMELEN - 1 },
    [NFTA_OBJREF_IMM_TYPE]	= { .type = NLA_U32 },
    [NFTA_OBJREF_SET_SREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_OBJREF_SET_NAME]	= { .type = NLA_STRING,
    .len = NFT_SET_MAXNAMELEN - 1 },
    [NFTA_OBJREF_SET_ID]	= { .type = NLA_U32 },
    };
    struct nft_expr_type nft_objref_type __read_mostly = {
    .name		= "objref",
    .select_ops	= nft_objref_select_ops,
    .policy		= nft_objref_policy,
    .maxattr	= NFTA_OBJREF_MAX,
    .owner		= THIS_MODULE,
    };
