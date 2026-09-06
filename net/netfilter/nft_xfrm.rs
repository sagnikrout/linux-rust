//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_xfrm.c
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
// Generic part shared by ipv4 and ipv6 backends.
//

    static const struct nla_policy nft_xfrm_policy[NFTA_XFRM_MAX + 1] = {
    [NFTA_XFRM_KEY]		= NLA_POLICY_MAX(NLA_BE32, 255),
    [NFTA_XFRM_DIR]		= NLA_POLICY_MAX(NLA_U8, XFRM_POLICY_OUT),
    [NFTA_XFRM_SPNUM]	= NLA_POLICY_MAX(NLA_BE32, XFRM_MAX_DEPTH - 1),
    [NFTA_XFRM_DREG]	= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_xfrm {
    pub key:8: enum nft_xfrm_keys,
    pub dreg: u8,
    pub dir: u8,
    pub spnum: u8,
    pub len: u8,
}

    static int nft_xfrm_get_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_xfrm *priv = nft_expr_priv(expr);
    let mut len: c_uint = 0;
    let mut spnum: u32 = 0;
    u8 dir;
    if (!tb[NFTA_XFRM_KEY] || !tb[NFTA_XFRM_DIR] || !tb[NFTA_XFRM_DREG])
    return -EINVAL;
    switch (ctx.family) {
    case NFPROTO_IPV4:
    case NFPROTO_IPV6:
    case NFPROTO_INET:
    break;
    default:
    return -EOPNOTSUPP;
    }
    priv.key = ntohl(nla_get_be32(tb[NFTA_XFRM_KEY]));
    switch (priv.key) {
    case NFT_XFRM_KEY_REQID:
    case NFT_XFRM_KEY_SPI:
    len = sizeof(u32);
    break;
    case NFT_XFRM_KEY_DADDR_IP4:
    case NFT_XFRM_KEY_SADDR_IP4:
    len = sizeof(struct in_addr);
    break;
    case NFT_XFRM_KEY_DADDR_IP6:
    case NFT_XFRM_KEY_SADDR_IP6:
    len = sizeof(struct in6_addr);
    break;
    default:
    return -EINVAL;
    }
    dir = nla_get_u8(tb[NFTA_XFRM_DIR]);
    switch (dir) {
    case XFRM_POLICY_IN:
    case XFRM_POLICY_OUT:
    priv.dir = dir;
    break;
    default:
    return -EINVAL;
    }
    if (tb[NFTA_XFRM_SPNUM])
    spnum = ntohl(nla_get_be32(tb[NFTA_XFRM_SPNUM]));
    if (spnum >= XFRM_MAX_DEPTH)
    return -ERANGE;
    priv.spnum = spnum;
    priv.len = len;
    return nft_parse_register_store(ctx, tb[NFTA_XFRM_DREG], &priv.dreg,
    core::ptr::null_mut(), NFT_DATA_VALUE, len);
    }
