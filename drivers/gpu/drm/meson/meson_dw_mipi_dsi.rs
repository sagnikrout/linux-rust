//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/meson/meson_dw_mipi_dsi.h
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
// Copyright (C) 2020 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
// Copyright (C) 2018 Amlogic, Inc. All rights reserved.
//
// Top-level registers
// [31: 4]    Reserved.     Default 0.
// [3] RW timing_rst_n: Default 1.
// 1=Assert SW reset of timing feature.   0=Release reset.
// [2] RW dpi_rst_n: Default 1.
// 1=Assert SW reset on mipi_dsi_host_dpi block.   0=Release reset.
// [1] RW intr_rst_n: Default 1.
// 1=Assert SW reset on mipi_dsi_host_intr block.  0=Release reset.
// [0] RW dwc_rst_n:  Default 1.
// 1=Assert SW reset on IP core.   0=Release reset.
//
pub const MIPI_DSI_TOP_SW_RESET: c_uint = 0x3c0;

// [31: 5] Reserved.   Default 0.
// [4] RW manual_edpihalt: Default 0.
// 1=Manual suspend VencL; 0=do not suspend VencL.
// [3] RW auto_edpihalt_en: Default 0.
// 1=Enable IP's edpihalt signal to suspend VencL;
// 0=IP's edpihalt signal does not affect VencL.
// [2] RW clock_freerun: Apply to auto-clock gate only. Default 0.
// 0=Default, use auto-clock gating to save power;
// 1=use free-run clock, disable auto-clock gating, for debug mode.
// [1] RW enable_pixclk: A manual clock gate option, due to DWC IP does not
// have auto-clock gating. 1=Enable pixclk.      Default 0.
// [0] RW enable_sysclk: A manual clock gate option, due to DWC IP does not
// have auto-clock gating. 1=Enable sysclk.      Default 0.
//
pub const MIPI_DSI_TOP_CLK_CNTL: c_uint = 0x3c4;

// [31:24]    Reserved. Default 0.
// [23:20] RW dpi_color_mode: Define DPI pixel format. Default 0.
// 0=16-bit RGB565 config 1;
// 1=16-bit RGB565 config 2;
// 2=16-bit RGB565 config 3;
// 3=18-bit RGB666 config 1;
// 4=18-bit RGB666 config 2;
// 5=24-bit RGB888;
// 6=20-bit YCbCr 4:2:2;
// 7=24-bit YCbCr 4:2:2;
// 8=16-bit YCbCr 4:2:2;
// 9=30-bit RGB;
// 10=36-bit RGB;
// 11=12-bit YCbCr 4:2:0.
// [19] Reserved. Default 0.
// [18:16] RW in_color_mode:  Define VENC data width. Default 0.
// 0=30-bit pixel;
// 1=24-bit pixel;
// 2=18-bit pixel, RGB666;
// 3=16-bit pixel, RGB565.
// [15:14] RW chroma_subsample: Define method of chroma subsampling. Default 0.
// Applicable to YUV422 or YUV420 only.
// 0=Use even pixel's chroma;
// 1=Use odd pixel's chroma;
// 2=Use averaged value between even and odd pair.
// [13:12] RW comp2_sel:  Select which component to be Cr or B: Default 2.
// 0=comp0; 1=comp1; 2=comp2.
// [11:10] RW comp1_sel:  Select which component to be Cb or G: Default 1.
// 0=comp0; 1=comp1; 2=comp2.
// [9: 8] RW comp0_sel:  Select which component to be Y  or R: Default 0.
// 0=comp0; 1=comp1; 2=comp2.
// [7]    Reserved. Default 0.
// [6] RW de_pol:  Default 0.
// If DE input is active low, set to 1 to invert to active high.
// [5] RW hsync_pol: Default 0.
// If HS input is active low, set to 1 to invert to active high.
// [4] RW vsync_pol: Default 0.
// If VS input is active low, set to 1 to invert to active high.
// [3] RW dpicolorm: Signal to IP.   Default 0.
// [2] RW dpishutdn: Signal to IP.   Default 0.
// [1]    Reserved.  Default 0.
// [0]    Reserved.  Default 0.
//
pub const MIPI_DSI_TOP_CNTL: c_uint = 0x3c8;
// VENC data width
pub const VENC_IN_COLOR_30B: c_uint = 0x0;
pub const VENC_IN_COLOR_24B: c_uint = 0x1;
pub const VENC_IN_COLOR_18B: c_uint = 0x2;
pub const VENC_IN_COLOR_16B: c_uint = 0x3;
// DPI pixel format
pub const DPI_COLOR_16BIT_CFG_1: c_int = 0;
pub const DPI_COLOR_16BIT_CFG_2: c_int = 1;
pub const DPI_COLOR_16BIT_CFG_3: c_int = 2;
pub const DPI_COLOR_18BIT_CFG_1: c_int = 3;
pub const DPI_COLOR_18BIT_CFG_2: c_int = 4;
pub const DPI_COLOR_24BIT: c_int = 5;
pub const DPI_COLOR_20BIT_YCBCR_422: c_int = 6;
pub const DPI_COLOR_24BIT_YCBCR_422: c_int = 7;
pub const DPI_COLOR_16BIT_YCBCR_422: c_int = 8;
pub const DPI_COLOR_30BIT: c_int = 9;
pub const DPI_COLOR_36BIT: c_int = 10;
pub const DPI_COLOR_12BIT_YCBCR_420: c_int = 11;

pub const MIPI_DSI_TOP_SUSPEND_CNTL: c_uint = 0x3cc;
pub const MIPI_DSI_TOP_SUSPEND_LINE: c_uint = 0x3d0;
pub const MIPI_DSI_TOP_SUSPEND_PIX: c_uint = 0x3d4;
pub const MIPI_DSI_TOP_MEAS_CNTL: c_uint = 0x3d8;
// [0] R  stat_edpihalt:  edpihalt signal from IP.    Default 0.
pub const MIPI_DSI_TOP_STAT: c_uint = 0x3dc;
pub const MIPI_DSI_TOP_MEAS_STAT_TE0: c_uint = 0x3e0;
pub const MIPI_DSI_TOP_MEAS_STAT_TE1: c_uint = 0x3e4;
pub const MIPI_DSI_TOP_MEAS_STAT_VS0: c_uint = 0x3e8;
pub const MIPI_DSI_TOP_MEAS_STAT_VS1: c_uint = 0x3ec;
// [31:16] RW intr_stat/clr. Default 0.
// For each bit, read as this interrupt level status,
// write 1 to clear.
// [31:22] Reserved
// [   21] stat/clr of eof interrupt
// [   21] vde_fall interrupt
// [   19] stat/clr of de_rise interrupt
// [   18] stat/clr of vs_fall interrupt
// [   17] stat/clr of vs_rise interrupt
// [   16] stat/clr of dwc_edpite interrupt
// [15: 0] RW intr_enable. Default 0.
// For each bit, 1=enable this interrupt, 0=disable.
// [15: 6] Reserved
// [    5] eof interrupt
// [    4] de_fall interrupt
// [    3] de_rise interrupt
// [    2] vs_fall interrupt
// [    1] vs_rise interrupt
// [    0] dwc_edpite interrupt
//
pub const MIPI_DSI_TOP_INTR_CNTL_STAT: c_uint = 0x3f0;
// 31: 2    Reserved.   Default 0.
// 1: 0 RW mem_pd.     Default 3.
pub const MIPI_DSI_TOP_MEM_PD: c_uint = 0x3f4;
