//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_map_ops.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } hash_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_STACK);
    __uint(max_entries, 1);
    __type(value, int);
    } stack_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } array_map SEC(".maps");
    const volatile pid_t pid;
    let mut err: c_long = 0;
#[no_mangle]
unsafe extern "C" fn callback(map: u64, key: u64, val: u64, ctx: u64, flags: u64) -> u64 {
    static u64 callback(u64 map, u64 key, u64 val, u64 ctx, u64 flags)
    {
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getpid")
#[no_mangle]
pub unsafe extern "C" fn map_update(ctx: *mut c_void) -> c_int {
    int map_update(void *ctx)
    {
    let mut key: c_int = 0;
    let mut val: c_int = 1;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    err = bpf_map_update_elem(&hash_map, &key, &val, BPF_NOEXIST);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getppid")
#[no_mangle]
pub unsafe extern "C" fn map_delete(ctx: *mut c_void) -> c_int {
    int map_delete(void *ctx)
    {
    let mut key: c_int = 0;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    err = bpf_map_delete_elem(&hash_map, &key);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getuid")
#[no_mangle]
pub unsafe extern "C" fn map_push(ctx: *mut c_void) -> c_int {
    int map_push(void *ctx)
    {
    let mut val: c_int = 1;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    err = bpf_map_push_elem(&stack_map, &val, 0);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_geteuid")
#[no_mangle]
pub unsafe extern "C" fn map_pop(ctx: *mut c_void) -> c_int {
    int map_pop(void *ctx)
    {
    int val;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    err = bpf_map_pop_elem(&stack_map, &val);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getgid")
#[no_mangle]
pub unsafe extern "C" fn map_peek(ctx: *mut c_void) -> c_int {
    int map_peek(void *ctx)
    {
    int val;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    err = bpf_map_peek_elem(&stack_map, &val);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_gettid")
#[no_mangle]
pub unsafe extern "C" fn map_for_each_pass(ctx: *mut c_void) -> c_int {
    int map_for_each_pass(void *ctx)
    {
    let mut key: c_int = 0;
    let mut val: c_int = 1;
    let mut flags: u64 = 0;
    int callback_ctx;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    bpf_map_update_elem(&array_map, &key, &val, flags);
    err = bpf_for_each_map_elem(&array_map, callback, &callback_ctx, flags);
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn map_for_each_fail(ctx: *mut c_void) -> c_int {
    int map_for_each_fail(void *ctx)
    {
    let mut key: c_int = 0;
    let mut val: c_int = 1;
    let mut flags: u64 = BPF_NOEXIST;
    int callback_ctx;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    bpf_map_update_elem(&array_map, &key, &val, flags);
// calling for_each with non-zero flags will return error
    err = bpf_for_each_map_elem(&array_map, callback, &callback_ctx, flags);
    return 0;
    }
