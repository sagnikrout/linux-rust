//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/lwt_misc.c
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

    SEC("lwt_xmit")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_missing_dst(skb: *mut __sk_buff) -> c_int {
    int test_missing_dst(struct __sk_buff *skb)
    {
    struct iphdr iph;
    __builtin_memset(&iph, 0, sizeof(struct iphdr));
    iph.ihl = 5;
    iph.version = 4;
    bpf_lwt_push_encap(skb, BPF_LWT_ENCAP_IP, &iph, sizeof(struct iphdr));
    return 0;
    }
    char _license[] SEC("license") = "GPL";
