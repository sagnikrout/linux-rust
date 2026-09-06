//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/vpe/vpe_regs.h
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
// Copyright (c) 2013 Texas Instruments Inc.
//
// David Griego, <dagriego@biglakesoftware.com>
// Dale Farnsworth, <dale@farnsworth.org>
// Archit Taneja, <archit@ti.com>
//
// VPE register offsets and field selectors
// VPE top level regs
pub const VPE_PID: c_uint = 0x0000;
pub const VPE_PID_MINOR_MASK: c_uint = 0x3f;
pub const VPE_PID_MINOR_SHIFT: c_int = 0;
pub const VPE_PID_CUSTOM_MASK: c_uint = 0x03;
pub const VPE_PID_CUSTOM_SHIFT: c_int = 6;
pub const VPE_PID_MAJOR_MASK: c_uint = 0x07;
pub const VPE_PID_MAJOR_SHIFT: c_int = 8;
pub const VPE_PID_RTL_MASK: c_uint = 0x1f;
pub const VPE_PID_RTL_SHIFT: c_int = 11;
pub const VPE_PID_FUNC_MASK: c_uint = 0xfff;
pub const VPE_PID_FUNC_SHIFT: c_int = 16;
pub const VPE_PID_SCHEME_MASK: c_uint = 0x03;
pub const VPE_PID_SCHEME_SHIFT: c_int = 30;
pub const VPE_SYSCONFIG: c_uint = 0x0010;
pub const VPE_SYSCONFIG_IDLE_MASK: c_uint = 0x03;
pub const VPE_SYSCONFIG_IDLE_SHIFT: c_int = 2;
pub const VPE_SYSCONFIG_STANDBY_MASK: c_uint = 0x03;
pub const VPE_SYSCONFIG_STANDBY_SHIFT: c_int = 4;
pub const VPE_FORCE_IDLE_MODE: c_int = 0;
pub const VPE_NO_IDLE_MODE: c_int = 1;
pub const VPE_SMART_IDLE_MODE: c_int = 2;
pub const VPE_SMART_IDLE_WAKEUP_MODE: c_int = 3;
pub const VPE_FORCE_STANDBY_MODE: c_int = 0;
pub const VPE_NO_STANDBY_MODE: c_int = 1;
pub const VPE_SMART_STANDBY_MODE: c_int = 2;
pub const VPE_SMART_STANDBY_WAKEUP_MODE: c_int = 3;
pub const VPE_INT0_STATUS0_RAW_SET: c_uint = 0x0020;

pub const VPE_INT0_STATUS0_CLR: c_uint = 0x0028;

pub const VPE_INT0_ENABLE0_SET: c_uint = 0x0030;

pub const VPE_INT0_ENABLE0_CLR: c_uint = 0x0038;

pub const VPE_INT0_STATUS1_RAW_SET: c_uint = 0x0024;

pub const VPE_INT0_STATUS1_CLR: c_uint = 0x002c;

pub const VPE_INT0_ENABLE1_SET: c_uint = 0x0034;

pub const VPE_INT0_ENABLE1_CLR: c_uint = 0x003c;

pub const VPE_INTC_EOI: c_uint = 0x00a0;
pub const VPE_CLK_ENABLE: c_uint = 0x0100;

pub const VPE_CLK_RESET: c_uint = 0x0104;
pub const VPE_VPDMA_CLK_RESET_MASK: c_uint = 0x1;
pub const VPE_VPDMA_CLK_RESET_SHIFT: c_int = 0;
pub const VPE_DATA_PATH_CLK_RESET_MASK: c_uint = 0x1;
pub const VPE_DATA_PATH_CLK_RESET_SHIFT: c_int = 1;
pub const VPE_MAIN_RESET_MASK: c_uint = 0x1;
pub const VPE_MAIN_RESET_SHIFT: c_int = 31;
pub const VPE_CLK_FORMAT_SELECT: c_uint = 0x010c;
pub const VPE_CSC_SRC_SELECT_MASK: c_uint = 0x03;
pub const VPE_CSC_SRC_SELECT_SHIFT: c_int = 0;

pub const VPE_DS_SRC_SELECT_MASK: c_uint = 0x07;
pub const VPE_DS_SRC_SELECT_SHIFT: c_int = 9;

pub const VPE_CLK_RANGE_MAP: c_uint = 0x011c;
pub const VPE_RANGE_RANGE_MAP_Y_MASK: c_uint = 0x07;
pub const VPE_RANGE_RANGE_MAP_Y_SHIFT: c_int = 0;
pub const VPE_RANGE_RANGE_MAP_UV_MASK: c_uint = 0x07;
pub const VPE_RANGE_RANGE_MAP_UV_SHIFT: c_int = 3;

