//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bench_bpf_timing.h
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const BENCH_NR_SAMPLES: c_int = 4096;

pub const BENCH_NR_CPUS: c_int = 256;

extern "C" {
    pub fn void(ctx: *mut *mut bpf_bench_run_fn)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_bench_timing {
    pub /: *mut *mut *mut __u64 (samples)[BENCH_NR_SAMPLES]; / skel->bss->timing_samples,
    pub /: *mut *mut *mut __u32 idx; / skel->bss->timing_idx,
    pub /: *mut *mut *mut volatile __u32 timing_enabled; / &skel->bss->timing_enabled,
    pub /: *mut *mut *mut volatile __u32 batch_iters_bss; / &skel->bss->batch_iters,
    pub batch_iters: __u32,
    pub target_samples: __u32,
    pub nr_cpus: __u32,
    pub warmup_ticks: c_int,
    pub done: bool,
    pub machine_readable: bool,
}

extern "C" {
    pub fn bpf_bench_timing_measure(t: *mut bpf_bench_timing, res: *mut bench_res);
}
extern "C" {
    pub fn bpf_bench_timing_report(t: *mut bpf_bench_timing, name: *const c_char, desc: *const c_char);
}
extern "C" {
    pub fn bpf_bench_calibrate(t: *mut bpf_bench_timing, run_fn: bpf_bench_run_fn, ctx: *mut c_void);
}
