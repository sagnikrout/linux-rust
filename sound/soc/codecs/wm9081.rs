//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm9081.h
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
// wm9081.c  --  WM9081 ALSA SoC Audio driver
//
// Author: Mark Brown
//
// Copyright 2009 Wolfson Microelectronics plc
//

//
// SYSCLK sources
//

//
// Register values.
//
pub const WM9081_SOFTWARE_RESET: c_uint = 0x00;
pub const WM9081_ANALOGUE_LINEOUT: c_uint = 0x02;
pub const WM9081_ANALOGUE_SPEAKER_PGA: c_uint = 0x03;
pub const WM9081_VMID_CONTROL: c_uint = 0x04;
pub const WM9081_BIAS_CONTROL_1: c_uint = 0x05;
pub const WM9081_ANALOGUE_MIXER: c_uint = 0x07;
pub const WM9081_ANTI_POP_CONTROL: c_uint = 0x08;
pub const WM9081_ANALOGUE_SPEAKER_1: c_uint = 0x09;
pub const WM9081_ANALOGUE_SPEAKER_2: c_uint = 0x0A;
pub const WM9081_POWER_MANAGEMENT: c_uint = 0x0B;
pub const WM9081_CLOCK_CONTROL_1: c_uint = 0x0C;
pub const WM9081_CLOCK_CONTROL_2: c_uint = 0x0D;
pub const WM9081_CLOCK_CONTROL_3: c_uint = 0x0E;
pub const WM9081_FLL_CONTROL_1: c_uint = 0x10;
pub const WM9081_FLL_CONTROL_2: c_uint = 0x11;
pub const WM9081_FLL_CONTROL_3: c_uint = 0x12;
pub const WM9081_FLL_CONTROL_4: c_uint = 0x13;
pub const WM9081_FLL_CONTROL_5: c_uint = 0x14;
pub const WM9081_AUDIO_INTERFACE_1: c_uint = 0x16;
pub const WM9081_AUDIO_INTERFACE_2: c_uint = 0x17;
pub const WM9081_AUDIO_INTERFACE_3: c_uint = 0x18;
pub const WM9081_AUDIO_INTERFACE_4: c_uint = 0x19;
pub const WM9081_INTERRUPT_STATUS: c_uint = 0x1A;
pub const WM9081_INTERRUPT_STATUS_MASK: c_uint = 0x1B;
pub const WM9081_INTERRUPT_POLARITY: c_uint = 0x1C;
pub const WM9081_INTERRUPT_CONTROL: c_uint = 0x1D;
pub const WM9081_DAC_DIGITAL_1: c_uint = 0x1E;
pub const WM9081_DAC_DIGITAL_2: c_uint = 0x1F;
pub const WM9081_DRC_1: c_uint = 0x20;
pub const WM9081_DRC_2: c_uint = 0x21;
pub const WM9081_DRC_3: c_uint = 0x22;
pub const WM9081_DRC_4: c_uint = 0x23;
pub const WM9081_WRITE_SEQUENCER_1: c_uint = 0x26;
pub const WM9081_WRITE_SEQUENCER_2: c_uint = 0x27;
pub const WM9081_MW_SLAVE_1: c_uint = 0x28;
pub const WM9081_EQ_1: c_uint = 0x2A;
pub const WM9081_EQ_2: c_uint = 0x2B;
pub const WM9081_EQ_3: c_uint = 0x2C;
pub const WM9081_EQ_4: c_uint = 0x2D;
pub const WM9081_EQ_5: c_uint = 0x2E;
pub const WM9081_EQ_6: c_uint = 0x2F;
pub const WM9081_EQ_7: c_uint = 0x30;
pub const WM9081_EQ_8: c_uint = 0x31;
pub const WM9081_EQ_9: c_uint = 0x32;
pub const WM9081_EQ_10: c_uint = 0x33;
pub const WM9081_EQ_11: c_uint = 0x34;
pub const WM9081_EQ_12: c_uint = 0x35;
pub const WM9081_EQ_13: c_uint = 0x36;
pub const WM9081_EQ_14: c_uint = 0x37;
pub const WM9081_EQ_15: c_uint = 0x38;
pub const WM9081_EQ_16: c_uint = 0x39;
pub const WM9081_EQ_17: c_uint = 0x3A;
pub const WM9081_EQ_18: c_uint = 0x3B;
pub const WM9081_EQ_19: c_uint = 0x3C;
pub const WM9081_EQ_20: c_uint = 0x3D;
pub const WM9081_REGISTER_COUNT: c_int = 55;
pub const WM9081_MAX_REGISTER: c_uint = 0x3D;
//
// Field Definitions.
//
// R0 (0x00) - Software Reset
//
pub const WM9081_SW_RST_DEV_ID1_MASK: c_uint = 0xFFFF  /* SW_RST_DEV_ID1 - [15:0] */;

