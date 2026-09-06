//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/i9xx_wm_regs.h
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
// Copyright © 2024 Intel Corporation

pub const DSPARB_CSTART_SHIFT: c_int = 7;

pub const DSPARB_BSTART_SHIFT: c_int = 0;

pub const DSPARB_AEND_SHIFT: c_int = 0;
pub const DSPARB_SPRITEA_SHIFT_VLV: c_int = 0;

pub const DSPARB_SPRITEB_SHIFT_VLV: c_int = 8;

pub const DSPARB_SPRITEC_SHIFT_VLV: c_int = 16;

pub const DSPARB_SPRITED_SHIFT_VLV: c_int = 24;

pub const DSPARB_SPRITEA_HI_SHIFT_VLV: c_int = 0;

pub const DSPARB_SPRITEB_HI_SHIFT_VLV: c_int = 4;

pub const DSPARB_SPRITEC_HI_SHIFT_VLV: c_int = 8;

pub const DSPARB_SPRITED_HI_SHIFT_VLV: c_int = 12;

pub const DSPARB_SPRITEE_HI_SHIFT_VLV: c_int = 16;

pub const DSPARB_SPRITEF_HI_SHIFT_VLV: c_int = 20;

pub const DSPARB_SPRITEE_SHIFT_VLV: c_int = 0;

pub const DSPARB_SPRITEF_SHIFT_VLV: c_int = 8;

// pnv/gen4/g4x/vlv/chv

pub const DSPFW_SR_SHIFT: c_int = 23;

pub const DSPFW_CURSORB_SHIFT: c_int = 16;

pub const DSPFW_PLANEB_SHIFT: c_int = 8;

pub const DSPFW_PLANEA_SHIFT: c_int = 0;

pub const DSPFW_FBC_SR_SHIFT: c_int = 28;

pub const DSPFW_FBC_HPLL_SR_SHIFT: c_int = 24;

pub const DSPFW_CURSORA_SHIFT: c_int = 8;

pub const DSPFW_PLANEC_OLD_SHIFT: c_int = 0;

pub const DSPFW_SPRITEA_SHIFT: c_int = 0;

pub const DSPFW_CURSOR_SR_SHIFT: c_int = 24;

pub const DSPFW_HPLL_CURSOR_SHIFT: c_int = 16;

pub const DSPFW_HPLL_SR_SHIFT: c_int = 0;

// vlv/chv

pub const DSPFW_SPRITEB_WM1_SHIFT: c_int = 16;

pub const DSPFW_CURSORA_WM1_SHIFT: c_int = 8;

pub const DSPFW_SPRITEA_WM1_SHIFT: c_int = 0;

pub const DSPFW_PLANEB_WM1_SHIFT: c_int = 24;

pub const DSPFW_PLANEA_WM1_SHIFT: c_int = 16;

pub const DSPFW_CURSORB_WM1_SHIFT: c_int = 8;

pub const DSPFW_CURSOR_SR_WM1_SHIFT: c_int = 0;

pub const DSPFW_SR_WM1_SHIFT: c_int = 0;

pub const DSPFW_SPRITED_WM1_SHIFT: c_int = 24;

pub const DSPFW_SPRITED_SHIFT: c_int = 16;

pub const DSPFW_SPRITEC_WM1_SHIFT: c_int = 8;

pub const DSPFW_SPRITEC_SHIFT: c_int = 0;

pub const DSPFW_SPRITEF_WM1_SHIFT: c_int = 24;

pub const DSPFW_SPRITEF_SHIFT: c_int = 16;

pub const DSPFW_SPRITEE_WM1_SHIFT: c_int = 8;

pub const DSPFW_SPRITEE_SHIFT: c_int = 0;

pub const DSPFW_PLANEC_WM1_SHIFT: c_int = 24;

pub const DSPFW_PLANEC_SHIFT: c_int = 16;

pub const DSPFW_CURSORC_WM1_SHIFT: c_int = 8;

pub const DSPFW_CURSORC_SHIFT: c_int = 0;

// vlv/chv high order bits

