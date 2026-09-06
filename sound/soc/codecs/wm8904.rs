//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8904.h
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
// wm8904.h  --  WM8904 ASoC driver
//
// Copyright 2009 Wolfson Microelectronics, plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
pub const WM8904_CLK_AUTO: c_int = 0;
pub const WM8904_CLK_MCLK: c_int = 1;
pub const WM8904_CLK_FLL: c_int = 2;
pub const WM8904_FLL_MCLK: c_int = 1;
pub const WM8904_FLL_BCLK: c_int = 2;
pub const WM8904_FLL_LRCLK: c_int = 3;
pub const WM8904_FLL_FREE_RUNNING: c_int = 4;
//
// Register values.
//
pub const WM8904_SW_RESET_AND_ID: c_uint = 0x00;
pub const WM8904_REVISION: c_uint = 0x01;
pub const WM8904_BIAS_CONTROL_0: c_uint = 0x04;
pub const WM8904_VMID_CONTROL_0: c_uint = 0x05;
pub const WM8904_MIC_BIAS_CONTROL_0: c_uint = 0x06;
pub const WM8904_MIC_BIAS_CONTROL_1: c_uint = 0x07;
pub const WM8904_ANALOGUE_DAC_0: c_uint = 0x08;
pub const WM8904_MIC_FILTER_CONTROL: c_uint = 0x09;
pub const WM8904_ANALOGUE_ADC_0: c_uint = 0x0A;
pub const WM8904_POWER_MANAGEMENT_0: c_uint = 0x0C;
pub const WM8904_POWER_MANAGEMENT_2: c_uint = 0x0E;
pub const WM8904_POWER_MANAGEMENT_3: c_uint = 0x0F;
pub const WM8904_POWER_MANAGEMENT_6: c_uint = 0x12;
pub const WM8904_CLOCK_RATES_0: c_uint = 0x14;
pub const WM8904_CLOCK_RATES_1: c_uint = 0x15;
pub const WM8904_CLOCK_RATES_2: c_uint = 0x16;
pub const WM8904_AUDIO_INTERFACE_0: c_uint = 0x18;
pub const WM8904_AUDIO_INTERFACE_1: c_uint = 0x19;
pub const WM8904_AUDIO_INTERFACE_2: c_uint = 0x1A;
pub const WM8904_AUDIO_INTERFACE_3: c_uint = 0x1B;
pub const WM8904_DAC_DIGITAL_VOLUME_LEFT: c_uint = 0x1E;
pub const WM8904_DAC_DIGITAL_VOLUME_RIGHT: c_uint = 0x1F;
pub const WM8904_DAC_DIGITAL_0: c_uint = 0x20;
pub const WM8904_DAC_DIGITAL_1: c_uint = 0x21;
pub const WM8904_ADC_DIGITAL_VOLUME_LEFT: c_uint = 0x24;
pub const WM8904_ADC_DIGITAL_VOLUME_RIGHT: c_uint = 0x25;
pub const WM8904_ADC_DIGITAL_0: c_uint = 0x26;
pub const WM8904_DIGITAL_MICROPHONE_0: c_uint = 0x27;
pub const WM8904_DRC_0: c_uint = 0x28;
pub const WM8904_DRC_1: c_uint = 0x29;
pub const WM8904_DRC_2: c_uint = 0x2A;
pub const WM8904_DRC_3: c_uint = 0x2B;
pub const WM8904_ANALOGUE_LEFT_INPUT_0: c_uint = 0x2C;
pub const WM8904_ANALOGUE_RIGHT_INPUT_0: c_uint = 0x2D;
pub const WM8904_ANALOGUE_LEFT_INPUT_1: c_uint = 0x2E;
pub const WM8904_ANALOGUE_RIGHT_INPUT_1: c_uint = 0x2F;
pub const WM8904_ANALOGUE_OUT1_LEFT: c_uint = 0x39;
pub const WM8904_ANALOGUE_OUT1_RIGHT: c_uint = 0x3A;
pub const WM8904_ANALOGUE_OUT2_LEFT: c_uint = 0x3B;
pub const WM8904_ANALOGUE_OUT2_RIGHT: c_uint = 0x3C;
pub const WM8904_ANALOGUE_OUT12_ZC: c_uint = 0x3D;
pub const WM8904_DC_SERVO_0: c_uint = 0x43;
pub const WM8904_DC_SERVO_1: c_uint = 0x44;
pub const WM8904_DC_SERVO_2: c_uint = 0x45;
pub const WM8904_DC_SERVO_4: c_uint = 0x47;
pub const WM8904_DC_SERVO_5: c_uint = 0x48;
pub const WM8904_DC_SERVO_6: c_uint = 0x49;
pub const WM8904_DC_SERVO_7: c_uint = 0x4A;
pub const WM8904_DC_SERVO_8: c_uint = 0x4B;
pub const WM8904_DC_SERVO_9: c_uint = 0x4C;
pub const WM8904_DC_SERVO_READBACK_0: c_uint = 0x4D;
pub const WM8904_ANALOGUE_HP_0: c_uint = 0x5A;
pub const WM8904_ANALOGUE_LINEOUT_0: c_uint = 0x5E;
pub const WM8904_CHARGE_PUMP_0: c_uint = 0x62;
pub const WM8904_CLASS_W_0: c_uint = 0x68;
pub const WM8904_WRITE_SEQUENCER_0: c_uint = 0x6C;
pub const WM8904_WRITE_SEQUENCER_1: c_uint = 0x6D;
pub const WM8904_WRITE_SEQUENCER_2: c_uint = 0x6E;
pub const WM8904_WRITE_SEQUENCER_3: c_uint = 0x6F;
pub const WM8904_WRITE_SEQUENCER_4: c_uint = 0x70;
pub const WM8904_FLL_CONTROL_1: c_uint = 0x74;
pub const WM8904_FLL_CONTROL_2: c_uint = 0x75;
pub const WM8904_FLL_CONTROL_3: c_uint = 0x76;
pub const WM8904_FLL_CONTROL_4: c_uint = 0x77;
pub const WM8904_FLL_CONTROL_5: c_uint = 0x78;
pub const WM8904_GPIO_CONTROL_1: c_uint = 0x79;
pub const WM8904_GPIO_CONTROL_2: c_uint = 0x7A;
pub const WM8904_GPIO_CONTROL_3: c_uint = 0x7B;
pub const WM8904_GPIO_CONTROL_4: c_uint = 0x7C;
pub const WM8904_DIGITAL_PULLS: c_uint = 0x7E;
pub const WM8904_INTERRUPT_STATUS: c_uint = 0x7F;
pub const WM8904_INTERRUPT_STATUS_MASK: c_uint = 0x80;
pub const WM8904_INTERRUPT_POLARITY: c_uint = 0x81;
pub const WM8904_INTERRUPT_DEBOUNCE: c_uint = 0x82;
pub const WM8904_EQ1: c_uint = 0x86;
pub const WM8904_EQ2: c_uint = 0x87;
pub const WM8904_EQ3: c_uint = 0x88;
pub const WM8904_EQ4: c_uint = 0x89;
pub const WM8904_EQ5: c_uint = 0x8A;
pub const WM8904_EQ6: c_uint = 0x8B;
pub const WM8904_EQ7: c_uint = 0x8C;
pub const WM8904_EQ8: c_uint = 0x8D;
pub const WM8904_EQ9: c_uint = 0x8E;
pub const WM8904_EQ10: c_uint = 0x8F;
pub const WM8904_EQ11: c_uint = 0x90;
pub const WM8904_EQ12: c_uint = 0x91;
pub const WM8904_EQ13: c_uint = 0x92;
pub const WM8904_EQ14: c_uint = 0x93;
pub const WM8904_EQ15: c_uint = 0x94;
pub const WM8904_EQ16: c_uint = 0x95;
pub const WM8904_EQ17: c_uint = 0x96;
pub const WM8904_EQ18: c_uint = 0x97;
pub const WM8904_EQ19: c_uint = 0x98;
pub const WM8904_EQ20: c_uint = 0x99;
pub const WM8904_EQ21: c_uint = 0x9A;
pub const WM8904_EQ22: c_uint = 0x9B;
pub const WM8904_EQ23: c_uint = 0x9C;
pub const WM8904_EQ24: c_uint = 0x9D;
pub const WM8904_CONTROL_INTERFACE_TEST_1: c_uint = 0xA1;
pub const WM8904_ADC_TEST_0: c_uint = 0xC6;
pub const WM8904_ANALOGUE_OUTPUT_BIAS_0: c_uint = 0xCC;
pub const WM8904_FLL_NCO_TEST_0: c_uint = 0xF7;
pub const WM8904_FLL_NCO_TEST_1: c_uint = 0xF8;
pub const WM8904_REGISTER_COUNT: c_int = 101;
pub const WM8904_MAX_REGISTER: c_uint = 0xF8;
//
// Field Definitions.
//
// R0 (0x00) - SW Reset and ID
//
pub const WM8904_SW_RST_DEV_ID1_MASK: c_uint = 0xFFFF  /* SW_RST_DEV_ID1 - [15:0] */;

