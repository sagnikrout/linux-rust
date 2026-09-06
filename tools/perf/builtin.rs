//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/builtin.h
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
pub struct feature_status {
    pub name: *const c_char,
    pub macro: *const c_char,
    pub tip: *const c_char,
    pub is_builtin: c_int,
}

extern "C" {
    pub fn feature_status__printf(feature: *const feature_status);
}
extern "C" {
    pub fn list_common_cmds_help();
}
extern "C" {
    pub fn cmd_annotate(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_bench(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_buildid_cache(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_buildid_list(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_check(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_config(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_c2c(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_diff(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_evlist(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_help(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_sched(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_kallsyms(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_list(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_record(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_report(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_stat(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_timechart(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_top(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_script(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_version(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_probe(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_kmem(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_lock(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_kvm(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_test(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_trace(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_inject(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_mem(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_data(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_ftrace(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_daemon(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn cmd_kwork(argc: c_int, argv: *const c_char) -> c_int;
}
