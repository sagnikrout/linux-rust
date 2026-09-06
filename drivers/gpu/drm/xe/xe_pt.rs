//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pt.h
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
// Copyright © 2022 Intel Corporation
//

// Largest huge pte is currently 1GiB. May become device dependent.
pub const MAX_HUGEPTE_LEVEL: c_int = 2;

extern "C" {
    pub fn xe_pt_shift(level: c_uint) -> c_uint;
}
extern "C" {
    pub fn xe_pt_destroy(pt: *mut xe_pt, flags: u32, deferred: *mut llist_head);
}
extern "C" {
    pub fn xe_pt_clear(xe: *mut xe_device, pt: *mut xe_pt);
}
extern "C" {
    pub fn xe_pt_update_ops_prepare(tile: *mut xe_tile, vops: *mut xe_vma_ops) -> c_int;
}
extern "C" {
    pub fn xe_pt_update_ops_fini(tile: *mut xe_tile, vops: *mut xe_vma_ops);
}
extern "C" {
    pub fn xe_pt_update_ops_abort(tile: *mut xe_tile, vops: *mut xe_vma_ops);
}
extern "C" {
    pub fn xe_pt_zap_ptes(tile: *mut xe_tile, vma: *mut xe_vma) -> bool;
}
