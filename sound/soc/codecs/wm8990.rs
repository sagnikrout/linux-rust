//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8990.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// wm8990.h  --  audio driver for WM8990
//
// Copyright 2007 Wolfson Microelectronics PLC.
// Author: Graeme Gregory
// graeme.gregory@wolfsonmicro.com or linux@wolfsonmicro.com
//
// Register values.
//
pub const WM8990_RESET: c_uint = 0x00;
pub const WM8990_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const WM8990_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const WM8990_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const WM8990_AUDIO_INTERFACE_1: c_uint = 0x04;
pub const WM8990_AUDIO_INTERFACE_2: c_uint = 0x05;
pub const WM8990_CLOCKING_1: c_uint = 0x06;
pub const WM8990_CLOCKING_2: c_uint = 0x07;
pub const WM8990_AUDIO_INTERFACE_3: c_uint = 0x08;
pub const WM8990_AUDIO_INTERFACE_4: c_uint = 0x09;
pub const WM8990_DAC_CTRL: c_uint = 0x0A;
pub const WM8990_LEFT_DAC_DIGITAL_VOLUME: c_uint = 0x0B;
pub const WM8990_RIGHT_DAC_DIGITAL_VOLUME: c_uint = 0x0C;
pub const WM8990_DIGITAL_SIDE_TONE: c_uint = 0x0D;
pub const WM8990_ADC_CTRL: c_uint = 0x0E;
pub const WM8990_LEFT_ADC_DIGITAL_VOLUME: c_uint = 0x0F;
pub const WM8990_RIGHT_ADC_DIGITAL_VOLUME: c_uint = 0x10;
pub const WM8990_GPIO_CTRL_1: c_uint = 0x12;
pub const WM8990_GPIO1_GPIO2: c_uint = 0x13;
pub const WM8990_GPIO3_GPIO4: c_uint = 0x14;
pub const WM8990_GPIO5_GPIO6: c_uint = 0x15;
pub const WM8990_GPIOCTRL_2: c_uint = 0x16;
pub const WM8990_GPIO_POL: c_uint = 0x17;
pub const WM8990_LEFT_LINE_INPUT_1_2_VOLUME: c_uint = 0x18;
pub const WM8990_LEFT_LINE_INPUT_3_4_VOLUME: c_uint = 0x19;
pub const WM8990_RIGHT_LINE_INPUT_1_2_VOLUME: c_uint = 0x1A;
pub const WM8990_RIGHT_LINE_INPUT_3_4_VOLUME: c_uint = 0x1B;
pub const WM8990_LEFT_OUTPUT_VOLUME: c_uint = 0x1C;
pub const WM8990_RIGHT_OUTPUT_VOLUME: c_uint = 0x1D;
pub const WM8990_LINE_OUTPUTS_VOLUME: c_uint = 0x1E;
pub const WM8990_OUT3_4_VOLUME: c_uint = 0x1F;
pub const WM8990_LEFT_OPGA_VOLUME: c_uint = 0x20;
pub const WM8990_RIGHT_OPGA_VOLUME: c_uint = 0x21;
pub const WM8990_SPEAKER_VOLUME: c_uint = 0x22;
pub const WM8990_CLASSD1: c_uint = 0x23;
pub const WM8990_CLASSD3: c_uint = 0x25;
pub const WM8990_CLASSD4: c_uint = 0x26;
pub const WM8990_INPUT_MIXER1: c_uint = 0x27;
pub const WM8990_INPUT_MIXER2: c_uint = 0x28;
pub const WM8990_INPUT_MIXER3: c_uint = 0x29;
pub const WM8990_INPUT_MIXER4: c_uint = 0x2A;
pub const WM8990_INPUT_MIXER5: c_uint = 0x2B;
pub const WM8990_INPUT_MIXER6: c_uint = 0x2C;
pub const WM8990_OUTPUT_MIXER1: c_uint = 0x2D;
pub const WM8990_OUTPUT_MIXER2: c_uint = 0x2E;
pub const WM8990_OUTPUT_MIXER3: c_uint = 0x2F;
pub const WM8990_OUTPUT_MIXER4: c_uint = 0x30;
pub const WM8990_OUTPUT_MIXER5: c_uint = 0x31;
pub const WM8990_OUTPUT_MIXER6: c_uint = 0x32;
pub const WM8990_OUT3_4_MIXER: c_uint = 0x33;
pub const WM8990_LINE_MIXER1: c_uint = 0x34;
pub const WM8990_LINE_MIXER2: c_uint = 0x35;
pub const WM8990_SPEAKER_MIXER: c_uint = 0x36;
pub const WM8990_ADDITIONAL_CONTROL: c_uint = 0x37;
pub const WM8990_ANTIPOP1: c_uint = 0x38;
pub const WM8990_ANTIPOP2: c_uint = 0x39;
pub const WM8990_MICBIAS: c_uint = 0x3A;
pub const WM8990_PLL1: c_uint = 0x3C;
pub const WM8990_PLL2: c_uint = 0x3D;
pub const WM8990_PLL3: c_uint = 0x3E;
pub const WM8990_EXT_ACCESS_ENA: c_uint = 0x75;
pub const WM8990_EXT_CTL1: c_uint = 0x7a;
//
// Field Definitions.
//
// R0 (0x00) - Reset
//
pub const WM8990_SW_RESET_CHIP_ID_MASK: c_uint = 0xFFFF  /* SW_RESET_CHIP_ID */;
//
// R1 (0x01) - Power Management (1)
//
pub const WM8990_SPK_ENA: c_uint = 0x1000  /* SPK_ENA */;
pub const WM8990_SPK_ENA_BIT: c_int = 12;
pub const WM8990_OUT3_ENA: c_uint = 0x0800  /* OUT3_ENA */;
pub const WM8990_OUT3_ENA_BIT: c_int = 11;
pub const WM8990_OUT4_ENA: c_uint = 0x0400  /* OUT4_ENA */;
pub const WM8990_OUT4_ENA_BIT: c_int = 10;
pub const WM8990_LOUT_ENA: c_uint = 0x0200  /* LOUT_ENA */;
pub const WM8990_LOUT_ENA_BIT: c_int = 9;
pub const WM8990_ROUT_ENA: c_uint = 0x0100  /* ROUT_ENA */;
pub const WM8990_ROUT_ENA_BIT: c_int = 8;
pub const WM8990_MICBIAS_ENA: c_uint = 0x0010  /* MICBIAS_ENA */;
pub const WM8990_MICBIAS_ENA_BIT: c_int = 4;
pub const WM8990_VMID_MODE_MASK: c_uint = 0x0006  /* VMID_MODE - [2:1] */;
pub const WM8990_VREF_ENA: c_uint = 0x0001  /* VREF_ENA */;
pub const WM8990_VREF_ENA_BIT: c_int = 0;
//
// R2 (0x02) - Power Management (2)
//
pub const WM8990_PLL_ENA: c_uint = 0x8000  /* PLL_ENA */;
pub const WM8990_PLL_ENA_BIT: c_int = 15;
pub const WM8990_TSHUT_ENA: c_uint = 0x4000  /* TSHUT_ENA */;
pub const WM8990_TSHUT_ENA_BIT: c_int = 14;
pub const WM8990_TSHUT_OPDIS: c_uint = 0x2000  /* TSHUT_OPDIS */;
pub const WM8990_TSHUT_OPDIS_BIT: c_int = 13;
pub const WM8990_OPCLK_ENA: c_uint = 0x0800  /* OPCLK_ENA */;
pub const WM8990_OPCLK_ENA_BIT: c_int = 11;
pub const WM8990_AINL_ENA: c_uint = 0x0200  /* AINL_ENA */;
pub const WM8990_AINL_ENA_BIT: c_int = 9;
pub const WM8990_AINR_ENA: c_uint = 0x0100  /* AINR_ENA */;
pub const WM8990_AINR_ENA_BIT: c_int = 8;
pub const WM8990_LIN34_ENA: c_uint = 0x0080  /* LIN34_ENA */;
pub const WM8990_LIN34_ENA_BIT: c_int = 7;
pub const WM8990_LIN12_ENA: c_uint = 0x0040  /* LIN12_ENA */;
pub const WM8990_LIN12_ENA_BIT: c_int = 6;
pub const WM8990_RIN34_ENA: c_uint = 0x0020  /* RIN34_ENA */;
pub const WM8990_RIN34_ENA_BIT: c_int = 5;
pub const WM8990_RIN12_ENA: c_uint = 0x0010  /* RIN12_ENA */;
pub const WM8990_RIN12_ENA_BIT: c_int = 4;
pub const WM8990_ADCL_ENA: c_uint = 0x0002  /* ADCL_ENA */;
pub const WM8990_ADCL_ENA_BIT: c_int = 1;
pub const WM8990_ADCR_ENA: c_uint = 0x0001  /* ADCR_ENA */;
pub const WM8990_ADCR_ENA_BIT: c_int = 0;
//
// R3 (0x03) - Power Management (3)
//
pub const WM8990_LON_ENA: c_uint = 0x2000  /* LON_ENA */;
pub const WM8990_LON_ENA_BIT: c_int = 13;
pub const WM8990_LOP_ENA: c_uint = 0x1000  /* LOP_ENA */;
pub const WM8990_LOP_ENA_BIT: c_int = 12;
pub const WM8990_RON_ENA: c_uint = 0x0800  /* RON_ENA */;
pub const WM8990_RON_ENA_BIT: c_int = 11;
pub const WM8990_ROP_ENA: c_uint = 0x0400  /* ROP_ENA */;
pub const WM8990_ROP_ENA_BIT: c_int = 10;
pub const WM8990_LOPGA_ENA: c_uint = 0x0080  /* LOPGA_ENA */;
pub const WM8990_LOPGA_ENA_BIT: c_int = 7;
pub const WM8990_ROPGA_ENA: c_uint = 0x0040  /* ROPGA_ENA */;
pub const WM8990_ROPGA_ENA_BIT: c_int = 6;
pub const WM8990_LOMIX_ENA: c_uint = 0x0020  /* LOMIX_ENA */;
pub const WM8990_LOMIX_ENA_BIT: c_int = 5;
pub const WM8990_ROMIX_ENA: c_uint = 0x0010  /* ROMIX_ENA */;
pub const WM8990_ROMIX_ENA_BIT: c_int = 4;
pub const WM8990_DACL_ENA: c_uint = 0x0002  /* DACL_ENA */;
pub const WM8990_DACL_ENA_BIT: c_int = 1;
pub const WM8990_DACR_ENA: c_uint = 0x0001  /* DACR_ENA */;
pub const WM8990_DACR_ENA_BIT: c_int = 0;
//
// R4 (0x04) - Audio Interface (1)
//
pub const WM8990_AIFADCL_SRC: c_uint = 0x8000  /* AIFADCL_SRC */;
pub const WM8990_AIFADCR_SRC: c_uint = 0x4000  /* AIFADCR_SRC */;
pub const WM8990_AIFADC_TDM: c_uint = 0x2000  /* AIFADC_TDM */;
pub const WM8990_AIFADC_TDM_CHAN: c_uint = 0x1000  /* AIFADC_TDM_CHAN */;
pub const WM8990_AIF_BCLK_INV: c_uint = 0x0100  /* AIF_BCLK_INV */;
pub const WM8990_AIF_LRCLK_INV: c_uint = 0x0080  /* AIF_LRCLK_INV */;
pub const WM8990_AIF_WL_MASK: c_uint = 0x0060  /* AIF_WL - [6:5] */;

