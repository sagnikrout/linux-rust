//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs4234.h
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
// ALSA SoC Audio driver for CS4234 codec
//
// Copyright (C) 2020 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
pub const CS4234_DEVID_AB: c_uint = 0x01;
pub const CS4234_DEVID_CD: c_uint = 0x02;
pub const CS4234_DEVID_EF: c_uint = 0x03;
pub const CS4234_REVID: c_uint = 0x05;
pub const CS4234_CLOCK_SP: c_uint = 0x06;
pub const CS4234_BASE_RATE_MASK: c_uint = 0xC0;
pub const CS4234_BASE_RATE_SHIFT: c_int = 6;
pub const CS4234_SPEED_MODE_MASK: c_uint = 0x30;
pub const CS4234_SPEED_MODE_SHIFT: c_int = 4;
pub const CS4234_MCLK_RATE_MASK: c_uint = 0x0E;
pub const CS4234_MCLK_RATE_SHIFT: c_int = 1;
pub const CS4234_SAMPLE_WIDTH: c_uint = 0x07;
pub const CS4234_SDOUTX_SW_MASK: c_uint = 0xC0;
pub const CS4234_SDOUTX_SW_SHIFT: c_int = 6;
pub const CS4234_INPUT_SW_MASK: c_uint = 0x30;
pub const CS4234_INPUT_SW_SHIFT: c_int = 4;
pub const CS4234_LOW_LAT_SW_MASK: c_uint = 0x0C;
pub const CS4234_LOW_LAT_SW_SHIFT: c_int = 2;
pub const CS4234_DAC5_SW_MASK: c_uint = 0x03;
pub const CS4234_DAC5_SW_SHIFT: c_int = 0;
pub const CS4234_SP_CTRL: c_uint = 0x08;
pub const CS4234_INVT_SCLK_MASK: c_uint = 0x80;
pub const CS4234_INVT_SCLK_SHIFT: c_int = 7;
pub const CS4234_DAC5_SRC_MASK: c_uint = 0x70;
pub const CS4234_DAC5_SRC_SHIFT: c_int = 4;
pub const CS4234_SP_FORMAT_MASK: c_uint = 0x0C;
pub const CS4234_SP_FORMAT_SHIFT: c_int = 2;
pub const CS4234_SDO_CHAIN_MASK: c_uint = 0x02;
pub const CS4234_SDO_CHAIN_SHIFT: c_int = 1;
pub const CS4234_MST_SLV_MASK: c_uint = 0x01;
pub const CS4234_MST_SLV_SHIFT: c_int = 0;
pub const CS4234_SP_DATA_SEL: c_uint = 0x09;
pub const CS4234_DAC14_SRC_MASK: c_uint = 0x38;
pub const CS4234_DAC14_SRC_SHIFT: c_int = 3;
pub const CS4234_LL_SRC_MASK: c_uint = 0x07;
pub const CS4234_LL_SRC_SHIFT: c_int = 0;
pub const CS4234_SDIN1_MASK1: c_uint = 0x0A;
pub const CS4234_SDIN1_MASK2: c_uint = 0x0B;
pub const CS4234_SDIN2_MASK1: c_uint = 0x0C;
pub const CS4234_SDIN2_MASK2: c_uint = 0x0D;
pub const CS4234_TPS_CTRL: c_uint = 0x0E;
pub const CS4234_TPS_MODE_MASK: c_uint = 0x80;
pub const CS4234_TPS_MODE_SHIFT: c_int = 7;
pub const CS4234_TPS_OFST_MASK: c_uint = 0x70;
pub const CS4234_TPS_OFST_SHIFT: c_int = 4;
pub const CS4234_GRP_DELAY_MASK: c_uint = 0x0F;
pub const CS4234_GRP_DELAY_SHIFT: c_int = 0;
pub const CS4234_ADC_CTRL1: c_uint = 0x0F;
pub const CS4234_VA_SEL_MASK: c_uint = 0x20;
pub const CS4234_VA_SEL_SHIFT: c_int = 5;
pub const CS4234_ENA_HPF_MASK: c_uint = 0x10;
pub const CS4234_ENA_HPF_SHIFT: c_int = 4;
pub const CS4234_INV_ADC_MASK: c_uint = 0x0F;
pub const CS4234_INV_ADC4_MASK: c_uint = 0x08;
pub const CS4234_INV_ADC4_SHIFT: c_int = 3;
pub const CS4234_INV_ADC3_MASK: c_uint = 0x04;
pub const CS4234_INV_ADC3_SHIFT: c_int = 2;
pub const CS4234_INV_ADC2_MASK: c_uint = 0x02;
pub const CS4234_INV_ADC2_SHIFT: c_int = 1;
pub const CS4234_INV_ADC1_MASK: c_uint = 0x01;
pub const CS4234_INV_ADC1_SHIFT: c_int = 0;
pub const CS4234_ADC_CTRL2: c_uint = 0x10;
pub const CS4234_MUTE_ADC4_MASK: c_uint = 0x80;
pub const CS4234_MUTE_ADC4_SHIFT: c_int = 7;
pub const CS4234_MUTE_ADC3_MASK: c_uint = 0x40;
pub const CS4234_MUTE_ADC3_SHIFT: c_int = 6;
pub const CS4234_MUTE_ADC2_MASK: c_uint = 0x20;
pub const CS4234_MUTE_ADC2_SHIFT: c_int = 5;
pub const CS4234_MUTE_ADC1_MASK: c_uint = 0x10;
pub const CS4234_MUTE_ADC1_SHIFT: c_int = 4;
pub const CS4234_PDN_ADC4_MASK: c_uint = 0x08;
pub const CS4234_PDN_ADC4_SHIFT: c_int = 3;
pub const CS4234_PDN_ADC3_MASK: c_uint = 0x04;
pub const CS4234_PDN_ADC3_SHIFT: c_int = 2;
pub const CS4234_PDN_ADC2_MASK: c_uint = 0x02;
pub const CS4234_PDN_ADC2_SHIFT: c_int = 1;
pub const CS4234_PDN_ADC1_MASK: c_uint = 0x01;
pub const CS4234_PDN_ADC1_SHIFT: c_int = 0;
pub const CS4234_LOW_LAT_CTRL1: c_uint = 0x11;
pub const CS4234_LL_NG_MASK: c_uint = 0xE0;
pub const CS4234_LL_NG_SHIFT: c_int = 5;
pub const CS4234_INV_LL_MASK: c_uint = 0x0F;
pub const CS4234_INV_LL4_MASK: c_uint = 0x08;
pub const CS4234_INV_LL4_SHIFT: c_int = 3;
pub const CS4234_INV_LL3_MASK: c_uint = 0x04;
pub const CS4234_INV_LL3_SHIFT: c_int = 2;
pub const CS4234_INV_LL2_MASK: c_uint = 0x02;
pub const CS4234_INV_LL2_SHIFT: c_int = 1;
pub const CS4234_INV_LL1_MASK: c_uint = 0x01;
pub const CS4234_INV_LL1_SHIFT: c_int = 0;
pub const CS4234_DAC_CTRL1: c_uint = 0x12;
pub const CS4234_DAC14_NG_MASK: c_uint = 0xE0;
pub const CS4234_DAC14_NG_SHIFT: c_int = 5;
pub const CS4234_DAC14_DE_MASK: c_uint = 0x10;
pub const CS4234_DAC14_DE_SHIFT: c_int = 4;
pub const CS4234_DAC5_DE_MASK: c_uint = 0x08;
pub const CS4234_DAC5_DE_SHIFT: c_int = 3;
pub const CS4234_DAC5_MVC_MASK: c_uint = 0x04;
pub const CS4234_DAC5_MVC_SHIFT: c_int = 2;
pub const CS4234_DAC5_CFG_FLTR_MASK: c_uint = 0x03;
pub const CS4234_DAC5_CFG_FLTR_SHIFT: c_int = 0;
pub const CS4234_DAC_CTRL2: c_uint = 0x13;
pub const CS4234_DAC5_NG_MASK: c_uint = 0xE0;
pub const CS4234_DAC5_NG_SHIFT: c_int = 5;
pub const CS4234_INV_DAC_MASK: c_uint = 0x1F;
pub const CS4234_INV_DAC5_MASK: c_uint = 0x10;
pub const CS4234_INV_DAC5_SHIFT: c_int = 4;
pub const CS4234_INV_DAC4_MASK: c_uint = 0x08;
pub const CS4234_INV_DAC4_SHIFT: c_int = 3;
pub const CS4234_INV_DAC3_MASK: c_uint = 0x04;
pub const CS4234_INV_DAC3_SHIFT: c_int = 2;
pub const CS4234_INV_DAC2_MASK: c_uint = 0x02;
pub const CS4234_INV_DAC2_SHIFT: c_int = 1;
pub const CS4234_INV_DAC1_MASK: c_uint = 0x01;
pub const CS4234_INV_DAC1_SHIFT: c_int = 0;
pub const CS4234_DAC_CTRL3: c_uint = 0x14;
pub const CS4234_DAC5_ATT_MASK: c_uint = 0x80;
pub const CS4234_DAC5_ATT_SHIFT: c_int = 7;
pub const CS4234_DAC14_ATT_MASK: c_uint = 0x40;
pub const CS4234_DAC14_ATT_SHIFT: c_int = 6;
pub const CS4234_MUTE_LL_MASK: c_uint = 0x20;
pub const CS4234_MUTE_LL_SHIFT: c_int = 5;
pub const CS4234_MUTE_DAC5_MASK: c_uint = 0x10;
pub const CS4234_MUTE_DAC5_SHIFT: c_int = 4;
pub const CS4234_MUTE_DAC4_MASK: c_uint = 0x08;
pub const CS4234_MUTE_DAC4_SHIFT: c_int = 3;
pub const CS4234_MUTE_DAC3_MASK: c_uint = 0x04;
pub const CS4234_MUTE_DAC3_SHIFT: c_int = 2;
pub const CS4234_MUTE_DAC2_MASK: c_uint = 0x02;
pub const CS4234_MUTE_DAC2_SHIFT: c_int = 1;
pub const CS4234_MUTE_DAC1_MASK: c_uint = 0x01;
pub const CS4234_MUTE_DAC1_SHIFT: c_int = 0;
pub const CS4234_DAC_CTRL4: c_uint = 0x15;
pub const CS4234_VQ_RAMP_MASK: c_uint = 0x80;
pub const CS4234_VQ_RAMP_SHIFT: c_int = 7;
pub const CS4234_TPS_GAIN_MASK: c_uint = 0x40;
pub const CS4234_TPS_GAIN_SHIFT: c_int = 6;
pub const CS4234_PDN_DAC5_MASK: c_uint = 0x10;
pub const CS4234_PDN_DAC5_SHIFT: c_int = 4;
pub const CS4234_PDN_DAC4_MASK: c_uint = 0x08;
pub const CS4234_PDN_DAC4_SHIFT: c_int = 3;
pub const CS4234_PDN_DAC3_MASK: c_uint = 0x04;
pub const CS4234_PDN_DAC3_SHIFT: c_int = 2;
pub const CS4234_PDN_DAC2_MASK: c_uint = 0x02;
pub const CS4234_PDN_DAC2_SHIFT: c_int = 1;
pub const CS4234_PDN_DAC1_MASK: c_uint = 0x01;
pub const CS4234_PDN_DAC1_SHIFT: c_int = 0;
pub const CS4234_VOLUME_MODE: c_uint = 0x16;
pub const CS4234_MUTE_DELAY_MASK: c_uint = 0xC0;
pub const CS4234_MUTE_DELAY_SHIFT: c_int = 6;
pub const CS4234_MIN_DELAY_MASK: c_uint = 0x38;
pub const CS4234_MIN_DELAY_SHIFT: c_int = 3;
pub const CS4234_MAX_DELAY_MASK: c_uint = 0x07;
pub const CS4234_MAX_DELAY_SHIFT: c_int = 0;
pub const CS4234_MASTER_VOL: c_uint = 0x17;
pub const CS4234_DAC1_VOL: c_uint = 0x18;
pub const CS4234_DAC2_VOL: c_uint = 0x19;
pub const CS4234_DAC3_VOL: c_uint = 0x1A;
pub const CS4234_DAC4_VOL: c_uint = 0x1B;
pub const CS4234_DAC5_VOL: c_uint = 0x1C;
pub const CS4234_INT_CTRL: c_uint = 0x1E;
pub const CS4234_INT_MODE_MASK: c_uint = 0x80;
pub const CS4234_INT_MODE_SHIFT: c_int = 7;
pub const CS4234_INT_PIN_MASK: c_uint = 0x60;
pub const CS4234_INT_PIN_SHIFT: c_int = 5;
pub const CS4234_INT_MASK1: c_uint = 0x1F;
pub const CS4234_MSK_TST_MODE_MASK: c_uint = 0x80;
pub const CS4234_MSK_TST_MODE_ERR_SHIFT: c_int = 7;
pub const CS4234_MSK_SP_ERR_MASK: c_uint = 0x40;
pub const CS4234_MSK_SP_ERR_SHIFT: c_int = 6;
pub const CS4234_MSK_CLK_ERR_MASK: c_uint = 0x08;
pub const CS4234_MSK_CLK_ERR_SHIFT: c_int = 5;
pub const CS4234_MSK_ADC4_OVFL_MASK: c_uint = 0x08;
pub const CS4234_MSK_ADC4_OVFL_SHIFT: c_int = 3;
pub const CS4234_MSK_ADC3_OVFL_MASK: c_uint = 0x04;
pub const CS4234_MSK_ADC3_OVFL_SHIFT: c_int = 2;
pub const CS4234_MSK_ADC2_OVFL_MASK: c_uint = 0x02;
pub const CS4234_MSK_ADC2_OVFL_SHIFT: c_int = 1;
pub const CS4234_MSK_ADC1_OVFL_MASK: c_uint = 0x01;
pub const CS4234_MSK_ADC1_OVFL_SHIFT: c_int = 0;
pub const CS4234_INT_MASK2: c_uint = 0x20;
pub const CS4234_MSK_DAC5_CLIP_MASK: c_uint = 0x10;
pub const CS4234_MSK_DAC5_CLIP_SHIFT: c_int = 4;
pub const CS4234_MSK_DAC4_CLIP_MASK: c_uint = 0x08;
pub const CS4234_MSK_DAC4_CLIP_SHIFT: c_int = 3;
pub const CS4234_MSK_DAC3_CLIP_MASK: c_uint = 0x04;
pub const CS4234_MSK_DAC3_CLIP_SHIFT: c_int = 2;
pub const CS4234_MSK_DAC2_CLIP_MASK: c_uint = 0x02;
pub const CS4234_MSK_DAC2_CLIP_SHIFT: c_int = 1;
pub const CS4234_MSK_DAC1_CLIP_MASK: c_uint = 0x01;
pub const CS4234_MSK_DAC1_CLIP_SHIFT: c_int = 0;
pub const CS4234_INT_NOTIFY1: c_uint = 0x21;
pub const CS4234_TST_MODE_MASK: c_uint = 0x80;
pub const CS4234_TST_MODE_SHIFT: c_int = 7;
pub const CS4234_SP_ERR_MASK: c_uint = 0x40;
pub const CS4234_SP_ERR_SHIFT: c_int = 6;
pub const CS4234_CLK_MOD_ERR_MASK: c_uint = 0x08;
pub const CS4234_CLK_MOD_ERR_SHIFT: c_int = 5;
pub const CS4234_ADC4_OVFL_MASK: c_uint = 0x08;
pub const CS4234_ADC4_OVFL_SHIFT: c_int = 3;
pub const CS4234_ADC3_OVFL_MASK: c_uint = 0x04;
pub const CS4234_ADC3_OVFL_SHIFT: c_int = 2;
pub const CS4234_ADC2_OVFL_MASK: c_uint = 0x02;
pub const CS4234_ADC2_OVFL_SHIFT: c_int = 1;
pub const CS4234_ADC1_OVFL_MASK: c_uint = 0x01;
pub const CS4234_ADC1_OVFL_SHIFT: c_int = 0;
pub const CS4234_INT_NOTIFY2: c_uint = 0x22;
pub const CS4234_DAC5_CLIP_MASK: c_uint = 0x10;
pub const CS4234_DAC5_CLIP_SHIFT: c_int = 4;
pub const CS4234_DAC4_CLIP_MASK: c_uint = 0x08;
pub const CS4234_DAC4_CLIP_SHIFT: c_int = 3;
pub const CS4234_DAC3_CLIP_MASK: c_uint = 0x04;
pub const CS4234_DAC3_CLIP_SHIFT: c_int = 2;
pub const CS4234_DAC2_CLIP_MASK: c_uint = 0x02;
pub const CS4234_DAC2_CLIP_SHIFT: c_int = 1;
pub const CS4234_DAC1_CLIP_MASK: c_uint = 0x01;
pub const CS4234_DAC1_CLIP_SHIFT: c_int = 0;

pub const CS4234_SUPPORTED_ID: c_uint = 0x423400;
pub const CS4234_BOOT_TIME_US: c_int = 3000;
pub const CS4234_HOLD_RESET_TIME_US: c_int = 1000;
pub const CS4234_VQ_CHARGE_MS: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs4234_supplies {
    CS4234_SUPPLY_VA = 0,
    CS4234_SUPPLY_VL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs4234_va_sel {
    CS4234_3V3 = 0,
    CS4234_5V,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs4234_sp_format {
    CS4234_LEFT_J = 0,
    CS4234_I2S,
    CS4234_TDM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs4234_base_rate_advisory {
    CS4234_48K = 0,
    CS4234_44K1,
    CS4234_32K,
}
