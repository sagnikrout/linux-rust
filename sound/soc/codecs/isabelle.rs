//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/isabelle.h
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
// isabelle.h - Low power high fidelity audio codec driver header file
//
// Copyright (c) 2012 Texas Instruments, Inc
//

// ISABELLE REGISTERS
pub const ISABELLE_PWR_CFG_REG: c_uint = 0x01;
pub const ISABELLE_PWR_EN_REG: c_uint = 0x02;
pub const ISABELLE_PS_EN1_REG: c_uint = 0x03;
pub const ISABELLE_INT1_STATUS_REG: c_uint = 0x04;
pub const ISABELLE_INT1_MASK_REG: c_uint = 0x05;
pub const ISABELLE_INT2_STATUS_REG: c_uint = 0x06;
pub const ISABELLE_INT2_MASK_REG: c_uint = 0x07;
pub const ISABELLE_HKCTL1_REG: c_uint = 0x08;
pub const ISABELLE_HKCTL2_REG: c_uint = 0x09;
pub const ISABELLE_HKCTL3_REG: c_uint = 0x0A;
pub const ISABELLE_ACCDET_STATUS_REG: c_uint = 0x0B;
pub const ISABELLE_BUTTON_ID_REG: c_uint = 0x0C;
pub const ISABELLE_PLL_CFG_REG: c_uint = 0x10;
pub const ISABELLE_PLL_EN_REG: c_uint = 0x11;
pub const ISABELLE_FS_RATE_CFG_REG: c_uint = 0x12;
pub const ISABELLE_INTF_CFG_REG: c_uint = 0x13;
pub const ISABELLE_INTF_EN_REG: c_uint = 0x14;
pub const ISABELLE_ULATX12_INTF_CFG_REG: c_uint = 0x15;
pub const ISABELLE_DL12_INTF_CFG_REG: c_uint = 0x16;
pub const ISABELLE_DL34_INTF_CFG_REG: c_uint = 0x17;
pub const ISABELLE_DL56_INTF_CFG_REG: c_uint = 0x18;
pub const ISABELLE_ATX_STPGA1_CFG_REG: c_uint = 0x19;
pub const ISABELLE_ATX_STPGA2_CFG_REG: c_uint = 0x1A;
pub const ISABELLE_VTX_STPGA1_CFG_REG: c_uint = 0x1B;
pub const ISABELLE_VTX2_STPGA2_CFG_REG: c_uint = 0x1C;
pub const ISABELLE_ATX1_DPGA_REG: c_uint = 0x1D;
pub const ISABELLE_ATX2_DPGA_REG: c_uint = 0x1E;
pub const ISABELLE_VTX1_DPGA_REG: c_uint = 0x1F;
pub const ISABELLE_VTX2_DPGA_REG: c_uint = 0x20;
pub const ISABELLE_TX_INPUT_CFG_REG: c_uint = 0x21;
pub const ISABELLE_RX_INPUT_CFG_REG: c_uint = 0x22;
pub const ISABELLE_RX_INPUT_CFG2_REG: c_uint = 0x23;
pub const ISABELLE_VOICE_HPF_CFG_REG: c_uint = 0x24;
pub const ISABELLE_AUDIO_HPF_CFG_REG: c_uint = 0x25;
pub const ISABELLE_RX1_DPGA_REG: c_uint = 0x26;
pub const ISABELLE_RX2_DPGA_REG: c_uint = 0x27;
pub const ISABELLE_RX3_DPGA_REG: c_uint = 0x28;
pub const ISABELLE_RX4_DPGA_REG: c_uint = 0x29;
pub const ISABELLE_RX5_DPGA_REG: c_uint = 0x2A;
pub const ISABELLE_RX6_DPGA_REG: c_uint = 0x2B;
pub const ISABELLE_ALU_TX_EN_REG: c_uint = 0x2C;
pub const ISABELLE_ALU_RX_EN_REG: c_uint = 0x2D;
pub const ISABELLE_IIR_RESYNC_REG: c_uint = 0x2E;
pub const ISABELLE_ABIAS_CFG_REG: c_uint = 0x30;
pub const ISABELLE_DBIAS_CFG_REG: c_uint = 0x31;
pub const ISABELLE_MIC1_GAIN_REG: c_uint = 0x32;
pub const ISABELLE_MIC2_GAIN_REG: c_uint = 0x33;
pub const ISABELLE_AMIC_CFG_REG: c_uint = 0x34;
pub const ISABELLE_DMIC_CFG_REG: c_uint = 0x35;
pub const ISABELLE_APGA_GAIN_REG: c_uint = 0x36;
pub const ISABELLE_APGA_CFG_REG: c_uint = 0x37;
pub const ISABELLE_TX_GAIN_DLY_REG: c_uint = 0x38;
pub const ISABELLE_RX_GAIN_DLY_REG: c_uint = 0x39;
pub const ISABELLE_RX_PWR_CTRL_REG: c_uint = 0x3A;
pub const ISABELLE_DPGA1LR_IN_SEL_REG: c_uint = 0x3B;
pub const ISABELLE_DPGA1L_GAIN_REG: c_uint = 0x3C;
pub const ISABELLE_DPGA1R_GAIN_REG: c_uint = 0x3D;
pub const ISABELLE_DPGA2L_IN_SEL_REG: c_uint = 0x3E;
pub const ISABELLE_DPGA2R_IN_SEL_REG: c_uint = 0x3F;
pub const ISABELLE_DPGA2L_GAIN_REG: c_uint = 0x40;
pub const ISABELLE_DPGA2R_GAIN_REG: c_uint = 0x41;
pub const ISABELLE_DPGA3LR_IN_SEL_REG: c_uint = 0x42;
pub const ISABELLE_DPGA3L_GAIN_REG: c_uint = 0x43;
pub const ISABELLE_DPGA3R_GAIN_REG: c_uint = 0x44;
pub const ISABELLE_DAC1_SOFTRAMP_REG: c_uint = 0x45;
pub const ISABELLE_DAC2_SOFTRAMP_REG: c_uint = 0x46;
pub const ISABELLE_DAC3_SOFTRAMP_REG: c_uint = 0x47;
pub const ISABELLE_DAC_CFG_REG: c_uint = 0x48;
pub const ISABELLE_EARDRV_CFG1_REG: c_uint = 0x49;
pub const ISABELLE_EARDRV_CFG2_REG: c_uint = 0x4A;
pub const ISABELLE_HSDRV_GAIN_REG: c_uint = 0x4B;
pub const ISABELLE_HSDRV_CFG1_REG: c_uint = 0x4C;
pub const ISABELLE_HSDRV_CFG2_REG: c_uint = 0x4D;
pub const ISABELLE_HS_NG_CFG1_REG: c_uint = 0x4E;
pub const ISABELLE_HS_NG_CFG2_REG: c_uint = 0x4F;
pub const ISABELLE_LINEAMP_GAIN_REG: c_uint = 0x50;
pub const ISABELLE_LINEAMP_CFG_REG: c_uint = 0x51;
pub const ISABELLE_HFL_VOL_CTRL_REG: c_uint = 0x52;
pub const ISABELLE_HFL_SFTVOL_CTRL_REG: c_uint = 0x53;
pub const ISABELLE_HFL_LIM_CTRL_1_REG: c_uint = 0x54;
pub const ISABELLE_HFL_LIM_CTRL_2_REG: c_uint = 0x55;
pub const ISABELLE_HFR_VOL_CTRL_REG: c_uint = 0x56;
pub const ISABELLE_HFR_SFTVOL_CTRL_REG: c_uint = 0x57;
pub const ISABELLE_HFR_LIM_CTRL_1_REG: c_uint = 0x58;
pub const ISABELLE_HFR_LIM_CTRL_2_REG: c_uint = 0x59;
pub const ISABELLE_HF_MODE_REG: c_uint = 0x5A;
pub const ISABELLE_HFLPGA_CFG_REG: c_uint = 0x5B;
pub const ISABELLE_HFRPGA_CFG_REG: c_uint = 0x5C;
pub const ISABELLE_HFDRV_CFG_REG: c_uint = 0x5D;
pub const ISABELLE_PDMOUT_CFG1_REG: c_uint = 0x5E;
pub const ISABELLE_PDMOUT_CFG2_REG: c_uint = 0x5F;
pub const ISABELLE_PDMOUT_L_WM_REG: c_uint = 0x60;
pub const ISABELLE_PDMOUT_R_WM_REG: c_uint = 0x61;
pub const ISABELLE_HF_NG_CFG1_REG: c_uint = 0x62;
pub const ISABELLE_HF_NG_CFG2_REG: c_uint = 0x63;
// ISABELLE_PWR_EN_REG (0x02h)

