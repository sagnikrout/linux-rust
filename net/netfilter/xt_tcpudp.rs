//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_tcpudp.c
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

    MODULE_DESCRIPTION("Xtables: TCP, UDP and UDP-Lite match");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("xt_tcp");
    MODULE_ALIAS("xt_udp");
    MODULE_ALIAS("ipt_udp");
    MODULE_ALIAS("ipt_tcp");
    MODULE_ALIAS("ip6t_udp");
    MODULE_ALIAS("ip6t_tcp");
    MODULE_ALIAS("ipt_icmp");
    MODULE_ALIAS("ip6t_icmp6");
// Returns 1 if the port is matched by the range, 0 otherwise
    static inline bool
    port_match(u_int16_t min, u_int16_t max, u_int16_t port, bool invert)
    {
    return (port >= min && port <= max) ^ invert;
    }
    static bool
    tcp_find_option(u_int8_t option,
    const struct sk_buff *skb,
    unsigned int protoff,
    unsigned int optlen,
    bool invert,
    bool *hotdrop)
    {
// tcp.doff is only 4 bits, ie. max 15 * 4 bytes
    const u_int8_t *op;
    u_int8_t _opt[60 - sizeof(struct tcphdr)];
    unsigned int i;
    if (!optlen)
    return invert;
// If we don't have the whole header, drop packet.
    op = skb_header_pointer(skb, protoff + sizeof(struct tcphdr),
    optlen, _opt);
    if (op == core::ptr::null_mut()) {
// hotdrop = true;
    return false;
    }
    for (i = 0; i < optlen; ) {
    if (op[i] == option) return !invert;
    if (op[i] < 2 || i == optlen - 1)
    i++;
    else
    i += op[i + 1] ? : 1;
    }
    return invert;
    }
#[no_mangle]
unsafe extern "C" fn tcp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool tcp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct tcphdr *th;
    struct tcphdr _tcph;
    const struct xt_tcp *tcpinfo = par.matchinfo;
    if (par.fragoff != 0) {
// To quote Alan:
    Don't allow a fragment of TCP 8 bytes in. Nobody normal
    causes this. Its a cracker trying to break in by doing a
    flag overwrite to pass the direction checks.
//
    if (par.fragoff == 1)
    par.hotdrop = true;
// Must not be a fragment.
    return false;
    }
    th = skb_header_pointer(skb, par.thoff, sizeof(_tcph), &_tcph);
    if (th == core::ptr::null_mut()) {
// We've been asked to examine this packet, and we
    can't.  Hence, no choice but to drop. */
    par.hotdrop = true;
    return false;
    }
    if (!port_match(tcpinfo.spts[0], tcpinfo.spts[1],
    ntohs(th.source),
    !!(tcpinfo.invflags & XT_TCP_INV_SRCPT)))
    return false;
    if (!port_match(tcpinfo.dpts[0], tcpinfo.dpts[1],
    ntohs(th.dest),
    !!(tcpinfo.invflags & XT_TCP_INV_DSTPT)))
    return false;
    if (!NF_INVF(tcpinfo, XT_TCP_INV_FLAGS,
    (((unsigned char *)th)[13] & tcpinfo.flg_mask) == tcpinfo.flg_cmp))
    return false;
    if (tcpinfo.option) {
    if (th.doff * 4 < sizeof(_tcph)) {
    par.hotdrop = true;
    return false;
    }
    if (!tcp_find_option(tcpinfo.option, skb, par.thoff,
    th.doff*4 - sizeof(_tcph),
    tcpinfo.invflags & XT_TCP_INV_OPTION,
    &par.hotdrop))
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn tcp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int tcp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_tcp *tcpinfo = par.matchinfo;
// Must specify no unknown invflags
    return (tcpinfo.invflags & ~XT_TCP_INV_MASK) ? -EINVAL : 0;
    }
#[no_mangle]
unsafe extern "C" fn udp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool udp_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct udphdr *uh;
    struct udphdr _udph;
    const struct xt_udp *udpinfo = par.matchinfo;
// Must not be a fragment.
    if (par.fragoff != 0)
    return false;
    uh = skb_header_pointer(skb, par.thoff, sizeof(_udph), &_udph);
    if (uh == core::ptr::null_mut()) {
// We've been asked to examine this packet, and we
    can't.  Hence, no choice but to drop. */
    par.hotdrop = true;
    return false;
    }
    return port_match(udpinfo.spts[0], udpinfo.spts[1],
    ntohs(uh.source),
    !!(udpinfo.invflags & XT_UDP_INV_SRCPT))
    && port_match(udpinfo.dpts[0], udpinfo.dpts[1],
    ntohs(uh.dest),
    !!(udpinfo.invflags & XT_UDP_INV_DSTPT));
    }
#[no_mangle]
unsafe extern "C" fn udp_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int udp_mt_check(const struct xt_mtchk_param *par)
    {
    const struct xt_udp *udpinfo = par.matchinfo;
// Must specify no unknown invflags
    return (udpinfo.invflags & ~XT_UDP_INV_MASK) ? -EINVAL : 0;
    }
