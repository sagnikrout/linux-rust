//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_storage.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, __u64);
    } cgroup_storage SEC(".maps");
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog(skb: *mut __sk_buff) -> c_int {
    int bpf_prog(struct __sk_buff *skb)
    {
    __u64 *counter;
    counter = bpf_get_local_storage(&cgroup_storage, 0);
    __sync_fetch_and_add(counter, 1);
// Drop one out of every two packets
    return (*counter & 1);
    }
// Maps for OOB test
    struct {
    __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, __u32);  /* 4-byte value - not 8-byte aligned */
    } cgroup_storage_oob SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_PERCPU_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);  /* 4-byte value - same as cgroup storage */
    } lru_map SEC(".maps");
    SEC("cgroup/sock_create")
#[no_mangle]
pub unsafe extern "C" fn trigger_oob(sk: *mut bpf_sock) -> c_int {
    int trigger_oob(struct bpf_sock *sk)
    {
    let mut key: __u32 = 0;
    __u32 *cgroup_val;
    let mut value: __u32 = 0x12345678;
// Get cgroup storage value
    cgroup_val = bpf_get_local_storage(&cgroup_storage_oob, 0);
    if (!cgroup_val)
    return 0;
// Initialize cgroup storage
// cgroup_val = value;
// This triggers the OOB read:
// bpf_map_update_elem() -> htab_map_update_elem() ->
// pcpu_init_value() -> copy_map_value_long() ->
// bpf_obj_memcpy(..., long_memcpy=true) ->
// bpf_long_memcpy(dst, src, round_up(4, 8))
//
// The copy size is rounded up to 8 bytes, but cgroup_val
// points to a 4-byte buffer, causing a 4-byte OOB read.
//
    bpf_map_update_elem(&lru_map, &key, cgroup_val, BPF_ANY);
    return 1;
    }
    char _license[] SEC("license") = "GPL";
