//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8375.h
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
// ES8375.h  --  ES8375 ALSA SoC Audio Codec
//
// Authors:
//
// Based on ES8375.h by Michael Zhang
//
// Registors
pub const ES8375_RESET1: c_uint = 0x00;
pub const ES8375_MCLK_SEL: c_uint = 0x01;
pub const ES8375_CLK_MGR2: c_uint = 0x02;
pub const ES8375_CLK_MGR3: c_uint = 0x03;
pub const ES8375_CLK_MGR4: c_uint = 0x04;
pub const ES8375_CLK_MGR5: c_uint = 0x05;
pub const ES8375_CLK_MGR6: c_uint = 0x06;
pub const ES8375_CLK_MGR7: c_uint = 0x07;
pub const ES8375_CLK_MGR8: c_uint = 0x08;
pub const ES8375_CLK_MGR9: c_uint = 0x09;
pub const ES8375_CLK_MGR10: c_uint = 0x0A;
pub const ES8375_CLK_MGR11: c_uint = 0x0B;
pub const ES8375_CLK_MGR12: c_uint = 0x0C;
pub const ES8375_DIV_SPKCLK: c_uint = 0x0E;
pub const ES8375_CSM1: c_uint = 0x0F;
pub const ES8375_CSM2: c_uint = 0x10;
pub const ES8375_VMID_CHARGE2: c_uint = 0x11;
pub const ES8375_VMID_CHARGE3: c_uint = 0x12;
pub const ES8375_SDP: c_uint = 0x15;
pub const ES8375_SDP2: c_uint = 0x16;
pub const ES8375_ADC1: c_uint = 0x17;
pub const ES8375_ADC2: c_uint = 0x18;
pub const ES8375_ADC_OSR_GAIN: c_uint = 0x19;
pub const ES8375_ADC_VOLUME: c_uint = 0x1A;
pub const ES8375_ADC_AUTOMUTE: c_uint = 0x1B;
pub const ES8375_ADC_AUTOMUTE_ATTN: c_uint = 0x1C;
pub const ES8375_HPF1: c_uint = 0x1D;
pub const ES8375_DAC1: c_uint = 0x1F;
pub const ES8375_DAC2: c_uint = 0x20;
pub const ES8375_DAC_VOLUME: c_uint = 0x21;
pub const ES8375_DAC_VPPSCALE: c_uint = 0x22;
pub const ES8375_DAC_AUTOMUTE1: c_uint = 0x23;
pub const ES8375_DAC_AUTOMUTE: c_uint = 0x24;
pub const ES8375_DAC_CAL: c_uint = 0x25;
pub const ES8375_DAC_OTP: c_uint = 0x27;
pub const ES8375_ANALOG_SPK1: c_uint = 0x28;
pub const ES8375_ANALOG_SPK2: c_uint = 0x29;
pub const ES8375_VMID_SEL: c_uint = 0x2D;
pub const ES8375_ANALOG1: c_uint = 0x2E;
pub const ES8375_ANALOG2: c_uint = 0x32;
pub const ES8375_ANALOG3: c_uint = 0x37;
pub const ES8375_ADC2DAC_CLKTRI: c_uint = 0xF8;
pub const ES8375_SYS_CTRL2: c_uint = 0xF9;
pub const ES8375_FLAGS2: c_uint = 0xFB;
pub const ES8375_SPK_OFFSET: c_uint = 0xFC;
pub const ES8375_CHIP_ID1: c_uint = 0xFD;
pub const ES8375_CHIP_ID0: c_uint = 0xFE;
pub const ES8375_CHIP_VERSION: c_uint = 0xFF;
// Bit Shifts
pub const ADC_OSR_GAIN_SHIFT_0: c_int = 0;
pub const ADC_RAMPRATE_SHIFT_0: c_int = 0;
pub const ADC_VOLUME_SHIFT_0: c_int = 0;
pub const ADC_AUTOMUTE_NG_SHIFT_0: c_int = 0;
pub const ADC_AUTOMUTE_ATTN_SHIFT_0: c_int = 0;
pub const DAC_RAMPRATE_SHIFT_0: c_int = 0;
pub const DAC_VOLUME_SHIFT_0: c_int = 0;
pub const DAC_VPPSCALE_SHIFT_0: c_int = 0;
pub const DAC_AUTOMUTE_NG_SHIFT_0: c_int = 0;
pub const DAC_AUTOMUTE_ATTN_SHIFT_0: c_int = 0;
pub const DMIC_GAIN_SHIFT_2: c_int = 2;
pub const ADC_AUTOMUTE_WS_SHIFT_3: c_int = 3;
pub const DMIC_POL_SHIFT_4: c_int = 4;
pub const DAC_RAMCLR_SHIFT_4: c_int = 4;
pub const ES8375_EN_MODL_SHIFT_4: c_int = 4;
pub const ADC_RAMCLR_SHIFT_5: c_int = 5;
pub const ADC_HPF_SHIFT_5: c_int = 5;
pub const DAC_INV_SHIFT_5: c_int = 5;
pub const DAC_AUTOMUTE_WS_SHIFT_5: c_int = 5;
pub const ES8375_EN_PGAL_SHIFT_5: c_int = 5;
pub const ES8375_ADC_P2S_MUTE_SHIFT_5: c_int = 5;
pub const ADC_INV_SHIFT_6: c_int = 6;
pub const DAC_DEMMUTE_SHIFT_6: c_int = 6;
pub const ES8375_DAC_S2P_MUTE_SHIFT_6: c_int = 6;
pub const ADC_SRC_SHIFT_7: c_int = 7;
pub const ADC_AUTOMUTE_SHIFT_7: c_int = 7;
pub const DAC_DSMMUTE_SHIFT_7: c_int = 7;
pub const DAC_AUTOMUTE_EN_SHIFT_7: c_int = 7;
// Function values
pub const ES8375_ADC_OSR_GAIN_MAX: c_uint = 0x3F;
pub const ES8375_DMIC_GAIN_MAX: c_uint = 0x04;
pub const ES8375_ADC_AUTOMUTE_ATTN_MAX: c_uint = 0x1F;
pub const ES8375_AUTOMUTE_NG_MAX: c_uint = 0x07;
pub const ES8375_ADC_VOLUME_MAX: c_uint = 0xFF;
pub const ES8375_DAC_VOLUME_MAX: c_uint = 0xFF;
pub const ES8375_DAC_VPPSCALE_MAX: c_uint = 0x3F;
pub const ES8375_DAC_AUTOMUTE_ATTN_MAX: c_uint = 0x17;
pub const ES8375_REG_MAX: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ES8375_supplies {
    ES8375_SUPPLY_VD = 0,
    ES8375_SUPPLY_VA,
}

// Properties
pub const ES8375_3V3: c_int = 1;
pub const ES8375_1V8: c_int = 0;
pub const ES8375_MCLK_PIN: c_int = 0;
pub const ES8375_BCLK_PIN: c_int = 1;

pub const DMIC_POSITIVE_EDGE: c_int = 0;
pub const DMIC_NEGATIVE_EDGE: c_int = 1;

pub const PA_SHUTDOWN: c_int = 0;
pub const PA_ENABLE: c_int = 1;
