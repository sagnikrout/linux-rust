//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320aic3x.h
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
// ALSA SoC TLV320AIC3X codec driver
//
// Author:      Vladimir Barinov, <vbarinov@embeddedalley.com>
// Copyright:   (C) 2007 MontaVista Software, Inc., <source@mvista.com>
//
extern "C" {
    pub fn aic3x_probe(dev: *mut device, regmap: *mut regmap, driver_data: kernel_ulong_t) -> c_int;
}
extern "C" {
    pub fn aic3x_remove(dev: *mut device);
}
pub const AIC3X_MODEL_3X: c_int = 0;
pub const AIC3X_MODEL_33: c_int = 1;
pub const AIC3X_MODEL_3007: c_int = 2;
pub const AIC3X_MODEL_3104: c_int = 3;
pub const AIC3X_MODEL_3106: c_int = 4;
// AIC3X register space
pub const AIC3X_CACHEREGNUM: c_int = 110;
// Page select register
pub const AIC3X_PAGE_SELECT: c_int = 0;
// Software reset register
pub const AIC3X_RESET: c_int = 1;
// Codec Sample rate select register
pub const AIC3X_SAMPLE_RATE_SEL_REG: c_int = 2;
// PLL progrramming register A
pub const AIC3X_PLL_PROGA_REG: c_int = 3;
// PLL progrramming register B
pub const AIC3X_PLL_PROGB_REG: c_int = 4;
// PLL progrramming register C
pub const AIC3X_PLL_PROGC_REG: c_int = 5;
// PLL progrramming register D
pub const AIC3X_PLL_PROGD_REG: c_int = 6;
// Codec datapath setup register
pub const AIC3X_CODEC_DATAPATH_REG: c_int = 7;
// Audio serial data interface control register A
pub const AIC3X_ASD_INTF_CTRLA: c_int = 8;
// Audio serial data interface control register B
pub const AIC3X_ASD_INTF_CTRLB: c_int = 9;
// Audio serial data interface control register C
pub const AIC3X_ASD_INTF_CTRLC: c_int = 10;
// Audio overflow status and PLL R value programming register
pub const AIC3X_OVRF_STATUS_AND_PLLR_REG: c_int = 11;
// Audio codec digital filter control register
pub const AIC3X_CODEC_DFILT_CTRL: c_int = 12;
// Headset/button press detection register
pub const AIC3X_HEADSET_DETECT_CTRL_A: c_int = 13;
pub const AIC3X_HEADSET_DETECT_CTRL_B: c_int = 14;
// ADC PGA Gain control registers
pub const LADC_VOL: c_int = 15;
pub const RADC_VOL: c_int = 16;
// MIC3 control registers
pub const MIC3LR_2_LADC_CTRL: c_int = 17;
pub const MIC3LR_2_RADC_CTRL: c_int = 18;
// Line1 Input control registers
pub const LINE1L_2_LADC_CTRL: c_int = 19;
pub const LINE1R_2_LADC_CTRL: c_int = 21;
pub const LINE1R_2_RADC_CTRL: c_int = 22;
pub const LINE1L_2_RADC_CTRL: c_int = 24;
// Line2 Input control registers
pub const LINE2L_2_LADC_CTRL: c_int = 20;
pub const LINE2R_2_RADC_CTRL: c_int = 23;
// MICBIAS Control Register
pub const MICBIAS_CTRL: c_int = 25;
// AGC Control Registers A, B, C
pub const LAGC_CTRL_A: c_int = 26;
pub const LAGC_CTRL_B: c_int = 27;
pub const LAGC_CTRL_C: c_int = 28;
pub const RAGC_CTRL_A: c_int = 29;
pub const RAGC_CTRL_B: c_int = 30;
pub const RAGC_CTRL_C: c_int = 31;
// DAC Power and Left High Power Output control registers
pub const DAC_PWR: c_int = 37;
pub const HPLCOM_CFG: c_int = 37;
// Right High Power Output control registers
pub const HPRCOM_CFG: c_int = 38;
// High Power Output Stage Control Register
pub const HPOUT_SC: c_int = 40;
// DAC Output Switching control registers
pub const DAC_LINE_MUX: c_int = 41;
// High Power Output Driver Pop Reduction registers
pub const HPOUT_POP_REDUCTION: c_int = 42;
// DAC Digital control registers
pub const LDAC_VOL: c_int = 43;
pub const RDAC_VOL: c_int = 44;
// Left High Power Output control registers
pub const LINE2L_2_HPLOUT_VOL: c_int = 45;
pub const PGAL_2_HPLOUT_VOL: c_int = 46;
pub const DACL1_2_HPLOUT_VOL: c_int = 47;
pub const LINE2R_2_HPLOUT_VOL: c_int = 48;
pub const PGAR_2_HPLOUT_VOL: c_int = 49;
pub const DACR1_2_HPLOUT_VOL: c_int = 50;
pub const HPLOUT_CTRL: c_int = 51;
// Left High Power COM control registers
pub const LINE2L_2_HPLCOM_VOL: c_int = 52;
pub const PGAL_2_HPLCOM_VOL: c_int = 53;
pub const DACL1_2_HPLCOM_VOL: c_int = 54;
pub const LINE2R_2_HPLCOM_VOL: c_int = 55;
pub const PGAR_2_HPLCOM_VOL: c_int = 56;
pub const DACR1_2_HPLCOM_VOL: c_int = 57;
pub const HPLCOM_CTRL: c_int = 58;
// Right High Power Output control registers
pub const LINE2L_2_HPROUT_VOL: c_int = 59;
pub const PGAL_2_HPROUT_VOL: c_int = 60;
pub const DACL1_2_HPROUT_VOL: c_int = 61;
pub const LINE2R_2_HPROUT_VOL: c_int = 62;
pub const PGAR_2_HPROUT_VOL: c_int = 63;
pub const DACR1_2_HPROUT_VOL: c_int = 64;
pub const HPROUT_CTRL: c_int = 65;
// Right High Power COM control registers
pub const LINE2L_2_HPRCOM_VOL: c_int = 66;
pub const PGAL_2_HPRCOM_VOL: c_int = 67;
pub const DACL1_2_HPRCOM_VOL: c_int = 68;
pub const LINE2R_2_HPRCOM_VOL: c_int = 69;
pub const PGAR_2_HPRCOM_VOL: c_int = 70;
pub const DACR1_2_HPRCOM_VOL: c_int = 71;
pub const HPRCOM_CTRL: c_int = 72;
// Mono Line Output Plus/Minus control registers
pub const LINE2L_2_MONOLOPM_VOL: c_int = 73;
pub const PGAL_2_MONOLOPM_VOL: c_int = 74;
pub const DACL1_2_MONOLOPM_VOL: c_int = 75;
pub const LINE2R_2_MONOLOPM_VOL: c_int = 76;
pub const PGAR_2_MONOLOPM_VOL: c_int = 77;
pub const DACR1_2_MONOLOPM_VOL: c_int = 78;
pub const MONOLOPM_CTRL: c_int = 79;
// Class-D speaker driver on tlv320aic3007
pub const CLASSD_CTRL: c_int = 73;
// Left Line Output Plus/Minus control registers
pub const LINE2L_2_LLOPM_VOL: c_int = 80;
pub const PGAL_2_LLOPM_VOL: c_int = 81;
pub const DACL1_2_LLOPM_VOL: c_int = 82;
pub const LINE2R_2_LLOPM_VOL: c_int = 83;
pub const PGAR_2_LLOPM_VOL: c_int = 84;
pub const DACR1_2_LLOPM_VOL: c_int = 85;
pub const LLOPM_CTRL: c_int = 86;
// Right Line Output Plus/Minus control registers
pub const LINE2L_2_RLOPM_VOL: c_int = 87;
pub const PGAL_2_RLOPM_VOL: c_int = 88;
pub const DACL1_2_RLOPM_VOL: c_int = 89;
pub const LINE2R_2_RLOPM_VOL: c_int = 90;
pub const PGAR_2_RLOPM_VOL: c_int = 91;
pub const DACR1_2_RLOPM_VOL: c_int = 92;
pub const RLOPM_CTRL: c_int = 93;
// GPIO/IRQ registers
pub const AIC3X_STICKY_IRQ_FLAGS_REG: c_int = 96;
pub const AIC3X_RT_IRQ_FLAGS_REG: c_int = 97;
pub const AIC3X_GPIO1_REG: c_int = 98;
pub const AIC3X_GPIO2_REG: c_int = 99;
pub const AIC3X_GPIOA_REG: c_int = 100;
pub const AIC3X_GPIOB_REG: c_int = 101;
// Clock generation control register
pub const AIC3X_CLKGEN_CTRL_REG: c_int = 102;
// New AGC registers
pub const LAGCN_ATTACK: c_int = 103;
pub const LAGCN_DECAY: c_int = 104;
pub const RAGCN_ATTACK: c_int = 105;
pub const RAGCN_DECAY: c_int = 106;
// New Programmable ADC Digital Path and I2C Bus Condition Register
pub const NEW_ADC_DIGITALPATH: c_int = 107;
// Passive Analog Signal Bypass Selection During Powerdown Register
pub const PASSIVE_BYPASS: c_int = 108;
// DAC Quiescent Current Adjustment Register
pub const DAC_ICC_ADJ: c_int = 109;
// Page select register bits
pub const PAGE0_SELECT: c_int = 0;
pub const PAGE1_SELECT: c_int = 1;
// Audio serial data interface control register A bits
pub const BIT_CLK_MASTER: c_uint = 0x80;
pub const WORD_CLK_MASTER: c_uint = 0x40;
pub const DOUT_TRISTATE: c_uint = 0x20;
// Codec Datapath setup register 7

