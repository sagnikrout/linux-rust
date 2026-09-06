//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_retro.c
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
// Copyright (c) 2020 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct {
    pub tgid: c_int,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } exp_tgid_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } results,
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter(ctx: *mut c_void) -> c_int {
    int handle_sys_enter(void *ctx)
    {
    pub )bpf_get_current_task(): *mut *mut task_task = (void,
    pub tgid): int tgid = BPF_CORE_READ(task,,
    pub 0: int zero =,
    pub 32: int real_tgid = bpf_get_current_pid_tgid() >>,
    pub &zero): *mut *mut int exp_tgid = bpf_map_lookup_elem(&exp_tgid_map,,
// only pass through sys_enters from test process
    if (!exp_tgid || *exp_tgid != real_tgid)
    pub 0: return,
    pub 0): bpf_map_update_elem(&results, &zero, &tgid,,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
