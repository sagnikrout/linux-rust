//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/atc260x/atc2609a.h
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
// ATC2609A PMIC register definitions
//
// Copyright (C) 2019 Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atc2609a_irq_def {
    ATC2609A_IRQ_AUDIO = 0,
    ATC2609A_IRQ_OV,
    ATC2609A_IRQ_OC,
    ATC2609A_IRQ_OT,
    ATC2609A_IRQ_UV,
    ATC2609A_IRQ_ALARM,
    ATC2609A_IRQ_ONOFF,
    ATC2609A_IRQ_WKUP,
    ATC2609A_IRQ_IR,
    ATC2609A_IRQ_REMCON,
    ATC2609A_IRQ_POWER_IN,
}

// PMU Registers
pub const ATC2609A_PMU_SYS_CTL0: c_uint = 0x00;
pub const ATC2609A_PMU_SYS_CTL1: c_uint = 0x01;
pub const ATC2609A_PMU_SYS_CTL2: c_uint = 0x02;
pub const ATC2609A_PMU_SYS_CTL3: c_uint = 0x03;
pub const ATC2609A_PMU_SYS_CTL4: c_uint = 0x04;
pub const ATC2609A_PMU_SYS_CTL5: c_uint = 0x05;
pub const ATC2609A_PMU_SYS_CTL6: c_uint = 0x06;
pub const ATC2609A_PMU_SYS_CTL7: c_uint = 0x07;
pub const ATC2609A_PMU_SYS_CTL8: c_uint = 0x08;
pub const ATC2609A_PMU_SYS_CTL9: c_uint = 0x09;
pub const ATC2609A_PMU_BAT_CTL0: c_uint = 0x0A;
pub const ATC2609A_PMU_BAT_CTL1: c_uint = 0x0B;
pub const ATC2609A_PMU_VBUS_CTL0: c_uint = 0x0C;
pub const ATC2609A_PMU_VBUS_CTL1: c_uint = 0x0D;
pub const ATC2609A_PMU_WALL_CTL0: c_uint = 0x0E;
pub const ATC2609A_PMU_WALL_CTL1: c_uint = 0x0F;
pub const ATC2609A_PMU_SYS_PENDING: c_uint = 0x10;
pub const ATC2609A_PMU_APDS_CTL0: c_uint = 0x11;
pub const ATC2609A_PMU_APDS_CTL1: c_uint = 0x12;
pub const ATC2609A_PMU_APDS_CTL2: c_uint = 0x13;
pub const ATC2609A_PMU_CHARGER_CTL: c_uint = 0x14;
pub const ATC2609A_PMU_BAKCHARGER_CTL: c_uint = 0x15;
pub const ATC2609A_PMU_SWCHG_CTL0: c_uint = 0x16;
pub const ATC2609A_PMU_SWCHG_CTL1: c_uint = 0x17;
pub const ATC2609A_PMU_SWCHG_CTL2: c_uint = 0x18;
pub const ATC2609A_PMU_SWCHG_CTL3: c_uint = 0x19;
pub const ATC2609A_PMU_SWCHG_CTL4: c_uint = 0x1A;
pub const ATC2609A_PMU_DC_OSC: c_uint = 0x1B;
pub const ATC2609A_PMU_DC0_CTL0: c_uint = 0x1C;
pub const ATC2609A_PMU_DC0_CTL1: c_uint = 0x1D;
pub const ATC2609A_PMU_DC0_CTL2: c_uint = 0x1E;
pub const ATC2609A_PMU_DC0_CTL3: c_uint = 0x1F;
pub const ATC2609A_PMU_DC0_CTL4: c_uint = 0x20;
pub const ATC2609A_PMU_DC0_CTL5: c_uint = 0x21;
pub const ATC2609A_PMU_DC0_CTL6: c_uint = 0x22;
pub const ATC2609A_PMU_DC1_CTL0: c_uint = 0x23;
pub const ATC2609A_PMU_DC1_CTL1: c_uint = 0x24;
pub const ATC2609A_PMU_DC1_CTL2: c_uint = 0x25;
pub const ATC2609A_PMU_DC1_CTL3: c_uint = 0x26;
pub const ATC2609A_PMU_DC1_CTL4: c_uint = 0x27;
pub const ATC2609A_PMU_DC1_CTL5: c_uint = 0x28;
pub const ATC2609A_PMU_DC1_CTL6: c_uint = 0x29;
pub const ATC2609A_PMU_DC2_CTL0: c_uint = 0x2A;
pub const ATC2609A_PMU_DC2_CTL1: c_uint = 0x2B;
pub const ATC2609A_PMU_DC2_CTL2: c_uint = 0x2C;
pub const ATC2609A_PMU_DC2_CTL3: c_uint = 0x2D;
pub const ATC2609A_PMU_DC2_CTL4: c_uint = 0x2E;
pub const ATC2609A_PMU_DC2_CTL5: c_uint = 0x2F;
pub const ATC2609A_PMU_DC2_CTL6: c_uint = 0x30;
pub const ATC2609A_PMU_DC3_CTL0: c_uint = 0x31;
pub const ATC2609A_PMU_DC3_CTL1: c_uint = 0x32;
pub const ATC2609A_PMU_DC3_CTL2: c_uint = 0x33;
pub const ATC2609A_PMU_DC3_CTL3: c_uint = 0x34;
pub const ATC2609A_PMU_DC3_CTL4: c_uint = 0x35;
pub const ATC2609A_PMU_DC3_CTL5: c_uint = 0x36;
pub const ATC2609A_PMU_DC3_CTL6: c_uint = 0x37;
pub const ATC2609A_PMU_DC_ZR: c_uint = 0x38;
pub const ATC2609A_PMU_LDO0_CTL0: c_uint = 0x39;
pub const ATC2609A_PMU_LDO0_CTL1: c_uint = 0x3A;
pub const ATC2609A_PMU_LDO1_CTL0: c_uint = 0x3B;
pub const ATC2609A_PMU_LDO1_CTL1: c_uint = 0x3C;
pub const ATC2609A_PMU_LDO2_CTL0: c_uint = 0x3D;
pub const ATC2609A_PMU_LDO2_CTL1: c_uint = 0x3E;
pub const ATC2609A_PMU_LDO3_CTL0: c_uint = 0x3F;
pub const ATC2609A_PMU_LDO3_CTL1: c_uint = 0x40;
pub const ATC2609A_PMU_LDO4_CTL0: c_uint = 0x41;
pub const ATC2609A_PMU_LDO4_CTL1: c_uint = 0x42;
pub const ATC2609A_PMU_LDO5_CTL0: c_uint = 0x43;
pub const ATC2609A_PMU_LDO5_CTL1: c_uint = 0x44;
pub const ATC2609A_PMU_LDO6_CTL0: c_uint = 0x45;
pub const ATC2609A_PMU_LDO6_CTL1: c_uint = 0x46;
pub const ATC2609A_PMU_LDO7_CTL0: c_uint = 0x47;
pub const ATC2609A_PMU_LDO7_CTL1: c_uint = 0x48;
pub const ATC2609A_PMU_LDO8_CTL0: c_uint = 0x49;
pub const ATC2609A_PMU_LDO8_CTL1: c_uint = 0x4A;
pub const ATC2609A_PMU_LDO9_CTL: c_uint = 0x4B;
pub const ATC2609A_PMU_OV_INT_EN: c_uint = 0x4C;
pub const ATC2609A_PMU_OV_STATUS: c_uint = 0x4D;
pub const ATC2609A_PMU_UV_INT_EN: c_uint = 0x4E;
pub const ATC2609A_PMU_UV_STATUS: c_uint = 0x4F;
pub const ATC2609A_PMU_OC_INT_EN: c_uint = 0x50;
pub const ATC2609A_PMU_OC_STATUS: c_uint = 0x51;
pub const ATC2609A_PMU_OT_CTL: c_uint = 0x52;
pub const ATC2609A_PMU_CM_CTL0: c_uint = 0x53;
pub const ATC2609A_PMU_FW_USE0: c_uint = 0x54;
pub const ATC2609A_PMU_FW_USE1: c_uint = 0x55;
pub const ATC2609A_PMU_ADC12B_I: c_uint = 0x56;
pub const ATC2609A_PMU_ADC12B_V: c_uint = 0x57;
pub const ATC2609A_PMU_ADC12B_DUMMY: c_uint = 0x58;
pub const ATC2609A_PMU_AUXADC_CTL0: c_uint = 0x59;
pub const ATC2609A_PMU_AUXADC_CTL1: c_uint = 0x5A;
pub const ATC2609A_PMU_BATVADC: c_uint = 0x5B;
pub const ATC2609A_PMU_BATIADC: c_uint = 0x5C;
pub const ATC2609A_PMU_WALLVADC: c_uint = 0x5D;
pub const ATC2609A_PMU_WALLIADC: c_uint = 0x5E;
pub const ATC2609A_PMU_VBUSVADC: c_uint = 0x5F;
pub const ATC2609A_PMU_VBUSIADC: c_uint = 0x60;
pub const ATC2609A_PMU_SYSPWRADC: c_uint = 0x61;
pub const ATC2609A_PMU_REMCONADC: c_uint = 0x62;
pub const ATC2609A_PMU_SVCCADC: c_uint = 0x63;
pub const ATC2609A_PMU_CHGIADC: c_uint = 0x64;
pub const ATC2609A_PMU_IREFADC: c_uint = 0x65;
pub const ATC2609A_PMU_BAKBATADC: c_uint = 0x66;
pub const ATC2609A_PMU_ICTEMPADC: c_uint = 0x67;
pub const ATC2609A_PMU_AUXADC0: c_uint = 0x68;
pub const ATC2609A_PMU_AUXADC1: c_uint = 0x69;
pub const ATC2609A_PMU_AUXADC2: c_uint = 0x6A;
pub const ATC2609A_PMU_AUXADC3: c_uint = 0x6B;
pub const ATC2609A_PMU_ICTEMPADC_ADJ: c_uint = 0x6C;
pub const ATC2609A_PMU_BDG_CTL: c_uint = 0x6D;
pub const ATC2609A_RTC_CTL: c_uint = 0x6E;
pub const ATC2609A_RTC_MSALM: c_uint = 0x6F;
pub const ATC2609A_RTC_HALM: c_uint = 0x70;
pub const ATC2609A_RTC_YMDALM: c_uint = 0x71;
pub const ATC2609A_RTC_MS: c_uint = 0x72;
pub const ATC2609A_RTC_H: c_uint = 0x73;
pub const ATC2609A_RTC_DC: c_uint = 0x74;
pub const ATC2609A_RTC_YMD: c_uint = 0x75;
pub const ATC2609A_EFUSE_DAT: c_uint = 0x76;
pub const ATC2609A_EFUSECRTL1: c_uint = 0x77;
pub const ATC2609A_EFUSECRTL2: c_uint = 0x78;
pub const ATC2609A_PMU_DC4_CTL0: c_uint = 0x79;
pub const ATC2609A_PMU_DC4_CTL1: c_uint = 0x7A;
pub const ATC2609A_PMU_DC4_CTL2: c_uint = 0x7B;
pub const ATC2609A_PMU_DC4_CTL3: c_uint = 0x7C;
pub const ATC2609A_PMU_DC4_CTL4: c_uint = 0x7D;
pub const ATC2609A_PMU_DC4_CTL5: c_uint = 0x7E;
pub const ATC2609A_PMU_DC4_CTL6: c_uint = 0x7F;
pub const ATC2609A_PMU_PWR_STATUS: c_uint = 0x80;
pub const ATC2609A_PMU_S2_PWR: c_uint = 0x81;
pub const ATC2609A_CLMT_CTL0: c_uint = 0x82;
pub const ATC2609A_CLMT_DATA0: c_uint = 0x83;
pub const ATC2609A_CLMT_DATA1: c_uint = 0x84;
pub const ATC2609A_CLMT_DATA2: c_uint = 0x85;
pub const ATC2609A_CLMT_DATA3: c_uint = 0x86;
pub const ATC2609A_CLMT_ADD0: c_uint = 0x87;
pub const ATC2609A_CLMT_ADD1: c_uint = 0x88;
pub const ATC2609A_CLMT_OCV_TABLE: c_uint = 0x89;
pub const ATC2609A_CLMT_R_TABLE: c_uint = 0x8A;
pub const ATC2609A_PMU_PWRON_CTL0: c_uint = 0x8D;
pub const ATC2609A_PMU_PWRON_CTL1: c_uint = 0x8E;
pub const ATC2609A_PMU_PWRON_CTL2: c_uint = 0x8F;
pub const ATC2609A_IRC_CTL: c_uint = 0x90;
pub const ATC2609A_IRC_STAT: c_uint = 0x91;
pub const ATC2609A_IRC_CC: c_uint = 0x92;
pub const ATC2609A_IRC_KDC: c_uint = 0x93;
pub const ATC2609A_IRC_WK: c_uint = 0x94;
pub const ATC2609A_IRC_RCC: c_uint = 0x95;
// AUDIO_OUT Registers
pub const ATC2609A_AUDIOINOUT_CTL: c_uint = 0xA0;
pub const ATC2609A_AUDIO_DEBUGOUTCTL: c_uint = 0xA1;
pub const ATC2609A_DAC_DIGITALCTL: c_uint = 0xA2;
pub const ATC2609A_DAC_VOLUMECTL0: c_uint = 0xA3;
pub const ATC2609A_DAC_ANALOG0: c_uint = 0xA4;
pub const ATC2609A_DAC_ANALOG1: c_uint = 0xA5;
pub const ATC2609A_DAC_ANALOG2: c_uint = 0xA6;
pub const ATC2609A_DAC_ANALOG3: c_uint = 0xA7;
// AUDIO_IN Registers
pub const ATC2609A_ADC_DIGITALCTL: c_uint = 0xA8;
pub const ATC2609A_ADC_HPFCTL: c_uint = 0xA9;
pub const ATC2609A_ADC_CTL: c_uint = 0xAA;
pub const ATC2609A_AGC_CTL0: c_uint = 0xAB;
pub const ATC2609A_AGC_CTL1: c_uint = 0xAC;
pub const ATC2609A_AGC_CTL2: c_uint = 0xAD;
pub const ATC2609A_ADC_ANALOG0: c_uint = 0xAE;
pub const ATC2609A_ADC_ANALOG1: c_uint = 0xAF;
// PCM_IF Registers
pub const ATC2609A_PCM0_CTL: c_uint = 0xB0;
pub const ATC2609A_PCM1_CTL: c_uint = 0xB1;
pub const ATC2609A_PCM2_CTL: c_uint = 0xB2;
pub const ATC2609A_PCMIF_CTL: c_uint = 0xB3;
// CMU_CONTROL Registers
pub const ATC2609A_CMU_DEVRST: c_uint = 0xC1;
// INTS Registers
pub const ATC2609A_INTS_PD: c_uint = 0xC8;
pub const ATC2609A_INTS_MSK: c_uint = 0xC9;
// MFP Registers
pub const ATC2609A_MFP_CTL: c_uint = 0xD0;
pub const ATC2609A_PAD_VSEL: c_uint = 0xD1;
pub const ATC2609A_GPIO_OUTEN: c_uint = 0xD2;
pub const ATC2609A_GPIO_INEN: c_uint = 0xD3;
pub const ATC2609A_GPIO_DAT: c_uint = 0xD4;
pub const ATC2609A_PAD_DRV: c_uint = 0xD5;
pub const ATC2609A_PAD_EN: c_uint = 0xD6;
pub const ATC2609A_DEBUG_SEL: c_uint = 0xD7;
pub const ATC2609A_DEBUG_IE: c_uint = 0xD8;
pub const ATC2609A_DEBUG_OE: c_uint = 0xD9;
pub const ATC2609A_CHIP_VER: c_uint = 0xDC;
// PWSI Registers
pub const ATC2609A_PWSI_CTL: c_uint = 0xF0;
pub const ATC2609A_PWSI_STATUS: c_uint = 0xF1;
// TWSI Registers
pub const ATC2609A_SADDR: c_uint = 0xFF;
// PMU_SYS_CTL0 Register Mask Bits

// PMU_SYS_CTL1 Register Mask Bits

// PMU_SYS_CTL2 Register Mask Bits

// PMU_SYS_CTL3 Register Mask Bits

// PMU_SYS_CTL5 Register Mask Bits

// INTS_MSK Register Mask Bits

// CMU_DEVRST Register Mask Bits

// PAD_EN Register Mask Bits

