//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_limit.c
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
// ebt_limit
//
// Authors:
// Tom Marshall <tommy@home.tig-grr.com>
//
// Mostly copied from netfilter's ipt_limit.c, see that file for
// more explanation
//
// September, 2003
//

    static DEFINE_SPINLOCK(limit_lock);

    static bool
    ebt_limit_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct ebt_limit_info *info = (void *)par.matchinfo;
    let mut now: c_ulong = jiffies;
    spin_lock_bh(&limit_lock);
    info.credit += (now - xchg(&info.prev, now)) * CREDITS_PER_JIFFY;
    if (info.credit > info.credit_cap)
    info.credit = info.credit_cap;
    if (info.credit >= info.cost) {
// We're not limited.
    info.credit -= info.cost;
    spin_unlock_bh(&limit_lock);
    return true;
    }
    spin_unlock_bh(&limit_lock);
    return false;
    }
// Precision saver.
    static u_int32_t
    user2credits(u_int32_t user)
    {
// If multiplying would overflow...
    if (user > 0xFFFFFFFF / (HZ*CREDITS_PER_JIFFY))
// Divide first.
    return (user / EBT_LIMIT_SCALE) * HZ * CREDITS_PER_JIFFY;
    return (user * HZ * CREDITS_PER_JIFFY) / EBT_LIMIT_SCALE;
    }
#[no_mangle]
unsafe extern "C" fn ebt_limit_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_limit_mt_check(const struct xt_mtchk_param *par)
    {
    struct ebt_limit_info *info = par.matchinfo;
// Check for overflow.
    if (info.burst == 0 ||
    user2credits(info.avg * info.burst) < user2credits(info.avg)) {
    pr_info_ratelimited("overflow, try lower: %u/%u\n",
    info.avg, info.burst);
    return -EINVAL;
    }
// User avg in seconds * EBT_LIMIT_SCALE: convert to jiffies * 128.
    info.prev = jiffies;
    info.credit = user2credits(info.avg * info.burst);
    info.credit_cap = user2credits(info.avg * info.burst);
    info.cost = user2credits(info.avg);
    return 0;
    }

//
// no conversion function needed --
// only avg/burst have meaningful values in userspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_compat_limit_info {
    pub burst: compat_uint_t avg,,
    pub prev: compat_ulong_t,
    pub cost: compat_uint_t credit, credit_cap,,
}

    static struct xt_match ebt_limit_mt_reg __read_mostly = {
    .name		= "limit",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_limit_mt,
    .checkentry	= ebt_limit_mt_check,
    .matchsize	= sizeof(struct ebt_limit_info),
    .usersize	= offsetof(struct ebt_limit_info, prev),

    .compatsize	= sizeof(struct ebt_compat_limit_info),

    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_limit_init() -> int __init {
    static int __init ebt_limit_init(void)
    {
    return xt_register_match(&ebt_limit_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_limit_fini() -> void __exit {
    static void __exit ebt_limit_fini(void)
    {
    xt_unregister_match(&ebt_limit_mt_reg);
    }
    module_init(ebt_limit_init);
    module_exit(ebt_limit_fini);
    MODULE_DESCRIPTION("Ebtables: Rate-limit match");
    MODULE_LICENSE("GPL");
