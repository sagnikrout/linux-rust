//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_ct_fast.c
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

    void nft_ct_get_fast_eval(const struct nft_expr *expr,
    struct nft_regs *regs,
    const struct nft_pktinfo *pkt)
    {
    const struct nft_ct *priv = nft_expr_priv(expr);
    u32 *dest = &regs.data[priv.dreg];
    enum ip_conntrack_info ctinfo;
    const struct nf_conn *ct;
    unsigned int state;
    ct = nf_ct_get(pkt.skb, &ctinfo);
    switch (priv.key) {
    case NFT_CT_STATE:
    if (ct)
    state = NF_CT_STATE_BIT(ctinfo);
#[no_mangle]
pub unsafe extern "C" fn if(IP_CT_UNTRACKED: ctinfo ==) -> else {
    else if (ctinfo == IP_CT_UNTRACKED)
    state = NF_CT_STATE_UNTRACKED_BIT;
    else
    state = NF_CT_STATE_INVALID_BIT;
// dest = state;
    return;
    default:
    break;
    }
    if (!ct || nf_ct_is_template(ct)) {
    regs.verdict.code = NFT_BREAK;
    return;
    }
    switch (priv.key) {
    case NFT_CT_DIRECTION:
    nft_reg_store8(dest, CTINFO2DIR(ctinfo));
    return;
    case NFT_CT_STATUS:
// dest = ct->status;
    return;

    case NFT_CT_MARK:
// dest = ct->mark;
    return;

    case NFT_CT_SECMARK:
// dest = ct->secmark;
    return;

    default:
    DEBUG_NET_WARN_ON_ONCE(1);
    regs.verdict.code = NFT_BREAK;
    break;
    }
    }
    EXPORT_SYMBOL_GPL(nft_ct_get_fast_eval);
