//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_proto_generic.c
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
// (C) 1999-2001 Paul `Rusty' Russell
// (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
//

    let mut nf_ct_generic_timeout: static unsigned int = 600*HZ;

    static int generic_timeout_nlattr_to_obj(struct nlattr *tb[],
    struct net *net, void *data)
    {
    struct nf_generic_net *gn = nf_generic_pernet(net);
    unsigned int *timeout = data;
    if (!timeout)
    timeout = &gn.timeout;
    if (tb[CTA_TIMEOUT_GENERIC_TIMEOUT])
// timeout =
    ntohl(nla_get_be32(tb[CTA_TIMEOUT_GENERIC_TIMEOUT])) * HZ;
    else {
// Set default generic timeout.
// timeout = gn->timeout;
    }
    return 0;
    }
    static int
    generic_timeout_obj_to_nlattr(struct sk_buff *skb, const void *data)
    {
    const unsigned int *timeout = data;
    if (nla_put_be32(skb, CTA_TIMEOUT_GENERIC_TIMEOUT, htonl(*timeout / HZ)))
    goto nla_put_failure;
    return 0;
    nla_put_failure:
    return -ENOSPC;
    }
    static const struct nla_policy
    generic_timeout_nla_policy[CTA_TIMEOUT_GENERIC_MAX+1] = {
    [CTA_TIMEOUT_GENERIC_TIMEOUT]	= { .type = NLA_U32 },
    };

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_generic_init_net(net: *mut net) {
    void nf_conntrack_generic_init_net(struct net *net)
    {
    struct nf_generic_net *gn = nf_generic_pernet(net);
    gn.timeout = nf_ct_generic_timeout;
    }
    const struct nf_conntrack_l4proto nf_conntrack_l4proto_generic =
    {
    .l4proto		= 255,
    .allow_clash            = true,

    .ctnl_timeout		= {
    .nlattr_to_obj	= generic_timeout_nlattr_to_obj,
    .obj_to_nlattr	= generic_timeout_obj_to_nlattr,
    .nlattr_max	= CTA_TIMEOUT_GENERIC_MAX,
    .obj_size	= sizeof(unsigned int),
    .nla_policy	= generic_timeout_nla_policy,
    },

    };
