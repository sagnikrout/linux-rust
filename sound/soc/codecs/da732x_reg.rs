//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/da732x_reg.h
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
// da732x_reg.h --- Dialog DA732X ALSA SoC Audio Registers Header File
//
// Copyright (C) 2012 Dialog Semiconductor GmbH
//
// Author: Michal Hajduk <Michal.Hajduk@diasemi.com>
//
// DA732X registers
pub const DA732X_REG_STATUS_EXT: c_uint = 0x00;
pub const DA732X_REG_STATUS: c_uint = 0x01;
pub const DA732X_REG_REF1: c_uint = 0x02;
pub const DA732X_REG_BIAS_EN: c_uint = 0x03;
pub const DA732X_REG_BIAS1: c_uint = 0x04;
pub const DA732X_REG_BIAS2: c_uint = 0x05;
pub const DA732X_REG_BIAS3: c_uint = 0x06;
pub const DA732X_REG_BIAS4: c_uint = 0x07;
pub const DA732X_REG_MICBIAS2: c_uint = 0x0F;
pub const DA732X_REG_MICBIAS1: c_uint = 0x10;
pub const DA732X_REG_MICDET: c_uint = 0x11;
pub const DA732X_REG_MIC1_PRE: c_uint = 0x12;
pub const DA732X_REG_MIC1: c_uint = 0x13;
pub const DA732X_REG_MIC2_PRE: c_uint = 0x14;
pub const DA732X_REG_MIC2: c_uint = 0x15;
pub const DA732X_REG_AUX1L: c_uint = 0x16;
pub const DA732X_REG_AUX1R: c_uint = 0x17;
pub const DA732X_REG_MIC3_PRE: c_uint = 0x18;
pub const DA732X_REG_MIC3: c_uint = 0x19;
pub const DA732X_REG_INP_PINBIAS: c_uint = 0x1A;
pub const DA732X_REG_INP_ZC_EN: c_uint = 0x1B;
pub const DA732X_REG_INP_MUX: c_uint = 0x1D;
pub const DA732X_REG_HP_DET: c_uint = 0x20;
pub const DA732X_REG_HPL_DAC_OFFSET: c_uint = 0x21;
pub const DA732X_REG_HPL_DAC_OFF_CNTL: c_uint = 0x22;
pub const DA732X_REG_HPL_OUT_OFFSET: c_uint = 0x23;
pub const DA732X_REG_HPL: c_uint = 0x24;
pub const DA732X_REG_HPL_VOL: c_uint = 0x25;
pub const DA732X_REG_HPR_DAC_OFFSET: c_uint = 0x26;
pub const DA732X_REG_HPR_DAC_OFF_CNTL: c_uint = 0x27;
pub const DA732X_REG_HPR_OUT_OFFSET: c_uint = 0x28;
pub const DA732X_REG_HPR: c_uint = 0x29;
pub const DA732X_REG_HPR_VOL: c_uint = 0x2A;
pub const DA732X_REG_LIN2: c_uint = 0x2B;
pub const DA732X_REG_LIN3: c_uint = 0x2C;
pub const DA732X_REG_LIN4: c_uint = 0x2D;
pub const DA732X_REG_OUT_ZC_EN: c_uint = 0x2E;
pub const DA732X_REG_HP_LIN1_GNDSEL: c_uint = 0x37;
pub const DA732X_REG_CP_HP1: c_uint = 0x3A;
pub const DA732X_REG_CP_HP2: c_uint = 0x3B;
pub const DA732X_REG_CP_CTRL1: c_uint = 0x40;
pub const DA732X_REG_CP_CTRL2: c_uint = 0x41;
pub const DA732X_REG_CP_CTRL3: c_uint = 0x42;
pub const DA732X_REG_CP_LEVEL_MASK: c_uint = 0x43;
pub const DA732X_REG_CP_DET: c_uint = 0x44;
pub const DA732X_REG_CP_STATUS: c_uint = 0x45;
pub const DA732X_REG_CP_THRESH1: c_uint = 0x46;
pub const DA732X_REG_CP_THRESH2: c_uint = 0x47;
pub const DA732X_REG_CP_THRESH3: c_uint = 0x48;
pub const DA732X_REG_CP_THRESH4: c_uint = 0x49;
pub const DA732X_REG_CP_THRESH5: c_uint = 0x4A;
pub const DA732X_REG_CP_THRESH6: c_uint = 0x4B;
pub const DA732X_REG_CP_THRESH7: c_uint = 0x4C;
pub const DA732X_REG_CP_THRESH8: c_uint = 0x4D;
pub const DA732X_REG_PLL_DIV_LO: c_uint = 0x50;
pub const DA732X_REG_PLL_DIV_MID: c_uint = 0x51;
pub const DA732X_REG_PLL_DIV_HI: c_uint = 0x52;
pub const DA732X_REG_PLL_CTRL: c_uint = 0x53;
pub const DA732X_REG_CLK_CTRL: c_uint = 0x54;
pub const DA732X_REG_CLK_DSP: c_uint = 0x5A;
pub const DA732X_REG_CLK_EN1: c_uint = 0x5B;
pub const DA732X_REG_CLK_EN2: c_uint = 0x5C;
pub const DA732X_REG_CLK_EN3: c_uint = 0x5D;
pub const DA732X_REG_CLK_EN4: c_uint = 0x5E;
pub const DA732X_REG_CLK_EN5: c_uint = 0x5F;
pub const DA732X_REG_AIF_MCLK: c_uint = 0x60;
pub const DA732X_REG_AIFA1: c_uint = 0x61;
pub const DA732X_REG_AIFA2: c_uint = 0x62;
pub const DA732X_REG_AIFA3: c_uint = 0x63;
pub const DA732X_REG_AIFB1: c_uint = 0x64;
pub const DA732X_REG_AIFB2: c_uint = 0x65;
pub const DA732X_REG_AIFB3: c_uint = 0x66;
pub const DA732X_REG_PC_CTRL: c_uint = 0x6A;
pub const DA732X_REG_DATA_ROUTE: c_uint = 0x70;
pub const DA732X_REG_DSP_CTRL: c_uint = 0x71;
pub const DA732X_REG_CIF_CTRL2: c_uint = 0x74;
pub const DA732X_REG_HANDSHAKE: c_uint = 0x75;
pub const DA732X_REG_MBOX0: c_uint = 0x76;
pub const DA732X_REG_MBOX1: c_uint = 0x77;
pub const DA732X_REG_MBOX2: c_uint = 0x78;
pub const DA732X_REG_MBOX_STATUS: c_uint = 0x79;
pub const DA732X_REG_SPARE1_OUT: c_uint = 0x7D;
pub const DA732X_REG_SPARE2_OUT: c_uint = 0x7E;
pub const DA732X_REG_SPARE1_IN: c_uint = 0x7F;
pub const DA732X_REG_ID: c_uint = 0x81;
pub const DA732X_REG_ADC1_PD: c_uint = 0x90;
pub const DA732X_REG_ADC1_HPF: c_uint = 0x93;
pub const DA732X_REG_ADC1_SEL: c_uint = 0x94;
pub const DA732X_REG_ADC1_EQ12: c_uint = 0x95;
pub const DA732X_REG_ADC1_EQ34: c_uint = 0x96;
pub const DA732X_REG_ADC1_EQ5: c_uint = 0x97;
pub const DA732X_REG_ADC2_PD: c_uint = 0x98;
pub const DA732X_REG_ADC2_HPF: c_uint = 0x9B;
pub const DA732X_REG_ADC2_SEL: c_uint = 0x9C;
pub const DA732X_REG_ADC2_EQ12: c_uint = 0x9D;
pub const DA732X_REG_ADC2_EQ34: c_uint = 0x9E;
pub const DA732X_REG_ADC2_EQ5: c_uint = 0x9F;
pub const DA732X_REG_DAC1_HPF: c_uint = 0xA0;
pub const DA732X_REG_DAC1_L_VOL: c_uint = 0xA1;
pub const DA732X_REG_DAC1_R_VOL: c_uint = 0xA2;
pub const DA732X_REG_DAC1_SEL: c_uint = 0xA3;
pub const DA732X_REG_DAC1_SOFTMUTE: c_uint = 0xA4;
pub const DA732X_REG_DAC1_EQ12: c_uint = 0xA5;
pub const DA732X_REG_DAC1_EQ34: c_uint = 0xA6;
pub const DA732X_REG_DAC1_EQ5: c_uint = 0xA7;
pub const DA732X_REG_DAC2_HPF: c_uint = 0xB0;
pub const DA732X_REG_DAC2_L_VOL: c_uint = 0xB1;
pub const DA732X_REG_DAC2_R_VOL: c_uint = 0xB2;
pub const DA732X_REG_DAC2_SEL: c_uint = 0xB3;
pub const DA732X_REG_DAC2_SOFTMUTE: c_uint = 0xB4;
pub const DA732X_REG_DAC2_EQ12: c_uint = 0xB5;
pub const DA732X_REG_DAC2_EQ34: c_uint = 0xB6;
pub const DA732X_REG_DAC2_EQ5: c_uint = 0xB7;
pub const DA732X_REG_DAC3_HPF: c_uint = 0xC0;
pub const DA732X_REG_DAC3_VOL: c_uint = 0xC1;
pub const DA732X_REG_DAC3_SEL: c_uint = 0xC3;
pub const DA732X_REG_DAC3_SOFTMUTE: c_uint = 0xC4;
pub const DA732X_REG_DAC3_EQ12: c_uint = 0xC5;
pub const DA732X_REG_DAC3_EQ34: c_uint = 0xC6;
pub const DA732X_REG_DAC3_EQ5: c_uint = 0xC7;
pub const DA732X_REG_BIQ_BYP: c_uint = 0xD2;
pub const DA732X_REG_DMA_CMD: c_uint = 0xD3;
pub const DA732X_REG_DMA_ADDR0: c_uint = 0xD4;
pub const DA732X_REG_DMA_ADDR1: c_uint = 0xD5;
pub const DA732X_REG_DMA_DATA0: c_uint = 0xD6;
pub const DA732X_REG_DMA_DATA1: c_uint = 0xD7;
pub const DA732X_REG_DMA_DATA2: c_uint = 0xD8;
pub const DA732X_REG_DMA_DATA3: c_uint = 0xD9;
pub const DA732X_REG_DMA_STATUS: c_uint = 0xDA;
pub const DA732X_REG_BROWNOUT: c_uint = 0xDF;
pub const DA732X_REG_UNLOCK: c_uint = 0xE0;

