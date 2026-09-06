//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/perfbuf_bench.c
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
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(value_size, sizeof(int));
    __uint(key_size, sizeof(int));
    } perfbuf SEC(".maps");
    let mut batch_cnt: volatile int = 0;
    let mut sample_val: c_long = 42;
    long dropped __attribute__((aligned(128))) = 0;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn bench_perfbuf(ctx: *mut c_void) -> c_int {
    int bench_perfbuf(void *ctx)
    {
    int i;
    for (i = 0; i < batch_cnt; i++) {
    if (bpf_perf_event_output(ctx, &perfbuf, BPF_F_CURRENT_CPU,
    &sample_val, sizeof(sample_val)))
    __sync_add_and_fetch(&dropped, 1);
    }
    return 0;
    }
