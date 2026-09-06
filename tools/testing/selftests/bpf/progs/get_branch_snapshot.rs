//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/get_branch_snapshot.c
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

    char _license[] SEC("license") = "GPL";
    let mut test1_hits: __u64 = 0;
    let mut address_low: __u64 = 0;
    let mut address_high: __u64 = 0;
    let mut wasted_entries: c_int = 0;
    let mut total_entries: c_long = 0;
pub const ENTRY_CNT: c_int = 32;
    struct perf_branch_entry entries[ENTRY_CNT] = {};
#[no_mangle]
pub unsafe extern "C" fn gbs_in_range(val: __u64) -> bool {
    static inline bool gbs_in_range(__u64 val)
    {
    return (val >= address_low) && (val < address_high);
    }
    SEC("fexit/bpf_testmod_loop_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1, n: c_int, ret: c_int) -> c_int {
    int BPF_PROG(test1, int n, int ret)
    {
    long i;
    total_entries = bpf_get_branch_snapshot(entries, sizeof(entries), 0);
    total_entries /= sizeof(struct perf_branch_entry);
    for (i = 0; i < ENTRY_CNT; i++) {
    if (i >= total_entries)
    break;
    if (gbs_in_range(entries[i].from) && gbs_in_range(entries[i].to))
    test1_hits++;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !test1_hits) -> else {
    else if (!test1_hits)
    wasted_entries++;
    }
    return 0;
    }
