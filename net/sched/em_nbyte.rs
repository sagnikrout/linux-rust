//! Automatically rewritten from C to Rust
//! Source: net/sched/em_nbyte.c
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
// net/sched/em_nbyte.c	N-Byte ematch
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbyte_data {
    pub hdr: tcf_em_nbyte,
    pub pattern: [c_char; ],
}

    static int em_nbyte_change(struct net *net, void *data, int data_len,
    struct tcf_ematch *em)
    {
    struct tcf_em_nbyte *nbyte = data;
    if (data_len < sizeof(*nbyte) ||
    data_len < (sizeof(*nbyte) + nbyte.len))
    return -EINVAL;
    em.datalen = sizeof(*nbyte) + nbyte.len;
    em.data = (unsigned long)kmemdup(data, em.datalen, GFP_KERNEL);
    if (em.data == 0UL)
    return -ENOMEM;
    return 0;
    }
    static int em_nbyte_match(struct sk_buff *skb, struct tcf_ematch *em,
    struct tcf_pkt_info *info)
    {
    struct nbyte_data *nbyte = (struct nbyte_data *) em.data;
    unsigned char *ptr = tcf_get_base_ptr(skb, nbyte.hdr.layer);
    if (!ptr)
    return 0;
    ptr += nbyte.hdr.off;
    if (!tcf_valid_offset(skb, ptr, nbyte.hdr.len))
    return 0;
    return !memcmp(ptr, nbyte.pattern, nbyte.hdr.len);
    }
    static struct tcf_ematch_ops em_nbyte_ops = {
    .kind	  = TCF_EM_NBYTE,
    .change	  = em_nbyte_change,
    .match	  = em_nbyte_match,
    .owner	  = THIS_MODULE,
    .link	  = LIST_HEAD_INIT(em_nbyte_ops.link)
    };
#[no_mangle]
unsafe extern "C" fn init_em_nbyte() -> int __init {
    static int __init init_em_nbyte(void)
    {
    return tcf_em_register(&em_nbyte_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_em_nbyte() -> void __exit {
    static void __exit exit_em_nbyte(void)
    {
    tcf_em_unregister(&em_nbyte_ops);
    }
    MODULE_DESCRIPTION("ematch classifier for arbitrary skb multi-bytes");
    MODULE_LICENSE("GPL");
    module_init(init_em_nbyte);
    module_exit(exit_em_nbyte);
    MODULE_ALIAS_TCF_EMATCH(TCF_EM_NBYTE);