//
// R1 (0x01) - Revision
//
pub const WM8904_REVISION_MASK: c_uint = 0x000F  /* REVISION - [3:0] */;

//
// R4 (0x04) - Bias Control 0
//
pub const WM8904_POBCTRL: c_uint = 0x0010  /* POBCTRL */;
pub const WM8904_POBCTRL_MASK: c_uint = 0x0010  /* POBCTRL */;

pub const WM8904_ISEL_MASK: c_uint = 0x000C  /* ISEL - [3:2] */;

pub const WM8904_STARTUP_BIAS_ENA: c_uint = 0x0002  /* STARTUP_BIAS_ENA */;
pub const WM8904_STARTUP_BIAS_ENA_MASK: c_uint = 0x0002  /* STARTUP_BIAS_ENA */;

pub const WM8904_BIAS_ENA: c_uint = 0x0001  /* BIAS_ENA */;
pub const WM8904_BIAS_ENA_MASK: c_uint = 0x0001  /* BIAS_ENA */;

//
// R5 (0x05) - VMID Control 0
//
pub const WM8904_VMID_BUF_ENA: c_uint = 0x0040  /* VMID_BUF_ENA */;
pub const WM8904_VMID_BUF_ENA_MASK: c_uint = 0x0040  /* VMID_BUF_ENA */;

pub const WM8904_VMID_RES_MASK: c_uint = 0x0006  /* VMID_RES - [2:1] */;

pub const WM8904_VMID_ENA: c_uint = 0x0001  /* VMID_ENA */;
pub const WM8904_VMID_ENA_MASK: c_uint = 0x0001  /* VMID_ENA */;

//
// R8 (0x08) - Analogue DAC 0
//
pub const WM8904_DAC_BIAS_SEL_MASK: c_uint = 0x0018  /* DAC_BIAS_SEL - [4:3] */;

pub const WM8904_DAC_VMID_BIAS_SEL_MASK: c_uint = 0x0006  /* DAC_VMID_BIAS_SEL - [2:1] */;

//
// R9 (0x09) - mic Filter Control
//
pub const WM8904_MIC_DET_SET_THRESHOLD_MASK: c_uint = 0xF000  /* MIC_DET_SET_THRESHOLD - [15:12] */;

pub const WM8904_MIC_DET_RESET_THRESHOLD_MASK: c_uint = 0x0F00  /* MIC_DET_RESET_THRESHOLD - [11:8] */;

pub const WM8904_MIC_SHORT_SET_THRESHOLD_MASK: c_uint = 0x00F0  /* MIC_SHORT_SET_THRESHOLD - [7:4] */;

pub const WM8904_MIC_SHORT_RESET_THRESHOLD_MASK: c_uint = 0x000F  /* MIC_SHORT_RESET_THRESHOLD - [3:0] */;

//
// R10 (0x0A) - Analogue ADC 0
//
pub const WM8904_ADC_OSR128: c_uint = 0x0001  /* ADC_OSR128 */;
pub const WM8904_ADC_OSR128_MASK: c_uint = 0x0001  /* ADC_OSR128 */;

//
// R12 (0x0C) - Power Management 0
//
pub const WM8904_INL_ENA: c_uint = 0x0002  /* INL_ENA */;
pub const WM8904_INL_ENA_MASK: c_uint = 0x0002  /* INL_ENA */;

pub const WM8904_INR_ENA: c_uint = 0x0001  /* INR_ENA */;
pub const WM8904_INR_ENA_MASK: c_uint = 0x0001  /* INR_ENA */;

//
// R14 (0x0E) - Power Management 2
//
pub const WM8904_HPL_PGA_ENA: c_uint = 0x0002  /* HPL_PGA_ENA */;
pub const WM8904_HPL_PGA_ENA_MASK: c_uint = 0x0002  /* HPL_PGA_ENA */;

pub const WM8904_HPR_PGA_ENA: c_uint = 0x0001  /* HPR_PGA_ENA */;
pub const WM8904_HPR_PGA_ENA_MASK: c_uint = 0x0001  /* HPR_PGA_ENA */;

//
// R15 (0x0F) - Power Management 3
//
pub const WM8904_LINEOUTL_PGA_ENA: c_uint = 0x0002  /* LINEOUTL_PGA_ENA */;
pub const WM8904_LINEOUTL_PGA_ENA_MASK: c_uint = 0x0002  /* LINEOUTL_PGA_ENA */;

pub const WM8904_LINEOUTR_PGA_ENA: c_uint = 0x0001  /* LINEOUTR_PGA_ENA */;
pub const WM8904_LINEOUTR_PGA_ENA_MASK: c_uint = 0x0001  /* LINEOUTR_PGA_ENA */;

//
// R18 (0x12) - Power Management 6
//
pub const WM8904_DACL_ENA: c_uint = 0x0008  /* DACL_ENA */;
pub const WM8904_DACL_ENA_MASK: c_uint = 0x0008  /* DACL_ENA */;

pub const WM8904_DACR_ENA: c_uint = 0x0004  /* DACR_ENA */;
pub const WM8904_DACR_ENA_MASK: c_uint = 0x0004  /* DACR_ENA */;

pub const WM8904_ADCL_ENA: c_uint = 0x0002  /* ADCL_ENA */;
pub const WM8904_ADCL_ENA_MASK: c_uint = 0x0002  /* ADCL_ENA */;

pub const WM8904_ADCR_ENA: c_uint = 0x0001  /* ADCR_ENA */;
pub const WM8904_ADCR_ENA_MASK: c_uint = 0x0001  /* ADCR_ENA */;

//
// R20 (0x14) - Clock Rates 0
//
pub const WM8904_TOCLK_RATE_DIV16: c_uint = 0x4000  /* TOCLK_RATE_DIV16 */;
pub const WM8904_TOCLK_RATE_DIV16_MASK: c_uint = 0x4000  /* TOCLK_RATE_DIV16 */;

pub const WM8904_TOCLK_RATE_X4: c_uint = 0x2000  /* TOCLK_RATE_X4 */;
pub const WM8904_TOCLK_RATE_X4_MASK: c_uint = 0x2000  /* TOCLK_RATE_X4 */;

pub const WM8904_SR_MODE: c_uint = 0x1000  /* SR_MODE */;
pub const WM8904_SR_MODE_MASK: c_uint = 0x1000  /* SR_MODE */;

pub const WM8904_MCLK_DIV: c_uint = 0x0001  /* MCLK_DIV */;
pub const WM8904_MCLK_DIV_MASK: c_uint = 0x0001  /* MCLK_DIV */;

//
// R21 (0x15) - Clock Rates 1
//
pub const WM8904_CLK_SYS_RATE_MASK: c_uint = 0x3C00  /* CLK_SYS_RATE - [13:10] */;

pub const WM8904_SAMPLE_RATE_MASK: c_uint = 0x0007  /* SAMPLE_RATE - [2:0] */;

//
// R22 (0x16) - Clock Rates 2
//
pub const WM8904_MCLK_INV: c_uint = 0x8000  /* MCLK_INV */;
pub const WM8904_MCLK_INV_MASK: c_uint = 0x8000  /* MCLK_INV */;

pub const WM8904_SYSCLK_SRC: c_uint = 0x4000  /* SYSCLK_SRC */;
pub const WM8904_SYSCLK_SRC_MASK: c_uint = 0x4000  /* SYSCLK_SRC */;

pub const WM8904_TOCLK_RATE: c_uint = 0x1000  /* TOCLK_RATE */;
pub const WM8904_TOCLK_RATE_MASK: c_uint = 0x1000  /* TOCLK_RATE */;

pub const WM8904_OPCLK_ENA: c_uint = 0x0008  /* OPCLK_ENA */;
pub const WM8904_OPCLK_ENA_MASK: c_uint = 0x0008  /* OPCLK_ENA */;

pub const WM8904_CLK_SYS_ENA: c_uint = 0x0004  /* CLK_SYS_ENA */;
pub const WM8904_CLK_SYS_ENA_MASK: c_uint = 0x0004  /* CLK_SYS_ENA */;

pub const WM8904_CLK_DSP_ENA: c_uint = 0x0002  /* CLK_DSP_ENA */;
pub const WM8904_CLK_DSP_ENA_MASK: c_uint = 0x0002  /* CLK_DSP_ENA */;

pub const WM8904_TOCLK_ENA: c_uint = 0x0001  /* TOCLK_ENA */;
pub const WM8904_TOCLK_ENA_MASK: c_uint = 0x0001  /* TOCLK_ENA */;

//
// R24 (0x18) - Audio Interface 0
//
pub const WM8904_DACL_DATINV: c_uint = 0x1000  /* DACL_DATINV */;
pub const WM8904_DACL_DATINV_MASK: c_uint = 0x1000  /* DACL_DATINV */;

pub const WM8904_DACR_DATINV: c_uint = 0x0800  /* DACR_DATINV */;
pub const WM8904_DACR_DATINV_MASK: c_uint = 0x0800  /* DACR_DATINV */;

pub const WM8904_DAC_BOOST_MASK: c_uint = 0x0600  /* DAC_BOOST - [10:9] */;

pub const WM8904_LOOPBACK: c_uint = 0x0100  /* LOOPBACK */;
pub const WM8904_LOOPBACK_MASK: c_uint = 0x0100  /* LOOPBACK */;

