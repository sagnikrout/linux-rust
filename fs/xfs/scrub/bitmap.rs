//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/bitmap.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// u64 bitmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xbitmap64 {
    pub xb_root: rb_root_cached,
}

extern "C" {
    pub fn xbitmap64_init(bitmap: *mut xbitmap64);
}
extern "C" {
    pub fn xbitmap64_destroy(bitmap: *mut xbitmap64);
}
extern "C" {
    pub fn xbitmap64_clear(bitmap: *mut xbitmap64, start: u64, len: u64) -> c_int;
}
extern "C" {
    pub fn xbitmap64_set(bitmap: *mut xbitmap64, start: u64, len: u64) -> c_int;
}
extern "C" {
    pub fn xbitmap64_disunion(bitmap: *mut xbitmap64, sub: *mut xbitmap64) -> c_int;
}
extern "C" {
    pub fn xbitmap64_hweight(bitmap: *mut xbitmap64) -> u64;
}
//
// Return codes for the bitmap iterator functions are 0 to continue iterating,
// and non-zero to stop iterating.  Any non-zero value will be passed up to the
// iteration caller.  The special value -ECANCELED can be used to stop
// iteration, because neither bitmap iterator ever generates that error code on
// its own.  Callers must not modify the bitmap while walking it.
//
extern "C" {
    pub fn int(start: *mut *mut xbitmap64_walk_fn)(uint64_t, len: u64, priv: *mut c_void) -> typedef;
}
extern "C" {
    pub fn xbitmap64_empty(bitmap: *mut xbitmap64) -> bool;
}
extern "C" {
    pub fn xbitmap64_test(bitmap: *mut xbitmap64, start: u64, len: *mut u64) -> bool;
}
// u32 bitmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xbitmap32 {
    pub xb_root: rb_root_cached,
}

extern "C" {
    pub fn xbitmap32_init(bitmap: *mut xbitmap32);
}
extern "C" {
    pub fn xbitmap32_destroy(bitmap: *mut xbitmap32);
}
extern "C" {
    pub fn xbitmap32_clear(bitmap: *mut xbitmap32, start: u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xbitmap32_set(bitmap: *mut xbitmap32, start: u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xbitmap32_disunion(bitmap: *mut xbitmap32, sub: *mut xbitmap32) -> c_int;
}
extern "C" {
    pub fn xbitmap32_hweight(bitmap: *mut xbitmap32) -> u32;
}
//
// Return codes for the bitmap iterator functions are 0 to continue iterating,
// and non-zero to stop iterating.  Any non-zero value will be passed up to the
// iteration caller.  The special value -ECANCELED can be used to stop
// iteration, because neither bitmap iterator ever generates that error code on
// its own.  Callers must not modify the bitmap while walking it.
//
extern "C" {
    pub fn int(start: *mut *mut xbitmap32_walk_fn)(uint32_t, len: u32, priv: *mut c_void) -> typedef;
}
extern "C" {
    pub fn xbitmap32_empty(bitmap: *mut xbitmap32) -> bool;
}
extern "C" {
    pub fn xbitmap32_test(bitmap: *mut xbitmap32, start: u32, len: *mut u32) -> bool;
}
extern "C" {
    pub fn xbitmap32_count_set_regions(bitmap: *mut xbitmap32) -> u32;
}
