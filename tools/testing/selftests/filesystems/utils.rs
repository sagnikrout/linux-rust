//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/filesystems/utils.h
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

extern "C" {
    pub fn caps_down() -> c_int;
}
extern "C" {
    pub fn cap_down(down: cap_value_t) -> c_int;
}
extern "C" {
    pub fn switch_ids(uid: uid_t, gid: gid_t) -> bool;
}
extern "C" {
    pub fn setup_userns() -> c_int;
}
extern "C" {
    pub fn enter_userns() -> c_int;
}
extern "C" {
    pub fn wait_for_pid(pid: pid_t) -> c_int;
}
extern "C" {
    pub fn write_file(path: *const c_char, val: *const c_char) -> c_int;
}
extern "C" {
    pub fn get_unique_mnt_id(path: *const c_char) -> u64;
}
