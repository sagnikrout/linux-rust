//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/agb_bitmap.h
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
// Bitmaps, but for type-checked for xfs_agblock_t
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xagb_bitmap {
    pub agbitmap: xbitmap32,
}

extern "C" {
    pub fn xbitmap32_clear(_arg: &bitmap->agbitmap, _arg: start, _arg: len) -> return;
}
extern "C" {
    pub fn xbitmap32_set(_arg: &bitmap->agbitmap, _arg: start, _arg: len) -> return;
}
extern "C" {
    pub fn xbitmap32_test(_arg: &bitmap->agbitmap, _arg: start, _arg: len) -> return;
}
extern "C" {
    pub fn xbitmap32_disunion(_arg: &bitmap->agbitmap, _arg: &sub->agbitmap) -> return;
}
extern "C" {
    pub fn xbitmap32_hweight(_arg: &bitmap->agbitmap) -> return;
}
extern "C" {
    pub fn xbitmap32_empty(_arg: &bitmap->agbitmap) -> return;
}
extern "C" {
    pub fn xbitmap32_walk(_arg: &bitmap->agbitmap, _arg: fn, _arg: priv) -> return;
}
extern "C" {
    pub fn xbitmap32_count_set_regions(_arg: &b->agbitmap) -> return;
}