//
// R2 (0x02) - Analogue Lineout
//
pub const WM9081_LINEOUT_MUTE: c_uint = 0x0080  /* LINEOUT_MUTE */;
pub const WM9081_LINEOUT_MUTE_MASK: c_uint = 0x0080  /* LINEOUT_MUTE */;

pub const WM9081_LINEOUTZC: c_uint = 0x0040  /* LINEOUTZC */;
pub const WM9081_LINEOUTZC_MASK: c_uint = 0x0040  /* LINEOUTZC */;

pub const WM9081_LINEOUT_VOL_MASK: c_uint = 0x003F  /* LINEOUT_VOL - [5:0] */;

//
// R3 (0x03) - Analogue Speaker PGA
//
pub const WM9081_SPKPGA_MUTE: c_uint = 0x0080  /* SPKPGA_MUTE */;
pub const WM9081_SPKPGA_MUTE_MASK: c_uint = 0x0080  /* SPKPGA_MUTE */;

pub const WM9081_SPKPGAZC: c_uint = 0x0040  /* SPKPGAZC */;
pub const WM9081_SPKPGAZC_MASK: c_uint = 0x0040  /* SPKPGAZC */;

pub const WM9081_SPKPGA_VOL_MASK: c_uint = 0x003F  /* SPKPGA_VOL - [5:0] */;

//
// R4 (0x04) - VMID Control
//
pub const WM9081_VMID_BUF_ENA: c_uint = 0x0020  /* VMID_BUF_ENA */;
pub const WM9081_VMID_BUF_ENA_MASK: c_uint = 0x0020  /* VMID_BUF_ENA */;

pub const WM9081_VMID_RAMP: c_uint = 0x0008  /* VMID_RAMP */;
pub const WM9081_VMID_RAMP_MASK: c_uint = 0x0008  /* VMID_RAMP */;

pub const WM9081_VMID_SEL_MASK: c_uint = 0x0006  /* VMID_SEL - [2:1] */;

pub const WM9081_VMID_FAST_ST: c_uint = 0x0001  /* VMID_FAST_ST */;
pub const WM9081_VMID_FAST_ST_MASK: c_uint = 0x0001  /* VMID_FAST_ST */;

//
// R5 (0x05) - Bias Control 1
//
pub const WM9081_BIAS_SRC: c_uint = 0x0040  /* BIAS_SRC */;
pub const WM9081_BIAS_SRC_MASK: c_uint = 0x0040  /* BIAS_SRC */;

pub const WM9081_STBY_BIAS_LVL: c_uint = 0x0020  /* STBY_BIAS_LVL */;
pub const WM9081_STBY_BIAS_LVL_MASK: c_uint = 0x0020  /* STBY_BIAS_LVL */;

pub const WM9081_STBY_BIAS_ENA: c_uint = 0x0010  /* STBY_BIAS_ENA */;
pub const WM9081_STBY_BIAS_ENA_MASK: c_uint = 0x0010  /* STBY_BIAS_ENA */;

pub const WM9081_BIAS_LVL_MASK: c_uint = 0x000C  /* BIAS_LVL - [3:2] */;

pub const WM9081_BIAS_ENA: c_uint = 0x0002  /* BIAS_ENA */;
pub const WM9081_BIAS_ENA_MASK: c_uint = 0x0002  /* BIAS_ENA */;

