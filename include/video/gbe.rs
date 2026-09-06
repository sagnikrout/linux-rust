//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/gbe.h
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
// include/video/gbe.h -- SGI GBE (Graphics Back End)
//
// Copyright (C) 1999 Silicon Graphics, Inc. (Jeffrey Newquist)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgi_gbe {
    pub /: *mut *mut volatile uint32_t ctrlstat; / general control,
    pub /: *mut *mut volatile uint32_t dotclock; / dot clock PLL control,
    pub /: *mut *mut volatile uint32_t i2c; / crt I2C control,
    pub /: *mut *mut volatile uint32_t sysclk; / system clock PLL control,
    pub /: *mut *mut volatile uint32_t i2cfp; / flat panel I2C control,
    pub /: *mut *mut volatile uint32_t id; / device id/chip revision,
    pub /: *mut *mut volatile uint32_t config; / power on configuration [1],
    pub /: *mut *mut volatile uint32_t bist; / internal bist status [1],
    pub 8]: uint32_t _pad0[0x010000/4 -,
    pub /: *mut *mut volatile uint32_t vt_xy; / current dot coords,
    pub /: *mut *mut volatile uint32_t vt_xymax; / maximum dot coords,
    pub /: *mut *mut volatile uint32_t vt_vsync; / vsync on/off,
    pub /: *mut *mut volatile uint32_t vt_hsync; / hsync on/off,
    pub /: *mut *mut volatile uint32_t vt_vblank; / vblank on/off,
    pub /: *mut *mut volatile uint32_t vt_hblank; / hblank on/off,
    pub /: *mut *mut volatile uint32_t vt_flags; / polarity of vt signals,
    pub /: *mut *mut volatile uint32_t vt_f2rf_lock; / f2rf & framelck y coord,
    pub /: *mut *mut volatile uint32_t vt_intr01; / intr 0,1 y coords,
    pub /: *mut *mut volatile uint32_t vt_intr23; / intr 2,3 y coords,
    pub /: *mut *mut volatile uint32_t fp_hdrv; / flat panel hdrv on/off,
    pub /: *mut *mut volatile uint32_t fp_vdrv; / flat panel vdrv on/off,
    pub /: *mut *mut volatile uint32_t fp_de; / flat panel de on/off,
    pub /: *mut *mut volatile uint32_t vt_hpixen; / intrnl horiz pixel on/off,
    pub /: *mut *mut volatile uint32_t vt_vpixen; / intrnl vert pixel on/off,
    pub /: *mut *mut volatile uint32_t vt_hcmap; / cmap write (horiz),
    pub /: *mut *mut volatile uint32_t vt_vcmap; / cmap write (vert),
    pub /: *mut *mut volatile uint32_t did_start_xy; / eol/f did/xy reset val,
    pub /: *mut *mut volatile uint32_t crs_start_xy; / eol/f crs/xy reset val,
    pub /: *mut *mut volatile uint32_t vc_start_xy; / eol/f vc/xy reset val,
    pub _pad1: [u32; 0xffb0/4],
    pub /: *mut *mut volatile uint32_t ovr_width_tile;/overlay plane ctrl 0,
    pub /: *mut *mut volatile uint32_t ovr_inhwctrl; / overlay plane ctrl 1,
    pub /: *mut *mut volatile uint32_t ovr_control; / overlay plane ctrl 1,
    pub _pad2: [u32; 0xfff4/4],
    pub /: *mut *mut volatile uint32_t frm_size_tile;/ normal plane ctrl 0,
    pub /: *mut *mut volatile uint32_t frm_size_pixel;/normal plane ctrl 1,
    pub /: *mut *mut volatile uint32_t frm_inhwctrl; / normal plane ctrl 2,
    pub /: *mut *mut volatile uint32_t frm_control; / normal plane ctrl 3,
    pub _pad3: [u32; 0xfff0/4],
    pub /: *mut *mut volatile uint32_t did_inhwctrl; / DID control,
    pub /: *mut *mut volatile uint32_t did_control; / DID shadow,
    pub _pad4: [u32; 0x7ff8/4],
    pub /: *mut *mut volatile uint32_t mode_regs[32];/ WID table,
    pub _pad5: [u32; 0x7f80/4],
    pub /: *mut *mut volatile uint32_t cmap[6144]; / color map,
    pub _pad6: [u32; 0x2000/4],
    pub /: *mut *mut volatile uint32_t cm_fifo; / color map fifo status,
    pub _pad7: [u32; 0x7ffc/4],
    pub /: *mut *mut volatile uint32_t gmap[256]; / gamma map,
    pub _pad8: [u32; 0x7c00/4],
    pub /: *mut *mut volatile uint32_t gmap10[1024]; / gamma map,
    pub _pad9: [u32; 0x7000/4],
    pub /: *mut *mut volatile uint32_t crs_pos; / cusror control 0,
    pub /: *mut *mut volatile uint32_t crs_ctl; / cusror control 1,
    pub /: *mut *mut volatile uint32_t crs_cmap[3]; / crs cmap,
    pub _pad10: [u32; 0x7fec/4],
    pub /: *mut *mut volatile uint32_t crs_glyph[64];/ crs glyph,
    pub _pad11: [u32; 0x7f00/4],
    pub /: *mut *mut volatile uint32_t vc_0; / video capture crtl 0,
    pub /: *mut *mut volatile uint32_t vc_1; / video capture crtl 1,
    pub /: *mut *mut volatile uint32_t vc_2; / video capture crtl 2,
    pub /: *mut *mut volatile uint32_t vc_3; / video capture crtl 3,
    pub /: *mut *mut volatile uint32_t vc_4; / video capture crtl 4,
    pub /: *mut *mut volatile uint32_t vc_5; / video capture crtl 5,
    pub /: *mut *mut volatile uint32_t vc_6; / video capture crtl 6,
    pub /: *mut *mut volatile uint32_t vc_7; / video capture crtl 7,
    pub /: *mut *mut volatile uint32_t vc_8; / video capture crtl 8,
}

