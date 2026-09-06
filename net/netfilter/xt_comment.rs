//! Automatically rewritten from C to Rust
//! Source: net/netfilter/xt_comment.c
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
// Implements a dummy match to allow attaching comments to rules
//
// 2003-05-13 Brad Fisher (brad@info-link.net)
//

    MODULE_AUTHOR("Brad Fisher <brad@info-link.net>");
    MODULE_DESCRIPTION("Xtables: No-op match which can be tagged with a comment");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ipt_comment");
    MODULE_ALIAS("ip6t_comment");
    static bool
    comment_mt(const struct sk_buff *skb, struct xt_action_param *par)
    {
// We always match
    return true;
    }
    static struct xt_match comment_mt_reg __read_mostly = {
    .name      = "comment",
    .revision  = 0,
    .family    = NFPROTO_UNSPEC,
    .match     = comment_mt,
    .matchsize = sizeof(struct xt_comment_info),
    .me        = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn comment_mt_init() -> int __init {
    static int __init comment_mt_init(void)
    {
    return xt_register_match(&comment_mt_reg);
    }
#[no_mangle]
unsafe extern "C" fn comment_mt_exit() -> void __exit {
    static void __exit comment_mt_exit(void)
    {
    xt_unregister_match(&comment_mt_reg);
    }
    module_init(comment_mt_init);
    module_exit(comment_mt_exit);