pub const WM9081_STARTUP_BIAS_ENA: c_uint = 0x0001  /* STARTUP_BIAS_ENA */;
pub const WM9081_STARTUP_BIAS_ENA_MASK: c_uint = 0x0001  /* STARTUP_BIAS_ENA */;

//
// R7 (0x07) - Analogue Mixer
//
pub const WM9081_DAC_SEL: c_uint = 0x0010  /* DAC_SEL */;
pub const WM9081_DAC_SEL_MASK: c_uint = 0x0010  /* DAC_SEL */;

pub const WM9081_IN2_VOL: c_uint = 0x0008  /* IN2_VOL */;
pub const WM9081_IN2_VOL_MASK: c_uint = 0x0008  /* IN2_VOL */;

pub const WM9081_IN2_ENA: c_uint = 0x0004  /* IN2_ENA */;
pub const WM9081_IN2_ENA_MASK: c_uint = 0x0004  /* IN2_ENA */;

pub const WM9081_IN1_VOL: c_uint = 0x0002  /* IN1_VOL */;
pub const WM9081_IN1_VOL_MASK: c_uint = 0x0002  /* IN1_VOL */;

pub const WM9081_IN1_ENA: c_uint = 0x0001  /* IN1_ENA */;
pub const WM9081_IN1_ENA_MASK: c_uint = 0x0001  /* IN1_ENA */;

//
// R8 (0x08) - Anti Pop Control
//
pub const WM9081_LINEOUT_DISCH: c_uint = 0x0004  /* LINEOUT_DISCH */;
pub const WM9081_LINEOUT_DISCH_MASK: c_uint = 0x0004  /* LINEOUT_DISCH */;

pub const WM9081_LINEOUT_VROI: c_uint = 0x0002  /* LINEOUT_VROI */;
pub const WM9081_LINEOUT_VROI_MASK: c_uint = 0x0002  /* LINEOUT_VROI */;

pub const WM9081_LINEOUT_CLAMP: c_uint = 0x0001  /* LINEOUT_CLAMP */;
pub const WM9081_LINEOUT_CLAMP_MASK: c_uint = 0x0001  /* LINEOUT_CLAMP */;

//
// R9 (0x09) - Analogue Speaker 1
//
pub const WM9081_SPK_DCGAIN_MASK: c_uint = 0x0038  /* SPK_DCGAIN - [5:3] */;

pub const WM9081_SPK_ACGAIN_MASK: c_uint = 0x0007  /* SPK_ACGAIN - [2:0] */;

//
// R10 (0x0A) - Analogue Speaker 2
//
pub const WM9081_SPK_MODE: c_uint = 0x0040  /* SPK_MODE */;
pub const WM9081_SPK_MODE_MASK: c_uint = 0x0040  /* SPK_MODE */;

pub const WM9081_SPK_INV_MUTE: c_uint = 0x0010  /* SPK_INV_MUTE */;
pub const WM9081_SPK_INV_MUTE_MASK: c_uint = 0x0010  /* SPK_INV_MUTE */;

pub const WM9081_OUT_SPK_CTRL: c_uint = 0x0008  /* OUT_SPK_CTRL */;
pub const WM9081_OUT_SPK_CTRL_MASK: c_uint = 0x0008  /* OUT_SPK_CTRL */;

//
// R11 (0x0B) - Power Management
//
pub const WM9081_TSHUT_ENA: c_uint = 0x0100  /* TSHUT_ENA */;
pub const WM9081_TSHUT_ENA_MASK: c_uint = 0x0100  /* TSHUT_ENA */;

pub const WM9081_TSENSE_ENA: c_uint = 0x0080  /* TSENSE_ENA */;
pub const WM9081_TSENSE_ENA_MASK: c_uint = 0x0080  /* TSENSE_ENA */;

pub const WM9081_TEMP_SHUT: c_uint = 0x0040  /* TEMP_SHUT */;
pub const WM9081_TEMP_SHUT_MASK: c_uint = 0x0040  /* TEMP_SHUT */;

