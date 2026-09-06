//! Automatically rewritten from C to Rust
//! Source: net/bridge/netfilter/ebt_mark.c
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
// ebt_mark
//
// Authors:
// Bart De Schuymer <bdschuym@pandora.be>
//
// July, 2002
//
// The mark target can be used in any chain,
// I believe adding a mangle table just for marking is total overkill.
// Marking a frame doesn't really change anything in the frame anyway.
//

    static unsigned int
    ebt_mark_tg(struct sk_buff *skb, const struct xt_action_param *par)
    {
    const struct ebt_mark_t_info *info = par.targinfo;
    let mut action: c_int = info.target & -16;
    if (action == MARK_SET_VALUE)
    skb.mark = info.mark;
#[no_mangle]
pub unsafe extern "C" fn if(MARK_OR_VALUE: action ==) -> else {
    else if (action == MARK_OR_VALUE)
    skb.mark |= info.mark;
#[no_mangle]
pub unsafe extern "C" fn if(MARK_AND_VALUE: action ==) -> else {
    else if (action == MARK_AND_VALUE)
    skb.mark &= info.mark;
    else
    skb.mark ^= info.mark;
    return info.target | ~EBT_VERDICT_BITS;
    }
#[no_mangle]
unsafe extern "C" fn ebt_mark_tg_check(par: *const xt_tgchk_param) -> c_int {
    static int ebt_mark_tg_check(const struct xt_tgchk_param *par)
    {
    const struct ebt_mark_t_info *info = par.targinfo;
    int tmp;
    tmp = info.target | ~EBT_VERDICT_BITS;
    if (BASE_CHAIN && tmp == EBT_RETURN)
    return -EINVAL;
    if (ebt_invalid_target(tmp))
    return -EINVAL;
    tmp = info.target & ~EBT_VERDICT_BITS;
    if (tmp != MARK_SET_VALUE && tmp != MARK_OR_VALUE &&
    tmp != MARK_AND_VALUE && tmp != MARK_XOR_VALUE)
    return -EINVAL;
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ebt_mark_t_info {
    pub mark: compat_ulong_t,
    pub target: compat_uint_t,
}

#[no_mangle]
unsafe extern "C" fn mark_tg_compat_from_user(dst: *mut c_void, src: *const c_void) {
    static void mark_tg_compat_from_user(void *dst, const void *src)
    {
    const struct compat_ebt_mark_t_info *user = src;
    struct ebt_mark_t_info *kern = dst;
    kern.mark = user.mark;
    kern.target = user.target;
    }
#[no_mangle]
unsafe extern "C" fn mark_tg_compat_to_user(dst: *mut void __user, src: *const c_void) -> c_int {
    static int mark_tg_compat_to_user(void __user *dst, const void *src)
    {
    struct compat_ebt_mark_t_info __user *user = dst;
    const struct ebt_mark_t_info *kern = src;
    if (put_user(kern.mark, &user.mark) ||
    put_user(kern.target, &user.target))
    return -EFAULT;
    return 0;
    }

    static struct xt_target ebt_mark_tg_reg __read_mostly = {
    .name		= "mark",
    .revision	= 0,
    .family		= NFPROTO_BRIDGE,
    .target		= ebt_mark_tg,
    .checkentry	= ebt_mark_tg_check,
    .targetsize	= sizeof(struct ebt_mark_t_info),

    .compatsize	= sizeof(struct compat_ebt_mark_t_info),
    .compat_from_user = mark_tg_compat_from_user,
    .compat_to_user	= mark_tg_compat_to_user,

    .me		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ebt_mark_init() -> int __init {
    static int __init ebt_mark_init(void)
    {
    return xt_register_target(&ebt_mark_tg_reg);
    }
#[no_mangle]
unsafe extern "C" fn ebt_mark_fini() -> void __exit {
    static void __exit ebt_mark_fini(void)
    {
    xt_unregister_target(&ebt_mark_tg_reg);
    }
    module_init(ebt_mark_init);
    module_exit(ebt_mark_fini);
    MODULE_DESCRIPTION("Ebtables: Packet mark modification");
    MODULE_LICENSE("GPL");
