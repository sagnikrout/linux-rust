//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ringbuf_n.c
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
// Copyright (c) 2024 Andrea Righi <andrea.righi@canonical.com>

    char _license[] SEC("license") = "GPL";
pub const TASK_COMM_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}

    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    } ringbuf SEC(".maps");
    let mut pid: c_int = 0;
    let mut value: c_long = 0;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf_n(ctx: *mut c_void) -> c_int {
    int test_ringbuf_n(void *ctx)
    {
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    struct sample *sample;
    if (cur_pid != pid)
    return 0;
    sample = bpf_ringbuf_reserve(&ringbuf, sizeof(*sample), 0);
    if (!sample)
    return 0;
    sample.pid = pid;
    sample.value = value;
    bpf_get_current_comm(sample.comm, sizeof(sample.comm));
    bpf_ringbuf_submit(sample, 0);
    return 0;
    }
