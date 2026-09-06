//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_ipvs.c
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
// xt_ipvs - kernel module to match IPVS connection properties
//
// Author: Hannes Eder <heder@google.com>
//

    MODULE_AUTHOR("Hannes Eder <heder@google.com>");
    MODULE_DESCRIPTION("Xtables: match IPVS connection properties");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_ipvs");
    MODULE_ALIAS("ip6t_ipvs");
// borrowed from xt_conntrack
    static bool ipvs_mt_addrcmp(const union nf_inet_addr *kaddr,
    const union nf_inet_addr *uaddr,
    const union nf_inet_addr *umask,
    unsigned int l3proto)
    {
    if (l3proto == NFPROTO_IPV4)
    return ((kaddr.ip ^ uaddr.ip) & umask.ip) == 0;

#[no_mangle]
pub unsafe extern "C" fn if(NFPROTO_IPV6: l3proto ==) -> else {
    else if (l3proto == NFPROTO_IPV6)
    return ipv6_masked_addr_cmp(&kaddr.in6, &umask.in6,
    &uaddr.in6) == 0;

    else
    return false;
    }
    static bool
    ipvs_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_ipvs_mtinfo *data = par.matchinfo;
    struct netns_ipvs *ipvs = net_ipvs(xt_net(par));
// ipvs_mt_check ensures that family is only NFPROTO_IPV[46].
    let mut family: u_int8_t = xt_family(par);
    struct ip_vs_iphdr iph;
    struct ip_vs_protocol *pp;
    struct ip_vs_conn *cp;
    let mut match: bool = true;
    if (data.bitmask == XT_IPVS_IPVS_PROPERTY) {
    match = skb.ipvs_property ^
    !!(data.invert & XT_IPVS_IPVS_PROPERTY);
    goto out;
    }
// other flags than XT_IPVS_IPVS_PROPERTY are set
    if (!skb.ipvs_property) {
    match = false;
    goto out;
    }
    ip_vs_fill_iph_skb(family, skb, true, &iph);
    if (data.bitmask & XT_IPVS_PROTO)
    if ((iph.protocol == data.l4proto) ^
    !(data.invert & XT_IPVS_PROTO)) {
    match = false;
    goto out;
    }
    pp = ip_vs_proto_get(iph.protocol);
    if (unlikely(!pp)) {
    match = false;
    goto out;
    }
//
// Check if the packet belongs to an existing entry
//
    cp = pp.conn_out_get(ipvs, family, skb, &iph);
    if (unlikely(cp == core::ptr::null_mut())) {
    match = false;
    goto out;
    }
//
// We found a connection, i.e. ct != 0, make sure to call
// __ip_vs_conn_put before returning.  In our case jump to out_put_con.
//
    if (data.bitmask & XT_IPVS_VPORT)
    if ((cp.vport == data.vport) ^
    !(data.invert & XT_IPVS_VPORT)) {
    match = false;
    goto out_put_cp;
    }
    if (data.bitmask & XT_IPVS_VPORTCTL)
    if ((cp.control != core::ptr::null_mut() &&
    cp.control.vport == data.vportctl) ^
    !(data.invert & XT_IPVS_VPORTCTL)) {
    match = false;
    goto out_put_cp;
    }
    if (data.bitmask & XT_IPVS_DIR) {
    enum ip_conntrack_info ctinfo;
    struct nf_conn *ct = nf_ct_get(skb, &ctinfo);
    if (ct == core::ptr::null_mut()) {
    match = false;
    goto out_put_cp;
    }
    if ((ctinfo >= IP_CT_IS_REPLY) ^
    !!(data.invert & XT_IPVS_DIR)) {
    match = false;
    goto out_put_cp;
    }
    }
    if (data.bitmask & XT_IPVS_METHOD)
    if (((cp.flags & IP_VS_CONN_F_FWD_MASK) == data.fwd_method) ^
    !(data.invert & XT_IPVS_METHOD)) {
    match = false;
    goto out_put_cp;
    }
    if (data.bitmask & XT_IPVS_VADDR) {
    if (ipvs_mt_addrcmp(&cp.vaddr, &data.vaddr,
    &data.vmask, family) ^
    !(data.invert & XT_IPVS_VADDR)) {
    match = false;
    goto out_put_cp;
    }
    }
    out_put_cp:
    __ip_vs_conn_put(cp);
    out:
    return match;
    }
#[no_mangle]
unsafe extern "C" fn ipvs_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ipvs_mt_check(const struct xt_mtchk_param *par)
    {
    if (par.family != NFPROTO_IPV4

    && par.family != NFPROTO_IPV6

    ) {
    pr_info_ratelimited("protocol family %u not supported\n",
    par.family);
    return -EINVAL;
    }
    return 0;
    }
    static struct xt_match xt_ipvs_mt_reg __read_mostly = {
    .name       = "ipvs",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .match      = ipvs_mt,
    .checkentry = ipvs_mt_check,
    .matchsize  = XT_ALIGN(sizeof(struct xt_ipvs_mtinfo)),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ipvs_mt_init() -> int __init {
    static int __init ipvs_mt_init(void)
    {
    return xt_register_match(&xt_ipvs_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ipvs_mt_exit() -> void __exit {
    static void __exit ipvs_mt_exit(void)
    {
    xt_unregister_match(&xt_ipvs_mt_reg);
    }
    module_init(ipvs_mt_init);
    module_exit(ipvs_mt_exit);
