//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_cmp.c
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
// Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
//
// Development of this code funded by Astaro AG (http://www.astaro.com/)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_cmp_expr {
    pub data: nft_data,
    pub sreg: u8,
    pub len: u8,
    pub op:8: enum nft_cmp_ops,
}

    void nft_cmp_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_cmp_expr *priv = nft_expr_priv(expr);
    int d;
    d = memcmp(&regs.data[priv.sreg], &priv.data, priv.len);
    switch (priv.op) {
    case NFT_CMP_EQ:
    if (d != 0)
    goto mismatch;
    break;
    case NFT_CMP_NEQ:
    if (d == 0)
    goto mismatch;
    break;
    case NFT_CMP_LT:
    if (d == 0)
    goto mismatch;
    fallthrough;
    case NFT_CMP_LTE:
    if (d > 0)
    goto mismatch;
    break;
    case NFT_CMP_GT:
    if (d == 0)
    goto mismatch;
    fallthrough;
    case NFT_CMP_GTE:
    if (d < 0)
    goto mismatch;
    break;
    }
    return;
    mismatch:
    regs.verdict.code = NFT_BREAK;
    }
    static const struct nla_policy nft_cmp_policy[NFTA_CMP_MAX + 1] = {
    [NFTA_CMP_SREG]		= NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    [NFTA_CMP_OP]		= { .type = NLA_U32 },
    [NFTA_CMP_DATA]		= { .type = NLA_NESTED },
    };
    static int nft_cmp_init(const struct nft_ctx *ctx, const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_cmp_expr *priv = nft_expr_priv(expr);
    struct nft_data_desc desc = {
    .type	= NFT_DATA_VALUE,
    .size	= sizeof(priv.data),
    };
    int err;
    err = nft_data_init(core::ptr::null_mut(), &priv.data, &desc, tb[NFTA_CMP_DATA]);
    if (err < 0)
    return err;
    err = nft_parse_register_load(ctx, tb[NFTA_CMP_SREG], &priv.sreg, desc.len);
    if (err < 0)
    return err;
    priv.op  = ntohl(nla_get_be32(tb[NFTA_CMP_OP]));
    priv.len = desc.len;
    return 0;
    }
    static int nft_cmp_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_cmp_expr *priv = nft_expr_priv(expr);
    if (nft_dump_register(skb, NFTA_CMP_SREG, priv.sreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_CMP_OP, htonl(priv.op)))
    goto nla_put_failure;
    if (nft_data_dump(skb, NFTA_CMP_DATA, &priv.data,
    NFT_DATA_VALUE, priv.len) < 0)
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    union nft_cmp_offload_data {
    u16	val16;
    u32	val32;
    u64	val64;
    };
    static void nft_payload_n2h(union nft_cmp_offload_data *data,
    const u8 *val, u32 len)
    {
    switch (len) {
    case 2:
    data.val16 = ntohs(*((__be16 *)val));
    break;
    case 4:
    data.val32 = ntohl(*((__be32 *)val));
    break;
    case 8:
    data.val64 = be64_to_cpu(*((__be64 *)val));
    break;
    default:
    WARN_ON_ONCE(1);
    break;
    }
    }
    static int __nft_cmp_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    const struct nft_cmp_expr *priv)
    {
    struct nft_offload_reg *reg = &ctx.regs[priv.sreg];
    union nft_cmp_offload_data _data, _datamask;
    u8 *mask = (u8 *)&flow.match.mask;
    u8 *key = (u8 *)&flow.match.key;
    u8 *data, *datamask;
    if (priv.op != NFT_CMP_EQ || priv.len > reg.len)
    return -EOPNOTSUPP;
    if (reg.flags & NFT_OFFLOAD_F_NETWORK2HOST) {
    nft_payload_n2h(&_data, (u8 *)&priv.data, reg.len);
    nft_payload_n2h(&_datamask, (u8 *)&reg.mask, reg.len);
    data = (u8 *)&_data;
    datamask = (u8 *)&_datamask;
    } else {
    data = (u8 *)&priv.data;
    datamask = (u8 *)&reg.mask;
    }
    memcpy(key + reg.offset, data, reg.len);
    memcpy(mask + reg.offset, datamask, reg.len);
    flow.match.dissector.used_keys |= BIT_ULL(reg.key);
    flow.match.dissector.offset[reg.key] = reg.base_offset;
    if (reg.key == FLOW_DISSECTOR_KEY_META &&
    reg.offset == offsetof(struct nft_flow_key, meta.ingress_iftype) &&
    nft_reg_load16(priv.data.data) != ARPHRD_ETHER)
    return -EOPNOTSUPP;
    nft_offload_update_dependency(ctx, &priv.data, reg.len);
    return 0;
    }
    static int nft_cmp_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    const struct nft_expr *expr)
    {
    const struct nft_cmp_expr *priv = nft_expr_priv(expr);
    return __nft_cmp_offload(ctx, flow, priv);
    }
    static const struct nft_expr_ops nft_cmp_ops = {
    .type		= &nft_cmp_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_cmp_expr)),
    .eval		= nft_cmp_eval,
    .init		= nft_cmp_init,
    .dump		= nft_cmp_dump,
    .offload	= nft_cmp_offload,
    };