// Returns 1 if the type and code is matched by the range, 0 otherwise
    static bool type_code_in_range(u8 test_type, u8 min_code, u8 max_code,
    u8 type, u8 code)
    {
    let mut type: return = = test_type && code >= min_code && code <= max_code;
    }
    static bool icmp_type_code_match(u8 test_type, u8 min_code, u8 max_code,
    u8 type, u8 code, bool invert)
    {
    return (test_type == 0xFF ||
    type_code_in_range(test_type, min_code, max_code, type, code))
    ^ invert;
    }
    static bool icmp6_type_code_match(u8 test_type, u8 min_code, u8 max_code,
    u8 type, u8 code, bool invert)
    {
    return type_code_in_range(test_type, min_code, max_code, type, code) ^ invert;
    }
    static bool
    icmp_match(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct icmphdr *ic;
    struct icmphdr _icmph;
    const struct ipt_icmp *icmpinfo = par.matchinfo;
// Must not be a fragment.
    if (par.fragoff != 0)
    return false;
    ic = skb_header_pointer(skb, par.thoff, sizeof(_icmph), &_icmph);
    if (!ic) {
// We've been asked to examine this packet, and we
// can't.  Hence, no choice but to drop.
//
    par.hotdrop = true;
    return false;
    }
    return icmp_type_code_match(icmpinfo.type,
    icmpinfo.code[0],
    icmpinfo.code[1],
    ic.type, ic.code,
    !!(icmpinfo.invflags & IPT_ICMP_INV));
    }
    static bool
    icmp6_match(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct icmp6hdr *ic;
    struct icmp6hdr _icmph;
    const struct ip6t_icmp *icmpinfo = par.matchinfo;
// Must not be a fragment.
    if (par.fragoff != 0)
    return false;
    ic = skb_header_pointer(skb, par.thoff, sizeof(_icmph), &_icmph);
    if (!ic) {
// We've been asked to examine this packet, and we
// can't.  Hence, no choice but to drop.
//
    par.hotdrop = true;
    return false;
    }
    return icmp6_type_code_match(icmpinfo.type,
    icmpinfo.code[0],
    icmpinfo.code[1],
    ic.icmp6_type, ic.icmp6_code,
    !!(icmpinfo.invflags & IP6T_ICMP_INV));
    }
#[no_mangle]
unsafe extern "C" fn icmp_checkentry(par: *const xt_mtchk_param) -> c_int {
    static int icmp_checkentry(const struct xt_mtchk_param *par)
    {
    const struct ipt_icmp *icmpinfo = par.matchinfo;
    return (icmpinfo.invflags & ~IPT_ICMP_INV) ? -EINVAL : 0;
    }
#[no_mangle]
unsafe extern "C" fn icmp6_checkentry(par: *const xt_mtchk_param) -> c_int {
    static int icmp6_checkentry(const struct xt_mtchk_param *par)
    {
    const struct ip6t_icmp *icmpinfo = par.matchinfo;
    return (icmpinfo.invflags & ~IP6T_ICMP_INV) ? -EINVAL : 0;
    }
    static struct xt_match tcpudp_mt_reg[] __read_mostly = {
    {
    .name		= "tcp",
    .family		= NFPROTO_IPV4,
    .checkentry	= tcp_mt_check,
    .match		= tcp_mt,
    .matchsize	= sizeof(struct xt_tcp),
    .proto		= IPPROTO_TCP,
    .me		= THIS_MODULE,
    },
    {
    .name		= "tcp",
    .family		= NFPROTO_IPV6,
    .checkentry	= tcp_mt_check,
    .match		= tcp_mt,
    .matchsize	= sizeof(struct xt_tcp),
    .proto		= IPPROTO_TCP,
    .me		= THIS_MODULE,
    },
    {
    .name		= "udp",
    .family		= NFPROTO_IPV4,
    .checkentry	= udp_mt_check,
    .match		= udp_mt,
    .matchsize	= sizeof(struct xt_udp),
    .proto		= IPPROTO_UDP,
    .me		= THIS_MODULE,
    },
    {
    .name		= "udp",
    .family		= NFPROTO_IPV6,
    .checkentry	= udp_mt_check,
    .match		= udp_mt,
    .matchsize	= sizeof(struct xt_udp),
    .proto		= IPPROTO_UDP,
    .me		= THIS_MODULE,
    },
    {
    .name		= "udplite",
    .family		= NFPROTO_IPV4,
    .checkentry	= udp_mt_check,
    .match		= udp_mt,
    .matchsize	= sizeof(struct xt_udp),
    .proto		= IPPROTO_UDPLITE,
    .me		= THIS_MODULE,
    },
    {
    .name		= "udplite",
    .family		= NFPROTO_IPV6,
    .checkentry	= udp_mt_check,
    .match		= udp_mt,
    .matchsize	= sizeof(struct xt_udp),
    .proto		= IPPROTO_UDPLITE,
    .me		= THIS_MODULE,
    },
    {
    .name       = "icmp",
    .match      = icmp_match,
    .matchsize  = sizeof(struct ipt_icmp),
    .checkentry = icmp_checkentry,
    .proto      = IPPROTO_ICMP,
    .family     = NFPROTO_IPV4,
    .me         = THIS_MODULE,
    },
    {
    .name       = "icmp6",
    .match      = icmp6_match,
    .matchsize  = sizeof(struct ip6t_icmp),
    .checkentry = icmp6_checkentry,
    .proto      = IPPROTO_ICMPV6,
    .family     = NFPROTO_IPV6,
    .me	    = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn tcpudp_mt_init() -> int __init {
    static int __init tcpudp_mt_init(void)
    {
    return xt_register_matches(tcpudp_mt_reg, ARRAY_SIZE(tcpudp_mt_reg));
    }
#[no_mangle]
unsafe extern "C" fn tcpudp_mt_exit() -> void __exit {
    static void __exit tcpudp_mt_exit(void)
    {
    xt_unregister_matches(tcpudp_mt_reg, ARRAY_SIZE(tcpudp_mt_reg));
    }
    module_init(tcpudp_mt_init);
    module_exit(tcpudp_mt_exit);