// Return true if key asks for daddr/saddr and current
// state does have a valid address (BEET, TUNNEL).
//
#[no_mangle]
unsafe extern "C" fn xfrm_state_addr_ok(k: enum nft_xfrm_keys, family: u8, mode: u8) -> bool {
    static bool xfrm_state_addr_ok(enum nft_xfrm_keys k, u8 family, u8 mode)
    {
    switch (k) {
    case NFT_XFRM_KEY_DADDR_IP4:
    case NFT_XFRM_KEY_SADDR_IP4:
    if (family == NFPROTO_IPV4)
    break;
    return false;
    case NFT_XFRM_KEY_DADDR_IP6:
    case NFT_XFRM_KEY_SADDR_IP6:
    if (family == NFPROTO_IPV6)
    break;
    return false;
    default:
    return true;
    }
    return mode == XFRM_MODE_BEET || mode == XFRM_MODE_TUNNEL ||
    mode == XFRM_MODE_IPTFS;
    }
    static void nft_xfrm_state_get_key(const struct nft_xfrm *priv,
    struct nft_regs *regs,
    const struct xfrm_state *state)
    {
    u32 *dest = &regs.data[priv.dreg];
    if (!xfrm_state_addr_ok(priv.key,
    state.props.family,
    state.props.mode)) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    switch (priv.key) {
    case NFT_XFRM_KEY_UNSPEC:
    case __NFT_XFRM_KEY_MAX:
    DEBUG_NET_WARN_ON_ONCE(1);
    break;
    case NFT_XFRM_KEY_DADDR_IP4:
// dest = ( __u32)state->id.daddr.a4;
    return;
    case NFT_XFRM_KEY_DADDR_IP6:
    memcpy(dest, &state.id.daddr.in6, sizeof(struct in6_addr));
    return;
    case NFT_XFRM_KEY_SADDR_IP4:
// dest = ( __u32)state->props.saddr.a4;
    return;
    case NFT_XFRM_KEY_SADDR_IP6:
    memcpy(dest, &state.props.saddr.in6, sizeof(struct in6_addr));
    return;
    case NFT_XFRM_KEY_REQID:
// dest = state->props.reqid;
    return;
    case NFT_XFRM_KEY_SPI:
// dest = ( __u32)state->id.spi;
    return;
    }
    regs.verdict.code = NFT_BREAK;
    }
    static void nft_xfrm_get_eval_in(const struct nft_xfrm *priv,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct sec_path *sp = skb_sec_path(pkt.skb);
    const struct xfrm_state *state;
    if (sp == core::ptr::null_mut() || sp.len <= priv.spnum) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    state = sp.xvec[priv.spnum];
    nft_xfrm_state_get_key(priv, regs, state);
    }
    static void nft_xfrm_get_eval_out(const struct nft_xfrm *priv,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct dst_entry *dst;
    int i;
    if (!skb_valid_dst(pkt.skb)) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    dst = skb_dst(pkt.skb);
    for (i = 0; dst && dst.xfrm;
    dst = ((const struct xfrm_dst *)dst).child, i++) {
    if (i < priv.spnum)
    continue;
    nft_xfrm_state_get_key(priv, regs, dst.xfrm);
    return;
    }
    regs.verdict.code = NFT_BREAK;
    }
    static void nft_xfrm_get_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_xfrm *priv = nft_expr_priv(expr);
    switch (priv.dir) {
    case XFRM_POLICY_IN:
    nft_xfrm_get_eval_in(priv, regs, pkt);
    break;
    case XFRM_POLICY_OUT:
    nft_xfrm_get_eval_out(priv, regs, pkt);
    break;
    default:
    DEBUG_NET_WARN_ON_ONCE(1);
    regs.verdict.code = NFT_BREAK;
    break;
    }
    }
    static int nft_xfrm_get_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_xfrm *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_XFRM_DREG, priv.dreg))
    return -1;
    if (nla_put_be32(skb, NFTA_XFRM_KEY, htonl(priv.key)))
    return -1;
    if (nla_put_u8(skb, NFTA_XFRM_DIR, priv.dir))
    return -1;
    if (nla_put_be32(skb, NFTA_XFRM_SPNUM, htonl(priv.spnum)))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nft_xfrm_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int {
    static int nft_xfrm_validate(const struct nft_ctx *ctx, const struct nft_expr *expr)
    {
    const struct nft_xfrm *priv = nft_expr_priv(expr);
    unsigned int hooks;
    if (ctx.family != NFPROTO_IPV4 &&
    ctx.family != NFPROTO_IPV6 &&
    ctx.family != NFPROTO_INET)
    return -EOPNOTSUPP;
    switch (priv.dir) {
    case XFRM_POLICY_IN:
    hooks = (1 << NF_INET_FORWARD) |
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_PRE_ROUTING);
    break;
    case XFRM_POLICY_OUT:
    hooks = (1 << NF_INET_FORWARD) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING);
    break;
    default:
    DEBUG_NET_WARN_ON_ONCE(1);
    return -EINVAL;
    }
    return nft_chain_validate_hooks(ctx.chain, hooks);
    }
    static struct nft_expr_type nft_xfrm_type;
    static const struct nft_expr_ops nft_xfrm_get_ops = {
    .type		= &nft_xfrm_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_xfrm)),
    .eval		= nft_xfrm_get_eval,
    .init		= nft_xfrm_get_init,
    .dump		= nft_xfrm_get_dump,
    .validate	= nft_xfrm_validate,
    };
    static struct nft_expr_type nft_xfrm_type __read_mostly = {
    .name		= "xfrm",
    .ops		= &nft_xfrm_get_ops,
    .policy		= nft_xfrm_policy,
    .maxattr	= NFTA_XFRM_MAX,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn nft_xfrm_module_init() -> int __init {
    static int __init nft_xfrm_module_init(void)
    {
    return nft_register_expr(&nft_xfrm_type);
    }
#[no_mangle]
unsafe extern "C" fn nft_xfrm_module_exit() -> void __exit {
    static void __exit nft_xfrm_module_exit(void)
    {
    nft_unregister_expr(&nft_xfrm_type);
    }
    module_init(nft_xfrm_module_init);
    module_exit(nft_xfrm_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("nf_tables: xfrm/IPSec matching");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
    MODULE_AUTHOR("Máté Eckl <ecklm94@gmail.com>");
    MODULE_ALIAS_NFT_EXPR("xfrm");
