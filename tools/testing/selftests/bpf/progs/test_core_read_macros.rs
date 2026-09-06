//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_read_macros.c
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

    char _license[] SEC("license") = "GPL";
// shuffled layout for relocatable (CO-RE) reads
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_head___shuffled {
    pub func: Option<unsafe extern "C" fn()>,
    pub next: *mut callback_head___shuffled,
}

    let mut k_probe_in: callback_head = {};
    let mut k_core_in: callback_head___shuffled = {};
    struct callback_head *u_probe_in = 0;
    struct callback_head___shuffled *u_core_in = 0;
    let mut k_probe_out: c_long = 0;
    let mut u_probe_out: c_long = 0;
    let mut k_core_out: c_long = 0;
    let mut u_core_out: c_long = 0;
    let mut my_pid: c_int = 0;
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler(ctx: *mut c_void) -> c_int {
    int handler(void *ctx)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
    if (my_pid != pid)
    return 0;
// next pointers for kernel address space have to be initialized from
// BPF side, user-space mmaped addresses are still user-space addresses
//
    k_probe_in.next = &k_probe_in;
    __builtin_preserve_access_index(({k_core_in.next = &k_core_in;}));
    k_probe_out = (long)BPF_PROBE_READ(&k_probe_in, next, next, func);
    k_core_out = (long)BPF_CORE_READ(&k_core_in, next, next, func);
    u_probe_out = (long)BPF_PROBE_READ_USER(u_probe_in, next, next, func);
    u_core_out = (long)BPF_CORE_READ_USER(u_core_in, next, next, func);
    return 0;
    }