pub const WM8904_AIFADCL_SRC: c_uint = 0x0080  /* AIFADCL_SRC */;
pub const WM8904_AIFADCL_SRC_MASK: c_uint = 0x0080  /* AIFADCL_SRC */;

pub const WM8904_AIFADCR_SRC: c_uint = 0x0040  /* AIFADCR_SRC */;
pub const WM8904_AIFADCR_SRC_MASK: c_uint = 0x0040  /* AIFADCR_SRC */;

pub const WM8904_AIFDACL_SRC: c_uint = 0x0020  /* AIFDACL_SRC */;
pub const WM8904_AIFDACL_SRC_MASK: c_uint = 0x0020  /* AIFDACL_SRC */;

pub const WM8904_AIFDACR_SRC: c_uint = 0x0010  /* AIFDACR_SRC */;
pub const WM8904_AIFDACR_SRC_MASK: c_uint = 0x0010  /* AIFDACR_SRC */;

pub const WM8904_ADC_COMP: c_uint = 0x0008  /* ADC_COMP */;
pub const WM8904_ADC_COMP_MASK: c_uint = 0x0008  /* ADC_COMP */;

pub const WM8904_ADC_COMPMODE: c_uint = 0x0004  /* ADC_COMPMODE */;
pub const WM8904_ADC_COMPMODE_MASK: c_uint = 0x0004  /* ADC_COMPMODE */;

pub const WM8904_DAC_COMP: c_uint = 0x0002  /* DAC_COMP */;
pub const WM8904_DAC_COMP_MASK: c_uint = 0x0002  /* DAC_COMP */;

pub const WM8904_DAC_COMPMODE: c_uint = 0x0001  /* DAC_COMPMODE */;
pub const WM8904_DAC_COMPMODE_MASK: c_uint = 0x0001  /* DAC_COMPMODE */;

//
// R25 (0x19) - Audio Interface 1
//
pub const WM8904_AIFDAC_TDM: c_uint = 0x2000  /* AIFDAC_TDM */;
pub const WM8904_AIFDAC_TDM_MASK: c_uint = 0x2000  /* AIFDAC_TDM */;

pub const WM8904_AIFDAC_TDM_CHAN: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;
pub const WM8904_AIFDAC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;

pub const WM8904_AIFADC_TDM: c_uint = 0x0800  /* AIFADC_TDM */;
pub const WM8904_AIFADC_TDM_MASK: c_uint = 0x0800  /* AIFADC_TDM */;

pub const WM8904_AIFADC_TDM_CHAN: c_uint = 0x0400  /* AIFADC_TDM_CHAN */;
pub const WM8904_AIFADC_TDM_CHAN_MASK: c_uint = 0x0400  /* AIFADC_TDM_CHAN */;

pub const WM8904_AIF_TRIS: c_uint = 0x0100  /* AIF_TRIS */;
pub const WM8904_AIF_TRIS_MASK: c_uint = 0x0100  /* AIF_TRIS */;

pub const WM8904_AIF_BCLK_INV: c_uint = 0x0080  /* AIF_BCLK_INV */;
pub const WM8904_AIF_BCLK_INV_MASK: c_uint = 0x0080  /* AIF_BCLK_INV */;

pub const WM8904_BCLK_DIR: c_uint = 0x0040  /* BCLK_DIR */;
pub const WM8904_BCLK_DIR_MASK: c_uint = 0x0040  /* BCLK_DIR */;

pub const WM8904_AIF_LRCLK_INV: c_uint = 0x0010  /* AIF_LRCLK_INV */;
pub const WM8904_AIF_LRCLK_INV_MASK: c_uint = 0x0010  /* AIF_LRCLK_INV */;

pub const WM8904_AIF_WL_MASK: c_uint = 0x000C  /* AIF_WL - [3:2] */;

pub const WM8904_AIF_FMT_MASK: c_uint = 0x0003  /* AIF_FMT - [1:0] */;

//
// R26 (0x1A) - Audio Interface 2
//
pub const WM8904_OPCLK_DIV_MASK: c_uint = 0x0F00  /* OPCLK_DIV - [11:8] */;

pub const WM8904_BCLK_DIV_MASK: c_uint = 0x001F  /* BCLK_DIV - [4:0] */;

//
// R27 (0x1B) - Audio Interface 3
//
pub const WM8904_LRCLK_DIR: c_uint = 0x0800  /* LRCLK_DIR */;
pub const WM8904_LRCLK_DIR_MASK: c_uint = 0x0800  /* LRCLK_DIR */;

pub const WM8904_LRCLK_RATE_MASK: c_uint = 0x07FF  /* LRCLK_RATE - [10:0] */;

//
// R30 (0x1E) - DAC Digital Volume Left
//
pub const WM8904_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8904_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8904_DACL_VOL_MASK: c_uint = 0x00FF  /* DACL_VOL - [7:0] */;

//
// R31 (0x1F) - DAC Digital Volume Right
//
pub const WM8904_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8904_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8904_DACR_VOL_MASK: c_uint = 0x00FF  /* DACR_VOL - [7:0] */;

//
// R32 (0x20) - DAC Digital 0
//
pub const WM8904_ADCL_DAC_SVOL_MASK: c_uint = 0x0F00  /* ADCL_DAC_SVOL - [11:8] */;

pub const WM8904_ADCR_DAC_SVOL_MASK: c_uint = 0x00F0  /* ADCR_DAC_SVOL - [7:4] */;

pub const WM8904_ADC_TO_DACL_MASK: c_uint = 0x000C  /* ADC_TO_DACL - [3:2] */;

pub const WM8904_ADC_TO_DACR_MASK: c_uint = 0x0003  /* ADC_TO_DACR - [1:0] */;

//
// R33 (0x21) - DAC Digital 1
//
pub const WM8904_DAC_MONO: c_uint = 0x1000  /* DAC_MONO */;
pub const WM8904_DAC_MONO_MASK: c_uint = 0x1000  /* DAC_MONO */;

pub const WM8904_DAC_SB_FILT: c_uint = 0x0800  /* DAC_SB_FILT */;
pub const WM8904_DAC_SB_FILT_MASK: c_uint = 0x0800  /* DAC_SB_FILT */;

pub const WM8904_DAC_MUTERATE: c_uint = 0x0400  /* DAC_MUTERATE */;
pub const WM8904_DAC_MUTERATE_MASK: c_uint = 0x0400  /* DAC_MUTERATE */;

pub const WM8904_DAC_UNMUTE_RAMP: c_uint = 0x0200  /* DAC_UNMUTE_RAMP */;
pub const WM8904_DAC_UNMUTE_RAMP_MASK: c_uint = 0x0200  /* DAC_UNMUTE_RAMP */;

pub const WM8904_DAC_OSR128: c_uint = 0x0040  /* DAC_OSR128 */;
pub const WM8904_DAC_OSR128_MASK: c_uint = 0x0040  /* DAC_OSR128 */;

pub const WM8904_DAC_MUTE: c_uint = 0x0008  /* DAC_MUTE */;
pub const WM8904_DAC_MUTE_MASK: c_uint = 0x0008  /* DAC_MUTE */;

pub const WM8904_DEEMPH_MASK: c_uint = 0x0006  /* DEEMPH - [2:1] */;

//
// R36 (0x24) - ADC Digital Volume Left
//
pub const WM8904_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8904_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8904_ADCL_VOL_MASK: c_uint = 0x00FF  /* ADCL_VOL - [7:0] */;

//
// R37 (0x25) - ADC Digital Volume Right
//
pub const WM8904_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8904_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8904_ADCR_VOL_MASK: c_uint = 0x00FF  /* ADCR_VOL - [7:0] */;

//
// R38 (0x26) - ADC Digital 0
//
pub const WM8904_ADC_HPF_CUT_MASK: c_uint = 0x0060  /* ADC_HPF_CUT - [6:5] */;

pub const WM8904_ADC_HPF: c_uint = 0x0010  /* ADC_HPF */;
pub const WM8904_ADC_HPF_MASK: c_uint = 0x0010  /* ADC_HPF */;

pub const WM8904_ADCL_DATINV: c_uint = 0x0002  /* ADCL_DATINV */;
pub const WM8904_ADCL_DATINV_MASK: c_uint = 0x0002  /* ADCL_DATINV */;

pub const WM8904_ADCR_DATINV: c_uint = 0x0001  /* ADCR_DATINV */;
pub const WM8904_ADCR_DATINV_MASK: c_uint = 0x0001  /* ADCR_DATINV */;

//
// R39 (0x27) - Digital Microphone 0
//
pub const WM8904_DMIC_ENA: c_uint = 0x1000  /* DMIC_ENA */;
pub const WM8904_DMIC_ENA_MASK: c_uint = 0x1000  /* DMIC_ENA */;

pub const WM8904_DMIC_SRC: c_uint = 0x0800  /* DMIC_SRC */;
pub const WM8904_DMIC_SRC_MASK: c_uint = 0x0800  /* DMIC_SRC */;

//
// R40 (0x28) - DRC 0
//
pub const WM8904_DRC_ENA: c_uint = 0x8000  /* DRC_ENA */;
pub const WM8904_DRC_ENA_MASK: c_uint = 0x8000  /* DRC_ENA */;

