//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/gcov/gcov.h
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
// Profiling infrastructure declarations.
//
// This file is based on gcc-internal definitions. Data structures are
// defined to be compatible with gcc counterparts. For a better
// understanding, refer to gcc source: gcc/gcov-io.h.
//
// Copyright IBM Corp. 2009
// Author(s): Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
//
// Uses gcc-internal data definitions.
//

//
// Profiling data types used for gcc 3.4 and above - these are defined by
// gcc and need to be kept as close to the original definition as possible to
// remain compatible.
//

pub type gcov_type = c_long;

pub type gcov_type = c_longlong;

// Opaque gcov_info. The gcov structures can change as for example in gcc 4.7 so
// we cannot use full definition here and they need to be placed in gcc specific
// implementation of gcov. This also means no direct access to the members in
// generic code and usage of the interface below.
// Interface to access gcov_info data
extern "C" {
    pub fn gcov_info_version(info: *mut gcov_info) -> c_uint;
}
extern "C" {
    pub fn gcov_info_link(info: *mut gcov_info);
}
extern "C" {
    pub fn gcov_info_unlink(prev: *mut gcov_info, info: *mut gcov_info);
}
extern "C" {
    pub fn gcov_info_within_module(info: *mut gcov_info, mod: *mut module) -> bool;
}
extern "C" {
    pub fn convert_to_gcda(buffer: *mut c_char, info: *mut gcov_info) -> usize;
}
// Base interface.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gcov_action {
    GCOV_ADD,
    GCOV_REMOVE,
}

extern "C" {
    pub fn gcov_event(action: gcov_action, info: *mut gcov_info);
}
extern "C" {
    pub fn gcov_enable_events();
}
// writing helpers
extern "C" {
    pub fn store_gcov_u32(buffer: *mut c_void, off: usize, v: u32) -> usize;
}
extern "C" {
    pub fn store_gcov_u64(buffer: *mut c_void, off: usize, v: u64) -> usize;
}
// gcov_info control.
extern "C" {
    pub fn gcov_info_reset(info: *mut gcov_info);
}
extern "C" {
    pub fn gcov_info_is_compatible(info1: *mut gcov_info, info2: *mut gcov_info) -> c_int;
}
extern "C" {
    pub fn gcov_info_add(dest: *mut gcov_info, source: *mut gcov_info);
}
extern "C" {
    pub fn gcov_info_free(info: *mut gcov_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_link {
    pub dir: },
    pub ext: *const c_char,
}
