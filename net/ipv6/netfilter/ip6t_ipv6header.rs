//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_ipv6header.c
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
// ipv6header match - matches IPv6 packets based
    on whether they contain certain headers */
// Original idea: Brad Chapman
// Rewritten by: Andras Kis-Szabo <kisza@sch.bme.hu>
// (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: IPv6 header types match");
    MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");
    static bool
    ipv6header_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ip6t_ipv6header_info *info = par.matchinfo;
    unsigned int temp;
    int len;
    u8 nexthdr;
    unsigned int ptr;
// Make sure this isn't an evil packet
// type of the 1st exthdr
    nexthdr = ipv6_hdr(skb).nexthdr;
// pointer to the 1st exthdr
    ptr = sizeof(struct ipv6hdr);
// available length
    len = skb.len - ptr;
    temp = 0;
    while (nf_ip6_ext_hdr(nexthdr)) {
    const struct ipv6_opt_hdr *hp;
    struct ipv6_opt_hdr _hdr;
    int hdrlen;
// No more exthdr -> evaluate
    if (nexthdr == NEXTHDR_NONE) {
    temp |= MASK_NONE;
    break;
    }
// Is there enough space for the next ext header?
    if (len < (int)sizeof(struct ipv6_opt_hdr))
    return false;
// ESP -> evaluate
    if (nexthdr == NEXTHDR_ESP) {
    temp |= MASK_ESP;
    break;
    }
    hp = skb_header_pointer(skb, ptr, sizeof(_hdr), &_hdr);
    if (!hp) {
    par.hotdrop = true;
    return false;
    }
// Calculate the header length
    if (nexthdr == NEXTHDR_FRAGMENT)
    hdrlen = 8;
#[no_mangle]
pub unsafe extern "C" fn if(NEXTHDR_AUTH: nexthdr ==) -> else {
    else if (nexthdr == NEXTHDR_AUTH)
    hdrlen = ipv6_authlen(hp);
    else
    hdrlen = ipv6_optlen(hp);
// set the flag
    switch (nexthdr) {
    case NEXTHDR_HOP:
    temp |= MASK_HOPOPTS;
    break;
    case NEXTHDR_ROUTING:
    temp |= MASK_ROUTING;
    break;
    case NEXTHDR_FRAGMENT:
    temp |= MASK_FRAGMENT;
    break;
    case NEXTHDR_AUTH:
    temp |= MASK_AH;
    break;
    case NEXTHDR_DEST:
    temp |= MASK_DSTOPTS;
    break;
    default:
    return false;
    }
    nexthdr = hp.nexthdr;
    len -= hdrlen;
    ptr += hdrlen;
    if (ptr > skb.len)
    break;
    }
    if (nexthdr != NEXTHDR_NONE && nexthdr != NEXTHDR_ESP)
    temp |= MASK_PROTO;
    if (info.modeflag)
    return !((temp ^ info.matchflags ^ info.invflags)
    & info.matchflags);
    else {
    if (info.invflags)
    return temp != info.matchflags;
    else
    let mut temp: return = = info.matchflags;
    }
    }
#[no_mangle]
unsafe extern "C" fn ipv6header_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int ipv6header_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_ipv6header_info *info = par.matchinfo;
// invflags is 0 or 0xff in hard mode
    if ((!info.modeflag) && info.invflags != 0x00 &&
    info.invflags != 0xFF)
    return -EINVAL;
    return 0;
    }
    static struct xt_match ipv6header_mt6_reg __read_mostly = {
    .name		= "ipv6header",
    .family		= NFPROTO_IPV6,
    .match		= ipv6header_mt6,
    .matchsize	= sizeof(struct ip6t_ipv6header_info),
    .checkentry	= ipv6header_mt6_check,
    .destroy	= core::ptr::null_mut(),
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ipv6header_mt6_init() -> int __init {
    static int __init ipv6header_mt6_init(void)
    {
    return xt_register_match(&ipv6header_mt6_reg);
    }
#[no_mangle]
unsafe extern "C" fn ipv6header_mt6_exit() -> void __exit {
    static void __exit ipv6header_mt6_exit(void)
    {
    xt_unregister_match(&ipv6header_mt6_reg);
    }
    module_init(ipv6header_mt6_init);
    module_exit(ipv6header_mt6_exit);
