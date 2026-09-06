//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/interconnect.h
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
// Copyright (c) 2018-2019, Linaro Ltd.
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//

// macros for converting to icc units

// macro to indicate dynamic id allocation

//
// struct icc_bulk_data - Data used for bulk icc operations.
//
// @path: reference to the interconnect path (internal use)
// @name: the name from the "interconnect-names" DT property
// @avg_bw: average bandwidth in icc units
// @peak_bw: peak bandwidth in icc units
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_bulk_data {
    pub path: *mut icc_path,
    pub name: *const c_char,
    pub avg_bw: u32,
    pub peak_bw: u32,
}

extern "C" {
    pub fn devm_of_icc_bulk_get(dev: *mut device, num_paths: c_int, paths: *mut icc_bulk_data) -> c_int;
}
extern "C" {
    pub fn icc_put(path: *mut icc_path);
}
extern "C" {
    pub fn icc_enable(path: *mut icc_path) -> c_int;
}
extern "C" {
    pub fn icc_disable(path: *mut icc_path) -> c_int;
}
extern "C" {
    pub fn icc_set_bw(path: *mut icc_path, avg_bw: u32, peak_bw: u32) -> c_int;
}
extern "C" {
    pub fn icc_set_tag(path: *mut icc_path, tag: u32);
}
extern "C" {
    pub fn icc_bulk_put(num_paths: c_int, paths: *mut icc_bulk_data);
}
extern "C" {
    pub fn icc_bulk_set_bw(num_paths: c_int, paths: *const icc_bulk_data) -> c_int;
}
extern "C" {
    pub fn icc_bulk_enable(num_paths: c_int, paths: *const icc_bulk_data) -> c_int;
}
extern "C" {
    pub fn icc_bulk_disable(num_paths: c_int, paths: *const icc_bulk_data);
}

