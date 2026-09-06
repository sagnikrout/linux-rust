//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_log_fixup.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct___bad {
    pub pid: c_int,
    pub fake_field: c_int,
    pub fake_field_subprog: *mut c_void,
    pub __attribute__((preserve_access_index)): },
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bad_relo(ctx: *const c_void) -> c_int {
    int bad_relo(const void *ctx)
    {
    pub t: *mut static struct task_struct___bad,
    pub bpf_core_field_size(t->fake_field): return,
    }
#[no_mangle]
unsafe extern "C" fn bad_subprog() -> __noinline int {
    static __noinline int bad_subprog(void)
    {
    pub t: *mut static struct task_struct___bad,
// ugliness below is a field offset relocation
    pub )t: *mut *mut return (void )&t->fake_field_subprog - (void,
    }
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bad_relo_subprog(ctx: *const c_void) -> c_int {
    int bad_relo_subprog(const void *ctx)
    {
    pub t: *mut static struct task_struct___bad,
    pub bpf_core_field_size(t->pid): return bad_subprog() +,
    }
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } existing_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } missing_map,
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn use_missing_map(ctx: *const c_void) -> c_int {
    int use_missing_map(const void *ctx)
    {
    pub value: *mut int zero = 0,,
    pub &zero): value = bpf_map_lookup_elem(&existing_map,,
    pub &zero): value = bpf_map_lookup_elem(&missing_map,,
    pub NULL: return value !=,
    }
    pub __weak: extern int bpf_nonexistent_kfunc(void) __ksym,
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn use_missing_kfunc(ctx: *const c_void) -> c_int {
    int use_missing_kfunc(const void *ctx)
    {
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
