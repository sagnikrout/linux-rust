//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bench.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_set {
    pub cpus: *mut bool,
    pub cpus_len: c_int,
    pub next_cpu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct env {
    pub bench_name: *mut c_char,
    pub duration_sec: c_int,
    pub warmup_sec: c_int,
    pub verbose: bool,
    pub list: bool,
    pub affinity: bool,
    pub quiet: bool,
    pub stacktrace: bool,
    pub consumer_cnt: c_int,
    pub producer_cnt: c_int,
    pub nr_cpus: c_int,
    pub prod_cpus: cpu_set,
    pub cons_cpus: cpu_set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_stats {
    pub mean: double,
    pub stddev: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bench_res {
    pub hits: c_long,
    pub drops: c_long,
    pub false_hits: c_long,
    pub important_hits: c_long,
    pub gp_ns: c_ulong,
    pub gp_ct: c_ulong,
    pub stime: c_uint,
    pub duration_ns: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub (*validate)(void): *mut c_void,
    pub (*setup)(void): *mut c_void,
    pub ctx): *mut *mut *mut void (producer_thread)(void,
    pub ctx): *mut *mut *mut void (consumer_thread)(void,
    pub res): *mut *mut *mut void (measure)(struct bench_res,
    pub delta_ns): *mut *mut *mut void (report_progress)(int iter, struct bench_res res, long,
    pub res_cnt): *mut *mut void (report_final)(struct bench_res res[], int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter {
    pub value: c_long,
    pub __attribute__((aligned(128))): },
    pub env: extern struct env,
    pub bench: *const extern struct bench,
    pub setup_libbpf(void): c_void,
    pub bench_force_done(void): c_void,
    pub delta_ns): *mut *mut void hits_drops_report_progress(int iter, struct bench_res res, long,
    pub res_cnt): void hits_drops_report_final(struct bench_res res[], int,
    pub delta_ns): *mut *mut void false_hits_report_progress(int iter, struct bench_res res, long,
    pub res_cnt): void false_hits_report_final(struct bench_res res[], int,
    pub delta_ns): *mut *mut void ops_report_progress(int iter, struct bench_res res, long,
    pub res_cnt): void ops_report_final(struct bench_res res[], int,
    pub delta_ns): c_long,
    pub res_cnt): void local_storage_report_final(struct bench_res res[], int,
    pub gp_stat): *mut basic_stats,
    pub gp_stat): *mut basic_stats,
    pub __ATOMIC_RELAXED): (void)__atomic_add_fetch(value, 1,,
    pub __ATOMIC_RELAXED): (void)__atomic_add_fetch(value, n,,
    pub __ATOMIC_RELAXED): return __atomic_exchange_n(value, n,,
