//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/include/utils.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2013, Michael Ellerman, IBM Corp.
//

// Avoid headaches with PRI?64 - just use %ll? always
pub type u64 = c_ulonglong;
pub type s64 = signed long long;
// Just for familiarity
pub type u32 = u32;
pub type u16 = u16;
pub type u8 = u8;
extern "C" {
    pub fn test_harness_set_timeout(time: u64);
}
extern "C" {
    pub fn test_harness((test_function)(void): c_int, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn read_auxv(buf: *mut c_char, buf_size: isize) -> c_int;
}

extern "C" {
    pub fn pick_online_cpu() -> c_int;
}
extern "C" {
    pub fn bind_to_cpu(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn parse_intmax(buffer: *const c_char, count: usize, result: *mut intmax_t, base: c_int) -> c_int;
}
extern "C" {
    pub fn parse_uintmax(buffer: *const c_char, count: usize, result: *mut uintmax_t, base: c_int) -> c_int;
}
extern "C" {
    pub fn parse_int(buffer: *const c_char, count: usize, result: *mut c_int, base: c_int) -> c_int;
}
extern "C" {
    pub fn parse_uint(buffer: *const c_char, count: usize, result: *mut c_uint, base: c_int) -> c_int;
}
extern "C" {
    pub fn parse_long(buffer: *const c_char, count: usize, result: *mut c_long, base: c_int) -> c_int;
}
extern "C" {
    pub fn parse_ulong(buffer: *const c_char, count: usize, result: *mut c_ulong, base: c_int) -> c_int;
}
extern "C" {
    pub fn read_file(path: *const c_char, buf: *mut c_char, count: usize, len: *mut usize) -> c_int;
}
extern "C" {
    pub fn write_file(path: *const c_char, buf: *const c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn read_file_alloc(path: *const c_char, buf: *mut c_char, len: *mut usize) -> c_int;
}
extern "C" {
    pub fn read_long(path: *const c_char, result: *mut c_long, base: c_int) -> c_int;
}
extern "C" {
    pub fn write_long(path: *const c_char, result: c_long, base: c_int) -> c_int;
}
extern "C" {
    pub fn read_ulong(path: *const c_char, result: *mut c_ulong, base: c_int) -> c_int;
}
extern "C" {
    pub fn write_ulong(path: *const c_char, result: c_ulong, base: c_int) -> c_int;
}
extern "C" {
    pub fn read_debugfs_file(debugfs_file: *const c_char, buf: *mut c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn write_debugfs_file(debugfs_file: *const c_char, buf: *const c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn read_debugfs_int(debugfs_file: *const c_char, result: *mut c_int) -> c_int;
}
extern "C" {
    pub fn write_debugfs_int(debugfs_file: *const c_char, result: c_int) -> c_int;
}
extern "C" {
    pub fn read_sysfs_file(debugfs_file: *mut c_char, result: *mut c_char, result_size: usize) -> c_int;
}
extern "C" {
    pub fn perf_event_enable(fd: c_int) -> c_int;
}
extern "C" {
    pub fn perf_event_disable(fd: c_int) -> c_int;
}
extern "C" {
    pub fn perf_event_reset(fd: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_read {
    pub nr: __u64,
    pub l1d_misses: __u64,
}

extern "C" {
    pub fn syscall(_arg: SYS_gettid) -> return;
}

extern "C" {
    pub fn is_ppc64le() -> bool;
}
extern "C" {
    pub fn using_hash_mmu(using_hash: *mut bool) -> c_int;
}
extern "C" {
    pub fn push_signal_handler(sig: c_int, (*fn)(int: *mut c_void, : *mut siginfo_t, ): *mut c_void) -> sigaction;
}
extern "C" {
    pub fn pop_signal_handler(sig: c_int, old_handler: sigaction) -> sigaction;
}
// Yes, this is evil

// The test harness uses this, yes it's gross
pub const MAGIC_SKIP_RETURN_VALUE: c_int = 99;

// POWER9 feature

pub const PPC_FEATURE2_ARCH_3_00: c_uint = 0x00800000;

// POWER10 feature

pub const PPC_FEATURE2_ARCH_3_1: c_uint = 0x00040000;

// POWER10 features

pub const PPC_FEATURE2_MMA: c_uint = 0x00020000;