pub const WM9081_LINEOUT_ENA: c_uint = 0x0010  /* LINEOUT_ENA */;
pub const WM9081_LINEOUT_ENA_MASK: c_uint = 0x0010  /* LINEOUT_ENA */;

pub const WM9081_SPKPGA_ENA: c_uint = 0x0004  /* SPKPGA_ENA */;
pub const WM9081_SPKPGA_ENA_MASK: c_uint = 0x0004  /* SPKPGA_ENA */;

pub const WM9081_SPK_ENA: c_uint = 0x0002  /* SPK_ENA */;
pub const WM9081_SPK_ENA_MASK: c_uint = 0x0002  /* SPK_ENA */;

pub const WM9081_DAC_ENA: c_uint = 0x0001  /* DAC_ENA */;
pub const WM9081_DAC_ENA_MASK: c_uint = 0x0001  /* DAC_ENA */;

//
// R12 (0x0C) - Clock Control 1
//
pub const WM9081_CLK_OP_DIV_MASK: c_uint = 0x1C00  /* CLK_OP_DIV - [12:10] */;

pub const WM9081_CLK_TO_DIV_MASK: c_uint = 0x0300  /* CLK_TO_DIV - [9:8] */;

pub const WM9081_MCLKDIV2: c_uint = 0x0080  /* MCLKDIV2 */;
pub const WM9081_MCLKDIV2_MASK: c_uint = 0x0080  /* MCLKDIV2 */;

//
// R13 (0x0D) - Clock Control 2
//
pub const WM9081_CLK_SYS_RATE_MASK: c_uint = 0x00F0  /* CLK_SYS_RATE - [7:4] */;

pub const WM9081_SAMPLE_RATE_MASK: c_uint = 0x000F  /* SAMPLE_RATE - [3:0] */;

//
// R14 (0x0E) - Clock Control 3
//
pub const WM9081_CLK_SRC_SEL: c_uint = 0x2000  /* CLK_SRC_SEL */;
pub const WM9081_CLK_SRC_SEL_MASK: c_uint = 0x2000  /* CLK_SRC_SEL */;

pub const WM9081_CLK_OP_ENA: c_uint = 0x0020  /* CLK_OP_ENA */;
pub const WM9081_CLK_OP_ENA_MASK: c_uint = 0x0020  /* CLK_OP_ENA */;

pub const WM9081_CLK_TO_ENA: c_uint = 0x0004  /* CLK_TO_ENA */;
pub const WM9081_CLK_TO_ENA_MASK: c_uint = 0x0004  /* CLK_TO_ENA */;

pub const WM9081_CLK_DSP_ENA: c_uint = 0x0002  /* CLK_DSP_ENA */;
pub const WM9081_CLK_DSP_ENA_MASK: c_uint = 0x0002  /* CLK_DSP_ENA */;

pub const WM9081_CLK_SYS_ENA: c_uint = 0x0001  /* CLK_SYS_ENA */;
pub const WM9081_CLK_SYS_ENA_MASK: c_uint = 0x0001  /* CLK_SYS_ENA */;

//
// R16 (0x10) - FLL Control 1
//
pub const WM9081_FLL_HOLD: c_uint = 0x0008  /* FLL_HOLD */;
pub const WM9081_FLL_HOLD_MASK: c_uint = 0x0008  /* FLL_HOLD */;

pub const WM9081_FLL_FRAC: c_uint = 0x0004  /* FLL_FRAC */;
pub const WM9081_FLL_FRAC_MASK: c_uint = 0x0004  /* FLL_FRAC */;

pub const WM9081_FLL_ENA: c_uint = 0x0001  /* FLL_ENA */;
pub const WM9081_FLL_ENA_MASK: c_uint = 0x0001  /* FLL_ENA */;

//
// R17 (0x11) - FLL Control 2
//
pub const WM9081_FLL_OUTDIV_MASK: c_uint = 0x0700  /* FLL_OUTDIV - [10:8] */;

pub const WM9081_FLL_CTRL_RATE_MASK: c_uint = 0x0070  /* FLL_CTRL_RATE - [6:4] */;

