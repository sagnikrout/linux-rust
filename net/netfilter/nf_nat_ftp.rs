//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_nat_ftp.c
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
// FTP extension for TCP NAT alteration.
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Rusty Russell <rusty@rustcorp.com.au>");
    MODULE_DESCRIPTION("ftp NAT helper");
    MODULE_ALIAS_NF_NAT_HELPER(NAT_HELPER_NAME);
// FIXME: Time out? --RR
    static struct nf_conntrack_nat_helper nat_helper_ftp =
    NF_CT_NAT_HELPER_INIT(NAT_HELPER_NAME);
    static int nf_nat_ftp_fmt_cmd(struct nf_conn *ct, enum nf_ct_ftp_type type,
    char *buffer, size_t buflen,
    union nf_inet_addr *addr, u16 port)
    {
    switch (type) {
    case NF_CT_FTP_PORT:
    case NF_CT_FTP_PASV:
    return snprintf(buffer, buflen, "%u,%u,%u,%u,%u,%u",
    ((unsigned char *)&addr.ip)[0],
    ((unsigned char *)&addr.ip)[1],
    ((unsigned char *)&addr.ip)[2],
    ((unsigned char *)&addr.ip)[3],
    port >> 8,
    port & 0xFF);
    case NF_CT_FTP_EPRT:
    if (nf_ct_l3num(ct) == NFPROTO_IPV4)
    return snprintf(buffer, buflen, "|1|%pI4|%u|",
    &addr.ip, port);
    else
    return snprintf(buffer, buflen, "|2|%pI6|%u|",
    &addr.ip6, port);
    case NF_CT_FTP_EPSV:
    return snprintf(buffer, buflen, "|||%u|", port);
    }
    return 0;
    }
// So, this packet has hit the connection tracking matching code.
    Mangle it, and change the expectation to match the new version. */
    static unsigned int nf_nat_ftp(struct sk_buff *skb,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo,
    enum nf_ct_ftp_type type,
    unsigned int protoff,
    unsigned int matchoff,
    unsigned int matchlen,
    struct nf_conntrack_expect *exp)
    {
    union nf_inet_addr newaddr;
    u_int16_t port;
    let mut dir: c_int = CTINFO2DIR(ctinfo);
    char buffer[sizeof("|1||65535|") + INET6_ADDRSTRLEN];
    unsigned int buflen;
    pr_debug("type %i, off %u len %u\n", type, matchoff, matchlen);
// Connection will come from wherever this packet goes, hence !dir
    newaddr = ct.tuplehash[!dir].tuple.dst.u3;
    exp.saved_proto.tcp.port = exp.tuple.dst.u.tcp.port;
    exp.dir = !dir;
// When you see the packet, we need to NAT it the same as the
// this one.
    exp.expectfn = nf_nat_follow_master;
    port = nf_nat_exp_find_port(exp, ntohs(exp.saved_proto.tcp.port));
    if (port == 0) {
    nf_ct_helper_log(skb, ct, "all ports in use");
    return NF_DROP;
    }
    buflen = nf_nat_ftp_fmt_cmd(ct, type, buffer, sizeof(buffer),
    &newaddr, port);
    if (!buflen)
    goto out;
    pr_debug("calling nf_nat_mangle_tcp_packet\n");
    if (!nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, matchoff,
    matchlen, buffer, buflen))
    goto out;
    return NF_ACCEPT;
    out:
    nf_ct_helper_log(skb, ct, "cannot mangle packet");
    nf_ct_unexpect_related(exp);
    return NF_DROP;
    }
#[no_mangle]
unsafe extern "C" fn nf_nat_ftp_fini() -> void __exit {
    static void __exit nf_nat_ftp_fini(void)
    {
    nf_nat_helper_unregister(&nat_helper_ftp);
    RCU_INIT_POINTER(nf_nat_ftp_hook, core::ptr::null_mut());
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn nf_nat_ftp_init() -> int __init {
    static int __init nf_nat_ftp_init(void)
    {
    BUG_ON(nf_nat_ftp_hook != core::ptr::null_mut());
    nf_nat_helper_register(&nat_helper_ftp);
    RCU_INIT_POINTER(nf_nat_ftp_hook, nf_nat_ftp);
    return 0;
    }
// Prior to 2.6.11, we had a ports param.  No longer, but don't break users.
#[no_mangle]
unsafe extern "C" fn warn_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int warn_set(const char *val, const struct kernel_param *kp)
    {
    pr_info("kernel >= 2.6.10 only uses 'ports' for conntrack modules\n");
    return 0;
    }
    module_param_call(ports, warn_set, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    module_init(nf_nat_ftp_init);
    module_exit(nf_nat_ftp_fini);
