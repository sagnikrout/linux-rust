//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_statistic.c
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
// Copyright (c) 2006 Patrick McHardy <kaber@trash.net>
//
// Based on ipt_random and ipt_nth by Fabrice MARIE <fabrice@netfilter.org>.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_statistic_priv {
    pub count: core::sync::atomic::AtomicI32,
    pub ____cacheline_aligned_in_smp: },
    pub <kaber@trash.net>"): MODULE_AUTHOR("Patrick McHardy,
    pub random)"): MODULE_DESCRIPTION("Xtables: statistics-based matching (\"Nth\",,
    static bool
    statistic_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    pub par->matchinfo: *const *const xt_statistic_info info =,
    pub XT_STATISTIC_INVERT: bool ret = info->flags &,
    pub oval: int nval,,
    switch (info.mode) {
    case XT_STATISTIC_MODE_RANDOM:
    if ((get_random_u32() & 0x7FFFFFFF) < info.u.random.probability)
    pub !ret: ret =,
    case XT_STATISTIC_MODE_NTH:
    do {
    pub atomic_read(&info->master->count): oval =,
    pub 1: nval = (oval == info->u.nth.every) ? 0 : oval +,
    pub oval): } while (atomic_cmpxchg(&info->master->count, oval, nval) !=,
    if (nval == 0)
    pub !ret: ret =,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn statistic_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int statistic_mt_check(const struct xt_mtchk_param *par)
    {
    pub par->matchinfo: *mut *mut xt_statistic_info info =,
    if (info.mode > XT_STATISTIC_MODE_MAX ||
    info.flags & ~XT_STATISTIC_MASK)
    pub -EINVAL: return,
    pub kzalloc_obj(*info->master): *mut info->master =,
    if (info.master == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub info->u.nth.count): atomic_set(&info->master->count,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn statistic_mt_destroy(par: *const xt_mtdtor_param) {
    static void statistic_mt_destroy(const struct xt_mtdtor_param *par)
    {
    pub par->matchinfo: *const *const xt_statistic_info info =,
    }
    static struct xt_match xt_statistic_mt_reg __read_mostly = {
    .name       = "statistic",
    .revision   = 0,
    .family     = NFPROTO_UNSPEC,
    .match      = statistic_mt,
    .checkentry = statistic_mt_check,
    .destroy    = statistic_mt_destroy,
    .matchsize  = sizeof(struct xt_statistic_info),
    .usersize   = offsetof(struct xt_statistic_info, master),
    .me         = THIS_MODULE,
}

#[no_mangle]
unsafe extern "C" fn statistic_mt_init() -> int __init {
    static int __init statistic_mt_init(void)
    {
    return xt_register_match(&xt_statistic_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn statistic_mt_exit() -> void __exit {
    static void __exit statistic_mt_exit(void)
    {
    xt_unregister_match(&xt_statistic_mt_reg);
    }
    module_init(statistic_mt_init);
    module_exit(statistic_mt_exit);
