//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_kfunc_param_nullable.c
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
// Copyright (c) 2024 Meta Platforms, Inc

    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_dynptr_nullable_test1(skb: *mut __sk_buff) -> c_int {
    int kfunc_dynptr_nullable_test1(struct __sk_buff *skb)
    {
    struct bpf_dynptr data;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_kfunc_dynptr_test(&data, core::ptr::null_mut());
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_dynptr_nullable_test2(skb: *mut __sk_buff) -> c_int {
    int kfunc_dynptr_nullable_test2(struct __sk_buff *skb)
    {
    struct bpf_dynptr data;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_kfunc_dynptr_test(&data, &data);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn kfunc_dynptr_nullable_test3(skb: *mut __sk_buff) -> c_int {
    int kfunc_dynptr_nullable_test3(struct __sk_buff *skb)
    {
    struct bpf_dynptr data;
    bpf_dynptr_from_skb(skb, 0, &data);
    bpf_kfunc_dynptr_test(core::ptr::null_mut(), &data);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
