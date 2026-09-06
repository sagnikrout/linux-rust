//! Automatically rewritten from C to Rust
//! Source: net/ipv6/fib6_notifier.c
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

    int call_fib6_notifier(struct notifier_block *nb,
    enum fib_event_type event_type,
    struct fib_notifier_info *info)
    {
    info.family = AF_INET6;
    return call_fib_notifier(nb, event_type, info);
    }
    int call_fib6_notifiers(struct net *net, enum fib_event_type event_type,
    struct fib_notifier_info *info)
    {
    info.family = AF_INET6;
    return call_fib_notifiers(net, event_type, info);
    }
#[no_mangle]
unsafe extern "C" fn fib6_seq_read(net: *const net) -> c_uint {
    static unsigned int fib6_seq_read(const struct net *net)
    {
    return fib6_tables_seq_read(net) + fib6_rules_seq_read(net);
    }
    static int fib6_dump(struct net *net, struct notifier_block *nb,
    struct netlink_ext_ack *extack)
    {
    int err;
    err = fib6_rules_dump(net, nb, extack);
    if (err)
    return err;
    return fib6_tables_dump(net, nb, extack);
    }
    static const struct fib_notifier_ops fib6_notifier_ops_template = {
    .family		= AF_INET6,
    .fib_seq_read	= fib6_seq_read,
    .fib_dump	= fib6_dump,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
pub unsafe extern "C" fn fib6_notifier_init(net: *mut net) -> int __net_init {
    int __net_init fib6_notifier_init(struct net *net)
    {
    struct fib_notifier_ops *ops;
    ops = fib_notifier_ops_register(&fib6_notifier_ops_template, net);
    if (IS_ERR(ops))
    return PTR_ERR(ops);
    net.ipv6.notifier_ops = ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fib6_notifier_exit(net: *mut net) -> void __net_exit {
    void __net_exit fib6_notifier_exit(struct net *net)
    {
    fib_notifier_ops_unregister(net.ipv6.notifier_ops);
    }