// ISABELLE DAI FORMATS
pub const ISABELLE_AIF_FMT_MASK: c_uint = 0x70;
pub const ISABELLE_I2S_MODE: c_uint = 0x0;
pub const ISABELLE_LEFT_J_MODE: c_uint = 0x1;
pub const ISABELLE_PDM_MODE: c_uint = 0x2;
pub const ISABELLE_AIF_LENGTH_MASK: c_uint = 0x30;
pub const ISABELLE_AIF_LENGTH_20: c_uint = 0x00;
pub const ISABELLE_AIF_LENGTH_32: c_uint = 0x10;
pub const ISABELLE_AIF_MS: c_uint = 0x80;
pub const ISABELLE_FS_RATE_MASK: c_uint = 0xF;
pub const ISABELLE_FS_RATE_8: c_uint = 0x0;
pub const ISABELLE_FS_RATE_11: c_uint = 0x1;
pub const ISABELLE_FS_RATE_12: c_uint = 0x2;
pub const ISABELLE_FS_RATE_16: c_uint = 0x4;
pub const ISABELLE_FS_RATE_22: c_uint = 0x5;
pub const ISABELLE_FS_RATE_24: c_uint = 0x6;
pub const ISABELLE_FS_RATE_32: c_uint = 0x8;
pub const ISABELLE_FS_RATE_44: c_uint = 0x9;
pub const ISABELLE_FS_RATE_48: c_uint = 0xA;
pub const ISABELLE_MAX_REGISTER: c_uint = 0xFF;
