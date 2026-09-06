//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/geode/lxfb.h
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
// Geode LX framebuffer driver
//
// Copyright (C) 2006-2007, Advanced Micro Devices,Inc.
// Copyright (c) 2008  Andres Salomon <dilinger@debian.org>
//

pub const DC_PAL_COUNT: c_uint = 0x104;
pub const DC_HFILT_COUNT: c_uint = 0x100;
pub const DC_VFILT_COUNT: c_uint = 0x100;
pub const VP_COEFF_SIZE: c_uint = 0x1000;
pub const VP_PAL_COUNT: c_uint = 0x100;
pub const OUTPUT_CRT: c_uint = 0x01;
pub const OUTPUT_PANEL: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxfb_par {
    pub output: c_int,
    pub gp_regs: *mut void __iomem,
    pub dc_regs: *mut void __iomem,
    pub vp_regs: *mut void __iomem,
    pub powered_down: c_int,
// register state, for power mgmt functionality
    pub padsel: u64,
    pub dotpll: u64,
    pub dfglcfg: u64,
    pub dcspare: u64,
    pub msr: },
    pub gp: [u32; GP_REG_COUNT],
    pub dc: [u32; DC_REG_COUNT],
    pub vp: [u64; VP_REG_COUNT],
    pub fp: [u64; FP_REG_COUNT],
    pub dc_pal: [u32; DC_PAL_COUNT],
    pub vp_pal: [u32; VP_PAL_COUNT],
    pub 2]: *mut *mut uint32_t hcoeff[DC_HFILT_COUNT,
    pub vcoeff: [u32; DC_VFILT_COUNT],
    pub 4]: uint32_t vp_coeff[VP_COEFF_SIZE /,
}

extern "C" {
    pub fn lx_set_mode(: *mut fb_info);
}
extern "C" {
    pub fn lx_framebuffer_size() -> c_uint;
}
extern "C" {
    pub fn lx_blank_display(: *mut fb_info, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn lx_powerdown(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn lx_powerup(info: *mut fb_info) -> c_int;
}
// Graphics Processor registers (table 6-29 from the data book)
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
    GP_BASE_OFFSET,

    GP_CMD_TOP,
    GP_CMD_BOT,
    GP_CMD_READ,
    GP_CMD_WRITE,

    GP_CH3_OFFSET,
    GP_CH3_MODE_STR,
    GP_CH3_WIDHI,
    GP_CH3_HSRC,

    GP_LUT_INDEX,
    GP_LUT_DATA,
    GP_INT_CNTRL, /* 0x78 */
}

// Display Controller registers (table 6-47 from the data book)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_registers {
    DC_UNLOCK = 0,
    DC_GENERAL_CFG,
    DC_DISPLAY_CFG,
    DC_ARB_CFG,

    DC_FB_ST_OFFSET,
    DC_CB_ST_OFFSET,
    DC_CURS_ST_OFFSET,
    DC_RSVD_0,

    DC_VID_Y_ST_OFFSET,
    DC_VID_U_ST_OFFSET,
    DC_VID_V_ST_OFFSET,
    DC_DV_TOP,

    DC_LINE_SIZE,
    DC_GFX_PITCH,
    DC_VID_YUV_PITCH,
    DC_RSVD_1,

    DC_H_ACTIVE_TIMING,
    DC_H_BLANK_TIMING,
    DC_H_SYNC_TIMING,
    DC_RSVD_2,

    DC_V_ACTIVE_TIMING,
    DC_V_BLANK_TIMING,
    DC_V_SYNC_TIMING,
    DC_FB_ACTIVE,

    DC_CURSOR_X,
    DC_CURSOR_Y,
    DC_RSVD_3,
    DC_LINE_CNT,

    DC_PAL_ADDRESS,
    DC_PAL_DATA,
    DC_DFIFO_DIAG,
    DC_CFIFO_DIAG,

    DC_VID_DS_DELTA,
    DC_GLIU0_MEM_OFFSET,
    DC_DV_CTL,
    DC_DV_ACCESS,

    DC_GFX_SCALE,
    DC_IRQ_FILT_CTL,
    DC_FILT_COEFF1,
    DC_FILT_COEFF2,

    DC_VBI_EVEN_CTL,
    DC_VBI_ODD_CTL,
    DC_VBI_HOR,
    DC_VBI_LN_ODD,

    DC_VBI_LN_EVEN,
    DC_VBI_PITCH,
    DC_CLR_KEY,
    DC_CLR_KEY_MASK,

    DC_CLR_KEY_X,
    DC_CLR_KEY_Y,
    DC_IRQ,
    DC_RSVD_4,

    DC_RSVD_5,
    DC_GENLK_CTL,
    DC_VID_EVEN_Y_ST_OFFSET,
    DC_VID_EVEN_U_ST_OFFSET,

    DC_VID_EVEN_V_ST_OFFSET,
    DC_V_ACTIVE_EVEN_TIMING,
    DC_V_BLANK_EVEN_TIMING,
    DC_V_SYNC_EVEN_TIMING,	/* 0xec */
}

pub const DC_UNLOCK_LOCK: c_uint = 0x00000000;
pub const DC_UNLOCK_UNLOCK: c_uint = 0x00004758	/* magic value */;

//
// Video Processor registers (table 6-71).
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

    VP_SCL,
    VP_VCK,

    VP_VCM,
    VP_PAR,

    VP_PDR,
    VP_SLR,

    VP_MISC,
    VP_CCS,

    VP_VYS,
    VP_VXS,

    VP_RSVD_0,
    VP_VDC,

    VP_RSVD_1,
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

    VP_VTM,
    VP_VYE,

    VP_A1YE,
    VP_A2YE,

    VP_A3YE,	/* 0x150 */

    VP_VCR = 0x1000, /* 0x1000 - 0x1fff */
}

//
// Flat Panel registers (table 6-71).
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

    FP_RSVD_0,
    FP_RSVD_1,

    FP_RSVD_2,
    FP_RSVD_3,

    FP_RSVD_4,
    FP_DCA,

    FP_DMD,
    FP_CRC, /* 0x458 */
}

// register access functions
extern "C" {
    pub fn readl(4*reg: *mut par->gp_regs +) -> return;
}
extern "C" {
    pub fn readl(4*reg: *mut par->dc_regs +) -> return;
}
extern "C" {
    pub fn readl(8*reg: *mut par->vp_regs +) -> return;
}
extern "C" {
    pub fn readl(VP_FP_START: *mut *mut par->vp_regs + 8reg +) -> return;
}
// MSRs are defined in linux/cs5535.h; their bitfields are here

// note: this is actually the VP's GLD_MSR_CONFIG

pub const MSR_LX_MSR_PADSEL_TFT_SEL_LOW: c_uint = 0xDFFFFFFF	/* ??? */;
pub const MSR_LX_MSR_PADSEL_TFT_SEL_HIGH: c_uint = 0x0000003F	/* ??? */;

