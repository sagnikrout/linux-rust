//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_quota.c
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
// netfilter module to enforce network quotas
//
// Sam Johnston <samj@samj.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_quota_priv {
    pub lock: spinlock_t,
    pub quota: u64,
}

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sam Johnston <samj@samj.net>");
    MODULE_DESCRIPTION("Xtables: countdown quota match");
    MODULE_ALIAS("ipt_quota");
    MODULE_ALIAS("ip6t_quota");
    static bool
    quota_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    struct xt_quota_info *q = (void *)par.matchinfo;
    struct xt_quota_priv *priv = q.master;
    let mut ret: bool = q.flags & XT_QUOTA_INVERT;
    spin_lock_bh(&priv.lock);
    if (priv.quota >= skb.len) {
    priv.quota -= skb.len;
    ret = !ret;
    } else {
// we do not allow even small packets from now on
    priv.quota = 0;
    }
    spin_unlock_bh(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn quota_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int quota_mt_check(const struct xt_mtchk_param *par)
    {
    struct xt_quota_info *q = par.matchinfo;
    if (q.flags & ~XT_QUOTA_MASK)
    return -EINVAL;
    q.master = kmalloc_obj(*q.master);
    if (q.master == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&q.master.lock);
    q.master.quota = q.quota;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn quota_mt_destroy(par: *const xt_mtdtor_param) {
    static void quota_mt_destroy(const struct xt_mtdtor_param *par)
    {
    const struct xt_quota_info *q = par.matchinfo;
    kfree(q.master);
    }
    static struct xt_match quota_mt_reg __read_mostly = {
    .name       = "quota",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .match      = quota_mt,
    .checkentry = quota_mt_check,
    .destroy    = quota_mt_destroy,
    .matchsize  = sizeof(struct xt_quota_info),
    .usersize   = offsetof(struct xt_quota_info, master),
    .me         = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn quota_mt_init() -> int __init {
    static int __init quota_mt_init(void)
    {
    return xt_register_match(&quota_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn quota_mt_exit() -> void __exit {
    static void __exit quota_mt_exit(void)
    {
    xt_unregister_match(&quota_mt_reg);
    }
    module_init(quota_mt_init);
    module_exit(quota_mt_exit);
