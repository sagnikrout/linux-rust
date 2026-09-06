//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ringbuf_multi.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ringbuf_map {
    pub BPF_MAP_TYPE_RINGBUF): __uint(type,,
// libbpf will adjust to valid page size
    pub 1000): __uint(max_entries,,
    } ringbuf1 SEC(".maps"),
    pub SEC(".maps"): ringbuf2,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 4): __uint(max_entries,,
    pub int): __type(key,,
    pub ringbuf_map): __array(values, struct,
    } ringbuf_arr SEC(".maps") = {
    .values = {
    [0] = &ringbuf1,
    [2] = &ringbuf2,
    },
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
    __uint(max_entries, 1);
    __type(key, int);
    __array(values, struct ringbuf_map);
    } ringbuf_hash SEC(".maps") = {
    .values = {
    [0] = &ringbuf1,
    },
    };
// inputs
    let mut pid: c_int = 0;
    let mut target_ring: c_int = 0;
    let mut value: c_long = 0;
// outputs
    let mut total: c_long = 0;
    let mut dropped: c_long = 0;
    let mut skipped: c_long = 0;
    SEC("tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf(ctx: *mut c_void) -> c_int {
    int test_ringbuf(void *ctx)
    {
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    struct sample *sample;
    void *rb;
    if (cur_pid != pid)
    return 0;
    rb = bpf_map_lookup_elem(&ringbuf_arr, &target_ring);
    if (!rb) {
    skipped += 1;
    return 1;
    }
    sample = bpf_ringbuf_reserve(rb, sizeof(*sample), 0);
    if (!sample) {
    dropped += 1;
    return 1;
    }
    sample.pid = pid;
    bpf_get_current_comm(sample.comm, sizeof(sample.comm));
    sample.value = value;
    sample.seq = total;
    total += 1;
    bpf_ringbuf_submit(sample, 0);
    return 0;
    }