pub const WM8990_AIF_FMT_MASK: c_uint = 0x0018  /* AIF_FMT - [4:3] */;

//
// R5 (0x05) - Audio Interface (2)
//
pub const WM8990_DACL_SRC: c_uint = 0x8000  /* DACL_SRC */;
pub const WM8990_DACR_SRC: c_uint = 0x4000  /* DACR_SRC */;
pub const WM8990_AIFDAC_TDM: c_uint = 0x2000  /* AIFDAC_TDM */;
pub const WM8990_AIFDAC_TDM_CHAN: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;
pub const WM8990_DAC_BOOST_MASK: c_uint = 0x0C00  /* DAC_BOOST */;
pub const WM8990_DAC_COMP: c_uint = 0x0010  /* DAC_COMP */;
pub const WM8990_DAC_COMPMODE: c_uint = 0x0008  /* DAC_COMPMODE */;
pub const WM8990_ADC_COMP: c_uint = 0x0004  /* ADC_COMP */;
pub const WM8990_ADC_COMPMODE: c_uint = 0x0002  /* ADC_COMPMODE */;
pub const WM8990_LOOPBACK: c_uint = 0x0001  /* LOOPBACK */;
//
// R6 (0x06) - Clocking (1)
//
pub const WM8990_TOCLK_RATE: c_uint = 0x8000  /* TOCLK_RATE */;
pub const WM8990_TOCLK_ENA: c_uint = 0x4000  /* TOCLK_ENA */;
pub const WM8990_OPCLKDIV_MASK: c_uint = 0x1E00  /* OPCLKDIV - [12:9] */;
pub const WM8990_DCLKDIV_MASK: c_uint = 0x01C0  /* DCLKDIV - [8:6] */;
pub const WM8990_BCLK_DIV_MASK: c_uint = 0x001E  /* BCLK_DIV - [4:1] */;

