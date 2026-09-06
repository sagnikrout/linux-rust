//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/build-id.h
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
pub const PERF_BUILD_ID_H_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct build_id {
    pub data: [u8; BUILD_ID_SIZE],
    pub size: u8,
}

extern "C" {
    pub fn build_id__init(bid: *mut build_id, data: *const u8, size: usize);
}
extern "C" {
    pub fn build_id__snprintf(build_id: *const build_id, bf: *mut c_char, bf_size: usize) -> c_int;
}
extern "C" {
    pub fn build_id__is_defined(bid: *const build_id) -> bool;
}
extern "C" {
    pub fn sysfs__snprintf_build_id(root_dir: *const c_char, sbuild_id: *mut c_char, sbuild_id_size: usize) -> c_int;
}
extern "C" {
    pub fn filename__snprintf_build_id(pathname: *const c_char, sbuild_id: *mut c_char, sbuild_id_size: usize) -> c_int;
}
extern "C" {
    pub fn perf_session__read_build_ids(session: *mut perf_session, with_hits: bool) -> bool;
}
extern "C" {
    pub fn perf_session__cache_build_ids(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn build_id_cache__cached(sbuild_id: *const c_char) -> bool;
}
extern "C" {
    pub fn __build_id_cache__add_s(_arg: sbuild_id, _arg: name, _arg: nsi, _arg: is_kallsyms, _arg: is_vdso, _arg: NULL, _arg: NULL) -> return;
}
extern "C" {
    pub fn build_id_cache__remove_s(sbuild_id: *const c_char) -> c_int;
}
extern "C" {
    pub fn set_buildid_dir(dir: *const c_char);
}
extern "C" {
    pub fn disable_buildid_cache();
}
