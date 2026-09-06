//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_dccp.c
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
// iptables module for DCCP protocol header matching
//
// (C) 2005 by Harald Welte <laforge@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: DCCP protocol packet match");
    MODULE_ALIAS("ipt_dccp");
    MODULE_ALIAS("ip6t_dccp");

    || (!!((invflag) & (option)) ^ (cond)))
    static unsigned char *dccp_optbuf;
    static DEFINE_SPINLOCK(dccp_buflock);
    static inline bool
    dccp_find_option(u_int8_t option,
    const struct sk_buff *skb,
    unsigned int protoff,
    const struct dccp_hdr *dh,
    bool *hotdrop)
    {
// tcp.doff is only 4 bits, ie. max 15 * 4 bytes
    const unsigned char *op;
    let mut optoff: c_uint = __dccp_hdr_len(dh);
    let mut optlen: c_uint = dh.dccph_doff*4 - __dccp_hdr_len(dh);
    unsigned int i;
    if (dh.dccph_doff * 4 < __dccp_hdr_len(dh))
    goto invalid;
    if (!optlen)
    return false;
    spin_lock_bh(&dccp_buflock);
    op = skb_header_pointer(skb, protoff + optoff, optlen, dccp_optbuf);
    if (op == core::ptr::null_mut()) {
// If we don't have the whole header, drop packet.
    goto partial;
    }
    for (i = 0; i < optlen; ) {
    if (op[i] == option) {
    spin_unlock_bh(&dccp_buflock);
    return true;
    }
    if (op[i] < 2 || i == optlen - 1)
    i++;
    else
    i += op[i + 1] ? : 1;
    }
    spin_unlock_bh(&dccp_buflock);
    return false;
    partial:
    spin_unlock_bh(&dccp_buflock);
    invalid:
// hotdrop = true;
    return false;
    }
    static inline bool
    match_types(const struct dccp_hdr *dh, u_int16_t typemask)
    {
    return typemask & (1 << dh.dccph_type);
    }
    static inline bool
    match_option(u_int8_t option, const struct sk_buff *skb, unsigned int protoff,
    const struct dccp_hdr *dh, bool *hotdrop)
    {
    return dccp_find_option(option, skb, protoff, dh, hotdrop);
    }
    static bool
    dccp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_dccp_info *info = par.matchinfo;
    const struct dccp_hdr *dh;
    struct dccp_hdr _dh;
    if (par.fragoff != 0)
    return false;
    dh = skb_header_pointer(skb, par.thoff, sizeof(_dh), &_dh);
    if (dh == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    return  DCCHECK(ntohs(dh.dccph_sport) >= info.spts[0]
    && ntohs(dh.dccph_sport) <= info.spts[1],
    XT_DCCP_SRC_PORTS, info.flags, info.invflags)
    && DCCHECK(ntohs(dh.dccph_dport) >= info.dpts[0]
    && ntohs(dh.dccph_dport) <= info.dpts[1],
    XT_DCCP_DEST_PORTS, info.flags, info.invflags)
    && DCCHECK(match_types(dh, info.typemask),
    XT_DCCP_TYPE, info.flags, info.invflags)
    && DCCHECK(match_option(info.option, skb, par.thoff, dh,
    &par.hotdrop),
    XT_DCCP_OPTION, info.flags, info.invflags);
    }
#[no_mangle]
unsafe extern "C" fn dccp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int dccp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_dccp_info *info = par.matchinfo;
    if (info.flags & ~XT_DCCP_VALID_FLAGS)
    return -EINVAL;
    if (info.invflags & ~XT_DCCP_VALID_FLAGS)
    return -EINVAL;
    if (info.invflags & ~info.flags)
    return -EINVAL;
    return 0;
    }
    static struct xt_match dccp_mt_reg[] __read_mostly = {
    {
    .name 		= "dccp",
    .family		= NFPROTO_IPV4,
    .checkentry	= dccp_mt_check,
    .match		= dccp_mt,
    .matchsize	= sizeof(struct xt_dccp_info),
    .proto		= IPPROTO_DCCP,
    .me 		= THIS_MODULE,
    },
    {
    .name 		= "dccp",
    .family		= NFPROTO_IPV6,
    .checkentry	= dccp_mt_check,
    .match		= dccp_mt,
    .matchsize	= sizeof(struct xt_dccp_info),
    .proto		= IPPROTO_DCCP,
    .me 		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn dccp_mt_init() -> int __init {
    static int __init dccp_mt_init(void)
    {
    int ret;
    pr_warn_once("The DCCP match is deprecated and scheduled to be removed in 2027.\n"
    "Please contact the netfilter-devel mailing list or update your iptables rules\n");
// doff is 8 bits, so the maximum option size is (4*256).  Don't put
// this in BSS since DaveM is worried about locked TLB's for kernel
// BSS.
    dccp_optbuf = kmalloc(256 * 4, GFP_KERNEL);
    if (!dccp_optbuf)
    return -ENOMEM;
    ret = xt_register_matches(dccp_mt_reg, ARRAY_SIZE(dccp_mt_reg));
    if (ret)
    goto out_kfree;
    return ret;
    out_kfree:
    kfree(dccp_optbuf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dccp_mt_exit() -> void __exit {
    static void __exit dccp_mt_exit(void)
    {
    xt_unregister_matches(dccp_mt_reg, ARRAY_SIZE(dccp_mt_reg));
    kfree(dccp_optbuf);
    }
    module_init(dccp_mt_init);
    module_exit(dccp_mt_exit);
