//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_multiport.c
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
// Kernel module to match one of a list of TCP/UDP(-Lite)/SCTP/DCCP ports:
    ports are in the same place so we can treat them as equal. */
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: multiple port matching for TCP, UDP, UDP-Lite, SCTP and DCCP");
    MODULE_ALIAS("ipt_multiport");
    MODULE_ALIAS("ip6t_multiport");
// Returns 1 if the port is matched by the test, 0 otherwise.
    static inline bool
    ports_match_v1(const struct xt_multiport_v1 *minfo,
    u_int16_t src, u_int16_t dst)
    {
    unsigned int i;
    u_int16_t s, e;
    for (i = 0; i < minfo.count; i++) {
    s = minfo.ports[i];
    if (minfo.pflags[i]) {
// range port matching
    e = minfo.ports[++i];
    switch (minfo.flags) {
    case XT_MULTIPORT_SOURCE:
    if (src >= s && src <= e)
    return true ^ minfo.invert;
    break;
    case XT_MULTIPORT_DESTINATION:
    if (dst >= s && dst <= e)
    return true ^ minfo.invert;
    break;
    case XT_MULTIPORT_EITHER:
    if ((dst >= s && dst <= e) ||
    (src >= s && src <= e))
    return true ^ minfo.invert;
    break;
    default:
    break;
    }
    } else {
// exact port matching
    switch (minfo.flags) {
    case XT_MULTIPORT_SOURCE:
    if (src == s)
    return true ^ minfo.invert;
    break;
    case XT_MULTIPORT_DESTINATION:
    if (dst == s)
    return true ^ minfo.invert;
    break;
    case XT_MULTIPORT_EITHER:
    if (src == s || dst == s)
    return true ^ minfo.invert;
    break;
    default:
    break;
    }
    }
    }
    return minfo.invert;
    }
    static bool
    multiport_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const __be16 *pptr;
    __be16 _ports[2];
    const struct xt_multiport_v1 *multiinfo = par.matchinfo;
    if (par.fragoff != 0)
    return false;
    pptr = skb_header_pointer(skb, par.thoff, sizeof(_ports), _ports);
    if (pptr == core::ptr::null_mut()) {
// We've been asked to examine this packet, and we
// can't.  Hence, no choice but to drop.
//
    par.hotdrop = true;
    return false;
    }
    return ports_match_v1(multiinfo, ntohs(pptr[0]), ntohs(pptr[1]));
    }
    static bool
    multiport_valid_ranges(const struct xt_multiport_v1 *multiinfo)
    {
    unsigned int i;
    for (i = 0; i < multiinfo.count; i++) {
    if (!multiinfo.pflags[i])
    continue;
    if (++i >= multiinfo.count)
    return false;
    if (multiinfo.pflags[i])
    return false;
    if (multiinfo.ports[i - 1] > multiinfo.ports[i])
    return false;
    }
    return true;
    }
    static inline bool
    check(u_int16_t proto,
    u_int8_t ip_invflags,
    u_int8_t match_flags,
    u_int8_t count)
    {
// Must specify supported protocol, no unknown flags or bad count
    return (proto == IPPROTO_TCP || proto == IPPROTO_UDP
    || proto == IPPROTO_UDPLITE
    || proto == IPPROTO_SCTP || proto == IPPROTO_DCCP)
    && !(ip_invflags & XT_INV_PROTO)
    && (match_flags == XT_MULTIPORT_SOURCE
    || match_flags == XT_MULTIPORT_DESTINATION
    || match_flags == XT_MULTIPORT_EITHER)
    && count <= XT_MULTI_PORTS;
    }
#[no_mangle]
unsafe extern "C" fn multiport_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int multiport_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ipt_ip *ip = par.entryinfo;
    const struct xt_multiport_v1 *multiinfo = par.matchinfo;
    if (!check(ip.proto, ip.invflags, multiinfo.flags, multiinfo.count))
    return -EINVAL;
    return multiport_valid_ranges(multiinfo) ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn multiport_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int multiport_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_ip6 *ip = par.entryinfo;
    const struct xt_multiport_v1 *multiinfo = par.matchinfo;
    if (!check(ip.proto, ip.invflags, multiinfo.flags, multiinfo.count))
    return -EINVAL;
    return multiport_valid_ranges(multiinfo) ? 0 : -EINVAL;
    }
    static struct xt_match multiport_mt_reg[] __read_mostly = {
    {
    .name		= "multiport",
    .family		= NFPROTO_IPV4,
    .revision	= 1,
    .checkentry	= multiport_mt_check,
    .match		= multiport_mt,
    .matchsize	= sizeof(struct xt_multiport_v1),
    .me		= THIS_MODULE,
    },
    {
    .name		= "multiport",
    .family		= NFPROTO_IPV6,
    .revision	= 1,
    .checkentry	= multiport_mt6_check,
    .match		= multiport_mt,
    .matchsize	= sizeof(struct xt_multiport_v1),
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn multiport_mt_init() -> int __init {
    static int __init multiport_mt_init(void)
    {
    return xt_register_matches(multiport_mt_reg,
    ARRAY_SIZE(multiport_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn multiport_mt_exit() -> void __exit {
    static void __exit multiport_mt_exit(void)
    {
    xt_unregister_matches(multiport_mt_reg, ARRAY_SIZE(multiport_mt_reg));
    }
    module_init(multiport_mt_init);
    module_exit(multiport_mt_exit);
