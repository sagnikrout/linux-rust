//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_snmp.c
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
//
// SNMP service broadcast connection tracking helper
//
// (c) 2011 Jiri Olsa <jolsa@redhat.com>
//

    MODULE_AUTHOR("Jiri Olsa <jolsa@redhat.com>");
    MODULE_DESCRIPTION("SNMP service broadcast connection tracking helper");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_NFCT_HELPER("snmp");
    let mut __read_mostly: static unsigned int timeout = 30;
    module_param(timeout, uint, 0400);
    MODULE_PARM_DESC(timeout, "timeout for master connection/replies in seconds");
    nf_nat_snmp_hook_fn __rcu *nf_nat_snmp_hook;
    EXPORT_SYMBOL_GPL(nf_nat_snmp_hook);
    static int snmp_conntrack_help(struct sk_buff *skb, unsigned int protoff,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo)
    {
    nf_nat_snmp_hook_fn *nf_nat_snmp;
    nf_conntrack_broadcast_help(skb, ct, ctinfo, timeout);
    nf_nat_snmp = rcu_dereference(nf_nat_snmp_hook);
    if (nf_nat_snmp && ct.status & IPS_NAT_MASK)
    return nf_nat_snmp(skb, protoff, ct, ctinfo);
    return NF_ACCEPT;
    }
    static struct nf_conntrack_expect_policy exp_policy = {
    .max_expected	= 1,
    };
    static struct nf_conntrack_helper helper __read_mostly;
    static struct nf_conntrack_helper *helper_ptr __read_mostly;
#[no_mangle]
unsafe extern "C" fn nf_conntrack_snmp_init() -> int __init {
    static int __init nf_conntrack_snmp_init(void)
    {
    exp_policy.timeout = timeout;
    nf_ct_helper_init(&helper, AF_INET, IPPROTO_UDP,
    "snmp",
    &exp_policy, 0, snmp_conntrack_help, core::ptr::null_mut(),
    THIS_MODULE);
    return nf_conntrack_helper_register(&helper, &helper_ptr);
    }
#[no_mangle]
unsafe extern "C" fn nf_conntrack_snmp_fini() -> void __exit {
    static void __exit nf_conntrack_snmp_fini(void)
    {
    nf_conntrack_helper_unregister(helper_ptr);
    }
    module_init(nf_conntrack_snmp_init);
    module_exit(nf_conntrack_snmp_fini);
