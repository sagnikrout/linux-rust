//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_rt.c
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
// Kernel module to match ROUTING parameters.
// (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: IPv6 Routing Header match");
    MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");
// Returns 1 if the id is matched by the range, 0 otherwise
    static inline bool
    segsleft_match(u_int32_t min, u_int32_t max, u_int32_t id, bool invert)
    {
    return (id >= min && id <= max) ^ invert;
    }
#[no_mangle]
unsafe extern "C" fn rt_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool rt_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct ipv6_rt_hdr _route;
    const struct ipv6_rt_hdr *rh;
    const struct ip6t_rt *rtinfo = par.matchinfo;
    unsigned int temp;
    let mut ptr: c_uint = 0;
    let mut hdrlen: c_uint = 0;
    let mut ret: bool = false;
    struct in6_addr _addr;
    const struct in6_addr *ap;
    int err;
    err = ipv6_find_hdr(skb, &ptr, NEXTHDR_ROUTING, core::ptr::null_mut(), core::ptr::null_mut());
    if (err < 0) {
    if (err != -ENOENT)
    par.hotdrop = true;
    return false;
    }
    rh = skb_header_pointer(skb, ptr, sizeof(_route), &_route);
    if (rh == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    hdrlen = ipv6_optlen(rh);
    if (skb.len - ptr < hdrlen) {
// Packet smaller than its length field
    par.hotdrop = true;
    return false;
    }
    ret = (segsleft_match(rtinfo.segsleft[0], rtinfo.segsleft[1],
    rh.segments_left,
    !!(rtinfo.invflags & IP6T_RT_INV_SGS))) &&
    (!(rtinfo.flags & IP6T_RT_LEN) ||
    ((rtinfo.hdrlen == hdrlen) ^
    !!(rtinfo.invflags & IP6T_RT_INV_LEN))) &&
    (!(rtinfo.flags & IP6T_RT_TYP) ||
    ((rtinfo.rt_type == rh.type) ^
    !!(rtinfo.invflags & IP6T_RT_INV_TYP)));
    if (ret && (rtinfo.flags & IP6T_RT_RES)) {
    const u_int32_t *rp;
    u_int32_t _reserved;
    rp = skb_header_pointer(skb,
    ptr + offsetof(struct rt0_hdr,
    reserved),
    sizeof(_reserved),
    &_reserved);
    if (!rp) {
    par.hotdrop = true;
    return false;
    }
    ret = (*rp == 0);
    }
    if (!(rtinfo.flags & IP6T_RT_FST)) {
    return ret;
    } else if (rtinfo.flags & IP6T_RT_FST_NSTRICT) {
    if (rtinfo.addrnr > (unsigned int)((hdrlen - 8) / 16)) {
    return false;
    } else {
    let mut i: c_uint = 0;
    for (temp = 0;
    temp < (unsigned int)((hdrlen - 8) / 16);
    temp++) {
    ap = skb_header_pointer(skb,
    ptr
    + sizeof(struct rt0_hdr)
    + temp * sizeof(_addr),
    sizeof(_addr),
    &_addr);
    if (ap == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    if (ipv6_addr_equal(ap, &rtinfo.addrs[i]))
    i++;
    if (i == rtinfo.addrnr)
    break;
    }
    if (i == rtinfo.addrnr)
    return ret;
    else
    return false;
    }
    } else {
    if (rtinfo.addrnr > (unsigned int)((hdrlen - 8) / 16)) {
    return false;
    } else {
    for (temp = 0; temp < rtinfo.addrnr; temp++) {
    ap = skb_header_pointer(skb,
    ptr
    + sizeof(struct rt0_hdr)
    + temp * sizeof(_addr),
    sizeof(_addr),
    &_addr);
    if (ap == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    if (!ipv6_addr_equal(ap, &rtinfo.addrs[temp]))
    break;
    }
    if (temp == rtinfo.addrnr &&
    temp == (unsigned int)((hdrlen - 8) / 16))
    return ret;
    else
    return false;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rt_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int rt_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_rt *rtinfo = par.matchinfo;
    if (rtinfo.invflags & ~IP6T_RT_INV_MASK) {
    pr_info_ratelimited("unknown flags %X\n", rtinfo.invflags);
    return -EINVAL;
    }
    if (rtinfo.addrnr > IP6T_RT_HOPS) {
    pr_info_ratelimited("too many addresses specified\n");
    return -EINVAL;
    }
    if ((rtinfo.flags & (IP6T_RT_RES | IP6T_RT_FST_MASK)) &&
    (!(rtinfo.flags & IP6T_RT_TYP) ||
    (rtinfo.rt_type != 0) ||
    (rtinfo.invflags & IP6T_RT_INV_TYP))) {
    pr_info_ratelimited("`--rt-type 0' required before `--rt-0-*'\n");
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match rt_mt6_reg __read_mostly = {
    .name		= "rt",
    .family		= NFPROTO_IPV6,
    .match		= rt_mt6,
    .matchsize	= sizeof(struct ip6t_rt),
    .checkentry	= rt_mt6_check,
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn rt_mt6_init() -> int __init {
    static int __init rt_mt6_init(void)
    {
    return xt_register_match(&rt_mt6_reg);
    }
#[no_mangle]
unsafe extern "C" fn rt_mt6_exit() -> void __exit {
    static void __exit rt_mt6_exit(void)
    {
    xt_unregister_match(&rt_mt6_reg);
    }
    module_init(rt_mt6_init);
    module_exit(rt_mt6_exit);