pub const WM9081_FLL_FRATIO_MASK: c_uint = 0x0007  /* FLL_FRATIO - [2:0] */;

//
// R18 (0x12) - FLL Control 3
//
pub const WM9081_FLL_K_MASK: c_uint = 0xFFFF  /* FLL_K - [15:0] */;

//
// R19 (0x13) - FLL Control 4
//
pub const WM9081_FLL_N_MASK: c_uint = 0x7FE0  /* FLL_N - [14:5] */;

pub const WM9081_FLL_GAIN_MASK: c_uint = 0x000F  /* FLL_GAIN - [3:0] */;

//
// R20 (0x14) - FLL Control 5
//
pub const WM9081_FLL_CLK_REF_DIV_MASK: c_uint = 0x0018  /* FLL_CLK_REF_DIV - [4:3] */;

pub const WM9081_FLL_CLK_SRC_MASK: c_uint = 0x0003  /* FLL_CLK_SRC - [1:0] */;

//
// R22 (0x16) - Audio Interface 1
//
pub const WM9081_AIFDAC_CHAN: c_uint = 0x0040  /* AIFDAC_CHAN */;
pub const WM9081_AIFDAC_CHAN_MASK: c_uint = 0x0040  /* AIFDAC_CHAN */;

pub const WM9081_AIFDAC_TDM_SLOT_MASK: c_uint = 0x0030  /* AIFDAC_TDM_SLOT - [5:4] */;

pub const WM9081_AIFDAC_TDM_MODE_MASK: c_uint = 0x000C  /* AIFDAC_TDM_MODE - [3:2] */;

pub const WM9081_DAC_COMP: c_uint = 0x0002  /* DAC_COMP */;
pub const WM9081_DAC_COMP_MASK: c_uint = 0x0002  /* DAC_COMP */;

pub const WM9081_DAC_COMPMODE: c_uint = 0x0001  /* DAC_COMPMODE */;
pub const WM9081_DAC_COMPMODE_MASK: c_uint = 0x0001  /* DAC_COMPMODE */;

//
// R23 (0x17) - Audio Interface 2
//
pub const WM9081_AIF_TRIS: c_uint = 0x0200  /* AIF_TRIS */;
pub const WM9081_AIF_TRIS_MASK: c_uint = 0x0200  /* AIF_TRIS */;

pub const WM9081_DAC_DAT_INV: c_uint = 0x0100  /* DAC_DAT_INV */;
pub const WM9081_DAC_DAT_INV_MASK: c_uint = 0x0100  /* DAC_DAT_INV */;

pub const WM9081_AIF_BCLK_INV: c_uint = 0x0080  /* AIF_BCLK_INV */;
pub const WM9081_AIF_BCLK_INV_MASK: c_uint = 0x0080  /* AIF_BCLK_INV */;

pub const WM9081_BCLK_DIR: c_uint = 0x0040  /* BCLK_DIR */;
pub const WM9081_BCLK_DIR_MASK: c_uint = 0x0040  /* BCLK_DIR */;

pub const WM9081_LRCLK_DIR: c_uint = 0x0020  /* LRCLK_DIR */;
pub const WM9081_LRCLK_DIR_MASK: c_uint = 0x0020  /* LRCLK_DIR */;

pub const WM9081_AIF_LRCLK_INV: c_uint = 0x0010  /* AIF_LRCLK_INV */;
pub const WM9081_AIF_LRCLK_INV_MASK: c_uint = 0x0010  /* AIF_LRCLK_INV */;

pub const WM9081_AIF_WL_MASK: c_uint = 0x000C  /* AIF_WL - [3:2] */;

pub const WM9081_AIF_FMT_MASK: c_uint = 0x0003  /* AIF_FMT - [1:0] */;

//
// R24 (0x18) - Audio Interface 3
//
pub const WM9081_BCLK_DIV_MASK: c_uint = 0x001F  /* BCLK_DIV - [4:0] */;

//
// R25 (0x19) - Audio Interface 4
//
pub const WM9081_LRCLK_RATE_MASK: c_uint = 0x07FF  /* LRCLK_RATE - [10:0] */;

