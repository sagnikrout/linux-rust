//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ringbuf_overwrite.c
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
// Copyright (C) 2025. Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(map_flags, BPF_F_RB_OVERWRITE);
    } ringbuf SEC(".maps");
    int pid;
    const volatile unsigned long LEN1;
    const volatile unsigned long LEN2;
    const volatile unsigned long LEN3;
    const volatile unsigned long LEN4;
    const volatile unsigned long LEN5;
    let mut reserve1_fail: c_long = 0;
    let mut reserve2_fail: c_long = 0;
    let mut reserve3_fail: c_long = 0;
    let mut reserve4_fail: c_long = 0;
    let mut reserve5_fail: c_long = 0;
    let mut avail_data: c_ulong = 0;
    let mut ring_size: c_ulong = 0;
    let mut cons_pos: c_ulong = 0;
    let mut prod_pos: c_ulong = 0;
    let mut over_pos: c_ulong = 0;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_overwrite_ringbuf(ctx: *mut c_void) -> c_int {
    int test_overwrite_ringbuf(void *ctx)
    {
    char *rec1, *rec2, *rec3, *rec4, *rec5;
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    if (cur_pid != pid)
    return 0;
    rec1 = bpf_ringbuf_reserve(&ringbuf, LEN1, 0);
    if (!rec1) {
    reserve1_fail = 1;
    return 0;
    }
    rec2 = bpf_ringbuf_reserve(&ringbuf, LEN2, 0);
    if (!rec2) {
    bpf_ringbuf_discard(rec1, 0);
    reserve2_fail = 1;
    return 0;
    }
    rec3 = bpf_ringbuf_reserve(&ringbuf, LEN3, 0);
// expect failure
    if (!rec3) {
    reserve3_fail = 1;
    } else {
    bpf_ringbuf_discard(rec1, 0);
    bpf_ringbuf_discard(rec2, 0);
    bpf_ringbuf_discard(rec3, 0);
    return 0;
    }
    rec4 = bpf_ringbuf_reserve(&ringbuf, LEN4, 0);
    if (!rec4) {
    reserve4_fail = 1;
    bpf_ringbuf_discard(rec1, 0);
    bpf_ringbuf_discard(rec2, 0);
    return 0;
    }
    bpf_ringbuf_submit(rec1, 0);
    bpf_ringbuf_submit(rec2, 0);
    bpf_ringbuf_submit(rec4, 0);
    rec5 = bpf_ringbuf_reserve(&ringbuf, LEN5, 0);
    if (!rec5) {
    reserve5_fail = 1;
    return 0;
    }
    for (int i = 0; i < LEN3; i++)
    rec5[i] = 0xdd;
    bpf_ringbuf_submit(rec5, 0);
    ring_size = bpf_ringbuf_query(&ringbuf, BPF_RB_RING_SIZE);
    avail_data = bpf_ringbuf_query(&ringbuf, BPF_RB_AVAIL_DATA);
    cons_pos = bpf_ringbuf_query(&ringbuf, BPF_RB_CONS_POS);
    prod_pos = bpf_ringbuf_query(&ringbuf, BPF_RB_PROD_POS);
    over_pos = bpf_ringbuf_query(&ringbuf, BPF_RB_OVERWRITE_POS);
    return 0;
    }
