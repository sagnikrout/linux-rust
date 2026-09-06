//! Automatically rewritten from C to Rust
//! Source: net/unix/sysctl_net_unix.c
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
// NET4:	Sysctl interface to net af_unix subsystem.
//
// Authors:	Mike Shaver.
//

    static const struct ctl_table unix_table[] = {
    {
    .procname	= "max_dgram_qlen",
    .data		= &init_net.unx.sysctl_max_dgram_qlen,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    };
    static const struct ctl_table *unix_table_dup(struct net *net)
    {
    struct ctl_table *table;
    table = kmemdup(unix_table, sizeof(unix_table), GFP_KERNEL);
    if (!table)
    return core::ptr::null_mut();
    table[0].data = &net.unx.sysctl_max_dgram_qlen;
    return table;
    }
#[no_mangle]
pub unsafe extern "C" fn unix_sysctl_register(net: *mut net) -> int __net_init {
    int __net_init unix_sysctl_register(struct net *net)
    {
    const struct ctl_table *table;
    if (net_eq(net, &init_net)) {
    table = unix_table;
    } else {
    table = unix_table_dup(net);
    if (!table)
    goto err_alloc;
    }
    net.unx.ctl = register_net_sysctl_sz(net, "net/unix", table,
    ARRAY_SIZE(unix_table));
    if (net.unx.ctl == core::ptr::null_mut())
    goto err_reg;
    return 0;
    err_reg:
    if (!net_eq(net, &init_net))
    kfree(table);
    err_alloc:
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn unix_sysctl_unregister(net: *mut net) {
    void unix_sysctl_unregister(struct net *net)
    {
    const struct ctl_table *table;
    table = net.unx.ctl.ctl_table_arg;
    unregister_net_sysctl_table(net.unx.ctl);
    if (!net_eq(net, &init_net))
    kfree(table);
    }
