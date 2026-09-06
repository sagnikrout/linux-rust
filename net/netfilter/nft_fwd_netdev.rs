//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_fwd_netdev.c
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
pub struct nft_fwd_netdev {
    pub sreg_dev: u8,
}

    static void nft_fwd_netdev_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_fwd_netdev *priv = nft_expr_priv(expr);
    let mut oif: c_int = regs.data[priv.sreg_dev];
    struct sk_buff *skb = pkt.skb;
// This is used by ifb only.
    skb.skb_iif = skb.dev.ifindex;
    skb_set_redirected(skb, nft_hook(pkt) == NF_NETDEV_INGRESS);
    nf_fwd_netdev_egress(pkt, oif);
    regs.verdict.code = NF_STOLEN;
    }
    static const struct nla_policy nft_fwd_netdev_policy[NFTA_FWD_MAX + 1] = {
    [NFTA_FWD_SREG_DEV]	= { .type = NLA_U32 },
    [NFTA_FWD_SREG_ADDR]	= { .type = NLA_U32 },
    [NFTA_FWD_NFPROTO]	= NLA_POLICY_MAX(NLA_BE32, 255),
    };
    static int nft_fwd_netdev_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_fwd_netdev *priv = nft_expr_priv(expr);
    if (tb[NFTA_FWD_SREG_DEV] == core::ptr::null_mut())
    return -EINVAL;
    return nft_parse_register_load(ctx, tb[NFTA_FWD_SREG_DEV], &priv.sreg_dev,
    sizeof(int));
    }
    static int nft_fwd_netdev_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    struct nft_fwd_netdev *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_FWD_SREG_DEV, priv.sreg_dev))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static int nft_fwd_netdev_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    const struct nft_expr *expr)
    {
    const struct nft_fwd_netdev *priv = nft_expr_priv(expr);
    let mut oif: c_int = ctx.regs[priv.sreg_dev].data.data[0];
    return nft_fwd_dup_netdev_offload(ctx, flow, FLOW_ACTION_REDIRECT, oif);
    }