// PLL registers bitfields
pub const PLLP_SHIFT: c_int = 0;
pub const PLLP_MASK: c_int = 7;
pub const PLLQ_SHIFT: c_int = 3;
pub const PLLR_SHIFT: c_int = 0;
pub const PLLJ_SHIFT: c_int = 2;
pub const PLLD_MSB_SHIFT: c_int = 0;
pub const PLLD_LSB_SHIFT: c_int = 2;
// Clock generation register bits
pub const CODEC_CLKIN_PLLDIV: c_int = 0;
pub const CODEC_CLKIN_CLKDIV: c_int = 1;
pub const PLL_CLKIN_SHIFT: c_int = 4;
pub const MCLK_SOURCE: c_uint = 0x0;
pub const PLL_CLKDIV_SHIFT: c_int = 0;
pub const PLLCLK_IN_MASK: c_uint = 0x30;
pub const PLLCLK_IN_SHIFT: c_int = 4;
pub const CLKDIV_IN_MASK: c_uint = 0xc0;
pub const CLKDIV_IN_SHIFT: c_int = 6;
// clock in source
pub const CLKIN_MCLK: c_int = 0;
pub const CLKIN_GPIO2: c_int = 1;
pub const CLKIN_BCLK: c_int = 2;
// Software reset register bits
pub const SOFT_RESET: c_uint = 0x80;
// PLL progrramming register A bits
pub const PLL_ENABLE: c_uint = 0x80;
// Route bits
pub const ROUTE_ON: c_uint = 0x80;
// Mute bits
pub const UNMUTE: c_uint = 0x08;
pub const MUTE_ON: c_uint = 0x80;
// Power bits
pub const LADC_PWR_ON: c_uint = 0x04;
pub const RADC_PWR_ON: c_uint = 0x04;
pub const LDAC_PWR_ON: c_uint = 0x80;
pub const RDAC_PWR_ON: c_uint = 0x40;
pub const HPLOUT_PWR_ON: c_uint = 0x01;
pub const HPROUT_PWR_ON: c_uint = 0x01;
pub const HPLCOM_PWR_ON: c_uint = 0x01;
pub const HPRCOM_PWR_ON: c_uint = 0x01;
pub const MONOLOPM_PWR_ON: c_uint = 0x01;
pub const LLOPM_PWR_ON: c_uint = 0x01;
pub const RLOPM_PWR_ON: c_uint = 0x01;