//
// R7 (0x07) - Clocking (2)
//
pub const WM8990_MCLK_SRC: c_uint = 0x8000  /* MCLK_SRC */;
pub const WM8990_SYSCLK_SRC: c_uint = 0x4000  /* SYSCLK_SRC */;
pub const WM8990_CLK_FORCE: c_uint = 0x2000  /* CLK_FORCE */;
pub const WM8990_MCLK_DIV_MASK: c_uint = 0x1800  /* MCLK_DIV - [12:11] */;

pub const WM8990_MCLK_INV: c_uint = 0x0400  /* MCLK_INV */;
pub const WM8990_ADC_CLKDIV_MASK: c_uint = 0x00E0  /* ADC_CLKDIV */;

pub const WM8990_DAC_CLKDIV_MASK: c_uint = 0x001C  /* DAC_CLKDIV - [4:2] */;

//
// R8 (0x08) - Audio Interface (3)
//
pub const WM8990_AIF_MSTR1: c_uint = 0x8000  /* AIF_MSTR1 */;
pub const WM8990_AIF_MSTR2: c_uint = 0x4000  /* AIF_MSTR2 */;
pub const WM8990_AIF_SEL: c_uint = 0x2000  /* AIF_SEL */;
pub const WM8990_ADCLRC_DIR: c_uint = 0x0800  /* ADCLRC_DIR */;
pub const WM8990_ADCLRC_RATE_MASK: c_uint = 0x07FF  /* ADCLRC_RATE */;
//
// R9 (0x09) - Audio Interface (4)
//
pub const WM8990_ALRCGPIO1: c_uint = 0x8000  /* ALRCGPIO1 */;
pub const WM8990_ALRCBGPIO6: c_uint = 0x4000  /* ALRCBGPIO6 */;
pub const WM8990_AIF_TRIS: c_uint = 0x2000  /* AIF_TRIS */;
pub const WM8990_DACLRC_DIR: c_uint = 0x0800  /* DACLRC_DIR */;
pub const WM8990_DACLRC_RATE_MASK: c_uint = 0x07FF  /* DACLRC_RATE */;
//
// R10 (0x0A) - DAC CTRL
//
pub const WM8990_AIF_LRCLKRATE: c_uint = 0x0400  /* AIF_LRCLKRATE */;
pub const WM8990_DAC_MONO: c_uint = 0x0200  /* DAC_MONO */;
pub const WM8990_DAC_SB_FILT: c_uint = 0x0100  /* DAC_SB_FILT */;
pub const WM8990_DAC_MUTERATE: c_uint = 0x0080  /* DAC_MUTERATE */;
pub const WM8990_DAC_MUTEMODE: c_uint = 0x0040  /* DAC_MUTEMODE */;
pub const WM8990_DEEMP_MASK: c_uint = 0x0030  /* DEEMP - [5:4] */;
pub const WM8990_DAC_MUTE: c_uint = 0x0004  /* DAC_MUTE */;
pub const WM8990_DACL_DATINV: c_uint = 0x0002  /* DACL_DATINV */;
pub const WM8990_DACR_DATINV: c_uint = 0x0001  /* DACR_DATINV */;
//
// R11 (0x0B) - Left DAC Digital Volume
//
pub const WM8990_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8990_DACL_VOL_MASK: c_uint = 0x00FF  /* DACL_VOL - [7:0] */;
pub const WM8990_DACL_VOL_SHIFT: c_int = 0;
//
// R12 (0x0C) - Right DAC Digital Volume
//
pub const WM8990_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8990_DACR_VOL_MASK: c_uint = 0x00FF  /* DACR_VOL - [7:0] */;
pub const WM8990_DACR_VOL_SHIFT: c_int = 0;
//
// R13 (0x0D) - Digital Side Tone
//
pub const WM8990_ADCL_DAC_SVOL_MASK: c_uint = 0x0F  /* ADCL_DAC_SVOL */;
pub const WM8990_ADCL_DAC_SVOL_SHIFT: c_int = 9;
pub const WM8990_ADCR_DAC_SVOL_MASK: c_uint = 0x0F  /* ADCR_DAC_SVOL */;
pub const WM8990_ADCR_DAC_SVOL_SHIFT: c_int = 5;
pub const WM8990_ADC_TO_DACL_MASK: c_uint = 0x03  /* ADC_TO_DACL - [3:2] */;
pub const WM8990_ADC_TO_DACL_SHIFT: c_int = 2;
pub const WM8990_ADC_TO_DACR_MASK: c_uint = 0x03  /* ADC_TO_DACR - [1:0] */;
pub const WM8990_ADC_TO_DACR_SHIFT: c_int = 0;
//
// R14 (0x0E) - ADC CTRL
//
pub const WM8990_ADC_HPF_ENA: c_uint = 0x0100  /* ADC_HPF_ENA */;
pub const WM8990_ADC_HPF_ENA_BIT: c_int = 8;
pub const WM8990_ADC_HPF_CUT_MASK: c_uint = 0x03  /* ADC_HPF_CUT - [6:5] */;
pub const WM8990_ADC_HPF_CUT_SHIFT: c_int = 5;
pub const WM8990_ADCL_DATINV: c_uint = 0x0002  /* ADCL_DATINV */;
pub const WM8990_ADCL_DATINV_BIT: c_int = 1;
pub const WM8990_ADCR_DATINV: c_uint = 0x0001  /* ADCR_DATINV */;
pub const WM8990_ADCR_DATINV_BIT: c_int = 0;
//
// R15 (0x0F) - Left ADC Digital Volume
//
pub const WM8990_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8990_ADCL_VOL_MASK: c_uint = 0x00FF  /* ADCL_VOL - [7:0] */;
pub const WM8990_ADCL_VOL_SHIFT: c_int = 0;
//
// R16 (0x10) - Right ADC Digital Volume
//
pub const WM8990_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8990_ADCR_VOL_MASK: c_uint = 0x00FF  /* ADCR_VOL - [7:0] */;
pub const WM8990_ADCR_VOL_SHIFT: c_int = 0;
//
// R18 (0x12) - GPIO CTRL 1
//
pub const WM8990_IRQ: c_uint = 0x1000  /* IRQ */;
pub const WM8990_TEMPOK: c_uint = 0x0800  /* TEMPOK */;
pub const WM8990_MICSHRT: c_uint = 0x0400  /* MICSHRT */;
pub const WM8990_MICDET: c_uint = 0x0200  /* MICDET */;
pub const WM8990_PLL_LCK: c_uint = 0x0100  /* PLL_LCK */;
pub const WM8990_GPI8_STATUS: c_uint = 0x0080  /* GPI8_STATUS */;
pub const WM8990_GPI7_STATUS: c_uint = 0x0040  /* GPI7_STATUS */;
pub const WM8990_GPIO6_STATUS: c_uint = 0x0020  /* GPIO6_STATUS */;
pub const WM8990_GPIO5_STATUS: c_uint = 0x0010  /* GPIO5_STATUS */;
pub const WM8990_GPIO4_STATUS: c_uint = 0x0008  /* GPIO4_STATUS */;
pub const WM8990_GPIO3_STATUS: c_uint = 0x0004  /* GPIO3_STATUS */;
pub const WM8990_GPIO2_STATUS: c_uint = 0x0002  /* GPIO2_STATUS */;
pub const WM8990_GPIO1_STATUS: c_uint = 0x0001  /* GPIO1_STATUS */;
//
// R19 (0x13) - GPIO1 & GPIO2
//
pub const WM8990_GPIO2_DEB_ENA: c_uint = 0x8000  /* GPIO2_DEB_ENA */;
pub const WM8990_GPIO2_IRQ_ENA: c_uint = 0x4000  /* GPIO2_IRQ_ENA */;
pub const WM8990_GPIO2_PU: c_uint = 0x2000  /* GPIO2_PU */;
pub const WM8990_GPIO2_PD: c_uint = 0x1000  /* GPIO2_PD */;
pub const WM8990_GPIO2_SEL_MASK: c_uint = 0x0F00  /* GPIO2_SEL - [11:8] */;
pub const WM8990_GPIO1_DEB_ENA: c_uint = 0x0080  /* GPIO1_DEB_ENA */;
pub const WM8990_GPIO1_IRQ_ENA: c_uint = 0x0040  /* GPIO1_IRQ_ENA */;
pub const WM8990_GPIO1_PU: c_uint = 0x0020  /* GPIO1_PU */;
pub const WM8990_GPIO1_PD: c_uint = 0x0010  /* GPIO1_PD */;
pub const WM8990_GPIO1_SEL_MASK: c_uint = 0x000F  /* GPIO1_SEL - [3:0] */;
//
// R20 (0x14) - GPIO3 & GPIO4
//
pub const WM8990_GPIO4_DEB_ENA: c_uint = 0x8000  /* GPIO4_DEB_ENA */;
pub const WM8990_GPIO4_IRQ_ENA: c_uint = 0x4000  /* GPIO4_IRQ_ENA */;
pub const WM8990_GPIO4_PU: c_uint = 0x2000  /* GPIO4_PU */;
pub const WM8990_GPIO4_PD: c_uint = 0x1000  /* GPIO4_PD */;
pub const WM8990_GPIO4_SEL_MASK: c_uint = 0x0F00  /* GPIO4_SEL - [11:8] */;
pub const WM8990_GPIO3_DEB_ENA: c_uint = 0x0080  /* GPIO3_DEB_ENA */;
pub const WM8990_GPIO3_IRQ_ENA: c_uint = 0x0040  /* GPIO3_IRQ_ENA */;
pub const WM8990_GPIO3_PU: c_uint = 0x0020  /* GPIO3_PU */;
pub const WM8990_GPIO3_PD: c_uint = 0x0010  /* GPIO3_PD */;
pub const WM8990_GPIO3_SEL_MASK: c_uint = 0x000F  /* GPIO3_SEL - [3:0] */;
//
// R21 (0x15) - GPIO5 & GPIO6
//
pub const WM8990_GPIO6_DEB_ENA: c_uint = 0x8000  /* GPIO6_DEB_ENA */;
pub const WM8990_GPIO6_IRQ_ENA: c_uint = 0x4000  /* GPIO6_IRQ_ENA */;
pub const WM8990_GPIO6_PU: c_uint = 0x2000  /* GPIO6_PU */;
pub const WM8990_GPIO6_PD: c_uint = 0x1000  /* GPIO6_PD */;
pub const WM8990_GPIO6_SEL_MASK: c_uint = 0x0F00  /* GPIO6_SEL - [11:8] */;
pub const WM8990_GPIO5_DEB_ENA: c_uint = 0x0080  /* GPIO5_DEB_ENA */;
pub const WM8990_GPIO5_IRQ_ENA: c_uint = 0x0040  /* GPIO5_IRQ_ENA */;
pub const WM8990_GPIO5_PU: c_uint = 0x0020  /* GPIO5_PU */;
pub const WM8990_GPIO5_PD: c_uint = 0x0010  /* GPIO5_PD */;
pub const WM8990_GPIO5_SEL_MASK: c_uint = 0x000F  /* GPIO5_SEL - [3:0] */;
//
// R22 (0x16) - GPIOCTRL 2
//
pub const WM8990_RD_3W_ENA: c_uint = 0x8000  /* RD_3W_ENA */;
pub const WM8990_MODE_3W4W: c_uint = 0x4000  /* MODE_3W4W */;
pub const WM8990_TEMPOK_IRQ_ENA: c_uint = 0x0800  /* TEMPOK_IRQ_ENA */;
pub const WM8990_MICSHRT_IRQ_ENA: c_uint = 0x0400  /* MICSHRT_IRQ_ENA */;
pub const WM8990_MICDET_IRQ_ENA: c_uint = 0x0200  /* MICDET_IRQ_ENA */;
pub const WM8990_PLL_LCK_IRQ_ENA: c_uint = 0x0100  /* PLL_LCK_IRQ_ENA */;
pub const WM8990_GPI8_DEB_ENA: c_uint = 0x0080  /* GPI8_DEB_ENA */;
pub const WM8990_GPI8_IRQ_ENA: c_uint = 0x0040  /* GPI8_IRQ_ENA */;
pub const WM8990_GPI8_ENA: c_uint = 0x0010  /* GPI8_ENA */;
pub const WM8990_GPI7_DEB_ENA: c_uint = 0x0008  /* GPI7_DEB_ENA */;
pub const WM8990_GPI7_IRQ_ENA: c_uint = 0x0004  /* GPI7_IRQ_ENA */;
pub const WM8990_GPI7_ENA: c_uint = 0x0001  /* GPI7_ENA */;
//
// R23 (0x17) - GPIO_POL
//
pub const WM8990_IRQ_INV: c_uint = 0x1000  /* IRQ_INV */;
pub const WM8990_TEMPOK_POL: c_uint = 0x0800  /* TEMPOK_POL */;
pub const WM8990_MICSHRT_POL: c_uint = 0x0400  /* MICSHRT_POL */;
pub const WM8990_MICDET_POL: c_uint = 0x0200  /* MICDET_POL */;
pub const WM8990_PLL_LCK_POL: c_uint = 0x0100  /* PLL_LCK_POL */;
pub const WM8990_GPI8_POL: c_uint = 0x0080  /* GPI8_POL */;
pub const WM8990_GPI7_POL: c_uint = 0x0040  /* GPI7_POL */;
pub const WM8990_GPIO6_POL: c_uint = 0x0020  /* GPIO6_POL */;
pub const WM8990_GPIO5_POL: c_uint = 0x0010  /* GPIO5_POL */;
pub const WM8990_GPIO4_POL: c_uint = 0x0008  /* GPIO4_POL */;
pub const WM8990_GPIO3_POL: c_uint = 0x0004  /* GPIO3_POL */;
pub const WM8990_GPIO2_POL: c_uint = 0x0002  /* GPIO2_POL */;
pub const WM8990_GPIO1_POL: c_uint = 0x0001  /* GPIO1_POL */;
//
// R24 (0x18) - Left Line Input 1&2 Volume
//
pub const WM8990_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8990_LI12MUTE: c_uint = 0x0080  /* LI12MUTE */;
pub const WM8990_LI12MUTE_BIT: c_int = 7;
pub const WM8990_LI12ZC: c_uint = 0x0040  /* LI12ZC */;
pub const WM8990_LI12ZC_BIT: c_int = 6;
pub const WM8990_LIN12VOL_MASK: c_uint = 0x001F  /* LIN12VOL - [4:0] */;
pub const WM8990_LIN12VOL_SHIFT: c_int = 0;
//
// R25 (0x19) - Left Line Input 3&4 Volume
//
pub const WM8990_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8990_LI34MUTE: c_uint = 0x0080  /* LI34MUTE */;
pub const WM8990_LI34MUTE_BIT: c_int = 7;
pub const WM8990_LI34ZC: c_uint = 0x0040  /* LI34ZC */;
pub const WM8990_LI34ZC_BIT: c_int = 6;
pub const WM8990_LIN34VOL_MASK: c_uint = 0x001F  /* LIN34VOL - [4:0] */;
pub const WM8990_LIN34VOL_SHIFT: c_int = 0;
//
// R26 (0x1A) - Right Line Input 1&2 Volume
//
pub const WM8990_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8990_RI12MUTE: c_uint = 0x0080  /* RI12MUTE */;
pub const WM8990_RI12MUTE_BIT: c_int = 7;
pub const WM8990_RI12ZC: c_uint = 0x0040  /* RI12ZC */;
pub const WM8990_RI12ZC_BIT: c_int = 6;
pub const WM8990_RIN12VOL_MASK: c_uint = 0x001F  /* RIN12VOL - [4:0] */;
pub const WM8990_RIN12VOL_SHIFT: c_int = 0;
//
// R27 (0x1B) - Right Line Input 3&4 Volume
//
pub const WM8990_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8990_RI34MUTE: c_uint = 0x0080  /* RI34MUTE */;
pub const WM8990_RI34MUTE_BIT: c_int = 7;
pub const WM8990_RI34ZC: c_uint = 0x0040  /* RI34ZC */;
pub const WM8990_RI34ZC_BIT: c_int = 6;
pub const WM8990_RIN34VOL_MASK: c_uint = 0x001F  /* RIN34VOL - [4:0] */;
pub const WM8990_RIN34VOL_SHIFT: c_int = 0;
//
// R28 (0x1C) - Left Output Volume
//
pub const WM8990_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8990_LOZC: c_uint = 0x0080  /* LOZC */;
pub const WM8990_LOZC_BIT: c_int = 7;
pub const WM8990_LOUTVOL_MASK: c_uint = 0x007F  /* LOUTVOL - [6:0] */;
pub const WM8990_LOUTVOL_SHIFT: c_int = 0;
//
// R29 (0x1D) - Right Output Volume
//
pub const WM8990_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8990_ROZC: c_uint = 0x0080  /* ROZC */;
pub const WM8990_ROZC_BIT: c_int = 7;
pub const WM8990_ROUTVOL_MASK: c_uint = 0x007F  /* ROUTVOL - [6:0] */;
pub const WM8990_ROUTVOL_SHIFT: c_int = 0;
//
// R30 (0x1E) - Line Outputs Volume
//
pub const WM8990_LONMUTE: c_uint = 0x0040  /* LONMUTE */;
pub const WM8990_LONMUTE_BIT: c_int = 6;
pub const WM8990_LOPMUTE: c_uint = 0x0020  /* LOPMUTE */;
pub const WM8990_LOPMUTE_BIT: c_int = 5;
pub const WM8990_LOATTN: c_uint = 0x0010  /* LOATTN */;
pub const WM8990_LOATTN_BIT: c_int = 4;
pub const WM8990_RONMUTE: c_uint = 0x0004  /* RONMUTE */;
pub const WM8990_RONMUTE_BIT: c_int = 2;
pub const WM8990_ROPMUTE: c_uint = 0x0002  /* ROPMUTE */;
pub const WM8990_ROPMUTE_BIT: c_int = 1;
pub const WM8990_ROATTN: c_uint = 0x0001  /* ROATTN */;
pub const WM8990_ROATTN_BIT: c_int = 0;
//
// R31 (0x1F) - Out3/4 Volume
//
pub const WM8990_OUT3MUTE: c_uint = 0x0020  /* OUT3MUTE */;
pub const WM8990_OUT3MUTE_BIT: c_int = 5;
pub const WM8990_OUT3ATTN: c_uint = 0x0010  /* OUT3ATTN */;
pub const WM8990_OUT3ATTN_BIT: c_int = 4;
pub const WM8990_OUT4MUTE: c_uint = 0x0002  /* OUT4MUTE */;
pub const WM8990_OUT4MUTE_BIT: c_int = 1;
pub const WM8990_OUT4ATTN: c_uint = 0x0001  /* OUT4ATTN */;
pub const WM8990_OUT4ATTN_BIT: c_int = 0;
//
// R32 (0x20) - Left OPGA Volume
//
pub const WM8990_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8990_LOPGAZC: c_uint = 0x0080  /* LOPGAZC */;
pub const WM8990_LOPGAZC_BIT: c_int = 7;
pub const WM8990_LOPGAVOL_MASK: c_uint = 0x007F  /* LOPGAVOL - [6:0] */;
pub const WM8990_LOPGAVOL_SHIFT: c_int = 0;
//
// R33 (0x21) - Right OPGA Volume
//
pub const WM8990_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8990_ROPGAZC: c_uint = 0x0080  /* ROPGAZC */;
pub const WM8990_ROPGAZC_BIT: c_int = 7;
pub const WM8990_ROPGAVOL_MASK: c_uint = 0x007F  /* ROPGAVOL - [6:0] */;
pub const WM8990_ROPGAVOL_SHIFT: c_int = 0;
//
// R34 (0x22) - Speaker Volume
//
pub const WM8990_SPKATTN_MASK: c_uint = 0x0003  /* SPKATTN - [1:0] */;
pub const WM8990_SPKATTN_SHIFT: c_int = 0;
//
// R35 (0x23) - ClassD1
//
pub const WM8990_CDMODE: c_uint = 0x0100  /* CDMODE */;
pub const WM8990_CDMODE_BIT: c_int = 8;
//
// R37 (0x25) - ClassD3
//
pub const WM8990_DCGAIN_MASK: c_uint = 0x0007  /* DCGAIN - [5:3] */;
pub const WM8990_DCGAIN_SHIFT: c_int = 3;
pub const WM8990_ACGAIN_MASK: c_uint = 0x0007  /* ACGAIN - [2:0] */;
pub const WM8990_ACGAIN_SHIFT: c_int = 0;
//
// R38 (0x26) - ClassD4
//
pub const WM8990_SPKZC_MASK: c_uint = 0x0001  /* SPKZC */;

