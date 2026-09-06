//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_cfg_special_regs.h
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
// DCORE0_TPC0_CFG_SPECIAL
// (Prototype: SPECIAL_REGS)
//
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_0: c_uint = 0x400BE80;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_1: c_uint = 0x400BE84;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_2: c_uint = 0x400BE88;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_3: c_uint = 0x400BE8C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_4: c_uint = 0x400BE90;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_5: c_uint = 0x400BE94;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_6: c_uint = 0x400BE98;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_7: c_uint = 0x400BE9C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_8: c_uint = 0x400BEA0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_9: c_uint = 0x400BEA4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_10: c_uint = 0x400BEA8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_11: c_uint = 0x400BEAC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_12: c_uint = 0x400BEB0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_13: c_uint = 0x400BEB4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_14: c_uint = 0x400BEB8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_15: c_uint = 0x400BEBC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_16: c_uint = 0x400BEC0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_17: c_uint = 0x400BEC4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_18: c_uint = 0x400BEC8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_19: c_uint = 0x400BECC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_20: c_uint = 0x400BED0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_21: c_uint = 0x400BED4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_22: c_uint = 0x400BED8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_23: c_uint = 0x400BEDC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_24: c_uint = 0x400BEE0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_25: c_uint = 0x400BEE4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_26: c_uint = 0x400BEE8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_27: c_uint = 0x400BEEC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_28: c_uint = 0x400BEF0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_29: c_uint = 0x400BEF4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_30: c_uint = 0x400BEF8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_PRIV_31: c_uint = 0x400BEFC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_GW_DATA: c_uint = 0x400BF00;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_GW_REQ: c_uint = 0x400BF04;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_NUMOF: c_uint = 0x400BF0C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_ECC_SEL: c_uint = 0x400BF10;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_ECC_CTL: c_uint = 0x400BF14;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_ECC_ERR_MASK: c_uint = 0x400BF18;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_ECC_GLBL_ERR_MASK: c_uint = 0x400BF1C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_ECC_ERR_STS: c_uint = 0x400BF20;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_ECC_ERR_ADDR: c_uint = 0x400BF24;
pub const mmDCORE0_TPC0_CFG_SPECIAL_MEM_RM: c_uint = 0x400BF28;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_ERR_MASK: c_uint = 0x400BF40;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_ERR_ADDR: c_uint = 0x400BF44;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_ERR_CAUSE: c_uint = 0x400BF48;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SPARE_0: c_uint = 0x400BF60;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SPARE_1: c_uint = 0x400BF64;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SPARE_2: c_uint = 0x400BF68;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SPARE_3: c_uint = 0x400BF6C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_0: c_uint = 0x400BF80;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_1: c_uint = 0x400BF84;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_2: c_uint = 0x400BF88;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_3: c_uint = 0x400BF8C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_4: c_uint = 0x400BF90;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_5: c_uint = 0x400BF94;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_6: c_uint = 0x400BF98;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_7: c_uint = 0x400BF9C;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_8: c_uint = 0x400BFA0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_9: c_uint = 0x400BFA4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_10: c_uint = 0x400BFA8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_11: c_uint = 0x400BFAC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_12: c_uint = 0x400BFB0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_13: c_uint = 0x400BFB4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_14: c_uint = 0x400BFB8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_15: c_uint = 0x400BFBC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_16: c_uint = 0x400BFC0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_17: c_uint = 0x400BFC4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_18: c_uint = 0x400BFC8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_19: c_uint = 0x400BFCC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_20: c_uint = 0x400BFD0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_21: c_uint = 0x400BFD4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_22: c_uint = 0x400BFD8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_23: c_uint = 0x400BFDC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_24: c_uint = 0x400BFE0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_25: c_uint = 0x400BFE4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_26: c_uint = 0x400BFE8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_27: c_uint = 0x400BFEC;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_28: c_uint = 0x400BFF0;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_29: c_uint = 0x400BFF4;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_30: c_uint = 0x400BFF8;
pub const mmDCORE0_TPC0_CFG_SPECIAL_GLBL_SEC_31: c_uint = 0x400BFFC;
