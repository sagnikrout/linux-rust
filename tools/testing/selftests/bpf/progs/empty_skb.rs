//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/empty_skb.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

    char _license[] SEC("license") = "GPL";
    int ifindex;
    int ret;
    SEC("lwt_xmit")
#[no_mangle]
pub unsafe extern "C" fn redirect_ingress(skb: *mut __sk_buff) -> c_int {
    int redirect_ingress(struct __sk_buff *skb)
    {
    ret = bpf_clone_redirect(skb, ifindex, BPF_F_INGRESS);
    return 0;
    }
    SEC("lwt_xmit")
#[no_mangle]
pub unsafe extern "C" fn redirect_egress(skb: *mut __sk_buff) -> c_int {
    int redirect_egress(struct __sk_buff *skb)
    {
    ret = bpf_clone_redirect(skb, ifindex, 0);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_redirect_ingress(skb: *mut __sk_buff) -> c_int {
    int tc_redirect_ingress(struct __sk_buff *skb)
    {
    ret = bpf_clone_redirect(skb, ifindex, BPF_F_INGRESS);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_redirect_egress(skb: *mut __sk_buff) -> c_int {
    int tc_redirect_egress(struct __sk_buff *skb)
    {
    ret = bpf_clone_redirect(skb, ifindex, 0);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_adjust_room(skb: *mut __sk_buff) -> c_int {
    int tc_adjust_room(struct __sk_buff *skb)
    {
    ret = bpf_skb_adjust_room(skb, 4, BPF_ADJ_ROOM_NET, 0);
    return 0;
    }