#[no_mangle]
unsafe extern "C" fn nft_fwd_netdev_offload_action(expr: *const nft_expr) -> bool {
    static bool nft_fwd_netdev_offload_action(const struct nft_expr *expr)
    {
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_fwd_neigh {
    pub sreg_dev: u8,
    pub sreg_addr: u8,
    pub nfproto: u8,
}

    static void nft_fwd_neigh_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    struct nft_fwd_neigh *priv = nft_expr_priv(expr);
    void *addr = &regs.data[priv.sreg_addr];
    let mut oif: c_int = regs.data[priv.sreg_dev];
    let mut verdict: c_uint = NF_STOLEN;
    struct sk_buff *skb = pkt.skb;
    let mut nhoff: c_int = skb_network_offset(skb);
    struct net_device *dev;
    unsigned int hh_len;
    int neigh_table;
    switch (priv.nfproto) {
    case NFPROTO_IPV4: {
    struct iphdr *iph;
    if (skb.protocol != htons(ETH_P_IP)) {
    verdict = NFT_BREAK;
    goto out;
    }
    if (skb_ensure_writable(skb, nhoff + sizeof(*iph))) {
    verdict = NF_DROP;
    goto out;
    }
    iph = ip_hdr(skb);
    if (iph.ttl <= 1) {
    verdict = NF_DROP;
    goto out;
    }
    ip_decrease_ttl(iph);
    neigh_table = NEIGH_ARP_TABLE;
    break;
    }
    case NFPROTO_IPV6: {
    struct ipv6hdr *ip6h;
    if (skb.protocol != htons(ETH_P_IPV6)) {
    verdict = NFT_BREAK;
    goto out;
    }
    if (skb_ensure_writable(skb, nhoff + sizeof(*ip6h))) {
    verdict = NF_DROP;
    goto out;
    }
    ip6h = ipv6_hdr(skb);
    if (ip6h.hop_limit <= 1) {
    verdict = NF_DROP;
    goto out;
    }
    ip6h.hop_limit--;
    neigh_table = NEIGH_ND_TABLE;
    break;
    }
    default:
    verdict = NFT_BREAK;
    goto out;
    }
    dev = dev_get_by_index_rcu(nft_net(pkt), oif);
    if (!dev) {
    verdict = NF_DROP;
    goto out;
    }
    local_bh_disable();
    if (nf_dev_xmit_recursion()) {
    local_bh_enable();
    verdict = NF_DROP;
    goto out;
    }
    hh_len = LL_RESERVED_SPACE(dev);
    if (unlikely(skb_headroom(skb) < hh_len && dev.header_ops)) {
    skb = skb_expand_head(skb, hh_len);
    if (!skb) {
    local_bh_enable();
    goto out;
    }
    }
    skb.dev = dev;
    skb_clear_tstamp(skb);
    nf_dev_xmit_recursion_inc();
    neigh_xmit(neigh_table, dev, addr, skb);
    nf_dev_xmit_recursion_dec();
    local_bh_enable();
    out:
    regs.verdict.code = verdict;
    }
    static int nft_fwd_neigh_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_fwd_neigh *priv = nft_expr_priv(expr);
    unsigned int addr_len;
    int err;
    if (!tb[NFTA_FWD_SREG_DEV] ||
    !tb[NFTA_FWD_SREG_ADDR] ||
    !tb[NFTA_FWD_NFPROTO])
    return -EINVAL;
    priv.nfproto = ntohl(nla_get_be32(tb[NFTA_FWD_NFPROTO]));
    switch (priv.nfproto) {
    case NFPROTO_IPV4:
    addr_len = sizeof(struct in_addr);
    break;
    case NFPROTO_IPV6:
    addr_len = sizeof(struct in6_addr);
    break;
    default:
    return -EOPNOTSUPP;
    }
    err = nft_parse_register_load(ctx, tb[NFTA_FWD_SREG_DEV], &priv.sreg_dev,
    sizeof(int));
    if (err < 0)
    return err;
    return nft_parse_register_load(ctx, tb[NFTA_FWD_SREG_ADDR], &priv.sreg_addr,
    addr_len);
    }
    static int nft_fwd_neigh_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    struct nft_fwd_neigh *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_FWD_SREG_DEV, priv.sreg_dev) ||
    nft_dump_register(skb, NFTA_FWD_SREG_ADDR, priv.sreg_addr) ||
    nla_put_be32(skb, NFTA_FWD_NFPROTO, htonl(priv.nfproto)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    static int nft_fwd_validate(const struct nft_ctx *ctx,
    const struct nft_expr *expr)
    {
    return nft_chain_validate_hooks(ctx.chain, (1 << NF_NETDEV_INGRESS) |
    (1 << NF_NETDEV_EGRESS));
    }
    static struct nft_expr_type nft_fwd_netdev_type;
    static const struct nft_expr_ops nft_fwd_neigh_netdev_ops = {
    .type		= &nft_fwd_netdev_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_fwd_neigh)),
    .eval		= nft_fwd_neigh_eval,
    .init		= nft_fwd_neigh_init,
    .dump		= nft_fwd_neigh_dump,
    .validate	= nft_fwd_validate,
    };
    static const struct nft_expr_ops nft_fwd_netdev_ops = {
    .type		= &nft_fwd_netdev_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_fwd_netdev)),
    .eval		= nft_fwd_netdev_eval,
    .init		= nft_fwd_netdev_init,
    .dump		= nft_fwd_netdev_dump,
    .validate	= nft_fwd_validate,
    .offload	= nft_fwd_netdev_offload,
    .offload_action	= nft_fwd_netdev_offload_action,
    };
    static const struct nft_expr_ops *
    nft_fwd_select_ops(const struct nft_ctx *ctx,
    const struct nlattr * const tb[])
    {
    if (tb[NFTA_FWD_SREG_ADDR])
    return &nft_fwd_neigh_netdev_ops;
    if (tb[NFTA_FWD_SREG_DEV])
    return &nft_fwd_netdev_ops;
    return ERR_PTR(-EOPNOTSUPP);
    }
    static struct nft_expr_type nft_fwd_netdev_type __read_mostly = {
    .family		= NFPROTO_NETDEV,
    .name		= "fwd",
    .select_ops	= nft_fwd_select_ops,
    .policy		= nft_fwd_netdev_policy,
    .maxattr	= NFTA_FWD_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_fwd_netdev_module_init() -> int __init {
    static int __init nft_fwd_netdev_module_init(void)
    {
    return nft_register_expr(&nft_fwd_netdev_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_fwd_netdev_module_exit() -> void __exit {
    static void __exit nft_fwd_netdev_module_exit(void)
    {
    nft_unregister_expr(&nft_fwd_netdev_type);
    }
    module_init(nft_fwd_netdev_module_init);
    module_exit(nft_fwd_netdev_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
    MODULE_DESCRIPTION("nftables netdev packet forwarding support");
    MODULE_ALIAS_NFT_AF_EXPR(5, "fwd");
