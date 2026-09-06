//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_nat_amanda.c
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
// Amanda extension for TCP NAT alteration.
// (C) 2002 by Brian J. Murrell <netfilter@interlinx.bc.ca>
// based on a copy of HW's ip_nat_irc.c as well as other modules
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
//

    MODULE_AUTHOR("Brian J. Murrell <netfilter@interlinx.bc.ca>");
    MODULE_DESCRIPTION("Amanda NAT helper");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_NF_NAT_HELPER(NAT_HELPER_NAME);
    static struct nf_conntrack_nat_helper nat_helper_amanda =
    NF_CT_NAT_HELPER_INIT(NAT_HELPER_NAME);
    static unsigned int help(struct sk_buff *skb,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo,
    unsigned int protoff,
    unsigned int matchoff,
    unsigned int matchlen,
    struct nf_conntrack_expect *exp)
    {
    char buffer[sizeof("65535")];
    u_int16_t port;
// Connection comes from client.
    exp.saved_proto.tcp.port = exp.tuple.dst.u.tcp.port;
    exp.dir = IP_CT_DIR_ORIGINAL;
// When you see the packet, we need to NAT it the same as the
// this one (ie. same IP: it will be TCP and master is UDP).
    exp.expectfn = nf_nat_follow_master;
// Try to get same port: if not, try to change it.
    port = nf_nat_exp_find_port(exp, ntohs(exp.saved_proto.tcp.port));
    if (port == 0) {
    nf_ct_helper_log(skb, ct, "all ports in use");
    return NF_DROP;
    }
    snprintf(buffer, sizeof(buffer), "%u", port);
    if (!nf_nat_mangle_udp_packet(skb, ct, ctinfo,
    protoff, matchoff, matchlen,
    buffer, strlen(buffer))) {
    nf_ct_helper_log(skb, ct, "cannot mangle packet");
    nf_ct_unexpect_related(exp);
    return NF_DROP;
    }
    return NF_ACCEPT;
    }
#[no_mangle]
unsafe extern "C" fn nf_nat_amanda_fini() -> void __exit {
    static void __exit nf_nat_amanda_fini(void)
    {
    nf_nat_helper_unregister(&nat_helper_amanda);
    RCU_INIT_POINTER(nf_nat_amanda_hook, core::ptr::null_mut());
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn nf_nat_amanda_init() -> int __init {
    static int __init nf_nat_amanda_init(void)
    {
    BUG_ON(nf_nat_amanda_hook != core::ptr::null_mut());
    nf_nat_helper_register(&nat_helper_amanda);
    RCU_INIT_POINTER(nf_nat_amanda_hook, help);
    return 0;
    }
    module_init(nf_nat_amanda_init);
    module_exit(nf_nat_amanda_fini);
