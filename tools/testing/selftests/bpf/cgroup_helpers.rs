//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/cgroup_helpers.h
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

// cgroupv2 related
extern "C" {
    pub fn enable_controllers(relative_path: *const c_char, controllers: *const c_char) -> c_int;
}
extern "C" {
    pub fn cgroup_setup_and_join(relative_path: *const c_char) -> c_int;
}
extern "C" {
    pub fn get_root_cgroup() -> c_int;
}
extern "C" {
    pub fn create_and_get_cgroup(relative_path: *const c_char) -> c_int;
}
extern "C" {
    pub fn remove_cgroup(relative_path: *const c_char);
}
extern "C" {
    pub fn remove_cgroup_pid(relative_path: *const c_char, pid: c_int);
}
extern "C" {
    pub fn get_cgroup_id(relative_path: *const c_char) -> c_ulonglong;
}
extern "C" {
    pub fn get_cgroup1_hierarchy_id(subsys_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn join_cgroup(relative_path: *const c_char) -> c_int;
}
extern "C" {
    pub fn join_root_cgroup() -> c_int;
}
extern "C" {
    pub fn join_parent_cgroup(relative_path: *const c_char) -> c_int;
}
extern "C" {
    pub fn setup_cgroup_environment() -> c_int;
}
extern "C" {
    pub fn cleanup_cgroup_environment();
}
// cgroupv1 related
extern "C" {
    pub fn set_classid() -> c_int;
}
extern "C" {
    pub fn join_classid() -> c_int;
}
extern "C" {
    pub fn get_classid_cgroup_id() -> c_ulonglong;
}
extern "C" {
    pub fn open_classid() -> c_int;
}
extern "C" {
    pub fn setup_classid_environment() -> c_int;
}
extern "C" {
    pub fn cleanup_classid_environment();
}