//
// Bits
//
// DA732X_REG_STATUS_EXT (addr=0x00)

// DA732X_REG_STATUS	(addr=0x01)

// DA732X_REG_REF1	(addr=0x02)

// DA732X_REG_BIAS_EN	(addr=0x03)

// DA732X_REG_BIAS1	(addr=0x04)

// DA732X_REG_BIAS2	(addr=0x05)

// DA732X_REG_BIAS3	(addr=0x06)

// DA732X_REG_BIAS4	(addr=0x07)

// DA732X_REG_SIF_VDD_SEL	(addr=0x08)

// DA732X_REG_MICBIAS2/1	(addr=0x0F/0x10)

pub const DA732X_MICBIAS_EN_SHIFT: c_int = 7;
pub const DA732X_MICBIAS_VOLTAGE_SHIFT: c_int = 0;
pub const DA732X_MICBIAS_VOLTAGE_MAX: c_uint = 0x0B;
// DA732X_REG_MICDET	(addr=0x11)

// DA732X_REG_MIC1/2/3_PRE (addr=0x11/0x14/0x18)
pub const DA732X_MICBOOST_MASK: c_uint = 0x7;
pub const DA732X_MICBOOST_SHIFT: c_int = 0;
pub const DA732X_MICBOOST_MIN: c_uint = 0x1;

