//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_printk.h
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
// Copyright © 2023 Intel Corporation
//

//
// The original xe_gt_dbg() callsite annotations are useless here,
// redirect to the tweaked xe_tile_dbg_printer() instead.
//
// xe_gt_err_printer - Construct a &drm_printer that outputs to xe_gt_err()
// @gt: the &xe_gt pointer to use in xe_gt_err()
//
// Return: The &drm_printer object.
//
// xe_gt_info_printer - Construct a &drm_printer that outputs to xe_gt_info()
// @gt: the &xe_gt pointer to use in xe_gt_info()
//
// Return: The &drm_printer object.
//
// xe_gt_dbg_printer - Construct a &drm_printer that outputs like xe_gt_dbg()
// @gt: the &xe_gt pointer to use in xe_gt_dbg()
//
// Return: The &drm_printer object.
//