pub const WM8904_DRC_DAC_PATH: c_uint = 0x4000  /* DRC_DAC_PATH */;
pub const WM8904_DRC_DAC_PATH_MASK: c_uint = 0x4000  /* DRC_DAC_PATH */;

pub const WM8904_DRC_GS_HYST_LVL_MASK: c_uint = 0x1800  /* DRC_GS_HYST_LVL - [12:11] */;

pub const WM8904_DRC_STARTUP_GAIN_MASK: c_uint = 0x07C0  /* DRC_STARTUP_GAIN - [10:6] */;

pub const WM8904_DRC_FF_DELAY: c_uint = 0x0020  /* DRC_FF_DELAY */;
pub const WM8904_DRC_FF_DELAY_MASK: c_uint = 0x0020  /* DRC_FF_DELAY */;

pub const WM8904_DRC_GS_ENA: c_uint = 0x0008  /* DRC_GS_ENA */;
pub const WM8904_DRC_GS_ENA_MASK: c_uint = 0x0008  /* DRC_GS_ENA */;

pub const WM8904_DRC_QR: c_uint = 0x0004  /* DRC_QR */;
pub const WM8904_DRC_QR_MASK: c_uint = 0x0004  /* DRC_QR */;

pub const WM8904_DRC_ANTICLIP: c_uint = 0x0002  /* DRC_ANTICLIP */;
pub const WM8904_DRC_ANTICLIP_MASK: c_uint = 0x0002  /* DRC_ANTICLIP */;

pub const WM8904_DRC_GS_HYST: c_uint = 0x0001  /* DRC_GS_HYST */;
pub const WM8904_DRC_GS_HYST_MASK: c_uint = 0x0001  /* DRC_GS_HYST */;

//
// R41 (0x29) - DRC 1
//
pub const WM8904_DRC_ATK_MASK: c_uint = 0xF000  /* DRC_ATK - [15:12] */;

pub const WM8904_DRC_DCY_MASK: c_uint = 0x0F00  /* DRC_DCY - [11:8] */;

pub const WM8904_DRC_QR_THR_MASK: c_uint = 0x00C0  /* DRC_QR_THR - [7:6] */;

pub const WM8904_DRC_QR_DCY_MASK: c_uint = 0x0030  /* DRC_QR_DCY - [5:4] */;

pub const WM8904_DRC_MINGAIN_MASK: c_uint = 0x000C  /* DRC_MINGAIN - [3:2] */;

pub const WM8904_DRC_MAXGAIN_MASK: c_uint = 0x0003  /* DRC_MAXGAIN - [1:0] */;

//
// R42 (0x2A) - DRC 2
//
pub const WM8904_DRC_HI_COMP_MASK: c_uint = 0x0038  /* DRC_HI_COMP - [5:3] */;

pub const WM8904_DRC_LO_COMP_MASK: c_uint = 0x0007  /* DRC_LO_COMP - [2:0] */;

//
// R43 (0x2B) - DRC 3
//
pub const WM8904_DRC_KNEE_IP_MASK: c_uint = 0x07E0  /* DRC_KNEE_IP - [10:5] */;

pub const WM8904_DRC_KNEE_OP_MASK: c_uint = 0x001F  /* DRC_KNEE_OP - [4:0] */;

//
// R44 (0x2C) - Analogue Left Input 0
//
pub const WM8904_LINMUTE: c_uint = 0x0080  /* LINMUTE */;
pub const WM8904_LINMUTE_MASK: c_uint = 0x0080  /* LINMUTE */;

pub const WM8904_LIN_VOL_MASK: c_uint = 0x001F  /* LIN_VOL - [4:0] */;

//
// R45 (0x2D) - Analogue Right Input 0
//
pub const WM8904_RINMUTE: c_uint = 0x0080  /* RINMUTE */;
pub const WM8904_RINMUTE_MASK: c_uint = 0x0080  /* RINMUTE */;

pub const WM8904_RIN_VOL_MASK: c_uint = 0x001F  /* RIN_VOL - [4:0] */;

//
// R46 (0x2E) - Analogue Left Input 1
//
pub const WM8904_INL_CM_ENA: c_uint = 0x0040  /* INL_CM_ENA */;
pub const WM8904_INL_CM_ENA_MASK: c_uint = 0x0040  /* INL_CM_ENA */;

pub const WM8904_L_IP_SEL_N_MASK: c_uint = 0x0030  /* L_IP_SEL_N - [5:4] */;

pub const WM8904_L_IP_SEL_P_MASK: c_uint = 0x000C  /* L_IP_SEL_P - [3:2] */;

pub const WM8904_L_MODE_MASK: c_uint = 0x0003  /* L_MODE - [1:0] */;

//
// R47 (0x2F) - Analogue Right Input 1
//
pub const WM8904_INR_CM_ENA: c_uint = 0x0040  /* INR_CM_ENA */;
pub const WM8904_INR_CM_ENA_MASK: c_uint = 0x0040  /* INR_CM_ENA */;

pub const WM8904_R_IP_SEL_N_MASK: c_uint = 0x0030  /* R_IP_SEL_N - [5:4] */;

pub const WM8904_R_IP_SEL_P_MASK: c_uint = 0x000C  /* R_IP_SEL_P - [3:2] */;

pub const WM8904_R_MODE_MASK: c_uint = 0x0003  /* R_MODE - [1:0] */;

//
// R57 (0x39) - Analogue OUT1 Left
//
pub const WM8904_HPOUTL_MUTE: c_uint = 0x0100  /* HPOUTL_MUTE */;
pub const WM8904_HPOUTL_MUTE_MASK: c_uint = 0x0100  /* HPOUTL_MUTE */;

pub const WM8904_HPOUT_VU: c_uint = 0x0080  /* HPOUT_VU */;
pub const WM8904_HPOUT_VU_MASK: c_uint = 0x0080  /* HPOUT_VU */;

pub const WM8904_HPOUTLZC: c_uint = 0x0040  /* HPOUTLZC */;
pub const WM8904_HPOUTLZC_MASK: c_uint = 0x0040  /* HPOUTLZC */;

pub const WM8904_HPOUTL_VOL_MASK: c_uint = 0x003F  /* HPOUTL_VOL - [5:0] */;

//
// R58 (0x3A) - Analogue OUT1 Right
//
pub const WM8904_HPOUTR_MUTE: c_uint = 0x0100  /* HPOUTR_MUTE */;
pub const WM8904_HPOUTR_MUTE_MASK: c_uint = 0x0100  /* HPOUTR_MUTE */;

pub const WM8904_HPOUT_VU: c_uint = 0x0080  /* HPOUT_VU */;
pub const WM8904_HPOUT_VU_MASK: c_uint = 0x0080  /* HPOUT_VU */;

pub const WM8904_HPOUTRZC: c_uint = 0x0040  /* HPOUTRZC */;
pub const WM8904_HPOUTRZC_MASK: c_uint = 0x0040  /* HPOUTRZC */;

pub const WM8904_HPOUTR_VOL_MASK: c_uint = 0x003F  /* HPOUTR_VOL - [5:0] */;

//
// R59 (0x3B) - Analogue OUT2 Left
//
pub const WM8904_LINEOUTL_MUTE: c_uint = 0x0100  /* LINEOUTL_MUTE */;
pub const WM8904_LINEOUTL_MUTE_MASK: c_uint = 0x0100  /* LINEOUTL_MUTE */;

pub const WM8904_LINEOUT_VU: c_uint = 0x0080  /* LINEOUT_VU */;
pub const WM8904_LINEOUT_VU_MASK: c_uint = 0x0080  /* LINEOUT_VU */;

pub const WM8904_LINEOUTLZC: c_uint = 0x0040  /* LINEOUTLZC */;
pub const WM8904_LINEOUTLZC_MASK: c_uint = 0x0040  /* LINEOUTLZC */;

pub const WM8904_LINEOUTL_VOL_MASK: c_uint = 0x003F  /* LINEOUTL_VOL - [5:0] */;

//
// R60 (0x3C) - Analogue OUT2 Right
//
pub const WM8904_LINEOUTR_MUTE: c_uint = 0x0100  /* LINEOUTR_MUTE */;
pub const WM8904_LINEOUTR_MUTE_MASK: c_uint = 0x0100  /* LINEOUTR_MUTE */;

pub const WM8904_LINEOUT_VU: c_uint = 0x0080  /* LINEOUT_VU */;
pub const WM8904_LINEOUT_VU_MASK: c_uint = 0x0080  /* LINEOUT_VU */;

pub const WM8904_LINEOUTRZC: c_uint = 0x0040  /* LINEOUTRZC */;
pub const WM8904_LINEOUTRZC_MASK: c_uint = 0x0040  /* LINEOUTRZC */;

pub const WM8904_LINEOUTR_VOL_MASK: c_uint = 0x003F  /* LINEOUTR_VOL - [5:0] */;

//
// R61 (0x3D) - Analogue OUT12 ZC
//
pub const WM8904_HPL_BYP_ENA: c_uint = 0x0008  /* HPL_BYP_ENA */;
pub const WM8904_HPL_BYP_ENA_MASK: c_uint = 0x0008  /* HPL_BYP_ENA */;