pub const WM8990_SPKVOL_MASK: c_uint = 0x007F  /* SPKVOL - [6:0] */;

//
// R39 (0x27) - Input Mixer1
//
pub const WM8990_AINLMODE_MASK: c_uint = 0x000C  /* AINLMODE - [3:2] */;
pub const WM8990_AINLMODE_SHIFT: c_int = 2;
pub const WM8990_AINRMODE_MASK: c_uint = 0x0003  /* AINRMODE - [1:0] */;
pub const WM8990_AINRMODE_SHIFT: c_int = 0;
//
// R40 (0x28) - Input Mixer2
//
pub const WM8990_LMP4: c_uint = 0x0080	/* LMP4 */;

pub const WM8990_LMN3: c_uint = 0x0040  /* LMN3 */;

pub const WM8990_LMP2: c_uint = 0x0020  /* LMP2 */;

pub const WM8990_LMN1: c_uint = 0x0010  /* LMN1 */;

pub const WM8990_RMP4: c_uint = 0x0008  /* RMP4 */;

pub const WM8990_RMN3: c_uint = 0x0004  /* RMN3 */;

pub const WM8990_RMP2: c_uint = 0x0002  /* RMP2 */;

pub const WM8990_RMN1: c_uint = 0x0001  /* RMN1 */;

