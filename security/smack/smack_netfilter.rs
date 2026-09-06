//! Automatically rewritten from C to Rust
//! Source: security/smack/smack_netfilter.c
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
// Simplified MAC Kernel (smack) security module
//
// This file contains the Smack netfilter implementation
//
// Author:
// Casey Schaufler <casey@schaufler-ca.com>
//
// Copyright (C) 2014 Casey Schaufler <casey@schaufler-ca.com>
// Copyright (C) 2014 Intel Corporation.
//

    static unsigned int smack_ip_output(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct sock *sk = skb_to_full_sk(skb);
    struct socket_smack *ssp;
    struct smack_known *skp;
    if (sk) {
    ssp = smack_sock(sk);
    skp = ssp.smk_out;
    skb.secmark = skp.smk_secid;
    }
    return NF_ACCEPT;
    }
    static const struct nf_hook_ops smack_nf_ops[] = {
    {
    .hook =		smack_ip_output,
    .pf =		NFPROTO_IPV4,
    .hooknum =	NF_INET_LOCAL_OUT,
    .priority =	NF_IP_PRI_SELINUX_FIRST,
    },

    {
    .hook =		smack_ip_output,
    .pf =		NFPROTO_IPV6,
    .hooknum =	NF_INET_LOCAL_OUT,
    .priority =	NF_IP6_PRI_SELINUX_FIRST,
    },

    };
#[no_mangle]
unsafe extern "C" fn smack_nf_register(net: *mut net) -> int __net_init {
    static int __net_init smack_nf_register(struct net *net)
    {
    return nf_register_net_hooks(net, smack_nf_ops,
    ARRAY_SIZE(smack_nf_ops));
    }
#[no_mangle]
unsafe extern "C" fn smack_nf_unregister(net: *mut net) -> void __net_exit {
    static void __net_exit smack_nf_unregister(struct net *net)
    {
    nf_unregister_net_hooks(net, smack_nf_ops, ARRAY_SIZE(smack_nf_ops));
    }
    static struct pernet_operations smack_net_ops = {
    .init = smack_nf_register,
    .exit = smack_nf_unregister,
    };
#[no_mangle]
pub unsafe extern "C" fn smack_nf_ip_init() -> int __init {
    int __init smack_nf_ip_init(void)
    {
    if (smack_enabled == 0)
    return 0;
    printk(KERN_DEBUG "Smack: Registering netfilter hooks\n");
    return register_pernet_subsys(&smack_net_ops);
    }
