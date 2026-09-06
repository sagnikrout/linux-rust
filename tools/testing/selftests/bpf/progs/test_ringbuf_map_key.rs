//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ringbuf_map_key.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

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
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1000);
    __type(key, struct sample);
    __type(value, int);
    } hash_map SEC(".maps");
// inputs
    let mut pid: c_int = 0;
// inner state
    let mut seq: c_long = 0;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf_mem_map_key(ctx: *mut c_void) -> c_int {
    int test_ringbuf_mem_map_key(void *ctx)
    {
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    struct sample *sample;
    int *lookup_val;
    if (cur_pid != pid)
    return 0;
    sample = bpf_ringbuf_reserve(&ringbuf, sizeof(*sample), 0);
    if (!sample)
    return 0;
    sample.pid = pid;
    bpf_get_current_comm(sample.comm, sizeof(sample.comm));
    sample.seq = ++seq;
    sample.value = 42;
// test using 'sample' (PTR_TO_MEM | MEM_ALLOC) as map key arg
//
    lookup_val = (int *)bpf_map_lookup_elem(&hash_map, sample);
    __sink(lookup_val);
//
// Since bpf_map_lookup_elem above uses 'sample' as key, test using
// sample field as value below
//
    bpf_map_update_elem(&hash_map, sample, &sample.seq, BPF_ANY);
    bpf_ringbuf_submit(sample, 0);
    return 0;
    }
