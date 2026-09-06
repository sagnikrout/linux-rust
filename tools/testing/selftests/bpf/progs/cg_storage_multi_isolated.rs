//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cg_storage_multi_isolated.c
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
    __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, struct cgroup_value);
    } cgroup_storage SEC(".maps");
    let mut invocations: __u32 = 0;
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn egress1(skb: *mut __sk_buff) -> c_int {
    int egress1(struct __sk_buff *skb)
    {
    struct cgroup_value *ptr_cg_storage =
    bpf_get_local_storage(&cgroup_storage, 0);
    __sync_fetch_and_add(&ptr_cg_storage.egress_pkts, 1);
    __sync_fetch_and_add(&invocations, 1);
    return 1;
    }
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn egress2(skb: *mut __sk_buff) -> c_int {
    int egress2(struct __sk_buff *skb)
    {
    struct cgroup_value *ptr_cg_storage =
    bpf_get_local_storage(&cgroup_storage, 0);
    __sync_fetch_and_add(&ptr_cg_storage.egress_pkts, 1);
    __sync_fetch_and_add(&invocations, 1);
    return 1;
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn ingress(skb: *mut __sk_buff) -> c_int {
    int ingress(struct __sk_buff *skb)
    {
    struct cgroup_value *ptr_cg_storage =
    bpf_get_local_storage(&cgroup_storage, 0);
    __sync_fetch_and_add(&ptr_cg_storage.ingress_pkts, 1);
    __sync_fetch_and_add(&invocations, 1);
    return 1;
    }