// Calculate the mask for the nft_cmp_fast expression. On big endian the
// mask needs to include the *upper* bytes when interpreting that data as
// something smaller than the full u32, therefore a cpu_to_le32 is done.
//
#[no_mangle]
unsafe extern "C" fn nft_cmp_fast_mask(len: c_uint) -> u32 {
    static u32 nft_cmp_fast_mask(unsigned int len)
    {
    __le32 mask = cpu_to_le32(~0U >> (sizeof_field(struct nft_cmp_fast_expr,
    data) * BITS_PER_BYTE - len));
    return ( u32)mask;
    }
    static int nft_cmp_fast_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_cmp_fast_expr *priv = nft_expr_priv(expr);
    struct nft_data data;
    struct nft_data_desc desc = {
    .type	= NFT_DATA_VALUE,
    .size	= sizeof(data),
    };
    int err;
    err = nft_data_init(core::ptr::null_mut(), &data, &desc, tb[NFTA_CMP_DATA]);
    if (err < 0)
    return err;
    err = nft_parse_register_load(ctx, tb[NFTA_CMP_SREG], &priv.sreg, desc.len);
    if (err < 0)
    return err;
    desc.len *= BITS_PER_BYTE;
    priv.mask = nft_cmp_fast_mask(desc.len);
    priv.data = data.data[0] & priv.mask;
    priv.len  = desc.len;
    priv.inv  = ntohl(nla_get_be32(tb[NFTA_CMP_OP])) != NFT_CMP_EQ;
    return 0;
    }
    static int nft_cmp_fast_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    const struct nft_expr *expr)
    {
    const struct nft_cmp_fast_expr *priv = nft_expr_priv(expr);
    struct nft_cmp_expr cmp = {
    .data	= {
    .data	= {
    [0] = priv.data,
    },
    },
    .sreg	= priv.sreg,
    .len	= priv.len / BITS_PER_BYTE,
    .op	= priv.inv ? NFT_CMP_NEQ : NFT_CMP_EQ,
    };
    return __nft_cmp_offload(ctx, flow, &cmp);
    }
    static int nft_cmp_fast_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_cmp_fast_expr *priv = nft_expr_priv(expr);
    let mut op: enum nft_cmp_ops = priv.inv ? NFT_CMP_NEQ : NFT_CMP_EQ;
    struct nft_data data;
    if (nft_dump_register(skb, NFTA_CMP_SREG, priv.sreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_CMP_OP, htonl(op)))
    goto nla_put_failure;
    data.data[0] = priv.data;
    if (nft_data_dump(skb, NFTA_CMP_DATA, &data,
    NFT_DATA_VALUE, priv.len / BITS_PER_BYTE) < 0)
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    const struct nft_expr_ops nft_cmp_fast_ops = {
    .type		= &nft_cmp_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_cmp_fast_expr)),
    .eval		= core::ptr::null_mut(),	/* inlined */
    .init		= nft_cmp_fast_init,
    .dump		= nft_cmp_fast_dump,
    .offload	= nft_cmp_fast_offload,
    };
