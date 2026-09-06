//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xfrm_info.c
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
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xfrm_info___local {
    pub if_id: u32,
    pub link: c_int,
    pub __attribute__((preserve_access_index)): },
    pub req_if_id: __u32,
    pub resp_if_id: __u32,
    int bpf_skb_set_xfrm_info(struct __sk_buff *skb_ctx,
    pub __ksym: *const *const bpf_xfrm_info___local from),
    int bpf_skb_get_xfrm_info(struct __sk_buff *skb_ctx,
    pub __ksym: *mut *mut bpf_xfrm_info___local to),
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn set_xfrm_info(skb: *mut __sk_buff) -> c_int {
    int set_xfrm_info(struct __sk_buff *skb)
    {
    pub }: bpf_xfrm_info___local info = { .if_id = req_if_id,
    pub TC_ACT_UNSPEC: return bpf_skb_set_xfrm_info(skb, &info) ? TC_ACT_SHOT :,
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn get_xfrm_info(skb: *mut __sk_buff) -> c_int {
    int get_xfrm_info(struct __sk_buff *skb)
    {
    pub {}: bpf_xfrm_info___local info =,
    if (bpf_skb_get_xfrm_info(skb, &info) < 0)
    pub TC_ACT_SHOT: return,
    pub info.if_id: resp_if_id =,
    pub TC_ACT_UNSPEC: return,
    }
    pub "GPL": char _license[] SEC("license") =,
