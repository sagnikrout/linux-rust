//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_tile.h
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

extern "C" {
    pub fn xe_tile_init_early(tile: *mut xe_tile, xe: *mut xe_device, id: u8) -> c_int;
}
extern "C" {
    pub fn xe_tile_init_noalloc(tile: *mut xe_tile) -> c_int;
}
extern "C" {
    pub fn xe_tile_init(tile: *mut xe_tile) -> c_int;
}
extern "C" {
    pub fn xe_tile_alloc_vram(tile: *mut xe_tile) -> c_int;
}
extern "C" {
    pub fn xe_tile_migrate_wait(tile: *mut xe_tile);
}
//
// xe_tile_to_vr() - Return the struct xe_vram_region pointer from a
// struct xe_tile pointer
// @tile: Pointer to the struct xe_tile.
//
// Return: Pointer to the struct xe_vram_region embedded in *@tile.
//

