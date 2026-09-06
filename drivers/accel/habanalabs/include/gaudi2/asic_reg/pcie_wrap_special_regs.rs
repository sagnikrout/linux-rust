//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pcie_wrap_special_regs.h
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
// PCIE_WRAP_SPECIAL
// (Prototype: SPECIAL_REGS)
//
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_0: c_uint = 0x4C01E80;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_1: c_uint = 0x4C01E84;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_2: c_uint = 0x4C01E88;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_3: c_uint = 0x4C01E8C;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_4: c_uint = 0x4C01E90;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_5: c_uint = 0x4C01E94;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_6: c_uint = 0x4C01E98;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_7: c_uint = 0x4C01E9C;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_8: c_uint = 0x4C01EA0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_9: c_uint = 0x4C01EA4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_10: c_uint = 0x4C01EA8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_11: c_uint = 0x4C01EAC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_12: c_uint = 0x4C01EB0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_13: c_uint = 0x4C01EB4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_14: c_uint = 0x4C01EB8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_15: c_uint = 0x4C01EBC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_16: c_uint = 0x4C01EC0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_17: c_uint = 0x4C01EC4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_18: c_uint = 0x4C01EC8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_19: c_uint = 0x4C01ECC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_20: c_uint = 0x4C01ED0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_21: c_uint = 0x4C01ED4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_22: c_uint = 0x4C01ED8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_23: c_uint = 0x4C01EDC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_24: c_uint = 0x4C01EE0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_25: c_uint = 0x4C01EE4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_26: c_uint = 0x4C01EE8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_27: c_uint = 0x4C01EEC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_28: c_uint = 0x4C01EF0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_29: c_uint = 0x4C01EF4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_30: c_uint = 0x4C01EF8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_PRIV_31: c_uint = 0x4C01EFC;
pub const mmPCIE_WRAP_SPECIAL_MEM_GW_DATA: c_uint = 0x4C01F00;
pub const mmPCIE_WRAP_SPECIAL_MEM_GW_REQ: c_uint = 0x4C01F04;
pub const mmPCIE_WRAP_SPECIAL_MEM_NUMOF: c_uint = 0x4C01F0C;
pub const mmPCIE_WRAP_SPECIAL_MEM_ECC_SEL: c_uint = 0x4C01F10;
pub const mmPCIE_WRAP_SPECIAL_MEM_ECC_CTL: c_uint = 0x4C01F14;
pub const mmPCIE_WRAP_SPECIAL_MEM_ECC_ERR_MASK: c_uint = 0x4C01F18;
pub const mmPCIE_WRAP_SPECIAL_MEM_ECC_GLBL_ERR_MASK: c_uint = 0x4C01F1C;
pub const mmPCIE_WRAP_SPECIAL_MEM_ECC_ERR_STS: c_uint = 0x4C01F20;
pub const mmPCIE_WRAP_SPECIAL_MEM_ECC_ERR_ADDR: c_uint = 0x4C01F24;
pub const mmPCIE_WRAP_SPECIAL_MEM_RM: c_uint = 0x4C01F28;
pub const mmPCIE_WRAP_SPECIAL_GLBL_ERR_MASK: c_uint = 0x4C01F40;
pub const mmPCIE_WRAP_SPECIAL_GLBL_ERR_ADDR: c_uint = 0x4C01F44;
pub const mmPCIE_WRAP_SPECIAL_GLBL_ERR_CAUSE: c_uint = 0x4C01F48;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SPARE_0: c_uint = 0x4C01F60;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SPARE_1: c_uint = 0x4C01F64;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SPARE_2: c_uint = 0x4C01F68;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SPARE_3: c_uint = 0x4C01F6C;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_0: c_uint = 0x4C01F80;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_1: c_uint = 0x4C01F84;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_2: c_uint = 0x4C01F88;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_3: c_uint = 0x4C01F8C;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_4: c_uint = 0x4C01F90;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_5: c_uint = 0x4C01F94;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_6: c_uint = 0x4C01F98;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_7: c_uint = 0x4C01F9C;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_8: c_uint = 0x4C01FA0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_9: c_uint = 0x4C01FA4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_10: c_uint = 0x4C01FA8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_11: c_uint = 0x4C01FAC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_12: c_uint = 0x4C01FB0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_13: c_uint = 0x4C01FB4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_14: c_uint = 0x4C01FB8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_15: c_uint = 0x4C01FBC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_16: c_uint = 0x4C01FC0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_17: c_uint = 0x4C01FC4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_18: c_uint = 0x4C01FC8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_19: c_uint = 0x4C01FCC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_20: c_uint = 0x4C01FD0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_21: c_uint = 0x4C01FD4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_22: c_uint = 0x4C01FD8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_23: c_uint = 0x4C01FDC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_24: c_uint = 0x4C01FE0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_25: c_uint = 0x4C01FE4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_26: c_uint = 0x4C01FE8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_27: c_uint = 0x4C01FEC;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_28: c_uint = 0x4C01FF0;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_29: c_uint = 0x4C01FF4;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_30: c_uint = 0x4C01FF8;
pub const mmPCIE_WRAP_SPECIAL_GLBL_SEC_31: c_uint = 0x4C01FFC;
