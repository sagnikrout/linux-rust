//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/accel.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//
pub const FB_ACCEL_VIA_UNICHROME: c_int = 50;
// MMIO Base Address Definition
pub const MMIO_VGABASE: c_uint = 0x8000;

// HW Cursor Status Define
pub const HW_Cursor_ON: c_int = 0;
pub const HW_Cursor_OFF: c_int = 1;

pub const VIA_MMIO_BLTBASE: c_uint = 0x200000;
pub const VIA_MMIO_BLTSIZE: c_uint = 0x200000;
// Defines for 2D registers
pub const VIA_REG_GECMD: c_uint = 0x000;
pub const VIA_REG_GEMODE: c_uint = 0x004;
pub const VIA_REG_SRCPOS: c_uint = 0x008;
pub const VIA_REG_DSTPOS: c_uint = 0x00C;
// width and height
pub const VIA_REG_DIMENSION: c_uint = 0x010;
pub const VIA_REG_PATADDR: c_uint = 0x014;
pub const VIA_REG_FGCOLOR: c_uint = 0x018;
pub const VIA_REG_BGCOLOR: c_uint = 0x01C;
// top and left of clipping
pub const VIA_REG_CLIPTL: c_uint = 0x020;
// bottom and right of clipping
pub const VIA_REG_CLIPBR: c_uint = 0x024;
pub const VIA_REG_OFFSET: c_uint = 0x028;
// color key control
pub const VIA_REG_KEYCONTROL: c_uint = 0x02C;
pub const VIA_REG_SRCBASE: c_uint = 0x030;
pub const VIA_REG_DSTBASE: c_uint = 0x034;
// pitch of src and dst
pub const VIA_REG_PITCH: c_uint = 0x038;
pub const VIA_REG_MONOPAT0: c_uint = 0x03C;
pub const VIA_REG_MONOPAT1: c_uint = 0x040;
// from 0x100 to 0x1ff
pub const VIA_REG_COLORPAT: c_uint = 0x100;
// defines for VIA 2D registers for vt3353/3409 (M1 engine)
pub const VIA_REG_GECMD_M1: c_uint = 0x000;
pub const VIA_REG_GEMODE_M1: c_uint = 0x004;
pub const VIA_REG_GESTATUS_M1: c_uint = 0x004       /* as same as VIA_REG_GEMODE */;
pub const VIA_REG_PITCH_M1: c_uint = 0x008       /* pitch of src and dst */;
pub const VIA_REG_DIMENSION_M1: c_uint = 0x00C       /* width and height */;
pub const VIA_REG_DSTPOS_M1: c_uint = 0x010;
pub const VIA_REG_LINE_XY_M1: c_uint = 0x010;
pub const VIA_REG_DSTBASE_M1: c_uint = 0x014;
pub const VIA_REG_SRCPOS_M1: c_uint = 0x018;
pub const VIA_REG_LINE_K1K2_M1: c_uint = 0x018;
pub const VIA_REG_SRCBASE_M1: c_uint = 0x01C;
pub const VIA_REG_PATADDR_M1: c_uint = 0x020;
pub const VIA_REG_MONOPAT0_M1: c_uint = 0x024;
pub const VIA_REG_MONOPAT1_M1: c_uint = 0x028;
pub const VIA_REG_OFFSET_M1: c_uint = 0x02C;
pub const VIA_REG_LINE_ERROR_M1: c_uint = 0x02C;
pub const VIA_REG_CLIPTL_M1: c_uint = 0x040       /* top and left of clipping */;
pub const VIA_REG_CLIPBR_M1: c_uint = 0x044       /* bottom and right of clipping */;
pub const VIA_REG_KEYCONTROL_M1: c_uint = 0x048       /* color key control */;
pub const VIA_REG_FGCOLOR_M1: c_uint = 0x04C;
pub const VIA_REG_DSTCOLORKEY_M1: c_uint = 0x04C       /* as same as VIA_REG_FG */;
pub const VIA_REG_BGCOLOR_M1: c_uint = 0x050;
pub const VIA_REG_SRCCOLORKEY_M1: c_uint = 0x050       /* as same as VIA_REG_BG */;
pub const VIA_REG_MONOPATFGC_M1: c_uint = 0x058       /* Add BG color of Pattern. */;
pub const VIA_REG_MONOPATBGC_M1: c_uint = 0x05C       /* Add FG color of Pattern. */;
pub const VIA_REG_COLORPAT_M1: c_uint = 0x100       /* from 0x100 to 0x1ff */;
// VIA_REG_PITCH(0x38): Pitch Setting
pub const VIA_PITCH_ENABLE: c_uint = 0x80000000;
// defines for VIA HW cursor registers
pub const VIA_REG_CURSOR_MODE: c_uint = 0x2D0;
pub const VIA_REG_CURSOR_POS: c_uint = 0x2D4;
pub const VIA_REG_CURSOR_ORG: c_uint = 0x2D8;
pub const VIA_REG_CURSOR_BG: c_uint = 0x2DC;
pub const VIA_REG_CURSOR_FG: c_uint = 0x2E0;
// VIA_REG_GEMODE(0x04): GE mode
pub const VIA_GEM_8bpp: c_uint = 0x00000000;
pub const VIA_GEM_16bpp: c_uint = 0x00000100;
pub const VIA_GEM_32bpp: c_uint = 0x00000300;
// VIA_REG_GECMD(0x00): 2D Engine Command
pub const VIA_GEC_NOOP: c_uint = 0x00000000;
pub const VIA_GEC_BLT: c_uint = 0x00000001;
pub const VIA_GEC_LINE: c_uint = 0x00000005;
// Rotate Command
pub const VIA_GEC_ROT: c_uint = 0x00000008;
pub const VIA_GEC_SRC_XY: c_uint = 0x00000000;
pub const VIA_GEC_SRC_LINEAR: c_uint = 0x00000010;
pub const VIA_GEC_DST_XY: c_uint = 0x00000000;
pub const VIA_GEC_DST_LINRAT: c_uint = 0x00000020;
pub const VIA_GEC_SRC_FB: c_uint = 0x00000000;
pub const VIA_GEC_SRC_SYS: c_uint = 0x00000040;
pub const VIA_GEC_DST_FB: c_uint = 0x00000000;
pub const VIA_GEC_DST_SYS: c_uint = 0x00000080;
// source is mono
pub const VIA_GEC_SRC_MONO: c_uint = 0x00000100;
// pattern is mono
pub const VIA_GEC_PAT_MONO: c_uint = 0x00000200;
// mono src is opaque
pub const VIA_GEC_MSRC_OPAQUE: c_uint = 0x00000000;
// mono src is transparent
pub const VIA_GEC_MSRC_TRANS: c_uint = 0x00000400;
// pattern is in frame buffer
pub const VIA_GEC_PAT_FB: c_uint = 0x00000000;
// pattern is from reg setting
pub const VIA_GEC_PAT_REG: c_uint = 0x00000800;
pub const VIA_GEC_CLIP_DISABLE: c_uint = 0x00000000;
pub const VIA_GEC_CLIP_ENABLE: c_uint = 0x00001000;
pub const VIA_GEC_FIXCOLOR_PAT: c_uint = 0x00002000;
pub const VIA_GEC_INCX: c_uint = 0x00000000;
pub const VIA_GEC_DECY: c_uint = 0x00004000;
pub const VIA_GEC_INCY: c_uint = 0x00000000;
pub const VIA_GEC_DECX: c_uint = 0x00008000;
// mono pattern is opaque
pub const VIA_GEC_MPAT_OPAQUE: c_uint = 0x00000000;
// mono pattern is transparent
pub const VIA_GEC_MPAT_TRANS: c_uint = 0x00010000;
pub const VIA_GEC_MONO_UNPACK: c_uint = 0x00000000;
pub const VIA_GEC_MONO_PACK: c_uint = 0x00020000;
pub const VIA_GEC_MONO_DWORD: c_uint = 0x00000000;
pub const VIA_GEC_MONO_WORD: c_uint = 0x00040000;
pub const VIA_GEC_MONO_BYTE: c_uint = 0x00080000;
pub const VIA_GEC_LASTPIXEL_ON: c_uint = 0x00000000;
pub const VIA_GEC_LASTPIXEL_OFF: c_uint = 0x00100000;
pub const VIA_GEC_X_MAJOR: c_uint = 0x00000000;
pub const VIA_GEC_Y_MAJOR: c_uint = 0x00200000;
pub const VIA_GEC_QUICK_START: c_uint = 0x00800000;
// defines for VIA 3D registers
pub const VIA_REG_STATUS: c_uint = 0x400;
pub const VIA_REG_CR_TRANSET: c_uint = 0x41C;
pub const VIA_REG_CR_TRANSPACE: c_uint = 0x420;
pub const VIA_REG_TRANSET: c_uint = 0x43C;
pub const VIA_REG_TRANSPACE: c_uint = 0x440;
// VIA_REG_STATUS(0x400): Engine Status
// Command Regulator is busy
pub const VIA_CMD_RGTR_BUSY: c_uint = 0x00000080;
// 2D Engine is busy
pub const VIA_2D_ENG_BUSY: c_uint = 0x00000002;
// 3D Engine is busy
pub const VIA_3D_ENG_BUSY: c_uint = 0x00000001;
// Virtual Queue is busy
pub const VIA_VR_QUEUE_BUSY: c_uint = 0x00020000;
// VIA_REG_STATUS(0x400): Engine Status for H5
pub const VIA_CMD_RGTR_BUSY_H5: c_uint = 0x00000010  /* Command Regulator is busy */;
pub const VIA_2D_ENG_BUSY_H5: c_uint = 0x00000002  /* 2D Engine is busy */;
pub const VIA_3D_ENG_BUSY_H5: c_uint = 0x00001FE1  /* 3D Engine is busy */;
pub const VIA_VR_QUEUE_BUSY_H5: c_uint = 0x00000004  /* Virtual Queue is busy */;
// VIA_REG_STATUS(0x400): Engine Status for VT3353/3409
pub const VIA_CMD_RGTR_BUSY_M1: c_uint = 0x00000010  /* Command Regulator is busy */;
pub const VIA_2D_ENG_BUSY_M1: c_uint = 0x00000002  /* 2D Engine is busy */;
pub const VIA_3D_ENG_BUSY_M1: c_uint = 0x00001FE1  /* 3D Engine is busy */;
pub const VIA_VR_QUEUE_BUSY_M1: c_uint = 0x00000004  /* Virtual Queue is busy */;
pub const MAXLOOP: c_uint = 0xFFFFFF;
pub const VIA_BITBLT_COLOR: c_int = 1;
pub const VIA_BITBLT_MONO: c_int = 2;
pub const VIA_BITBLT_FILL: c_int = 3;
extern "C" {
    pub fn viafb_setup_engine(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn viafb_reset_engine(viapar: *mut viafb_par);
}
extern "C" {
    pub fn viafb_show_hw_cursor(info: *mut fb_info, Status: c_int);
}
extern "C" {
    pub fn viafb_wait_engine_idle(info: *mut fb_info);
}
