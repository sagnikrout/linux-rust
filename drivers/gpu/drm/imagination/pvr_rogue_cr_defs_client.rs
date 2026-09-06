//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_cr_defs_client.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.
// clang-format off
//
// This register controls the anti-aliasing mode of the Tiling Co-Processor, independent control is
// provided in both X & Y axis.
// This register needs to be set based on the ISP Samples Per Pixel a core supports.
//
// When ISP Samples Per Pixel = 1:
// 2xmsaa is achieved by enabling Y - TE does AA on Y plane only
// 4xmsaa is achieved by enabling Y and X - TE does AA on X and Y plane
// 8xmsaa not supported by XE cores
//
// When ISP Samples Per Pixel = 2:
// 2xmsaa is achieved by enabling X2 - does not affect TE
// 4xmsaa is achieved by enabling Y and X2 - TE does AA on Y plane only
// 8xmsaa is achieved by enabling Y, X and X2 - TE does AA on X and Y plane
// 8xmsaa not supported by XE cores
//
// When ISP Samples Per Pixel = 4:
// 2xmsaa is achieved by enabling X2 - does not affect TE
// 4xmsaa is achieved by enabling Y2 and X2 - TE does AA on Y plane only
// 8xmsaa not supported by XE cores
//
// Register ROGUE_CR_TE_AA
pub const ROGUE_CR_TE_AA: c_uint = 0x0C00U;
pub const ROGUE_CR_TE_AA_MASKFULL: c_uint = 0x000000000000000Full;
// Y2
// Indicates 4xmsaa when X2 and Y2 are set to 1. This does not affect TE and is only used within
// TPW.
//
pub const ROGUE_CR_TE_AA_Y2_SHIFT: c_int = 3;
pub const ROGUE_CR_TE_AA_Y2_CLRMSK: c_uint = 0xFFFFFFF7;
pub const ROGUE_CR_TE_AA_Y2_EN: c_uint = 0x00000008;
// Y
// Anti-Aliasing in Y Plane Enabled
//
pub const ROGUE_CR_TE_AA_Y_SHIFT: c_int = 2;
pub const ROGUE_CR_TE_AA_Y_CLRMSK: c_uint = 0xFFFFFFFB;
pub const ROGUE_CR_TE_AA_Y_EN: c_uint = 0x00000004;
// X
// Anti-Aliasing in X Plane Enabled
//
pub const ROGUE_CR_TE_AA_X_SHIFT: c_int = 1;
pub const ROGUE_CR_TE_AA_X_CLRMSK: c_uint = 0xFFFFFFFD;
pub const ROGUE_CR_TE_AA_X_EN: c_uint = 0x00000002;
// X2
// 2x Anti-Aliasing Enabled, affects PPP only
//

// MacroTile Boundaries X Plane
// Register ROGUE_CR_TE_MTILE1
pub const ROGUE_CR_TE_MTILE1: c_uint = 0x0C08;
pub const ROGUE_CR_TE_MTILE1_MASKFULL: c_uint = 0x0000000007FFFFFFull;
// X1 default: 0x00000004
// X1 MacroTile boundary, left tile X for second column of macrotiles (16MT mode) - 32 pixels across
// tile
//
pub const ROGUE_CR_TE_MTILE1_X1_SHIFT: c_int = 18;
pub const ROGUE_CR_TE_MTILE1_X1_CLRMSK: c_uint = 0xF803FFFF;
// X2 default: 0x00000008
// X2 MacroTile boundary, left tile X for third(16MT) column of macrotiles - 32 pixels across tile
//

