//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/ipt_REJECT.c
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
// This is a module which is used for rejecting packets.
//
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Netfilter Core Team <coreteam@netfilter.org>");
    MODULE_DESCRIPTION("Xtables: packet \"rejection\" target for IPv4");
    static unsigned int
    reject_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ipt_reject_info *reject = par.targinfo;
    let mut hook: c_int = xt_hooknum(par);
    switch (reject.with) {
    case IPT_ICMP_NET_UNREACHABLE:
    nf_send_unreach(skb, ICMP_NET_UNREACH, hook);
    break;
    case IPT_ICMP_HOST_UNREACHABLE:
    nf_send_unreach(skb, ICMP_HOST_UNREACH, hook);
    break;
    case IPT_ICMP_PROT_UNREACHABLE:
    nf_send_unreach(skb, ICMP_PROT_UNREACH, hook);
    break;
    case IPT_ICMP_PORT_UNREACHABLE:
    nf_send_unreach(skb, ICMP_PORT_UNREACH, hook);
    break;
    case IPT_ICMP_NET_PROHIBITED:
    nf_send_unreach(skb, ICMP_NET_ANO, hook);
    break;
    case IPT_ICMP_HOST_PROHIBITED:
    nf_send_unreach(skb, ICMP_HOST_ANO, hook);
    break;
    case IPT_ICMP_ADMIN_PROHIBITED:
    nf_send_unreach(skb, ICMP_PKT_FILTERED, hook);
    break;
    case IPT_TCP_RESET:
    nf_send_reset(xt_net(par), par.state.sk, skb, hook);
    break;
    case IPT_ICMP_ECHOREPLY:
// Doesn't happen.
    break;
    }
    return NF_DROP;
    }
#[no_mangle]
unsafe extern "C" fn reject_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int reject_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ipt_reject_info *rejinfo = par.targinfo;
    const struct ipt_entry *e = par.entryinfo;
    if (rejinfo.with == IPT_ICMP_ECHOREPLY) {
    pr_info_ratelimited("ECHOREPLY no longer supported.\n");
    return -EINVAL;
    } else if (rejinfo.with == IPT_TCP_RESET) {
// Must specify that it's a TCP packet
    if (e.ip.proto != IPPROTO_TCP ||
    (e.ip.invflags & XT_INV_PROTO)) {
    pr_info_ratelimited("TCP_RESET invalid for non-tcp\n");
    return -EINVAL;
    }
    }
    return 0;
    }
    static struct xt_target reject_tg_reg __read_mostly = {
    .name		= "REJECT",
    .family		= NFPROTO_IPV4,
    .target		= reject_tg,
    .targetsize	= sizeof(struct ipt_reject_info),
    .table		= "filter",
    .hooks		= (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD) |
    (1 << NF_INET_LOCAL_OUT),
    .checkentry	= reject_tg_check,
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn reject_tg_init() -> int __init {
    static int __init reject_tg_init(void)
    {
    return xt_register_target(&reject_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn reject_tg_exit() -> void __exit {
    static void __exit reject_tg_exit(void)
    {
    xt_unregister_target(&reject_tg_reg);
    }
    module_init(reject_tg_init);
    module_exit(reject_tg_exit);