// DA732X_REG_MIC1/2/3	(addr=0x13/0x15/0x19)
pub const DA732X_MIC_VOL_SHIFT: c_int = 0;
pub const DA732X_MIC_VOL_VAL_MASK: c_uint = 0x1F;
pub const DA732X_MIC_MUTE_SHIFT: c_int = 6;
pub const DA732X_MIC_EN_SHIFT: c_int = 7;
pub const DA732X_MIC_VOL_VAL_MIN: c_uint = 0x7;

// DA732X_REG_AUX1L/R	(addr=0x16/0x17)
pub const DA732X_AUX_VOL_SHIFT: c_int = 0;
pub const DA732X_AUX_VOL_MASK: c_uint = 0x7;
pub const DA732X_AUX_MUTE_SHIFT: c_int = 6;
pub const DA732X_AUX_EN_SHIFT: c_int = 7;

// DA732X_REG_INP_PINBIAS	(addr=0x1A)

// DA732X_REG_INP_ZC_EN	(addr=0x1B)

// DA732X_REG_INP_MUX	(addr=0x1D)

pub const DA732X_ADC1L_MUX_SEL_SHIFT: c_int = 0;
pub const DA732X_ADC1R_MUX_SEL_SHIFT: c_int = 2;
pub const DA732X_ADC2L_MUX_SEL_SHIFT: c_int = 4;
pub const DA732X_ADC2R_MUX_SEL_SHIFT: c_int = 6;
// DA732X_REG_HP_DET		(addr=0x20)

