//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_vdec0_ctrl_special_regs.h
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
// DCORE0_VDEC0_CTRL_SPECIAL
// (Prototype: SPECIAL_REGS)
//
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_0: c_uint = 0x41E4E80;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_1: c_uint = 0x41E4E84;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_2: c_uint = 0x41E4E88;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_3: c_uint = 0x41E4E8C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_4: c_uint = 0x41E4E90;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_5: c_uint = 0x41E4E94;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_6: c_uint = 0x41E4E98;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_7: c_uint = 0x41E4E9C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_8: c_uint = 0x41E4EA0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_9: c_uint = 0x41E4EA4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_10: c_uint = 0x41E4EA8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_11: c_uint = 0x41E4EAC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_12: c_uint = 0x41E4EB0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_13: c_uint = 0x41E4EB4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_14: c_uint = 0x41E4EB8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_15: c_uint = 0x41E4EBC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_16: c_uint = 0x41E4EC0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_17: c_uint = 0x41E4EC4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_18: c_uint = 0x41E4EC8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_19: c_uint = 0x41E4ECC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_20: c_uint = 0x41E4ED0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_21: c_uint = 0x41E4ED4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_22: c_uint = 0x41E4ED8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_23: c_uint = 0x41E4EDC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_24: c_uint = 0x41E4EE0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_25: c_uint = 0x41E4EE4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_26: c_uint = 0x41E4EE8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_27: c_uint = 0x41E4EEC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_28: c_uint = 0x41E4EF0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_29: c_uint = 0x41E4EF4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_30: c_uint = 0x41E4EF8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_PRIV_31: c_uint = 0x41E4EFC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_GW_DATA: c_uint = 0x41E4F00;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_GW_REQ: c_uint = 0x41E4F04;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_NUMOF: c_uint = 0x41E4F0C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_ECC_SEL: c_uint = 0x41E4F10;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_ECC_CTL: c_uint = 0x41E4F14;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_ECC_ERR_MASK: c_uint = 0x41E4F18;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_ECC_GLBL_ERR_MASK: c_uint = 0x41E4F1C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_ECC_ERR_STS: c_uint = 0x41E4F20;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_ECC_ERR_ADDR: c_uint = 0x41E4F24;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_MEM_RM: c_uint = 0x41E4F28;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_ERR_MASK: c_uint = 0x41E4F40;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_ERR_ADDR: c_uint = 0x41E4F44;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_ERR_CAUSE: c_uint = 0x41E4F48;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SPARE_0: c_uint = 0x41E4F60;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SPARE_1: c_uint = 0x41E4F64;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SPARE_2: c_uint = 0x41E4F68;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SPARE_3: c_uint = 0x41E4F6C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_0: c_uint = 0x41E4F80;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_1: c_uint = 0x41E4F84;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_2: c_uint = 0x41E4F88;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_3: c_uint = 0x41E4F8C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_4: c_uint = 0x41E4F90;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_5: c_uint = 0x41E4F94;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_6: c_uint = 0x41E4F98;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_7: c_uint = 0x41E4F9C;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_8: c_uint = 0x41E4FA0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_9: c_uint = 0x41E4FA4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_10: c_uint = 0x41E4FA8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_11: c_uint = 0x41E4FAC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_12: c_uint = 0x41E4FB0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_13: c_uint = 0x41E4FB4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_14: c_uint = 0x41E4FB8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_15: c_uint = 0x41E4FBC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_16: c_uint = 0x41E4FC0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_17: c_uint = 0x41E4FC4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_18: c_uint = 0x41E4FC8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_19: c_uint = 0x41E4FCC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_20: c_uint = 0x41E4FD0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_21: c_uint = 0x41E4FD4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_22: c_uint = 0x41E4FD8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_23: c_uint = 0x41E4FDC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_24: c_uint = 0x41E4FE0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_25: c_uint = 0x41E4FE4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_26: c_uint = 0x41E4FE8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_27: c_uint = 0x41E4FEC;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_28: c_uint = 0x41E4FF0;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_29: c_uint = 0x41E4FF4;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_30: c_uint = 0x41E4FF8;
pub const mmDCORE0_VDEC0_CTRL_SPECIAL_GLBL_SEC_31: c_uint = 0x41E4FFC;
