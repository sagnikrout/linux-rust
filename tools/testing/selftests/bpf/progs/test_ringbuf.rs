//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ringbuf.c
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: c_int,
    pub seq: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}

    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    } ringbuf SEC(".maps");
// inputs
    let mut pid: c_int = 0;
    let mut value: c_long = 0;
    let mut flags: c_long = 0;
// outputs
    let mut total: c_long = 0;
    let mut discarded: c_long = 0;
    let mut dropped: c_long = 0;
    let mut avail_data: c_long = 0;
    let mut ring_size: c_long = 0;
    let mut cons_pos: c_long = 0;
    let mut prod_pos: c_long = 0;
// inner state
    let mut seq: c_long = 0;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf(ctx: *mut c_void) -> c_int {
    int test_ringbuf(void *ctx)
    {
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    struct sample *sample;
    if (cur_pid != pid)
    return 0;
    sample = bpf_ringbuf_reserve(&ringbuf, sizeof(*sample), 0);
    if (!sample) {
    __sync_fetch_and_add(&dropped, 1);
    return 0;
    }
    sample.pid = pid;
    bpf_get_current_comm(sample.comm, sizeof(sample.comm));
    sample.value = value;
    sample.seq = seq++;
    __sync_fetch_and_add(&total, 1);
    if (sample.seq & 1) {
// copy from reserved sample to a new one...
    bpf_ringbuf_output(&ringbuf, sample, sizeof(*sample), flags);
// ...and then discard reserved sample
    bpf_ringbuf_discard(sample, flags);
    __sync_fetch_and_add(&discarded, 1);
    } else {
    bpf_ringbuf_submit(sample, flags);
    }
    avail_data = bpf_ringbuf_query(&ringbuf, BPF_RB_AVAIL_DATA);
    ring_size = bpf_ringbuf_query(&ringbuf, BPF_RB_RING_SIZE);
    cons_pos = bpf_ringbuf_query(&ringbuf, BPF_RB_CONS_POS);
    prod_pos = bpf_ringbuf_query(&ringbuf, BPF_RB_PROD_POS);
    return 0;
    }