// VPE chrominance upsampler regs
pub const VPE_US1_R0: c_uint = 0x0304;
pub const VPE_US2_R0: c_uint = 0x0404;
pub const VPE_US3_R0: c_uint = 0x0504;
pub const VPE_US_C1_MASK: c_uint = 0x3fff;
pub const VPE_US_C1_SHIFT: c_int = 2;
pub const VPE_US_C0_MASK: c_uint = 0x3fff;
pub const VPE_US_C0_SHIFT: c_int = 18;
pub const VPE_US_MODE_MASK: c_uint = 0x03;
pub const VPE_US_MODE_SHIFT: c_int = 16;
pub const VPE_ANCHOR_FID0_C1_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID0_C1_SHIFT: c_int = 2;
pub const VPE_ANCHOR_FID0_C0_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID0_C0_SHIFT: c_int = 18;
pub const VPE_US1_R1: c_uint = 0x0308;
pub const VPE_US2_R1: c_uint = 0x0408;
pub const VPE_US3_R1: c_uint = 0x0508;
pub const VPE_ANCHOR_FID0_C3_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID0_C3_SHIFT: c_int = 2;
pub const VPE_ANCHOR_FID0_C2_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID0_C2_SHIFT: c_int = 18;
pub const VPE_US1_R2: c_uint = 0x030c;
pub const VPE_US2_R2: c_uint = 0x040c;
pub const VPE_US3_R2: c_uint = 0x050c;
pub const VPE_INTERP_FID0_C1_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID0_C1_SHIFT: c_int = 2;
pub const VPE_INTERP_FID0_C0_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID0_C0_SHIFT: c_int = 18;
pub const VPE_US1_R3: c_uint = 0x0310;
pub const VPE_US2_R3: c_uint = 0x0410;
pub const VPE_US3_R3: c_uint = 0x0510;
pub const VPE_INTERP_FID0_C3_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID0_C3_SHIFT: c_int = 2;
pub const VPE_INTERP_FID0_C2_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID0_C2_SHIFT: c_int = 18;
pub const VPE_US1_R4: c_uint = 0x0314;
pub const VPE_US2_R4: c_uint = 0x0414;
pub const VPE_US3_R4: c_uint = 0x0514;
pub const VPE_ANCHOR_FID1_C1_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID1_C1_SHIFT: c_int = 2;
pub const VPE_ANCHOR_FID1_C0_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID1_C0_SHIFT: c_int = 18;
pub const VPE_US1_R5: c_uint = 0x0318;
pub const VPE_US2_R5: c_uint = 0x0418;
pub const VPE_US3_R5: c_uint = 0x0518;
pub const VPE_ANCHOR_FID1_C3_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID1_C3_SHIFT: c_int = 2;
pub const VPE_ANCHOR_FID1_C2_MASK: c_uint = 0x3fff;
pub const VPE_ANCHOR_FID1_C2_SHIFT: c_int = 18;
pub const VPE_US1_R6: c_uint = 0x031c;
pub const VPE_US2_R6: c_uint = 0x041c;
pub const VPE_US3_R6: c_uint = 0x051c;
pub const VPE_INTERP_FID1_C1_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID1_C1_SHIFT: c_int = 2;
pub const VPE_INTERP_FID1_C0_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID1_C0_SHIFT: c_int = 18;
pub const VPE_US1_R7: c_uint = 0x0320;
pub const VPE_US2_R7: c_uint = 0x0420;
pub const VPE_US3_R7: c_uint = 0x0520;
pub const VPE_INTERP_FID0_C3_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID0_C3_SHIFT: c_int = 2;
pub const VPE_INTERP_FID0_C2_MASK: c_uint = 0x3fff;
pub const VPE_INTERP_FID0_C2_SHIFT: c_int = 18;
// VPE de-interlacer regs
pub const VPE_DEI_FRAME_SIZE: c_uint = 0x0600;
pub const VPE_DEI_WIDTH_MASK: c_uint = 0x07ff;
pub const VPE_DEI_WIDTH_SHIFT: c_int = 0;
pub const VPE_DEI_HEIGHT_MASK: c_uint = 0x07ff;
pub const VPE_DEI_HEIGHT_SHIFT: c_int = 16;

pub const VPE_MDT_BYPASS: c_uint = 0x0604;

pub const VPE_MDT_SF_THRESHOLD: c_uint = 0x0608;
pub const VPE_MDT_SF_SC_THR1_MASK: c_uint = 0xff;
pub const VPE_MDT_SF_SC_THR1_SHIFT: c_int = 0;
pub const VPE_MDT_SF_SC_THR2_MASK: c_uint = 0xff;
pub const VPE_MDT_SF_SC_THR2_SHIFT: c_int = 0;
pub const VPE_MDT_SF_SC_THR3_MASK: c_uint = 0xff;
pub const VPE_MDT_SF_SC_THR3_SHIFT: c_int = 0;
pub const VPE_EDI_CONFIG: c_uint = 0x060c;
pub const VPE_EDI_INP_MODE_MASK: c_uint = 0x03;
pub const VPE_EDI_INP_MODE_SHIFT: c_int = 0;

