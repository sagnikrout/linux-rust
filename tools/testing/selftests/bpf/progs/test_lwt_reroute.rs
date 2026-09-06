//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_lwt_reroute.c
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

// This function extracts the last byte of the daddr, and uses it
// as output dev index.
//
    SEC("lwt_xmit")
#[no_mangle]
pub unsafe extern "C" fn test_lwt_reroute(skb: *mut __sk_buff) -> c_int {
    int test_lwt_reroute(struct __sk_buff *skb)
    {
    struct iphdr *iph = core::ptr::null_mut();
    void *start = (void *)(long)skb.data;
    void *end = (void *)(long)skb.data_end;
// set mark at most once
    if (skb.mark != 0)
    return BPF_OK;
    if (start + sizeof(*iph) > end)
    return BPF_DROP;
    iph = (struct iphdr *)start;
    skb.mark = bpf_ntohl(iph.daddr) & 0xff;
// do not reroute x.x.x.0 packets
    if (skb.mark == 0)
    return BPF_OK;
    return BPF_LWT_REROUTE;
    }
    char _license[] SEC("license") = "GPL";