pub const WM8904_HPR_BYP_ENA: c_uint = 0x0004  /* HPR_BYP_ENA */;
pub const WM8904_HPR_BYP_ENA_MASK: c_uint = 0x0004  /* HPR_BYP_ENA */;

pub const WM8904_LINEOUTL_BYP_ENA: c_uint = 0x0002  /* LINEOUTL_BYP_ENA */;
pub const WM8904_LINEOUTL_BYP_ENA_MASK: c_uint = 0x0002  /* LINEOUTL_BYP_ENA */;

pub const WM8904_LINEOUTR_BYP_ENA: c_uint = 0x0001  /* LINEOUTR_BYP_ENA */;
pub const WM8904_LINEOUTR_BYP_ENA_MASK: c_uint = 0x0001  /* LINEOUTR_BYP_ENA */;

//
// R67 (0x43) - DC Servo 0
//
pub const WM8904_DCS_ENA_CHAN_3: c_uint = 0x0008  /* DCS_ENA_CHAN_3 */;
pub const WM8904_DCS_ENA_CHAN_3_MASK: c_uint = 0x0008  /* DCS_ENA_CHAN_3 */;

pub const WM8904_DCS_ENA_CHAN_2: c_uint = 0x0004  /* DCS_ENA_CHAN_2 */;
pub const WM8904_DCS_ENA_CHAN_2_MASK: c_uint = 0x0004  /* DCS_ENA_CHAN_2 */;

pub const WM8904_DCS_ENA_CHAN_1: c_uint = 0x0002  /* DCS_ENA_CHAN_1 */;
pub const WM8904_DCS_ENA_CHAN_1_MASK: c_uint = 0x0002  /* DCS_ENA_CHAN_1 */;

pub const WM8904_DCS_ENA_CHAN_0: c_uint = 0x0001  /* DCS_ENA_CHAN_0 */;
pub const WM8904_DCS_ENA_CHAN_0_MASK: c_uint = 0x0001  /* DCS_ENA_CHAN_0 */;

//
// R68 (0x44) - DC Servo 1
//
pub const WM8904_DCS_TRIG_SINGLE_3: c_uint = 0x8000  /* DCS_TRIG_SINGLE_3 */;
pub const WM8904_DCS_TRIG_SINGLE_3_MASK: c_uint = 0x8000  /* DCS_TRIG_SINGLE_3 */;

pub const WM8904_DCS_TRIG_SINGLE_2: c_uint = 0x4000  /* DCS_TRIG_SINGLE_2 */;
pub const WM8904_DCS_TRIG_SINGLE_2_MASK: c_uint = 0x4000  /* DCS_TRIG_SINGLE_2 */;

pub const WM8904_DCS_TRIG_SINGLE_1: c_uint = 0x2000  /* DCS_TRIG_SINGLE_1 */;
pub const WM8904_DCS_TRIG_SINGLE_1_MASK: c_uint = 0x2000  /* DCS_TRIG_SINGLE_1 */;

pub const WM8904_DCS_TRIG_SINGLE_0: c_uint = 0x1000  /* DCS_TRIG_SINGLE_0 */;
pub const WM8904_DCS_TRIG_SINGLE_0_MASK: c_uint = 0x1000  /* DCS_TRIG_SINGLE_0 */;

pub const WM8904_DCS_TRIG_SERIES_3: c_uint = 0x0800  /* DCS_TRIG_SERIES_3 */;
pub const WM8904_DCS_TRIG_SERIES_3_MASK: c_uint = 0x0800  /* DCS_TRIG_SERIES_3 */;

pub const WM8904_DCS_TRIG_SERIES_2: c_uint = 0x0400  /* DCS_TRIG_SERIES_2 */;
pub const WM8904_DCS_TRIG_SERIES_2_MASK: c_uint = 0x0400  /* DCS_TRIG_SERIES_2 */;

pub const WM8904_DCS_TRIG_SERIES_1: c_uint = 0x0200  /* DCS_TRIG_SERIES_1 */;
pub const WM8904_DCS_TRIG_SERIES_1_MASK: c_uint = 0x0200  /* DCS_TRIG_SERIES_1 */;

pub const WM8904_DCS_TRIG_SERIES_0: c_uint = 0x0100  /* DCS_TRIG_SERIES_0 */;
pub const WM8904_DCS_TRIG_SERIES_0_MASK: c_uint = 0x0100  /* DCS_TRIG_SERIES_0 */;

pub const WM8904_DCS_TRIG_STARTUP_3: c_uint = 0x0080  /* DCS_TRIG_STARTUP_3 */;
pub const WM8904_DCS_TRIG_STARTUP_3_MASK: c_uint = 0x0080  /* DCS_TRIG_STARTUP_3 */;

pub const WM8904_DCS_TRIG_STARTUP_2: c_uint = 0x0040  /* DCS_TRIG_STARTUP_2 */;
pub const WM8904_DCS_TRIG_STARTUP_2_MASK: c_uint = 0x0040  /* DCS_TRIG_STARTUP_2 */;

pub const WM8904_DCS_TRIG_STARTUP_1: c_uint = 0x0020  /* DCS_TRIG_STARTUP_1 */;
pub const WM8904_DCS_TRIG_STARTUP_1_MASK: c_uint = 0x0020  /* DCS_TRIG_STARTUP_1 */;

pub const WM8904_DCS_TRIG_STARTUP_0: c_uint = 0x0010  /* DCS_TRIG_STARTUP_0 */;
pub const WM8904_DCS_TRIG_STARTUP_0_MASK: c_uint = 0x0010  /* DCS_TRIG_STARTUP_0 */;

pub const WM8904_DCS_TRIG_DAC_WR_3: c_uint = 0x0008  /* DCS_TRIG_DAC_WR_3 */;
pub const WM8904_DCS_TRIG_DAC_WR_3_MASK: c_uint = 0x0008  /* DCS_TRIG_DAC_WR_3 */;

pub const WM8904_DCS_TRIG_DAC_WR_2: c_uint = 0x0004  /* DCS_TRIG_DAC_WR_2 */;
pub const WM8904_DCS_TRIG_DAC_WR_2_MASK: c_uint = 0x0004  /* DCS_TRIG_DAC_WR_2 */;

pub const WM8904_DCS_TRIG_DAC_WR_1: c_uint = 0x0002  /* DCS_TRIG_DAC_WR_1 */;
pub const WM8904_DCS_TRIG_DAC_WR_1_MASK: c_uint = 0x0002  /* DCS_TRIG_DAC_WR_1 */;

pub const WM8904_DCS_TRIG_DAC_WR_0: c_uint = 0x0001  /* DCS_TRIG_DAC_WR_0 */;
pub const WM8904_DCS_TRIG_DAC_WR_0_MASK: c_uint = 0x0001  /* DCS_TRIG_DAC_WR_0 */;

//
// R69 (0x45) - DC Servo 2
//
pub const WM8904_DCS_TIMER_PERIOD_23_MASK: c_uint = 0x0F00  /* DCS_TIMER_PERIOD_23 - [11:8] */;

pub const WM8904_DCS_TIMER_PERIOD_01_MASK: c_uint = 0x000F  /* DCS_TIMER_PERIOD_01 - [3:0] */;

//
// R71 (0x47) - DC Servo 4
//
pub const WM8904_DCS_SERIES_NO_23_MASK: c_uint = 0x007F  /* DCS_SERIES_NO_23 - [6:0] */;

//
// R72 (0x48) - DC Servo 5
//
pub const WM8904_DCS_SERIES_NO_01_MASK: c_uint = 0x007F  /* DCS_SERIES_NO_01 - [6:0] */;

//
// R73 (0x49) - DC Servo 6
//
pub const WM8904_DCS_DAC_WR_VAL_3_MASK: c_uint = 0x00FF  /* DCS_DAC_WR_VAL_3 - [7:0] */;

//
// R74 (0x4A) - DC Servo 7
//
pub const WM8904_DCS_DAC_WR_VAL_2_MASK: c_uint = 0x00FF  /* DCS_DAC_WR_VAL_2 - [7:0] */;

//
// R75 (0x4B) - DC Servo 8
//
pub const WM8904_DCS_DAC_WR_VAL_1_MASK: c_uint = 0x00FF  /* DCS_DAC_WR_VAL_1 - [7:0] */;

//
// R76 (0x4C) - DC Servo 9
//
pub const WM8904_DCS_DAC_WR_VAL_0_MASK: c_uint = 0x00FF  /* DCS_DAC_WR_VAL_0 - [7:0] */;

//
// R77 (0x4D) - DC Servo Readback 0
//
pub const WM8904_DCS_CAL_COMPLETE_MASK: c_uint = 0x0F00  /* DCS_CAL_COMPLETE - [11:8] */;

pub const WM8904_DCS_DAC_WR_COMPLETE_MASK: c_uint = 0x00F0  /* DCS_DAC_WR_COMPLETE - [7:4] */;

pub const WM8904_DCS_STARTUP_COMPLETE_MASK: c_uint = 0x000F  /* DCS_STARTUP_COMPLETE - [3:0] */;

