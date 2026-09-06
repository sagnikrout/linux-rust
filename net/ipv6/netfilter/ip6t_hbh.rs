//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_hbh.c
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
// Kernel module to match Hop-by-Hop and Destination parameters.
// (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: IPv6 Hop-By-Hop and Destination Header match");
    MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");
    MODULE_ALIAS("ip6t_dst");
//
// (Type & 0xC0) >> 6
// 0	-> ignorable
// 1	-> must drop the packet
// 2	-> send ICMP PARM PROB regardless and drop packet
// 3	-> Send ICMP if not a multicast address and drop packet
// (Type & 0x20) >> 5
// 0	-> invariant
// 1	-> can change the routing
// (Type & 0x1F) Type
// 0	-> Pad1 (only 1 byte!)
// 1	-> PadN LENGTH info (total length = length + 2)
// C0 | 2	-> JUMBO 4 x x x x ( xxxx > 64k )
// 5	-> RTALERT 2 x x
//
    static struct xt_match hbh_mt6_reg[] __read_mostly;
    static bool
    hbh_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct ipv6_opt_hdr _optsh;
    const struct ipv6_opt_hdr *oh;
    const struct ip6t_opts *optinfo = par.matchinfo;
    unsigned int temp;
    let mut ptr: c_uint = 0;
    let mut hdrlen: c_uint = 0;
    let mut ret: bool = false;
    u8 _opttype;
    u8 _optlen;
    const u_int8_t *tp = core::ptr::null_mut();
    const u_int8_t *lp = core::ptr::null_mut();
    unsigned int optlen;
    int err;
    err = ipv6_find_hdr(skb, &ptr,
    (par.match == &hbh_mt6_reg[0]) ?
    NEXTHDR_HOP : NEXTHDR_DEST, core::ptr::null_mut(), core::ptr::null_mut());
    if (err < 0) {
    if (err != -ENOENT)
    par.hotdrop = true;
    return false;
    }
    oh = skb_header_pointer(skb, ptr, sizeof(_optsh), &_optsh);
    if (oh == core::ptr::null_mut()) {
    par.hotdrop = true;
    return false;
    }
    hdrlen = ipv6_optlen(oh);
    if (skb.len - ptr < hdrlen) {
// Packet smaller than it's length field
    par.hotdrop = true;
    return false;
    }
    ret = (!(optinfo.flags & IP6T_OPTS_LEN) ||
    ((optinfo.hdrlen == hdrlen) ^
    !!(optinfo.invflags & IP6T_OPTS_INV_LEN)));
    ptr += 2;
    hdrlen -= 2;
    if (!(optinfo.flags & IP6T_OPTS_OPTS)) {
    return ret;
    } else {
    for (temp = 0; temp < optinfo.optsnr; temp++) {
// type field exists ?
    if (hdrlen < 1)
    break;
    tp = skb_header_pointer(skb, ptr, sizeof(_opttype),
    &_opttype);
    if (tp == core::ptr::null_mut())
    break;
// Type check
    if (*tp != (optinfo.opts[temp] & 0xFF00) >> 8)
    return false;
// Length check
    if (*tp) {
    u16 spec_len;
// length field exists ?
    if (hdrlen < 2)
    break;
    lp = skb_header_pointer(skb, ptr + 1,
    sizeof(_optlen),
    &_optlen);
    if (lp == core::ptr::null_mut())
    break;
    spec_len = optinfo.opts[temp] & 0x00FF;
    if (spec_len != 0x00FF && spec_len != *lp)
    return false;
    optlen = *lp + 2;
    } else {
    optlen = 1;
    }
    if ((ptr > skb.len - optlen || hdrlen < optlen) &&
    temp < optinfo.optsnr - 1)
    break;
    ptr += optlen;
    hdrlen -= optlen;
    }
    if (temp == optinfo.optsnr)
    return ret;
    else
    return false;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn hbh_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int hbh_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_opts *optsinfo = par.matchinfo;
    if (optsinfo.invflags & ~IP6T_OPTS_INV_MASK) {
    pr_info_ratelimited("unknown flags %X\n", optsinfo.invflags);
    return -EINVAL;
    }
    if (optsinfo.optsnr > IP6T_OPTS_OPTSNR) {
    pr_info_ratelimited("too many supported opts specified\n");
    return -EINVAL;
    }
    if (optsinfo.flags & IP6T_OPTS_NSTRICT) {
    pr_info_ratelimited("Not strict - not implemented\n");
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match hbh_mt6_reg[] __read_mostly = {
    {
// Note, hbh_mt6 relies on the order of hbh_mt6_reg
    .name		= "hbh",
    .family		= NFPROTO_IPV6,
    .match		= hbh_mt6,
    .matchsize	= sizeof(struct ip6t_opts),
    .checkentry	= hbh_mt6_check,
    .me		= THIS_MODULE,
    },
    {
    .name		= "dst",
    .family		= NFPROTO_IPV6,
    .match		= hbh_mt6,
    .matchsize	= sizeof(struct ip6t_opts),
    .checkentry	= hbh_mt6_check,
    .me		= THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn hbh_mt6_init() -> int __init {
    static int __init hbh_mt6_init(void)
    {
    return xt_register_matches(hbh_mt6_reg, ARRAY_SIZE(hbh_mt6_reg));
    }
#[no_mangle]
unsafe extern "C" fn hbh_mt6_exit() -> void __exit {
    static void __exit hbh_mt6_exit(void)
    {
    xt_unregister_matches(hbh_mt6_reg, ARRAY_SIZE(hbh_mt6_reg));
    }
    module_init(hbh_mt6_init);
    module_exit(hbh_mt6_exit);
