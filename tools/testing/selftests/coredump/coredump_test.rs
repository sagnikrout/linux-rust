//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/coredump/coredump_test.h
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

pub const PAGE_SIZE: c_int = 4096;

pub const NUM_THREAD_SPAWN: c_int = 128;
// Coredump fixture
// Shared helper function declarations
extern "C" {
    pub fn crashing_child();
}
extern "C" {
    pub fn create_detached_tmpfs() -> c_int;
}
extern "C" {
    pub fn create_and_listen_unix_socket(path: *const c_char) -> c_int;
}
extern "C" {
    pub fn set_core_pattern(pattern: *const c_char) -> bool;
}
extern "C" {
    pub fn get_peer_pidfd(fd: c_int) -> c_int;
}
extern "C" {
    pub fn get_pidfd_info(fd_peer_pidfd: c_int, info: *mut pidfd_info) -> bool;
}
// Inline helper that uses harness types
// Protocol helper function declarations
extern "C" {
    pub fn recv_marker(fd: c_int) -> isize;
}
extern "C" {
    pub fn read_marker(fd: c_int, mark: coredump_mark) -> bool;
}
extern "C" {
    pub fn read_coredump_req(fd: c_int, req: *mut coredump_req) -> bool;
}
extern "C" {
    pub fn open_coredump_tmpfile(fd_tmpfs_detached: c_int) -> c_int;
}
extern "C" {
    pub fn process_coredump_worker(fd_coredump: c_int, fd_peer_pidfd: c_int, fd_core_file: c_int);
}
