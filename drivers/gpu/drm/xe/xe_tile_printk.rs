//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_tile_printk.h
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
// Copyright © 2025 Intel Corporation
//

//
// The original xe_tile_dbg() callsite annotations are useless here,
// redirect to the tweaked xe_dbg_printer() instead.
//
// xe_tile_err_printer - Construct a &drm_printer that outputs to xe_tile_err()
// @tile: the &xe_tile pointer to use in xe_tile_err()
//
// Return: The &drm_printer object.
//
// xe_tile_info_printer - Construct a &drm_printer that outputs to xe_tile_info()
// @tile: the &xe_tile pointer to use in xe_tile_info()
//
// Return: The &drm_printer object.
//
// xe_tile_dbg_printer - Construct a &drm_printer that outputs like xe_tile_dbg()
// @tile: the &xe_tile pointer to use in xe_tile_dbg()
//
// Return: The &drm_printer object.
//
