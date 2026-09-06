//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/sram_y0_x4_rtr_regs.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// SRAM_Y0_X4_RTR (Prototype: IC_RTR)
//
pub const mmSRAM_Y0_X4_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x211100;
pub const mmSRAM_Y0_X4_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x211104;
pub const mmSRAM_Y0_X4_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x211110;
pub const mmSRAM_Y0_X4_RTR_HBW_E_ARB_MAX: c_uint = 0x211120;
pub const mmSRAM_Y0_X4_RTR_HBW_W_ARB_MAX: c_uint = 0x211124;
pub const mmSRAM_Y0_X4_RTR_HBW_L_ARB_MAX: c_uint = 0x211130;
pub const mmSRAM_Y0_X4_RTR_HBW_DATA_E_ARB: c_uint = 0x211140;
pub const mmSRAM_Y0_X4_RTR_HBW_DATA_W_ARB: c_uint = 0x211144;
pub const mmSRAM_Y0_X4_RTR_HBW_DATA_L_ARB: c_uint = 0x211148;
pub const mmSRAM_Y0_X4_RTR_HBW_WR_RS_E_ARB: c_uint = 0x211160;
pub const mmSRAM_Y0_X4_RTR_HBW_WR_RS_W_ARB: c_uint = 0x211164;
pub const mmSRAM_Y0_X4_RTR_HBW_WR_RS_L_ARB: c_uint = 0x211168;
pub const mmSRAM_Y0_X4_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x211200;
pub const mmSRAM_Y0_X4_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x211204;
pub const mmSRAM_Y0_X4_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x211210;
pub const mmSRAM_Y0_X4_RTR_LBW_E_ARB_MAX: c_uint = 0x211220;
pub const mmSRAM_Y0_X4_RTR_LBW_W_ARB_MAX: c_uint = 0x211224;
pub const mmSRAM_Y0_X4_RTR_LBW_L_ARB_MAX: c_uint = 0x211230;
pub const mmSRAM_Y0_X4_RTR_LBW_DATA_E_ARB: c_uint = 0x211240;
pub const mmSRAM_Y0_X4_RTR_LBW_DATA_W_ARB: c_uint = 0x211244;
pub const mmSRAM_Y0_X4_RTR_LBW_DATA_L_ARB: c_uint = 0x211248;
pub const mmSRAM_Y0_X4_RTR_LBW_WR_RS_E_ARB: c_uint = 0x211260;
pub const mmSRAM_Y0_X4_RTR_LBW_WR_RS_W_ARB: c_uint = 0x211264;
pub const mmSRAM_Y0_X4_RTR_LBW_WR_RS_L_ARB: c_uint = 0x211268;
pub const mmSRAM_Y0_X4_RTR_DBG_E_ARB: c_uint = 0x211300;
pub const mmSRAM_Y0_X4_RTR_DBG_W_ARB: c_uint = 0x211304;
pub const mmSRAM_Y0_X4_RTR_DBG_L_ARB: c_uint = 0x211310;
pub const mmSRAM_Y0_X4_RTR_DBG_E_ARB_MAX: c_uint = 0x211320;
pub const mmSRAM_Y0_X4_RTR_DBG_W_ARB_MAX: c_uint = 0x211324;
pub const mmSRAM_Y0_X4_RTR_DBG_L_ARB_MAX: c_uint = 0x211330;
