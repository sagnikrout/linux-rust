//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_limit.c
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
// (C) 1999 Jérôme de Vivie <devivie@info.enserb.u-bordeaux.fr>
// (C) 1999 Hervé Eychenne <eychenne@info.enserb.u-bordeaux.fr>
// (C) 2006-2012 Patrick McHardy <kaber@trash.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_limit_priv {
    pub prev: c_ulong,
    pub credit: u32,
}

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Herve Eychenne <rv@wallfire.org>");
    MODULE_DESCRIPTION("Xtables: rate-limit match");
    MODULE_ALIAS("ipt_limit");
    MODULE_ALIAS("ip6t_limit");
// The algorithm used is the Simple Token Bucket Filter (TBF)
// see net/sched/sch_tbf.c in the linux source tree
//
// Rusty: This is my (non-mathematically-inclined) understanding of
    this algorithm.  The `average rate' in jiffies becomes your initial
    amount of credit `credit' and the most credit you can ever have
    `credit_cap'.  The `peak rate' becomes the cost of passing the
    test, `cost'.
    `prev' tracks the last packet hit: you gain one credit per jiffy.
    If you get credit balance more than this, the extra credit is
    discarded.  Every time the match passes, you lose `cost' credits;
    if you don't have that many, the test fails.
    See Alexey's formal explanation in net/sched/sch_tbf.c.
    To get the maximum range, we multiply by this factor (ie. you get N
    credits per jiffy).  We want to allow a rate as low as 1 per day
    (slowest userspace tool allows), which means
    CREDITS_PER_JIFFY*HZ*60*60*24 < 2^32. ie. */

// Repeated shift and or gives us all 1s, final shift and add 1 gives
// us the power of 2 below the theoretical max, so GCC simply does a
// shift.

    static bool
    limit_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct xt_rateinfo *r = par.matchinfo;
    struct xt_limit_priv *priv = r.master;
    unsigned long now;
    u32 old_credit, new_credit, credit_increase = 0;
    bool ret;
// fastpath if there is nothing to update
    if ((READ_ONCE(priv.credit) < r.cost) && (READ_ONCE(priv.prev) == jiffies))
    return false;
    do {
    now = jiffies;
    credit_increase += (now - xchg(&priv.prev, now)) * CREDITS_PER_JIFFY;
    old_credit = READ_ONCE(priv.credit);
    new_credit = old_credit;
    new_credit += credit_increase;
    if (new_credit > r.credit_cap)
    new_credit = r.credit_cap;
    if (new_credit >= r.cost) {
    ret = true;
    new_credit -= r.cost;
    } else {
    ret = false;
    }
    } while (cmpxchg(&priv.credit, old_credit, new_credit) != old_credit);
    return ret;
    }
// Precision saver.
#[no_mangle]
unsafe extern "C" fn user2credits(user: u32) -> u32 {
    static u32 user2credits(u32 user)
    {
// If multiplying would overflow...
    if (user > 0xFFFFFFFF / (HZ*CREDITS_PER_JIFFY))
// Divide first.
    return (user / XT_LIMIT_SCALE) * HZ * CREDITS_PER_JIFFY;
    return (user * HZ * CREDITS_PER_JIFFY) / XT_LIMIT_SCALE;
    }
#[no_mangle]
unsafe extern "C" fn limit_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int limit_mt_check(const struct xt_mtchk_param *par)
    {
    struct xt_rateinfo *r = par.matchinfo;
    struct xt_limit_priv *priv;
// Check for overflow.
    if (r.burst == 0
    || user2credits(r.avg * r.burst) < user2credits(r.avg)) {
    pr_info_ratelimited("Overflow, try lower: %u/%u\n",
    r.avg, r.burst);
    return -ERANGE;
    }
    priv = kmalloc_obj(*priv);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
// For SMP, we only want to use one set of state.
    r.master = priv;
// User avg in seconds * XT_LIMIT_SCALE: convert to jiffies
    128. */
    priv.prev = jiffies;
    priv.credit = user2credits(r.avg * r.burst); /* Credits full. */
    if (r.cost == 0) {
    r.credit_cap = priv.credit; /* Credits full. */
    r.cost = user2credits(r.avg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn limit_mt_destroy(par: *const xt_mtdtor_param) {
    static void limit_mt_destroy(const struct xt_mtdtor_param *par)
    {
    const struct xt_rateinfo *info = par.matchinfo;
    kfree(info.master);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_xt_rateinfo {
    pub avg: u_int32_t,
    pub burst: u_int32_t,
    pub prev: compat_ulong_t,
    pub credit: u_int32_t,
    pub cost: u_int32_t credit_cap,,
    pub master: u_int32_t,
}

// To keep the full "prev" timestamp, the upper 32 bits are stored in the
// master pointer, which does not need to be preserved.
#[no_mangle]
unsafe extern "C" fn limit_mt_compat_from_user(dst: *mut c_void, src: *const c_void) {
    static void limit_mt_compat_from_user(void *dst, const void *src)
    {
    const struct compat_xt_rateinfo *cm = src;
    struct xt_rateinfo m = {
    .avg		= cm.avg,
    .burst		= cm.burst,
    .prev		= cm.prev | (unsigned long)cm.master << 32,
    .credit		= cm.credit,
    .credit_cap	= cm.credit_cap,
    .cost		= cm.cost,
    };
    memcpy(dst, &m, sizeof(m));
    }
#[no_mangle]
unsafe extern "C" fn limit_mt_compat_to_user(dst: *mut void __user, src: *const c_void) -> c_int {
    static int limit_mt_compat_to_user(void __user *dst, const void *src)
    {
    const struct xt_rateinfo *m = src;
    struct compat_xt_rateinfo cm = {
    .avg		= m.avg,
    .burst		= m.burst,
    .prev		= m.prev,
    .credit		= m.credit,
    .credit_cap	= m.credit_cap,
    .cost		= m.cost,
    .master		= m.prev >> 32,
    };
    return copy_to_user(dst, &cm, sizeof(cm)) ? -EFAULT : 0;
    }

    static struct xt_match limit_mt_reg __read_mostly = {
    .name             = "limit",
    .revision         = 0,
    .family           = NFPROTO_UNSPEC,
    .match            = limit_mt,
    .checkentry       = limit_mt_check,
    .destroy          = limit_mt_destroy,
    .matchsize        = sizeof(struct xt_rateinfo),

    .compatsize       = sizeof(struct compat_xt_rateinfo),
    .compat_from_user = limit_mt_compat_from_user,
    .compat_to_user   = limit_mt_compat_to_user,

    .usersize         = offsetof(struct xt_rateinfo, prev),
    .me               = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn limit_mt_init() -> int __init {
    static int __init limit_mt_init(void)
    {
    return xt_register_match(&limit_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn limit_mt_exit() -> void __exit {
    static void __exit limit_mt_exit(void)
    {
    xt_unregister_match(&limit_mt_reg);
    }
    module_init(limit_mt_init);
    module_exit(limit_mt_exit);
