//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_mark_m.c
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
// ebt_mark_m
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// July, 2002
//

    static bool
    ebt_mark_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
    const struct ebt_mark_m_info *info = par.matchinfo;
    if (info.bitmask & EBT_MARK_OR)
    return !!(skb.mark & info.mask) ^ info.invert;
    return ((skb.mark & info.mask) == info.mark) ^ info.invert;
    }
#[no_mangle]
unsafe extern "C" fn ebt_mark_mt_check(par: *const xt_mtchk_param) -> c_int {
    static int ebt_mark_mt_check(const struct xt_mtchk_param *par)
    {
    const struct ebt_mark_m_info *info = par.matchinfo;
    if (info.bitmask & ~EBT_MARK_MASK)
    return -EINVAL;
    if ((info.bitmask & EBT_MARK_OR) && (info.bitmask & EBT_MARK_AND))
    return -EINVAL;
    if (!info.bitmask)
    return -EINVAL;
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ebt_mark_m_info {
    pub mask: compat_ulong_t mark,,
    pub bitmask: uint8_t invert,,
}

#[no_mangle]
unsafe extern "C" fn mark_mt_compat_from_user(dst: *mut c_void, src: *const c_void) {
    static void mark_mt_compat_from_user(void *dst, const void *src)
    {
    const struct compat_ebt_mark_m_info *user = src;
    struct ebt_mark_m_info *kern = dst;
    kern.mark = user.mark;
    kern.mask = user.mask;
    kern.invert = user.invert;
    kern.bitmask = user.bitmask;
    }
#[no_mangle]
unsafe extern "C" fn mark_mt_compat_to_user(dst: *mut void __user, src: *const c_void) -> c_int {
    static int mark_mt_compat_to_user(void __user *dst, const void *src)
    {
    struct compat_ebt_mark_m_info __user *user = dst;
    const struct ebt_mark_m_info *kern = src;
    if (put_user(kern.mark, &user.mark) ||
    put_user(kern.mask, &user.mask) ||
    put_user(kern.invert, &user.invert) ||
    put_user(kern.bitmask, &user.bitmask))
    return -EFAULT;
    return 0;
    }

    static struct xt_match ebt_mark_mt_reg __read_mostly = {
    .name		= "mark_m",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .match		= ebt_mark_mt,
    .checkentry	= ebt_mark_mt_check,
    .matchsize	= sizeof(struct ebt_mark_m_info),

    .compatsize	= sizeof(struct compat_ebt_mark_m_info),
    .compat_from_user = mark_mt_compat_from_user,
    .compat_to_user	= mark_mt_compat_to_user,

    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_mark_m_init() -> int __init {
    static int __init ebt_mark_m_init(void)
    {
    return xt_register_match(&ebt_mark_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_mark_m_fini() -> void __exit {
    static void __exit ebt_mark_m_fini(void)
    {
    xt_unregister_match(&ebt_mark_mt_reg);
    }
    module_init(ebt_mark_m_init);
    module_exit(ebt_mark_m_fini);
    MODULE_DESCRIPTION("Ebtables: Packet mark match");
    MODULE_LICENSE("GPL");
