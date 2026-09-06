//! Automatically rewritten from C to Rust
//! Source: net/sched/em_u32.c
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
// net/sched/em_u32.c	U32 Ematch
//
// Authors:	Thomas Graf <tgraf@suug.ch>
// Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//
// Based on net/sched/cls_u32.c
//

    static int em_u32_match(struct sk_buff *skb, struct tcf_ematch *em,
    struct tcf_pkt_info *info)
    {
    struct tc_u32_key *key = (struct tc_u32_key *) em.data;
    const unsigned char *ptr = skb_network_header(skb);
    if (info) {
    if (info.ptr)
    ptr = info.ptr;
    ptr += (info.nexthdr & key.offmask);
    }
    ptr += key.off;
    if (!tcf_valid_offset(skb, ptr, sizeof(u32)))
    return 0;
    return !(((*(__be32 *) ptr)  ^ key.val) & key.mask);
    }
    static struct tcf_ematch_ops em_u32_ops = {
    .kind	  = TCF_EM_U32,
    .datalen  = sizeof(struct tc_u32_key),
    .match	  = em_u32_match,
    .owner	  = THIS_MODULE,
    .link	  = LIST_HEAD_INIT(em_u32_ops.link)
    };
#[no_mangle]
unsafe extern "C" fn init_em_u32() -> int __init {
    static int __init init_em_u32(void)
    {
    return tcf_em_register(&em_u32_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_em_u32() -> void __exit {
    static void __exit exit_em_u32(void)
    {
    tcf_em_unregister(&em_u32_ops);
    }
    MODULE_DESCRIPTION("ematch skb classifier using 32 bit chunks of data");
    MODULE_LICENSE("GPL");
    module_init(init_em_u32);
    module_exit(exit_em_u32);
    MODULE_ALIAS_TCF_EMATCH(TCF_EM_U32);
