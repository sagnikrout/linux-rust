//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_fib.c
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

    NFTA_FIB_F_MARK | NFTA_FIB_F_IIF | NFTA_FIB_F_OIF | \
    NFTA_FIB_F_PRESENT)
    const struct nla_policy nft_fib_policy[NFTA_FIB_MAX + 1] = {
    [NFTA_FIB_DREG]		= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_FIB_RESULT]	= { .type = NLA_U32 },
    [NFTA_FIB_FLAGS]	=
    NLA_POLICY_MASK(NLA_BE32, NFTA_FIB_F_ALL),
    };
    EXPORT_SYMBOL(nft_fib_policy);
#[no_mangle]
pub unsafe extern "C" fn nft_fib_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int {
    int nft_fib_validate(const struct nft_ctx *ctx, const struct nft_expr *expr)
    {
    const struct nft_fib *priv = nft_expr_priv(expr);
    unsigned int hooks;
    switch (ctx.family) {
    case NFPROTO_IPV4:
    case NFPROTO_IPV6:
    case NFPROTO_INET:
    break;
    default:
    return -EOPNOTSUPP;
    }
    switch (priv.result) {
    case NFT_FIB_RESULT_OIF:
    case NFT_FIB_RESULT_OIFNAME:
    hooks = (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_FORWARD);
    break;
    case NFT_FIB_RESULT_ADDRTYPE:
    if (priv.flags & NFTA_FIB_F_IIF)
    hooks = (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_FORWARD);
#[no_mangle]
pub unsafe extern "C" fn if(NFTA_FIB_F_OIF: priv->flags &) -> else {
    else if (priv.flags & NFTA_FIB_F_OIF)
    hooks = (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_FORWARD);
    else
    hooks = (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_FORWARD) |
    (1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_POST_ROUTING);
    break;
    default:
    return -EINVAL;
    }
    return nft_chain_validate_hooks(ctx.chain, hooks);
    }
    EXPORT_SYMBOL_GPL(nft_fib_validate);
    int nft_fib_init(const struct nft_ctx *ctx, const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_fib *priv = nft_expr_priv(expr);
    unsigned int len;
    int err;
    if (!tb[NFTA_FIB_DREG] || !tb[NFTA_FIB_RESULT] || !tb[NFTA_FIB_FLAGS])
    return -EINVAL;
    priv.flags = ntohl(nla_get_be32(tb[NFTA_FIB_FLAGS]));
    if (priv.flags == 0)
    return -EINVAL;
    if ((priv.flags & (NFTA_FIB_F_SADDR | NFTA_FIB_F_DADDR)) ==
    (NFTA_FIB_F_SADDR | NFTA_FIB_F_DADDR))
    return -EINVAL;
    if ((priv.flags & (NFTA_FIB_F_IIF | NFTA_FIB_F_OIF)) ==
    (NFTA_FIB_F_IIF | NFTA_FIB_F_OIF))
    return -EINVAL;
    if ((priv.flags & (NFTA_FIB_F_SADDR | NFTA_FIB_F_DADDR)) == 0)
    return -EINVAL;
    priv.result = ntohl(nla_get_be32(tb[NFTA_FIB_RESULT]));
    switch (priv.result) {
    case NFT_FIB_RESULT_OIF:
    if (priv.flags & NFTA_FIB_F_OIF)
    return -EINVAL;
    len = sizeof(int);
    break;
    case NFT_FIB_RESULT_OIFNAME:
    if (priv.flags & NFTA_FIB_F_OIF)
    return -EINVAL;
    len = IFNAMSIZ;
    break;
    case NFT_FIB_RESULT_ADDRTYPE:
    len = sizeof(u32);
    break;
    default:
    return -EINVAL;
    }
    if (priv.flags & NFTA_FIB_F_PRESENT) {
    if (priv.result != NFT_FIB_RESULT_OIF)
    return -EINVAL;
    len = sizeof(u8);
    }
    err = nft_parse_register_store(ctx, tb[NFTA_FIB_DREG], &priv.dreg,
    core::ptr::null_mut(), NFT_DATA_VALUE, len);
    if (err < 0)
    return err;
    return 0;
    }
    EXPORT_SYMBOL_GPL(nft_fib_init);
#[no_mangle]
pub unsafe extern "C" fn nft_fib_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> c_int {
    int nft_fib_dump(struct sk_buff *skb, const struct nft_expr *expr, bool reset)
    {
    const struct nft_fib *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_FIB_DREG, priv.dreg))
    return -1;
    if (nla_put_be32(skb, NFTA_FIB_RESULT, htonl(priv.result)))
    return -1;
    if (nla_put_be32(skb, NFTA_FIB_FLAGS, htonl(priv.flags)))
    return -1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(nft_fib_dump);
    void nft_fib_store_result(void *reg, const struct nft_fib *priv,
    const struct net_device *dev)
    {
    u32 *dreg = reg;
    int index;
    switch (priv.result) {
    case NFT_FIB_RESULT_OIF:
    index = dev ? dev.ifindex : 0;
    if (priv.flags & NFTA_FIB_F_PRESENT)
    nft_reg_store8(dreg, !!index);
    else
// dreg = index;
    break;
    case NFT_FIB_RESULT_OIFNAME:
    if (priv.flags & NFTA_FIB_F_PRESENT)
    nft_reg_store8(dreg, !!dev);
    else
    strscpy_pad(reg, dev ? dev.name : "", IFNAMSIZ);
    break;
    default:
    DEBUG_NET_WARN_ON_ONCE(1);
// dreg = 0;
    break;
    }
    }
    EXPORT_SYMBOL_GPL(nft_fib_store_result);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Query routing table from nftables");
    MODULE_AUTHOR("Florian Westphal <fw@strlen.de>");