pub const DSPFW_SR_HI_SHIFT: c_int = 24;

pub const DSPFW_SPRITEF_HI_SHIFT: c_int = 23;

pub const DSPFW_SPRITEE_HI_SHIFT: c_int = 22;

pub const DSPFW_PLANEC_HI_SHIFT: c_int = 21;

pub const DSPFW_SPRITED_HI_SHIFT: c_int = 20;

pub const DSPFW_SPRITEC_HI_SHIFT: c_int = 16;

pub const DSPFW_PLANEB_HI_SHIFT: c_int = 12;

pub const DSPFW_SPRITEB_HI_SHIFT: c_int = 8;

pub const DSPFW_SPRITEA_HI_SHIFT: c_int = 4;

pub const DSPFW_PLANEA_HI_SHIFT: c_int = 0;

pub const DSPFW_SR_WM1_HI_SHIFT: c_int = 24;

pub const DSPFW_SPRITEF_WM1_HI_SHIFT: c_int = 23;

pub const DSPFW_SPRITEE_WM1_HI_SHIFT: c_int = 22;

pub const DSPFW_PLANEC_WM1_HI_SHIFT: c_int = 21;

pub const DSPFW_SPRITED_WM1_HI_SHIFT: c_int = 20;

pub const DSPFW_SPRITEC_WM1_HI_SHIFT: c_int = 16;

pub const DSPFW_PLANEB_WM1_HI_SHIFT: c_int = 12;

pub const DSPFW_SPRITEB_WM1_HI_SHIFT: c_int = 8;

pub const DSPFW_SPRITEA_WM1_HI_SHIFT: c_int = 4;

pub const DSPFW_PLANEA_WM1_HI_SHIFT: c_int = 0;

// drain latency register values

pub const DDL_CURSOR_SHIFT: c_int = 24;

pub const DDL_PLANE_SHIFT: c_int = 0;

pub const DRAIN_LATENCY_MASK: c_uint = 0x7f;
// FIFO watermark sizes etc
pub const G4X_FIFO_LINE_SIZE: c_int = 64;
pub const I915_FIFO_LINE_SIZE: c_int = 64;
pub const I830_FIFO_LINE_SIZE: c_int = 32;
pub const VALLEYVIEW_FIFO_SIZE: c_int = 255;
pub const G4X_FIFO_SIZE: c_int = 127;
pub const I965_FIFO_SIZE: c_int = 512;
pub const I945_FIFO_SIZE: c_int = 127;
pub const I915_FIFO_SIZE: c_int = 95;

pub const I830_FIFO_SIZE: c_int = 95;
pub const VALLEYVIEW_MAX_WM: c_uint = 0xff;
pub const G4X_MAX_WM: c_uint = 0x3f;
pub const I915_MAX_WM: c_uint = 0x3f;

pub const PINEVIEW_FIFO_LINE_SIZE: c_int = 64;
pub const PINEVIEW_MAX_WM: c_uint = 0x1ff;
pub const PINEVIEW_DFT_WM: c_uint = 0x3f;
pub const PINEVIEW_DFT_HPLLOFF_WM: c_int = 0;
pub const PINEVIEW_GUARD_WM: c_int = 10;
pub const PINEVIEW_CURSOR_FIFO: c_int = 64;
pub const PINEVIEW_CURSOR_MAX_WM: c_uint = 0x3f;
pub const PINEVIEW_CURSOR_DFT_WM: c_int = 0;
pub const PINEVIEW_CURSOR_GUARD_WM: c_int = 5;
pub const VALLEYVIEW_CURSOR_MAX_WM: c_int = 64;
pub const I965_CURSOR_FIFO: c_int = 64;
pub const I965_CURSOR_MAX_WM: c_int = 32;
pub const I965_CURSOR_DFT_WM: c_int = 8;
// define the Watermark register on Ironlake
pub const _WM0_PIPEA_ILK: c_uint = 0x45100;
pub const _WM0_PIPEB_ILK: c_uint = 0x45104;
pub const _WM0_PIPEC_IVB: c_uint = 0x45200;