// DA732X_REG_HPL_DAC_OFFSET	(addr=0x21/0x26)

// DA732X_REG_HPL_DAC_OFF_CNTL	(addr=0x22/0x27)

pub const DA732X_HP_DAC_OFF_MASK: c_uint = 0x7F;
pub const DA732X_HP_DAC_COMPO_SHIFT: c_int = 3;
// DA732X_REG_HPL_OUT_OFFSET	(addr=0x23/0x28)

pub const DA732X_HP_DAC_OFFSET_TRIM_VAL: c_uint = 0x7F;
// DA732X_REG_HPL/R	(addr=0x24/0x29)

pub const DA732X_HP_OUT_COMPO_SHIFT: c_int = 3;
pub const DA732X_HP_OUT_DAC_EN_SHIFT: c_int = 4;
pub const DA732X_HP_HIZ_SHIFT: c_int = 5;
pub const DA732X_HP_MUTE_SHIFT: c_int = 6;
pub const DA732X_HP_OUT_EN_SHIFT: c_int = 7;

// DA732X_REG_HPL/R_VOL	(addr=0x25/0x2A)
pub const DA732X_HP_VOL_VAL_MASK: c_uint = 0xF;
pub const DA732X_HP_VOL_SHIFT: c_int = 0;

// DA732X_REG_LIN2/3/4	(addr=0x2B/0x2C/0x2D)
pub const DA732X_LOUT_VOL_SHIFT: c_int = 0;
pub const DA732X_LOUT_VOL_MASK: c_uint = 0x0F;

pub const DA732X_LOUT_DAC_EN_SHIFT: c_int = 4;
pub const DA732X_LOUT_MUTE_SHIFT: c_int = 6;
pub const DA732X_LIN_OUT_EN_SHIFT: c_int = 7;

// DA732X_REG_OUT_ZC_EN		(addr=0x2E)
pub const DA732X_HPL_ZC_EN_SHIFT: c_int = 0;
pub const DA732X_HPR_ZC_EN_SHIFT: c_int = 1;

// DA732X_REG_HP_LIN1_GNDSEL (addr=0x37)

// DA732X_REG_CP_HP2 (addr=0x3a)

// DA732X_REG_CP_CTRL1 (addr=0x40)

// DA732X_REG_CP_CTRL2 (addr=0x41)

// DA732X_REG_CP_CTRL3 (addr=0x42)

// DA732X_REG_PLL_CTRL (addr=0x53)

// DA732X_REG_CLK_CTRL (addr=0x54)

// DA732X_REG_CLK_DSP (addr=0x5A)

// DA732X_REG_CLK_EN1 (addr=0x5B)

// DA732X_REG_CLK_EN2 (addr=0x5C)

// DA732X_REG_CLK_EN3 (addr=0x5D)

// DA732X_REG_CLK_EN4 (addr=0x5E)

