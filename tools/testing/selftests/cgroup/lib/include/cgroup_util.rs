//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/cgroup/lib/include/cgroup_util.h
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

pub const BUF_SIZE: c_int = 4096;

//
// Checks if two given values differ by less than err% of their sum.
//
// Checks if two given values differ by less than err% of their sum and assert
// with detailed debug info if not.
//
extern "C" {
    pub fn read_text(path: *const c_char, buf: *mut c_char, max_len: usize) -> isize;
}
extern "C" {
    pub fn write_text(path: *const c_char, buf: *mut c_char, len: isize) -> isize;
}
extern "C" {
    pub fn cg_find_controller_root(root: *mut c_char, len: usize, controller: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_find_unified_root(root: *mut c_char, len: usize, nsdelegate: *mut bool) -> c_int;
}
extern "C" {
    pub fn cg_create(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_destroy(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long;
}
extern "C" {
    pub fn cg_read_long_fd(fd: c_int) -> c_long;
}
extern "C" {
    pub fn cg_read_key_long(cgroup: *const c_char, control: *const c_char, key: *const c_char) -> c_long;
}
extern "C" {
    pub fn cg_read_lc(cgroup: *const c_char, control: *const c_char) -> c_long;
}
extern "C" {
    pub fn cg_write(cgroup: *const c_char, control: *const c_char, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn cg_open(cgroup: *const c_char, control: *const c_char, flags: c_int) -> c_int;
}
extern "C" {
    pub fn cg_write_numeric(cgroup: *const c_char, control: *const c_char, value: c_long) -> c_int;
}
extern "C" {
    pub fn cg_enter(cgroup: *const c_char, pid: c_int) -> c_int;
}
extern "C" {
    pub fn cg_enter_current(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_enter_current_thread(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int;
}
extern "C" {
    pub fn cg_killall(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn proc_mount_contains(option: *const c_char) -> c_int;
}
extern "C" {
    pub fn cgroup_feature(feature: *const c_char) -> c_int;
}
extern "C" {
    pub fn proc_read_text(pid: c_int, thread: bool, item: *const c_char, buf: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn proc_read_strstr(pid: c_int, thread: bool, item: *const c_char, needle: *const c_char) -> c_int;
}
extern "C" {
    pub fn clone_into_cgroup(cgroup_fd: c_int) -> pid_t;
}
extern "C" {
    pub fn clone_reap(pid: pid_t, options: c_int) -> c_int;
}
extern "C" {
    pub fn clone_into_cgroup_run_wait(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn dirfd_open_opath(dir: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_prepare_for_wait(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn memcg_prepare_for_wait(cgroup: *const c_char) -> c_int;
}
extern "C" {
    pub fn cg_wait_for(fd: c_int) -> c_int;
}
