//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/task_io_accounting.h
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
// task_io_accounting: a structure which is used for recording a single task's
// IO statistics.
//
// Don't include this header file directly - it is designed to be dragged in via
// sched.h.
//
// Blame Andrew Morton for all this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_io_accounting {

// bytes read
    pub rchar: u64,
// bytes written
    pub wchar: u64,
// # of read syscalls
    pub syscr: u64,
// # of write syscalls
    pub syscw: u64,

//
// The number of bytes which this task has caused to be read from
// storage.
//
    pub read_bytes: u64,
//
// The number of bytes which this task has caused, or shall cause to be
// written to disk.
//
    pub write_bytes: u64,
//
// A task can cause "negative" IO too.  If this task truncates some
// dirty pagecache, some IO which another task has been accounted for
// (in its write_bytes) will not be happening.  We _could_ just
// subtract that from the truncating task's write_bytes, but there is
// information loss in doing that.
//
    pub cancelled_write_bytes: u64,

}
