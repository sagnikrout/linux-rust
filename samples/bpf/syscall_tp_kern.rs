//! Automatically rewritten from C to Rust
//! Source: samples/bpf/syscall_tp_kern.c
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
// Copyright (c) 2017 Facebook
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscalls_enter_open_args {
    pub unused: c_ulonglong,
    pub syscall_nr: c_long,
    pub filename_ptr: c_long,
    pub flags: c_long,
    pub mode: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscalls_exit_open_args {
    pub unused: c_ulonglong,
    pub syscall_nr: c_long,
    pub ret: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscalls_enter_open_at_args {
    pub unused: c_ulonglong,
    pub syscall_nr: c_long,
    pub dfd: c_longlong,
    pub filename_ptr: c_long,
    pub flags: c_long,
    pub mode: c_long,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u32);
    __uint(max_entries, 1);
    } enter_open_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u32);
    __uint(max_entries, 1);
    } exit_open_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn count(map: *mut c_void) -> __always_inline void {
    static __always_inline void count(void *map)
    {
    let mut key: u32 = 0;
    u32 *value, init_val = 1;
    value = bpf_map_lookup_elem(map, &key);
    if (value)
// value += 1;
    else
    bpf_map_update_elem(map, &key, &init_val, BPF_NOEXIST);
    }

    SEC("tracepoint/syscalls/sys_enter_open")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_open(ctx: *mut syscalls_enter_open_args) -> c_int {
    int trace_enter_open(struct syscalls_enter_open_args *ctx)
    {
    count(&enter_open_map);
    return 0;
    }

    SEC("tracepoint/syscalls/sys_enter_openat")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_open_at(ctx: *mut syscalls_enter_open_at_args) -> c_int {
    int trace_enter_open_at(struct syscalls_enter_open_at_args *ctx)
    {
    count(&enter_open_map);
    return 0;
    }
    SEC("tracepoint/syscalls/sys_enter_openat2")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_open_at2(ctx: *mut syscalls_enter_open_at_args) -> c_int {
    int trace_enter_open_at2(struct syscalls_enter_open_at_args *ctx)
    {
    count(&enter_open_map);
    return 0;
    }

    SEC("tracepoint/syscalls/sys_exit_open")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_exit(ctx: *mut syscalls_exit_open_args) -> c_int {
    int trace_enter_exit(struct syscalls_exit_open_args *ctx)
    {
    count(&exit_open_map);
    return 0;
    }

    SEC("tracepoint/syscalls/sys_exit_openat")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_exit_at(ctx: *mut syscalls_exit_open_args) -> c_int {
    int trace_enter_exit_at(struct syscalls_exit_open_args *ctx)
    {
    count(&exit_open_map);
    return 0;
    }
    SEC("tracepoint/syscalls/sys_exit_openat2")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_exit_at2(ctx: *mut syscalls_exit_open_args) -> c_int {
    int trace_enter_exit_at2(struct syscalls_exit_open_args *ctx)
    {
    count(&exit_open_map);
    return 0;
    }
