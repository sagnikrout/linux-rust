//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42xx8.h
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
// cs42xx8.h - Cirrus Logic CS42448/CS42888 Audio CODEC driver header file
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <Guangyu.Chen@freescale.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs42xx8_driver_data {
    pub name: [c_char; 32],
    pub num_adcs: c_int,
}

extern "C" {
    pub fn cs42xx8_probe(dev: *mut device, regmap: *mut regmap, drvdata: *mut cs42xx8_driver_data) -> c_int;
}
// CS42888 register map
pub const CS42XX8_CHIPID: c_uint = 0x01	/* Chip ID */;
pub const CS42XX8_PWRCTL: c_uint = 0x02	/* Power Control */;
pub const CS42XX8_FUNCMOD: c_uint = 0x03	/* Functional Mode */;
pub const CS42XX8_INTF: c_uint = 0x04	/* Interface Formats */;
pub const CS42XX8_ADCCTL: c_uint = 0x05	/* ADC Control */;
pub const CS42XX8_TXCTL: c_uint = 0x06	/* Transition Control */;
pub const CS42XX8_DACMUTE: c_uint = 0x07	/* DAC Mute Control */;
pub const CS42XX8_VOLAOUT1: c_uint = 0x08	/* Volume Control AOUT1 */;
pub const CS42XX8_VOLAOUT2: c_uint = 0x09	/* Volume Control AOUT2 */;
pub const CS42XX8_VOLAOUT3: c_uint = 0x0A	/* Volume Control AOUT3 */;
pub const CS42XX8_VOLAOUT4: c_uint = 0x0B	/* Volume Control AOUT4 */;
pub const CS42XX8_VOLAOUT5: c_uint = 0x0C	/* Volume Control AOUT5 */;
pub const CS42XX8_VOLAOUT6: c_uint = 0x0D	/* Volume Control AOUT6 */;
pub const CS42XX8_VOLAOUT7: c_uint = 0x0E	/* Volume Control AOUT7 */;
pub const CS42XX8_VOLAOUT8: c_uint = 0x0F	/* Volume Control AOUT8 */;
pub const CS42XX8_DACINV: c_uint = 0x10	/* DAC Channel Invert */;
pub const CS42XX8_VOLAIN1: c_uint = 0x11	/* Volume Control AIN1 */;
pub const CS42XX8_VOLAIN2: c_uint = 0x12	/* Volume Control AIN2 */;
pub const CS42XX8_VOLAIN3: c_uint = 0x13	/* Volume Control AIN3 */;
pub const CS42XX8_VOLAIN4: c_uint = 0x14	/* Volume Control AIN4 */;
pub const CS42XX8_VOLAIN5: c_uint = 0x15	/* Volume Control AIN5 */;
pub const CS42XX8_VOLAIN6: c_uint = 0x16	/* Volume Control AIN6 */;
pub const CS42XX8_ADCINV: c_uint = 0x17	/* ADC Channel Invert */;
pub const CS42XX8_STATUSCTL: c_uint = 0x18	/* Status Control */;
pub const CS42XX8_STATUS: c_uint = 0x19	/* Status */;
pub const CS42XX8_STATUSM: c_uint = 0x1A	/* Status Mask */;
pub const CS42XX8_MUTEC: c_uint = 0x1B	/* MUTEC Pin Control */;

pub const CS42XX8_I2C_INCR: c_uint = 0x80;
// Chip I.D. and Revision Register (Address 01h)
pub const CS42XX8_CHIPID_CHIP_ID_MASK: c_uint = 0xF0;
pub const CS42XX8_CHIPID_REV_ID_MASK: c_uint = 0x0F;
// Power Control (Address 02h)
pub const CS42XX8_PWRCTL_PDN_ADC3_SHIFT: c_int = 7;

pub const CS42XX8_PWRCTL_PDN_ADC2_SHIFT: c_int = 6;

pub const CS42XX8_PWRCTL_PDN_ADC1_SHIFT: c_int = 5;

pub const CS42XX8_PWRCTL_PDN_DAC4_SHIFT: c_int = 4;

pub const CS42XX8_PWRCTL_PDN_DAC3_SHIFT: c_int = 3;

pub const CS42XX8_PWRCTL_PDN_DAC2_SHIFT: c_int = 2;

pub const CS42XX8_PWRCTL_PDN_DAC1_SHIFT: c_int = 1;

pub const CS42XX8_PWRCTL_PDN_SHIFT: c_int = 0;

