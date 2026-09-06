//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_urandom_usdt.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

    int urand_pid;
    int urand_read_without_sema_call_cnt;
    int urand_read_without_sema_buf_sz_sum;
    SEC("usdt/./urandom_read:urand:read_without_sema")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: urand_read_without_sema, iter_num: c_int, iter_cnt: c_int, buf_sz: c_int) -> c_int {
    int BPF_USDT(urand_read_without_sema, int iter_num, int iter_cnt, int buf_sz)
    {
    if (urand_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&urand_read_without_sema_call_cnt, 1);
    __sync_fetch_and_add(&urand_read_without_sema_buf_sz_sum, buf_sz);
    return 0;
    }
    int urand_read_with_sema_call_cnt;
    int urand_read_with_sema_buf_sz_sum;
    SEC("usdt/./urandom_read:urand:read_with_sema")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: urand_read_with_sema, iter_num: c_int, iter_cnt: c_int, buf_sz: c_int) -> c_int {
    int BPF_USDT(urand_read_with_sema, int iter_num, int iter_cnt, int buf_sz)
    {
    if (urand_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&urand_read_with_sema_call_cnt, 1);
    __sync_fetch_and_add(&urand_read_with_sema_buf_sz_sum, buf_sz);
    return 0;
    }
    int urandlib_read_without_sema_call_cnt;
    int urandlib_read_without_sema_buf_sz_sum;
    SEC("usdt/./liburandom_read.so:urandlib:read_without_sema")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: urandlib_read_without_sema, iter_num: c_int, iter_cnt: c_int, buf_sz: c_int) -> c_int {
    int BPF_USDT(urandlib_read_without_sema, int iter_num, int iter_cnt, int buf_sz)
    {
    if (urand_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&urandlib_read_without_sema_call_cnt, 1);
    __sync_fetch_and_add(&urandlib_read_without_sema_buf_sz_sum, buf_sz);
    return 0;
    }
    int urandlib_read_with_sema_call_cnt;
    int urandlib_read_with_sema_buf_sz_sum;
    SEC("usdt/./liburandom_read.so:urandlib:read_with_sema")
#[no_mangle]
pub unsafe extern "C" fn BPF_USDT(_arg: urandlib_read_with_sema, iter_num: c_int, iter_cnt: c_int, buf_sz: c_int) -> c_int {
    int BPF_USDT(urandlib_read_with_sema, int iter_num, int iter_cnt, int buf_sz)
    {
    if (urand_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    __sync_fetch_and_add(&urandlib_read_with_sema_call_cnt, 1);
    __sync_fetch_and_add(&urandlib_read_with_sema_buf_sz_sum, buf_sz);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
