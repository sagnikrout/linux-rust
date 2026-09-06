//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_change_tail.c
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
// Copyright (c) 2024 ByteDance

    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } sock_map_rx SEC(".maps");
    let mut change_tail_ret: c_long = 1;
    SEC("sk_skb")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    int prog_skb_verdict(struct __sk_buff *skb)
    {
    char *data, *data_end;
    bpf_skb_pull_data(skb, 1);
    data = (char *)(unsigned long)skb.data;
    data_end = (char *)(unsigned long)skb.data_end;
    if (data + 1 > data_end)
    return SK_PASS;
    if (data[0] == 'T') { /* Trim the packet */
    change_tail_ret = bpf_skb_change_tail(skb, skb.len - 1, 0);
    return SK_PASS;
    } else if (data[0] == 'G') { /* Grow the packet */
    change_tail_ret = bpf_skb_change_tail(skb, skb.len + 1, 0);
    return SK_PASS;
    } else if (data[0] == 'E') { /* Error */
    change_tail_ret = bpf_skb_change_tail(skb, BPF_SKB_MAX_LEN, 0);
    return SK_PASS;
    }
    return SK_PASS;
    }
    char _license[] SEC("license") = "GPL";
