//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_policy.c
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
// IP tables module for matching IPsec policy
//
// Copyright (c) 2004,2005 Patrick McHardy, <kaber@trash.net>
//

    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_DESCRIPTION("Xtables: IPsec policy match");
    MODULE_LICENSE("GPL");
    static inline bool
    xt_addr_cmp(const union nf_inet_addr *a1, const union nf_inet_addr *m,
    const union nf_inet_addr *a2, unsigned short family)
    {
    switch (family) {
    case NFPROTO_IPV4:
    return ((a1.ip ^ a2.ip) & m.ip) == 0;
    case NFPROTO_IPV6:
    return ipv6_masked_addr_cmp(&a1.in6, &m.in6, &a2.in6) == 0;
    }
    return false;
    }
    static bool
    match_xfrm_state(const struct xfrm_state *x, const struct xt_policy_elem *e,
    unsigned short family)
    {

    (xt_addr_cmp(&e.x, &e.y, (const union nf_inet_addr *)(z), family) \
    ^ e.invert.x))

    return MATCH_ADDR(saddr, smask, &x.props.saddr) &&
    MATCH_ADDR(daddr, dmask, &x.id.daddr) &&
    MATCH(proto, x.id.proto) &&
    MATCH(mode, x.props.mode) &&
    MATCH(spi, x.id.spi) &&
    MATCH(reqid, x.props.reqid);
    }
    static int
    match_policy_in(const struct sk_buff *skb, const struct xt_policy_info *info,
    unsigned short family)
    {
    const struct xt_policy_elem *e;
    const struct sec_path *sp = skb_sec_path(skb);
    let mut strict: c_int = info.flags & XT_POLICY_MATCH_STRICT;
    int i, pos;
    if (sp == core::ptr::null_mut())
    return -1;
    if (strict && info.len != sp.len)
    return 0;
    for (i = sp.len - 1; i >= 0; i--) {
    pos = strict ? sp.len - i - 1 : 0;
    if (pos >= info.len)
    return 0;
    e = &info.pol[pos];
    if (match_xfrm_state(sp.xvec[i], e, family)) {
    if (!strict)
    return 1;
    } else if (strict)
    return 0;
    }
    return strict ? 1 : 0;
    }
    static int
    match_policy_out(const struct sk_buff *skb, const struct xt_policy_info *info,
    unsigned short family)
    {
    const struct xt_policy_elem *e;
    const struct dst_entry *dst = skb_dst(skb);
    let mut strict: c_int = info.flags & XT_POLICY_MATCH_STRICT;
    int i, pos;
    if (dst.xfrm == core::ptr::null_mut())
    return -1;
    for (i = 0; dst && dst.xfrm;
    dst = ((struct xfrm_dst *)dst).child, i++) {
    pos = strict ? i : 0;
    if (pos >= info.len)
    return 0;
    e = &info.pol[pos];
    if (match_xfrm_state(dst.xfrm, e, family)) {
    if (!strict)
    return 1;
    } else if (strict)
    return 0;
    }
    return strict ? i == info.len : 0;
    }
    static bool
    policy_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_policy_info *info = par.matchinfo;
    int ret;
    if (info.flags & XT_POLICY_MATCH_IN)
    ret = match_policy_in(skb, info, xt_family(par));
    else
    ret = match_policy_out(skb, info, xt_family(par));
    if (ret < 0)
    ret = info.flags & XT_POLICY_MATCH_NONE ? true : false;
#[no_mangle]
pub unsafe extern "C" fn if(XT_POLICY_MATCH_NONE: info->flags &) -> else {
    else if (info.flags & XT_POLICY_MATCH_NONE)
    ret = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn policy_mt_check_hooks(par: *const xt_mtchk_param) -> c_int {
    static int policy_mt_check_hooks(const struct xt_mtchk_param *par)
    {
    const struct xt_policy_info *info = par.matchinfo;
    const char *errmsg;
    if (par.hook_mask & ((1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN)) && info.flags & XT_POLICY_MATCH_OUT) {
    errmsg = "output policy not valid in PREROUTING and INPUT";
    goto err;
    }
    if (par.hook_mask & ((1 << NF_INET_POST_ROUTING) |
    (1 << NF_INET_LOCAL_OUT)) && info.flags & XT_POLICY_MATCH_IN) {
    errmsg = "input policy not valid in POSTROUTING and OUTPUT";
    goto err;
    }
    return 0;
    err:
    pr_info_ratelimited("%s\n", errmsg);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn policy_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int policy_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_policy_info *info = par.matchinfo;
    const char *errmsg = "neither incoming nor outgoing policy selected";
    if (!(info.flags & (XT_POLICY_MATCH_IN|XT_POLICY_MATCH_OUT)))
    goto err;
    if (info.len > XT_POLICY_MAX_ELEM) {
    errmsg = "too many policy elements";
    goto err;
    }
    return 0;
    err:
    pr_info_ratelimited("%s\n", errmsg);
    return -EINVAL;
    }
    static struct xt_match policy_mt_reg[] __read_mostly = {
    {
    .name		= "policy",
    .family		= NFPROTO_IPV4,
    .check_hooks	= policy_mt_check_hooks,
    .checkentry 	= policy_mt_check,
    .match		= policy_mt,
    .matchsize	= sizeof(struct xt_policy_info),
    .me		= THIS_MODULE,
    },
    {
    .name		= "policy",
    .family		= NFPROTO_IPV6,
    .check_hooks	= policy_mt_check_hooks,
    .checkentry	= policy_mt_check,
    .match		= policy_mt,
    .matchsize	= sizeof(struct xt_policy_info),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn policy_mt_init() -> int __init {
    static int __init policy_mt_init(void)
    {
    return xt_register_matches(policy_mt_reg, ARRAY_SIZE(policy_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn policy_mt_exit() -> void __exit {
    static void __exit policy_mt_exit(void)
    {
    xt_unregister_matches(policy_mt_reg, ARRAY_SIZE(policy_mt_reg));
    }
    module_init(policy_mt_init);
    module_exit(policy_mt_exit);
    MODULE_ALIAS("ipt_policy");
    MODULE_ALIAS("ip6t_policy");
