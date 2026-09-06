//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_buf.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2024 Intel Corporation
//

extern "C" {
    pub fn xe_guc_buf_cache_init(cache: *mut xe_guc_buf_cache) -> c_int;
}
extern "C" {
    pub fn xe_guc_buf_cache_init_with_size(cache: *mut xe_guc_buf_cache, size: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_buf_cache_dwords(cache: *mut xe_guc_buf_cache) -> u32;
}
extern "C" {
    pub fn xe_guc_buf_reserve(cache: *mut xe_guc_buf_cache, dwords: u32) -> xe_guc_buf;
}
extern "C" {
    pub fn xe_guc_buf_release(buf: xe_guc_buf);
}
//
// xe_guc_buf_is_valid() - Check if a buffer reference is valid.
// @buf: the &xe_guc_buf reference to check
//
// Return: true if @ref represents a valid sub-allication.
//
extern "C" {
    pub fn xe_guc_buf_flush(buf: xe_guc_buf) -> u64;
}
extern "C" {
    pub fn xe_guc_buf_gpu_addr(buf: xe_guc_buf) -> u64;
}
extern "C" {
    pub fn xe_guc_cache_gpu_addr_from_ptr(cache: *mut xe_guc_buf_cache, ptr: *const c_void, size: u32) -> u64;
}
