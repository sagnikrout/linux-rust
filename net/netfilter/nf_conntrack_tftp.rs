//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_tftp.c
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
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
//

    MODULE_AUTHOR("Magnus Boden <mb@ozaba.mine.nu>");
    MODULE_DESCRIPTION("TFTP connection tracking helper");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ip_conntrack_tftp");
    MODULE_ALIAS_NFCT_HELPER(HELPER_NAME);
    nf_nat_tftp_hook_fn __rcu *nf_nat_tftp_hook __read_mostly;
    EXPORT_SYMBOL_GPL(nf_nat_tftp_hook);
    static int tftp_help(struct sk_buff *skb,
    unsigned int protoff,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo)
    {
    const struct tftphdr *tfh;
    struct tftphdr _tftph;
    struct nf_conntrack_expect *exp;
    struct nf_conntrack_tuple *tuple;
    let mut ret: c_uint = NF_ACCEPT;
    nf_nat_tftp_hook_fn *nf_nat_tftp;
    tfh = skb_header_pointer(skb, protoff + sizeof(struct udphdr),
    sizeof(_tftph), &_tftph);
    if (tfh == core::ptr::null_mut())
    return NF_ACCEPT;
    switch (ntohs(tfh.opcode)) {
    case TFTP_OPCODE_READ:
    case TFTP_OPCODE_WRITE:
// RRQ and WRQ works the same way
    nf_ct_dump_tuple(&ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple);
    nf_ct_dump_tuple(&ct.tuplehash[IP_CT_DIR_REPLY].tuple);
    exp = nf_ct_expect_alloc(ct);
    if (exp == core::ptr::null_mut()) {
    nf_ct_helper_log(skb, ct, "cannot alloc expectation");
    return NF_DROP;
    }
    tuple = &ct.tuplehash[IP_CT_DIR_REPLY].tuple;
    nf_ct_expect_init(exp, NF_CT_EXPECT_CLASS_DEFAULT,
    nf_ct_l3num(ct),
    &tuple.src.u3, &tuple.dst.u3,
    IPPROTO_UDP, core::ptr::null_mut(), &tuple.dst.u.udp.port);
    pr_debug("expect: ");
    nf_ct_dump_tuple(&exp.tuple);
    nf_nat_tftp = rcu_dereference(nf_nat_tftp_hook);
    if (nf_nat_tftp && ct.status & IPS_NAT_MASK)
    ret = nf_nat_tftp(skb, ct, ctinfo, exp);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: nf_ct_expect_related(exp, 0: 0) !=) -> else {
    nf_ct_helper_log(skb, ct, "cannot add expectation");
    ret = NF_DROP;
    }
    nf_ct_expect_put(exp);
    break;
    case TFTP_OPCODE_DATA:
    case TFTP_OPCODE_ACK:
    pr_debug("Data/ACK opcode\n");
    break;
    case TFTP_OPCODE_ERROR:
    pr_debug("Error opcode\n");
    break;
    default:
    pr_debug("Unknown opcode\n");
    }
    return ret;
    }
    static struct nf_conntrack_helper tftp __read_mostly;
    static struct nf_conntrack_helper *tftp_ptr __read_mostly;
    static const struct nf_conntrack_expect_policy tftp_exp_policy = {
    .max_expected	= 1,
    .timeout	= 5 * 60,
    };
#[no_mangle]
unsafe extern "C" fn nf_conntrack_tftp_fini() -> void __exit {
    static void __exit nf_conntrack_tftp_fini(void)
    {
    nf_conntrack_helper_unregister(tftp_ptr);
    }
#[no_mangle]
unsafe extern "C" fn nf_conntrack_tftp_init() -> int __init {
    static int __init nf_conntrack_tftp_init(void)
    {
    int ret;
    NF_CT_HELPER_BUILD_BUG_ON(0);
    nf_ct_helper_init(&tftp, NFPROTO_UNSPEC, IPPROTO_UDP,
    HELPER_NAME,
    &tftp_exp_policy, 0, tftp_help, core::ptr::null_mut(),
    THIS_MODULE);
    ret = nf_conntrack_helper_register(&tftp, &tftp_ptr);
    if (ret < 0) {
    pr_err("failed to register helpers\n");
    return ret;
    }
    return 0;
    }
    module_init(nf_conntrack_tftp_init);
    module_exit(nf_conntrack_tftp_fini);
