//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_redir.c
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
// Copyright (c) 2014 Arturo Borrero Gonzalez <arturo@debian.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_redir {
    pub sreg_proto_min: u8,
    pub sreg_proto_max: u8,
    pub flags: u16,
}

    static const struct nla_policy nft_redir_policy[NFTA_REDIR_MAX + 1] = {
    [NFTA_REDIR_REG_PROTO_MIN]	= { .type = NLA_U32 },
    [NFTA_REDIR_REG_PROTO_MAX]	= { .type = NLA_U32 },
    [NFTA_REDIR_FLAGS]		=
    NLA_POLICY_MASK(NLA_BE32, NF_NAT_RANGE_MASK),
    };
    static int nft_redir_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    int err;
    err = nft_chain_validate_dependency(ctx.chain, NFT_CHAIN_T_NAT);
    if (err < 0)
    return err;
    return nft_chain_validate_hooks(ctx.chain,
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_OUT));
    }
    static int nft_redir_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_redir *priv = nft_expr_priv(expr);
    unsigned int plen;
    int err;
    plen = sizeof_field(struct nf_nat_range, min_proto.all);
    if (tb[NFTA_REDIR_REG_PROTO_MIN]) {
    err = nft_parse_register_load(ctx, tb[NFTA_REDIR_REG_PROTO_MIN],
    &priv.sreg_proto_min, plen);
    if (err < 0)
    return err;
    if (tb[NFTA_REDIR_REG_PROTO_MAX]) {
    err = nft_parse_register_load(ctx, tb[NFTA_REDIR_REG_PROTO_MAX],
    &priv.sreg_proto_max,
    plen);
    if (err < 0)
    return err;
    } else {
    priv.sreg_proto_max = priv.sreg_proto_min;
    }
    priv.flags |= NF_NAT_RANGE_PROTO_SPECIFIED;
    }
    if (tb[NFTA_REDIR_FLAGS])
    priv.flags = ntohl(nla_get_be32(tb[NFTA_REDIR_FLAGS]));
    return nf_ct_netns_get(ctx.net, ctx.family);
    }
    static int nft_redir_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_redir *priv = nft_expr_priv(expr);
    if (priv.sreg_proto_min) {
    if (nft_dump_register(skb, NFTA_REDIR_REG_PROTO_MIN,
    priv.sreg_proto_min))
    goto nla_put_failure;
    if (nft_dump_register(skb, NFTA_REDIR_REG_PROTO_MAX,
    priv.sreg_proto_max))
    goto nla_put_failure;
    }
    if (priv.flags != 0 &&
    nla_put_be32(skb, NFTA_REDIR_FLAGS, htonl(priv.flags)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static void nft_redir_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_redir *priv = nft_expr_priv(expr);
    struct nf_nat_range2 range;
    memset(&range, 0, sizeof(range));
    range.flags = priv.flags;
    if (priv.sreg_proto_min) {
    range.min_proto.all = ( __be16)
    nft_reg_load16(&regs.data[priv.sreg_proto_min]);
    range.max_proto.all = ( __be16)
    nft_reg_load16(&regs.data[priv.sreg_proto_max]);
    }
    switch (nft_pf(pkt)) {
    case NFPROTO_IPV4:
    regs.verdict.code = nf_nat_redirect_ipv4(pkt.skb, &range,
    nft_hook(pkt));
    break;

    case NFPROTO_IPV6:
    regs.verdict.code = nf_nat_redirect_ipv6(pkt.skb, &range,
    nft_hook(pkt));
    break;

    default:
    DEBUG_NET_WARN_ON_ONCE(1);
    break;
    }
    }
    static void
    nft_redir_ipv4_destroy(const struct nft_ctx *ctx, const struct nft_expr *expr)
    {
    nf_ct_netns_put(ctx.net, NFPROTO_IPV4);
    }
    static struct nft_expr_type nft_redir_ipv4_type;
    static const struct nft_expr_ops nft_redir_ipv4_ops = {
    .type		= &nft_redir_ipv4_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_redir)),
    .eval		= nft_redir_eval,
    .init		= nft_redir_init,
    .destroy	= nft_redir_ipv4_destroy,
    .dump		= nft_redir_dump,
    .validate	= nft_redir_validate,
    };
    static struct nft_expr_type nft_redir_ipv4_type __read_mostly = {
    .family		= NFPROTO_IPV4,
    .name		= "redir",
    .ops		= &nft_redir_ipv4_ops,
    .policy		= nft_redir_policy,
    .maxattr	= NFTA_REDIR_MAX,
    .owner		= THIS_MODULE,
    };

    static void
    nft_redir_ipv6_destroy(const struct nft_ctx *ctx, const struct nft_expr *expr)
    {
    nf_ct_netns_put(ctx.net, NFPROTO_IPV6);
    }
    static struct nft_expr_type nft_redir_ipv6_type;
    static const struct nft_expr_ops nft_redir_ipv6_ops = {
    .type		= &nft_redir_ipv6_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_redir)),
    .eval		= nft_redir_eval,
    .init		= nft_redir_init,
    .destroy	= nft_redir_ipv6_destroy,
    .dump		= nft_redir_dump,
    .validate	= nft_redir_validate,
    };
    static struct nft_expr_type nft_redir_ipv6_type __read_mostly = {
    .family		= NFPROTO_IPV6,
    .name		= "redir",
    .ops		= &nft_redir_ipv6_ops,
    .policy		= nft_redir_policy,
    .maxattr	= NFTA_REDIR_MAX,
    .owner		= THIS_MODULE,
    };

    static void
    nft_redir_inet_destroy(const struct nft_ctx *ctx, const struct nft_expr *expr)
    {
    nf_ct_netns_put(ctx.net, NFPROTO_INET);
    }
    static struct nft_expr_type nft_redir_inet_type;
    static const struct nft_expr_ops nft_redir_inet_ops = {
    .type		= &nft_redir_inet_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_redir)),
    .eval		= nft_redir_eval,
    .init		= nft_redir_init,
    .destroy	= nft_redir_inet_destroy,
    .dump		= nft_redir_dump,
    .validate	= nft_redir_validate,
    };
    static struct nft_expr_type nft_redir_inet_type __read_mostly = {
    .family		= NFPROTO_INET,
    .name		= "redir",
    .ops		= &nft_redir_inet_ops,
    .policy		= nft_redir_policy,
    .maxattr	= NFTA_REDIR_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_redir_module_init_inet() -> int __init {
    static int __init nft_redir_module_init_inet(void)
    {
    return nft_register_expr(&nft_redir_inet_type);
    }

    static inline int nft_redir_module_init_inet(void) { return 0; }

#[no_mangle]
unsafe extern "C" fn nft_redir_module_init() -> int __init {
    static int __init nft_redir_module_init(void)
    {
    let mut ret: c_int = nft_register_expr(&nft_redir_ipv4_type);
    if (ret)
    return ret;

    ret = nft_register_expr(&nft_redir_ipv6_type);
    if (ret) {
    nft_unregister_expr(&nft_redir_ipv4_type);
    return ret;
    }

    ret = nft_redir_module_init_inet();
    if (ret < 0) {
    nft_unregister_expr(&nft_redir_ipv4_type);

    nft_unregister_expr(&nft_redir_ipv6_type);

    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nft_redir_module_exit() -> void __exit {
    static void __exit nft_redir_module_exit(void)
    {
    nft_unregister_expr(&nft_redir_ipv4_type);

    nft_unregister_expr(&nft_redir_ipv6_type);

    nft_unregister_expr(&nft_redir_inet_type);

    }
    module_init(nft_redir_module_init);
    module_exit(nft_redir_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Arturo Borrero Gonzalez <arturo@debian.org>");
    MODULE_ALIAS_NFT_EXPR("redir");
    MODULE_DESCRIPTION("Netfilter nftables redirect support");
