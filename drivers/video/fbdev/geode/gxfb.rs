//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/geode/gxfb.h
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
// Copyright (C) 2008 Andres Salomon <dilinger@debian.org>
//
// Geode GX2 header information
//

pub const DC_PAL_COUNT: c_uint = 0x104;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxfb_par {
    pub enable_crt: c_int,
    pub dc_regs: *mut void __iomem,
    pub vid_regs: *mut void __iomem,
    pub gp_regs: *mut void __iomem,
    pub powered_down: c_int,
// register state, for power management functionality
    pub padsel: u64,
    pub dotpll: u64,
    pub msr: },
    pub gp: [u32; GP_REG_COUNT],
    pub dc: [u32; DC_REG_COUNT],
    pub vp: [u64; VP_REG_COUNT],
    pub fp: [u64; FP_REG_COUNT],
    pub pal: [u32; DC_PAL_COUNT],
}

extern "C" {
    pub fn gx_frame_buffer_size() -> c_uint;
}
extern "C" {
    pub fn gx_line_delta(xres: c_int, bpp: c_int) -> c_int;
}
extern "C" {
    pub fn gx_set_mode(info: *mut fb_info);
}
extern "C" {
    pub fn gx_set_dclk_frequency(info: *mut fb_info);
}
extern "C" {
    pub fn gx_configure_display(info: *mut fb_info);
}
extern "C" {
    pub fn gx_blank_display(info: *mut fb_info, blank_mode: c_int) -> c_int;
}
extern "C" {
    pub fn gx_powerdown(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn gx_powerup(info: *mut fb_info) -> c_int;
}
// Graphics Processor registers (table 6-23 from the data book)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gp_registers {
    GP_DST_OFFSET = 0,
    GP_SRC_OFFSET,
    GP_STRIDE,
    GP_WID_HEIGHT,

    GP_SRC_COLOR_FG,
    GP_SRC_COLOR_BG,
    GP_PAT_COLOR_0,
    GP_PAT_COLOR_1,

    GP_PAT_COLOR_2,
    GP_PAT_COLOR_3,
    GP_PAT_COLOR_4,
    GP_PAT_COLOR_5,

    GP_PAT_DATA_0,
    GP_PAT_DATA_1,
    GP_RASTER_MODE,
    GP_VECTOR_MODE,

    GP_BLT_MODE,
    GP_BLT_STATUS,
    GP_HST_SRC,
    GP_BASE_OFFSET, /* 0x4c */
}

// Display Controller registers (table 6-38 from the data book)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_registers {
    DC_UNLOCK = 0,
    DC_GENERAL_CFG,
    DC_DISPLAY_CFG,
    DC_RSVD_0,

    DC_FB_ST_OFFSET,
    DC_CB_ST_OFFSET,
    DC_CURS_ST_OFFSET,
    DC_ICON_ST_OFFSET,

    DC_VID_Y_ST_OFFSET,
    DC_VID_U_ST_OFFSET,
    DC_VID_V_ST_OFFSET,
    DC_RSVD_1,

    DC_LINE_SIZE,
    DC_GFX_PITCH,
    DC_VID_YUV_PITCH,
    DC_RSVD_2,

    DC_H_ACTIVE_TIMING,
    DC_H_BLANK_TIMING,
    DC_H_SYNC_TIMING,
    DC_RSVD_3,

    DC_V_ACTIVE_TIMING,
    DC_V_BLANK_TIMING,
    DC_V_SYNC_TIMING,
    DC_RSVD_4,

    DC_CURSOR_X,
    DC_CURSOR_Y,
    DC_ICON_X,
    DC_LINE_CNT,

    DC_PAL_ADDRESS,
    DC_PAL_DATA,
    DC_DFIFO_DIAG,
    DC_CFIFO_DIAG,

    DC_VID_DS_DELTA,
    DC_GLIU0_MEM_OFFSET,
    DC_RSVD_5,
    DC_DV_ACC, /* 0x8c */
}

pub const DC_UNLOCK_LOCK: c_uint = 0x00000000;
pub const DC_UNLOCK_UNLOCK: c_uint = 0x00004758	/* magic value */;

pub const DC_GENERAL_CFG_DFHPEL_SHIFT: c_int = 12;
pub const DC_GENERAL_CFG_DFHPSL_SHIFT: c_int = 8;

//
// Video Processor registers (table 6-54).
// There is space for 64 bit values, but we never use more than the
// lower 32 bits.  The actual register save/restore code only bothers
// to restore those 32 bits.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vp_registers {
    VP_VCFG = 0,
    VP_DCFG,

    VP_VX,
    VP_VY,

    VP_VS,
    VP_VCK,

    VP_VCM,
    VP_GAR,

    VP_GDR,
    VP_RSVD_0,

    VP_MISC,
    VP_CCS,

    VP_RSVD_1,
    VP_RSVD_2,

    VP_RSVD_3,
    VP_VDC,

    VP_VCO,
    VP_CRC,

    VP_CRC32,
    VP_VDE,

    VP_CCK,
    VP_CCM,

    VP_CC1,
    VP_CC2,

    VP_A1X,
    VP_A1Y,

    VP_A1C,
    VP_A1T,

    VP_A2X,
    VP_A2Y,

    VP_A2C,
    VP_A2T,

    VP_A3X,
    VP_A3Y,

    VP_A3C,
    VP_A3T,

    VP_VRR,
    VP_AWT,

    VP_VTM, /* 0x130 */
}

//
// Flat Panel registers (table 6-55).
// Also 64 bit registers; see above note about 32-bit handling.
//
// we're actually in the VP register space, starting at address 0x400
pub const VP_FP_START: c_uint = 0x400;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fp_registers {
    FP_PT1 = 0,
    FP_PT2,

    FP_PM,
    FP_DFC,

    FP_BLFSR,
    FP_RLFSR,

    FP_FMI,
    FP_FMD,

    FP_RSVD_0,
    FP_DCA,

    FP_DMD,
    FP_CRC,

    FP_FBB, /* 0x460 */
}

pub const FP_PT1_VSIZE_MASK: c_uint = 0x7FF0000	/* undocumented? */;

// register access functions
extern "C" {
    pub fn readl(4*reg: *mut par->gp_regs +) -> return;
}
extern "C" {
    pub fn readl(4*reg: *mut par->dc_regs +) -> return;
}
extern "C" {
    pub fn readl(8*reg: *mut par->vid_regs +) -> return;
}
extern "C" {
    pub fn readl(VP_FP_START: *mut *mut par->vid_regs + 8reg +) -> return;
}
// MSRs are defined in linux/cs5535.h; their bitfields are here

pub const MSR_GX_MSR_PADSEL_MASK: c_uint = 0x3FFFFFFF	/* undocumented? */;
pub const MSR_GX_MSR_PADSEL_TFT: c_uint = 0x1FFFFFFF	/* undocumented? */;