//
// R41 (0x29) - Input Mixer3
//
pub const WM8990_L34MNB: c_uint = 0x0100  /* L34MNB */;
pub const WM8990_L34MNB_BIT: c_int = 8;
pub const WM8990_L34MNBST: c_uint = 0x0080  /* L34MNBST */;
pub const WM8990_L34MNBST_BIT: c_int = 7;
pub const WM8990_L12MNB: c_uint = 0x0020  /* L12MNB */;
pub const WM8990_L12MNB_BIT: c_int = 5;
pub const WM8990_L12MNBST: c_uint = 0x0010  /* L12MNBST */;
pub const WM8990_L12MNBST_BIT: c_int = 4;
pub const WM8990_LDBVOL_MASK: c_uint = 0x0007  /* LDBVOL - [2:0] */;
pub const WM8990_LDBVOL_SHIFT: c_int = 0;
//
// R42 (0x2A) - Input Mixer4
//
pub const WM8990_R34MNB: c_uint = 0x0100  /* R34MNB */;
pub const WM8990_R34MNB_BIT: c_int = 8;
pub const WM8990_R34MNBST: c_uint = 0x0080  /* R34MNBST */;
pub const WM8990_R34MNBST_BIT: c_int = 7;
pub const WM8990_R12MNB: c_uint = 0x0020  /* R12MNB */;
pub const WM8990_R12MNB_BIT: c_int = 5;
pub const WM8990_R12MNBST: c_uint = 0x0010  /* R12MNBST */;
pub const WM8990_R12MNBST_BIT: c_int = 4;
pub const WM8990_RDBVOL_MASK: c_uint = 0x0007  /* RDBVOL - [2:0] */;
pub const WM8990_RDBVOL_SHIFT: c_int = 0;
//
// R43 (0x2B) - Input Mixer5
//
pub const WM8990_LI2BVOL_MASK: c_uint = 0x07  /* LI2BVOL - [8:6] */;
pub const WM8990_LI2BVOL_SHIFT: c_int = 6;
pub const WM8990_LR4BVOL_MASK: c_uint = 0x07  /* LR4BVOL - [5:3] */;
pub const WM8990_LR4BVOL_SHIFT: c_int = 3;
pub const WM8990_LL4BVOL_MASK: c_uint = 0x07  /* LL4BVOL - [2:0] */;
pub const WM8990_LL4BVOL_SHIFT: c_int = 0;
//
// R44 (0x2C) - Input Mixer6
//
pub const WM8990_RI2BVOL_MASK: c_uint = 0x07  /* RI2BVOL - [8:6] */;
pub const WM8990_RI2BVOL_SHIFT: c_int = 6;
pub const WM8990_RL4BVOL_MASK: c_uint = 0x07  /* RL4BVOL - [5:3] */;
pub const WM8990_RL4BVOL_SHIFT: c_int = 3;
pub const WM8990_RR4BVOL_MASK: c_uint = 0x07  /* RR4BVOL - [2:0] */;
pub const WM8990_RR4BVOL_SHIFT: c_int = 0;
//
// R45 (0x2D) - Output Mixer1
//
pub const WM8990_LRBLO: c_uint = 0x0080  /* LRBLO */;
pub const WM8990_LRBLO_BIT: c_int = 7;
pub const WM8990_LLBLO: c_uint = 0x0040  /* LLBLO */;
pub const WM8990_LLBLO_BIT: c_int = 6;
pub const WM8990_LRI3LO: c_uint = 0x0020  /* LRI3LO */;
pub const WM8990_LRI3LO_BIT: c_int = 5;
pub const WM8990_LLI3LO: c_uint = 0x0010  /* LLI3LO */;
pub const WM8990_LLI3LO_BIT: c_int = 4;
pub const WM8990_LR12LO: c_uint = 0x0008  /* LR12LO */;
pub const WM8990_LR12LO_BIT: c_int = 3;
pub const WM8990_LL12LO: c_uint = 0x0004  /* LL12LO */;
pub const WM8990_LL12LO_BIT: c_int = 2;
pub const WM8990_LDLO: c_uint = 0x0001  /* LDLO */;
pub const WM8990_LDLO_BIT: c_int = 0;
//
// R46 (0x2E) - Output Mixer2
//
pub const WM8990_RLBRO: c_uint = 0x0080  /* RLBRO */;
pub const WM8990_RLBRO_BIT: c_int = 7;
pub const WM8990_RRBRO: c_uint = 0x0040  /* RRBRO */;
pub const WM8990_RRBRO_BIT: c_int = 6;
pub const WM8990_RLI3RO: c_uint = 0x0020  /* RLI3RO */;
pub const WM8990_RLI3RO_BIT: c_int = 5;
pub const WM8990_RRI3RO: c_uint = 0x0010  /* RRI3RO */;
pub const WM8990_RRI3RO_BIT: c_int = 4;
pub const WM8990_RL12RO: c_uint = 0x0008  /* RL12RO */;
pub const WM8990_RL12RO_BIT: c_int = 3;
pub const WM8990_RR12RO: c_uint = 0x0004  /* RR12RO */;
pub const WM8990_RR12RO_BIT: c_int = 2;
pub const WM8990_RDRO: c_uint = 0x0001  /* RDRO */;
pub const WM8990_RDRO_BIT: c_int = 0;
//
// R47 (0x2F) - Output Mixer3
//
pub const WM8990_LLI3LOVOL_MASK: c_uint = 0x07  /* LLI3LOVOL - [8:6] */;
pub const WM8990_LLI3LOVOL_SHIFT: c_int = 6;
pub const WM8990_LR12LOVOL_MASK: c_uint = 0x07  /* LR12LOVOL - [5:3] */;
pub const WM8990_LR12LOVOL_SHIFT: c_int = 3;
pub const WM8990_LL12LOVOL_MASK: c_uint = 0x07  /* LL12LOVOL - [2:0] */;
pub const WM8990_LL12LOVOL_SHIFT: c_int = 0;
//
// R48 (0x30) - Output Mixer4
//
pub const WM8990_RRI3ROVOL_MASK: c_uint = 0x07  /* RRI3ROVOL - [8:6] */;
pub const WM8990_RRI3ROVOL_SHIFT: c_int = 6;
pub const WM8990_RL12ROVOL_MASK: c_uint = 0x07  /* RL12ROVOL - [5:3] */;
pub const WM8990_RL12ROVOL_SHIFT: c_int = 3;
pub const WM8990_RR12ROVOL_MASK: c_uint = 0x07  /* RR12ROVOL - [2:0] */;
pub const WM8990_RR12ROVOL_SHIFT: c_int = 0;
//
// R49 (0x31) - Output Mixer5
//
pub const WM8990_LRI3LOVOL_MASK: c_uint = 0x07  /* LRI3LOVOL - [8:6] */;
pub const WM8990_LRI3LOVOL_SHIFT: c_int = 6;
pub const WM8990_LRBLOVOL_MASK: c_uint = 0x07  /* LRBLOVOL - [5:3] */;
pub const WM8990_LRBLOVOL_SHIFT: c_int = 3;
pub const WM8990_LLBLOVOL_MASK: c_uint = 0x07  /* LLBLOVOL - [2:0] */;
pub const WM8990_LLBLOVOL_SHIFT: c_int = 0;
//
// R50 (0x32) - Output Mixer6
//
pub const WM8990_RLI3ROVOL_MASK: c_uint = 0x07  /* RLI3ROVOL - [8:6] */;
pub const WM8990_RLI3ROVOL_SHIFT: c_int = 6;
pub const WM8990_RLBROVOL_MASK: c_uint = 0x07  /* RLBROVOL - [5:3] */;
pub const WM8990_RLBROVOL_SHIFT: c_int = 3;
pub const WM8990_RRBROVOL_MASK: c_uint = 0x07  /* RRBROVOL - [2:0] */;
pub const WM8990_RRBROVOL_SHIFT: c_int = 0;
//
// R51 (0x33) - Out3/4 Mixer
//
pub const WM8990_VSEL_MASK: c_uint = 0x0180  /* VSEL - [8:7] */;
pub const WM8990_LI4O3: c_uint = 0x0020  /* LI4O3 */;
pub const WM8990_LI4O3_BIT: c_int = 5;
pub const WM8990_LPGAO3: c_uint = 0x0010  /* LPGAO3 */;
pub const WM8990_LPGAO3_BIT: c_int = 4;
pub const WM8990_RI4O4: c_uint = 0x0002  /* RI4O4 */;
pub const WM8990_RI4O4_BIT: c_int = 1;
pub const WM8990_RPGAO4: c_uint = 0x0001  /* RPGAO4 */;
pub const WM8990_RPGAO4_BIT: c_int = 0;
//
// R52 (0x34) - Line Mixer1
//
pub const WM8990_LLOPGALON: c_uint = 0x0040  /* LLOPGALON */;
pub const WM8990_LLOPGALON_BIT: c_int = 6;
pub const WM8990_LROPGALON: c_uint = 0x0020  /* LROPGALON */;
pub const WM8990_LROPGALON_BIT: c_int = 5;
pub const WM8990_LOPLON: c_uint = 0x0010  /* LOPLON */;
pub const WM8990_LOPLON_BIT: c_int = 4;
pub const WM8990_LR12LOP: c_uint = 0x0004  /* LR12LOP */;
pub const WM8990_LR12LOP_BIT: c_int = 2;
pub const WM8990_LL12LOP: c_uint = 0x0002  /* LL12LOP */;
pub const WM8990_LL12LOP_BIT: c_int = 1;
pub const WM8990_LLOPGALOP: c_uint = 0x0001  /* LLOPGALOP */;
pub const WM8990_LLOPGALOP_BIT: c_int = 0;
//
// R53 (0x35) - Line Mixer2
//
pub const WM8990_RROPGARON: c_uint = 0x0040  /* RROPGARON */;
pub const WM8990_RROPGARON_BIT: c_int = 6;
pub const WM8990_RLOPGARON: c_uint = 0x0020  /* RLOPGARON */;
pub const WM8990_RLOPGARON_BIT: c_int = 5;
pub const WM8990_ROPRON: c_uint = 0x0010  /* ROPRON */;
pub const WM8990_ROPRON_BIT: c_int = 4;
pub const WM8990_RL12ROP: c_uint = 0x0004  /* RL12ROP */;
pub const WM8990_RL12ROP_BIT: c_int = 2;
pub const WM8990_RR12ROP: c_uint = 0x0002  /* RR12ROP */;
pub const WM8990_RR12ROP_BIT: c_int = 1;
pub const WM8990_RROPGAROP: c_uint = 0x0001  /* RROPGAROP */;
pub const WM8990_RROPGAROP_BIT: c_int = 0;
//
// R54 (0x36) - Speaker Mixer
//
pub const WM8990_LB2SPK: c_uint = 0x0080  /* LB2SPK */;
pub const WM8990_LB2SPK_BIT: c_int = 7;
pub const WM8990_RB2SPK: c_uint = 0x0040  /* RB2SPK */;
pub const WM8990_RB2SPK_BIT: c_int = 6;
pub const WM8990_LI2SPK: c_uint = 0x0020  /* LI2SPK */;
pub const WM8990_LI2SPK_BIT: c_int = 5;
pub const WM8990_RI2SPK: c_uint = 0x0010  /* RI2SPK */;
pub const WM8990_RI2SPK_BIT: c_int = 4;
pub const WM8990_LOPGASPK: c_uint = 0x0008  /* LOPGASPK */;
pub const WM8990_LOPGASPK_BIT: c_int = 3;
pub const WM8990_ROPGASPK: c_uint = 0x0004  /* ROPGASPK */;
pub const WM8990_ROPGASPK_BIT: c_int = 2;
pub const WM8990_LDSPK: c_uint = 0x0002  /* LDSPK */;
pub const WM8990_LDSPK_BIT: c_int = 1;
pub const WM8990_RDSPK: c_uint = 0x0001  /* RDSPK */;
pub const WM8990_RDSPK_BIT: c_int = 0;
//
// R55 (0x37) - Additional Control
//
pub const WM8990_VROI: c_uint = 0x0001  /* VROI */;
//
// R56 (0x38) - AntiPOP1
//
pub const WM8990_DIS_LLINE: c_uint = 0x0020  /* DIS_LLINE */;
pub const WM8990_DIS_RLINE: c_uint = 0x0010  /* DIS_RLINE */;
pub const WM8990_DIS_OUT3: c_uint = 0x0008  /* DIS_OUT3 */;
pub const WM8990_DIS_OUT4: c_uint = 0x0004  /* DIS_OUT4 */;
pub const WM8990_DIS_LOUT: c_uint = 0x0002  /* DIS_LOUT */;
pub const WM8990_DIS_ROUT: c_uint = 0x0001  /* DIS_ROUT */;
//
// R57 (0x39) - AntiPOP2
//
pub const WM8990_SOFTST: c_uint = 0x0040  /* SOFTST */;
pub const WM8990_BUFIOEN: c_uint = 0x0008  /* BUFIOEN */;
pub const WM8990_BUFDCOPEN: c_uint = 0x0004  /* BUFDCOPEN */;
pub const WM8990_POBCTRL: c_uint = 0x0002  /* POBCTRL */;
pub const WM8990_VMIDTOG: c_uint = 0x0001  /* VMIDTOG */;
//
// R58 (0x3A) - MICBIAS
//
pub const WM8990_MCDSCTH_MASK: c_uint = 0x00C0  /* MCDSCTH - [7:6] */;
pub const WM8990_MCDTHR_MASK: c_uint = 0x0038  /* MCDTHR - [5:3] */;
pub const WM8990_MCD: c_uint = 0x0004  /* MCD */;
pub const WM8990_MBSEL: c_uint = 0x0001  /* MBSEL */;
//
// R60 (0x3C) - PLL1
//
pub const WM8990_SDM: c_uint = 0x0080  /* SDM */;
pub const WM8990_PRESCALE: c_uint = 0x0040  /* PRESCALE */;
pub const WM8990_PLLN_MASK: c_uint = 0x000F  /* PLLN - [3:0] */;
//
// R61 (0x3D) - PLL2
//
pub const WM8990_PLLK1_MASK: c_uint = 0x00FF  /* PLLK1 - [7:0] */;
//
// R62 (0x3E) - PLL3
//
pub const WM8990_PLLK2_MASK: c_uint = 0x00FF  /* PLLK2 - [7:0] */;
pub const WM8990_MCLK_DIV: c_int = 0;
pub const WM8990_DACCLK_DIV: c_int = 1;
pub const WM8990_ADCCLK_DIV: c_int = 2;
pub const WM8990_BCLK_DIV: c_int = 3;

// ------------------------------ END OF FILE ---------------------------------