pub const ROGUE_CR_TE_MTILE1_X2_CLRMSK: c_uint = 0xFFFC01FF;
// X3 default: 0x0000000c
// X3 MacroTile boundary, left tile X for fourth column of macrotiles (16MT) - 32 pixels across tile
//
pub const ROGUE_CR_TE_MTILE1_X3_SHIFT: c_int = 0;
pub const ROGUE_CR_TE_MTILE1_X3_CLRMSK: c_uint = 0xFFFFFE00;
// MacroTile Boundaries Y Plane.
// Register ROGUE_CR_TE_MTILE2
pub const ROGUE_CR_TE_MTILE2: c_uint = 0x0C10;
pub const ROGUE_CR_TE_MTILE2_MASKFULL: c_uint = 0x0000000007FFFFFFull;
// Y1 default: 0x00000004
// X1 MacroTile boundary, ltop tile Y for second column of macrotiles (16MT mode) - 32 pixels tile
// height
//
pub const ROGUE_CR_TE_MTILE2_Y1_SHIFT: c_int = 18;
pub const ROGUE_CR_TE_MTILE2_Y1_CLRMSK: c_uint = 0xF803FFFF;
// Y2 default: 0x00000008
// X2 MacroTile boundary, top tile Y for third(16MT) column of macrotiles - 32 pixels tile height
//
pub const ROGUE_CR_TE_MTILE2_Y2_SHIFT: c_int = 9;
pub const ROGUE_CR_TE_MTILE2_Y2_CLRMSK: c_uint = 0xFFFC01FF;
// Y3 default: 0x0000000c
// X3 MacroTile boundary, top tile Y for fourth column of macrotiles (16MT) - 32 pixels tile height
//
pub const ROGUE_CR_TE_MTILE2_Y3_SHIFT: c_int = 0;
pub const ROGUE_CR_TE_MTILE2_Y3_CLRMSK: c_uint = 0xFFFFFE00;
//
// In order to perform the tiling operation and generate the display list the maximum screen size
// must be configured in terms of the number of tiles in X & Y axis.
//
// Register ROGUE_CR_TE_SCREEN
pub const ROGUE_CR_TE_SCREEN: c_uint = 0x0C18U;
pub const ROGUE_CR_TE_SCREEN_MASKFULL: c_uint = 0x00000000001FF1FFull;
// YMAX default: 0x00000010
// Maximum Y tile address visible on screen, 32 pixel tile height, 16Kx16K max screen size
//
pub const ROGUE_CR_TE_SCREEN_YMAX_SHIFT: c_int = 12;
pub const ROGUE_CR_TE_SCREEN_YMAX_CLRMSK: c_uint = 0xFFE00FFF;
// XMAX default: 0x00000010
// Maximum X tile address visible on screen, 32 pixel tile width, 16Kx16K max screen size
//
pub const ROGUE_CR_TE_SCREEN_XMAX_SHIFT: c_int = 0;
pub const ROGUE_CR_TE_SCREEN_XMAX_CLRMSK: c_uint = 0xFFFFFE00;
//
// In order to perform the tiling operation and generate the display list the maximum screen size
// must be configured in terms of the number of pixels in X & Y axis since this may not be the same
// as the number of tiles defined in the RGX_CR_TE_SCREEN register.
//
// Register ROGUE_CR_PPP_SCREEN
pub const ROGUE_CR_PPP_SCREEN: c_uint = 0x0C98;
pub const ROGUE_CR_PPP_SCREEN_MASKFULL: c_uint = 0x000000007FFF7FFFull;
// PIXYMAX
// Screen height in pixels. (16K x 16K max screen size)
//
pub const ROGUE_CR_PPP_SCREEN_PIXYMAX_SHIFT: c_int = 16;
pub const ROGUE_CR_PPP_SCREEN_PIXYMAX_CLRMSK: c_uint = 0x8000FFFF;
// PIXXMAX
// Screen width in pixels.(16K x 16K max screen size)
//
pub const ROGUE_CR_PPP_SCREEN_PIXXMAX_SHIFT: c_int = 0;
pub const ROGUE_CR_PPP_SCREEN_PIXXMAX_CLRMSK: c_uint = 0xFFFF8000;
// Register ROGUE_CR_ISP_MTILE_SIZE
pub const ROGUE_CR_ISP_MTILE_SIZE: c_uint = 0x0F18;
pub const ROGUE_CR_ISP_MTILE_SIZE_MASKFULL: c_uint = 0x0000000003FF03FFull;
// X
// Macrotile width, in tiles. A value of zero corresponds to the maximum size
//
pub const ROGUE_CR_ISP_MTILE_SIZE_X_SHIFT: c_int = 16;
pub const ROGUE_CR_ISP_MTILE_SIZE_X_CLRMSK: c_uint = 0xFC00FFFF;
pub const ROGUE_CR_ISP_MTILE_SIZE_X_ALIGNSHIFT: c_int = 0;
pub const ROGUE_CR_ISP_MTILE_SIZE_X_ALIGNSIZE: c_int = 1;
// Y
// Macrotile height, in tiles. A value of zero corresponds to the maximum size
//
pub const ROGUE_CR_ISP_MTILE_SIZE_Y_SHIFT: c_int = 0;
pub const ROGUE_CR_ISP_MTILE_SIZE_Y_CLRMSK: c_uint = 0xFFFFFC00;
pub const ROGUE_CR_ISP_MTILE_SIZE_Y_ALIGNSHIFT: c_int = 0;
pub const ROGUE_CR_ISP_MTILE_SIZE_Y_ALIGNSIZE: c_int = 1;
// clang-format on
