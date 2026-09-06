//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_nat_tftp.c
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
// (C) 2001-2002 Magnus Boden <mb@ozaba.mine.nu>
//

    MODULE_AUTHOR("Magnus Boden <mb@ozaba.mine.nu>");
    MODULE_DESCRIPTION("TFTP NAT helper");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_NF_NAT_HELPER(NAT_HELPER_NAME);
    static struct nf_conntrack_nat_helper nat_helper_tftp =
    NF_CT_NAT_HELPER_INIT(NAT_HELPER_NAME);
    static unsigned int help(struct sk_buff *skb,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo,
    struct nf_conntrack_expect *exp)
    {
    exp.saved_proto.udp.port
    = ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple.src.u.udp.port;
    exp.dir = IP_CT_DIR_REPLY;
    exp.expectfn = nf_nat_follow_master;
    if (nf_ct_expect_related(exp, 0) != 0) {
    nf_ct_helper_log(skb, ct, "cannot add expectation");
    return NF_DROP;
    }
    return NF_ACCEPT;
    }
#[no_mangle]
unsafe extern "C" fn nf_nat_tftp_fini() -> void __exit {
    static void __exit nf_nat_tftp_fini(void)
    {
    nf_nat_helper_unregister(&nat_helper_tftp);
    RCU_INIT_POINTER(nf_nat_tftp_hook, core::ptr::null_mut());
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn nf_nat_tftp_init() -> int __init {
    static int __init nf_nat_tftp_init(void)
    {
    BUG_ON(nf_nat_tftp_hook != core::ptr::null_mut());
    nf_nat_helper_register(&nat_helper_tftp);
    RCU_INIT_POINTER(nf_nat_tftp_hook, help);
    return 0;
    }
    module_init(nf_nat_tftp_init);
    module_exit(nf_nat_tftp_fini);
