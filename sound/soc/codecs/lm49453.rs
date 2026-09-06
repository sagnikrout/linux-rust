//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/lm49453.h
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
// lm49453.h  -  LM49453 ALSA Soc Audio drive
//
// Copyright (c) 2012  Texas Instruments, Inc
//

// LM49453_P0 register space for page0
pub const LM49453_P0_PMC_SETUP_REG: c_uint = 0x00;
pub const LM49453_P0_PLL_CLK_SEL1_REG: c_uint = 0x01;
pub const LM49453_P0_PLL_CLK_SEL2_REG: c_uint = 0x02;
pub const LM49453_P0_PMC_CLK_DIV_REG: c_uint = 0x03;
pub const LM49453_P0_HSDET_CLK_DIV_REG: c_uint = 0x04;
pub const LM49453_P0_DMIC_CLK_DIV_REG: c_uint = 0x05;
pub const LM49453_P0_ADC_CLK_DIV_REG: c_uint = 0x06;
pub const LM49453_P0_DAC_OT_CLK_DIV_REG: c_uint = 0x07;
pub const LM49453_P0_PLL_HF_M_REG: c_uint = 0x08;
pub const LM49453_P0_PLL_LF_M_REG: c_uint = 0x09;
pub const LM49453_P0_PLL_NL_REG: c_uint = 0x0A;
pub const LM49453_P0_PLL_N_MODL_REG: c_uint = 0x0B;
pub const LM49453_P0_PLL_N_MODH_REG: c_uint = 0x0C;
pub const LM49453_P0_PLL_P1_REG: c_uint = 0x0D;
pub const LM49453_P0_PLL_P2_REG: c_uint = 0x0E;
pub const LM49453_P0_FLL_REF_FREQL_REG: c_uint = 0x0F;
pub const LM49453_P0_FLL_REF_FREQH_REG: c_uint = 0x10;
pub const LM49453_P0_VCO_TARGETLL_REG: c_uint = 0x11;
pub const LM49453_P0_VCO_TARGETLH_REG: c_uint = 0x12;
pub const LM49453_P0_VCO_TARGETHL_REG: c_uint = 0x13;
pub const LM49453_P0_VCO_TARGETHH_REG: c_uint = 0x14;
pub const LM49453_P0_PLL_CONFIG_REG: c_uint = 0x15;
pub const LM49453_P0_DAC_CLK_SEL_REG: c_uint = 0x16;
pub const LM49453_P0_DAC_HP_CLK_DIV_REG: c_uint = 0x17;
// Analog Mixer Input Stages
pub const LM49453_P0_MICL_REG: c_uint = 0x20;
pub const LM49453_P0_MICR_REG: c_uint = 0x21;
pub const LM49453_P0_EP_REG: c_uint = 0x24;
pub const LM49453_P0_DIS_PKVL_FB_REG: c_uint = 0x25;
// Analog Mixer Output Stages
pub const LM49453_P0_ANALOG_MIXER_ADC_REG: c_uint = 0x2E;
// ADC or DAC
pub const LM49453_P0_ADC_DSP_REG: c_uint = 0x30;
pub const LM49453_P0_DAC_DSP_REG: c_uint = 0x31;
// EFFECTS ENABLES
pub const LM49453_P0_ADC_FX_ENABLES_REG: c_uint = 0x33;
// GPIO
pub const LM49453_P0_GPIO1_REG: c_uint = 0x38;
pub const LM49453_P0_GPIO2_REG: c_uint = 0x39;
pub const LM49453_P0_GPIO3_REG: c_uint = 0x3A;
pub const LM49453_P0_HAP_CTL_REG: c_uint = 0x3B;
pub const LM49453_P0_HAP_FREQ_PROG_LEFTL_REG: c_uint = 0x3C;
pub const LM49453_P0_HAP_FREQ_PROG_LEFTH_REG: c_uint = 0x3D;
pub const LM49453_P0_HAP_FREQ_PROG_RIGHTL_REG: c_uint = 0x3E;
pub const LM49453_P0_HAP_FREQ_PROG_RIGHTH_REG: c_uint = 0x3F;
// DIGITAL MIXER
pub const LM49453_P0_DMIX_CLK_SEL_REG: c_uint = 0x40;
pub const LM49453_P0_PORT1_RX_LVL1_REG: c_uint = 0x41;
pub const LM49453_P0_PORT1_RX_LVL2_REG: c_uint = 0x42;
pub const LM49453_P0_PORT2_RX_LVL_REG: c_uint = 0x43;
pub const LM49453_P0_PORT1_TX1_REG: c_uint = 0x44;
pub const LM49453_P0_PORT1_TX2_REG: c_uint = 0x45;
pub const LM49453_P0_PORT1_TX3_REG: c_uint = 0x46;
pub const LM49453_P0_PORT1_TX4_REG: c_uint = 0x47;
pub const LM49453_P0_PORT1_TX5_REG: c_uint = 0x48;
pub const LM49453_P0_PORT1_TX6_REG: c_uint = 0x49;
pub const LM49453_P0_PORT1_TX7_REG: c_uint = 0x4A;
pub const LM49453_P0_PORT1_TX8_REG: c_uint = 0x4B;
pub const LM49453_P0_PORT2_TX1_REG: c_uint = 0x4C;
pub const LM49453_P0_PORT2_TX2_REG: c_uint = 0x4D;
pub const LM49453_P0_STN_SEL_REG: c_uint = 0x4F;
pub const LM49453_P0_DACHPL1_REG: c_uint = 0x50;
pub const LM49453_P0_DACHPL2_REG: c_uint = 0x51;
pub const LM49453_P0_DACHPR1_REG: c_uint = 0x52;
pub const LM49453_P0_DACHPR2_REG: c_uint = 0x53;
pub const LM49453_P0_DACLOL1_REG: c_uint = 0x54;
pub const LM49453_P0_DACLOL2_REG: c_uint = 0x55;
pub const LM49453_P0_DACLOR1_REG: c_uint = 0x56;
pub const LM49453_P0_DACLOR2_REG: c_uint = 0x57;
pub const LM49453_P0_DACLSL1_REG: c_uint = 0x58;
pub const LM49453_P0_DACLSL2_REG: c_uint = 0x59;
pub const LM49453_P0_DACLSR1_REG: c_uint = 0x5A;
pub const LM49453_P0_DACLSR2_REG: c_uint = 0x5B;
pub const LM49453_P0_DACHAL1_REG: c_uint = 0x5C;
pub const LM49453_P0_DACHAL2_REG: c_uint = 0x5D;
pub const LM49453_P0_DACHAR1_REG: c_uint = 0x5E;
pub const LM49453_P0_DACHAR2_REG: c_uint = 0x5F;
// AUDIO PORT 1 (TDM)
pub const LM49453_P0_AUDIO_PORT1_BASIC_REG: c_uint = 0x60;
pub const LM49453_P0_AUDIO_PORT1_CLK_GEN1_REG: c_uint = 0x61;
pub const LM49453_P0_AUDIO_PORT1_CLK_GEN2_REG: c_uint = 0x62;
pub const LM49453_P0_AUDIO_PORT1_CLK_GEN3_REG: c_uint = 0x63;
pub const LM49453_P0_AUDIO_PORT1_SYNC_RATE_REG: c_uint = 0x64;
pub const LM49453_P0_AUDIO_PORT1_SYNC_SDO_SETUP_REG: c_uint = 0x65;
pub const LM49453_P0_AUDIO_PORT1_DATA_WIDTH_REG: c_uint = 0x66;
pub const LM49453_P0_AUDIO_PORT1_RX_MSB_REG: c_uint = 0x67;
pub const LM49453_P0_AUDIO_PORT1_TX_MSB_REG: c_uint = 0x68;
pub const LM49453_P0_AUDIO_PORT1_TDM_CHANNELS_REG: c_uint = 0x69;
// AUDIO PORT 2
pub const LM49453_P0_AUDIO_PORT2_BASIC_REG: c_uint = 0x6A;
pub const LM49453_P0_AUDIO_PORT2_CLK_GEN1_REG: c_uint = 0x6B;
pub const LM49453_P0_AUDIO_PORT2_CLK_GEN2_REG: c_uint = 0x6C;
pub const LM49453_P0_AUDIO_PORT2_SYNC_GEN_REG: c_uint = 0x6D;
pub const LM49453_P0_AUDIO_PORT2_DATA_WIDTH_REG: c_uint = 0x6E;
pub const LM49453_P0_AUDIO_PORT2_RX_MODE_REG: c_uint = 0x6F;
pub const LM49453_P0_AUDIO_PORT2_TX_MODE_REG: c_uint = 0x70;
// SAMPLE RATE
pub const LM49453_P0_PORT1_SR_LSB_REG: c_uint = 0x79;
pub const LM49453_P0_PORT1_SR_MSB_REG: c_uint = 0x7A;
pub const LM49453_P0_PORT2_SR_LSB_REG: c_uint = 0x7B;
pub const LM49453_P0_PORT2_SR_MSB_REG: c_uint = 0x7C;
// EFFECTS - HPFs
pub const LM49453_P0_HPF_REG: c_uint = 0x80;
// EFFECTS ADC ALC
pub const LM49453_P0_ADC_ALC1_REG: c_uint = 0x82;
pub const LM49453_P0_ADC_ALC2_REG: c_uint = 0x83;
pub const LM49453_P0_ADC_ALC3_REG: c_uint = 0x84;
pub const LM49453_P0_ADC_ALC4_REG: c_uint = 0x85;
pub const LM49453_P0_ADC_ALC5_REG: c_uint = 0x86;
pub const LM49453_P0_ADC_ALC6_REG: c_uint = 0x87;
pub const LM49453_P0_ADC_ALC7_REG: c_uint = 0x88;
pub const LM49453_P0_ADC_ALC8_REG: c_uint = 0x89;
pub const LM49453_P0_DMIC1_LEVELL_REG: c_uint = 0x8A;
pub const LM49453_P0_DMIC1_LEVELR_REG: c_uint = 0x8B;
pub const LM49453_P0_DMIC2_LEVELL_REG: c_uint = 0x8C;
pub const LM49453_P0_DMIC2_LEVELR_REG: c_uint = 0x8D;
pub const LM49453_P0_ADC_LEVELL_REG: c_uint = 0x8E;
pub const LM49453_P0_ADC_LEVELR_REG: c_uint = 0x8F;
pub const LM49453_P0_DAC_HP_LEVELL_REG: c_uint = 0x90;
pub const LM49453_P0_DAC_HP_LEVELR_REG: c_uint = 0x91;
pub const LM49453_P0_DAC_LO_LEVELL_REG: c_uint = 0x92;
pub const LM49453_P0_DAC_LO_LEVELR_REG: c_uint = 0x93;
pub const LM49453_P0_DAC_LS_LEVELL_REG: c_uint = 0x94;
pub const LM49453_P0_DAC_LS_LEVELR_REG: c_uint = 0x95;
pub const LM49453_P0_DAC_HA_LEVELL_REG: c_uint = 0x96;
pub const LM49453_P0_DAC_HA_LEVELR_REG: c_uint = 0x97;
pub const LM49453_P0_SOFT_MUTE_REG: c_uint = 0x98;
pub const LM49453_P0_DMIC_MUTE_CFG_REG: c_uint = 0x99;
pub const LM49453_P0_ADC_MUTE_CFG_REG: c_uint = 0x9A;
pub const LM49453_P0_DAC_MUTE_CFG_REG: c_uint = 0x9B;
// DIGITAL MIC1
pub const LM49453_P0_DIGITAL_MIC1_CONFIG_REG: c_uint = 0xB0;
pub const LM49453_P0_DIGITAL_MIC1_DATA_DELAYL_REG: c_uint = 0xB1;
pub const LM49453_P0_DIGITAL_MIC1_DATA_DELAYR_REG: c_uint = 0xB2;
// DIGITAL MIC2
pub const LM49453_P0_DIGITAL_MIC2_CONFIG_REG: c_uint = 0xB3;
pub const LM49453_P0_DIGITAL_MIC2_DATA_DELAYL_REG: c_uint = 0xB4;
pub const LM49453_P0_DIGITAL_MIC2_DATA_DELAYR_REG: c_uint = 0xB5;
// ADC DECIMATOR
pub const LM49453_P0_ADC_DECIMATOR_REG: c_uint = 0xB6;
// DAC CONFIGURE
pub const LM49453_P0_DAC_CONFIG_REG: c_uint = 0xB7;
// SIDETONE
pub const LM49453_P0_STN_VOL_ADCL_REG: c_uint = 0xB8;
pub const LM49453_P0_STN_VOL_ADCR_REG: c_uint = 0xB9;
pub const LM49453_P0_STN_VOL_DMIC1L_REG: c_uint = 0xBA;
pub const LM49453_P0_STN_VOL_DMIC1R_REG: c_uint = 0xBB;
pub const LM49453_P0_STN_VOL_DMIC2L_REG: c_uint = 0xBC;
pub const LM49453_P0_STN_VOL_DMIC2R_REG: c_uint = 0xBD;
// ADC/DAC CLIPPING MONITORS (Read Only/Write to Clear)
pub const LM49453_P0_ADC_DEC_CLIP_REG: c_uint = 0xC2;
pub const LM49453_P0_ADC_HPF_CLIP_REG: c_uint = 0xC3;
pub const LM49453_P0_ADC_LVL_CLIP_REG: c_uint = 0xC4;
pub const LM49453_P0_DAC_LVL_CLIP_REG: c_uint = 0xC5;
// ADC ALC EFFECT MONITORS (Read Only)
pub const LM49453_P0_ADC_LVLMONL_REG: c_uint = 0xC8;
pub const LM49453_P0_ADC_LVLMONR_REG: c_uint = 0xC9;
pub const LM49453_P0_ADC_ALCMONL_REG: c_uint = 0xCA;
pub const LM49453_P0_ADC_ALCMONR_REG: c_uint = 0xCB;
pub const LM49453_P0_ADC_MUTED_REG: c_uint = 0xCC;
pub const LM49453_P0_DAC_MUTED_REG: c_uint = 0xCD;
// HEADSET DETECT
pub const LM49453_P0_HSD_PPB_LONG_CNT_LIMITL_REG: c_uint = 0xD0;
pub const LM49453_P0_HSD_PPB_LONG_CNT_LIMITR_REG: c_uint = 0xD1;
pub const LM49453_P0_HSD_PIN3_4_EX_LOOP_CNT_LIMITL_REG: c_uint = 0xD2;
pub const LM49453_P0_HSD_PIN3_4_EX_LOOP_CNT_LIMITH_REG: c_uint = 0xD3;
pub const LM49453_P0_HSD_TIMEOUT1_REG: c_uint = 0xD4;
pub const LM49453_P0_HSD_TIMEOUT2_REG: c_uint = 0xD5;
pub const LM49453_P0_HSD_TIMEOUT3_REG: c_uint = 0xD6;
pub const LM49453_P0_HSD_PIN3_4_CFG_REG: c_uint = 0xD7;
pub const LM49453_P0_HSD_IRQ1_REG: c_uint = 0xD8;
pub const LM49453_P0_HSD_IRQ2_REG: c_uint = 0xD9;
pub const LM49453_P0_HSD_IRQ3_REG: c_uint = 0xDA;
pub const LM49453_P0_HSD_IRQ4_REG: c_uint = 0xDB;
pub const LM49453_P0_HSD_IRQ_MASK1_REG: c_uint = 0xDC;
pub const LM49453_P0_HSD_IRQ_MASK2_REG: c_uint = 0xDD;
pub const LM49453_P0_HSD_IRQ_MASK3_REG: c_uint = 0xDE;
pub const LM49453_P0_HSD_R_HPLL_REG: c_uint = 0xE0;
pub const LM49453_P0_HSD_R_HPLH_REG: c_uint = 0xE1;
pub const LM49453_P0_HSD_R_HPLU_REG: c_uint = 0xE2;
pub const LM49453_P0_HSD_R_HPRL_REG: c_uint = 0xE3;
pub const LM49453_P0_HSD_R_HPRH_REG: c_uint = 0xE4;
pub const LM49453_P0_HSD_R_HPRU_REG: c_uint = 0xE5;
pub const LM49453_P0_HSD_VEL_L_FINALL_REG: c_uint = 0xE6;
pub const LM49453_P0_HSD_VEL_L_FINALH_REG: c_uint = 0xE7;
pub const LM49453_P0_HSD_VEL_L_FINALU_REG: c_uint = 0xE8;
pub const LM49453_P0_HSD_RO_FINALL_REG: c_uint = 0xE9;
pub const LM49453_P0_HSD_RO_FINALH_REG: c_uint = 0xEA;
pub const LM49453_P0_HSD_RO_FINALU_REG: c_uint = 0xEB;
pub const LM49453_P0_HSD_VMIC_BIAS_FINALL_REG: c_uint = 0xEC;
pub const LM49453_P0_HSD_VMIC_BIAS_FINALH_REG: c_uint = 0xED;
pub const LM49453_P0_HSD_VMIC_BIAS_FINALU_REG: c_uint = 0xEE;
pub const LM49453_P0_HSD_PIN_CONFIG_REG: c_uint = 0xEF;
pub const LM49453_P0_HSD_PLUG_DETECT_BB_IRQ_STATUS1_REG: c_uint = 0xF1;
pub const LM49453_P0_HSD_PLUG_DETECT_BB_IRQ_STATUS2_REG: c_uint = 0xF2;
pub const LM49453_P0_HSD_PLUG_DETECT_BB_IRQ_STATUS3_REG: c_uint = 0xF3;
pub const LM49453_P0_HSD_PLUG_DETECT_BB_IRQ_STATEL_REG: c_uint = 0xF4;
pub const LM49453_P0_HSD_PLUG_DETECT_BB_IRQ_STATEH_REG: c_uint = 0xF5;
// I/O PULLDOWN CONFIG
pub const LM49453_P0_PULL_CONFIG1_REG: c_uint = 0xF8;
pub const LM49453_P0_PULL_CONFIG2_REG: c_uint = 0xF9;
pub const LM49453_P0_PULL_CONFIG3_REG: c_uint = 0xFA;
// RESET
pub const LM49453_P0_RESET_REG: c_uint = 0xFE;
// PAGE
pub const LM49453_PAGE_REG: c_uint = 0xFF;