//
// R90 (0x5A) - Analogue HP 0
//
pub const WM8904_HPL_RMV_SHORT: c_uint = 0x0080  /* HPL_RMV_SHORT */;
pub const WM8904_HPL_RMV_SHORT_MASK: c_uint = 0x0080  /* HPL_RMV_SHORT */;

pub const WM8904_HPL_ENA_OUTP: c_uint = 0x0040  /* HPL_ENA_OUTP */;
pub const WM8904_HPL_ENA_OUTP_MASK: c_uint = 0x0040  /* HPL_ENA_OUTP */;

pub const WM8904_HPL_ENA_DLY: c_uint = 0x0020  /* HPL_ENA_DLY */;
pub const WM8904_HPL_ENA_DLY_MASK: c_uint = 0x0020  /* HPL_ENA_DLY */;

pub const WM8904_HPL_ENA: c_uint = 0x0010  /* HPL_ENA */;
pub const WM8904_HPL_ENA_MASK: c_uint = 0x0010  /* HPL_ENA */;

pub const WM8904_HPR_RMV_SHORT: c_uint = 0x0008  /* HPR_RMV_SHORT */;
pub const WM8904_HPR_RMV_SHORT_MASK: c_uint = 0x0008  /* HPR_RMV_SHORT */;

pub const WM8904_HPR_ENA_OUTP: c_uint = 0x0004  /* HPR_ENA_OUTP */;
pub const WM8904_HPR_ENA_OUTP_MASK: c_uint = 0x0004  /* HPR_ENA_OUTP */;

pub const WM8904_HPR_ENA_DLY: c_uint = 0x0002  /* HPR_ENA_DLY */;
pub const WM8904_HPR_ENA_DLY_MASK: c_uint = 0x0002  /* HPR_ENA_DLY */;

pub const WM8904_HPR_ENA: c_uint = 0x0001  /* HPR_ENA */;
pub const WM8904_HPR_ENA_MASK: c_uint = 0x0001  /* HPR_ENA */;

//
// R94 (0x5E) - Analogue Lineout 0
//
pub const WM8904_LINEOUTL_RMV_SHORT: c_uint = 0x0080  /* LINEOUTL_RMV_SHORT */;
pub const WM8904_LINEOUTL_RMV_SHORT_MASK: c_uint = 0x0080  /* LINEOUTL_RMV_SHORT */;

pub const WM8904_LINEOUTL_ENA_OUTP: c_uint = 0x0040  /* LINEOUTL_ENA_OUTP */;
pub const WM8904_LINEOUTL_ENA_OUTP_MASK: c_uint = 0x0040  /* LINEOUTL_ENA_OUTP */;

pub const WM8904_LINEOUTL_ENA_DLY: c_uint = 0x0020  /* LINEOUTL_ENA_DLY */;
pub const WM8904_LINEOUTL_ENA_DLY_MASK: c_uint = 0x0020  /* LINEOUTL_ENA_DLY */;

pub const WM8904_LINEOUTL_ENA: c_uint = 0x0010  /* LINEOUTL_ENA */;
pub const WM8904_LINEOUTL_ENA_MASK: c_uint = 0x0010  /* LINEOUTL_ENA */;

pub const WM8904_LINEOUTR_RMV_SHORT: c_uint = 0x0008  /* LINEOUTR_RMV_SHORT */;
pub const WM8904_LINEOUTR_RMV_SHORT_MASK: c_uint = 0x0008  /* LINEOUTR_RMV_SHORT */;

pub const WM8904_LINEOUTR_ENA_OUTP: c_uint = 0x0004  /* LINEOUTR_ENA_OUTP */;
pub const WM8904_LINEOUTR_ENA_OUTP_MASK: c_uint = 0x0004  /* LINEOUTR_ENA_OUTP */;

pub const WM8904_LINEOUTR_ENA_DLY: c_uint = 0x0002  /* LINEOUTR_ENA_DLY */;
pub const WM8904_LINEOUTR_ENA_DLY_MASK: c_uint = 0x0002  /* LINEOUTR_ENA_DLY */;

pub const WM8904_LINEOUTR_ENA: c_uint = 0x0001  /* LINEOUTR_ENA */;
pub const WM8904_LINEOUTR_ENA_MASK: c_uint = 0x0001  /* LINEOUTR_ENA */;

//
// R98 (0x62) - Charge Pump 0
//
pub const WM8904_CP_ENA: c_uint = 0x0001  /* CP_ENA */;
pub const WM8904_CP_ENA_MASK: c_uint = 0x0001  /* CP_ENA */;

//
// R104 (0x68) - Class W 0
//
pub const WM8904_CP_DYN_PWR: c_uint = 0x0001  /* CP_DYN_PWR */;
pub const WM8904_CP_DYN_PWR_MASK: c_uint = 0x0001  /* CP_DYN_PWR */;

//
// R108 (0x6C) - Write Sequencer 0
//
pub const WM8904_WSEQ_ENA: c_uint = 0x0100  /* WSEQ_ENA */;
pub const WM8904_WSEQ_ENA_MASK: c_uint = 0x0100  /* WSEQ_ENA */;

pub const WM8904_WSEQ_WRITE_INDEX_MASK: c_uint = 0x001F  /* WSEQ_WRITE_INDEX - [4:0] */;

//
// R109 (0x6D) - Write Sequencer 1
//
pub const WM8904_WSEQ_DATA_WIDTH_MASK: c_uint = 0x7000  /* WSEQ_DATA_WIDTH - [14:12] */;

pub const WM8904_WSEQ_DATA_START_MASK: c_uint = 0x0F00  /* WSEQ_DATA_START - [11:8] */;

pub const WM8904_WSEQ_ADDR_MASK: c_uint = 0x00FF  /* WSEQ_ADDR - [7:0] */;

//
// R110 (0x6E) - Write Sequencer 2
//
pub const WM8904_WSEQ_EOS: c_uint = 0x4000  /* WSEQ_EOS */;
pub const WM8904_WSEQ_EOS_MASK: c_uint = 0x4000  /* WSEQ_EOS */;

pub const WM8904_WSEQ_DELAY_MASK: c_uint = 0x0F00  /* WSEQ_DELAY - [11:8] */;

pub const WM8904_WSEQ_DATA_MASK: c_uint = 0x00FF  /* WSEQ_DATA - [7:0] */;

//
// R111 (0x6F) - Write Sequencer 3
//
pub const WM8904_WSEQ_ABORT: c_uint = 0x0200  /* WSEQ_ABORT */;
pub const WM8904_WSEQ_ABORT_MASK: c_uint = 0x0200  /* WSEQ_ABORT */;

pub const WM8904_WSEQ_START: c_uint = 0x0100  /* WSEQ_START */;
pub const WM8904_WSEQ_START_MASK: c_uint = 0x0100  /* WSEQ_START */;

pub const WM8904_WSEQ_START_INDEX_MASK: c_uint = 0x003F  /* WSEQ_START_INDEX - [5:0] */;

//
// R112 (0x70) - Write Sequencer 4
//
pub const WM8904_WSEQ_CURRENT_INDEX_MASK: c_uint = 0x03F0  /* WSEQ_CURRENT_INDEX - [9:4] */;

pub const WM8904_WSEQ_BUSY: c_uint = 0x0001  /* WSEQ_BUSY */;
pub const WM8904_WSEQ_BUSY_MASK: c_uint = 0x0001  /* WSEQ_BUSY */;

//
// R116 (0x74) - FLL Control 1
//
pub const WM8904_FLL_FRACN_ENA: c_uint = 0x0004  /* FLL_FRACN_ENA */;
pub const WM8904_FLL_FRACN_ENA_MASK: c_uint = 0x0004  /* FLL_FRACN_ENA */;

pub const WM8904_FLL_OSC_ENA: c_uint = 0x0002  /* FLL_OSC_ENA */;
pub const WM8904_FLL_OSC_ENA_MASK: c_uint = 0x0002  /* FLL_OSC_ENA */;

pub const WM8904_FLL_ENA: c_uint = 0x0001  /* FLL_ENA */;
pub const WM8904_FLL_ENA_MASK: c_uint = 0x0001  /* FLL_ENA */;

//
// R117 (0x75) - FLL Control 2
//
pub const WM8904_FLL_OUTDIV_MASK: c_uint = 0x3F00  /* FLL_OUTDIV - [13:8] */;

pub const WM8904_FLL_CTRL_RATE_MASK: c_uint = 0x0070  /* FLL_CTRL_RATE - [6:4] */;

pub const WM8904_FLL_FRATIO_MASK: c_uint = 0x0007  /* FLL_FRATIO - [2:0] */;

//
// R118 (0x76) - FLL Control 3
//
pub const WM8904_FLL_K_MASK: c_uint = 0xFFFF  /* FLL_K - [15:0] */;

//
// R119 (0x77) - FLL Control 4
//
pub const WM8904_FLL_N_MASK: c_uint = 0x7FE0  /* FLL_N - [14:5] */;

pub const WM8904_FLL_GAIN_MASK: c_uint = 0x000F  /* FLL_GAIN - [3:0] */;

