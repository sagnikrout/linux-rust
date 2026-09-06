//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_tcpmss.c
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
// Kernel module to match TCP MSS values.
// Copyright (C) 2000 Marc Boucher <marc@mbsi.ca>
// Portions (C) 2005 by Harald Welte <laforge@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marc Boucher <marc@mbsi.ca>");
    MODULE_DESCRIPTION("Xtables: TCP MSS match");
    MODULE_ALIAS("ipt_tcpmss");
    MODULE_ALIAS("ip6t_tcpmss");
    static bool
    tcpmss_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_tcpmss_match_info *info = par.matchinfo;
    const struct tcphdr *th;
    struct tcphdr _tcph;
// tcp.doff is only 4 bits, ie. max 15 * 4 bytes
    const u_int8_t *op;
    u8 _opt[15 * 4 - sizeof(_tcph)];
    unsigned int i, optlen;
// this is fine for IPv6 as xt_tcpmss enforces -p tcp
    if (par.fragoff)
    return false;
// If we don't have the whole header, drop packet.
    th = skb_header_pointer(skb, par.thoff, sizeof(_tcph), &_tcph);
    if (th == core::ptr::null_mut())
    goto dropit;
// Malformed.
    if (th.doff*4 < sizeof(*th))
    goto dropit;
    optlen = th.doff*4 - sizeof(*th);
    if (!optlen)
    goto out;
// Truncated options.
    op = skb_header_pointer(skb, par.thoff + sizeof(*th), optlen, _opt);
    if (op == core::ptr::null_mut())
    goto dropit;
    for (i = 0; i < optlen; ) {
    if (op[i] == TCPOPT_MSS
    && (optlen - i) >= TCPOLEN_MSS
    && op[i+1] == TCPOLEN_MSS) {
    u_int16_t mssval;
    mssval = (op[i+2] << 8) | op[i+3];
    return (mssval >= info.mss_min &&
    mssval <= info.mss_max) ^ info.invert;
    }
    if (op[i] < 2 || i == optlen - 1)
    i++;
    else
    i += op[i+1] ? : 1;
    }
    out:
    return info.invert;
    dropit:
    par.hotdrop = true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tcpmss_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int tcpmss_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_tcpmss_match_info *info = par.matchinfo;
    if (info.mss_min > info.mss_max)
    return -EINVAL;
    if (info.invert > 1)
    return -EINVAL;
    return 0;
    }
    static struct xt_match tcpmss_mt_reg[] __read_mostly = {
    {
    .name		= "tcpmss",
    .family		= NFPROTO_IPV4,
    .checkentry	= tcpmss_mt_check,
    .match		= tcpmss_mt,
    .matchsize	= sizeof(struct xt_tcpmss_match_info),
    .proto		= IPPROTO_TCP,
    .me		= THIS_MODULE,
    },
    {
    .name		= "tcpmss",
    .family		= NFPROTO_IPV6,
    .checkentry	= tcpmss_mt_check,
    .match		= tcpmss_mt,
    .matchsize	= sizeof(struct xt_tcpmss_match_info),
    .proto		= IPPROTO_TCP,
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn tcpmss_mt_init() -> int __init {
    static int __init tcpmss_mt_init(void)
    {
    return xt_register_matches(tcpmss_mt_reg, ARRAY_SIZE(tcpmss_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn tcpmss_mt_exit() -> void __exit {
    static void __exit tcpmss_mt_exit(void)
    {
    xt_unregister_matches(tcpmss_mt_reg, ARRAY_SIZE(tcpmss_mt_reg));
    }
    module_init(tcpmss_mt_init);
    module_exit(tcpmss_mt_exit);
