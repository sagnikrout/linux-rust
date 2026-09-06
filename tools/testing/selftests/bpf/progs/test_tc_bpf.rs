//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_bpf.c
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

// Dummy prog to test TC-BPF API
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn cls(skb: *mut __sk_buff) -> c_int {
    int cls(struct __sk_buff *skb)
    {
    return 0;
    }
// Prog to verify tc-bpf without cap_sys_admin and cap_perfmon
    SEC("tcx/ingress")
#[no_mangle]
pub unsafe extern "C" fn pkt_ptr(skb: *mut __sk_buff) -> c_int {
    int pkt_ptr(struct __sk_buff *skb)
    {
    struct iphdr *iph = (void *)(long)skb.data + sizeof(struct ethhdr);
    if ((long)(iph + 1) > (long)skb.data_end)
    return 1;
    return 0;
    }
