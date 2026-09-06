//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_sane.c
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
// SANE connection tracking helper
// (SANE = Scanner Access Now Easy)
// For documentation about the SANE network protocol see
// http://www.sane-project.org/html/doc015.html
//
// Copyright (C) 2007 Red Hat, Inc.
// Author: Michal Schmidt <mschmidt@redhat.com>
// Based on the FTP conntrack helper (net/netfilter/nf_conntrack_ftp.c):
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
// (C) 2003,2004 USAGI/WIDE Project <http://www.linux-ipv6.org>
// (C) 2003 Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michal Schmidt <mschmidt@redhat.com>");
    MODULE_DESCRIPTION("SANE connection tracking helper");
    MODULE_ALIAS_NFCT_HELPER(HELPER_NAME);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sane_request {
    pub RPC_code: __be32,

    pub handle: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sane_reply_net_start {
    pub status: __be32,
pub const SANE_STATUS_SUCCESS: c_int = 0;
    pub zero: __be16,
    pub port: __be16,
// other fields aren't interesting for conntrack
}

    static int help(struct sk_buff *skb,
    unsigned int protoff,
    struct nf_conn *ct,
    enum ip_conntrack_info ctinfo)
    {
    unsigned int dataoff, datalen;
    const struct tcphdr *th;
    struct tcphdr _tcph;
    let mut ret: c_int = NF_ACCEPT;
    let mut dir: c_int = CTINFO2DIR(ctinfo);
    struct nf_ct_sane_master *ct_sane_info = nfct_help_data(ct);
    struct nf_conntrack_expect *exp;
    struct nf_conntrack_tuple *tuple;
    struct sane_reply_net_start *reply;
    union {
    struct sane_request req;
    struct sane_reply_net_start repl;
    } buf;
    if (!ct_sane_info)
    return NF_DROP;
// Until there's been traffic both ways, don't look in packets.
    if (ctinfo != IP_CT_ESTABLISHED &&
    ctinfo != IP_CT_ESTABLISHED_REPLY)
    return NF_ACCEPT;
// Not a full tcp header?
    th = skb_header_pointer(skb, protoff, sizeof(_tcph), &_tcph);
    if (th == core::ptr::null_mut())
    return NF_ACCEPT;
// No data?
    dataoff = protoff + th.doff * 4;
    if (dataoff >= skb.len)
    return NF_ACCEPT;
    datalen = skb.len - dataoff;
    if (dir == IP_CT_DIR_ORIGINAL) {
    const struct sane_request *req;
    if (datalen != sizeof(struct sane_request))
    return NF_ACCEPT;
    req = skb_header_pointer(skb, dataoff, datalen, &buf.req);
    if (!req)
    return NF_ACCEPT;
    if (req.RPC_code != htonl(SANE_NET_START)) {
// Not an interesting command
    WRITE_ONCE(ct_sane_info.state, SANE_STATE_NORMAL);
    return NF_ACCEPT;
    }
// We're interested in the next reply
    WRITE_ONCE(ct_sane_info.state, SANE_STATE_START_REQUESTED);
    return NF_ACCEPT;
    }
// IP_CT_DIR_REPLY
// Is it a reply to an uninteresting command?
    if (READ_ONCE(ct_sane_info.state) != SANE_STATE_START_REQUESTED)
    return NF_ACCEPT;
// It's a reply to SANE_NET_START.
    WRITE_ONCE(ct_sane_info.state, SANE_STATE_NORMAL);
    if (datalen < sizeof(struct sane_reply_net_start)) {
    pr_debug("NET_START reply too short\n");
    return NF_ACCEPT;
    }
    datalen = sizeof(struct sane_reply_net_start);
    reply = skb_header_pointer(skb, dataoff, datalen, &buf.repl);
    if (!reply)
    return NF_ACCEPT;
    if (reply.status != htonl(SANE_STATUS_SUCCESS)) {
// saned refused the command
    pr_debug("unsuccessful SANE_STATUS = %u\n",
    ntohl(reply.status));
    return NF_ACCEPT;
    }
// Invalid saned reply? Ignore it.
    if (reply.zero != 0)
    return NF_ACCEPT;
    exp = nf_ct_expect_alloc(ct);
    if (exp == core::ptr::null_mut()) {
    nf_ct_helper_log(skb, ct, "cannot alloc expectation");
    return NF_DROP;
    }
    tuple = &ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple;
    nf_ct_expect_init(exp, NF_CT_EXPECT_CLASS_DEFAULT, nf_ct_l3num(ct),
    &tuple.src.u3, &tuple.dst.u3,
    IPPROTO_TCP, core::ptr::null_mut(), &reply.port);
    pr_debug("expect: ");
    nf_ct_dump_tuple(&exp.tuple);
// Can't expect this?  Best to drop packet now.
    if (nf_ct_expect_related(exp, 0) != 0) {
    nf_ct_helper_log(skb, ct, "cannot add expectation");
    ret = NF_DROP;
    }
    nf_ct_expect_put(exp);
    return ret;
    }
    static struct nf_conntrack_helper sane __read_mostly;
    static struct nf_conntrack_helper *sane_ptr __read_mostly;
    static const struct nf_conntrack_expect_policy sane_exp_policy = {
    .max_expected	= 1,
    .timeout	= 5 * 60,
    };
#[no_mangle]
unsafe extern "C" fn nf_conntrack_sane_fini() -> void __exit {
    static void __exit nf_conntrack_sane_fini(void)
    {
    nf_conntrack_helper_unregister(sane_ptr);
    }
#[no_mangle]
unsafe extern "C" fn nf_conntrack_sane_init() -> int __init {
    static int __init nf_conntrack_sane_init(void)
    {
    let mut ret: c_int = 0;
    NF_CT_HELPER_BUILD_BUG_ON(sizeof(struct nf_ct_sane_master));
    nf_ct_helper_init(&sane, NFPROTO_UNSPEC, IPPROTO_TCP,
    HELPER_NAME,
    &sane_exp_policy, 0, help, core::ptr::null_mut(),
    THIS_MODULE);
    ret = nf_conntrack_helper_register(&sane, &sane_ptr);
    if (ret < 0) {
    pr_err("failed to register helpers\n");
    return ret;
    }
    return 0;
    }
    module_init(nf_conntrack_sane_init);
    module_exit(nf_conntrack_sane_fini);
