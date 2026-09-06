//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/bench/bench.h
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

//
// The madvise transparent hugepage constants were added in glibc
// 2.13. For compatibility with older versions of glibc, define these
// tokens if they are not already defined.
//

extern "C" {
    pub fn bench_numa(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_sched_messaging(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_sched_pipe(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_sched_seccomp_notify(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_syscall_basic(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_syscall_getpgid(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_syscall_fork(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_syscall_execve(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_mem_memcpy(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_mem_memset(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_mem_mmap(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_mem_find_bit(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_futex_hash(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_futex_wake(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_futex_wake_parallel(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_futex_requeue(argc: c_int, argv: *const c_char) -> c_int;
}
// pi futexes
extern "C" {
    pub fn bench_futex_lock_pi(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_epoll_wait(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_epoll_ctl(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_synthesize(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_kallsyms_parse(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_inject_build_id(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_evlist_open_close(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_breakpoint_thread(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_breakpoint_enable(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_uprobe_baseline(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_uprobe_empty(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_uprobe_trace_printk(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_uprobe_empty_ret(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_uprobe_trace_printk_ret(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn bench_pmu_scan(argc: c_int, argv: *const c_char) -> c_int;
}

pub const BENCH_FORMAT_DEFAULT: c_int = 0;

pub const BENCH_FORMAT_SIMPLE: c_int = 1;