//
// R26 (0x1A) - Interrupt Status
//
pub const WM9081_WSEQ_BUSY_EINT: c_uint = 0x0004  /* WSEQ_BUSY_EINT */;
pub const WM9081_WSEQ_BUSY_EINT_MASK: c_uint = 0x0004  /* WSEQ_BUSY_EINT */;

pub const WM9081_TSHUT_EINT: c_uint = 0x0001  /* TSHUT_EINT */;
pub const WM9081_TSHUT_EINT_MASK: c_uint = 0x0001  /* TSHUT_EINT */;

//
// R27 (0x1B) - Interrupt Status Mask
//
pub const WM9081_IM_WSEQ_BUSY_EINT: c_uint = 0x0004  /* IM_WSEQ_BUSY_EINT */;
pub const WM9081_IM_WSEQ_BUSY_EINT_MASK: c_uint = 0x0004  /* IM_WSEQ_BUSY_EINT */;

pub const WM9081_IM_TSHUT_EINT: c_uint = 0x0001  /* IM_TSHUT_EINT */;
pub const WM9081_IM_TSHUT_EINT_MASK: c_uint = 0x0001  /* IM_TSHUT_EINT */;

//
// R28 (0x1C) - Interrupt Polarity
//
pub const WM9081_TSHUT_INV: c_uint = 0x0001  /* TSHUT_INV */;
pub const WM9081_TSHUT_INV_MASK: c_uint = 0x0001  /* TSHUT_INV */;

//
// R29 (0x1D) - Interrupt Control
//
pub const WM9081_IRQ_POL: c_uint = 0x8000  /* IRQ_POL */;
pub const WM9081_IRQ_POL_MASK: c_uint = 0x8000  /* IRQ_POL */;

pub const WM9081_IRQ_OP_CTRL: c_uint = 0x0001  /* IRQ_OP_CTRL */;
pub const WM9081_IRQ_OP_CTRL_MASK: c_uint = 0x0001  /* IRQ_OP_CTRL */;

//
// R30 (0x1E) - DAC Digital 1
//
pub const WM9081_DAC_VOL_MASK: c_uint = 0x00FF  /* DAC_VOL - [7:0] */;

//
// R31 (0x1F) - DAC Digital 2
//
pub const WM9081_DAC_MUTERATE: c_uint = 0x0400  /* DAC_MUTERATE */;
pub const WM9081_DAC_MUTERATE_MASK: c_uint = 0x0400  /* DAC_MUTERATE */;

pub const WM9081_DAC_MUTEMODE: c_uint = 0x0200  /* DAC_MUTEMODE */;
pub const WM9081_DAC_MUTEMODE_MASK: c_uint = 0x0200  /* DAC_MUTEMODE */;

pub const WM9081_DAC_MUTE: c_uint = 0x0008  /* DAC_MUTE */;
pub const WM9081_DAC_MUTE_MASK: c_uint = 0x0008  /* DAC_MUTE */;

pub const WM9081_DEEMPH_MASK: c_uint = 0x0006  /* DEEMPH - [2:1] */;

//
// R32 (0x20) - DRC 1
//
pub const WM9081_DRC_ENA: c_uint = 0x8000  /* DRC_ENA */;
pub const WM9081_DRC_ENA_MASK: c_uint = 0x8000  /* DRC_ENA */;

pub const WM9081_DRC_STARTUP_GAIN_MASK: c_uint = 0x07C0  /* DRC_STARTUP_GAIN - [10:6] */;

pub const WM9081_DRC_FF_DLY: c_uint = 0x0020  /* DRC_FF_DLY */;
pub const WM9081_DRC_FF_DLY_MASK: c_uint = 0x0020  /* DRC_FF_DLY */;

pub const WM9081_DRC_QR: c_uint = 0x0004  /* DRC_QR */;
pub const WM9081_DRC_QR_MASK: c_uint = 0x0004  /* DRC_QR */;

pub const WM9081_DRC_ANTICLIP: c_uint = 0x0002  /* DRC_ANTICLIP */;
pub const WM9081_DRC_ANTICLIP_MASK: c_uint = 0x0002  /* DRC_ANTICLIP */;

