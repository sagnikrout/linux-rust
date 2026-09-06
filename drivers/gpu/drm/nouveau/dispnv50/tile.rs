//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/tile.h
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
// Tiling parameters for NV50+.
// GOB = Group of bytes, the main unit for tiling blocks.
// Tiling blocks are a power of 2 number of GOB.
// All GOBs and blocks have the same width: 64 bytes (so 16 pixels in 32bits).
// tile_mode is the log2 of the number of GOB per block.
//

pub const NV_TILE_GOB_WIDTH_BYTES: c_int = 64;
// Number of blocks to cover the width of the framebuffer
extern "C" {
    pub fn DIV_ROUND_UP(_arg: stride, _arg: NV_TILE_GOB_WIDTH_BYTES) -> return;
}
// Return the height in pixel of one GOB
// Number of blocks to cover the heigth of the framebuffer
extern "C" {
    pub fn DIV_ROUND_UP(_arg: height, gobs_in_block: *mut *mut nouveau_get_gob_height(family)) -> return;
}
// Return the GOB size in bytes
// Return the number of GOB in a block
// Return true if tile_mode is invalid
