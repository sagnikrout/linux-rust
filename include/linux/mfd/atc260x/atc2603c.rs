//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/atc260x/atc2603c.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// ATC2603C PMIC register definitions
//
// Copyright (C) 2020 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atc2603c_irq_def {
    ATC2603C_IRQ_AUDIO = 0,
    ATC2603C_IRQ_OV,
    ATC2603C_IRQ_OC,
    ATC2603C_IRQ_OT,
    ATC2603C_IRQ_UV,
    ATC2603C_IRQ_ALARM,
    ATC2603C_IRQ_ONOFF,
    ATC2603C_IRQ_SGPIO,
    ATC2603C_IRQ_IR,
    ATC2603C_IRQ_REMCON,
    ATC2603C_IRQ_POWER_IN,
}

// PMU Registers
pub const ATC2603C_PMU_SYS_CTL0: c_uint = 0x00;
pub const ATC2603C_PMU_SYS_CTL1: c_uint = 0x01;
pub const ATC2603C_PMU_SYS_CTL2: c_uint = 0x02;
pub const ATC2603C_PMU_SYS_CTL3: c_uint = 0x03;
pub const ATC2603C_PMU_SYS_CTL4: c_uint = 0x04;
pub const ATC2603C_PMU_SYS_CTL5: c_uint = 0x05;
pub const ATC2603C_PMU_SYS_CTL6: c_uint = 0x06;
pub const ATC2603C_PMU_SYS_CTL7: c_uint = 0x07;
pub const ATC2603C_PMU_SYS_CTL8: c_uint = 0x08;
pub const ATC2603C_PMU_SYS_CTL9: c_uint = 0x09;
pub const ATC2603C_PMU_BAT_CTL0: c_uint = 0x0A;
pub const ATC2603C_PMU_BAT_CTL1: c_uint = 0x0B;
pub const ATC2603C_PMU_VBUS_CTL0: c_uint = 0x0C;
pub const ATC2603C_PMU_VBUS_CTL1: c_uint = 0x0D;
pub const ATC2603C_PMU_WALL_CTL0: c_uint = 0x0E;
pub const ATC2603C_PMU_WALL_CTL1: c_uint = 0x0F;
pub const ATC2603C_PMU_SYS_PENDING: c_uint = 0x10;
pub const ATC2603C_PMU_DC1_CTL0: c_uint = 0x11;
pub const ATC2603C_PMU_DC1_CTL1: c_uint = 0x12 // Undocumented;
pub const ATC2603C_PMU_DC1_CTL2: c_uint = 0x13 // Undocumented;
pub const ATC2603C_PMU_DC2_CTL0: c_uint = 0x14;
pub const ATC2603C_PMU_DC2_CTL1: c_uint = 0x15 // Undocumented;
pub const ATC2603C_PMU_DC2_CTL2: c_uint = 0x16 // Undocumented;
pub const ATC2603C_PMU_DC3_CTL0: c_uint = 0x17;
pub const ATC2603C_PMU_DC3_CTL1: c_uint = 0x18 // Undocumented;
pub const ATC2603C_PMU_DC3_CTL2: c_uint = 0x19 // Undocumented;
pub const ATC2603C_PMU_DC4_CTL0: c_uint = 0x1A // Undocumented;
pub const ATC2603C_PMU_DC4_CTL1: c_uint = 0x1B // Undocumented;
pub const ATC2603C_PMU_DC5_CTL0: c_uint = 0x1C // Undocumented;
pub const ATC2603C_PMU_DC5_CTL1: c_uint = 0x1D // Undocumented;
pub const ATC2603C_PMU_LDO1_CTL: c_uint = 0x1E;
pub const ATC2603C_PMU_LDO2_CTL: c_uint = 0x1F;
pub const ATC2603C_PMU_LDO3_CTL: c_uint = 0x20;
pub const ATC2603C_PMU_LDO4_CTL: c_uint = 0x21 // Undocumented;
pub const ATC2603C_PMU_LDO5_CTL: c_uint = 0x22;
pub const ATC2603C_PMU_LDO6_CTL: c_uint = 0x23;
pub const ATC2603C_PMU_LDO7_CTL: c_uint = 0x24;
pub const ATC2603C_PMU_LDO8_CTL: c_uint = 0x25 // Undocumented;
pub const ATC2603C_PMU_LDO9_CTL: c_uint = 0x26 // Undocumented;
pub const ATC2603C_PMU_LDO10_CTL: c_uint = 0x27 // Undocumented;
pub const ATC2603C_PMU_LDO11_CTL: c_uint = 0x28;
pub const ATC2603C_PMU_SWITCH_CTL: c_uint = 0x29;
pub const ATC2603C_PMU_OV_CTL0: c_uint = 0x2A;
pub const ATC2603C_PMU_OV_CTL1: c_uint = 0x2B;
pub const ATC2603C_PMU_OV_STATUS: c_uint = 0x2C;
pub const ATC2603C_PMU_OV_EN: c_uint = 0x2D;
pub const ATC2603C_PMU_OV_INT_EN: c_uint = 0x2E;
pub const ATC2603C_PMU_OC_CTL: c_uint = 0x2F;
pub const ATC2603C_PMU_OC_STATUS: c_uint = 0x30;
pub const ATC2603C_PMU_OC_EN: c_uint = 0x31;
pub const ATC2603C_PMU_OC_INT_EN: c_uint = 0x32;
pub const ATC2603C_PMU_UV_CTL0: c_uint = 0x33;
pub const ATC2603C_PMU_UV_CTL1: c_uint = 0x34;
pub const ATC2603C_PMU_UV_STATUS: c_uint = 0x35;
pub const ATC2603C_PMU_UV_EN: c_uint = 0x36;
pub const ATC2603C_PMU_UV_INT_EN: c_uint = 0x37;
pub const ATC2603C_PMU_OT_CTL: c_uint = 0x38;
pub const ATC2603C_PMU_CHARGER_CTL0: c_uint = 0x39;
pub const ATC2603C_PMU_CHARGER_CTL1: c_uint = 0x3A;
pub const ATC2603C_PMU_CHARGER_CTL2: c_uint = 0x3B;
pub const ATC2603C_PMU_BAKCHARGER_CTL: c_uint = 0x3C // Undocumented;
pub const ATC2603C_PMU_APDS_CTL: c_uint = 0x3D;
pub const ATC2603C_PMU_AUXADC_CTL0: c_uint = 0x3E;
pub const ATC2603C_PMU_AUXADC_CTL1: c_uint = 0x3F;
pub const ATC2603C_PMU_BATVADC: c_uint = 0x40;
pub const ATC2603C_PMU_BATIADC: c_uint = 0x41;
pub const ATC2603C_PMU_WALLVADC: c_uint = 0x42;
pub const ATC2603C_PMU_WALLIADC: c_uint = 0x43;
pub const ATC2603C_PMU_VBUSVADC: c_uint = 0x44;
pub const ATC2603C_PMU_VBUSIADC: c_uint = 0x45;
pub const ATC2603C_PMU_SYSPWRADC: c_uint = 0x46;
pub const ATC2603C_PMU_REMCONADC: c_uint = 0x47;
pub const ATC2603C_PMU_SVCCADC: c_uint = 0x48;
pub const ATC2603C_PMU_CHGIADC: c_uint = 0x49;
pub const ATC2603C_PMU_IREFADC: c_uint = 0x4A;
pub const ATC2603C_PMU_BAKBATADC: c_uint = 0x4B;
pub const ATC2603C_PMU_ICTEMPADC: c_uint = 0x4C;
pub const ATC2603C_PMU_AUXADC0: c_uint = 0x4D;
pub const ATC2603C_PMU_AUXADC1: c_uint = 0x4E;
pub const ATC2603C_PMU_AUXADC2: c_uint = 0x4F;
pub const ATC2603C_PMU_ICMADC: c_uint = 0x50;
pub const ATC2603C_PMU_BDG_CTL: c_uint = 0x51 // Undocumented;
pub const ATC2603C_RTC_CTL: c_uint = 0x52;
pub const ATC2603C_RTC_MSALM: c_uint = 0x53;
pub const ATC2603C_RTC_HALM: c_uint = 0x54;
pub const ATC2603C_RTC_YMDALM: c_uint = 0x55;
pub const ATC2603C_RTC_MS: c_uint = 0x56;
pub const ATC2603C_RTC_H: c_uint = 0x57;
pub const ATC2603C_RTC_DC: c_uint = 0x58;
pub const ATC2603C_RTC_YMD: c_uint = 0x59;
pub const ATC2603C_EFUSE_DAT: c_uint = 0x5A // Undocumented;
pub const ATC2603C_EFUSECRTL1: c_uint = 0x5B // Undocumented;
pub const ATC2603C_EFUSECRTL2: c_uint = 0x5C // Undocumented;
pub const ATC2603C_PMU_FW_USE0: c_uint = 0x5D // Undocumented;
pub const ATC2603C_PMU_FW_USE1: c_uint = 0x5E // Undocumented;
pub const ATC2603C_PMU_FW_USE2: c_uint = 0x5F // Undocumented;
pub const ATC2603C_PMU_FW_USE3: c_uint = 0x60 // Undocumented;
pub const ATC2603C_PMU_FW_USE4: c_uint = 0x61 // Undocumented;
pub const ATC2603C_PMU_ABNORMAL_STATUS: c_uint = 0x62;
pub const ATC2603C_PMU_WALL_APDS_CTL: c_uint = 0x63;
pub const ATC2603C_PMU_REMCON_CTL0: c_uint = 0x64;
pub const ATC2603C_PMU_REMCON_CTL1: c_uint = 0x65;
pub const ATC2603C_PMU_MUX_CTL0: c_uint = 0x66;
pub const ATC2603C_PMU_SGPIO_CTL0: c_uint = 0x67;
pub const ATC2603C_PMU_SGPIO_CTL1: c_uint = 0x68;
pub const ATC2603C_PMU_SGPIO_CTL2: c_uint = 0x69;
pub const ATC2603C_PMU_SGPIO_CTL3: c_uint = 0x6A;
pub const ATC2603C_PMU_SGPIO_CTL4: c_uint = 0x6B;
pub const ATC2603C_PWMCLK_CTL: c_uint = 0x6C;
pub const ATC2603C_PWM0_CTL: c_uint = 0x6D;
pub const ATC2603C_PWM1_CTL: c_uint = 0x6E;
pub const ATC2603C_PMU_ADC_DBG0: c_uint = 0x70;
pub const ATC2603C_PMU_ADC_DBG1: c_uint = 0x71;
pub const ATC2603C_PMU_ADC_DBG2: c_uint = 0x72;
pub const ATC2603C_PMU_ADC_DBG3: c_uint = 0x73;
pub const ATC2603C_PMU_ADC_DBG4: c_uint = 0x74;
pub const ATC2603C_IRC_CTL: c_uint = 0x80;
pub const ATC2603C_IRC_STAT: c_uint = 0x81;
pub const ATC2603C_IRC_CC: c_uint = 0x82;
pub const ATC2603C_IRC_KDC: c_uint = 0x83;
pub const ATC2603C_IRC_WK: c_uint = 0x84;
pub const ATC2603C_IRC_RCC: c_uint = 0x85;
pub const ATC2603C_IRC_FILTER: c_uint = 0x86;
// AUDIO_OUT Registers
pub const ATC2603C_AUDIOINOUT_CTL: c_uint = 0xA0;
pub const ATC2603C_AUDIO_DEBUGOUTCTL: c_uint = 0xA1;
pub const ATC2603C_DAC_DIGITALCTL: c_uint = 0xA2;
pub const ATC2603C_DAC_VOLUMECTL0: c_uint = 0xA3;
pub const ATC2603C_DAC_ANALOG0: c_uint = 0xA4;
pub const ATC2603C_DAC_ANALOG1: c_uint = 0xA5;
pub const ATC2603C_DAC_ANALOG2: c_uint = 0xA6;
pub const ATC2603C_DAC_ANALOG3: c_uint = 0xA7;
// AUDIO_IN Registers
pub const ATC2603C_ADC_DIGITALCTL: c_uint = 0xA8;
pub const ATC2603C_ADC_HPFCTL: c_uint = 0xA9;
pub const ATC2603C_ADC_CTL: c_uint = 0xAA;
pub const ATC2603C_AGC_CTL0: c_uint = 0xAB;
pub const ATC2603C_AGC_CTL1: c_uint = 0xAC // Undocumented;
pub const ATC2603C_AGC_CTL2: c_uint = 0xAD;
pub const ATC2603C_ADC_ANALOG0: c_uint = 0xAE;
pub const ATC2603C_ADC_ANALOG1: c_uint = 0xAF;
// PCM_IF Registers
pub const ATC2603C_PCM0_CTL: c_uint = 0xB0 // Undocumented;
pub const ATC2603C_PCM1_CTL: c_uint = 0xB1 // Undocumented;
pub const ATC2603C_PCM2_CTL: c_uint = 0xB2 // Undocumented;
pub const ATC2603C_PCMIF_CTL: c_uint = 0xB3 // Undocumented;
// CMU_CONTROL Registers
pub const ATC2603C_CMU_DEVRST: c_uint = 0xC1 // Undocumented;
// INTS Registers
pub const ATC2603C_INTS_PD: c_uint = 0xC8;
pub const ATC2603C_INTS_MSK: c_uint = 0xC9;
// MFP Registers
pub const ATC2603C_MFP_CTL: c_uint = 0xD0;
pub const ATC2603C_PAD_VSEL: c_uint = 0xD1 // Undocumented;
pub const ATC2603C_GPIO_OUTEN: c_uint = 0xD2;
pub const ATC2603C_GPIO_INEN: c_uint = 0xD3;
pub const ATC2603C_GPIO_DAT: c_uint = 0xD4;
pub const ATC2603C_PAD_DRV: c_uint = 0xD5;
pub const ATC2603C_PAD_EN: c_uint = 0xD6;
pub const ATC2603C_DEBUG_SEL: c_uint = 0xD7 // Undocumented;
pub const ATC2603C_DEBUG_IE: c_uint = 0xD8 // Undocumented;
pub const ATC2603C_DEBUG_OE: c_uint = 0xD9 // Undocumented;
pub const ATC2603C_BIST_START: c_uint = 0x0A // Undocumented;
pub const ATC2603C_BIST_RESULT: c_uint = 0x0B // Undocumented;
pub const ATC2603C_CHIP_VER: c_uint = 0xDC;
// TWSI Registers
pub const ATC2603C_SADDR: c_uint = 0xFF;
// PMU_SYS_CTL0 Register Mask Bits

// PMU_SYS_CTL1 Register Mask Bits

// PMU_SYS_CTL2 Register Mask Bits

// PMU_SYS_CTL3 Register Mask Bits

// PMU_SYS_CTL5 Register Mask Bits

// INTS_MSK Register Mask Bits

// CMU_DEVRST Register Mask Bits

// PAD_EN Register Mask Bits

