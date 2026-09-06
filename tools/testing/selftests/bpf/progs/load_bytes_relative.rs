//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/load_bytes_relative.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2020 Google LLC.
//

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } test_result SEC(".maps");
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_relative(skb: *mut __sk_buff) -> c_int {
    int load_bytes_relative(struct __sk_buff *skb)
    {
    struct ethhdr eth;
    struct iphdr iph;
    let mut map_key: __u32 = 0;
    let mut test_passed: __u32 = 0;
// MAC header is not set by the time cgroup_skb/egress triggers
    if (bpf_skb_load_bytes_relative(skb, 0, &eth, sizeof(eth),
    BPF_HDR_START_MAC) != -EFAULT)
    goto fail;
    if (bpf_skb_load_bytes_relative(skb, 0, &iph, sizeof(iph),
    BPF_HDR_START_NET))
    goto fail;
    if (bpf_skb_load_bytes_relative(skb, 0xffff, &iph, sizeof(iph),
    BPF_HDR_START_NET) != -EFAULT)
    goto fail;
    test_passed = 1;
    fail:
    bpf_map_update_elem(&test_result, &map_key, &test_passed, BPF_ANY);
    return 1;
    }
