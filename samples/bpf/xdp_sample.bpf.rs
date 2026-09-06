//! Automatically rewritten from C Header to Rust Module
//! Source: samples/bpf/xdp_sample.bpf.h
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

pub const EINVAL: c_int = 22;
pub const ENETDOWN: c_int = 100;
pub const EMSGSIZE: c_int = 90;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOSPC: c_int = 28;
//
// Note: including linux/compiler.h or linux/kernel.h for the macros below
// conflicts with vmlinux.h include in BPF files, so we define them here.
//
// Following functions are taken from kernel sources and
// break aliasing rules in their original form.
//
// While kernel is compiled with -fno-strict-aliasing,
// perf uses -Wstrict-aliasing=3 which makes build fail
// under gcc 4.4.
//
// Using extra __may_alias__ type to allow aliasing
// in this case.
//
extern "C" {
    pub fn volatile("memory": "" : : :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "" : : :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "" : : :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "" : : :) -> asm;
}

// Add a value using relaxed read and relaxed write. Less expensive than
// fetch_add when there is no write concurrency.
//

