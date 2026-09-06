//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/psb_reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) (2005-2007) Imagination Technologies Limited.
// Copyright (c) 2007, Intel Corporation.
// All Rights Reserved.
//
pub const PSB_CR_CLKGATECTL: c_uint = 0x0000;

pub const PSB_CR_CORE_ID: c_uint = 0x0010;

pub const PSB_CR_CORE_REVISION: c_uint = 0x0014;

pub const PSB_CR_DESIGNER_REV_FIELD1: c_uint = 0x0018;
pub const PSB_CR_SOFT_RESET: c_uint = 0x0080;

pub const PSB_CR_DESIGNER_REV_FIELD2: c_uint = 0x001C;
pub const PSB_CR_EVENT_HOST_ENABLE2: c_uint = 0x0110;
pub const PSB_CR_EVENT_STATUS2: c_uint = 0x0118;
pub const PSB_CR_EVENT_HOST_CLEAR2: c_uint = 0x0114;

pub const PSB_CR_EVENT_STATUS: c_uint = 0x012C;
pub const PSB_CR_EVENT_HOST_ENABLE: c_uint = 0x0130;
pub const PSB_CR_EVENT_HOST_CLEAR: c_uint = 0x0134;

pub const PSB_USE_OFFSET_MASK: c_uint = 0x0007FFFF;

pub const PSB_CR_USE_CODE_BASE0: c_uint = 0x0A0C;
pub const PSB_CR_USE_CODE_BASE1: c_uint = 0x0A10;
pub const PSB_CR_USE_CODE_BASE2: c_uint = 0x0A14;
pub const PSB_CR_USE_CODE_BASE3: c_uint = 0x0A18;
pub const PSB_CR_USE_CODE_BASE4: c_uint = 0x0A1C;
pub const PSB_CR_USE_CODE_BASE5: c_uint = 0x0A20;
pub const PSB_CR_USE_CODE_BASE6: c_uint = 0x0A24;
pub const PSB_CR_USE_CODE_BASE7: c_uint = 0x0A28;
pub const PSB_CR_USE_CODE_BASE8: c_uint = 0x0A2C;
pub const PSB_CR_USE_CODE_BASE9: c_uint = 0x0A30;
pub const PSB_CR_USE_CODE_BASE10: c_uint = 0x0A34;
pub const PSB_CR_USE_CODE_BASE11: c_uint = 0x0A38;
pub const PSB_CR_USE_CODE_BASE12: c_uint = 0x0A3C;
pub const PSB_CR_USE_CODE_BASE13: c_uint = 0x0A40;
pub const PSB_CR_USE_CODE_BASE14: c_uint = 0x0A44;
pub const PSB_CR_USE_CODE_BASE15: c_uint = 0x0A48;

pub const PSB_CR_PDS_EXEC_BASE: c_uint = 0x0AB8;

pub const PSB_CR_EVENT_KICKER: c_uint = 0x0AC4;

pub const PSB_CR_EVENT_KICK: c_uint = 0x0AC8;

pub const PSB_CR_BIF_DIR_LIST_BASE1: c_uint = 0x0C38;
pub const PSB_CR_BIF_CTRL: c_uint = 0x0C00;

pub const PSB_CR_BIF_INT_STAT: c_uint = 0x0C04;
pub const PSB_CR_BIF_FAULT: c_uint = 0x0C08;

pub const PSB_CR_BIF_BANK0: c_uint = 0x0C78;
pub const PSB_CR_BIF_BANK1: c_uint = 0x0C7C;
pub const PSB_CR_BIF_DIR_LIST_BASE0: c_uint = 0x0C84;
pub const PSB_CR_BIF_TWOD_REQ_BASE: c_uint = 0x0C88;
pub const PSB_CR_BIF_3D_REQ_BASE: c_uint = 0x0CAC;
pub const PSB_CR_2D_SOCIF: c_uint = 0x0E18;

pub const PSB_CR_2D_BLIT_STATUS: c_uint = 0x0E04;

//
// 2D defs.
//
// 2D Slave Port Data : Block Header's Object Type
//

//
// Clip Definition block (PSB_2D_CLIP_BH)
//

// clip rectangle min & max

// clip rectangle offset

//
// Pattern Control (PSB_2D_PAT_BH)
//

//
// 2D Control block (PSB_2D_CTRL_BH)
//
// Present Flags

// Colour Key Colour (SRC/DST)

// Colour Key Mask (SRC/DST)

// Alpha Control (Alpha/RGB)

//
// Source Offset (PSB_2D_SRC_OFF_BH)
//

//
// Mask Offset (PSB_2D_MASK_OFF_BH)
//

//
// 2D Fence (see PSB_2D_FENCE_BH): bits 0:27 are ignored
//
// Blit Rectangle (PSB_2D_BLIT_BH)
//

//
// Tungsten Graphics note on rop codes: If rop A and rop B are
// identical, the mask surface will not be read and need not be
// set up.
//

// rop code A

//
// DWORD0:	(Only pass if Pattern control == Use Fill Colour)
// Fill Colour RGBA8888
//

//
// DWORD1: (Always Present)
// X Start (Dest)
// Y Start (Dest)
//

//
// DWORD2: (Always Present)
// X Size (Dest)
// Y Size (Dest)
//

//
// Source Surface (PSB_2D_SRC_SURF_BH)
//
// WORD 0
//

//
// WORD 1 - Base Address
//

//
// Pattern Surface (PSB_2D_PAT_SURF_BH)
//
// WORD 0
//

//
// WORD 1 - Base Address
//

//
// Destination Surface (PSB_2D_DST_SURF_BH)
//
// WORD 0
//

//
// WORD 1 - Base Address
//

//
// Mask Surface (PSB_2D_MASK_SURF_BH)
//
// WORD 0
//

//
// WORD 1 - Base Address
//

//
// Source Palette (PSB_2D_SRC_PAL_BH)
//

//
// Pattern Palette (PSB_2D_PAT_PAL_BH)
//

//
// Rop3 Codes (2 LS bytes)
//

//
// Sizes.
//
pub const PSB_SCENE_HW_COOKIE_SIZE: c_int = 16;
pub const PSB_TA_MEM_HW_COOKIE_SIZE: c_int = 16;
//
// Scene stuff.
//
pub const PSB_NUM_HW_SCENES: c_int = 2;
//
// Scheduler completion actions.
//
pub const PSB_RASTER_BLOCK: c_int = 0;
pub const PSB_RASTER: c_int = 1;
pub const PSB_RETURN: c_int = 2;
pub const PSB_TA: c_int = 3;
// Power management
pub const PSB_PUNIT_PORT: c_uint = 0x04;
pub const PSB_OSPMBA: c_uint = 0x78;
pub const PSB_APMBA: c_uint = 0x7a;
pub const PSB_APM_CMD: c_uint = 0x0;
pub const PSB_APM_STS: c_uint = 0x04;
pub const PSB_PWRGT_VID_ENC_MASK: c_uint = 0x30;
pub const PSB_PWRGT_VID_DEC_MASK: c_uint = 0xc;
pub const PSB_PWRGT_GL3_MASK: c_uint = 0xc0;
pub const PSB_PM_SSC: c_uint = 0x20;
pub const PSB_PM_SSS: c_uint = 0x30;
pub const PSB_PWRGT_DISPLAY_MASK: c_uint = 0xc /*on a different BA than video/gfx*/;
// Display SSS register bits are different in A0 vs. B0
pub const PSB_PWRGT_GFX_MASK: c_uint = 0x3;
pub const PSB_PWRGT_GFX_MASK_B0: c_uint = 0xc3;
