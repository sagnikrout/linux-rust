//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/dpcs/dpcs_6_0_0_sh_mask.h
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


//
// SPDX-License-Identifier: MIT
//
// Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.
//

// Macro flag: #define _dpcs_6_0_0_SH_MASK_HEADER
// addressBlock: dpcssys_dcio_dcio_dispdec
// HPD_CTRL
pub const HPD_CTRL__HPD1_Y_POL_INVERT__SHIFT: c_uint = 0x0;
pub const HPD_CTRL__HPD2_Y_POL_INVERT__SHIFT: c_uint = 0x1;
pub const HPD_CTRL__HPD3_Y_POL_INVERT__SHIFT: c_uint = 0x2;
pub const HPD_CTRL__HPD4_Y_POL_INVERT__SHIFT: c_uint = 0x3;
pub const HPD_CTRL__HPD1_Y_POL_INVERT_MASK: c_uint = 0x00000001L;
pub const HPD_CTRL__HPD2_Y_POL_INVERT_MASK: c_uint = 0x00000002L;
pub const HPD_CTRL__HPD3_Y_POL_INVERT_MASK: c_uint = 0x00000004L;
pub const HPD_CTRL__HPD4_Y_POL_INVERT_MASK: c_uint = 0x00000008L;
// DC_PINSTRAPS
pub const DC_PINSTRAPS__DC_PINSTRAPS_AUDIO__SHIFT: c_uint = 0xe;
pub const DC_PINSTRAPS__DC_PINSTRAPS_AUDIO_MASK: c_uint = 0x0000C000L;
// addressBlock: dpcssys_dcio_dcio_chip_dispdec
// DC_GPIO_DDC1_MASK
pub const DC_GPIO_DDC1_MASK__AUX_PAD1_MODE__SHIFT: c_uint = 0x10;
pub const DC_GPIO_DDC1_MASK__AUX_PAD1_MODE_MASK: c_uint = 0x00010000L;
// addressBlock: dpcssys_dcio_i3c_pad_control_ddc1_dc_i3c_dispdec
// DC_I3C0_DC_I3CPAD_CONTROL0
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DDCCLK_MASK__SHIFT: c_uint = 0x0;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DDCDATA_MASK__SHIFT: c_uint = 0x1;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_PD_EN__SHIFT: c_uint = 0x3;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_CLK_A__SHIFT: c_uint = 0x5;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DATA_A__SHIFT: c_uint = 0x8;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_CLK_EN__SHIFT: c_uint = 0xc;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DATA_EN__SHIFT: c_uint = 0x10;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_CLK_Y__SHIFT: c_uint = 0x14;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DATA_Y__SHIFT: c_uint = 0x18;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DDCCLK_MASK_MASK: c_uint = 0x00000001L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DDCDATA_MASK_MASK: c_uint = 0x00000002L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_PD_EN_MASK: c_uint = 0x00000018L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_CLK_A_MASK: c_uint = 0x00000020L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DATA_A_MASK: c_uint = 0x00000100L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_CLK_EN_MASK: c_uint = 0x00001000L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DATA_EN_MASK: c_uint = 0x00010000L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_CLK_Y_MASK: c_uint = 0x00100000L;
pub const DC_I3C0_DC_I3CPAD_CONTROL0__DC_I3CPAD_DATA_Y_MASK: c_uint = 0x01000000L;
// DC_I3C0_DC_I3CPAD_CONTROL1
pub const DC_I3C0_DC_I3CPAD_CONTROL1__DC_I3CPAD_STR__SHIFT: c_uint = 0x0;
pub const DC_I3C0_DC_I3CPAD_CONTROL1__DC_I3CPAD_RXSEL__SHIFT: c_uint = 0x6;
pub const DC_I3C0_DC_I3CPAD_CONTROL1__DC_I3CPAD_STR_MASK: c_uint = 0x0000000FL;
pub const DC_I3C0_DC_I3CPAD_CONTROL1__DC_I3CPAD_RXSEL_MASK: c_uint = 0x000000C0L;