//
// R33 (0x21) - DRC 2
//
pub const WM9081_DRC_ATK_MASK: c_uint = 0xF000  /* DRC_ATK - [15:12] */;

pub const WM9081_DRC_DCY_MASK: c_uint = 0x0F00  /* DRC_DCY - [11:8] */;

pub const WM9081_DRC_QR_THR_MASK: c_uint = 0x00C0  /* DRC_QR_THR - [7:6] */;

pub const WM9081_DRC_QR_DCY_MASK: c_uint = 0x0030  /* DRC_QR_DCY - [5:4] */;

pub const WM9081_DRC_MINGAIN_MASK: c_uint = 0x000C  /* DRC_MINGAIN - [3:2] */;

pub const WM9081_DRC_MAXGAIN_MASK: c_uint = 0x0003  /* DRC_MAXGAIN - [1:0] */;

//
// R34 (0x22) - DRC 3
//
pub const WM9081_DRC_HI_COMP_MASK: c_uint = 0x0038  /* DRC_HI_COMP - [5:3] */;

pub const WM9081_DRC_LO_COMP_MASK: c_uint = 0x0007  /* DRC_LO_COMP - [2:0] */;

//
// R35 (0x23) - DRC 4
//
pub const WM9081_DRC_KNEE_IP_MASK: c_uint = 0x07E0  /* DRC_KNEE_IP - [10:5] */;

pub const WM9081_DRC_KNEE_OP_MASK: c_uint = 0x001F  /* DRC_KNEE_OP - [4:0] */;

//
// R38 (0x26) - Write Sequencer 1
//
pub const WM9081_WSEQ_ENA: c_uint = 0x8000  /* WSEQ_ENA */;
pub const WM9081_WSEQ_ENA_MASK: c_uint = 0x8000  /* WSEQ_ENA */;

pub const WM9081_WSEQ_ABORT: c_uint = 0x0200  /* WSEQ_ABORT */;
pub const WM9081_WSEQ_ABORT_MASK: c_uint = 0x0200  /* WSEQ_ABORT */;

pub const WM9081_WSEQ_START: c_uint = 0x0100  /* WSEQ_START */;
pub const WM9081_WSEQ_START_MASK: c_uint = 0x0100  /* WSEQ_START */;

pub const WM9081_WSEQ_START_INDEX_MASK: c_uint = 0x007F  /* WSEQ_START_INDEX - [6:0] */;

//
// R39 (0x27) - Write Sequencer 2
//
pub const WM9081_WSEQ_CURRENT_INDEX_MASK: c_uint = 0x07F0  /* WSEQ_CURRENT_INDEX - [10:4] */;

pub const WM9081_WSEQ_BUSY: c_uint = 0x0001  /* WSEQ_BUSY */;
pub const WM9081_WSEQ_BUSY_MASK: c_uint = 0x0001  /* WSEQ_BUSY */;

//
// R40 (0x28) - MW Slave 1
//
pub const WM9081_SPI_CFG: c_uint = 0x0020  /* SPI_CFG */;
pub const WM9081_SPI_CFG_MASK: c_uint = 0x0020  /* SPI_CFG */;

pub const WM9081_SPI_4WIRE: c_uint = 0x0010  /* SPI_4WIRE */;
pub const WM9081_SPI_4WIRE_MASK: c_uint = 0x0010  /* SPI_4WIRE */;

pub const WM9081_ARA_ENA: c_uint = 0x0008  /* ARA_ENA */;
pub const WM9081_ARA_ENA_MASK: c_uint = 0x0008  /* ARA_ENA */;

pub const WM9081_AUTO_INC: c_uint = 0x0002  /* AUTO_INC */;
pub const WM9081_AUTO_INC_MASK: c_uint = 0x0002  /* AUTO_INC */;

//
// R42 (0x2A) - EQ 1
//
pub const WM9081_EQ_B1_GAIN_MASK: c_uint = 0xF800  /* EQ_B1_GAIN - [15:11] */;

pub const WM9081_EQ_B2_GAIN_MASK: c_uint = 0x07C0  /* EQ_B2_GAIN - [10:6] */;

