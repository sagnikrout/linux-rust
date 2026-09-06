//! Automatically rewritten from C to Rust
//! Source: net/ipv6/netfilter/ip6t_srh.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Kernel module to match Segment Routing Header (SRH) parameters.
// Author:
// Ahmed Abdelsalam <amsalam20@gmail.com>
//

// Test a struct->mt_invflags and a boolean for inequality

    ((boolean) ^ !!((ptr).mt_invflags & (flag)))
#[no_mangle]
unsafe extern "C" fn srh_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool srh_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ip6t_srh *srhinfo = par.matchinfo;
    struct ipv6_sr_hdr *srh;
    struct ipv6_sr_hdr _srh;
    int hdrlen, srhoff = 0;
    if (ipv6_find_hdr(skb, &srhoff, IPPROTO_ROUTING, core::ptr::null_mut(), core::ptr::null_mut()) < 0)
    return false;
    srh = skb_header_pointer(skb, srhoff, sizeof(_srh), &_srh);
    if (!srh)
    return false;
    hdrlen = ipv6_optlen(srh);
    if (skb.len - srhoff < hdrlen)
    return false;
    if (srh.type != IPV6_SRCRT_TYPE_4)
    return false;
    if (srh.segments_left > srh.first_segment)
    return false;
// Next Header matching
    if (srhinfo.mt_flags & IP6T_SRH_NEXTHDR)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_NEXTHDR,
    !(srh.nexthdr == srhinfo.next_hdr)))
    return false;
// Header Extension Length matching
    if (srhinfo.mt_flags & IP6T_SRH_LEN_EQ)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LEN_EQ,
    !(srh.hdrlen == srhinfo.hdr_len)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LEN_GT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LEN_GT,
    !(srh.hdrlen > srhinfo.hdr_len)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LEN_LT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LEN_LT,
    !(srh.hdrlen < srhinfo.hdr_len)))
    return false;
// Segments Left matching
    if (srhinfo.mt_flags & IP6T_SRH_SEGS_EQ)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_SEGS_EQ,
    !(srh.segments_left == srhinfo.segs_left)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_SEGS_GT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_SEGS_GT,
    !(srh.segments_left > srhinfo.segs_left)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_SEGS_LT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_SEGS_LT,
    !(srh.segments_left < srhinfo.segs_left)))
    return false;
//
// Last Entry matching
// Last_Entry field was introduced in revision 6 of the SRH draft.
// It was called First_Segment in the previous revision
//
    if (srhinfo.mt_flags & IP6T_SRH_LAST_EQ)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LAST_EQ,
    !(srh.first_segment == srhinfo.last_entry)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LAST_GT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LAST_GT,
    !(srh.first_segment > srhinfo.last_entry)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LAST_LT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LAST_LT,
    !(srh.first_segment < srhinfo.last_entry)))
    return false;
//
// Tag matchig
// Tag field was introduced in revision 6 of the SRH draft.
//
    if (srhinfo.mt_flags & IP6T_SRH_TAG)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_TAG,
    !(srh.tag == srhinfo.tag)))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn srh1_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool srh1_mt6(const struct sk_buff *skb, struct xt_action_param *par)
    {
    int hdrlen, psidoff, nsidoff, lsidoff, srhoff = 0;
    const struct ip6t_srh1 *srhinfo = par.matchinfo;
    struct in6_addr *psid, *nsid, *lsid;
    struct in6_addr _psid, _nsid, _lsid;
    struct ipv6_sr_hdr *srh;
    struct ipv6_sr_hdr _srh;
    if (ipv6_find_hdr(skb, &srhoff, IPPROTO_ROUTING, core::ptr::null_mut(), core::ptr::null_mut()) < 0)
    return false;
    srh = skb_header_pointer(skb, srhoff, sizeof(_srh), &_srh);
    if (!srh)
    return false;
    hdrlen = ipv6_optlen(srh);
    if (skb.len - srhoff < hdrlen)
    return false;
    if (srh.type != IPV6_SRCRT_TYPE_4)
    return false;
    if (srh.segments_left > srh.first_segment)
    return false;
// Next Header matching
    if (srhinfo.mt_flags & IP6T_SRH_NEXTHDR)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_NEXTHDR,
    !(srh.nexthdr == srhinfo.next_hdr)))
    return false;
// Header Extension Length matching
    if (srhinfo.mt_flags & IP6T_SRH_LEN_EQ)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LEN_EQ,
    !(srh.hdrlen == srhinfo.hdr_len)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LEN_GT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LEN_GT,
    !(srh.hdrlen > srhinfo.hdr_len)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LEN_LT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LEN_LT,
    !(srh.hdrlen < srhinfo.hdr_len)))
    return false;
// Segments Left matching
    if (srhinfo.mt_flags & IP6T_SRH_SEGS_EQ)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_SEGS_EQ,
    !(srh.segments_left == srhinfo.segs_left)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_SEGS_GT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_SEGS_GT,
    !(srh.segments_left > srhinfo.segs_left)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_SEGS_LT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_SEGS_LT,
    !(srh.segments_left < srhinfo.segs_left)))
    return false;
