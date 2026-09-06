//! Automatically rewritten from C to Rust
//! Source: net/ipv4/netfilter/ipt_SYNPROXY.c
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
// Copyright (c) 2013 Patrick McHardy <kaber@trash.net>
//

    static unsigned int
    synproxy_tg4(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct xt_synproxy_info *info = par.targinfo;
    struct net *net = xt_net(par);
    struct synproxy_net *snet = synproxy_pernet(net);
    let mut opts: synproxy_options = {};
    struct tcphdr *th, _th;
    if (nf_ip_checksum(skb, xt_hooknum(par), par.thoff, IPPROTO_TCP))
    return NF_DROP;
    th = skb_header_pointer(skb, par.thoff, sizeof(_th), &_th);
    if (th == core::ptr::null_mut())
    return NF_DROP;
    if (!synproxy_parse_options(skb, par.thoff, th, &opts))
    return NF_DROP;
    if (th.syn && !(th.ack || th.fin || th.rst)) {
// Initial SYN from client
    this_cpu_inc(snet.stats.syn_received);
    if (th.ece && th.cwr)
    opts.options |= XT_SYNPROXY_OPT_ECN;
    opts.options &= info.options;
    opts.mss_encode = opts.mss_option;
    opts.mss_option = info.mss;
    if (opts.options & XT_SYNPROXY_OPT_TIMESTAMP)
    synproxy_init_timestamp_cookie(info, &opts);
    else
    opts.options &= ~(XT_SYNPROXY_OPT_WSCALE |
    XT_SYNPROXY_OPT_SACK_PERM |
    XT_SYNPROXY_OPT_ECN);
    synproxy_send_client_synack(net, skb, th, &opts);
    consume_skb(skb);
    return NF_STOLEN;
    } else if (th.ack && !(th.fin || th.rst || th.syn)) {
// ACK from client
    if (synproxy_recv_client_ack(net, skb, th, &opts, ntohl(th.seq))) {
    consume_skb(skb);
    return NF_STOLEN;
    } else {
    return NF_DROP;
    }
    }
    return XT_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn synproxy_tg4_check(par: *const xt_tgchk_param) -> c_int {
    static int synproxy_tg4_check(const struct xt_tgchk_param *par)
    {
    struct synproxy_net *snet = synproxy_pernet(par.net);
    const struct ipt_entry *e = par.entryinfo;
    int err;
    if (e.ip.proto != IPPROTO_TCP ||
    e.ip.invflags & XT_INV_PROTO)
    return -EINVAL;
    err = nf_ct_netns_get(par.net, par.family);
    if (err)
    return err;
    err = nf_synproxy_ipv4_init(snet, par.net);
    if (err) {
    nf_ct_netns_put(par.net, par.family);
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn synproxy_tg4_destroy(par: *const xt_tgdtor_param) {
    static void synproxy_tg4_destroy(const struct xt_tgdtor_param *par)
    {
    struct synproxy_net *snet = synproxy_pernet(par.net);
    nf_synproxy_ipv4_fini(snet, par.net);
    nf_ct_netns_put(par.net, par.family);
    }
    static struct xt_target synproxy_tg4_reg __read_mostly = {
    .name		= "SYNPROXY",
    .family		= NFPROTO_IPV4,
    .hooks		= (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD),
    .target		= synproxy_tg4,
    .targetsize	= sizeof(struct xt_synproxy_info),
    .checkentry	= synproxy_tg4_check,
    .destroy	= synproxy_tg4_destroy,
    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn synproxy_tg4_init() -> int __init {
    static int __init synproxy_tg4_init(void)
    {
    return xt_register_target(&synproxy_tg4_reg);
    }
#[no_mangle]
unsafe extern "C" fn synproxy_tg4_exit() -> void __exit {
    static void __exit synproxy_tg4_exit(void)
    {
    xt_unregister_target(&synproxy_tg4_reg);
    }
    module_init(synproxy_tg4_init);
    module_exit(synproxy_tg4_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_DESCRIPTION("Intercept TCP connections and establish them using syncookies");