pub const WM9081_EQ_B4_GAIN_MASK: c_uint = 0x003E  /* EQ_B4_GAIN - [5:1] */;

pub const WM9081_EQ_ENA: c_uint = 0x0001  /* EQ_ENA */;
pub const WM9081_EQ_ENA_MASK: c_uint = 0x0001  /* EQ_ENA */;

//
// R43 (0x2B) - EQ 2
//
pub const WM9081_EQ_B3_GAIN_MASK: c_uint = 0xF800  /* EQ_B3_GAIN - [15:11] */;

pub const WM9081_EQ_B5_GAIN_MASK: c_uint = 0x07C0  /* EQ_B5_GAIN - [10:6] */;

//
// R44 (0x2C) - EQ 3
//
pub const WM9081_EQ_B1_A_MASK: c_uint = 0xFFFF  /* EQ_B1_A - [15:0] */;

//
// R45 (0x2D) - EQ 4
//
pub const WM9081_EQ_B1_B_MASK: c_uint = 0xFFFF  /* EQ_B1_B - [15:0] */;

//
// R46 (0x2E) - EQ 5
//
pub const WM9081_EQ_B1_PG_MASK: c_uint = 0xFFFF  /* EQ_B1_PG - [15:0] */;

//
// R47 (0x2F) - EQ 6
//
pub const WM9081_EQ_B2_A_MASK: c_uint = 0xFFFF  /* EQ_B2_A - [15:0] */;

//
// R48 (0x30) - EQ 7
//
pub const WM9081_EQ_B2_B_MASK: c_uint = 0xFFFF  /* EQ_B2_B - [15:0] */;

//
// R49 (0x31) - EQ 8
//
pub const WM9081_EQ_B2_C_MASK: c_uint = 0xFFFF  /* EQ_B2_C - [15:0] */;

//
// R50 (0x32) - EQ 9
//
pub const WM9081_EQ_B2_PG_MASK: c_uint = 0xFFFF  /* EQ_B2_PG - [15:0] */;

//
// R51 (0x33) - EQ 10
//
pub const WM9081_EQ_B4_A_MASK: c_uint = 0xFFFF  /* EQ_B4_A - [15:0] */;

//
// R52 (0x34) - EQ 11
//
pub const WM9081_EQ_B4_B_MASK: c_uint = 0xFFFF  /* EQ_B4_B - [15:0] */;

//
// R53 (0x35) - EQ 12
//
pub const WM9081_EQ_B4_C_MASK: c_uint = 0xFFFF  /* EQ_B4_C - [15:0] */;

//
// R54 (0x36) - EQ 13
//
pub const WM9081_EQ_B4_PG_MASK: c_uint = 0xFFFF  /* EQ_B4_PG - [15:0] */;

//
// R55 (0x37) - EQ 14
//
pub const WM9081_EQ_B3_A_MASK: c_uint = 0xFFFF  /* EQ_B3_A - [15:0] */;

//
// R56 (0x38) - EQ 15
//
pub const WM9081_EQ_B3_B_MASK: c_uint = 0xFFFF  /* EQ_B3_B - [15:0] */;

//
// R57 (0x39) - EQ 16
//
pub const WM9081_EQ_B3_C_MASK: c_uint = 0xFFFF  /* EQ_B3_C - [15:0] */;

//
// R58 (0x3A) - EQ 17
//
pub const WM9081_EQ_B3_PG_MASK: c_uint = 0xFFFF  /* EQ_B3_PG - [15:0] */;

//
// R59 (0x3B) - EQ 18
//
pub const WM9081_EQ_B5_A_MASK: c_uint = 0xFFFF  /* EQ_B5_A - [15:0] */;

//
// R60 (0x3C) - EQ 19
//
pub const WM9081_EQ_B5_B_MASK: c_uint = 0xFFFF  /* EQ_B5_B - [15:0] */;

//
// R61 (0x3D) - EQ 20
//
pub const WM9081_EQ_B5_PG_MASK: c_uint = 0xFFFF  /* EQ_B5_PG - [15:0] */;

