//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-decon7.h
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Ajay Kumar <ajaykumar.rs@samsung.com>
//
// VIDCON0
pub const VIDCON0: c_uint = 0x00;

// VIDOUTCON0
pub const VIDOUTCON0: c_uint = 0x4;

pub const VIDOUTCON0_IF_SHIFT: c_int = 23;

// VIDCON3
pub const VIDCON3: c_uint = 0x8;
// VIDCON4
pub const VIDCON4: c_uint = 0xC;

// VCLKCON0
pub const VCLKCON0: c_uint = 0x10;

// VCLKCON
pub const VCLKCON1: c_uint = 0x14;

pub const VCLKCON2: c_uint = 0x18;
// SHADOWCON
pub const SHADOWCON: c_uint = 0x30;

// WINCONx

pub const WINCONx_BUFSEL_SHIFT: c_int = 28;

pub const WINCONx_BPPMODE_SHIFT: c_int = 2;

// VIDOSDxH: The height for the OSD image(READ ONLY)

// Frame buffer start addresses: VIDWxxADD0n

// Interrupt controls register
pub const VIDINTCON2: c_uint = 0x228;

// Interrupt controls and status register
pub const VIDINTCON3: c_uint = 0x22C;

// VIDOSDxA ~ VIDOSDxE
pub const VIDOSD_BASE: c_uint = 0x230;
pub const OSD_STRIDE: c_uint = 0x20;

pub const VIDOSDxA_TOPLEFT_X_SHIFT: c_int = 13;
pub const VIDOSDxA_TOPLEFT_X_LIMIT: c_uint = 0x1fff;

pub const VIDOSDxA_TOPLEFT_Y_SHIFT: c_int = 0;
pub const VIDOSDxA_TOPLEFT_Y_LIMIT: c_uint = 0x1fff;

pub const VIDOSDxB_BOTRIGHT_X_SHIFT: c_int = 13;
pub const VIDOSDxB_BOTRIGHT_X_LIMIT: c_uint = 0x1fff;

pub const VIDOSDxB_BOTRIGHT_Y_SHIFT: c_int = 0;
pub const VIDOSDxB_BOTRIGHT_Y_LIMIT: c_uint = 0x1fff;

// Window MAP (Color map)

pub const WINxMAP_MAP_COLOUR_SHIFT: c_int = 0;
pub const WINxMAP_MAP_COLOUR_LIMIT: c_uint = 0xffffff;

// Window colour-key control registers
pub const WKEYCON: c_uint = 0x370;
pub const WKEYCON0: c_uint = 0x00;
pub const WKEYCON1: c_uint = 0x04;

pub const WxKEYCON0_COMPKEY_SHIFT: c_int = 0;
pub const WxKEYCON0_COMPKEY_LIMIT: c_uint = 0xffffff;

pub const WxKEYCON1_COLVAL_SHIFT: c_int = 0;
pub const WxKEYCON1_COLVAL_LIMIT: c_uint = 0xffffff;

// color key control register for hardware window 1 ~ 4.

// color key value register for hardware window 1 ~ 4.

// Window KEY Alpha value

pub const Wx_KEYALPHA_R_F_SHIFT: c_int = 16;
pub const Wx_KEYALPHA_G_F_SHIFT: c_int = 8;
pub const Wx_KEYALPHA_B_F_SHIFT: c_int = 0;
// Blending equation

pub const BLENDE_COEF_ZERO: c_uint = 0x0;
pub const BLENDE_COEF_ONE: c_uint = 0x1;
pub const BLENDE_COEF_ALPHA_A: c_uint = 0x2;
pub const BLENDE_COEF_ONE_MINUS_ALPHA_A: c_uint = 0x3;
pub const BLENDE_COEF_ALPHA_B: c_uint = 0x4;
pub const BLENDE_COEF_ONE_MINUS_ALPHA_B: c_uint = 0x5;
pub const BLENDE_COEF_ALPHA0: c_uint = 0x6;
pub const BLENDE_COEF_A: c_uint = 0xA;
pub const BLENDE_COEF_ONE_MINUS_A: c_uint = 0xB;
pub const BLENDE_COEF_B: c_uint = 0xC;
pub const BLENDE_COEF_ONE_MINUS_B: c_uint = 0xD;

// Blending equation control
pub const BLENDCON: c_uint = 0x3D8;

// Interrupt control register
pub const VIDINTCON0: c_uint = 0x500;

pub const VIDINTCON0_FRAMESEL0_SHIFT: c_int = 15;

pub const VIDINTCON0_FIFOLEVEL_SHIFT: c_int = 3;

// Interrupt controls and status register
pub const VIDINTCON1: c_uint = 0x504;

// VIDCON1

// VIDTCON0
pub const VIDTCON0: c_uint = 0x610;

pub const VIDTCON0_VBPD_SHIFT: c_int = 16;
pub const VIDTCON0_VBPD_LIMIT: c_uint = 0xffff;

pub const VIDTCON0_VFPD_SHIFT: c_int = 0;
pub const VIDTCON0_VFPD_LIMIT: c_uint = 0xffff;

// VIDTCON1
pub const VIDTCON1: c_uint = 0x614;

pub const VIDTCON1_VSPW_SHIFT: c_int = 16;
pub const VIDTCON1_VSPW_LIMIT: c_uint = 0xffff;

// VIDTCON2
pub const VIDTCON2: c_uint = 0x618;

pub const VIDTCON2_HBPD_SHIFT: c_int = 16;
pub const VIDTCON2_HBPD_LIMIT: c_uint = 0xffff;

pub const VIDTCON2_HFPD_SHIFT: c_int = 0;
pub const VIDTCON2_HFPD_LIMIT: c_uint = 0xffff;

// VIDTCON3
pub const VIDTCON3: c_uint = 0x61C;

pub const VIDTCON3_HSPW_SHIFT: c_int = 16;
pub const VIDTCON3_HSPW_LIMIT: c_uint = 0xffff;

// VIDTCON4
pub const VIDTCON4: c_uint = 0x620;

pub const VIDTCON4_LINEVAL_SHIFT: c_int = 16;
pub const VIDTCON4_LINEVAL_LIMIT: c_uint = 0xfff;

pub const VIDTCON4_HOZVAL_SHIFT: c_int = 0;
pub const VIDTCON4_HOZVAL_LIMIT: c_uint = 0xfff;

// LINECNT OP THRSHOLD
pub const LINECNT_OP_THRESHOLD: c_uint = 0x630;
// CRCCTRL
pub const CRCCTRL: c_uint = 0x6C8;

// DECON_CMU
pub const DECON_CMU: c_uint = 0x704;
pub const DECON_CMU_ALL_CLKGATE_ENABLE: c_uint = 0x3;

// DECON_UPDATE
pub const DECON_UPDATE: c_uint = 0x710;

