//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_connlimit.c
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_connlimit {
    pub list: *mut nf_conncount_list,
    pub limit: u32,
    pub invert: bool,
}

    static inline void nft_connlimit_do_eval(struct nft_connlimit *priv,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt,
    const struct nft_set_ext *ext)
    {
    unsigned int count;
    int err;
    err = nf_conncount_add_skb(nft_net(pkt), pkt.skb, nft_pf(pkt), priv.list);
    if (err) {
    if (err == -EEXIST) {
// Call gc to update the list count if any connection has
// been closed already. This is useful for softlimit
// connections like limiting bandwidth based on a number
// of open connections.
//
    nf_conncount_gc_list(nft_net(pkt), priv.list);
    } else {
    regs.verdict.code = NF_DROP;
    return;
    }
    }
    count = READ_ONCE(priv.list.count);
    if ((count > READ_ONCE(priv.limit)) ^ READ_ONCE(priv.invert)) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    }
    static int nft_connlimit_do_init(const struct nft_ctx *ctx,
    const struct nlattr * const tb[],
    struct nft_connlimit *priv)
    {
    let mut invert: bool = false;
    u32 flags, limit;
    int err;
    if (!tb[NFTA_CONNLIMIT_COUNT])
    return -EINVAL;
    limit = ntohl(nla_get_be32(tb[NFTA_CONNLIMIT_COUNT]));
    if (tb[NFTA_CONNLIMIT_FLAGS]) {
    flags = ntohl(nla_get_be32(tb[NFTA_CONNLIMIT_FLAGS]));
    if (flags & ~NFT_CONNLIMIT_F_INV)
    return -EOPNOTSUPP;
    if (flags & NFT_CONNLIMIT_F_INV)
    invert = true;
    }
    priv.list = kmalloc_obj(*priv.list, GFP_KERNEL_ACCOUNT);
    if (!priv.list)
    return -ENOMEM;
    nf_conncount_list_init(priv.list);
    priv.limit	= limit;
    priv.invert	= invert;
    err = nf_ct_netns_get(ctx.net, ctx.family);
    if (err < 0)
    goto err_netns;
    return 0;
    err_netns:
    kfree(priv.list);
    return err;
    }
    static void nft_connlimit_do_destroy(const struct nft_ctx *ctx,
    struct nft_connlimit *priv)
    {
    nf_ct_netns_put(ctx.net, ctx.family);
    nf_conncount_cache_free(priv.list);
    kfree(priv.list);
    }
    static int nft_connlimit_do_dump(struct sk_buff *skb,
    struct nft_connlimit *priv)
    {
    if (nla_put_be32(skb, NFTA_CONNLIMIT_COUNT, htonl(priv.limit)))
    goto nla_put_failure;
    if (priv.invert &&
    nla_put_be32(skb, NFTA_CONNLIMIT_FLAGS, htonl(NFT_CONNLIMIT_F_INV)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static inline void nft_connlimit_obj_eval(struct nft_object *obj,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_connlimit *priv = nft_obj_data(obj);
    nft_connlimit_do_eval(priv, regs, pkt, core::ptr::null_mut());
    }
    static int nft_connlimit_obj_init(const struct nft_ctx *ctx,
    const struct nlattr * const tb[],
    struct nft_object *obj)
    {
    struct nft_connlimit *priv = nft_obj_data(obj);
    return nft_connlimit_do_init(ctx, tb, priv);
    }
    static void nft_connlimit_obj_update(struct nft_object *obj,
    struct nft_object *newobj)
    {
    struct nft_connlimit *newpriv = nft_obj_data(newobj);
    struct nft_connlimit *priv = nft_obj_data(obj);
    WRITE_ONCE(priv.limit, newpriv.limit);
    WRITE_ONCE(priv.invert, newpriv.invert);
    }
    static void nft_connlimit_obj_destroy(const struct nft_ctx *ctx,
    struct nft_object *obj)
    {
    struct nft_connlimit *priv = nft_obj_data(obj);
    nft_connlimit_do_destroy(ctx, priv);
    }
    static int nft_connlimit_obj_dump(struct sk_buff *skb,
    struct nft_object *obj, bool reset)
    {
    struct nft_connlimit *priv = nft_obj_data(obj);
    return nft_connlimit_do_dump(skb, priv);
    }
    static const struct nla_policy nft_connlimit_policy[NFTA_CONNLIMIT_MAX + 1] = {
    [NFTA_CONNLIMIT_COUNT]	= { .type = NLA_U32 },
    [NFTA_CONNLIMIT_FLAGS]	= NLA_POLICY_MASK(NLA_BE32, NFT_CONNLIMIT_F_INV),
    };
    static struct nft_object_type nft_connlimit_obj_type;
    static const struct nft_object_ops nft_connlimit_obj_ops = {
    .type		= &nft_connlimit_obj_type,
    .size		= sizeof(struct nft_connlimit),
    .eval		= nft_connlimit_obj_eval,
    .init		= nft_connlimit_obj_init,
    .destroy	= nft_connlimit_obj_destroy,
    .dump		= nft_connlimit_obj_dump,
    .update		= nft_connlimit_obj_update,
    };
    static struct nft_object_type nft_connlimit_obj_type __read_mostly = {
    .type		= NFT_OBJECT_CONNLIMIT,
    .ops		= &nft_connlimit_obj_ops,
    .maxattr	= NFTA_CONNLIMIT_MAX,
    .policy		= nft_connlimit_policy,
    .owner		= THIS_MODULE,
    };
    static void nft_connlimit_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_connlimit *priv = nft_expr_priv(expr);
    nft_connlimit_do_eval(priv, regs, pkt, core::ptr::null_mut());
    }
    static int nft_connlimit_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    struct nft_connlimit *priv = nft_expr_priv(expr);
    return nft_connlimit_do_dump(skb, priv);
    }
    static int nft_connlimit_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_connlimit *priv = nft_expr_priv(expr);
    return nft_connlimit_do_init(ctx, tb, priv);
    }
    static void nft_connlimit_destroy(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_connlimit *priv = nft_expr_priv(expr);
    nft_connlimit_do_destroy(ctx, priv);
    }
