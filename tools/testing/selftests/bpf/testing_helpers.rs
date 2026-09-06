//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/testing_helpers.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (C) 2020 Facebook, Inc.

extern "C" {
    pub fn parse_num_list(s: *const c_char, set: *mut bool, set_len: *mut c_int) -> c_int;
}
extern "C" {
    pub fn link_info_prog_id(link: *const bpf_link, info: *mut bpf_link_info) -> __u32;
}
//
// below function is exported for testing in prog_test test
//
extern "C" {
    pub fn read_perf_max_sample_freq() -> __u64;
}
extern "C" {
    pub fn load_bpf_testmod(verbose: bool) -> c_int;
}
extern "C" {
    pub fn unload_bpf_testmod(verbose: bool) -> c_int;
}
extern "C" {
    pub fn kern_sync_rcu() -> c_int;
}
extern "C" {
    pub fn finit_module(fd: c_int, param_values: *const c_char, flags: c_int) -> c_int;
}
extern "C" {
    pub fn delete_module(name: *const c_char, flags: c_int) -> c_int;
}
extern "C" {
    pub fn load_module(path: *const c_char, verbose: bool) -> c_int;
}
extern "C" {
    pub fn load_module_params(path: *const c_char, param_values: *const c_char, verbose: bool) -> c_int;
}
extern "C" {
    pub fn try_unload_module(name: *const c_char, retries: c_int, verbose: bool) -> c_int;
}
extern "C" {
    pub fn unload_module(name: *const c_char, verbose: bool) -> c_int;
}
// Request BPF program instructions after all rewrites are applied,
// e.g. verifier.c:convert_ctx_access() is done.
//
extern "C" {
    pub fn get_xlated_program(fd_prog: c_int, buf: *mut bpf_insn, cnt: *mut __u32) -> c_int;
}
extern "C" {
    pub fn testing_prog_flags() -> c_int;
}
extern "C" {
    pub fn is_jit_enabled() -> bool;
}
extern "C" {
    pub fn stack_mprotect() -> c_int;
}
