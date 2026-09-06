//! Automatically rewritten from C Header to Rust Module
//! Source: net/rds/info.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_info_lengths {
    pub nr: c_uint,
    pub each: c_uint,
}

//
// These functions must fill in the fields of @lens to reflect the size
// of the available info source.  If the snapshot fits in @len then it
// should be copied using @iter.  The caller will deduce if it was copied
// or not by comparing the lengths.
//
extern "C" {
    pub fn rds_info_register_func(optname: c_int, func: rds_info_func);
}
extern "C" {
    pub fn rds_info_deregister_func(optname: c_int, func: rds_info_func);
}
extern "C" {
    pub fn rds_info_getsockopt(sock: *mut socket, optname: c_int, opt: *mut sockopt_t) -> c_int;
}
extern "C" {
    pub fn rds_info_iter_unmap(iter: *mut rds_info_iterator);
}