#[no_mangle]
unsafe extern "C" fn nft_connlimit_clone(dst: *mut nft_expr, src: *const nft_expr, gfp: gfp_t) -> c_int {
    static int nft_connlimit_clone(struct nft_expr *dst, const struct nft_expr *src, gfp_t gfp)
    {
    struct nft_connlimit *priv_dst = nft_expr_priv(dst);
    struct nft_connlimit *priv_src = nft_expr_priv(src);
    priv_dst.list = kmalloc_obj(*priv_dst.list, gfp);
    if (!priv_dst.list)
    return -ENOMEM;
    nf_conncount_list_init(priv_dst.list);
    priv_dst.limit	 = priv_src.limit;
    priv_dst.invert = priv_src.invert;
    return 0;
    }
    static void nft_connlimit_destroy_clone(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    struct nft_connlimit *priv = nft_expr_priv(expr);
    nf_conncount_cache_free(priv.list);
    kfree(priv.list);
    }
#[no_mangle]
unsafe extern "C" fn nft_connlimit_gc(net: *mut net, expr: *const nft_expr) -> bool {
    static bool nft_connlimit_gc(struct net *net, const struct nft_expr *expr)
    {
    struct nft_connlimit *priv = nft_expr_priv(expr);
    return nf_conncount_gc_list(net, priv.list);
    }
    static struct nft_expr_type nft_connlimit_type;
    static const struct nft_expr_ops nft_connlimit_ops = {
    .type		= &nft_connlimit_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_connlimit)),
    .eval		= nft_connlimit_eval,
    .init		= nft_connlimit_init,
    .destroy	= nft_connlimit_destroy,
    .clone		= nft_connlimit_clone,
    .destroy_clone	= nft_connlimit_destroy_clone,
    .dump		= nft_connlimit_dump,
    .gc		= nft_connlimit_gc,
    };
    static struct nft_expr_type nft_connlimit_type __read_mostly = {
    .name		= "connlimit",
    .ops		= &nft_connlimit_ops,
    .policy		= nft_connlimit_policy,
    .maxattr	= NFTA_CONNLIMIT_MAX,
    .flags		= NFT_EXPR_STATEFUL | NFT_EXPR_GC,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_connlimit_module_init() -> int __init {
    static int __init nft_connlimit_module_init(void)
    {
    int err;
    err = nft_register_obj(&nft_connlimit_obj_type);
    if (err < 0)
    return err;
    err = nft_register_expr(&nft_connlimit_type);
    if (err < 0)
    goto err1;
    return 0;
    err1:
    nft_unregister_obj(&nft_connlimit_obj_type);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nft_connlimit_module_exit() -> void __exit {
    static void __exit nft_connlimit_module_exit(void)
    {
    nft_unregister_expr(&nft_connlimit_type);
    nft_unregister_obj(&nft_connlimit_obj_type);
    }
    module_init(nft_connlimit_module_init);
    module_exit(nft_connlimit_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pablo Neira Ayuso");
    MODULE_ALIAS_NFT_EXPR("connlimit");
    MODULE_ALIAS_NFT_OBJ(NFT_OBJECT_CONNLIMIT);
    MODULE_DESCRIPTION("nftables connlimit rule support");