pub const DA732X_DACA_BB_CLK_SHIFT: c_int = 0;
pub const DA732X_DACC_BB_CLK_SHIFT: c_int = 4;
// DA732X_REG_CLK_EN5 (addr=0x5F)

pub const DA732X_DACE_BB_CLK_SHIFT: c_int = 0;
// DA732X_REG_AIF_MCLK (addr=0x60)

pub const DA732X_NO_CLK_GENERATION: c_uint = 0x0;
// DA732X_REG_AIFA1 (addr=0x61)

// DA732X_REG_AIFA3 (addr=0x63)
pub const DA732X_AIF_MODE_SHIFT: c_int = 0;
pub const DA732X_AIF_MODE_MASK: c_uint = 0x3;

pub const DA732X_AIF_EN_SHIFT: c_int = 7;
// DA732X_REG_PC_CTRL (addr=0x6a)

// DA732X_REG_DATA_ROUTE (addr=0x70)

// DA732X_REG_DSP_CTRL (addr=0x71)

// DA732X_REG_SPARE1_OUT (addr=0x7D)

// DA732X_REG_ID (addr=0x81)

// DA732X_REG_ADC1/2_PD (addr=0x90/0x98)

// DA732X_REG_ADC1/2_SEL (addr=0x94/0x9C)
pub const DA732X_ADC_VOL_VAL_MASK: c_uint = 0x7;
pub const DA732X_ADCL_VOL_SHIFT: c_int = 0;
pub const DA732X_ADCR_VOL_SHIFT: c_int = 4;
pub const DA732X_ADCL_EN_SHIFT: c_int = 2;
pub const DA732X_ADCR_EN_SHIFT: c_int = 3;

//
// DA732X_REG_ADC1/2_HPF (addr=0x93/0x9b)
// DA732x_REG_DAC1/2/3_HPG	(addr=0xA5/0xB5/0xC5)
//

// DA732X_REG_DAC1/2/3_VOL
pub const DA732X_DAC_VOL_VAL_MASK: c_uint = 0x7F;
pub const DA732X_DAC_VOL_SHIFT: c_int = 0;

// DA732X_REG_DAC1/2/3_SEL (addr=0xA3/0xB3/0xC3)
pub const DA732X_DACL_EN_SHIFT: c_int = 3;
pub const DA732X_DACR_EN_SHIFT: c_int = 7;
pub const DA732X_DACL_MUTE_SHIFT: c_int = 2;
pub const DA732X_DACR_MUTE_SHIFT: c_int = 6;

// DA732X_REG_DAC_SOFTMUTE (addr=0xA4/0xB4/0xC4)

pub const DA732X_SOFTMUTE_SHIFT: c_int = 7;
//
// DA732x_REG_ADC1/2_EQ12	(addr=0x95/0x9D)
// DA732x_REG_ADC1/2_EQ34	(addr=0x96/0x9E)
// DA732x_REG_ADC1/2_EQ5	(addr=0x97/0x9F)
// DA732x_REG_DAC1/2/3_EQ12	(addr=0xA5/0xB5/0xC5)
// DA732x_REG_DAC1/2/3_EQ34	(addr=0xA6/0xB6/0xC6)
// DA732x_REG_DAC1/2/3_EQ5	(addr=0xA7/0xB7/0xB7)
//
pub const DA732X_EQ_VOL_VAL_MASK: c_uint = 0xF;
pub const DA732X_EQ_BAND1_SHIFT: c_int = 0;
pub const DA732X_EQ_BAND2_SHIFT: c_int = 4;
pub const DA732X_EQ_BAND3_SHIFT: c_int = 0;
pub const DA732X_EQ_BAND4_SHIFT: c_int = 4;
pub const DA732X_EQ_BAND5_SHIFT: c_int = 0;
pub const DA732X_EQ_OVERALL_SHIFT: c_int = 4;
pub const DA732X_EQ_OVERALL_VOL_VAL_MASK: c_uint = 0x3;

pub const DA732X_EQ_EN_SHIFT: c_int = 7;

// DA732X_REG_DMA_CMD (addr=0xD3)

// DA732X_REG_DMA_STATUS (addr=0xDA)

