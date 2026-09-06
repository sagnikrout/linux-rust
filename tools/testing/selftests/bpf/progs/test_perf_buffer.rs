//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_perf_buffer.c
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
// Copyright (c) 2019 Facebook

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, int);
    __uint(max_entries, 1);
    } my_pid_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __type(key, int);
    __type(value, int);
    } perf_buf_map SEC(".maps");
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter(ctx: *mut c_void) -> c_int {
    int handle_sys_enter(void *ctx)
    {
    let mut zero: c_int = 0, *my_pid, cur_pid;
    let mut cpu: c_int = bpf_get_smp_processor_id();
    my_pid = bpf_map_lookup_elem(&my_pid_map, &zero);
    if (!my_pid)
    return 1;
    cur_pid = bpf_get_current_pid_tgid() >> 32;
    if (cur_pid != *my_pid)
    return 1;
    bpf_perf_event_output(ctx, &perf_buf_map, BPF_F_CURRENT_CPU,
    &cpu, sizeof(cpu));
    return 1;
    }
    char _license[] SEC("license") = "GPL";