#[no_mangle]
unsafe extern "C" fn nft_cmp_mask(bitlen: u32) -> u32 {
    static u32 nft_cmp_mask(u32 bitlen)
    {
    return ( u32)cpu_to_le32(~0U >> (sizeof(u32) * BITS_PER_BYTE - bitlen));
    }
#[no_mangle]
unsafe extern "C" fn nft_cmp16_fast_mask(data: *mut nft_data, bitlen: c_uint) {
    static void nft_cmp16_fast_mask(struct nft_data *data, unsigned int bitlen)
    {
    let mut len: c_int = bitlen / BITS_PER_BYTE;
    int i, words = len / sizeof(u32);
    for (i = 0; i < words; i++) {
    data.data[i] = 0xffffffff;
    bitlen -= sizeof(u32) * BITS_PER_BYTE;
    }
    if (len % sizeof(u32))
    data.data[i++] = nft_cmp_mask(bitlen);
    for (; i < 4; i++)
    data.data[i] = 0;
    }
    static int nft_cmp16_fast_init(const struct nft_ctx *ctx,
    const struct nft_expr *expr,
    const struct nlattr * const tb[])
    {
    struct nft_cmp16_fast_expr *priv = nft_expr_priv(expr);
    struct nft_data_desc desc = {
    .type	= NFT_DATA_VALUE,
    .size	= sizeof(priv.data),
    };
    int err;
    err = nft_data_init(core::ptr::null_mut(), &priv.data, &desc, tb[NFTA_CMP_DATA]);
    if (err < 0)
    return err;
    err = nft_parse_register_load(ctx, tb[NFTA_CMP_SREG], &priv.sreg, desc.len);
    if (err < 0)
    return err;
    nft_cmp16_fast_mask(&priv.mask, desc.len * BITS_PER_BYTE);
    priv.inv = ntohl(nla_get_be32(tb[NFTA_CMP_OP])) != NFT_CMP_EQ;
    priv.len = desc.len;
    return 0;
    }
    static int nft_cmp16_fast_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    const struct nft_expr *expr)
    {
    const struct nft_cmp16_fast_expr *priv = nft_expr_priv(expr);
    struct nft_cmp_expr cmp = {
    .data	= priv.data,
    .sreg	= priv.sreg,
    .len	= priv.len,
    .op	= priv.inv ? NFT_CMP_NEQ : NFT_CMP_EQ,
    };
    return __nft_cmp_offload(ctx, flow, &cmp);
    }
    static int nft_cmp16_fast_dump(struct sk_buff *skb,
    const struct nft_expr *expr, bool reset)
    {
    const struct nft_cmp16_fast_expr *priv = nft_expr_priv(expr);
    let mut op: enum nft_cmp_ops = priv.inv ? NFT_CMP_NEQ : NFT_CMP_EQ;
    if (nft_dump_register(skb, NFTA_CMP_SREG, priv.sreg))
    goto nla_put_failure;
    if (nla_put_be32(skb, NFTA_CMP_OP, htonl(op)))
    goto nla_put_failure;
    if (nft_data_dump(skb, NFTA_CMP_DATA, &priv.data,
    NFT_DATA_VALUE, priv.len) < 0)
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -1;
    }
    const struct nft_expr_ops nft_cmp16_fast_ops = {
    .type		= &nft_cmp_type,
    .size		= NFT_EXPR_SIZE(sizeof(struct nft_cmp16_fast_expr)),
    .eval		= core::ptr::null_mut(),	/* inlined */
    .init		= nft_cmp16_fast_init,
    .dump		= nft_cmp16_fast_dump,
    .offload	= nft_cmp16_fast_offload,
    };
    static const struct nft_expr_ops *
    nft_cmp_select_ops(const struct nft_ctx *ctx, const struct nlattr * const tb[])
    {
    struct nft_data data;
    struct nft_data_desc desc = {
    .type	= NFT_DATA_VALUE,
    .size	= sizeof(data),
    };
    enum nft_cmp_ops op;
    u8 sreg;
    int err;
    if (tb[NFTA_CMP_SREG] == core::ptr::null_mut() ||
    tb[NFTA_CMP_OP] == core::ptr::null_mut() ||
    tb[NFTA_CMP_DATA] == core::ptr::null_mut())
    return ERR_PTR(-EINVAL);
    op = ntohl(nla_get_be32(tb[NFTA_CMP_OP]));
    switch (op) {
    case NFT_CMP_EQ:
    case NFT_CMP_NEQ:
    case NFT_CMP_LT:
    case NFT_CMP_LTE:
    case NFT_CMP_GT:
    case NFT_CMP_GTE:
    break;
    default:
    return ERR_PTR(-EINVAL);
    }
    err = nft_data_init(core::ptr::null_mut(), &data, &desc, tb[NFTA_CMP_DATA]);
    if (err < 0)
    return ERR_PTR(err);
    sreg = ntohl(nla_get_be32(tb[NFTA_CMP_SREG]));
    if (op == NFT_CMP_EQ || op == NFT_CMP_NEQ) {
    if (desc.len <= sizeof(u32))
    return &nft_cmp_fast_ops;
    else if (desc.len <= sizeof(data) &&
    ((sreg >= NFT_REG_1 && sreg <= NFT_REG_4) ||
    (sreg >= NFT_REG32_00 && sreg <= NFT_REG32_12 && sreg % 2 == 0)))
    return &nft_cmp16_fast_ops;
    }
    return &nft_cmp_ops;
    }
    struct nft_expr_type nft_cmp_type __read_mostly = {
    .name		= "cmp",
    .select_ops	= nft_cmp_select_ops,
    .policy		= nft_cmp_policy,
    .maxattr	= NFTA_CMP_MAX,
    .owner		= THIS_MODULE,
    };
