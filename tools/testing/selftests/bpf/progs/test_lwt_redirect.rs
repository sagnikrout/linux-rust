//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_lwt_redirect.c
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

// We don't care about whether the packet can be received by network stack.
// Just care if the packet is sent to the correct device at correct direction
// and not panic the kernel.
//
#[no_mangle]
unsafe extern "C" fn prepend_dummy_mac(skb: *mut __sk_buff) -> c_int {
    static int prepend_dummy_mac(struct __sk_buff *skb)
    {
    char mac[] = {0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0xf,
    0xe, 0xd, 0xc, 0xb, 0xa, 0x08, 0x00};
    if (bpf_skb_change_head(skb, ETH_HLEN, 0))
    return -1;
    if (bpf_skb_store_bytes(skb, 0, mac, sizeof(mac), 0))
    return -1;
    return 0;
    }
// Use the last byte of IP address to redirect the packet
#[no_mangle]
unsafe extern "C" fn get_redirect_target(skb: *mut __sk_buff) -> c_int {
    static int get_redirect_target(struct __sk_buff *skb)
    {
    struct iphdr *iph = core::ptr::null_mut();
    void *start = (void *)(long)skb.data;
    void *end = (void *)(long)skb.data_end;
    if (start + sizeof(*iph) > end)
    return -1;
    iph = (struct iphdr *)start;
    return bpf_ntohl(iph.daddr) & 0xff;
    }
    SEC("redir_ingress")
#[no_mangle]
pub unsafe extern "C" fn test_lwt_redirect_in(skb: *mut __sk_buff) -> c_int {
    int test_lwt_redirect_in(struct __sk_buff *skb)
    {
    let mut target: c_int = get_redirect_target(skb);
    if (target < 0)
    return BPF_OK;
    if (prepend_dummy_mac(skb))
    return BPF_DROP;
    return bpf_redirect(target, BPF_F_INGRESS);
    }
    SEC("redir_egress")
#[no_mangle]
pub unsafe extern "C" fn test_lwt_redirect_out(skb: *mut __sk_buff) -> c_int {
    int test_lwt_redirect_out(struct __sk_buff *skb)
    {
    let mut target: c_int = get_redirect_target(skb);
    if (target < 0)
    return BPF_OK;
    if (prepend_dummy_mac(skb))
    return BPF_DROP;
    return bpf_redirect(target, 0);
    }
    SEC("redir_egress_nomac")
#[no_mangle]
pub unsafe extern "C" fn test_lwt_redirect_out_nomac(skb: *mut __sk_buff) -> c_int {
    int test_lwt_redirect_out_nomac(struct __sk_buff *skb)
    {
    let mut target: c_int = get_redirect_target(skb);
    if (target < 0)
    return BPF_OK;
    return bpf_redirect(target, 0);
    }
    SEC("redir_ingress_nomac")
#[no_mangle]
pub unsafe extern "C" fn test_lwt_redirect_in_nomac(skb: *mut __sk_buff) -> c_int {
    int test_lwt_redirect_in_nomac(struct __sk_buff *skb)
    {
    let mut target: c_int = get_redirect_target(skb);
    if (target < 0)
    return BPF_OK;
    return bpf_redirect(target, BPF_F_INGRESS);
    }
    char _license[] SEC("license") = "GPL";
