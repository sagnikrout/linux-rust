//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_legacy_printk.c
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
// Copyright (c) 2021 Facebook

// Macro flag: #define BPF_NO_GLOBAL_DATA

    char LICENSE[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, int);
    __uint(max_entries, 1);
    } my_pid_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, int);
    __uint(max_entries, 1);
    } res_map SEC(".maps");
    let mut my_pid_var: volatile int = 0;
    let mut res_var: volatile int = 0;
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_legacy(ctx: *mut c_void) -> c_int {
    int handle_legacy(void *ctx)
    {
    let mut zero: c_int = 0, *my_pid, cur_pid, *my_res;
    my_pid = bpf_map_lookup_elem(&my_pid_map, &zero);
    if (!my_pid)
    return 1;
    cur_pid = bpf_get_current_pid_tgid() >> 32;
    if (cur_pid != *my_pid)
    return 1;
    my_res = bpf_map_lookup_elem(&res_map, &zero);
    if (!my_res)
    return 1;
    if (*my_res == 0)
// use bpf_printk() in combination with BPF_NO_GLOBAL_DATA to
// force .rodata.str1.1 section that previously caused
// problems on old kernels due to libbpf always tried to
// create a global data map for it
//
    bpf_printk("Legacy-case bpf_printk test, pid %d\n", cur_pid);
// my_res = 1;
    return *my_res;
    }
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle_modern(ctx: *mut c_void) -> c_int {
    int handle_modern(void *ctx)
    {
    int cur_pid;
    cur_pid = bpf_get_current_pid_tgid() >> 32;
    if (cur_pid != my_pid_var)
    return 1;
    if (res_var == 0)
// we need bpf_printk() to validate libbpf logic around unused
// global maps and legacy kernels; see comment in handle_legacy()
//
    bpf_printk("Modern-case bpf_printk test, pid %d\n", cur_pid);
    res_var = 1;
    return res_var;
    }
