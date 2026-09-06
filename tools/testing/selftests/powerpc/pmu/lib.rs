//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/pmu/lib.h
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
// Copyright 2014, Michael Ellerman, IBM Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe {
    pub read_fd: c_int,
    pub write_fd: c_int,
}

extern "C" {
    pub fn kill_child_and_wait(child_pid: pid_t) -> c_int;
}
extern "C" {
    pub fn wait_for_child(child_pid: pid_t) -> c_int;
}
extern "C" {
    pub fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
}
extern "C" {
    pub fn wait_for_parent(read_pipe: pipe) -> c_int;
}
extern "C" {
    pub fn notify_parent(write_pipe: pipe) -> c_int;
}
extern "C" {
    pub fn notify_parent_of_error(write_pipe: pipe) -> c_int;
}
extern "C" {
    pub fn eat_cpu((test_function)(void): c_int) -> pid_t;
}
extern "C" {
    pub fn require_paranoia_below(level: c_int) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_range {
    pub last: uint64_t first,,
}

extern "C" {
    pub fn parse_proc_maps() -> c_int;
}
