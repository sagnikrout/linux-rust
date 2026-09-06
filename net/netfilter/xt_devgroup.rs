//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_devgroup.c
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
// Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
//

    MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Xtables: Device group match");
    MODULE_ALIAS("ipt_devgroup");
    MODULE_ALIAS("ip6t_devgroup");
#[no_mangle]
unsafe extern "C" fn devgroup_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    static bool devgroup_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_devgroup_info *info = par.matchinfo;
    if (info.flags & XT_DEVGROUP_MATCH_SRC &&
    (((info.src_group ^ xt_in(par).group) & info.src_mask ? 1 : 0) ^
    ((info.flags & XT_DEVGROUP_INVERT_SRC) ? 1 : 0)))
    return false;
    if (info.flags & XT_DEVGROUP_MATCH_DST &&
    (((info.dst_group ^ xt_out(par).group) & info.dst_mask ? 1 : 0) ^
    ((info.flags & XT_DEVGROUP_INVERT_DST) ? 1 : 0)))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn devgroup_mt_check_hooks(par: *const xt_mtchk_param) -> c_int {
    static int devgroup_mt_check_hooks(const struct xt_mtchk_param *par)
    {
    const struct xt_devgroup_info *info = par.matchinfo;
    if (info.flags & XT_DEVGROUP_MATCH_SRC &&
    par.hook_mask & ~((1 << NF_INET_PRE_ROUTING) |
    (1 << NF_INET_LOCAL_IN) |
    (1 << NF_INET_FORWARD)))
    return -EINVAL;
    if (info.flags & XT_DEVGROUP_MATCH_DST &&
    par.hook_mask & ~((1 << NF_INET_FORWARD) |
    (1 << NF_INET_LOCAL_OUT) |
    (1 << NF_INET_POST_ROUTING)))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn devgroup_mt_checkentry(par: *const xt_mtchk_param) -> c_int {
    static int devgroup_mt_checkentry(const struct xt_mtchk_param *par)
    {
    const struct xt_devgroup_info *info = par.matchinfo;
    if (info.flags & ~(XT_DEVGROUP_MATCH_SRC | XT_DEVGROUP_INVERT_SRC |
    XT_DEVGROUP_MATCH_DST | XT_DEVGROUP_INVERT_DST))
    return -EINVAL;
    return 0;
    }
    static struct xt_match devgroup_mt_reg __read_mostly = {
    .name		= "devgroup",
    .match		= devgroup_mt,
    .check_hooks	= devgroup_mt_check_hooks,
    .checkentry	= devgroup_mt_checkentry,
    .matchsize	= sizeof(struct xt_devgroup_info),
    .family		= NFPROTO_UNSPEC,
    .me		= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn devgroup_mt_init() -> int __init {
    static int __init devgroup_mt_init(void)
    {
    return xt_register_match(&devgroup_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn devgroup_mt_exit() -> void __exit {
    static void __exit devgroup_mt_exit(void)
    {
    xt_unregister_match(&devgroup_mt_reg);
    }
    module_init(devgroup_mt_init);
    module_exit(devgroup_mt_exit);
