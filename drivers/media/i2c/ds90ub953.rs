//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ds90ub953.h
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

pub const UB953_REG_RESET_CTL: c_uint = 0x01;

pub const UB953_REG_GENERAL_CFG: c_uint = 0x02;

pub const UB953_REG_GENERAL_CFG_CSI_LANE_SEL_SHIFT: c_int = 4;

pub const UB953_REG_MODE_SEL: c_uint = 0x03;

pub const UB953_REG_CLKOUT_CTRL0: c_uint = 0x06;
pub const UB953_REG_CLKOUT_CTRL1: c_uint = 0x07;
pub const UB953_REG_I2C_CONTROL2: c_uint = 0x0a;
pub const UB953_REG_I2C_CONTROL2_SDA_OUTPUT_SETUP_SHIFT: c_int = 4;

pub const UB953_REG_SCL_HIGH_TIME: c_uint = 0x0b;
pub const UB953_REG_SCL_LOW_TIME: c_uint = 0x0c;
pub const UB953_REG_LOCAL_GPIO_DATA: c_uint = 0x0d;

pub const UB953_REG_GPIO_INPUT_CTRL: c_uint = 0x0e;

pub const UB953_REG_BC_CTRL: c_uint = 0x49;

pub const UB953_REG_REV_MASK_ID: c_uint = 0x50;
pub const UB953_REG_GENERAL_STATUS: c_uint = 0x52;
pub const UB953_REG_GPIO_PIN_STS: c_uint = 0x53;

pub const UB953_REG_BIST_ERR_CNT: c_uint = 0x54;
pub const UB953_REG_CRC_ERR_CNT1: c_uint = 0x55;
pub const UB953_REG_CRC_ERR_CNT2: c_uint = 0x56;
pub const UB953_REG_CSI_ERR_CNT: c_uint = 0x5c;
pub const UB953_REG_CSI_ERR_STATUS: c_uint = 0x5d;
pub const UB953_REG_CSI_ERR_DLANE01: c_uint = 0x5e;
pub const UB953_REG_CSI_ERR_DLANE23: c_uint = 0x5f;
pub const UB953_REG_CSI_ERR_CLK_LANE: c_uint = 0x60;
pub const UB953_REG_CSI_PKT_HDR_VC_ID: c_uint = 0x61;
pub const UB953_REG_PKT_HDR_WC_LSB: c_uint = 0x62;
pub const UB953_REG_PKT_HDR_WC_MSB: c_uint = 0x63;
pub const UB953_REG_CSI_ECC: c_uint = 0x64;
pub const UB953_REG_IND_ACC_CTL: c_uint = 0xb0;
pub const UB953_REG_IND_ACC_ADDR: c_uint = 0xb1;
pub const UB953_REG_IND_ACC_DATA: c_uint = 0xb2;

pub const UB953_REG_FPD3_RX_ID_LEN: c_int = 6;
// Indirect register blocks
pub const UB953_IND_TARGET_PAT_GEN: c_uint = 0x00;
pub const UB953_IND_TARGET_ANALOG: c_uint = 0x01;
pub const UB953_IND_TARGET_DIE_ID: c_uint = 0x02;
pub const UB953_IND_PGEN_CTL: c_uint = 0x01;

pub const UB953_IND_PGEN_CFG: c_uint = 0x02;
pub const UB953_IND_PGEN_CSI_DI: c_uint = 0x03;
pub const UB953_IND_PGEN_LINE_SIZE1: c_uint = 0x04;
pub const UB953_IND_PGEN_LINE_SIZE0: c_uint = 0x05;
pub const UB953_IND_PGEN_BAR_SIZE1: c_uint = 0x06;
pub const UB953_IND_PGEN_BAR_SIZE0: c_uint = 0x07;
pub const UB953_IND_PGEN_ACT_LPF1: c_uint = 0x08;
pub const UB953_IND_PGEN_ACT_LPF0: c_uint = 0x09;
pub const UB953_IND_PGEN_TOT_LPF1: c_uint = 0x0a;
pub const UB953_IND_PGEN_TOT_LPF0: c_uint = 0x0b;
pub const UB953_IND_PGEN_LINE_PD1: c_uint = 0x0c;
pub const UB953_IND_PGEN_LINE_PD0: c_uint = 0x0d;
pub const UB953_IND_PGEN_VBP: c_uint = 0x0e;
pub const UB953_IND_PGEN_VFP: c_uint = 0x0f;

pub const UB953_IND_ANA_TEMP_DYNAMIC_CFG: c_uint = 0x4b;

pub const UB953_IND_ANA_TEMP_STATIC_CFG: c_uint = 0x4c;

// UB971 Registers
pub const UB971_ENH_BC_CHK: c_uint = 0x4b;