pub const VPE_EDI_CHROMA3D_COR_THR_MASK: c_uint = 0xff;
pub const VPE_EDI_CHROMA3D_COR_THR_SHIFT: c_int = 8;
pub const VPE_EDI_DIR_COR_LOWER_THR_MASK: c_uint = 0xff;
pub const VPE_EDI_DIR_COR_LOWER_THR_SHIFT: c_int = 16;
pub const VPE_EDI_COR_SCALE_FACTOR_MASK: c_uint = 0xff;
pub const VPE_EDI_COR_SCALE_FACTOR_SHIFT: c_int = 23;
pub const VPE_DEI_EDI_LUT_R0: c_uint = 0x0610;
pub const VPE_EDI_LUT0_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT0_SHIFT: c_int = 0;
pub const VPE_EDI_LUT1_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT1_SHIFT: c_int = 8;
pub const VPE_EDI_LUT2_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT2_SHIFT: c_int = 16;
pub const VPE_EDI_LUT3_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT3_SHIFT: c_int = 24;
pub const VPE_DEI_EDI_LUT_R1: c_uint = 0x0614;
pub const VPE_EDI_LUT0_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT0_SHIFT: c_int = 0;
pub const VPE_EDI_LUT1_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT1_SHIFT: c_int = 8;
pub const VPE_EDI_LUT2_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT2_SHIFT: c_int = 16;
pub const VPE_EDI_LUT3_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT3_SHIFT: c_int = 24;
pub const VPE_DEI_EDI_LUT_R2: c_uint = 0x0618;
pub const VPE_EDI_LUT4_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT4_SHIFT: c_int = 0;
pub const VPE_EDI_LUT5_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT5_SHIFT: c_int = 8;
pub const VPE_EDI_LUT6_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT6_SHIFT: c_int = 16;
pub const VPE_EDI_LUT7_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT7_SHIFT: c_int = 24;
pub const VPE_DEI_EDI_LUT_R3: c_uint = 0x061c;
pub const VPE_EDI_LUT8_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT8_SHIFT: c_int = 0;
pub const VPE_EDI_LUT9_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT9_SHIFT: c_int = 8;
pub const VPE_EDI_LUT10_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT10_SHIFT: c_int = 16;
pub const VPE_EDI_LUT11_MASK: c_uint = 0x1f;
pub const VPE_EDI_LUT11_SHIFT: c_int = 24;
pub const VPE_DEI_FMD_WINDOW_R0: c_uint = 0x0620;
pub const VPE_FMD_WINDOW_MINX_MASK: c_uint = 0x07ff;
pub const VPE_FMD_WINDOW_MINX_SHIFT: c_int = 0;
pub const VPE_FMD_WINDOW_MAXX_MASK: c_uint = 0x07ff;
pub const VPE_FMD_WINDOW_MAXX_SHIFT: c_int = 16;

pub const VPE_DEI_FMD_WINDOW_R1: c_uint = 0x0624;
pub const VPE_FMD_WINDOW_MINY_MASK: c_uint = 0x07ff;
pub const VPE_FMD_WINDOW_MINY_SHIFT: c_int = 0;
pub const VPE_FMD_WINDOW_MAXY_MASK: c_uint = 0x07ff;
pub const VPE_FMD_WINDOW_MAXY_SHIFT: c_int = 16;
pub const VPE_DEI_FMD_CONTROL_R0: c_uint = 0x0628;

pub const VPE_FMD_CAF_FIELD_THR_MASK: c_uint = 0xff;
pub const VPE_FMD_CAF_FIELD_THR_SHIFT: c_int = 16;
pub const VPE_FMD_CAF_LINE_THR_MASK: c_uint = 0xff;
pub const VPE_FMD_CAF_LINE_THR_SHIFT: c_int = 24;
pub const VPE_DEI_FMD_CONTROL_R1: c_uint = 0x062c;
pub const VPE_FMD_CAF_THR_MASK: c_uint = 0x000fffff;
pub const VPE_FMD_CAF_THR_SHIFT: c_int = 0;
pub const VPE_DEI_FMD_STATUS_R0: c_uint = 0x0630;
pub const VPE_FMD_CAF_MASK: c_uint = 0x000fffff;
pub const VPE_FMD_CAF_SHIFT: c_int = 0;

pub const VPE_DEI_FMD_STATUS_R1: c_uint = 0x0634;
pub const VPE_FMD_FIELD_DIFF_MASK: c_uint = 0x0fffffff;
pub const VPE_FMD_FIELD_DIFF_SHIFT: c_int = 0;
pub const VPE_DEI_FMD_STATUS_R2: c_uint = 0x0638;
pub const VPE_FMD_FRAME_DIFF_MASK: c_uint = 0x000fffff;
pub const VPE_FMD_FRAME_DIFF_SHIFT: c_int = 0;
