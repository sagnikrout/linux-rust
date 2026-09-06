//! Automatically rewritten from C to Rust
//! Source: net/sched/em_cmp.c
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
// net/sched/em_cmp.c	Simple packet data comparison ematch
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//

#[no_mangle]
pub unsafe extern "C" fn cmp_needs_transformation(cmp: *mut tcf_em_cmp) -> c_int {
    static inline int cmp_needs_transformation(struct tcf_em_cmp *cmp)
    {
    return unlikely(cmp.flags & TCF_EM_CMP_TRANS);
    }
    static int em_cmp_match(struct sk_buff *skb, struct tcf_ematch *em,
    struct tcf_pkt_info *info)
    {
    struct tcf_em_cmp *cmp = (struct tcf_em_cmp *) em.data;
    unsigned char *ptr = tcf_get_base_ptr(skb, cmp.layer);
    let mut val: u32 = 0;
    if (!ptr)
    return 0;
    ptr += cmp.off;
    if (!tcf_valid_offset(skb, ptr, cmp.align))
    return 0;
    switch (cmp.align) {
    case TCF_EM_ALIGN_U8:
    val = *ptr;
    break;
    case TCF_EM_ALIGN_U16:
    val = get_unaligned_be16(ptr);
    if (cmp_needs_transformation(cmp))
    val = be16_to_cpu(val);
    break;
    case TCF_EM_ALIGN_U32:
// Worth checking boundaries? The branching seems
// to get worse. Visit again.
//
    val = get_unaligned_be32(ptr);
    if (cmp_needs_transformation(cmp))
    val = be32_to_cpu(val);
    break;
    default:
    return 0;
    }
    if (cmp.mask)
    val &= cmp.mask;
    switch (cmp.opnd) {
    case TCF_EM_OPND_EQ:
    let mut val: return = = cmp.val;
    case TCF_EM_OPND_LT:
    return val < cmp.val;
    case TCF_EM_OPND_GT:
    return val > cmp.val;
    }
    return 0;
    }
    static struct tcf_ematch_ops em_cmp_ops = {
    .kind	  = TCF_EM_CMP,
    .datalen  = sizeof(struct tcf_em_cmp),
    .match	  = em_cmp_match,
    .owner	  = THIS_MODULE,
    .link	  = LIST_HEAD_INIT(em_cmp_ops.link)
    };
#[no_mangle]
unsafe extern "C" fn init_em_cmp() -> int __init {
    static int __init init_em_cmp(void)
    {
    return tcf_em_register(&em_cmp_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_em_cmp() -> void __exit {
    static void __exit exit_em_cmp(void)
    {
    tcf_em_unregister(&em_cmp_ops);
    }
    MODULE_DESCRIPTION("ematch classifier for basic data types(8/16/32 bit) against skb data");
    MODULE_LICENSE("GPL");
    module_init(init_em_cmp);
    module_exit(exit_em_cmp);
    MODULE_ALIAS_TCF_EMATCH(TCF_EM_CMP);
