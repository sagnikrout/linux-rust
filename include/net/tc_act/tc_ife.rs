//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_ife.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ife_params {
    pub eth_dst: [u8; ETH_ALEN],
    pub eth_src: [u8; ETH_ALEN],
    pub eth_type: u16,
    pub flags: u16,
    pub metalist: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ife_info {
    pub common: tc_action,
    pub params: *mut tcf_ife_params __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_meta_info {
    pub ops: *const tcf_meta_ops,
    pub metaval: *mut c_void,
    pub metaid: u16,
    pub metalist: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_meta_ops {
    pub /: *mut *mut u16 metaid; /Maintainer provided ID,
    pub /: *mut *mut u16 metatype; /netlink attribute type (look at net/netlink.h),
    pub name: *const c_char,
    pub synopsis: *const c_char,
    pub list: list_head,
    pub ): *mut *mut *mut int (check_presence)(struct sk_buff , struct tcf_meta_info,
    pub ): *mut *mut *mut *mut int (encode)(struct sk_buff , void , struct tcf_meta_info,
    pub len): *mut *mut *mut *mut int (decode)(struct sk_buff , void , u16,
    pub mi): *mut *mut *mut int (get)(struct sk_buff skb, struct tcf_meta_info,
    pub gfp_t): *mut *mut *mut *mut int (alloc)(struct tcf_meta_info , void ,,
    pub ): *mut *mut void (release)(struct tcf_meta_info,
    pub len): *mut *mut *mut int (validate)(void val, int,
    pub owner: *mut module,
}

extern "C" {
    pub fn ife_get_meta_u32(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> c_int;
}
extern "C" {
    pub fn ife_get_meta_u16(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> c_int;
}
extern "C" {
    pub fn ife_alloc_meta_u32(mi: *mut tcf_meta_info, metaval: *mut c_void, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn ife_alloc_meta_u16(mi: *mut tcf_meta_info, metaval: *mut c_void, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn ife_check_meta_u32(metaval: u32, mi: *mut tcf_meta_info) -> c_int;
}
extern "C" {
    pub fn ife_check_meta_u16(metaval: u16, mi: *mut tcf_meta_info) -> c_int;
}
extern "C" {
    pub fn ife_encode_meta_u32(metaval: u32, skbdata: *mut c_void, mi: *mut tcf_meta_info) -> c_int;
}
extern "C" {
    pub fn ife_validate_meta_u32(val: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn ife_validate_meta_u16(val: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn ife_encode_meta_u16(metaval: u16, skbdata: *mut c_void, mi: *mut tcf_meta_info) -> c_int;
}
extern "C" {
    pub fn ife_release_meta_gen(mi: *mut tcf_meta_info);
}
extern "C" {
    pub fn register_ife_op(mops: *mut tcf_meta_ops) -> c_int;
}
extern "C" {
    pub fn unregister_ife_op(mops: *mut tcf_meta_ops) -> c_int;
}
