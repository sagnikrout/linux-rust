//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/sram_y0_x0_rtr_regs.h
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
// SRAM_Y0_X0_RTR (Prototype: IC_RTR)
//
pub const mmSRAM_Y0_X0_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x201100;
pub const mmSRAM_Y0_X0_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x201104;
pub const mmSRAM_Y0_X0_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x201110;
pub const mmSRAM_Y0_X0_RTR_HBW_E_ARB_MAX: c_uint = 0x201120;
pub const mmSRAM_Y0_X0_RTR_HBW_W_ARB_MAX: c_uint = 0x201124;
pub const mmSRAM_Y0_X0_RTR_HBW_L_ARB_MAX: c_uint = 0x201130;
pub const mmSRAM_Y0_X0_RTR_HBW_DATA_E_ARB: c_uint = 0x201140;
pub const mmSRAM_Y0_X0_RTR_HBW_DATA_W_ARB: c_uint = 0x201144;
pub const mmSRAM_Y0_X0_RTR_HBW_DATA_L_ARB: c_uint = 0x201148;
pub const mmSRAM_Y0_X0_RTR_HBW_WR_RS_E_ARB: c_uint = 0x201160;
pub const mmSRAM_Y0_X0_RTR_HBW_WR_RS_W_ARB: c_uint = 0x201164;
pub const mmSRAM_Y0_X0_RTR_HBW_WR_RS_L_ARB: c_uint = 0x201168;
pub const mmSRAM_Y0_X0_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x201200;
pub const mmSRAM_Y0_X0_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x201204;
pub const mmSRAM_Y0_X0_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x201210;
pub const mmSRAM_Y0_X0_RTR_LBW_E_ARB_MAX: c_uint = 0x201220;
pub const mmSRAM_Y0_X0_RTR_LBW_W_ARB_MAX: c_uint = 0x201224;
pub const mmSRAM_Y0_X0_RTR_LBW_L_ARB_MAX: c_uint = 0x201230;
pub const mmSRAM_Y0_X0_RTR_LBW_DATA_E_ARB: c_uint = 0x201240;
pub const mmSRAM_Y0_X0_RTR_LBW_DATA_W_ARB: c_uint = 0x201244;
pub const mmSRAM_Y0_X0_RTR_LBW_DATA_L_ARB: c_uint = 0x201248;
pub const mmSRAM_Y0_X0_RTR_LBW_WR_RS_E_ARB: c_uint = 0x201260;
pub const mmSRAM_Y0_X0_RTR_LBW_WR_RS_W_ARB: c_uint = 0x201264;
pub const mmSRAM_Y0_X0_RTR_LBW_WR_RS_L_ARB: c_uint = 0x201268;
pub const mmSRAM_Y0_X0_RTR_DBG_E_ARB: c_uint = 0x201300;
pub const mmSRAM_Y0_X0_RTR_DBG_W_ARB: c_uint = 0x201304;
pub const mmSRAM_Y0_X0_RTR_DBG_L_ARB: c_uint = 0x201310;
pub const mmSRAM_Y0_X0_RTR_DBG_E_ARB_MAX: c_uint = 0x201320;
pub const mmSRAM_Y0_X0_RTR_DBG_W_ARB_MAX: c_uint = 0x201324;
pub const mmSRAM_Y0_X0_RTR_DBG_L_ARB_MAX: c_uint = 0x201330;