//
// R120 (0x78) - FLL Control 5
//
pub const WM8904_FLL_CLK_REF_DIV_MASK: c_uint = 0x0018  /* FLL_CLK_REF_DIV - [4:3] */;

pub const WM8904_FLL_CLK_REF_SRC_MASK: c_uint = 0x0003  /* FLL_CLK_REF_SRC - [1:0] */;

//
// R126 (0x7E) - Digital Pulls
//
pub const WM8904_MCLK_PU: c_uint = 0x0080  /* MCLK_PU */;
pub const WM8904_MCLK_PU_MASK: c_uint = 0x0080  /* MCLK_PU */;

pub const WM8904_MCLK_PD: c_uint = 0x0040  /* MCLK_PD */;
pub const WM8904_MCLK_PD_MASK: c_uint = 0x0040  /* MCLK_PD */;

pub const WM8904_DACDAT_PU: c_uint = 0x0020  /* DACDAT_PU */;
pub const WM8904_DACDAT_PU_MASK: c_uint = 0x0020  /* DACDAT_PU */;

pub const WM8904_DACDAT_PD: c_uint = 0x0010  /* DACDAT_PD */;
pub const WM8904_DACDAT_PD_MASK: c_uint = 0x0010  /* DACDAT_PD */;

pub const WM8904_LRCLK_PU: c_uint = 0x0008  /* LRCLK_PU */;
pub const WM8904_LRCLK_PU_MASK: c_uint = 0x0008  /* LRCLK_PU */;

pub const WM8904_LRCLK_PD: c_uint = 0x0004  /* LRCLK_PD */;
pub const WM8904_LRCLK_PD_MASK: c_uint = 0x0004  /* LRCLK_PD */;

pub const WM8904_BCLK_PU: c_uint = 0x0002  /* BCLK_PU */;
pub const WM8904_BCLK_PU_MASK: c_uint = 0x0002  /* BCLK_PU */;

pub const WM8904_BCLK_PD: c_uint = 0x0001  /* BCLK_PD */;
pub const WM8904_BCLK_PD_MASK: c_uint = 0x0001  /* BCLK_PD */;

//
// R127 (0x7F) - Interrupt Status
//
pub const WM8904_IRQ: c_uint = 0x0400  /* IRQ */;
pub const WM8904_IRQ_MASK: c_uint = 0x0400  /* IRQ */;

pub const WM8904_GPIO_BCLK_EINT: c_uint = 0x0200  /* GPIO_BCLK_EINT */;
pub const WM8904_GPIO_BCLK_EINT_MASK: c_uint = 0x0200  /* GPIO_BCLK_EINT */;

pub const WM8904_WSEQ_EINT: c_uint = 0x0100  /* WSEQ_EINT */;
pub const WM8904_WSEQ_EINT_MASK: c_uint = 0x0100  /* WSEQ_EINT */;

pub const WM8904_GPIO3_EINT: c_uint = 0x0080  /* GPIO3_EINT */;
pub const WM8904_GPIO3_EINT_MASK: c_uint = 0x0080  /* GPIO3_EINT */;

pub const WM8904_GPIO2_EINT: c_uint = 0x0040  /* GPIO2_EINT */;
pub const WM8904_GPIO2_EINT_MASK: c_uint = 0x0040  /* GPIO2_EINT */;

pub const WM8904_GPIO1_EINT: c_uint = 0x0020  /* GPIO1_EINT */;
pub const WM8904_GPIO1_EINT_MASK: c_uint = 0x0020  /* GPIO1_EINT */;

pub const WM8904_GPI8_EINT: c_uint = 0x0010  /* GPI8_EINT */;
pub const WM8904_GPI8_EINT_MASK: c_uint = 0x0010  /* GPI8_EINT */;

pub const WM8904_GPI7_EINT: c_uint = 0x0008  /* GPI7_EINT */;
pub const WM8904_GPI7_EINT_MASK: c_uint = 0x0008  /* GPI7_EINT */;

pub const WM8904_FLL_LOCK_EINT: c_uint = 0x0004  /* FLL_LOCK_EINT */;
pub const WM8904_FLL_LOCK_EINT_MASK: c_uint = 0x0004  /* FLL_LOCK_EINT */;

pub const WM8904_MIC_SHRT_EINT: c_uint = 0x0002  /* MIC_SHRT_EINT */;
pub const WM8904_MIC_SHRT_EINT_MASK: c_uint = 0x0002  /* MIC_SHRT_EINT */;

pub const WM8904_MIC_DET_EINT: c_uint = 0x0001  /* MIC_DET_EINT */;
pub const WM8904_MIC_DET_EINT_MASK: c_uint = 0x0001  /* MIC_DET_EINT */;

//
// R128 (0x80) - Interrupt Status Mask
//
pub const WM8904_IM_GPIO_BCLK_EINT: c_uint = 0x0200  /* IM_GPIO_BCLK_EINT */;
pub const WM8904_IM_GPIO_BCLK_EINT_MASK: c_uint = 0x0200  /* IM_GPIO_BCLK_EINT */;

pub const WM8904_IM_WSEQ_EINT: c_uint = 0x0100  /* IM_WSEQ_EINT */;
pub const WM8904_IM_WSEQ_EINT_MASK: c_uint = 0x0100  /* IM_WSEQ_EINT */;

pub const WM8904_IM_GPIO3_EINT: c_uint = 0x0080  /* IM_GPIO3_EINT */;
pub const WM8904_IM_GPIO3_EINT_MASK: c_uint = 0x0080  /* IM_GPIO3_EINT */;

pub const WM8904_IM_GPIO2_EINT: c_uint = 0x0040  /* IM_GPIO2_EINT */;
pub const WM8904_IM_GPIO2_EINT_MASK: c_uint = 0x0040  /* IM_GPIO2_EINT */;

pub const WM8904_IM_GPIO1_EINT: c_uint = 0x0020  /* IM_GPIO1_EINT */;
pub const WM8904_IM_GPIO1_EINT_MASK: c_uint = 0x0020  /* IM_GPIO1_EINT */;

pub const WM8904_IM_GPI8_EINT: c_uint = 0x0010  /* IM_GPI8_EINT */;
pub const WM8904_IM_GPI8_EINT_MASK: c_uint = 0x0010  /* IM_GPI8_EINT */;

pub const WM8904_IM_GPI7_EINT: c_uint = 0x0008  /* IM_GPI7_EINT */;
pub const WM8904_IM_GPI7_EINT_MASK: c_uint = 0x0008  /* IM_GPI7_EINT */;

pub const WM8904_IM_FLL_LOCK_EINT: c_uint = 0x0004  /* IM_FLL_LOCK_EINT */;
pub const WM8904_IM_FLL_LOCK_EINT_MASK: c_uint = 0x0004  /* IM_FLL_LOCK_EINT */;

pub const WM8904_IM_MIC_SHRT_EINT: c_uint = 0x0002  /* IM_MIC_SHRT_EINT */;
pub const WM8904_IM_MIC_SHRT_EINT_MASK: c_uint = 0x0002  /* IM_MIC_SHRT_EINT */;

pub const WM8904_IM_MIC_DET_EINT: c_uint = 0x0001  /* IM_MIC_DET_EINT */;
pub const WM8904_IM_MIC_DET_EINT_MASK: c_uint = 0x0001  /* IM_MIC_DET_EINT */;

//
// R129 (0x81) - Interrupt Polarity
//
pub const WM8904_GPIO_BCLK_EINT_POL: c_uint = 0x0200  /* GPIO_BCLK_EINT_POL */;
pub const WM8904_GPIO_BCLK_EINT_POL_MASK: c_uint = 0x0200  /* GPIO_BCLK_EINT_POL */;

pub const WM8904_WSEQ_EINT_POL: c_uint = 0x0100  /* WSEQ_EINT_POL */;
pub const WM8904_WSEQ_EINT_POL_MASK: c_uint = 0x0100  /* WSEQ_EINT_POL */;

pub const WM8904_GPIO3_EINT_POL: c_uint = 0x0080  /* GPIO3_EINT_POL */;
pub const WM8904_GPIO3_EINT_POL_MASK: c_uint = 0x0080  /* GPIO3_EINT_POL */;

pub const WM8904_GPIO2_EINT_POL: c_uint = 0x0040  /* GPIO2_EINT_POL */;
pub const WM8904_GPIO2_EINT_POL_MASK: c_uint = 0x0040  /* GPIO2_EINT_POL */;

pub const WM8904_GPIO1_EINT_POL: c_uint = 0x0020  /* GPIO1_EINT_POL */;
pub const WM8904_GPIO1_EINT_POL_MASK: c_uint = 0x0020  /* GPIO1_EINT_POL */;

pub const WM8904_GPI8_EINT_POL: c_uint = 0x0010  /* GPI8_EINT_POL */;
pub const WM8904_GPI8_EINT_POL_MASK: c_uint = 0x0010  /* GPI8_EINT_POL */;

pub const WM8904_GPI7_EINT_POL: c_uint = 0x0008  /* GPI7_EINT_POL */;
pub const WM8904_GPI7_EINT_POL_MASK: c_uint = 0x0008  /* GPI7_EINT_POL */;

