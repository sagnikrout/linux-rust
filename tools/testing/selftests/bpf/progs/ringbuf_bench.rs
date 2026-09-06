//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/ringbuf_bench.c
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
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    } ringbuf SEC(".maps");
    let mut batch_cnt: volatile int = 0;
    let mut use_output: volatile long = 0;
    let mut bench_producer: volatile bool = false;
    let mut sample_val: c_long = 42;
    long dropped __attribute__((aligned(128))) = 0;
    long hits __attribute__((aligned(128))) = 0;
    let mut wakeup_data_size: volatile long = 0;
#[no_mangle]
unsafe extern "C" fn get_flags() -> __always_inline long {
    static __always_inline long get_flags()
    {
    long sz;
    if (bench_producer)
    return BPF_RB_NO_WAKEUP;
    if (!wakeup_data_size)
    return 0;
    sz = bpf_ringbuf_query(&ringbuf, BPF_RB_AVAIL_DATA);
    return sz >= wakeup_data_size ? BPF_RB_FORCE_WAKEUP : BPF_RB_NO_WAKEUP;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn bench_ringbuf(ctx: *mut c_void) -> c_int {
    int bench_ringbuf(void *ctx)
    {
    long *sample, flags;
    int i;
    if (!use_output) {
    for (i = 0; i < batch_cnt; i++) {
    sample = bpf_ringbuf_reserve(&ringbuf,
    sizeof(sample_val), 0);
    if (!sample) {
    __sync_add_and_fetch(&dropped, 1);
    } else {
// sample = sample_val;
    flags = get_flags();
    bpf_ringbuf_submit(sample, flags);
    if (bench_producer)
    __sync_add_and_fetch(&hits, 1);
    }
    }
    } else {
    for (i = 0; i < batch_cnt; i++) {
    flags = get_flags();
    if (bpf_ringbuf_output(&ringbuf, &sample_val,
    sizeof(sample_val), flags))
    __sync_add_and_fetch(&dropped, 1);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: bench_producer) -> else {
    else if (bench_producer)
    __sync_add_and_fetch(&hits, 1);
    }
    }
    return 0;
    }