//
// Last Entry matching
// Last_Entry field was introduced in revision 6 of the SRH draft.
// It was called First_Segment in the previous revision
//
    if (srhinfo.mt_flags & IP6T_SRH_LAST_EQ)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LAST_EQ,
    !(srh.first_segment == srhinfo.last_entry)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LAST_GT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LAST_GT,
    !(srh.first_segment > srhinfo.last_entry)))
    return false;
    if (srhinfo.mt_flags & IP6T_SRH_LAST_LT)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LAST_LT,
    !(srh.first_segment < srhinfo.last_entry)))
    return false;
//
// Tag matchig
// Tag field was introduced in revision 6 of the SRH draft
//
    if (srhinfo.mt_flags & IP6T_SRH_TAG)
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_TAG,
    !(srh.tag == srhinfo.tag)))
    return false;
// Previous SID matching
    if (srhinfo.mt_flags & IP6T_SRH_PSID) {
    if (srh.segments_left == srh.first_segment)
    return false;
    psidoff = srhoff + sizeof(struct ipv6_sr_hdr) +
    ((srh.segments_left + 1) * sizeof(struct in6_addr));
    psid = skb_header_pointer(skb, psidoff, sizeof(_psid), &_psid);
    if (!psid)
    return false;
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_PSID,
    ipv6_masked_addr_cmp(psid, &srhinfo.psid_msk,
    &srhinfo.psid_addr)))
    return false;
    }
// Next SID matching
    if (srhinfo.mt_flags & IP6T_SRH_NSID) {
    if (srh.segments_left == 0)
    return false;
    nsidoff = srhoff + sizeof(struct ipv6_sr_hdr) +
    ((srh.segments_left - 1) * sizeof(struct in6_addr));
    nsid = skb_header_pointer(skb, nsidoff, sizeof(_nsid), &_nsid);
    if (!nsid)
    return false;
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_NSID,
    ipv6_masked_addr_cmp(nsid, &srhinfo.nsid_msk,
    &srhinfo.nsid_addr)))
    return false;
    }
// Last SID matching
    if (srhinfo.mt_flags & IP6T_SRH_LSID) {
    lsidoff = srhoff + sizeof(struct ipv6_sr_hdr);
    lsid = skb_header_pointer(skb, lsidoff, sizeof(_lsid), &_lsid);
    if (!lsid)
    return false;
    if (NF_SRH_INVF(srhinfo, IP6T_SRH_INV_LSID,
    ipv6_masked_addr_cmp(lsid, &srhinfo.lsid_msk,
    &srhinfo.lsid_addr)))
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn srh_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int srh_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_srh *srhinfo = par.matchinfo;
    if (srhinfo.mt_flags & ~IP6T_SRH_MASK) {
    pr_info_ratelimited("unknown srh match flags  %X\n",
    srhinfo.mt_flags);
    return -EINVAL;
    }
    if (srhinfo.mt_invflags & ~IP6T_SRH_INV_MASK) {
    pr_info_ratelimited("unknown srh invflags %X\n",
    srhinfo.mt_invflags);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn srh1_mt6_check(par: *const xt_mtchk_param) -> c_int {
    static int srh1_mt6_check(const struct xt_mtchk_param *par)
    {
    const struct ip6t_srh1 *srhinfo = par.matchinfo;
    if (srhinfo.mt_flags & ~IP6T_SRH_MASK) {
    pr_info_ratelimited("unknown srh match flags  %X\n",
    srhinfo.mt_flags);
    return -EINVAL;
    }
    if (srhinfo.mt_invflags & ~IP6T_SRH_INV_MASK) {
    pr_info_ratelimited("unknown srh invflags %X\n",
    srhinfo.mt_invflags);
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match srh_mt6_reg[] __read_mostly = {
    {
    .name		= "srh",
    .revision	= 0,
    .family		= NFPROTO_IPV6,
    .match		= srh_mt6,
    .matchsize	= sizeof(struct ip6t_srh),
    .checkentry	= srh_mt6_check,
    .me		= THIS_MODULE,
    },
    {
    .name           = "srh",
    .revision       = 1,
    .family         = NFPROTO_IPV6,
    .match          = srh1_mt6,
    .matchsize      = sizeof(struct ip6t_srh1),
    .checkentry     = srh1_mt6_check,
    .me             = THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn srh_mt6_init() -> int __init {
    static int __init srh_mt6_init(void)
    {
    return xt_register_matches(srh_mt6_reg, ARRAY_SIZE(srh_mt6_reg));
    }
#[no_mangle]
unsafe extern "C" fn srh_mt6_exit() -> void __exit {
    static void __exit srh_mt6_exit(void)
    {
    xt_unregister_matches(srh_mt6_reg, ARRAY_SIZE(srh_mt6_reg));
    }
    module_init(srh_mt6_init);
    module_exit(srh_mt6_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: IPv6 Segment Routing Header match");
    MODULE_AUTHOR("Ahmed Abdelsalam <amsalam20@gmail.com>");
