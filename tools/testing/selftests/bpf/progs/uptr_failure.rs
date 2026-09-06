//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uptr_failure.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct value_type);
    } datamap SEC(".maps");
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "store to uptr) -> __failure {
    __failure __msg("store to uptr disallowed")
#[no_mangle]
pub unsafe extern "C" fn uptr_write(ctx: *const c_void) -> c_int {
    int uptr_write(const void *ctx)
    {
    struct task_struct *task;
    struct value_type *v;
    task = bpf_get_current_task_btf();
    v = bpf_task_storage_get(&datamap, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!v)
    return 0;
    v.udata = core::ptr::null_mut();
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "store to uptr) -> __failure {
    __failure __msg("store to uptr disallowed")
#[no_mangle]
pub unsafe extern "C" fn uptr_write_nested(ctx: *const c_void) -> c_int {
    int uptr_write_nested(const void *ctx)
    {
    struct task_struct *task;
    struct value_type *v;
    task = bpf_get_current_task_btf();
    v = bpf_task_storage_get(&datamap, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!v)
    return 0;
    v.nested.udata = core::ptr::null_mut();
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg('mem_or_null'": "R1 invalid mem access) -> __failure {
    __failure __msg("R1 invalid mem access 'mem_or_null'")
#[no_mangle]
pub unsafe extern "C" fn uptr_no_null_check(ctx: *const c_void) -> c_int {
    int uptr_no_null_check(const void *ctx)
    {
    struct task_struct *task;
    struct value_type *v;
    task = bpf_get_current_task_btf();
    v = bpf_task_storage_get(&datamap, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!v)
    return 0;
    v.udata.result = 0;
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(kptr": "doesn't point to) -> __failure {
    __failure __msg("doesn't point to kptr")
#[no_mangle]
pub unsafe extern "C" fn uptr_kptr_xchg(ctx: *const c_void) -> c_int {
    int uptr_kptr_xchg(const void *ctx)
    {
    struct task_struct *task;
    struct value_type *v;
    task = bpf_get_current_task_btf();
    v = bpf_task_storage_get(&datamap, task, 0,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!v)
    return 0;
    bpf_kptr_xchg(&v.udata, core::ptr::null_mut());
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg('scalar'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn uptr_obj_new(ctx: *const c_void) -> c_int {
    int uptr_obj_new(const void *ctx)
    {
    struct value_type *v;
    v = bpf_obj_new(typeof(*v));
    if (!v)
    return 0;
    if (v.udata)
    v.udata.result = 0;
    bpf_obj_drop(v);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
