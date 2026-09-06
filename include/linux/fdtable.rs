//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fdtable.h
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
// descriptor table internals; you almost certainly want file.h instead.
//

//
// The default fd array needs to be at least BITS_PER_LONG,
// as this is the granularity returned by copy_fdset().
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdtable {
    pub max_fds: c_uint,
    pub /: *mut *mut *mut *mut file __rcu fd; / current fd array,
    pub close_on_exec: *mut c_ulong,
    pub open_fds: *mut c_ulong,
    pub full_fds_bits: *mut c_ulong,
    pub rcu: rcu_head,
}

//
// Open file table structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct files_struct {
//
// read mostly part
//
    pub count: core::sync::atomic::AtomicI32,
    pub resize_in_progress: bool,
    pub resize_wait: wait_queue_head_t,
    pub fdt: *mut fdtable __rcu,
    pub fdtab: fdtable,
//
// written part on a separate cache line in SMP
//
    pub ____cacheline_aligned_in_smp: spinlock_t file_lock,
    pub next_fd: c_uint,
    pub close_on_exec_init: [c_ulong; 1],
    pub open_fds_init: [c_ulong; 1],
    pub full_fds_bits_init: [c_ulong; 1],
    pub fd_array: [*mut *mut file __rcu; NR_OPEN_DEFAULT],
}

//
// The caller must ensure that fd table isn't shared or hold rcu or file lock
//
// 'mask' is zero for an out-of-bounds fd, all ones for ok.
// 'fd&mask' is 'fd' for ok, or 0 for out of bounds.
//
// Accessing fdt->fd[0] is ok, but needs masking of the result.
//
extern "C" {
    pub fn files_lookup_fd_raw(_arg: files, _arg: fd) -> return;
}
extern "C" {
    pub fn test_bit(_arg: fd, _arg: files_fdtable(files)->close_on_exec) -> return;
}
extern "C" {
    pub fn put_files_struct(fs: *mut files_struct);
}
extern "C" {
    pub fn unshare_files() -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_range {
    pub to: unsigned int from,,
}

extern "C" {
    pub fn do_close_on_exec(: *mut files_struct);
}
extern "C" {
    pub fn close_fd(fd: c_uint) -> c_int;
}