// Default output volume (inverted)

// Default input volume
pub const DEFAULT_GAIN: c_uint = 0x20;
// MICBIAS Control Register

// HPOUT_SC

pub const HPOUT_SC_OCMV_1_35V: c_int = 0;
pub const HPOUT_SC_OCMV_1_5V: c_int = 1;
pub const HPOUT_SC_OCMV_1_65V: c_int = 2;
pub const HPOUT_SC_OCMV_1_8V: c_int = 3;
// headset detection / button API
// The AIC3x supports detection of stereo headsets (GND + left + right signal)
// and cellular headsets (GND + speaker output + microphone input).
// It is recommended to enable MIC bias for this function to work properly.
// For more information, please refer to the datasheet.
pub const AIC3X_HEADSET_DETECT_ENABLED: c_uint = 0x80;
pub const AIC3X_HEADSET_DETECT_SHIFT: c_int = 5;
pub const AIC3X_HEADSET_DETECT_MASK: c_int = 3;
pub const AIC3X_HEADSET_DEBOUNCE_SHIFT: c_int = 2;
pub const AIC3X_HEADSET_DEBOUNCE_MASK: c_int = 7;
pub const AIC3X_BUTTON_DEBOUNCE_SHIFT: c_int = 0;
pub const AIC3X_BUTTON_DEBOUNCE_MASK: c_int = 3;
// GPIO API
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic3x_micbias_voltage {
    AIC3X_MICBIAS_OFF = 0,
    AIC3X_MICBIAS_2_0V = 1,
    AIC3X_MICBIAS_2_5V = 2,
    AIC3X_MICBIAS_AVDDV = 3,
}
