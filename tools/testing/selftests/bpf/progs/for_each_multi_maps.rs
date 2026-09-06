//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/for_each_multi_maps.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 3);
    __type(key, __u32);
    __type(value, __u64);
    } arraymap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 5);
    __type(key, __u32);
    __type(value, __u64);
    } hashmap SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_ctx {
    pub output: c_int,
}

    let mut data_output: u32 = 0;
    let mut use_array: c_int = 0;
    static __u64
    check_map_elem(struct bpf_map *map, __u32 *key, __u64 *val,
    struct callback_ctx *data)
    {
    data.output += *val;
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_pkt_access(skb: *mut __sk_buff) -> c_int {
    int test_pkt_access(struct __sk_buff *skb)
    {
    struct callback_ctx data;
    data.output = 0;
    if (use_array)
    bpf_for_each_map_elem(&arraymap, check_map_elem, &data, 0);
    else
    bpf_for_each_map_elem(&hashmap, check_map_elem, &data, 0);
    data_output = data.output;
    return 0;
    }
