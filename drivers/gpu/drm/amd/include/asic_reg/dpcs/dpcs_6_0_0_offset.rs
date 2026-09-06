//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/dpcs/dpcs_6_0_0_offset.h
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

// Macro flag: #define _dpcs_6_0_0_OFFSET_HEADER
// addressBlock: dpcssys_dcio_dcio_dispdec
// base address: 0x0
pub const regHPD_CTRL: c_uint = 0x286c;
pub const regHPD_CTRL_BASE_IDX: c_int = 2;
pub const regDC_PINSTRAPS: c_uint = 0x2880;
pub const regDC_PINSTRAPS_BASE_IDX: c_int = 2;
// addressBlock: dpcssys_dcio_dcio_chip_dispdec
// base address: 0x0
pub const regDC_GPIO_DDC1_MASK: c_uint = 0x28d0;
pub const regDC_GPIO_DDC1_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC2_MASK: c_uint = 0x28d4;
pub const regDC_GPIO_DDC2_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC3_MASK: c_uint = 0x28d8;
pub const regDC_GPIO_DDC3_MASK_BASE_IDX: c_int = 2;
pub const regDC_GPIO_DDC4_MASK: c_uint = 0x28dc;
pub const regDC_GPIO_DDC4_MASK_BASE_IDX: c_int = 2;
pub const regPHY_AUX_CNTL: c_uint = 0x28ff;
pub const regPHY_AUX_CNTL_BASE_IDX: c_int = 2;
pub const regDC_GPIO_AUX_CTRL_5: c_uint = 0x291d;
pub const regDC_GPIO_AUX_CTRL_5_BASE_IDX: c_int = 2;
// addressBlock: dpcssys_dcio_i3c_pad_control_ddc1_dc_i3c_dispdec
// base address: 0x0
pub const regDC_I3C0_DC_I3CPAD_CONTROL0: c_uint = 0x2f6c;
pub const regDC_I3C0_DC_I3CPAD_CONTROL0_BASE_IDX: c_int = 2;
pub const regDC_I3C0_DC_I3CPAD_CONTROL1: c_uint = 0x2f6d;
pub const regDC_I3C0_DC_I3CPAD_CONTROL1_BASE_IDX: c_int = 2;
// addressBlock: dpcssys_dcio_i3c_pad_control_ddc2_dc_i3c_dispdec
// base address: 0x8
pub const regDC_I3C1_DC_I3CPAD_CONTROL0: c_uint = 0x2f6e;
pub const regDC_I3C1_DC_I3CPAD_CONTROL0_BASE_IDX: c_int = 2;
pub const regDC_I3C1_DC_I3CPAD_CONTROL1: c_uint = 0x2f6f;
pub const regDC_I3C1_DC_I3CPAD_CONTROL1_BASE_IDX: c_int = 2;