pub const WM8904_FLL_LOCK_EINT_POL: c_uint = 0x0004  /* FLL_LOCK_EINT_POL */;
pub const WM8904_FLL_LOCK_EINT_POL_MASK: c_uint = 0x0004  /* FLL_LOCK_EINT_POL */;

pub const WM8904_MIC_SHRT_EINT_POL: c_uint = 0x0002  /* MIC_SHRT_EINT_POL */;
pub const WM8904_MIC_SHRT_EINT_POL_MASK: c_uint = 0x0002  /* MIC_SHRT_EINT_POL */;

pub const WM8904_MIC_DET_EINT_POL: c_uint = 0x0001  /* MIC_DET_EINT_POL */;
pub const WM8904_MIC_DET_EINT_POL_MASK: c_uint = 0x0001  /* MIC_DET_EINT_POL */;

//
// R130 (0x82) - Interrupt Debounce
//
pub const WM8904_GPIO_BCLK_EINT_DB: c_uint = 0x0200  /* GPIO_BCLK_EINT_DB */;
pub const WM8904_GPIO_BCLK_EINT_DB_MASK: c_uint = 0x0200  /* GPIO_BCLK_EINT_DB */;

pub const WM8904_WSEQ_EINT_DB: c_uint = 0x0100  /* WSEQ_EINT_DB */;
pub const WM8904_WSEQ_EINT_DB_MASK: c_uint = 0x0100  /* WSEQ_EINT_DB */;

pub const WM8904_GPIO3_EINT_DB: c_uint = 0x0080  /* GPIO3_EINT_DB */;
pub const WM8904_GPIO3_EINT_DB_MASK: c_uint = 0x0080  /* GPIO3_EINT_DB */;

pub const WM8904_GPIO2_EINT_DB: c_uint = 0x0040  /* GPIO2_EINT_DB */;
pub const WM8904_GPIO2_EINT_DB_MASK: c_uint = 0x0040  /* GPIO2_EINT_DB */;

pub const WM8904_GPIO1_EINT_DB: c_uint = 0x0020  /* GPIO1_EINT_DB */;
pub const WM8904_GPIO1_EINT_DB_MASK: c_uint = 0x0020  /* GPIO1_EINT_DB */;

pub const WM8904_GPI8_EINT_DB: c_uint = 0x0010  /* GPI8_EINT_DB */;
pub const WM8904_GPI8_EINT_DB_MASK: c_uint = 0x0010  /* GPI8_EINT_DB */;

pub const WM8904_GPI7_EINT_DB: c_uint = 0x0008  /* GPI7_EINT_DB */;
pub const WM8904_GPI7_EINT_DB_MASK: c_uint = 0x0008  /* GPI7_EINT_DB */;

pub const WM8904_FLL_LOCK_EINT_DB: c_uint = 0x0004  /* FLL_LOCK_EINT_DB */;
pub const WM8904_FLL_LOCK_EINT_DB_MASK: c_uint = 0x0004  /* FLL_LOCK_EINT_DB */;

pub const WM8904_MIC_SHRT_EINT_DB: c_uint = 0x0002  /* MIC_SHRT_EINT_DB */;
pub const WM8904_MIC_SHRT_EINT_DB_MASK: c_uint = 0x0002  /* MIC_SHRT_EINT_DB */;

pub const WM8904_MIC_DET_EINT_DB: c_uint = 0x0001  /* MIC_DET_EINT_DB */;
pub const WM8904_MIC_DET_EINT_DB_MASK: c_uint = 0x0001  /* MIC_DET_EINT_DB */;

//
// R134 (0x86) - EQ1
//
pub const WM8904_EQ_ENA: c_uint = 0x0001  /* EQ_ENA */;
pub const WM8904_EQ_ENA_MASK: c_uint = 0x0001  /* EQ_ENA */;

//
// R135 (0x87) - EQ2
//
pub const WM8904_EQ_B1_GAIN_MASK: c_uint = 0x001F  /* EQ_B1_GAIN - [4:0] */;

//
// R136 (0x88) - EQ3
//
pub const WM8904_EQ_B2_GAIN_MASK: c_uint = 0x001F  /* EQ_B2_GAIN - [4:0] */;

//
// R137 (0x89) - EQ4
//
pub const WM8904_EQ_B3_GAIN_MASK: c_uint = 0x001F  /* EQ_B3_GAIN - [4:0] */;

//
// R138 (0x8A) - EQ5
//
pub const WM8904_EQ_B4_GAIN_MASK: c_uint = 0x001F  /* EQ_B4_GAIN - [4:0] */;

//
// R139 (0x8B) - EQ6
//
pub const WM8904_EQ_B5_GAIN_MASK: c_uint = 0x001F  /* EQ_B5_GAIN - [4:0] */;

//
// R140 (0x8C) - EQ7
//
pub const WM8904_EQ_B1_A_MASK: c_uint = 0xFFFF  /* EQ_B1_A - [15:0] */;

//
// R141 (0x8D) - EQ8
//
pub const WM8904_EQ_B1_B_MASK: c_uint = 0xFFFF  /* EQ_B1_B - [15:0] */;

//
// R142 (0x8E) - EQ9
//
pub const WM8904_EQ_B1_PG_MASK: c_uint = 0xFFFF  /* EQ_B1_PG - [15:0] */;

//
// R143 (0x8F) - EQ10
//
pub const WM8904_EQ_B2_A_MASK: c_uint = 0xFFFF  /* EQ_B2_A - [15:0] */;

//
// R144 (0x90) - EQ11
//
pub const WM8904_EQ_B2_B_MASK: c_uint = 0xFFFF  /* EQ_B2_B - [15:0] */;

//
// R145 (0x91) - EQ12
//
pub const WM8904_EQ_B2_C_MASK: c_uint = 0xFFFF  /* EQ_B2_C - [15:0] */;

//
// R146 (0x92) - EQ13
//
pub const WM8904_EQ_B2_PG_MASK: c_uint = 0xFFFF  /* EQ_B2_PG - [15:0] */;

//
// R147 (0x93) - EQ14
//
pub const WM8904_EQ_B3_A_MASK: c_uint = 0xFFFF  /* EQ_B3_A - [15:0] */;

//
// R148 (0x94) - EQ15
//
pub const WM8904_EQ_B3_B_MASK: c_uint = 0xFFFF  /* EQ_B3_B - [15:0] */;

//
// R149 (0x95) - EQ16
//
pub const WM8904_EQ_B3_C_MASK: c_uint = 0xFFFF  /* EQ_B3_C - [15:0] */;

//
// R150 (0x96) - EQ17
//
pub const WM8904_EQ_B3_PG_MASK: c_uint = 0xFFFF  /* EQ_B3_PG - [15:0] */;

//
// R151 (0x97) - EQ18
//
pub const WM8904_EQ_B4_A_MASK: c_uint = 0xFFFF  /* EQ_B4_A - [15:0] */;

//
// R152 (0x98) - EQ19
//
pub const WM8904_EQ_B4_B_MASK: c_uint = 0xFFFF  /* EQ_B4_B - [15:0] */;

//
// R153 (0x99) - EQ20
//
pub const WM8904_EQ_B4_C_MASK: c_uint = 0xFFFF  /* EQ_B4_C - [15:0] */;

//
// R154 (0x9A) - EQ21
//
pub const WM8904_EQ_B4_PG_MASK: c_uint = 0xFFFF  /* EQ_B4_PG - [15:0] */;

//
// R155 (0x9B) - EQ22
//
pub const WM8904_EQ_B5_A_MASK: c_uint = 0xFFFF  /* EQ_B5_A - [15:0] */;

//
// R156 (0x9C) - EQ23
//
pub const WM8904_EQ_B5_B_MASK: c_uint = 0xFFFF  /* EQ_B5_B - [15:0] */;

//
// R157 (0x9D) - EQ24
//
pub const WM8904_EQ_B5_PG_MASK: c_uint = 0xFFFF  /* EQ_B5_PG - [15:0] */;

//
// R161 (0xA1) - Control Interface Test 1
//
pub const WM8904_USER_KEY: c_uint = 0x0002  /* USER_KEY */;
pub const WM8904_USER_KEY_MASK: c_uint = 0x0002  /* USER_KEY */;

//
// R198 (0xC6) - ADC Test 0
//
pub const WM8904_ADC_128_OSR_TST_MODE: c_uint = 0x0004  /* ADC_128_OSR_TST_MODE */;

pub const WM8904_ADC_BIASX1P5: c_uint = 0x0001  /* ADC_BIASX1P5 */;

//
// R204 (0xCC) - Analogue Output Bias 0
//
pub const WM8904_PGA_BIAS_MASK: c_uint = 0x0070  /* PGA_BIAS - [6:4] */;

//
// R247 (0xF7) - FLL NCO Test 0
//
pub const WM8904_FLL_FRC_NCO: c_uint = 0x0001  /* FLL_FRC_NCO */;
pub const WM8904_FLL_FRC_NCO_MASK: c_uint = 0x0001  /* FLL_FRC_NCO */;

//
// R248 (0xF8) - FLL NCO Test 1
//
pub const WM8904_FLL_FRC_NCO_VAL_MASK: c_uint = 0x003F  /* FLL_FRC_NCO_VAL - [5:0] */;

