//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/rot0_desc_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// ROT0_DESC
// (Prototype: ROT_DESC)
//
pub const mmROT0_DESC_CONTEXT_ID: c_uint = 0x4E0B100;
pub const mmROT0_DESC_IN_IMG_START_ADDR_L: c_uint = 0x4E0B104;
pub const mmROT0_DESC_IN_IMG_START_ADDR_H: c_uint = 0x4E0B108;
pub const mmROT0_DESC_OUT_IMG_START_ADDR_L: c_uint = 0x4E0B10C;
pub const mmROT0_DESC_OUT_IMG_START_ADDR_H: c_uint = 0x4E0B110;
pub const mmROT0_DESC_CFG: c_uint = 0x4E0B114;
pub const mmROT0_DESC_IM_READ_SLOPE: c_uint = 0x4E0B118;
pub const mmROT0_DESC_SIN_D: c_uint = 0x4E0B11C;
pub const mmROT0_DESC_COS_D: c_uint = 0x4E0B120;
pub const mmROT0_DESC_IN_IMG: c_uint = 0x4E0B124;
pub const mmROT0_DESC_IN_STRIDE: c_uint = 0x4E0B128;
pub const mmROT0_DESC_IN_STRIPE: c_uint = 0x4E0B12C;
pub const mmROT0_DESC_IN_CENTER: c_uint = 0x4E0B130;
pub const mmROT0_DESC_OUT_IMG: c_uint = 0x4E0B134;
pub const mmROT0_DESC_OUT_STRIDE: c_uint = 0x4E0B138;
pub const mmROT0_DESC_OUT_STRIPE: c_uint = 0x4E0B13C;
pub const mmROT0_DESC_OUT_CENTER: c_uint = 0x4E0B140;
pub const mmROT0_DESC_BACKGROUND: c_uint = 0x4E0B144;
pub const mmROT0_DESC_CPL_MSG_EN: c_uint = 0x4E0B148;
pub const mmROT0_DESC_IDLE_STATE: c_uint = 0x4E0B14C;
pub const mmROT0_DESC_CPL_MSG_ADDR: c_uint = 0x4E0B150;
pub const mmROT0_DESC_CPL_MSG_DATA: c_uint = 0x4E0B154;
pub const mmROT0_DESC_CPL_MSG_AWUSER: c_uint = 0x4E0B158;
pub const mmROT0_DESC_X_I_START_OFFSET: c_uint = 0x4E0B15C;
pub const mmROT0_DESC_X_I_START_OFFSET_FLIP: c_uint = 0x4E0B160;
pub const mmROT0_DESC_X_I_FIRST: c_uint = 0x4E0B164;
pub const mmROT0_DESC_Y_I_FIRST: c_uint = 0x4E0B168;
pub const mmROT0_DESC_Y_I: c_uint = 0x4E0B16C;
pub const mmROT0_DESC_OUT_STRIPE_SIZE: c_uint = 0x4E0B170;
pub const mmROT0_DESC_RSB_CFG_0: c_uint = 0x4E0B174;
pub const mmROT0_DESC_RSB_PAD_VAL: c_uint = 0x4E0B178;
pub const mmROT0_DESC_HBW_ARUSER_HI: c_uint = 0x4E0B17C;
pub const mmROT0_DESC_HBW_ARUSER_LO: c_uint = 0x4E0B180;
pub const mmROT0_DESC_HBW_AWUSER_HI: c_uint = 0x4E0B184;
pub const mmROT0_DESC_HBW_AWUSER_LO: c_uint = 0x4E0B188;
pub const mmROT0_DESC_OWM_CFG: c_uint = 0x4E0B18C;
pub const mmROT0_DESC_CTRL_CFG: c_uint = 0x4E0B190;
pub const mmROT0_DESC_PIXEL_PAD: c_uint = 0x4E0B194;
pub const mmROT0_DESC_PREC_SHIFT: c_uint = 0x4E0B198;
pub const mmROT0_DESC_MAX_VAL: c_uint = 0x4E0B19C;
pub const mmROT0_DESC_A0_M11: c_uint = 0x4E0B1A0;
pub const mmROT0_DESC_A1_M12: c_uint = 0x4E0B1A4;
pub const mmROT0_DESC_A2: c_uint = 0x4E0B1A8;
pub const mmROT0_DESC_B0_M21: c_uint = 0x4E0B1AC;
pub const mmROT0_DESC_B1_M22: c_uint = 0x4E0B1B0;
pub const mmROT0_DESC_B2: c_uint = 0x4E0B1B4;
pub const mmROT0_DESC_C0: c_uint = 0x4E0B1B8;
pub const mmROT0_DESC_C1: c_uint = 0x4E0B1BC;
pub const mmROT0_DESC_C2: c_uint = 0x4E0B1C0;
pub const mmROT0_DESC_D0: c_uint = 0x4E0B1C4;
pub const mmROT0_DESC_D1: c_uint = 0x4E0B1C8;
pub const mmROT0_DESC_D2: c_uint = 0x4E0B1CC;
pub const mmROT0_DESC_INV_PROC_SIZE_M_1: c_uint = 0x4E0B1D0;
pub const mmROT0_DESC_MESH_IMG_START_ADDR_L: c_uint = 0x4E0B1D4;
pub const mmROT0_DESC_MESH_IMG_START_ADDR_H: c_uint = 0x4E0B1D8;
pub const mmROT0_DESC_MESH_IMG: c_uint = 0x4E0B1DC;
pub const mmROT0_DESC_MESH_STRIDE: c_uint = 0x4E0B1E0;
pub const mmROT0_DESC_MESH_STRIPE: c_uint = 0x4E0B1E4;
pub const mmROT0_DESC_MESH_CTRL: c_uint = 0x4E0B1E8;
pub const mmROT0_DESC_MESH_GH: c_uint = 0x4E0B1EC;
pub const mmROT0_DESC_MESH_GV: c_uint = 0x4E0B1F0;
pub const mmROT0_DESC_MRSB_CFG_0: c_uint = 0x4E0B1F4;
pub const mmROT0_DESC_MRSB_PAD_VAL: c_uint = 0x4E0B1F8;
pub const mmROT0_DESC_BUF_CFG: c_uint = 0x4E0B1FC;
pub const mmROT0_DESC_CID_OFFSET: c_uint = 0x4E0B200;
pub const mmROT0_DESC_PUSH_DESC: c_uint = 0x4E0B204;