// LM49453_P0_PMC_SETUP_REG (0x00h)

// Chip Enable bits
pub const LM49453_CHIP_EN_SHUTDOWN: c_uint = 0x00;
pub const LM49453_CHIP_EN: c_uint = 0x01;
pub const LM49453_CHIP_EN_HSD_DETECT: c_uint = 0x02;
pub const LM49453_CHIP_EN_INVALID_HSD: c_uint = 0x03;
// LM49453_P0_PLL_CLK_SEL1_REG (0x01h)
pub const LM49453_CLK_SEL1_MCLK_SEL: c_uint = 0x11;
pub const LM49453_CLK_SEL1_RTC_SEL: c_uint = 0x11;
pub const LM49453_CLK_SEL1_PORT1_SEL: c_uint = 0x10;
pub const LM49453_CLK_SEL1_PORT2_SEL: c_uint = 0x11;
// LM49453_P0_PLL_CLK_SEL2_REG (0x02h)
pub const LM49453_CLK_SEL2_ADC_CLK_SEL: c_uint = 0x38;
// LM49453_P0_FLL_REF_FREQL_REG (0x0F)
pub const LM49453_FLL_REF_FREQ_VAL: c_uint = 0x8ca0001;
// LM49453_P0_VCO_TARGETLL_REG (0x11)
pub const LM49453_VCO_TARGET_VAL: c_uint = 0x8ca0001;
// LM49453_P0_ADC_DSP_REG (0x30h)