// Functional Mode (Address 03h)
pub const CS42XX8_FUNCMOD_DAC_FM_SHIFT: c_int = 6;
pub const CS42XX8_FUNCMOD_DAC_FM_WIDTH: c_int = 2;

pub const CS42XX8_FUNCMOD_ADC_FM_SHIFT: c_int = 4;
pub const CS42XX8_FUNCMOD_ADC_FM_WIDTH: c_int = 2;

pub const CS42XX8_FUNCMOD_MFREQ_SHIFT: c_int = 1;
pub const CS42XX8_FUNCMOD_MFREQ_WIDTH: c_int = 3;

pub const CS42XX8_FM_SINGLE: c_int = 0;
pub const CS42XX8_FM_DOUBLE: c_int = 1;
pub const CS42XX8_FM_QUAD: c_int = 2;
pub const CS42XX8_FM_AUTO: c_int = 3;
// Interface Formats (Address 04h)
pub const CS42XX8_INTF_FREEZE_SHIFT: c_int = 7;

pub const CS42XX8_INTF_AUX_DIF_SHIFT: c_int = 6;

pub const CS42XX8_INTF_DAC_DIF_SHIFT: c_int = 3;
pub const CS42XX8_INTF_DAC_DIF_WIDTH: c_int = 3;

pub const CS42XX8_INTF_ADC_DIF_SHIFT: c_int = 0;
pub const CS42XX8_INTF_ADC_DIF_WIDTH: c_int = 3;

// ADC Control & DAC De-Emphasis (Address 05h)
pub const CS42XX8_ADCCTL_ADC_HPF_FREEZE_SHIFT: c_int = 7;

pub const CS42XX8_ADCCTL_DAC_DEM_SHIFT: c_int = 5;

pub const CS42XX8_ADCCTL_ADC1_SINGLE_SHIFT: c_int = 4;

pub const CS42XX8_ADCCTL_ADC2_SINGLE_SHIFT: c_int = 3;

pub const CS42XX8_ADCCTL_ADC3_SINGLE_SHIFT: c_int = 2;

pub const CS42XX8_ADCCTL_AIN5_MUX_SHIFT: c_int = 1;

pub const CS42XX8_ADCCTL_AIN6_MUX_SHIFT: c_int = 0;

// Transition Control (Address 06h)
pub const CS42XX8_TXCTL_DAC_SNGVOL_SHIFT: c_int = 7;

pub const CS42XX8_TXCTL_DAC_SZC_SHIFT: c_int = 5;
pub const CS42XX8_TXCTL_DAC_SZC_WIDTH: c_int = 2;

pub const CS42XX8_TXCTL_AMUTE_SHIFT: c_int = 4;

pub const CS42XX8_TXCTL_MUTE_ADC_SP_SHIFT: c_int = 3;

pub const CS42XX8_TXCTL_ADC_SNGVOL_SHIFT: c_int = 2;

pub const CS42XX8_TXCTL_ADC_SZC_SHIFT: c_int = 0;

// DAC Channel Mute (Address 07h)

pub const CS42XX8_DACMUTE_ALL: c_uint = 0xff;
// Status Control (Address 18h)
pub const CS42XX8_STATUSCTL_INI_SHIFT: c_int = 2;
pub const CS42XX8_STATUSCTL_INI_WIDTH: c_int = 2;

// Status (Address 19h)
pub const CS42XX8_STATUS_DAC_CLK_ERR_SHIFT: c_int = 4;

pub const CS42XX8_STATUS_ADC_CLK_ERR_SHIFT: c_int = 3;

pub const CS42XX8_STATUS_ADC3_OVFL_SHIFT: c_int = 2;

pub const CS42XX8_STATUS_ADC2_OVFL_SHIFT: c_int = 1;

pub const CS42XX8_STATUS_ADC1_OVFL_SHIFT: c_int = 0;

// Status Mask (Address 1Ah)
pub const CS42XX8_STATUS_DAC_CLK_ERR_M_SHIFT: c_int = 4;

pub const CS42XX8_STATUS_ADC_CLK_ERR_M_SHIFT: c_int = 3;

pub const CS42XX8_STATUS_ADC3_OVFL_M_SHIFT: c_int = 2;

pub const CS42XX8_STATUS_ADC2_OVFL_M_SHIFT: c_int = 1;

pub const CS42XX8_STATUS_ADC1_OVFL_M_SHIFT: c_int = 0;

// MUTEC Pin Control (Address 1Bh)
pub const CS42XX8_MUTEC_MCPOLARITY_SHIFT: c_int = 1;

pub const CS42XX8_MUTEC_MUTEC_ACTIVE_SHIFT: c_int = 0;