//
// Bit mask information
//
pub const GBE_CTRLSTAT_CHIPID_MSB: c_int = 3;
pub const GBE_CTRLSTAT_CHIPID_LSB: c_int = 0;
pub const GBE_CTRLSTAT_SENSE_N_MSB: c_int = 4;
pub const GBE_CTRLSTAT_SENSE_N_LSB: c_int = 4;
pub const GBE_CTRLSTAT_PCLKSEL_MSB: c_int = 29;
pub const GBE_CTRLSTAT_PCLKSEL_LSB: c_int = 28;
pub const GBE_DOTCLK_M_MSB: c_int = 7;
pub const GBE_DOTCLK_M_LSB: c_int = 0;
pub const GBE_DOTCLK_N_MSB: c_int = 13;
pub const GBE_DOTCLK_N_LSB: c_int = 8;
pub const GBE_DOTCLK_P_MSB: c_int = 15;
pub const GBE_DOTCLK_P_LSB: c_int = 14;
pub const GBE_DOTCLK_RUN_MSB: c_int = 20;
pub const GBE_DOTCLK_RUN_LSB: c_int = 20;
pub const GBE_VT_XY_Y_MSB: c_int = 23;
pub const GBE_VT_XY_Y_LSB: c_int = 12;
pub const GBE_VT_XY_X_MSB: c_int = 11;
pub const GBE_VT_XY_X_LSB: c_int = 0;
pub const GBE_VT_XY_FREEZE_MSB: c_int = 31;
pub const GBE_VT_XY_FREEZE_LSB: c_int = 31;
pub const GBE_FP_VDRV_ON_MSB: c_int = 23;
pub const GBE_FP_VDRV_ON_LSB: c_int = 12;
pub const GBE_FP_VDRV_OFF_MSB: c_int = 11;
pub const GBE_FP_VDRV_OFF_LSB: c_int = 0;
pub const GBE_FP_HDRV_ON_MSB: c_int = 23;
pub const GBE_FP_HDRV_ON_LSB: c_int = 12;
pub const GBE_FP_HDRV_OFF_MSB: c_int = 11;
pub const GBE_FP_HDRV_OFF_LSB: c_int = 0;
pub const GBE_FP_DE_ON_MSB: c_int = 23;
pub const GBE_FP_DE_ON_LSB: c_int = 12;
pub const GBE_FP_DE_OFF_MSB: c_int = 11;
pub const GBE_FP_DE_OFF_LSB: c_int = 0;
pub const GBE_VT_VSYNC_VSYNC_ON_MSB: c_int = 23;
pub const GBE_VT_VSYNC_VSYNC_ON_LSB: c_int = 12;
pub const GBE_VT_VSYNC_VSYNC_OFF_MSB: c_int = 11;
pub const GBE_VT_VSYNC_VSYNC_OFF_LSB: c_int = 0;
pub const GBE_VT_HSYNC_HSYNC_ON_MSB: c_int = 23;
pub const GBE_VT_HSYNC_HSYNC_ON_LSB: c_int = 12;
pub const GBE_VT_HSYNC_HSYNC_OFF_MSB: c_int = 11;
pub const GBE_VT_HSYNC_HSYNC_OFF_LSB: c_int = 0;
pub const GBE_VT_VBLANK_VBLANK_ON_MSB: c_int = 23;
pub const GBE_VT_VBLANK_VBLANK_ON_LSB: c_int = 12;
pub const GBE_VT_VBLANK_VBLANK_OFF_MSB: c_int = 11;
pub const GBE_VT_VBLANK_VBLANK_OFF_LSB: c_int = 0;
pub const GBE_VT_HBLANK_HBLANK_ON_MSB: c_int = 23;
pub const GBE_VT_HBLANK_HBLANK_ON_LSB: c_int = 12;
pub const GBE_VT_HBLANK_HBLANK_OFF_MSB: c_int = 11;
pub const GBE_VT_HBLANK_HBLANK_OFF_LSB: c_int = 0;
pub const GBE_VT_FLAGS_F2RF_HIGH_MSB: c_int = 6;
pub const GBE_VT_FLAGS_F2RF_HIGH_LSB: c_int = 6;
pub const GBE_VT_FLAGS_SYNC_LOW_MSB: c_int = 5;
pub const GBE_VT_FLAGS_SYNC_LOW_LSB: c_int = 5;
pub const GBE_VT_FLAGS_SYNC_HIGH_MSB: c_int = 4;
pub const GBE_VT_FLAGS_SYNC_HIGH_LSB: c_int = 4;
pub const GBE_VT_FLAGS_HDRV_LOW_MSB: c_int = 3;
pub const GBE_VT_FLAGS_HDRV_LOW_LSB: c_int = 3;
pub const GBE_VT_FLAGS_HDRV_INVERT_MSB: c_int = 2;
pub const GBE_VT_FLAGS_HDRV_INVERT_LSB: c_int = 2;
pub const GBE_VT_FLAGS_VDRV_LOW_MSB: c_int = 1;
pub const GBE_VT_FLAGS_VDRV_LOW_LSB: c_int = 1;
pub const GBE_VT_FLAGS_VDRV_INVERT_MSB: c_int = 0;
pub const GBE_VT_FLAGS_VDRV_INVERT_LSB: c_int = 0;
pub const GBE_VT_VCMAP_VCMAP_ON_MSB: c_int = 23;
pub const GBE_VT_VCMAP_VCMAP_ON_LSB: c_int = 12;
pub const GBE_VT_VCMAP_VCMAP_OFF_MSB: c_int = 11;
pub const GBE_VT_VCMAP_VCMAP_OFF_LSB: c_int = 0;
pub const GBE_VT_HCMAP_HCMAP_ON_MSB: c_int = 23;
pub const GBE_VT_HCMAP_HCMAP_ON_LSB: c_int = 12;
pub const GBE_VT_HCMAP_HCMAP_OFF_MSB: c_int = 11;
pub const GBE_VT_HCMAP_HCMAP_OFF_LSB: c_int = 0;
pub const GBE_VT_XYMAX_MAXX_MSB: c_int = 11;
pub const GBE_VT_XYMAX_MAXX_LSB: c_int = 0;
pub const GBE_VT_XYMAX_MAXY_MSB: c_int = 23;
pub const GBE_VT_XYMAX_MAXY_LSB: c_int = 12;
pub const GBE_VT_HPIXEN_HPIXEN_ON_MSB: c_int = 23;
pub const GBE_VT_HPIXEN_HPIXEN_ON_LSB: c_int = 12;
pub const GBE_VT_HPIXEN_HPIXEN_OFF_MSB: c_int = 11;
pub const GBE_VT_HPIXEN_HPIXEN_OFF_LSB: c_int = 0;
pub const GBE_VT_VPIXEN_VPIXEN_ON_MSB: c_int = 23;
pub const GBE_VT_VPIXEN_VPIXEN_ON_LSB: c_int = 12;
pub const GBE_VT_VPIXEN_VPIXEN_OFF_MSB: c_int = 11;
pub const GBE_VT_VPIXEN_VPIXEN_OFF_LSB: c_int = 0;
pub const GBE_OVR_CONTROL_OVR_DMA_ENABLE_MSB: c_int = 0;
pub const GBE_OVR_CONTROL_OVR_DMA_ENABLE_LSB: c_int = 0;
pub const GBE_OVR_INHWCTRL_OVR_DMA_ENABLE_MSB: c_int = 0;
pub const GBE_OVR_INHWCTRL_OVR_DMA_ENABLE_LSB: c_int = 0;
pub const GBE_OVR_WIDTH_TILE_OVR_FIFO_RESET_MSB: c_int = 13;
pub const GBE_OVR_WIDTH_TILE_OVR_FIFO_RESET_LSB: c_int = 13;
pub const GBE_FRM_CONTROL_FRM_DMA_ENABLE_MSB: c_int = 0;
pub const GBE_FRM_CONTROL_FRM_DMA_ENABLE_LSB: c_int = 0;
pub const GBE_FRM_CONTROL_FRM_TILE_PTR_MSB: c_int = 31;
pub const GBE_FRM_CONTROL_FRM_TILE_PTR_LSB: c_int = 9;
pub const GBE_FRM_CONTROL_FRM_LINEAR_MSB: c_int = 1;
pub const GBE_FRM_CONTROL_FRM_LINEAR_LSB: c_int = 1;
pub const GBE_FRM_INHWCTRL_FRM_DMA_ENABLE_MSB: c_int = 0;
pub const GBE_FRM_INHWCTRL_FRM_DMA_ENABLE_LSB: c_int = 0;
pub const GBE_FRM_SIZE_TILE_FRM_WIDTH_TILE_MSB: c_int = 12;
pub const GBE_FRM_SIZE_TILE_FRM_WIDTH_TILE_LSB: c_int = 5;
pub const GBE_FRM_SIZE_TILE_FRM_RHS_MSB: c_int = 4;
pub const GBE_FRM_SIZE_TILE_FRM_RHS_LSB: c_int = 0;
pub const GBE_FRM_SIZE_TILE_FRM_DEPTH_MSB: c_int = 14;
pub const GBE_FRM_SIZE_TILE_FRM_DEPTH_LSB: c_int = 13;
pub const GBE_FRM_SIZE_TILE_FRM_FIFO_RESET_MSB: c_int = 15;
pub const GBE_FRM_SIZE_TILE_FRM_FIFO_RESET_LSB: c_int = 15;
pub const GBE_FRM_SIZE_PIXEL_FB_HEIGHT_PIX_MSB: c_int = 31;
pub const GBE_FRM_SIZE_PIXEL_FB_HEIGHT_PIX_LSB: c_int = 16;
pub const GBE_DID_CONTROL_DID_DMA_ENABLE_MSB: c_int = 0;
pub const GBE_DID_CONTROL_DID_DMA_ENABLE_LSB: c_int = 0;
pub const GBE_DID_INHWCTRL_DID_DMA_ENABLE_MSB: c_int = 0;
pub const GBE_DID_INHWCTRL_DID_DMA_ENABLE_LSB: c_int = 0;
pub const GBE_DID_START_XY_DID_STARTY_MSB: c_int = 23;
pub const GBE_DID_START_XY_DID_STARTY_LSB: c_int = 12;
pub const GBE_DID_START_XY_DID_STARTX_MSB: c_int = 11;
pub const GBE_DID_START_XY_DID_STARTX_LSB: c_int = 0;
pub const GBE_CRS_START_XY_CRS_STARTY_MSB: c_int = 23;
pub const GBE_CRS_START_XY_CRS_STARTY_LSB: c_int = 12;
pub const GBE_CRS_START_XY_CRS_STARTX_MSB: c_int = 11;
pub const GBE_CRS_START_XY_CRS_STARTX_LSB: c_int = 0;
pub const GBE_WID_AUX_MSB: c_int = 12;
pub const GBE_WID_AUX_LSB: c_int = 11;
pub const GBE_WID_GAMMA_MSB: c_int = 10;
pub const GBE_WID_GAMMA_LSB: c_int = 10;
pub const GBE_WID_CM_MSB: c_int = 9;
pub const GBE_WID_CM_LSB: c_int = 5;
pub const GBE_WID_TYP_MSB: c_int = 4;
pub const GBE_WID_TYP_LSB: c_int = 2;
pub const GBE_WID_BUF_MSB: c_int = 1;
pub const GBE_WID_BUF_LSB: c_int = 0;
pub const GBE_VC_START_XY_VC_STARTY_MSB: c_int = 23;
pub const GBE_VC_START_XY_VC_STARTY_LSB: c_int = 12;
pub const GBE_VC_START_XY_VC_STARTX_MSB: c_int = 11;
pub const GBE_VC_START_XY_VC_STARTX_LSB: c_int = 0;
// Constants
pub const GBE_FRM_DEPTH_8: c_int = 0;
pub const GBE_FRM_DEPTH_16: c_int = 1;
pub const GBE_FRM_DEPTH_32: c_int = 2;
pub const GBE_CMODE_I8: c_int = 0;
pub const GBE_CMODE_I12: c_int = 1;
pub const GBE_CMODE_RG3B2: c_int = 2;
pub const GBE_CMODE_RGB4: c_int = 3;
pub const GBE_CMODE_ARGB5: c_int = 4;
pub const GBE_CMODE_RGB8: c_int = 5;
pub const GBE_CMODE_RGBA5: c_int = 6;
pub const GBE_CMODE_RGB10: c_int = 7;
pub const GBE_BMODE_BOTH: c_int = 3;
pub const GBE_CRS_MAGIC: c_int = 54;
pub const GBE_PIXEN_MAGIC_ON: c_int = 19;
pub const GBE_PIXEN_MAGIC_OFF: c_int = 2;
pub const GBE_TLB_SIZE: c_int = 128;
// [1] - only GBE revision 2 and later
//
// Video Timing Data Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gbe_timing_info {
    pub flags: c_int,
    pub /: *mut *mut short width; / Monitor resolution,
    pub height: c_short,
    pub /: *mut *mut int fields_sec; / fields/sec (Hz -3 dec. places,
    pub /: *mut *mut int cfreq; / pixel clock frequency (MHz -3 dec. places),
    pub /: *mut *mut short htotal; / Horizontal total pixels,
    pub /: *mut *mut short hblank_start; / Horizontal blank start,
    pub /: *mut *mut short hblank_end; / Horizontal blank end,
    pub /: *mut *mut short hsync_start; / Horizontal sync start,
    pub /: *mut *mut short hsync_end; / Horizontal sync end,
    pub /: *mut *mut short vtotal; / Vertical total lines,
    pub /: *mut *mut short vblank_start; / Vertical blank start,
    pub /: *mut *mut short vblank_end; / Vertical blank end,
    pub /: *mut *mut short vsync_start; / Vertical sync start,
    pub /: *mut *mut short vsync_end; / Vertical sync end,
    pub /: *mut *mut short pll_m; / PLL M parameter,
    pub /: *mut *mut short pll_n; / PLL P parameter,
    pub /: *mut *mut short pll_p; / PLL N parameter,
}

// Defines for gbe_vof_info_t flags
pub const GBE_VOF_UNKNOWNMON: c_int = 1;
pub const GBE_VOF_STEREO: c_int = 2;

pub const GBE_VOF_FLATPANEL: c_uint = 0x1000	/* FLATPANEL Timing */;
pub const GBE_VOF_MAGICKEY: c_uint = 0x2000	/* Backdoor key */;
