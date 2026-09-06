//! Automatically rewritten from C to Rust
//! Source: net/sched/act_meta_skbprio.c
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
// net/sched/act_meta_prio.c IFE skb->priority metadata module
//
// copyright Jamal Hadi Salim (2015)
//

#[no_mangle]
unsafe extern "C" fn skbprio_check(skb: *mut sk_buff, e: *mut tcf_meta_info) -> c_int {
    static int skbprio_check(struct sk_buff *skb, struct tcf_meta_info *e)
    {
    return ife_check_meta_u32(skb.priority, e);
    }
    static int skbprio_encode(struct sk_buff *skb, void *skbdata,
    struct tcf_meta_info *e)
    {
    u32 ifeprio = skb.priority; /* avoid having to cast skb.priority*/
    return ife_encode_meta_u32(ifeprio, skbdata, e);
    }
#[no_mangle]
unsafe extern "C" fn skbprio_decode(skb: *mut sk_buff, data: *mut c_void, len: u16) -> c_int {
    static int skbprio_decode(struct sk_buff *skb, void *data, u16 len)
    {
    let mut ifeprio: u32 = *(u32 *)data;
    skb.priority = ntohl(ifeprio);
    return 0;
    }
    static struct tcf_meta_ops ife_prio_ops = {
    .metaid = IFE_META_PRIO,
    .metatype = NLA_U32,
    .name = "skbprio",
    .synopsis = "skb prio metadata",
    .check_presence = skbprio_check,
    .encode = skbprio_encode,
    .decode = skbprio_decode,
    .get = ife_get_meta_u32,
    .alloc = ife_alloc_meta_u32,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ifeprio_init_module() -> int __init {
    static int __init ifeprio_init_module(void)
    {
    return register_ife_op(&ife_prio_ops);
    }
#[no_mangle]
unsafe extern "C" fn ifeprio_cleanup_module() -> void __exit {
    static void __exit ifeprio_cleanup_module(void)
    {
    unregister_ife_op(&ife_prio_ops);
    }
    module_init(ifeprio_init_module);
    module_exit(ifeprio_cleanup_module);
    MODULE_AUTHOR("Jamal Hadi Salim(2015)");
    MODULE_DESCRIPTION("Inter-FE skb prio metadata action");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_IFE_META("skbprio");