pub const LM49453_ADC_DSP_MUTE_ALL: c_uint = 0x3F;
// LM49453_P0_DAC_DSP_REG (0x31h)
pub const LM49453_DAC_DSP_MUTE_ALL: c_uint = 0xFF;
// LM49453_P0_AUDIO_PORT1_BASIC_REG (0x60h)

// LM49453_P0_RESET_REG (0xFEh)

// Page select register bits (0xFF)
pub const LM49453_PAGE0_SELECT: c_uint = 0x0;
pub const LM49453_PAGE1_SELECT: c_uint = 0x1;
// LM49453_P0_HSD_PIN3_4_CFG_REG (Jack Pin config - 0xD7)
pub const LM49453_JACK_DISABLE: c_uint = 0x00;
pub const LM49453_JACK_CONFIG1: c_uint = 0x01;
pub const LM49453_JACK_CONFIG2: c_uint = 0x02;
pub const LM49453_JACK_CONFIG3: c_uint = 0x03;
pub const LM49453_JACK_CONFIG4: c_uint = 0x04;
pub const LM49453_JACK_CONFIG5: c_uint = 0x05;
// Page 1 REGISTERS
// SIDETONE
pub const LM49453_P1_SIDETONE_SA0L_REG: c_uint = 0x80;
pub const LM49453_P1_SIDETONE_SA0H_REG: c_uint = 0x81;
pub const LM49453_P1_SIDETONE_SAB0U_REG: c_uint = 0x82;
pub const LM49453_P1_SIDETONE_SB0L_REG: c_uint = 0x83;
pub const LM49453_P1_SIDETONE_SB0H_REG: c_uint = 0x84;
pub const LM49453_P1_SIDETONE_SH0L_REG: c_uint = 0x85;
pub const LM49453_P1_SIDETONE_SH0H_REG: c_uint = 0x86;
pub const LM49453_P1_SIDETONE_SH0U_REG: c_uint = 0x87;
pub const LM49453_P1_SIDETONE_SA1L_REG: c_uint = 0x88;
pub const LM49453_P1_SIDETONE_SA1H_REG: c_uint = 0x89;
pub const LM49453_P1_SIDETONE_SAB1U_REG: c_uint = 0x8A;
pub const LM49453_P1_SIDETONE_SB1L_REG: c_uint = 0x8B;
pub const LM49453_P1_SIDETONE_SB1H_REG: c_uint = 0x8C;
pub const LM49453_P1_SIDETONE_SH1L_REG: c_uint = 0x8D;
pub const LM49453_P1_SIDETONE_SH1H_REG: c_uint = 0x8E;
pub const LM49453_P1_SIDETONE_SH1U_REG: c_uint = 0x8F;
pub const LM49453_P1_SIDETONE_SA2L_REG: c_uint = 0x90;
pub const LM49453_P1_SIDETONE_SA2H_REG: c_uint = 0x91;
pub const LM49453_P1_SIDETONE_SAB2U_REG: c_uint = 0x92;
pub const LM49453_P1_SIDETONE_SB2L_REG: c_uint = 0x93;
pub const LM49453_P1_SIDETONE_SB2H_REG: c_uint = 0x94;
pub const LM49453_P1_SIDETONE_SH2L_REG: c_uint = 0x95;
pub const LM49453_P1_SIDETONE_SH2H_REG: c_uint = 0x96;
pub const LM49453_P1_SIDETONE_SH2U_REG: c_uint = 0x97;
pub const LM49453_P1_SIDETONE_SA3L_REG: c_uint = 0x98;
pub const LM49453_P1_SIDETONE_SA3H_REG: c_uint = 0x99;
pub const LM49453_P1_SIDETONE_SAB3U_REG: c_uint = 0x9A;
pub const LM49453_P1_SIDETONE_SB3L_REG: c_uint = 0x9B;
pub const LM49453_P1_SIDETONE_SB3H_REG: c_uint = 0x9C;
pub const LM49453_P1_SIDETONE_SH3L_REG: c_uint = 0x9D;
pub const LM49453_P1_SIDETONE_SH3H_REG: c_uint = 0x9E;
pub const LM49453_P1_SIDETONE_SH3U_REG: c_uint = 0x9F;
pub const LM49453_P1_SIDETONE_SA4L_REG: c_uint = 0xA0;
pub const LM49453_P1_SIDETONE_SA4H_REG: c_uint = 0xA1;
pub const LM49453_P1_SIDETONE_SAB4U_REG: c_uint = 0xA2;
pub const LM49453_P1_SIDETONE_SB4L_REG: c_uint = 0xA3;
pub const LM49453_P1_SIDETONE_SB4H_REG: c_uint = 0xA4;
pub const LM49453_P1_SIDETONE_SH4L_REG: c_uint = 0xA5;
pub const LM49453_P1_SIDETONE_SH4H_REG: c_uint = 0xA6;
pub const LM49453_P1_SIDETONE_SH4U_REG: c_uint = 0xA7;
pub const LM49453_P1_SIDETONE_SA5L_REG: c_uint = 0xA8;
pub const LM49453_P1_SIDETONE_SA5H_REG: c_uint = 0xA9;
pub const LM49453_P1_SIDETONE_SAB5U_REG: c_uint = 0xAA;
pub const LM49453_P1_SIDETONE_SB5L_REG: c_uint = 0xAB;
pub const LM49453_P1_SIDETONE_SB5H_REG: c_uint = 0xAC;
pub const LM49453_P1_SIDETONE_SH5L_REG: c_uint = 0xAD;
pub const LM49453_P1_SIDETONE_SH5H_REG: c_uint = 0xAE;
pub const LM49453_P1_SIDETONE_SH5U_REG: c_uint = 0xAF;
// CHARGE PUMP CONFIG
pub const LM49453_P1_CP_CONFIG1_REG: c_uint = 0xB0;
pub const LM49453_P1_CP_CONFIG2_REG: c_uint = 0xB1;
pub const LM49453_P1_CP_CONFIG3_REG: c_uint = 0xB2;
pub const LM49453_P1_CP_CONFIG4_REG: c_uint = 0xB3;
pub const LM49453_P1_CP_LA_VTH1L_REG: c_uint = 0xB4;
pub const LM49453_P1_CP_LA_VTH1M_REG: c_uint = 0xB5;
pub const LM49453_P1_CP_LA_VTH2L_REG: c_uint = 0xB6;
pub const LM49453_P1_CP_LA_VTH2M_REG: c_uint = 0xB7;
pub const LM49453_P1_CP_LA_VTH3L_REG: c_uint = 0xB8;
pub const LM49453_P1_CP_LA_VTH3H_REG: c_uint = 0xB9;
pub const LM49453_P1_CP_CLK_DIV_REG: c_uint = 0xBA;
// DAC
pub const LM49453_P1_DAC_CHOP_REG: c_uint = 0xC0;
pub const LM49453_CLK_SRC_MCLK: c_int = 1;
