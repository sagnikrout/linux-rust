//! Automatically rewritten from C to Rust
//! Source: net/sched/act_meta_skbtcindex.c
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
// net/sched/act_meta_tc_index.c IFE skb->tc_index metadata module
//
// copyright Jamal Hadi Salim (2016)
//

    static int skbtcindex_encode(struct sk_buff *skb, void *skbdata,
    struct tcf_meta_info *e)
    {
    let mut ifetc_index: u32 = skb.tc_index;
    return ife_encode_meta_u16(ifetc_index, skbdata, e);
    }
#[no_mangle]
unsafe extern "C" fn skbtcindex_decode(skb: *mut sk_buff, data: *mut c_void, len: u16) -> c_int {
    static int skbtcindex_decode(struct sk_buff *skb, void *data, u16 len)
    {
    let mut ifetc_index: u16 = *(u16 *)data;
    skb.tc_index = ntohs(ifetc_index);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skbtcindex_check(skb: *mut sk_buff, e: *mut tcf_meta_info) -> c_int {
    static int skbtcindex_check(struct sk_buff *skb, struct tcf_meta_info *e)
    {
    return ife_check_meta_u16(skb.tc_index, e);
    }
    static struct tcf_meta_ops ife_skbtcindex_ops = {
    .metaid = IFE_META_TCINDEX,
    .metatype = NLA_U16,
    .name = "tc_index",
    .synopsis = "skb tc_index 16 bit metadata",
    .check_presence = skbtcindex_check,
    .encode = skbtcindex_encode,
    .decode = skbtcindex_decode,
    .get = ife_get_meta_u16,
    .alloc = ife_alloc_meta_u16,
    .release = ife_release_meta_gen,
    .validate = ife_validate_meta_u16,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ifetc_index_init_module() -> int __init {
    static int __init ifetc_index_init_module(void)
    {
    return register_ife_op(&ife_skbtcindex_ops);
    }
#[no_mangle]
unsafe extern "C" fn ifetc_index_cleanup_module() -> void __exit {
    static void __exit ifetc_index_cleanup_module(void)
    {
    unregister_ife_op(&ife_skbtcindex_ops);
    }
    module_init(ifetc_index_init_module);
    module_exit(ifetc_index_cleanup_module);
    MODULE_AUTHOR("Jamal Hadi Salim(2016)");
    MODULE_DESCRIPTION("Inter-FE skb tc_index metadata module");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_IFE_META("tcindex");
