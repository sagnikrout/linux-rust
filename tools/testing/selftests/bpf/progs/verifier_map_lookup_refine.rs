//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_map_lookup_refine.c
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } inner_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub inner_map): __array(values, struct,
    } outer_map SEC(".maps") = {
    .values = { [0] = &inner_map },
}

    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=fp": "type=map_ptr_or_null) -> __failure {
    __failure __msg("type=map_ptr_or_null expected=fp")
#[no_mangle]
pub unsafe extern "C" fn mapofmaps_value_as_kfunc_mem_buf(skb: *mut __sk_buff) -> c_int {
    int mapofmaps_value_as_kfunc_mem_buf(struct __sk_buff *skb)
    {
    struct bpf_dynptr dptr;
    let mut key: __u32 = 0;
    void *inner;
    char *p;
    inner = bpf_map_lookup_elem(&outer_map, &key);
// intentionally NOT NULL-checked: type is map_ptr_or_null
    bpf_dynptr_from_skb(skb, 0, &dptr);
// arg3 is mem+size
    p = bpf_dynptr_slice(&dptr, 0, inner, 4);
    if (p)
    return p[0];
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=fp": "type=map_ptr_or_null) -> __failure {
    __failure __msg("type=map_ptr_or_null expected=fp")
#[no_mangle]
pub unsafe extern "C" fn mapofmaps_value_as_helper_mem_buf(skb: *mut __sk_buff) -> c_int {
    int mapofmaps_value_as_helper_mem_buf(struct __sk_buff *skb)
    {
    let mut key: __u32 = 0;
    void *inner;
    inner = bpf_map_lookup_elem(&outer_map, &key);
// intentionally NOT NULL-checked: type is map_ptr_or_null
// arg1 is mem+size
    return bpf_csum_diff(inner, 4, core::ptr::null_mut(), 0, 0) + skb.len;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=fp": "type=map_ptr_or_null) -> __failure {
    __failure __msg("type=map_ptr_or_null expected=fp")
#[no_mangle]
pub unsafe extern "C" fn mapofmaps_value_as_helper_fixed_mem(skb: *mut __sk_buff) -> c_int {
    int mapofmaps_value_as_helper_fixed_mem(struct __sk_buff *skb)
    {
    char th[sizeof(struct tcphdr)] = {};
    let mut key: __u32 = 0;
    void *inner;
    inner = bpf_map_lookup_elem(&outer_map, &key);
// intentionally NOT NULL-checked: type is map_ptr_or_null
// arg1 is fixed-sized mem
    return bpf_tcp_raw_check_syncookie_ipv4(inner, (void *)th);
    }
