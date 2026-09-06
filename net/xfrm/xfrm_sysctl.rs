//! Automatically rewritten from C to Rust
//! Source: net/xfrm/xfrm_sysctl.c
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


// SPDX-License-Identifier: GPL-2.0

#[no_mangle]
unsafe extern "C" fn __xfrm_sysctl_init(net: *mut net) -> void __net_init {
    static void __net_init __xfrm_sysctl_init(struct net *net)
    {
    net.xfrm.sysctl_aevent_etime = XFRM_AE_ETIME;
    net.xfrm.sysctl_aevent_rseqth = XFRM_AE_SEQT_SIZE;
    net.xfrm.sysctl_larval_drop = 1;
    net.xfrm.sysctl_acq_expires = 30;
    }

    static const struct ctl_table xfrm_table[] = {
    {
    .procname	= "xfrm_aevent_etime",
    .maxlen		= sizeof(u32),
    .mode		= 0644,
    .proc_handler	= proc_douintvec
    },
    {
    .procname	= "xfrm_aevent_rseqth",
    .maxlen		= sizeof(u32),
    .mode		= 0644,
    .proc_handler	= proc_douintvec
    },
    {
    .procname	= "xfrm_larval_drop",
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "xfrm_acq_expires",
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    };
#[no_mangle]
pub unsafe extern "C" fn xfrm_sysctl_init(net: *mut net) -> int __net_init {
    int __net_init xfrm_sysctl_init(struct net *net)
    {
    struct ctl_table *table;
    let mut table_size: usize = ARRAY_SIZE(xfrm_table);
    __xfrm_sysctl_init(net);
    table = kmemdup(xfrm_table, sizeof(xfrm_table), GFP_KERNEL);
    if (!table)
    goto out_kmemdup;
    table[0].data = &net.xfrm.sysctl_aevent_etime;
    table[1].data = &net.xfrm.sysctl_aevent_rseqth;
    table[2].data = &net.xfrm.sysctl_larval_drop;
    table[3].data = &net.xfrm.sysctl_acq_expires;
// Don't export sysctls to unprivileged users
    if (net.user_ns != &init_user_ns)
    table_size = 0;
    net.xfrm.sysctl_hdr = register_net_sysctl_sz(net, "net/core", table,
    table_size);
    if (!net.xfrm.sysctl_hdr)
    goto out_register;
    return 0;
    out_register:
    kfree(table);
    out_kmemdup:
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm_sysctl_fini(net: *mut net) -> void __net_exit {
    void __net_exit xfrm_sysctl_fini(struct net *net)
    {
    const struct ctl_table *table;
    table = net.xfrm.sysctl_hdr.ctl_table_arg;
    unregister_net_sysctl_table(net.xfrm.sysctl_hdr);
    kfree(table);
    }

#[no_mangle]
pub unsafe extern "C" fn xfrm_sysctl_init(net: *mut net) -> int __net_init {
    int __net_init xfrm_sysctl_init(struct net *net)
    {
    __xfrm_sysctl_init(net);
    return 0;
    }
