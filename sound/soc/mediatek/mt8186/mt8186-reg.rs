//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8186/mt8186-reg.h
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
// mt8186-reg.h  --  Mediatek 8186 audio driver reg definition
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//
// reg bit enum
//
// R E G I S T E R       D E F I N I T I O N
//
// AUDIO_TOP_CON0
pub const RESERVED_SFT: c_int = 31;

pub const AHB_IDLE_EN_INT_SFT: c_int = 30;

pub const AHB_IDLE_EN_EXT_SFT: c_int = 29;

pub const PDN_NLE_SFT: c_int = 28;

pub const PDN_TML_SFT: c_int = 27;

pub const PDN_DAC_PREDIS_SFT: c_int = 26;

pub const PDN_DAC_SFT: c_int = 25;

pub const PDN_ADC_SFT: c_int = 24;

pub const PDN_TDM_CK_SFT: c_int = 20;

pub const PDN_APLL_TUNER_SFT: c_int = 19;

pub const PDN_APLL2_TUNER_SFT: c_int = 18;

pub const APB3_SEL_SFT: c_int = 14;

pub const APB_R2T_SFT: c_int = 13;

pub const APB_W2T_SFT: c_int = 12;

pub const PDN_24M_SFT: c_int = 9;

pub const PDN_22M_SFT: c_int = 8;

pub const PDN_AFE_SFT: c_int = 2;

// AUDIO_TOP_CON1
pub const PDN_3RD_DAC_HIRES_SFT: c_int = 31;

pub const PDN_3RD_DAC_TML_SFT: c_int = 30;

pub const PDN_3RD_DAC_PREDIS_SFT: c_int = 29;

pub const PDN_3RD_DAC_SFT: c_int = 28;

pub const I2S_SOFT_RST5_SFT: c_int = 22;

pub const PDN_ADDA6_ADC_HIRES_SFT: c_int = 21;

pub const PDN_ADDA6_ADC_SFT: c_int = 20;

pub const PDN_ADC_HIRES_TML_SFT: c_int = 17;

pub const PDN_ADC_HIRES_SFT: c_int = 16;

pub const PDN_DAC_HIRES_SFT: c_int = 15;

pub const PDN_GENERAL2_ASRC_SFT: c_int = 14;

pub const PDN_GENERAL1_ASRC_SFT: c_int = 13;

pub const PDN_CONNSYS_I2S_ASRC_SFT: c_int = 12;

pub const I2S4_BCLK_SW_CG_SFT: c_int = 7;

pub const I2S3_BCLK_SW_CG_SFT: c_int = 6;

pub const I2S2_BCLK_SW_CG_SFT: c_int = 5;

pub const I2S1_BCLK_SW_CG_SFT: c_int = 4;

pub const I2S_SOFT_RST2_SFT: c_int = 2;

pub const I2S_SOFT_RST_SFT: c_int = 1;

// AUDIO_TOP_CON3
pub const BUSY_SFT: c_int = 31;

pub const OS_DISABLE_SFT: c_int = 30;

pub const CG_DISABLE_SFT: c_int = 29;

pub const CLEAR_FLAG_SFT: c_int = 0;

// AFE_DAC_CON0
pub const VUL12_ON_SFT: c_int = 31;

pub const MOD_DAI_ON_SFT: c_int = 30;

pub const DAI_ON_SFT: c_int = 29;

pub const DAI2_ON_SFT: c_int = 28;

pub const VUL6_ON_SFT: c_int = 23;

pub const VUL5_ON_SFT: c_int = 22;

pub const VUL4_ON_SFT: c_int = 21;

pub const VUL3_ON_SFT: c_int = 20;

pub const VUL2_ON_SFT: c_int = 19;

pub const VUL_ON_SFT: c_int = 18;

pub const AWB2_ON_SFT: c_int = 17;

pub const AWB_ON_SFT: c_int = 16;

pub const DL12_ON_SFT: c_int = 15;

pub const DL8_ON_SFT: c_int = 11;

pub const DL7_ON_SFT: c_int = 10;

pub const DL6_ON_SFT: c_int = 9;

pub const DL5_ON_SFT: c_int = 8;

pub const DL4_ON_SFT: c_int = 7;

pub const DL3_ON_SFT: c_int = 6;

pub const DL2_ON_SFT: c_int = 5;

pub const DL1_ON_SFT: c_int = 4;

pub const AUDIO_AFE_ON_SFT: c_int = 0;

// AFE_DAC_MON
pub const AFE_ON_RETM_SFT: c_int = 0;

// AFE_I2S_CON
pub const BCK_NEG_EG_LATCH_SFT: c_int = 30;

pub const BCK_INV_SFT: c_int = 29;

pub const I2SIN_PAD_SEL_SFT: c_int = 28;

pub const I2S_LOOPBACK_SFT: c_int = 20;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;

pub const I2S1_HD_EN_SFT: c_int = 12;

pub const I2S_OUT_MODE_SFT: c_int = 8;

pub const INV_PAD_CTRL_SFT: c_int = 7;

pub const I2S_BYPSRC_SFT: c_int = 6;

pub const INV_LRCK_SFT: c_int = 5;

pub const I2S_FMT_SFT: c_int = 3;

pub const I2S_SRC_SFT: c_int = 2;

pub const I2S_WLEN_SFT: c_int = 1;

pub const I2S_EN_SFT: c_int = 0;

// AFE_I2S_CON1
pub const I2S2_LR_SWAP_SFT: c_int = 31;

pub const I2S2_SEL_O19_O20_SFT: c_int = 18;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;

pub const I2S2_SEL_O03_O04_SFT: c_int = 16;

pub const I2S2_HD_EN_SFT: c_int = 12;

pub const I2S2_OUT_MODE_SFT: c_int = 8;

pub const INV_LRCK_SFT: c_int = 5;

pub const I2S2_FMT_SFT: c_int = 3;

pub const I2S2_WLEN_SFT: c_int = 1;

pub const I2S2_EN_SFT: c_int = 0;

// AFE_I2S_CON2
pub const I2S3_LR_SWAP_SFT: c_int = 31;

pub const I2S3_UPDATE_WORD_SFT: c_int = 24;

pub const I2S3_BCK_INV_SFT: c_int = 23;

pub const I2S3_FPGA_BIT_TEST_SFT: c_int = 22;

pub const I2S3_FPGA_BIT_SFT: c_int = 21;

pub const I2S3_LOOPBACK_SFT: c_int = 20;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;

pub const I2S3_HD_EN_SFT: c_int = 12;

pub const I2S3_OUT_MODE_SFT: c_int = 8;

pub const I2S3_FMT_SFT: c_int = 3;

pub const I2S3_WLEN_SFT: c_int = 1;

pub const I2S3_EN_SFT: c_int = 0;

// AFE_I2S_CON3
pub const I2S4_LR_SWAP_SFT: c_int = 31;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;

pub const I2S4_HD_EN_SFT: c_int = 12;

pub const I2S4_OUT_MODE_SFT: c_int = 8;

pub const INV_LRCK_SFT: c_int = 5;

pub const I2S4_FMT_SFT: c_int = 3;

pub const I2S4_WLEN_SFT: c_int = 1;

pub const I2S4_EN_SFT: c_int = 0;

// AFE_I2S_CON4
pub const I2S_LOOPBACK_SFT: c_int = 20;
pub const I2S_LOOPBACK_MASK: c_uint = 0x1;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;
pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_MASK: c_uint = 0x1;

pub const INV_LRCK_SFT: c_int = 5;
pub const INV_LRCK_MASK: c_uint = 0x1;

// AFE_CONNSYS_I2S_CON
pub const BCK_NEG_EG_LATCH_SFT: c_int = 30;

pub const BCK_INV_SFT: c_int = 29;

pub const I2SIN_PAD_SEL_SFT: c_int = 28;

pub const I2S_LOOPBACK_SFT: c_int = 20;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;

pub const I2S_MODE_SFT: c_int = 8;

pub const INV_PAD_CTRL_SFT: c_int = 7;

pub const I2S_BYPSRC_SFT: c_int = 6;

pub const INV_LRCK_SFT: c_int = 5;

pub const I2S_FMT_SFT: c_int = 3;

pub const I2S_SRC_SFT: c_int = 2;

pub const I2S_WLEN_SFT: c_int = 1;

pub const I2S_EN_SFT: c_int = 0;

// AFE_ASRC_2CH_CON2
pub const CHSET_O16BIT_SFT: c_int = 19;

pub const CHSET_CLR_IIR_HISTORY_SFT: c_int = 17;

pub const CHSET_IS_MONO_SFT: c_int = 16;

pub const CHSET_IIR_EN_SFT: c_int = 11;

pub const CHSET_IIR_STAGE_SFT: c_int = 8;

pub const CHSET_STR_CLR_SFT: c_int = 5;

pub const CHSET_ON_SFT: c_int = 2;

pub const COEFF_SRAM_CTRL_SFT: c_int = 1;

pub const ASM_ON_SFT: c_int = 0;

// AFE_GAIN1_CON0
pub const GAIN1_SAMPLE_PER_STEP_SFT: c_int = 8;

pub const GAIN1_MODE_SFT: c_int = 4;

pub const GAIN1_ON_SFT: c_int = 0;

// AFE_GAIN1_CON1
pub const GAIN1_TARGET_SFT: c_int = 0;
pub const GAIN1_TARGET_MASK: c_uint = 0xfffffff;

// AFE_GAIN2_CON0
pub const GAIN2_SAMPLE_PER_STEP_SFT: c_int = 8;

pub const GAIN2_MODE_SFT: c_int = 4;

pub const GAIN2_ON_SFT: c_int = 0;

// AFE_GAIN2_CON1
pub const GAIN2_TARGET_SFT: c_int = 0;
pub const GAIN2_TARGET_MASK: c_uint = 0xfffffff;

// AFE_GAIN1_CUR
pub const AFE_GAIN1_CUR_SFT: c_int = 0;

// AFE_GAIN2_CUR
pub const AFE_GAIN2_CUR_SFT: c_int = 0;

// PCM_INTF_CON1
pub const PCM_FIX_VALUE_SEL_SFT: c_int = 31;

pub const PCM_BUFFER_LOOPBACK_SFT: c_int = 30;

pub const PCM_PARALLEL_LOOPBACK_SFT: c_int = 29;

pub const PCM_SERIAL_LOOPBACK_SFT: c_int = 28;

pub const PCM_DAI_PCM_LOOPBACK_SFT: c_int = 27;

pub const PCM_I2S_PCM_LOOPBACK_SFT: c_int = 26;

pub const PCM_SYNC_DELSEL_SFT: c_int = 25;

pub const PCM_TX_LR_SWAP_SFT: c_int = 24;

pub const PCM_SYNC_OUT_INV_SFT: c_int = 23;

pub const PCM_BCLK_OUT_INV_SFT: c_int = 22;

pub const PCM_SYNC_IN_INV_SFT: c_int = 21;

pub const PCM_BCLK_IN_INV_SFT: c_int = 20;

pub const PCM_TX_LCH_RPT_SFT: c_int = 19;

pub const PCM_VBT_16K_MODE_SFT: c_int = 18;

pub const PCM_EXT_MODEM_SFT: c_int = 17;

pub const PCM_24BIT_SFT: c_int = 16;

pub const PCM_WLEN_SFT: c_int = 14;

pub const PCM_SYNC_LENGTH_SFT: c_int = 9;

pub const PCM_SYNC_TYPE_SFT: c_int = 8;

pub const PCM_BT_MODE_SFT: c_int = 7;

pub const PCM_BYP_ASRC_SFT: c_int = 6;

pub const PCM_SLAVE_SFT: c_int = 5;

pub const PCM_MODE_SFT: c_int = 3;

pub const PCM_FMT_SFT: c_int = 1;

pub const PCM_EN_SFT: c_int = 0;

// PCM_INTF_CON2
pub const PCM1_TX_FIFO_OV_SFT: c_int = 31;

pub const PCM1_RX_FIFO_OV_SFT: c_int = 30;

pub const PCM2_TX_FIFO_OV_SFT: c_int = 29;

pub const PCM2_RX_FIFO_OV_SFT: c_int = 28;

pub const PCM1_SYNC_GLITCH_SFT: c_int = 27;

pub const PCM2_SYNC_GLITCH_SFT: c_int = 26;

pub const TX3_RCH_DBG_MODE_SFT: c_int = 17;

pub const PCM1_PCM2_LOOPBACK_SFT: c_int = 16;

pub const DAI_PCM_LOOPBACK_CH_SFT: c_int = 14;

pub const I2S_PCM_LOOPBACK_CH_SFT: c_int = 12;

pub const TX_FIX_VALUE_SFT: c_int = 0;

// PCM2_INTF_CON
pub const PCM2_TX_FIX_VALUE_SFT: c_int = 24;

pub const PCM2_FIX_VALUE_SEL_SFT: c_int = 23;

pub const PCM2_BUFFER_LOOPBACK_SFT: c_int = 22;

pub const PCM2_PARALLEL_LOOPBACK_SFT: c_int = 21;

pub const PCM2_SERIAL_LOOPBACK_SFT: c_int = 20;

pub const PCM2_DAI_PCM_LOOPBACK_SFT: c_int = 19;

pub const PCM2_I2S_PCM_LOOPBACK_SFT: c_int = 18;

pub const PCM2_SYNC_DELSEL_SFT: c_int = 17;

pub const PCM2_TX_LR_SWAP_SFT: c_int = 16;

pub const PCM2_SYNC_IN_INV_SFT: c_int = 15;

pub const PCM2_BCLK_IN_INV_SFT: c_int = 14;

pub const PCM2_TX_LCH_RPT_SFT: c_int = 13;

pub const PCM2_VBT_16K_MODE_SFT: c_int = 12;

pub const PCM2_LOOPBACK_CH_SEL_SFT: c_int = 10;

pub const PCM2_TX2_BT_MODE_SFT: c_int = 8;

pub const PCM2_BT_MODE_SFT: c_int = 7;

pub const PCM2_AFIFO_SFT: c_int = 6;

pub const PCM2_WLEN_SFT: c_int = 5;

pub const PCM2_MODE_SFT: c_int = 3;

pub const PCM2_FMT_SFT: c_int = 1;

pub const PCM2_EN_SFT: c_int = 0;

// AFE_CM1_CON

// AFE_ADDA_MTKAIF_CFG0
pub const MTKAIF_RXIF_CLKINV_ADC_SFT: c_int = 31;

pub const MTKAIF_RXIF_BYPASS_SRC_SFT: c_int = 17;

pub const MTKAIF_RXIF_PROTOCOL2_SFT: c_int = 16;

pub const MTKAIF_TXIF_BYPASS_SRC_SFT: c_int = 5;

pub const MTKAIF_TXIF_PROTOCOL2_SFT: c_int = 4;

pub const MTKAIF_TXIF_8TO5_SFT: c_int = 2;

pub const MTKAIF_RXIF_8TO5_SFT: c_int = 1;

pub const MTKAIF_IF_LOOPBACK1_SFT: c_int = 0;

// AFE_ADDA_MTKAIF_RX_CFG2
pub const MTKAIF_RXIF_DETECT_ON_PROTOCOL2_SFT: c_int = 16;

pub const MTKAIF_RXIF_DELAY_CYCLE_SFT: c_int = 12;

pub const MTKAIF_RXIF_DELAY_DATA_SFT: c_int = 8;
pub const MTKAIF_RXIF_DELAY_DATA_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_SFT: c_int = 4;

// AFE_ADDA_DL_SRC2_CON0
pub const DL_2_INPUT_MODE_CTL_SFT: c_int = 28;

pub const DL_2_CH1_SATURATION_EN_CTL_SFT: c_int = 27;

pub const DL_2_CH2_SATURATION_EN_CTL_SFT: c_int = 26;

pub const DL_2_OUTPUT_SEL_CTL_SFT: c_int = 24;

pub const DL_2_FADEIN_0START_EN_SFT: c_int = 16;

pub const DL_DISABLE_HW_CG_CTL_SFT: c_int = 15;

pub const C_DATA_EN_SEL_CTL_PRE_SFT: c_int = 14;

pub const DL_2_SIDE_TONE_ON_CTL_PRE_SFT: c_int = 13;

pub const DL_2_MUTE_CH1_OFF_CTL_PRE_SFT: c_int = 12;

pub const DL_2_MUTE_CH2_OFF_CTL_PRE_SFT: c_int = 11;

pub const DL2_ARAMPSP_CTL_PRE_SFT: c_int = 9;

pub const DL_2_IIRMODE_CTL_PRE_SFT: c_int = 6;

pub const DL_2_VOICE_MODE_CTL_PRE_SFT: c_int = 5;

pub const D2_2_MUTE_CH1_ON_CTL_PRE_SFT: c_int = 4;

pub const D2_2_MUTE_CH2_ON_CTL_PRE_SFT: c_int = 3;

pub const DL_2_IIR_ON_CTL_PRE_SFT: c_int = 2;

pub const DL_2_GAIN_ON_CTL_PRE_SFT: c_int = 1;

pub const DL_2_SRC_ON_CTL_PRE_SFT: c_int = 0;

// AFE_ADDA_DL_SRC2_CON1
pub const DL_2_GAIN_CTL_PRE_SFT: c_int = 16;
pub const DL_2_GAIN_CTL_PRE_MASK: c_uint = 0xffff;

pub const DL_2_GAIN_MODE_CTL_SFT: c_int = 0;

// AFE_ADDA_UL_SRC_CON0
pub const ULCF_CFG_EN_CTL_SFT: c_int = 31;

pub const UL_DMIC_PHASE_SEL_CH1_SFT: c_int = 27;

pub const UL_DMIC_PHASE_SEL_CH2_SFT: c_int = 24;

pub const UL_MODE_3P25M_CH2_CTL_SFT: c_int = 22;

pub const UL_MODE_3P25M_CH1_CTL_SFT: c_int = 21;

pub const UL_VOICE_MODE_CH1_CH2_CTL_SFT: c_int = 17;

pub const UL_AP_DMIC_ON_SFT: c_int = 16;

pub const DMIC_LOW_POWER_CTL_SFT: c_int = 14;

pub const UL_DISABLE_HW_CG_CTL_SFT: c_int = 12;

pub const UL_IIR_ON_TMP_CTL_SFT: c_int = 10;

pub const UL_IIRMODE_CTL_SFT: c_int = 7;

pub const DIGMIC_4P33M_SEL_SFT: c_int = 6;

pub const DIGMIC_3P25M_1P625M_SEL_SFT: c_int = 5;

pub const UL_LOOP_BACK_MODE_SFT: c_int = 2;

pub const UL_SDM_3_LEVEL_SFT: c_int = 1;

pub const UL_SRC_ON_CTL_SFT: c_int = 0;

// AFE_ADDA_UL_SRC_CON1
pub const C_DAC_EN_CTL_SFT: c_int = 27;

pub const C_MUTE_SW_CTL_SFT: c_int = 26;

pub const ASDM_SRC_SEL_CTL_SFT: c_int = 25;

pub const C_AMP_DIV_CH2_CTL_SFT: c_int = 21;

pub const C_FREQ_DIV_CH2_CTL_SFT: c_int = 16;

pub const C_SINE_MODE_CH2_CTL_SFT: c_int = 12;

pub const C_AMP_DIV_CH1_CTL_SFT: c_int = 9;

pub const C_FREQ_DIV_CH1_CTL_SFT: c_int = 4;

pub const C_SINE_MODE_CH1_CTL_SFT: c_int = 0;

// AFE_ADDA_TOP_CON0
pub const C_LOOP_BACK_MODE_CTL_SFT: c_int = 12;

pub const ADDA_UL_GAIN_MODE_SFT: c_int = 8;

pub const C_EXT_ADC_CTL_SFT: c_int = 0;

// AFE_ADDA_UL_DL_CON0
pub const AFE_ADDA_UL_LR_SWAP_SFT: c_int = 31;

pub const AFE_ADDA_CKDIV_RST_SFT: c_int = 30;

pub const AFE_ADDA_FIFO_AUTO_RST_SFT: c_int = 29;

pub const AFE_ADDA_UL_FIFO_DIGMIC_TESTIN_SFT: c_int = 21;

pub const AFE_ADDA_UL_FIFO_DIGMIC_WDATA_TESTEN_SFT: c_int = 20;

pub const AFE_ADDA6_UL_LR_SWAP_SFT: c_int = 15;

pub const AFE_ADDA6_CKDIV_RST_SFT: c_int = 14;

pub const AFE_ADDA6_FIFO_AUTO_RST_SFT: c_int = 13;

pub const AFE_ADDA6_UL_FIFO_DIGMIC_TESTIN_SFT: c_int = 5;

pub const AFE_ADDA6_UL_FIFO_DIGMIC_WDATA_TESTEN_SFT: c_int = 4;

pub const ADDA_AFE_ON_SFT: c_int = 0;

// AFE_SIDETONE_CON0
pub const R_RDY_SFT: c_int = 30;

pub const W_RDY_SFT: c_int = 29;

pub const R_W_EN_SFT: c_int = 25;

pub const R_W_SEL_SFT: c_int = 24;

pub const SEL_CH2_SFT: c_int = 23;

pub const SIDE_TONE_COEFFICIENT_ADDR_SFT: c_int = 16;

pub const SIDE_TONE_COEFFICIENT_SFT: c_int = 0;

// AFE_SIDETONE_COEFF
pub const SIDE_TONE_COEFF_SFT: c_int = 0;

// AFE_SIDETONE_CON1
pub const STF_BYPASS_MODE_SFT: c_int = 31;

pub const STF_BYPASS_MODE_O28_O29_SFT: c_int = 30;

pub const STF_BYPASS_MODE_I2S4_SFT: c_int = 29;

pub const STF_BYPASS_MODE_DL3_SFT: c_int = 27;

pub const STF_BYPASS_MODE_I2S7_SFT: c_int = 26;

pub const STF_BYPASS_MODE_I2S9_SFT: c_int = 25;

pub const STF_O19O20_OUT_EN_SEL_SFT: c_int = 13;

pub const STF_SOURCE_FROM_O19O20_SFT: c_int = 12;

pub const SIDE_TONE_ON_SFT: c_int = 8;

pub const SIDE_TONE_HALF_TAP_NUM_SFT: c_int = 0;

// AFE_SIDETONE_GAIN
pub const POSITIVE_GAIN_SFT: c_int = 16;

pub const SIDE_TONE_GAIN_SFT: c_int = 0;

// AFE_ADDA_DL_SDM_DCCOMP_CON
pub const USE_3RD_SDM_SFT: c_int = 28;

pub const DL_FIFO_START_POINT_SFT: c_int = 24;

pub const DL_FIFO_SWAP_SFT: c_int = 20;

pub const C_AUDSDM1ORDSELECT_CTL_SFT: c_int = 19;

pub const C_SDM7BITSEL_CTL_SFT: c_int = 18;

pub const GAIN_AT_SDM_RST_PRE_CTL_SFT: c_int = 15;

pub const DL_DCM_AUTO_IDLE_EN_SFT: c_int = 14;

pub const AFE_DL_SRC_DCM_EN_SFT: c_int = 13;

pub const AFE_DL_POST_SRC_DCM_EN_SFT: c_int = 12;

pub const AUD_SDM_MONO_SFT: c_int = 9;

pub const AUD_DC_COMP_EN_SFT: c_int = 8;

pub const ATTGAIN_CTL_SFT: c_int = 0;

// AFE_SINEGEN_CON0
pub const DAC_EN_SFT: c_int = 26;
pub const DAC_EN_MASK: c_uint = 0x1;

pub const MUTE_SW_CH2_SFT: c_int = 25;
pub const MUTE_SW_CH2_MASK: c_uint = 0x1;

pub const MUTE_SW_CH1_SFT: c_int = 24;
pub const MUTE_SW_CH1_MASK: c_uint = 0x1;

pub const SINE_MODE_CH2_SFT: c_int = 20;
pub const SINE_MODE_CH2_MASK: c_uint = 0xf;

pub const AMP_DIV_CH2_SFT: c_int = 17;
pub const AMP_DIV_CH2_MASK: c_uint = 0x7;

pub const FREQ_DIV_CH2_SFT: c_int = 12;
pub const FREQ_DIV_CH2_MASK: c_uint = 0x1f;

pub const SINE_MODE_CH1_SFT: c_int = 8;
pub const SINE_MODE_CH1_MASK: c_uint = 0xf;

pub const AMP_DIV_CH1_SFT: c_int = 5;
pub const AMP_DIV_CH1_MASK: c_uint = 0x7;

pub const FREQ_DIV_CH1_SFT: c_int = 0;
pub const FREQ_DIV_CH1_MASK: c_uint = 0x1f;

// AFE_SINEGEN_CON2
pub const INNER_LOOP_BACK_MODE_SFT: c_int = 0;

// AFE_HD_ENGEN_ENABLE
pub const AFE_24M_ON_SFT: c_int = 1;

pub const AFE_22M_ON_SFT: c_int = 0;

// AFE_ADDA_DL_NLE_FIFO_MON
pub const DL_NLE_FIFO_WBIN_SFT: c_int = 8;

pub const DL_NLE_FIFO_RBIN_SFT: c_int = 4;

pub const DL_NLE_FIFO_RDACTIVE_SFT: c_int = 3;

pub const DL_NLE_FIFO_STARTRD_SFT: c_int = 2;

pub const DL_NLE_FIFO_RD_EMPTY_SFT: c_int = 1;

pub const DL_NLE_FIFO_WR_FULL_SFT: c_int = 0;

// AFE_DL1_CON0
pub const DL1_MODE_SFT: c_int = 24;
pub const DL1_MODE_MASK: c_uint = 0xf;

pub const DL1_MINLEN_SFT: c_int = 20;
pub const DL1_MINLEN_MASK: c_uint = 0xf;

pub const DL1_MAXLEN_SFT: c_int = 16;
pub const DL1_MAXLEN_MASK: c_uint = 0xf;

pub const DL1_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL1_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL1_PBUF_SIZE_SFT: c_int = 12;
pub const DL1_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL1_MONO_SFT: c_int = 8;
pub const DL1_MONO_MASK: c_uint = 0x1;

pub const DL1_NORMAL_MODE_SFT: c_int = 5;
pub const DL1_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL1_HALIGN_SFT: c_int = 4;
pub const DL1_HALIGN_MASK: c_uint = 0x1;

pub const DL1_HD_MODE_SFT: c_int = 0;
pub const DL1_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL2_CON0
pub const DL2_MODE_SFT: c_int = 24;
pub const DL2_MODE_MASK: c_uint = 0xf;

pub const DL2_MINLEN_SFT: c_int = 20;
pub const DL2_MINLEN_MASK: c_uint = 0xf;

pub const DL2_MAXLEN_SFT: c_int = 16;
pub const DL2_MAXLEN_MASK: c_uint = 0xf;

pub const DL2_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL2_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL2_PBUF_SIZE_SFT: c_int = 12;
pub const DL2_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL2_MONO_SFT: c_int = 8;
pub const DL2_MONO_MASK: c_uint = 0x1;

pub const DL2_NORMAL_MODE_SFT: c_int = 5;
pub const DL2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL2_HALIGN_SFT: c_int = 4;
pub const DL2_HALIGN_MASK: c_uint = 0x1;

pub const DL2_HD_MODE_SFT: c_int = 0;
pub const DL2_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL3_CON0
pub const DL3_MODE_SFT: c_int = 24;
pub const DL3_MODE_MASK: c_uint = 0xf;

pub const DL3_MINLEN_SFT: c_int = 20;
pub const DL3_MINLEN_MASK: c_uint = 0xf;

pub const DL3_MAXLEN_SFT: c_int = 16;
pub const DL3_MAXLEN_MASK: c_uint = 0xf;

pub const DL3_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL3_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL3_PBUF_SIZE_SFT: c_int = 12;
pub const DL3_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL3_MONO_SFT: c_int = 8;
pub const DL3_MONO_MASK: c_uint = 0x1;

pub const DL3_NORMAL_MODE_SFT: c_int = 5;
pub const DL3_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL3_HALIGN_SFT: c_int = 4;
pub const DL3_HALIGN_MASK: c_uint = 0x1;

pub const DL3_HD_MODE_SFT: c_int = 0;
pub const DL3_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL4_CON0
pub const DL4_MODE_SFT: c_int = 24;
pub const DL4_MODE_MASK: c_uint = 0xf;

pub const DL4_MINLEN_SFT: c_int = 20;
pub const DL4_MINLEN_MASK: c_uint = 0xf;

pub const DL4_MAXLEN_SFT: c_int = 16;
pub const DL4_MAXLEN_MASK: c_uint = 0xf;

pub const DL4_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL4_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL4_PBUF_SIZE_SFT: c_int = 12;
pub const DL4_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL4_MONO_SFT: c_int = 8;
pub const DL4_MONO_MASK: c_uint = 0x1;

pub const DL4_NORMAL_MODE_SFT: c_int = 5;
pub const DL4_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL4_HALIGN_SFT: c_int = 4;
pub const DL4_HALIGN_MASK: c_uint = 0x1;

pub const DL4_HD_MODE_SFT: c_int = 0;
pub const DL4_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL5_CON0
pub const DL5_MODE_SFT: c_int = 24;
pub const DL5_MODE_MASK: c_uint = 0xf;

pub const DL5_MINLEN_SFT: c_int = 20;
pub const DL5_MINLEN_MASK: c_uint = 0xf;

pub const DL5_MAXLEN_SFT: c_int = 16;
pub const DL5_MAXLEN_MASK: c_uint = 0xf;

pub const DL5_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL5_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL5_PBUF_SIZE_SFT: c_int = 12;
pub const DL5_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL5_MONO_SFT: c_int = 8;
pub const DL5_MONO_MASK: c_uint = 0x1;

pub const DL5_NORMAL_MODE_SFT: c_int = 5;
pub const DL5_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL5_HALIGN_SFT: c_int = 4;
pub const DL5_HALIGN_MASK: c_uint = 0x1;

pub const DL5_HD_MODE_SFT: c_int = 0;
pub const DL5_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL6_CON0
pub const DL6_MODE_SFT: c_int = 24;
pub const DL6_MODE_MASK: c_uint = 0xf;

pub const DL6_MINLEN_SFT: c_int = 20;
pub const DL6_MINLEN_MASK: c_uint = 0xf;

pub const DL6_MAXLEN_SFT: c_int = 16;
pub const DL6_MAXLEN_MASK: c_uint = 0xf;

pub const DL6_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL6_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL6_PBUF_SIZE_SFT: c_int = 12;
pub const DL6_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL6_MONO_SFT: c_int = 8;
pub const DL6_MONO_MASK: c_uint = 0x1;

pub const DL6_NORMAL_MODE_SFT: c_int = 5;
pub const DL6_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL6_HALIGN_SFT: c_int = 4;
pub const DL6_HALIGN_MASK: c_uint = 0x1;

pub const DL6_HD_MODE_SFT: c_int = 0;
pub const DL6_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL7_CON0
pub const DL7_MODE_SFT: c_int = 24;
pub const DL7_MODE_MASK: c_uint = 0xf;

pub const DL7_MINLEN_SFT: c_int = 20;
pub const DL7_MINLEN_MASK: c_uint = 0xf;

pub const DL7_MAXLEN_SFT: c_int = 16;
pub const DL7_MAXLEN_MASK: c_uint = 0xf;

pub const DL7_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL7_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL7_PBUF_SIZE_SFT: c_int = 12;
pub const DL7_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL7_MONO_SFT: c_int = 8;
pub const DL7_MONO_MASK: c_uint = 0x1;

pub const DL7_NORMAL_MODE_SFT: c_int = 5;
pub const DL7_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL7_HALIGN_SFT: c_int = 4;
pub const DL7_HALIGN_MASK: c_uint = 0x1;

pub const DL7_HD_MODE_SFT: c_int = 0;
pub const DL7_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL8_CON0
pub const DL8_MODE_SFT: c_int = 24;
pub const DL8_MODE_MASK: c_uint = 0xf;

pub const DL8_MINLEN_SFT: c_int = 20;
pub const DL8_MINLEN_MASK: c_uint = 0xf;

pub const DL8_MAXLEN_SFT: c_int = 16;
pub const DL8_MAXLEN_MASK: c_uint = 0xf;

pub const DL8_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL8_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL8_PBUF_SIZE_SFT: c_int = 12;
pub const DL8_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL8_MONO_SFT: c_int = 8;
pub const DL8_MONO_MASK: c_uint = 0x1;

pub const DL8_NORMAL_MODE_SFT: c_int = 5;
pub const DL8_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL8_HALIGN_SFT: c_int = 4;
pub const DL8_HALIGN_MASK: c_uint = 0x1;

pub const DL8_HD_MODE_SFT: c_int = 0;
pub const DL8_HD_MODE_MASK: c_uint = 0x3;

// AFE_DL12_CON0
pub const DL12_MODE_SFT: c_int = 24;
pub const DL12_MODE_MASK: c_uint = 0xf;

pub const DL12_MINLEN_SFT: c_int = 20;
pub const DL12_MINLEN_MASK: c_uint = 0xf;

pub const DL12_MAXLEN_SFT: c_int = 16;
pub const DL12_MAXLEN_MASK: c_uint = 0xf;

pub const DL12_SW_CLEAR_BUF_EMPTY_SFT: c_int = 15;
pub const DL12_SW_CLEAR_BUF_EMPTY_MASK: c_uint = 0x1;

pub const DL12_PBUF_SIZE_SFT: c_int = 12;
pub const DL12_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL12_4CH_EN_SFT: c_int = 11;
pub const DL12_4CH_EN_MASK: c_uint = 0x1;

pub const DL12_MONO_SFT: c_int = 8;
pub const DL12_MONO_MASK: c_uint = 0x1;

pub const DL12_NORMAL_MODE_SFT: c_int = 5;
pub const DL12_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL12_HALIGN_SFT: c_int = 4;
pub const DL12_HALIGN_MASK: c_uint = 0x1;

pub const DL12_HD_MODE_SFT: c_int = 0;
pub const DL12_HD_MODE_MASK: c_uint = 0x3;

// AFE_AWB_CON0
pub const AWB_MODE_SFT: c_int = 24;
pub const AWB_MODE_MASK: c_uint = 0xf;

pub const AWB_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const AWB_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const AWB_R_MONO_SFT: c_int = 9;
pub const AWB_R_MONO_MASK: c_uint = 0x1;

pub const AWB_MONO_SFT: c_int = 8;
pub const AWB_MONO_MASK: c_uint = 0x1;

pub const AWB_WR_SIGN_SFT: c_int = 6;
pub const AWB_WR_SIGN_MASK: c_uint = 0x1;

pub const AWB_NORMAL_MODE_SFT: c_int = 5;
pub const AWB_NORMAL_MODE_MASK: c_uint = 0x1;

pub const AWB_HALIGN_SFT: c_int = 4;
pub const AWB_HALIGN_MASK: c_uint = 0x1;

pub const AWB_HD_MODE_SFT: c_int = 0;
pub const AWB_HD_MODE_MASK: c_uint = 0x3;

// AFE_AWB2_CON0
pub const AWB2_MODE_SFT: c_int = 24;
pub const AWB2_MODE_MASK: c_uint = 0xf;

pub const AWB2_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const AWB2_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const AWB2_R_MONO_SFT: c_int = 9;
pub const AWB2_R_MONO_MASK: c_uint = 0x1;

pub const AWB2_MONO_SFT: c_int = 8;
pub const AWB2_MONO_MASK: c_uint = 0x1;

pub const AWB2_WR_SIGN_SFT: c_int = 6;
pub const AWB2_WR_SIGN_MASK: c_uint = 0x1;

pub const AWB2_NORMAL_MODE_SFT: c_int = 5;
pub const AWB2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const AWB2_HALIGN_SFT: c_int = 4;
pub const AWB2_HALIGN_MASK: c_uint = 0x1;

pub const AWB2_HD_MODE_SFT: c_int = 0;
pub const AWB2_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL_CON0
pub const VUL_MODE_SFT: c_int = 24;
pub const VUL_MODE_MASK: c_uint = 0xf;

pub const VUL_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL_R_MONO_SFT: c_int = 9;
pub const VUL_R_MONO_MASK: c_uint = 0x1;

pub const VUL_MONO_SFT: c_int = 8;
pub const VUL_MONO_MASK: c_uint = 0x1;

pub const VUL_WR_SIGN_SFT: c_int = 6;
pub const VUL_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL_NORMAL_MODE_SFT: c_int = 5;
pub const VUL_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL_HALIGN_SFT: c_int = 4;
pub const VUL_HALIGN_MASK: c_uint = 0x1;

pub const VUL_HD_MODE_SFT: c_int = 0;
pub const VUL_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL12_CON0
pub const VUL12_MODE_SFT: c_int = 24;
pub const VUL12_MODE_MASK: c_uint = 0xf;

pub const VUL12_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL12_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL12_4CH_EN_SFT: c_int = 11;
pub const VUL12_4CH_EN_MASK: c_uint = 0x1;

pub const VUL12_R_MONO_SFT: c_int = 9;
pub const VUL12_R_MONO_MASK: c_uint = 0x1;

pub const VUL12_MONO_SFT: c_int = 8;
pub const VUL12_MONO_MASK: c_uint = 0x1;

pub const VUL12_WR_SIGN_SFT: c_int = 6;
pub const VUL12_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL12_NORMAL_MODE_SFT: c_int = 5;
pub const VUL12_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL12_HALIGN_SFT: c_int = 4;
pub const VUL12_HALIGN_MASK: c_uint = 0x1;

pub const VUL12_HD_MODE_SFT: c_int = 0;
pub const VUL12_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL2_CON0
pub const VUL2_MODE_SFT: c_int = 24;
pub const VUL2_MODE_MASK: c_uint = 0xf;

pub const VUL2_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL2_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL2_R_MONO_SFT: c_int = 9;
pub const VUL2_R_MONO_MASK: c_uint = 0x1;

pub const VUL2_MONO_SFT: c_int = 8;
pub const VUL2_MONO_MASK: c_uint = 0x1;

pub const VUL2_WR_SIGN_SFT: c_int = 6;
pub const VUL2_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL2_NORMAL_MODE_SFT: c_int = 5;
pub const VUL2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL2_HALIGN_SFT: c_int = 4;
pub const VUL2_HALIGN_MASK: c_uint = 0x1;

pub const VUL2_HD_MODE_SFT: c_int = 0;
pub const VUL2_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL3_CON0
pub const VUL3_MODE_SFT: c_int = 24;
pub const VUL3_MODE_MASK: c_uint = 0xf;

pub const VUL3_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL3_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL3_R_MONO_SFT: c_int = 9;
pub const VUL3_R_MONO_MASK: c_uint = 0x1;

pub const VUL3_MONO_SFT: c_int = 8;
pub const VUL3_MONO_MASK: c_uint = 0x1;

pub const VUL3_WR_SIGN_SFT: c_int = 6;
pub const VUL3_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL3_NORMAL_MODE_SFT: c_int = 5;
pub const VUL3_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL3_HALIGN_SFT: c_int = 4;
pub const VUL3_HALIGN_MASK: c_uint = 0x1;

pub const VUL3_HD_MODE_SFT: c_int = 0;
pub const VUL3_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL4_CON0
pub const VUL4_MODE_SFT: c_int = 24;
pub const VUL4_MODE_MASK: c_uint = 0xf;

pub const VUL4_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL4_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL4_R_MONO_SFT: c_int = 9;
pub const VUL4_R_MONO_MASK: c_uint = 0x1;

pub const VUL4_MONO_SFT: c_int = 8;
pub const VUL4_MONO_MASK: c_uint = 0x1;

pub const VUL4_WR_SIGN_SFT: c_int = 6;
pub const VUL4_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL4_NORMAL_MODE_SFT: c_int = 5;
pub const VUL4_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL4_HALIGN_SFT: c_int = 4;
pub const VUL4_HALIGN_MASK: c_uint = 0x1;

pub const VUL4_HD_MODE_SFT: c_int = 0;
pub const VUL4_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL5_CON0
pub const VUL5_MODE_SFT: c_int = 24;
pub const VUL5_MODE_MASK: c_uint = 0xf;

pub const VUL5_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL5_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL5_R_MONO_SFT: c_int = 9;
pub const VUL5_R_MONO_MASK: c_uint = 0x1;

pub const VUL5_MONO_SFT: c_int = 8;
pub const VUL5_MONO_MASK: c_uint = 0x1;

pub const VUL5_WR_SIGN_SFT: c_int = 6;
pub const VUL5_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL5_NORMAL_MODE_SFT: c_int = 5;
pub const VUL5_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL5_HALIGN_SFT: c_int = 4;
pub const VUL5_HALIGN_MASK: c_uint = 0x1;

pub const VUL5_HD_MODE_SFT: c_int = 0;
pub const VUL5_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL6_CON0
pub const VUL6_MODE_SFT: c_int = 24;
pub const VUL6_MODE_MASK: c_uint = 0xf;

pub const VUL6_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const VUL6_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const VUL6_R_MONO_SFT: c_int = 9;
pub const VUL6_R_MONO_MASK: c_uint = 0x1;

pub const VUL6_MONO_SFT: c_int = 8;
pub const VUL6_MONO_MASK: c_uint = 0x1;

pub const VUL6_WR_SIGN_SFT: c_int = 6;
pub const VUL6_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL6_NORMAL_MODE_SFT: c_int = 5;
pub const VUL6_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL6_HALIGN_SFT: c_int = 4;
pub const VUL6_HALIGN_MASK: c_uint = 0x1;

pub const VUL6_HD_MODE_SFT: c_int = 0;
pub const VUL6_HD_MODE_MASK: c_uint = 0x3;

// AFE_DAI_CON0
pub const DAI_MODE_SFT: c_int = 24;
pub const DAI_MODE_MASK: c_uint = 0x3;

pub const DAI_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const DAI_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const DAI_DUPLICATE_WR_SFT: c_int = 10;
pub const DAI_DUPLICATE_WR_MASK: c_uint = 0x1;

pub const DAI_MONO_SFT: c_int = 8;
pub const DAI_MONO_MASK: c_uint = 0x1;

pub const DAI_WR_SIGN_SFT: c_int = 6;
pub const DAI_WR_SIGN_MASK: c_uint = 0x1;

pub const DAI_NORMAL_MODE_SFT: c_int = 5;
pub const DAI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DAI_HALIGN_SFT: c_int = 4;
pub const DAI_HALIGN_MASK: c_uint = 0x1;

pub const DAI_HD_MODE_SFT: c_int = 0;
pub const DAI_HD_MODE_MASK: c_uint = 0x3;

// AFE_MOD_DAI_CON0
pub const MOD_DAI_MODE_SFT: c_int = 24;
pub const MOD_DAI_MODE_MASK: c_uint = 0x3;

pub const MOD_DAI_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const MOD_DAI_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const MOD_DAI_DUPLICATE_WR_SFT: c_int = 10;
pub const MOD_DAI_DUPLICATE_WR_MASK: c_uint = 0x1;

pub const MOD_DAI_MONO_SFT: c_int = 8;
pub const MOD_DAI_MONO_MASK: c_uint = 0x1;

pub const MOD_DAI_WR_SIGN_SFT: c_int = 6;
pub const MOD_DAI_WR_SIGN_MASK: c_uint = 0x1;

pub const MOD_DAI_NORMAL_MODE_SFT: c_int = 5;
pub const MOD_DAI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const MOD_DAI_HALIGN_SFT: c_int = 4;
pub const MOD_DAI_HALIGN_MASK: c_uint = 0x1;

pub const MOD_DAI_HD_MODE_SFT: c_int = 0;
pub const MOD_DAI_HD_MODE_MASK: c_uint = 0x3;

// AFE_DAI2_CON0
pub const DAI2_MODE_SFT: c_int = 24;
pub const DAI2_MODE_MASK: c_uint = 0xf;

pub const DAI2_SW_CLEAR_BUF_FULL_SFT: c_int = 15;
pub const DAI2_SW_CLEAR_BUF_FULL_MASK: c_uint = 0x1;

pub const DAI2_DUPLICATE_WR_SFT: c_int = 10;
pub const DAI2_DUPLICATE_WR_MASK: c_uint = 0x1;

pub const DAI2_MONO_SFT: c_int = 8;
pub const DAI2_MONO_MASK: c_uint = 0x1;

pub const DAI2_WR_SIGN_SFT: c_int = 6;
pub const DAI2_WR_SIGN_MASK: c_uint = 0x1;

pub const DAI2_NORMAL_MODE_SFT: c_int = 5;
pub const DAI2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DAI2_HALIGN_SFT: c_int = 4;
pub const DAI2_HALIGN_MASK: c_uint = 0x1;

pub const DAI2_HD_MODE_SFT: c_int = 0;
pub const DAI2_HD_MODE_MASK: c_uint = 0x3;

// AFE_MEMIF_CON0
pub const CPU_COMPACT_MODE_SFT: c_int = 2;

pub const CPU_HD_ALIGN_SFT: c_int = 1;

pub const SYSRAM_SIGN_SFT: c_int = 0;

// AFE_IRQ_MCU_CON0
pub const IRQ31_MCU_ON_SFT: c_int = 31;
pub const IRQ31_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ26_MCU_ON_SFT: c_int = 26;
pub const IRQ26_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ25_MCU_ON_SFT: c_int = 25;
pub const IRQ25_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ24_MCU_ON_SFT: c_int = 24;
pub const IRQ24_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ23_MCU_ON_SFT: c_int = 23;
pub const IRQ23_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ22_MCU_ON_SFT: c_int = 22;
pub const IRQ22_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ21_MCU_ON_SFT: c_int = 21;
pub const IRQ21_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ20_MCU_ON_SFT: c_int = 20;
pub const IRQ20_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ19_MCU_ON_SFT: c_int = 19;
pub const IRQ19_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ18_MCU_ON_SFT: c_int = 18;
pub const IRQ18_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ17_MCU_ON_SFT: c_int = 17;
pub const IRQ17_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ16_MCU_ON_SFT: c_int = 16;
pub const IRQ16_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ15_MCU_ON_SFT: c_int = 15;
pub const IRQ15_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ14_MCU_ON_SFT: c_int = 14;
pub const IRQ14_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ13_MCU_ON_SFT: c_int = 13;
pub const IRQ13_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ12_MCU_ON_SFT: c_int = 12;
pub const IRQ12_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ11_MCU_ON_SFT: c_int = 11;
pub const IRQ11_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ10_MCU_ON_SFT: c_int = 10;
pub const IRQ10_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ9_MCU_ON_SFT: c_int = 9;
pub const IRQ9_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ8_MCU_ON_SFT: c_int = 8;
pub const IRQ8_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ7_MCU_ON_SFT: c_int = 7;
pub const IRQ7_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ6_MCU_ON_SFT: c_int = 6;
pub const IRQ6_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ5_MCU_ON_SFT: c_int = 5;
pub const IRQ5_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ4_MCU_ON_SFT: c_int = 4;
pub const IRQ4_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ3_MCU_ON_SFT: c_int = 3;
pub const IRQ3_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ2_MCU_ON_SFT: c_int = 2;
pub const IRQ2_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ1_MCU_ON_SFT: c_int = 1;
pub const IRQ1_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ0_MCU_ON_SFT: c_int = 0;
pub const IRQ0_MCU_ON_MASK: c_uint = 0x1;

// AFE_IRQ_MCU_CON1
pub const IRQ7_MCU_MODE_SFT: c_int = 28;
pub const IRQ7_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ6_MCU_MODE_SFT: c_int = 24;
pub const IRQ6_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ5_MCU_MODE_SFT: c_int = 20;
pub const IRQ5_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ4_MCU_MODE_SFT: c_int = 16;
pub const IRQ4_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ3_MCU_MODE_SFT: c_int = 12;
pub const IRQ3_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ2_MCU_MODE_SFT: c_int = 8;
pub const IRQ2_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ1_MCU_MODE_SFT: c_int = 4;
pub const IRQ1_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ0_MCU_MODE_SFT: c_int = 0;
pub const IRQ0_MCU_MODE_MASK: c_uint = 0xf;

// AFE_IRQ_MCU_CON2
pub const IRQ15_MCU_MODE_SFT: c_int = 28;
pub const IRQ15_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ14_MCU_MODE_SFT: c_int = 24;
pub const IRQ14_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ13_MCU_MODE_SFT: c_int = 20;
pub const IRQ13_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ12_MCU_MODE_SFT: c_int = 16;
pub const IRQ12_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ11_MCU_MODE_SFT: c_int = 12;
pub const IRQ11_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ10_MCU_MODE_SFT: c_int = 8;
pub const IRQ10_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ9_MCU_MODE_SFT: c_int = 4;
pub const IRQ9_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ8_MCU_MODE_SFT: c_int = 0;
pub const IRQ8_MCU_MODE_MASK: c_uint = 0xf;

// AFE_IRQ_MCU_CON3
pub const IRQ23_MCU_MODE_SFT: c_int = 28;
pub const IRQ23_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ22_MCU_MODE_SFT: c_int = 24;
pub const IRQ22_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ21_MCU_MODE_SFT: c_int = 20;
pub const IRQ21_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ20_MCU_MODE_SFT: c_int = 16;
pub const IRQ20_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ19_MCU_MODE_SFT: c_int = 12;
pub const IRQ19_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ18_MCU_MODE_SFT: c_int = 8;
pub const IRQ18_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ17_MCU_MODE_SFT: c_int = 4;
pub const IRQ17_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ16_MCU_MODE_SFT: c_int = 0;
pub const IRQ16_MCU_MODE_MASK: c_uint = 0xf;

// AFE_IRQ_MCU_CON4
pub const IRQ26_MCU_MODE_SFT: c_int = 8;
pub const IRQ26_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ25_MCU_MODE_SFT: c_int = 4;
pub const IRQ25_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ24_MCU_MODE_SFT: c_int = 0;
pub const IRQ24_MCU_MODE_MASK: c_uint = 0xf;

// AFE_IRQ_MCU_CLR
pub const IRQ31_MCU_CLR_SFT: c_int = 31;

pub const IRQ26_MCU_CLR_SFT: c_int = 26;

pub const IRQ25_MCU_CLR_SFT: c_int = 25;

pub const IRQ24_MCU_CLR_SFT: c_int = 24;

pub const IRQ23_MCU_CLR_SFT: c_int = 23;

pub const IRQ22_MCU_CLR_SFT: c_int = 22;

pub const IRQ21_MCU_CLR_SFT: c_int = 21;

pub const IRQ20_MCU_CLR_SFT: c_int = 20;

pub const IRQ19_MCU_CLR_SFT: c_int = 19;

pub const IRQ18_MCU_CLR_SFT: c_int = 18;

pub const IRQ17_MCU_CLR_SFT: c_int = 17;

pub const IRQ16_MCU_CLR_SFT: c_int = 16;

pub const IRQ15_MCU_CLR_SFT: c_int = 15;

pub const IRQ14_MCU_CLR_SFT: c_int = 14;

pub const IRQ13_MCU_CLR_SFT: c_int = 13;

pub const IRQ12_MCU_CLR_SFT: c_int = 12;

pub const IRQ11_MCU_CLR_SFT: c_int = 11;

pub const IRQ10_MCU_CLR_SFT: c_int = 10;

pub const IRQ9_MCU_CLR_SFT: c_int = 9;

pub const IRQ8_MCU_CLR_SFT: c_int = 8;

pub const IRQ7_MCU_CLR_SFT: c_int = 7;

pub const IRQ6_MCU_CLR_SFT: c_int = 6;

pub const IRQ5_MCU_CLR_SFT: c_int = 5;

pub const IRQ4_MCU_CLR_SFT: c_int = 4;

pub const IRQ3_MCU_CLR_SFT: c_int = 3;

pub const IRQ2_MCU_CLR_SFT: c_int = 2;

pub const IRQ1_MCU_CLR_SFT: c_int = 1;

pub const IRQ0_MCU_CLR_SFT: c_int = 0;

// AFE_IRQ_MCU_EN
pub const IRQ31_MCU_EN_SFT: c_int = 31;
pub const IRQ30_MCU_EN_SFT: c_int = 30;
pub const IRQ29_MCU_EN_SFT: c_int = 29;
pub const IRQ28_MCU_EN_SFT: c_int = 28;
pub const IRQ27_MCU_EN_SFT: c_int = 27;
pub const IRQ26_MCU_EN_SFT: c_int = 26;
pub const IRQ25_MCU_EN_SFT: c_int = 25;
pub const IRQ24_MCU_EN_SFT: c_int = 24;
pub const IRQ23_MCU_EN_SFT: c_int = 23;
pub const IRQ22_MCU_EN_SFT: c_int = 22;
pub const IRQ21_MCU_EN_SFT: c_int = 21;
pub const IRQ20_MCU_EN_SFT: c_int = 20;
pub const IRQ19_MCU_EN_SFT: c_int = 19;
pub const IRQ18_MCU_EN_SFT: c_int = 18;
pub const IRQ17_MCU_EN_SFT: c_int = 17;
pub const IRQ16_MCU_EN_SFT: c_int = 16;
pub const IRQ15_MCU_EN_SFT: c_int = 15;
pub const IRQ14_MCU_EN_SFT: c_int = 14;
pub const IRQ13_MCU_EN_SFT: c_int = 13;
pub const IRQ12_MCU_EN_SFT: c_int = 12;
pub const IRQ11_MCU_EN_SFT: c_int = 11;
pub const IRQ10_MCU_EN_SFT: c_int = 10;
pub const IRQ9_MCU_EN_SFT: c_int = 9;
pub const IRQ8_MCU_EN_SFT: c_int = 8;
pub const IRQ7_MCU_EN_SFT: c_int = 7;
pub const IRQ6_MCU_EN_SFT: c_int = 6;
pub const IRQ5_MCU_EN_SFT: c_int = 5;
pub const IRQ4_MCU_EN_SFT: c_int = 4;
pub const IRQ3_MCU_EN_SFT: c_int = 3;
pub const IRQ2_MCU_EN_SFT: c_int = 2;
pub const IRQ1_MCU_EN_SFT: c_int = 1;
pub const IRQ0_MCU_EN_SFT: c_int = 0;
// AFE_IRQ_MCU_SCP_EN
pub const IRQ31_MCU_SCP_EN_SFT: c_int = 31;
pub const IRQ30_MCU_SCP_EN_SFT: c_int = 30;
pub const IRQ29_MCU_SCP_EN_SFT: c_int = 29;
pub const IRQ28_MCU_SCP_EN_SFT: c_int = 28;
pub const IRQ27_MCU_SCP_EN_SFT: c_int = 27;
pub const IRQ26_MCU_SCP_EN_SFT: c_int = 26;
pub const IRQ25_MCU_SCP_EN_SFT: c_int = 25;
pub const IRQ24_MCU_SCP_EN_SFT: c_int = 24;
pub const IRQ23_MCU_SCP_EN_SFT: c_int = 23;
pub const IRQ22_MCU_SCP_EN_SFT: c_int = 22;
pub const IRQ21_MCU_SCP_EN_SFT: c_int = 21;
pub const IRQ20_MCU_SCP_EN_SFT: c_int = 20;
pub const IRQ19_MCU_SCP_EN_SFT: c_int = 19;
pub const IRQ18_MCU_SCP_EN_SFT: c_int = 18;
pub const IRQ17_MCU_SCP_EN_SFT: c_int = 17;
pub const IRQ16_MCU_SCP_EN_SFT: c_int = 16;
pub const IRQ15_MCU_SCP_EN_SFT: c_int = 15;
pub const IRQ14_MCU_SCP_EN_SFT: c_int = 14;
pub const IRQ13_MCU_SCP_EN_SFT: c_int = 13;
pub const IRQ12_MCU_SCP_EN_SFT: c_int = 12;
pub const IRQ11_MCU_SCP_EN_SFT: c_int = 11;
pub const IRQ10_MCU_SCP_EN_SFT: c_int = 10;
pub const IRQ9_MCU_SCP_EN_SFT: c_int = 9;
pub const IRQ8_MCU_SCP_EN_SFT: c_int = 8;
pub const IRQ7_MCU_SCP_EN_SFT: c_int = 7;
pub const IRQ6_MCU_SCP_EN_SFT: c_int = 6;
pub const IRQ5_MCU_SCP_EN_SFT: c_int = 5;
pub const IRQ4_MCU_SCP_EN_SFT: c_int = 4;
pub const IRQ3_MCU_SCP_EN_SFT: c_int = 3;
pub const IRQ2_MCU_SCP_EN_SFT: c_int = 2;
pub const IRQ1_MCU_SCP_EN_SFT: c_int = 1;
pub const IRQ0_MCU_SCP_EN_SFT: c_int = 0;
// AFE_IRQ_MCU_DSP_EN
pub const IRQ31_MCU_DSP_EN_SFT: c_int = 31;
pub const IRQ30_MCU_DSP_EN_SFT: c_int = 30;
pub const IRQ29_MCU_DSP_EN_SFT: c_int = 29;
pub const IRQ28_MCU_DSP_EN_SFT: c_int = 28;
pub const IRQ27_MCU_DSP_EN_SFT: c_int = 27;
pub const IRQ26_MCU_DSP_EN_SFT: c_int = 26;
pub const IRQ25_MCU_DSP_EN_SFT: c_int = 25;
pub const IRQ24_MCU_DSP_EN_SFT: c_int = 24;
pub const IRQ23_MCU_DSP_EN_SFT: c_int = 23;
pub const IRQ22_MCU_DSP_EN_SFT: c_int = 22;
pub const IRQ21_MCU_DSP_EN_SFT: c_int = 21;
pub const IRQ20_MCU_DSP_EN_SFT: c_int = 20;
pub const IRQ19_MCU_DSP_EN_SFT: c_int = 19;
pub const IRQ18_MCU_DSP_EN_SFT: c_int = 18;
pub const IRQ17_MCU_DSP_EN_SFT: c_int = 17;
pub const IRQ16_MCU_DSP_EN_SFT: c_int = 16;
pub const IRQ15_MCU_DSP_EN_SFT: c_int = 15;
pub const IRQ14_MCU_DSP_EN_SFT: c_int = 14;
pub const IRQ13_MCU_DSP_EN_SFT: c_int = 13;
pub const IRQ12_MCU_DSP_EN_SFT: c_int = 12;
pub const IRQ11_MCU_DSP_EN_SFT: c_int = 11;
pub const IRQ10_MCU_DSP_EN_SFT: c_int = 10;
pub const IRQ9_MCU_DSP_EN_SFT: c_int = 9;
pub const IRQ8_MCU_DSP_EN_SFT: c_int = 8;
pub const IRQ7_MCU_DSP_EN_SFT: c_int = 7;
pub const IRQ6_MCU_DSP_EN_SFT: c_int = 6;
pub const IRQ5_MCU_DSP_EN_SFT: c_int = 5;
pub const IRQ4_MCU_DSP_EN_SFT: c_int = 4;
pub const IRQ3_MCU_DSP_EN_SFT: c_int = 3;
pub const IRQ2_MCU_DSP_EN_SFT: c_int = 2;
pub const IRQ1_MCU_DSP_EN_SFT: c_int = 1;
pub const IRQ0_MCU_DSP_EN_SFT: c_int = 0;
// AFE_AUD_PAD_TOP
pub const AUD_PAD_TOP_MON_SFT: c_int = 15;

pub const AUD_PAD_TOP_FIFO_RSP_SFT: c_int = 4;

pub const RG_RX_PROTOCOL2_SFT: c_int = 3;

pub const RESERVDED_01_SFT: c_int = 1;

pub const RG_RX_FIFO_ON_SFT: c_int = 0;

// AFE_ADDA_MTKAIF_SYNCWORD_CFG
pub const RG_ADDA6_MTKAIF_RX_SYNC_WORD2_DISABLE_SFT: c_int = 23;

// AFE_ADDA_MTKAIF_RX_CFG0
pub const MTKAIF_RXIF_VOICE_MODE_SFT: c_int = 20;

pub const MTKAIF_RXIF_DETECT_ON_SFT: c_int = 16;

pub const MTKAIF_RXIF_DATA_BIT_SFT: c_int = 8;

pub const MTKAIF_RXIF_FIFO_RSP_SFT: c_int = 4;

pub const MTKAIF_RXIF_DATA_MODE_SFT: c_int = 0;

// GENERAL_ASRC_MODE
pub const GENERAL2_ASRCOUT_MODE_SFT: c_int = 12;
pub const GENERAL2_ASRCOUT_MODE_MASK: c_uint = 0xf;

pub const GENERAL2_ASRCIN_MODE_SFT: c_int = 8;
pub const GENERAL2_ASRCIN_MODE_MASK: c_uint = 0xf;

pub const GENERAL1_ASRCOUT_MODE_SFT: c_int = 4;
pub const GENERAL1_ASRCOUT_MODE_MASK: c_uint = 0xf;

pub const GENERAL1_ASRCIN_MODE_SFT: c_int = 0;
pub const GENERAL1_ASRCIN_MODE_MASK: c_uint = 0xf;

// GENERAL_ASRC_EN_ON
pub const GENERAL2_ASRC_EN_ON_SFT: c_int = 1;

pub const GENERAL1_ASRC_EN_ON_SFT: c_int = 0;

// AFE_GENERAL1_ASRC_2CH_CON0
pub const G_SRC_CHSET_STR_CLR_SFT: c_int = 4;

pub const G_SRC_CHSET_ON_SFT: c_int = 2;

pub const G_SRC_COEFF_SRAM_CTRL_SFT: c_int = 1;

pub const G_SRC_ASM_ON_SFT: c_int = 0;

// AFE_GENERAL1_ASRC_2CH_CON3
pub const G_SRC_ASM_FREQ_4_SFT: c_int = 0;

// AFE_GENERAL1_ASRC_2CH_CON4
pub const G_SRC_ASM_FREQ_5_SFT: c_int = 0;

// AFE_GENERAL1_ASRC_2CH_CON13
pub const G_SRC_COEFF_SRAM_ADR_SFT: c_int = 0;

// AFE_GENERAL1_ASRC_2CH_CON2
pub const G_SRC_CHSET_O16BIT_SFT: c_int = 19;

pub const G_SRC_CHSET_CLR_IIR_HISTORY_SFT: c_int = 17;

pub const G_SRC_CHSET_IS_MONO_SFT: c_int = 16;

pub const G_SRC_CHSET_IIR_EN_SFT: c_int = 11;

pub const G_SRC_CHSET_IIR_STAGE_SFT: c_int = 8;

pub const G_SRC_CHSET_STR_CLR_RU_SFT: c_int = 5;

pub const G_SRC_CHSET_ON_SFT: c_int = 2;

pub const G_SRC_COEFF_SRAM_CTRL_SFT: c_int = 1;

pub const G_SRC_ASM_ON_SFT: c_int = 0;

// AFE_ADDA_DL_SDM_DITHER_CON
pub const AFE_DL_SDM_DITHER_64TAP_EN_SFT: c_int = 20;

pub const AFE_DL_SDM_DITHER_EN_SFT: c_int = 16;

pub const AFE_DL_SDM_DITHER_GAIN_SFT: c_int = 0;

// AFE_ADDA_DL_SDM_AUTO_RESET_CON
pub const SDM_AUTO_RESET_TEST_ON_SFT: c_int = 31;

pub const AFE_DL_USE_NEW_2ND_SDM_SFT: c_int = 28;

pub const SDM_AUTO_RESET_COUNT_TH_SFT: c_int = 0;

// AFE_ASRC_2CH_CON0
pub const CON0_CHSET_STR_CLR_SFT: c_int = 4;

pub const CON0_ASM_ON_SFT: c_int = 0;

// AFE_ASRC_2CH_CON5
pub const CALI_EN_SFT: c_int = 0;

// FPGA_CFG4
pub const IRQ_COUNTER_SFT: c_int = 3;

pub const IRQ_CLK_COUNTER_CLEAN_SFT: c_int = 2;

pub const IRQ_CLK_COUNTER_PAUSE_SFT: c_int = 1;

pub const IRQ_CLK_COUNTER_ON_SFT: c_int = 0;

// FPGA_CFG5
pub const WR_MSTR_ON_SFT: c_int = 16;

pub const WR_AG_SEL_SFT: c_int = 0;

// FPGA_CFG6
pub const WR_MSTR_REQ_REAL_SFT: c_int = 16;

pub const WR_MSTR_REQ_IN_SFT: c_int = 0;

// FPGA_CFG7
pub const MEM1_WDATA_MON0_SFT: c_int = 0;

// FPGA_CFG8
pub const MEM1_WDATA_MON1_SFT: c_int = 0;

// FPGA_CFG9
pub const MEM_WE_SFT: c_int = 31;

pub const AFE_HREADY_SFT: c_int = 30;

pub const MEM_WR_REQ_SFT: c_int = 29;

pub const WR_AG_REG_MON_SFT: c_int = 16;

pub const HCLK_CK_SFT: c_int = 15;

pub const MEM_RD_REQ_SFT: c_int = 14;

pub const RD_AG_REQ_MON_SFT: c_int = 0;

// FPGA_CFG10
pub const MEM_BYTE_0_SFT: c_int = 0;

// FPGA_CFG11
pub const MEM_BYTE_1_SFT: c_int = 0;

// FPGA_CFG12
pub const RDATA_CNT_SFT: c_int = 30;

pub const MS2_HREADY_SFT: c_int = 29;

pub const MS1_HREADY_SFT: c_int = 28;

pub const AG_SEL_SFT: c_int = 0;

// FPGA_CFG13
pub const AFE_ST_SFT: c_int = 27;

pub const AG_IN_SERVICE_SFT: c_int = 0;

// ETDM_IN1_CON0
pub const ETDM_IN1_CON0_REG_ETDM_IN_EN_SFT: c_int = 0;

pub const ETDM_IN1_CON0_REG_SYNC_MODE_SFT: c_int = 1;

pub const ETDM_IN1_CON0_REG_LSB_FIRST_SFT: c_int = 3;

pub const ETDM_IN1_CON0_REG_SOFT_RST_SFT: c_int = 4;

pub const ETDM_IN1_CON0_REG_SLAVE_MODE_SFT: c_int = 5;

pub const ETDM_IN1_CON0_REG_FMT_SFT: c_int = 6;

pub const ETDM_IN1_CON0_REG_LRCK_EDGE_SEL_SFT: c_int = 10;

pub const ETDM_IN1_CON0_REG_BIT_LENGTH_SFT: c_int = 11;

pub const ETDM_IN1_CON0_REG_WORD_LENGTH_SFT: c_int = 16;

pub const ETDM_IN1_CON0_REG_CH_NUM_SFT: c_int = 23;

pub const ETDM_IN1_CON0_REG_RELATCH_1X_EN_SEL_DOMAIN_SFT: c_int = 28;

pub const ETDM_IN1_CON0_REG_VALID_TOGETHER_SFT: c_int = 31;

pub const ETDM_IN_CON0_CTRL_MASK: c_uint = 0x1f9ff9e2;
// ETDM_IN1_CON1
pub const ETDM_IN1_CON1_REG_INITIAL_COUNT_SFT: c_int = 0;

pub const ETDM_IN1_CON1_REG_INITIAL_POINT_SFT: c_int = 5;

pub const ETDM_IN1_CON1_REG_LRCK_AUTO_OFF_SFT: c_int = 10;

pub const ETDM_IN1_CON1_REG_BCK_AUTO_OFF_SFT: c_int = 11;

pub const ETDM_IN1_CON1_REG_INITIAL_LRCK_SFT: c_int = 13;

pub const ETDM_IN1_CON1_REG_LRCK_RESET_SFT: c_int = 15;

pub const ETDM_IN1_CON1_PINMUX_MCLK_CTRL_OE_SFT: c_int = 16;

pub const ETDM_IN1_CON1_REG_OUTPUT_CR_EN_SFT: c_int = 18;

pub const ETDM_IN1_CON1_REG_LR_ALIGN_SFT: c_int = 19;

pub const ETDM_IN1_CON1_REG_LRCK_WIDTH_SFT: c_int = 20;

pub const ETDM_IN1_CON1_REG_DIRECT_INPUT_MASTER_BCK_SFT: c_int = 30;

pub const ETDM_IN1_CON1_REG_LRCK_AUTO_MODE_SFT: c_int = 31;

pub const ETDM_IN_CON1_CTRL_MASK: c_uint = 0xbff10000;
// ETDM_IN1_CON2
pub const ETDM_IN1_CON2_REG_UPDATE_POINT_SFT: c_int = 0;

pub const ETDM_IN1_CON2_REG_UPDATE_GAP_SFT: c_int = 5;

pub const ETDM_IN1_CON2_REG_CLOCK_SOURCE_SEL_SFT: c_int = 10;

pub const ETDM_IN1_CON2_REG_AGENT_USE_ETDM_BCK_SFT: c_int = 13;

pub const ETDM_IN1_CON2_REG_CK_EN_SEL_AUTO_SFT: c_int = 14;

pub const ETDM_IN1_CON2_REG_MULTI_IP_ONE_DATA_CH_NUM_SFT: c_int = 15;

pub const ETDM_IN1_CON2_REG_MASK_AUTO_SFT: c_int = 20;

pub const ETDM_IN1_CON2_REG_MASK_NUM_SFT: c_int = 21;

pub const ETDM_IN1_CON2_REG_UPDATE_POINT_AUTO_SFT: c_int = 26;

pub const ETDM_IN1_CON2_REG_SDATA_DELAY_0P5T_EN_SFT: c_int = 27;

pub const ETDM_IN1_CON2_REG_SDATA_DELAY_BCK_INV_SFT: c_int = 28;

pub const ETDM_IN1_CON2_REG_LRCK_DELAY_0P5T_EN_SFT: c_int = 29;

pub const ETDM_IN1_CON2_REG_LRCK_DELAY_BCK_INV_SFT: c_int = 30;

pub const ETDM_IN1_CON2_REG_MULTI_IP_MODE_SFT: c_int = 31;

pub const ETDM_IN_CON2_CTRL_MASK: c_uint = 0x800f8000;

// ETDM_IN1_CON3
pub const ETDM_IN1_CON3_REG_DISABLE_OUT_0_SFT: c_int = 0;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_1_SFT: c_int = 1;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_2_SFT: c_int = 2;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_3_SFT: c_int = 3;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_4_SFT: c_int = 4;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_5_SFT: c_int = 5;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_6_SFT: c_int = 6;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_7_SFT: c_int = 7;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_8_SFT: c_int = 8;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_9_SFT: c_int = 9;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_10_SFT: c_int = 10;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_11_SFT: c_int = 11;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_12_SFT: c_int = 12;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_13_SFT: c_int = 13;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_14_SFT: c_int = 14;

pub const ETDM_IN1_CON3_REG_DISABLE_OUT_15_SFT: c_int = 15;

pub const ETDM_IN1_CON3_REG_RJ_DATA_RIGHT_ALIGN_SFT: c_int = 16;

pub const ETDM_IN1_CON3_REG_MONITOR_SEL_SFT: c_int = 17;

pub const ETDM_IN1_CON3_REG_CNT_UPPER_LIMIT_SFT: c_int = 19;

pub const ETDM_IN1_CON3_REG_COMPACT_SAMPLE_END_DIS_SFT: c_int = 25;

pub const ETDM_IN1_CON3_REG_FS_TIMING_SEL_SFT: c_int = 26;

pub const ETDM_IN1_CON3_REG_SAMPLE_END_MODE_SFT: c_int = 31;

// ETDM_IN1_CON4
pub const ETDM_IN1_CON4_REG_DSD_MODE_SFT: c_int = 0;

pub const ETDM_IN1_CON4_REG_DSD_REPACK_AUTO_MODE_SFT: c_int = 8;

pub const ETDM_IN1_CON4_REG_REPACK_WORD_LENGTH_SFT: c_int = 9;

pub const ETDM_IN1_CON4_REG_ASYNC_RESET_SFT: c_int = 11;

pub const ETDM_IN1_CON4_REG_DSD_CHNUM_SFT: c_int = 12;

pub const ETDM_IN1_CON4_REG_SLAVE_BCK_INV_SFT: c_int = 16;

pub const ETDM_IN1_CON4_REG_SLAVE_LRCK_INV_SFT: c_int = 17;

pub const ETDM_IN1_CON4_REG_MASTER_BCK_INV_SFT: c_int = 18;

pub const ETDM_IN1_CON4_REG_MASTER_LRCK_INV_SFT: c_int = 19;

pub const ETDM_IN1_CON4_REG_RELATCH_1X_EN_SEL_SFT: c_int = 20;

pub const ETDM_IN1_CON4_REG_SAMPLE_END_POINT_SFT: c_int = 25;

pub const ETDM_IN1_CON4_REG_WAIT_LAST_SAMPLE_SFT: c_int = 30;

pub const ETDM_IN1_CON4_REG_MASTER_BCK_FORCE_ON_SFT: c_int = 31;

pub const ETDM_IN_CON4_CTRL_MASK: c_uint = 0x1ff0000;

// ETDM_IN1_CON5
pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_0_SFT: c_int = 0;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_1_SFT: c_int = 1;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_2_SFT: c_int = 2;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_3_SFT: c_int = 3;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_4_SFT: c_int = 4;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_5_SFT: c_int = 5;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_6_SFT: c_int = 6;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_7_SFT: c_int = 7;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_8_SFT: c_int = 8;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_9_SFT: c_int = 9;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_10_SFT: c_int = 10;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_11_SFT: c_int = 11;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_12_SFT: c_int = 12;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_13_SFT: c_int = 13;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_14_SFT: c_int = 14;

pub const ETDM_IN1_CON5_REG_ODD_FLAG_EN_15_SFT: c_int = 15;

pub const ETDM_IN1_CON5_REG_LR_SWAP_0_SFT: c_int = 16;

pub const ETDM_IN1_CON5_REG_LR_SWAP_1_SFT: c_int = 17;

pub const ETDM_IN1_CON5_REG_LR_SWAP_2_SFT: c_int = 18;

pub const ETDM_IN1_CON5_REG_LR_SWAP_3_SFT: c_int = 19;

pub const ETDM_IN1_CON5_REG_LR_SWAP_4_SFT: c_int = 20;

pub const ETDM_IN1_CON5_REG_LR_SWAP_5_SFT: c_int = 21;

pub const ETDM_IN1_CON5_REG_LR_SWAP_6_SFT: c_int = 22;

pub const ETDM_IN1_CON5_REG_LR_SWAP_7_SFT: c_int = 23;

pub const ETDM_IN1_CON5_REG_LR_SWAP_8_SFT: c_int = 24;

pub const ETDM_IN1_CON5_REG_LR_SWAP_9_SFT: c_int = 25;

pub const ETDM_IN1_CON5_REG_LR_SWAP_10_SFT: c_int = 26;

pub const ETDM_IN1_CON5_REG_LR_SWAP_11_SFT: c_int = 27;

pub const ETDM_IN1_CON5_REG_LR_SWAP_12_SFT: c_int = 28;

pub const ETDM_IN1_CON5_REG_LR_SWAP_13_SFT: c_int = 29;

pub const ETDM_IN1_CON5_REG_LR_SWAP_14_SFT: c_int = 30;

pub const ETDM_IN1_CON5_REG_LR_SWAP_15_SFT: c_int = 31;

// ETDM_IN1_CON6
pub const ETDM_IN1_CON6_LCH_DATA_REG_SFT: c_int = 0;

// ETDM_IN1_CON7
pub const ETDM_IN1_CON7_RCH_DATA_REG_SFT: c_int = 0;

// ETDM_IN1_CON8
pub const ETDM_IN1_CON8_REG_AFIFO_THRESHOLD_SFT: c_int = 29;

pub const ETDM_IN1_CON8_REG_CK_EN_SEL_MANUAL_SFT: c_int = 16;

pub const ETDM_IN1_CON8_REG_AFIFO_SW_RESET_SFT: c_int = 15;

pub const ETDM_IN1_CON8_REG_AFIFO_RESET_SEL_SFT: c_int = 14;

pub const ETDM_IN1_CON8_REG_AFIFO_AUTO_RESET_DIS_SFT: c_int = 9;

pub const ETDM_IN1_CON8_REG_ETDM_USE_AFIFO_SFT: c_int = 8;

pub const ETDM_IN1_CON8_REG_AFIFO_CLOCK_DOMAIN_SEL_SFT: c_int = 5;

pub const ETDM_IN1_CON8_REG_AFIFO_MODE_SFT: c_int = 0;

pub const ETDM_IN_CON8_CTRL_MASK: c_uint = 0x13f;
pub const AUDIO_TOP_CON0: c_uint = 0x0000;
pub const AUDIO_TOP_CON1: c_uint = 0x0004;
pub const AUDIO_TOP_CON2: c_uint = 0x0008;
pub const AUDIO_TOP_CON3: c_uint = 0x000c;
pub const AFE_DAC_CON0: c_uint = 0x0010;
pub const AFE_I2S_CON: c_uint = 0x0018;
pub const AFE_CONN0: c_uint = 0x0020;
pub const AFE_CONN1: c_uint = 0x0024;
pub const AFE_CONN2: c_uint = 0x0028;
pub const AFE_CONN3: c_uint = 0x002c;
pub const AFE_CONN4: c_uint = 0x0030;
pub const AFE_I2S_CON1: c_uint = 0x0034;
pub const AFE_I2S_CON2: c_uint = 0x0038;
pub const AFE_I2S_CON3: c_uint = 0x0040;
pub const AFE_CONN5: c_uint = 0x0044;
pub const AFE_CONN_24BIT: c_uint = 0x0048;
pub const AFE_DL1_CON0: c_uint = 0x004c;
pub const AFE_DL1_BASE_MSB: c_uint = 0x0050;
pub const AFE_DL1_BASE: c_uint = 0x0054;
pub const AFE_DL1_CUR_MSB: c_uint = 0x0058;
pub const AFE_DL1_CUR: c_uint = 0x005c;
pub const AFE_DL1_END_MSB: c_uint = 0x0060;
pub const AFE_DL1_END: c_uint = 0x0064;
pub const AFE_DL2_CON0: c_uint = 0x0068;
pub const AFE_DL2_BASE_MSB: c_uint = 0x006c;
pub const AFE_DL2_BASE: c_uint = 0x0070;
pub const AFE_DL2_CUR_MSB: c_uint = 0x0074;
pub const AFE_DL2_CUR: c_uint = 0x0078;
pub const AFE_DL2_END_MSB: c_uint = 0x007c;
pub const AFE_DL2_END: c_uint = 0x0080;
pub const AFE_DL3_CON0: c_uint = 0x0084;
pub const AFE_DL3_BASE_MSB: c_uint = 0x0088;
pub const AFE_DL3_BASE: c_uint = 0x008c;
pub const AFE_DL3_CUR_MSB: c_uint = 0x0090;
pub const AFE_DL3_CUR: c_uint = 0x0094;
pub const AFE_DL3_END_MSB: c_uint = 0x0098;
pub const AFE_DL3_END: c_uint = 0x009c;
pub const AFE_CONN6: c_uint = 0x00bc;
pub const AFE_DL4_CON0: c_uint = 0x00cc;
pub const AFE_DL4_BASE_MSB: c_uint = 0x00d0;
pub const AFE_DL4_BASE: c_uint = 0x00d4;
pub const AFE_DL4_CUR_MSB: c_uint = 0x00d8;
pub const AFE_DL4_CUR: c_uint = 0x00dc;
pub const AFE_DL4_END_MSB: c_uint = 0x00e0;
pub const AFE_DL4_END: c_uint = 0x00e4;
pub const AFE_DL12_CON0: c_uint = 0x00e8;
pub const AFE_DL12_BASE_MSB: c_uint = 0x00ec;
pub const AFE_DL12_BASE: c_uint = 0x00f0;
pub const AFE_DL12_CUR_MSB: c_uint = 0x00f4;
pub const AFE_DL12_CUR: c_uint = 0x00f8;
pub const AFE_DL12_END_MSB: c_uint = 0x00fc;
pub const AFE_DL12_END: c_uint = 0x0100;
pub const AFE_ADDA_DL_SRC2_CON0: c_uint = 0x0108;
pub const AFE_ADDA_DL_SRC2_CON1: c_uint = 0x010c;
pub const AFE_ADDA_UL_SRC_CON0: c_uint = 0x0114;
pub const AFE_ADDA_UL_SRC_CON1: c_uint = 0x0118;
pub const AFE_ADDA_TOP_CON0: c_uint = 0x0120;
pub const AFE_ADDA_UL_DL_CON0: c_uint = 0x0124;
pub const AFE_ADDA_SRC_DEBUG: c_uint = 0x012c;
pub const AFE_ADDA_SRC_DEBUG_MON0: c_uint = 0x0130;
pub const AFE_ADDA_SRC_DEBUG_MON1: c_uint = 0x0134;
pub const AFE_ADDA_UL_SRC_MON0: c_uint = 0x0148;
pub const AFE_ADDA_UL_SRC_MON1: c_uint = 0x014c;
pub const AFE_SECURE_CON0: c_uint = 0x0150;
pub const AFE_SRAM_BOUND: c_uint = 0x0154;
pub const AFE_SECURE_CON1: c_uint = 0x0158;
pub const AFE_SECURE_CONN0: c_uint = 0x015c;
pub const AFE_VUL_CON0: c_uint = 0x0170;
pub const AFE_VUL_BASE_MSB: c_uint = 0x0174;
pub const AFE_VUL_BASE: c_uint = 0x0178;
pub const AFE_VUL_CUR_MSB: c_uint = 0x017c;
pub const AFE_VUL_CUR: c_uint = 0x0180;
pub const AFE_VUL_END_MSB: c_uint = 0x0184;
pub const AFE_VUL_END: c_uint = 0x0188;
pub const AFE_SIDETONE_DEBUG: c_uint = 0x01d0;
pub const AFE_SIDETONE_MON: c_uint = 0x01d4;
pub const AFE_SINEGEN_CON2: c_uint = 0x01dc;
pub const AFE_SIDETONE_CON0: c_uint = 0x01e0;
pub const AFE_SIDETONE_COEFF: c_uint = 0x01e4;
pub const AFE_SIDETONE_CON1: c_uint = 0x01e8;
pub const AFE_SIDETONE_GAIN: c_uint = 0x01ec;
pub const AFE_SINEGEN_CON0: c_uint = 0x01f0;
pub const AFE_TOP_CON0: c_uint = 0x0200;
pub const AFE_VUL2_CON0: c_uint = 0x020c;
pub const AFE_VUL2_BASE_MSB: c_uint = 0x0210;
pub const AFE_VUL2_BASE: c_uint = 0x0214;
pub const AFE_VUL2_CUR_MSB: c_uint = 0x0218;
pub const AFE_VUL2_CUR: c_uint = 0x021c;
pub const AFE_VUL2_END_MSB: c_uint = 0x0220;
pub const AFE_VUL2_END: c_uint = 0x0224;
pub const AFE_VUL3_CON0: c_uint = 0x0228;
pub const AFE_VUL3_BASE_MSB: c_uint = 0x022c;
pub const AFE_VUL3_BASE: c_uint = 0x0230;
pub const AFE_VUL3_CUR_MSB: c_uint = 0x0234;
pub const AFE_VUL3_CUR: c_uint = 0x0238;
pub const AFE_VUL3_END_MSB: c_uint = 0x023c;
pub const AFE_VUL3_END: c_uint = 0x0240;
pub const AFE_BUSY: c_uint = 0x0244;
pub const AFE_BUS_CFG: c_uint = 0x0250;
pub const AFE_ADDA_PREDIS_CON0: c_uint = 0x0260;
pub const AFE_ADDA_PREDIS_CON1: c_uint = 0x0264;
pub const AFE_I2S_MON: c_uint = 0x027c;
pub const AFE_ADDA_IIR_COEF_02_01: c_uint = 0x0290;
pub const AFE_ADDA_IIR_COEF_04_03: c_uint = 0x0294;
pub const AFE_ADDA_IIR_COEF_06_05: c_uint = 0x0298;
pub const AFE_ADDA_IIR_COEF_08_07: c_uint = 0x029c;
pub const AFE_ADDA_IIR_COEF_10_09: c_uint = 0x02a0;
pub const AFE_IRQ_MCU_CON1: c_uint = 0x02e4;
pub const AFE_IRQ_MCU_CON2: c_uint = 0x02e8;
pub const AFE_DAC_MON: c_uint = 0x02ec;
pub const AFE_IRQ_MCU_CON3: c_uint = 0x02f0;
pub const AFE_IRQ_MCU_CON4: c_uint = 0x02f4;
pub const AFE_IRQ_MCU_CNT0: c_uint = 0x0300;
pub const AFE_IRQ_MCU_CNT6: c_uint = 0x0304;
pub const AFE_IRQ_MCU_CNT8: c_uint = 0x0308;
pub const AFE_IRQ_MCU_DSP2_EN: c_uint = 0x030c;
pub const AFE_IRQ0_MCU_CNT_MON: c_uint = 0x0310;
pub const AFE_IRQ6_MCU_CNT_MON: c_uint = 0x0314;
pub const AFE_VUL4_CON0: c_uint = 0x0358;
pub const AFE_VUL4_BASE_MSB: c_uint = 0x035c;
pub const AFE_VUL4_BASE: c_uint = 0x0360;
pub const AFE_VUL4_CUR_MSB: c_uint = 0x0364;
pub const AFE_VUL4_CUR: c_uint = 0x0368;
pub const AFE_VUL4_END_MSB: c_uint = 0x036c;
pub const AFE_VUL4_END: c_uint = 0x0370;
pub const AFE_VUL12_CON0: c_uint = 0x0374;
pub const AFE_VUL12_BASE_MSB: c_uint = 0x0378;
pub const AFE_VUL12_BASE: c_uint = 0x037c;
pub const AFE_VUL12_CUR_MSB: c_uint = 0x0380;
pub const AFE_VUL12_CUR: c_uint = 0x0384;
pub const AFE_VUL12_END_MSB: c_uint = 0x0388;
pub const AFE_VUL12_END: c_uint = 0x038c;
pub const AFE_IRQ3_MCU_CNT_MON: c_uint = 0x0398;
pub const AFE_IRQ4_MCU_CNT_MON: c_uint = 0x039c;
pub const AFE_IRQ_MCU_CON0: c_uint = 0x03a0;
pub const AFE_IRQ_MCU_STATUS: c_uint = 0x03a4;
pub const AFE_IRQ_MCU_CLR: c_uint = 0x03a8;
pub const AFE_IRQ_MCU_CNT1: c_uint = 0x03ac;
pub const AFE_IRQ_MCU_CNT2: c_uint = 0x03b0;
pub const AFE_IRQ_MCU_EN: c_uint = 0x03b4;
pub const AFE_IRQ_MCU_MON2: c_uint = 0x03b8;
pub const AFE_IRQ_MCU_CNT5: c_uint = 0x03bc;
pub const AFE_IRQ1_MCU_CNT_MON: c_uint = 0x03c0;
pub const AFE_IRQ2_MCU_CNT_MON: c_uint = 0x03c4;
pub const AFE_IRQ5_MCU_CNT_MON: c_uint = 0x03cc;
pub const AFE_IRQ_MCU_DSP_EN: c_uint = 0x03d0;
pub const AFE_IRQ_MCU_SCP_EN: c_uint = 0x03d4;
pub const AFE_IRQ_MCU_CNT7: c_uint = 0x03dc;
pub const AFE_IRQ7_MCU_CNT_MON: c_uint = 0x03e0;
pub const AFE_IRQ_MCU_CNT3: c_uint = 0x03e4;
pub const AFE_IRQ_MCU_CNT4: c_uint = 0x03e8;
pub const AFE_IRQ_MCU_CNT11: c_uint = 0x03ec;
pub const AFE_APLL1_TUNER_CFG: c_uint = 0x03f0;
pub const AFE_APLL2_TUNER_CFG: c_uint = 0x03f4;
pub const AFE_IRQ_MCU_MISS_CLR: c_uint = 0x03f8;
pub const AFE_CONN33: c_uint = 0x0408;
pub const AFE_IRQ_MCU_CNT12: c_uint = 0x040c;
pub const AFE_GAIN1_CON0: c_uint = 0x0410;
pub const AFE_GAIN1_CON1: c_uint = 0x0414;
pub const AFE_GAIN1_CON2: c_uint = 0x0418;
pub const AFE_GAIN1_CON3: c_uint = 0x041c;
pub const AFE_CONN7: c_uint = 0x0420;
pub const AFE_GAIN1_CUR: c_uint = 0x0424;
pub const AFE_GAIN2_CON0: c_uint = 0x0428;
pub const AFE_GAIN2_CON1: c_uint = 0x042c;
pub const AFE_GAIN2_CON2: c_uint = 0x0430;
pub const AFE_GAIN2_CON3: c_uint = 0x0434;
pub const AFE_CONN8: c_uint = 0x0438;
pub const AFE_GAIN2_CUR: c_uint = 0x043c;
pub const AFE_CONN9: c_uint = 0x0440;
pub const AFE_CONN10: c_uint = 0x0444;
pub const AFE_CONN11: c_uint = 0x0448;
pub const AFE_CONN12: c_uint = 0x044c;
pub const AFE_CONN13: c_uint = 0x0450;
pub const AFE_CONN14: c_uint = 0x0454;
pub const AFE_CONN15: c_uint = 0x0458;
pub const AFE_CONN16: c_uint = 0x045c;
pub const AFE_CONN17: c_uint = 0x0460;
pub const AFE_CONN18: c_uint = 0x0464;
pub const AFE_CONN19: c_uint = 0x0468;
pub const AFE_CONN20: c_uint = 0x046c;
pub const AFE_CONN21: c_uint = 0x0470;
pub const AFE_CONN22: c_uint = 0x0474;
pub const AFE_CONN23: c_uint = 0x0478;
pub const AFE_CONN24: c_uint = 0x047c;
pub const AFE_CONN_RS: c_uint = 0x0494;
pub const AFE_CONN_DI: c_uint = 0x0498;
pub const AFE_CONN25: c_uint = 0x04b0;
pub const AFE_CONN26: c_uint = 0x04b4;
pub const AFE_CONN27: c_uint = 0x04b8;
pub const AFE_CONN28: c_uint = 0x04bc;
pub const AFE_CONN29: c_uint = 0x04c0;
pub const AFE_CONN30: c_uint = 0x04c4;
pub const AFE_CONN31: c_uint = 0x04c8;
pub const AFE_CONN32: c_uint = 0x04cc;
pub const AFE_SRAM_DELSEL_CON1: c_uint = 0x04f4;
pub const AFE_CONN56: c_uint = 0x0500;
pub const AFE_CONN57: c_uint = 0x0504;
pub const AFE_CONN58: c_uint = 0x0508;
pub const AFE_CONN59: c_uint = 0x050c;
pub const AFE_CONN56_1: c_uint = 0x0510;
pub const AFE_CONN57_1: c_uint = 0x0514;
pub const AFE_CONN58_1: c_uint = 0x0518;
pub const AFE_CONN59_1: c_uint = 0x051c;
pub const PCM_INTF_CON1: c_uint = 0x0530;
pub const PCM_INTF_CON2: c_uint = 0x0538;
pub const PCM2_INTF_CON: c_uint = 0x053c;
pub const AFE_CM1_CON: c_uint = 0x0550;
pub const AFE_CONN34: c_uint = 0x0580;
pub const FPGA_CFG0: c_uint = 0x05b0;
pub const FPGA_CFG1: c_uint = 0x05b4;
pub const FPGA_CFG2: c_uint = 0x05c0;
pub const FPGA_CFG3: c_uint = 0x05c4;
pub const AUDIO_TOP_DBG_CON: c_uint = 0x05c8;
pub const AUDIO_TOP_DBG_MON0: c_uint = 0x05cc;
pub const AUDIO_TOP_DBG_MON1: c_uint = 0x05d0;
pub const AFE_IRQ8_MCU_CNT_MON: c_uint = 0x05e4;
pub const AFE_IRQ11_MCU_CNT_MON: c_uint = 0x05e8;
pub const AFE_IRQ12_MCU_CNT_MON: c_uint = 0x05ec;
pub const AFE_IRQ_MCU_CNT9: c_uint = 0x0600;
pub const AFE_IRQ_MCU_CNT10: c_uint = 0x0604;
pub const AFE_IRQ_MCU_CNT13: c_uint = 0x0608;
pub const AFE_IRQ_MCU_CNT14: c_uint = 0x060c;
pub const AFE_IRQ_MCU_CNT15: c_uint = 0x0610;
pub const AFE_IRQ_MCU_CNT16: c_uint = 0x0614;
pub const AFE_IRQ_MCU_CNT17: c_uint = 0x0618;
pub const AFE_IRQ_MCU_CNT18: c_uint = 0x061c;
pub const AFE_IRQ_MCU_CNT19: c_uint = 0x0620;
pub const AFE_IRQ_MCU_CNT20: c_uint = 0x0624;
pub const AFE_IRQ_MCU_CNT21: c_uint = 0x0628;
pub const AFE_IRQ_MCU_CNT22: c_uint = 0x062c;
pub const AFE_IRQ_MCU_CNT23: c_uint = 0x0630;
pub const AFE_IRQ_MCU_CNT24: c_uint = 0x0634;
pub const AFE_IRQ_MCU_CNT25: c_uint = 0x0638;
pub const AFE_IRQ_MCU_CNT26: c_uint = 0x063c;
pub const AFE_IRQ9_MCU_CNT_MON: c_uint = 0x0660;
pub const AFE_IRQ10_MCU_CNT_MON: c_uint = 0x0664;
pub const AFE_IRQ13_MCU_CNT_MON: c_uint = 0x0668;
pub const AFE_IRQ14_MCU_CNT_MON: c_uint = 0x066c;
pub const AFE_IRQ15_MCU_CNT_MON: c_uint = 0x0670;
pub const AFE_IRQ16_MCU_CNT_MON: c_uint = 0x0674;
pub const AFE_IRQ17_MCU_CNT_MON: c_uint = 0x0678;
pub const AFE_IRQ18_MCU_CNT_MON: c_uint = 0x067c;
pub const AFE_IRQ19_MCU_CNT_MON: c_uint = 0x0680;
pub const AFE_IRQ20_MCU_CNT_MON: c_uint = 0x0684;
pub const AFE_IRQ21_MCU_CNT_MON: c_uint = 0x0688;
pub const AFE_IRQ22_MCU_CNT_MON: c_uint = 0x068c;
pub const AFE_IRQ23_MCU_CNT_MON: c_uint = 0x0690;
pub const AFE_IRQ24_MCU_CNT_MON: c_uint = 0x0694;
pub const AFE_IRQ25_MCU_CNT_MON: c_uint = 0x0698;
pub const AFE_IRQ26_MCU_CNT_MON: c_uint = 0x069c;
pub const AFE_IRQ31_MCU_CNT_MON: c_uint = 0x06a0;
pub const AFE_GENERAL_REG0: c_uint = 0x0800;
pub const AFE_GENERAL_REG1: c_uint = 0x0804;
pub const AFE_GENERAL_REG2: c_uint = 0x0808;
pub const AFE_GENERAL_REG3: c_uint = 0x080c;
pub const AFE_GENERAL_REG4: c_uint = 0x0810;
pub const AFE_GENERAL_REG5: c_uint = 0x0814;
pub const AFE_GENERAL_REG6: c_uint = 0x0818;
pub const AFE_GENERAL_REG7: c_uint = 0x081c;
pub const AFE_GENERAL_REG8: c_uint = 0x0820;
pub const AFE_GENERAL_REG9: c_uint = 0x0824;
pub const AFE_GENERAL_REG10: c_uint = 0x0828;
pub const AFE_GENERAL_REG11: c_uint = 0x082c;
pub const AFE_GENERAL_REG12: c_uint = 0x0830;
pub const AFE_GENERAL_REG13: c_uint = 0x0834;
pub const AFE_GENERAL_REG14: c_uint = 0x0838;
pub const AFE_GENERAL_REG15: c_uint = 0x083c;
pub const AFE_CBIP_CFG0: c_uint = 0x0840;
pub const AFE_CBIP_MON0: c_uint = 0x0844;
pub const AFE_CBIP_SLV_MUX_MON0: c_uint = 0x0848;
pub const AFE_CBIP_SLV_DECODER_MON0: c_uint = 0x084c;
pub const AFE_ADDA6_MTKAIF_MON0: c_uint = 0x0854;
pub const AFE_ADDA6_MTKAIF_MON1: c_uint = 0x0858;
pub const AFE_AWB_CON0: c_uint = 0x085c;
pub const AFE_AWB_BASE_MSB: c_uint = 0x0860;
pub const AFE_AWB_BASE: c_uint = 0x0864;
pub const AFE_AWB_CUR_MSB: c_uint = 0x0868;
pub const AFE_AWB_CUR: c_uint = 0x086c;
pub const AFE_AWB_END_MSB: c_uint = 0x0870;
pub const AFE_AWB_END: c_uint = 0x0874;
pub const AFE_AWB2_CON0: c_uint = 0x0878;
pub const AFE_AWB2_BASE_MSB: c_uint = 0x087c;
pub const AFE_AWB2_BASE: c_uint = 0x0880;
pub const AFE_AWB2_CUR_MSB: c_uint = 0x0884;
pub const AFE_AWB2_CUR: c_uint = 0x0888;
pub const AFE_AWB2_END_MSB: c_uint = 0x088c;
pub const AFE_AWB2_END: c_uint = 0x0890;
pub const AFE_DAI_CON0: c_uint = 0x0894;
pub const AFE_DAI_BASE_MSB: c_uint = 0x0898;
pub const AFE_DAI_BASE: c_uint = 0x089c;
pub const AFE_DAI_CUR_MSB: c_uint = 0x08a0;
pub const AFE_DAI_CUR: c_uint = 0x08a4;
pub const AFE_DAI_END_MSB: c_uint = 0x08a8;
pub const AFE_DAI_END: c_uint = 0x08ac;
pub const AFE_DAI2_CON0: c_uint = 0x08b0;
pub const AFE_DAI2_BASE_MSB: c_uint = 0x08b4;
pub const AFE_DAI2_BASE: c_uint = 0x08b8;
pub const AFE_DAI2_CUR_MSB: c_uint = 0x08bc;
pub const AFE_DAI2_CUR: c_uint = 0x08c0;
pub const AFE_DAI2_END_MSB: c_uint = 0x08c4;
pub const AFE_DAI2_END: c_uint = 0x08c8;
pub const AFE_MEMIF_CON0: c_uint = 0x08cc;
pub const AFE_CONN0_1: c_uint = 0x0900;
pub const AFE_CONN1_1: c_uint = 0x0904;
pub const AFE_CONN2_1: c_uint = 0x0908;
pub const AFE_CONN3_1: c_uint = 0x090c;
pub const AFE_CONN4_1: c_uint = 0x0910;
pub const AFE_CONN5_1: c_uint = 0x0914;
pub const AFE_CONN6_1: c_uint = 0x0918;
pub const AFE_CONN7_1: c_uint = 0x091c;
pub const AFE_CONN8_1: c_uint = 0x0920;
pub const AFE_CONN9_1: c_uint = 0x0924;
pub const AFE_CONN10_1: c_uint = 0x0928;
pub const AFE_CONN11_1: c_uint = 0x092c;
pub const AFE_CONN12_1: c_uint = 0x0930;
pub const AFE_CONN13_1: c_uint = 0x0934;
pub const AFE_CONN14_1: c_uint = 0x0938;
pub const AFE_CONN15_1: c_uint = 0x093c;
pub const AFE_CONN16_1: c_uint = 0x0940;
pub const AFE_CONN17_1: c_uint = 0x0944;
pub const AFE_CONN18_1: c_uint = 0x0948;
pub const AFE_CONN19_1: c_uint = 0x094c;
pub const AFE_CONN20_1: c_uint = 0x0950;
pub const AFE_CONN21_1: c_uint = 0x0954;
pub const AFE_CONN22_1: c_uint = 0x0958;
pub const AFE_CONN23_1: c_uint = 0x095c;
pub const AFE_CONN24_1: c_uint = 0x0960;
pub const AFE_CONN25_1: c_uint = 0x0964;
pub const AFE_CONN26_1: c_uint = 0x0968;
pub const AFE_CONN27_1: c_uint = 0x096c;
pub const AFE_CONN28_1: c_uint = 0x0970;
pub const AFE_CONN29_1: c_uint = 0x0974;
pub const AFE_CONN30_1: c_uint = 0x0978;
pub const AFE_CONN31_1: c_uint = 0x097c;
pub const AFE_CONN32_1: c_uint = 0x0980;
pub const AFE_CONN33_1: c_uint = 0x0984;
pub const AFE_CONN34_1: c_uint = 0x0988;
pub const AFE_CONN_RS_1: c_uint = 0x098c;
pub const AFE_CONN_DI_1: c_uint = 0x0990;
pub const AFE_CONN_24BIT_1: c_uint = 0x0994;
pub const AFE_CONN_REG: c_uint = 0x0998;
pub const AFE_CONN35: c_uint = 0x09a0;
pub const AFE_CONN36: c_uint = 0x09a4;
pub const AFE_CONN37: c_uint = 0x09a8;
pub const AFE_CONN38: c_uint = 0x09ac;
pub const AFE_CONN35_1: c_uint = 0x09b0;
pub const AFE_CONN36_1: c_uint = 0x09b4;
pub const AFE_CONN37_1: c_uint = 0x09b8;
pub const AFE_CONN38_1: c_uint = 0x09bc;
pub const AFE_CONN39: c_uint = 0x09c0;
pub const AFE_CONN40: c_uint = 0x09c4;
pub const AFE_CONN41: c_uint = 0x09c8;
pub const AFE_CONN42: c_uint = 0x09cc;
pub const AFE_CONN39_1: c_uint = 0x09e0;
pub const AFE_CONN40_1: c_uint = 0x09e4;
pub const AFE_CONN41_1: c_uint = 0x09e8;
pub const AFE_CONN42_1: c_uint = 0x09ec;
pub const AFE_I2S_CON4: c_uint = 0x09f8;
pub const AFE_CONN60: c_uint = 0x0a64;
pub const AFE_CONN61: c_uint = 0x0a68;
pub const AFE_CONN62: c_uint = 0x0a6c;
pub const AFE_CONN63: c_uint = 0x0a70;
pub const AFE_CONN64: c_uint = 0x0a74;
pub const AFE_CONN65: c_uint = 0x0a78;
pub const AFE_CONN66: c_uint = 0x0a7c;
pub const AFE_ADDA6_TOP_CON0: c_uint = 0x0a80;
pub const AFE_ADDA6_UL_SRC_CON0: c_uint = 0x0a84;
pub const AFE_ADDA6_UL_SRC_CON1: c_uint = 0x0a88;
pub const AFE_ADDA6_SRC_DEBUG: c_uint = 0x0a8c;
pub const AFE_ADDA6_SRC_DEBUG_MON0: c_uint = 0x0a90;
pub const AFE_ADDA6_ULCF_CFG_02_01: c_uint = 0x0aa0;
pub const AFE_ADDA6_ULCF_CFG_04_03: c_uint = 0x0aa4;
pub const AFE_ADDA6_ULCF_CFG_06_05: c_uint = 0x0aa8;
pub const AFE_ADDA6_ULCF_CFG_08_07: c_uint = 0x0aac;
pub const AFE_ADDA6_ULCF_CFG_10_09: c_uint = 0x0ab0;
pub const AFE_ADDA6_ULCF_CFG_12_11: c_uint = 0x0ab4;
pub const AFE_ADDA6_ULCF_CFG_14_13: c_uint = 0x0ab8;
pub const AFE_ADDA6_ULCF_CFG_16_15: c_uint = 0x0abc;
pub const AFE_ADDA6_ULCF_CFG_18_17: c_uint = 0x0ac0;
pub const AFE_ADDA6_ULCF_CFG_20_19: c_uint = 0x0ac4;
pub const AFE_ADDA6_ULCF_CFG_22_21: c_uint = 0x0ac8;
pub const AFE_ADDA6_ULCF_CFG_24_23: c_uint = 0x0acc;
pub const AFE_ADDA6_ULCF_CFG_26_25: c_uint = 0x0ad0;
pub const AFE_ADDA6_ULCF_CFG_28_27: c_uint = 0x0ad4;
pub const AFE_ADDA6_ULCF_CFG_30_29: c_uint = 0x0ad8;
pub const AFE_ADD6A_UL_SRC_MON0: c_uint = 0x0ae4;
pub const AFE_ADDA6_UL_SRC_MON1: c_uint = 0x0ae8;
pub const AFE_CONN43: c_uint = 0x0af8;
pub const AFE_CONN43_1: c_uint = 0x0afc;
pub const AFE_MOD_DAI_CON0: c_uint = 0x0b00;
pub const AFE_MOD_DAI_BASE_MSB: c_uint = 0x0b04;
pub const AFE_MOD_DAI_BASE: c_uint = 0x0b08;
pub const AFE_MOD_DAI_CUR_MSB: c_uint = 0x0b0c;
pub const AFE_MOD_DAI_CUR: c_uint = 0x0b10;
pub const AFE_MOD_DAI_END_MSB: c_uint = 0x0b14;
pub const AFE_MOD_DAI_END: c_uint = 0x0b18;
pub const AFE_AWB_RCH_MON: c_uint = 0x0b70;
pub const AFE_AWB_LCH_MON: c_uint = 0x0b74;
pub const AFE_VUL_RCH_MON: c_uint = 0x0b78;
pub const AFE_VUL_LCH_MON: c_uint = 0x0b7c;
pub const AFE_VUL12_RCH_MON: c_uint = 0x0b80;
pub const AFE_VUL12_LCH_MON: c_uint = 0x0b84;
pub const AFE_VUL2_RCH_MON: c_uint = 0x0b88;
pub const AFE_VUL2_LCH_MON: c_uint = 0x0b8c;
pub const AFE_DAI_DATA_MON: c_uint = 0x0b90;
pub const AFE_MOD_DAI_DATA_MON: c_uint = 0x0b94;
pub const AFE_DAI2_DATA_MON: c_uint = 0x0b98;
pub const AFE_AWB2_RCH_MON: c_uint = 0x0b9c;
pub const AFE_AWB2_LCH_MON: c_uint = 0x0ba0;
pub const AFE_VUL3_RCH_MON: c_uint = 0x0ba4;
pub const AFE_VUL3_LCH_MON: c_uint = 0x0ba8;
pub const AFE_VUL4_RCH_MON: c_uint = 0x0bac;
pub const AFE_VUL4_LCH_MON: c_uint = 0x0bb0;
pub const AFE_VUL5_RCH_MON: c_uint = 0x0bb4;
pub const AFE_VUL5_LCH_MON: c_uint = 0x0bb8;
pub const AFE_VUL6_RCH_MON: c_uint = 0x0bbc;
pub const AFE_VUL6_LCH_MON: c_uint = 0x0bc0;
pub const AFE_DL1_RCH_MON: c_uint = 0x0bc4;
pub const AFE_DL1_LCH_MON: c_uint = 0x0bc8;
pub const AFE_DL2_RCH_MON: c_uint = 0x0bcc;
pub const AFE_DL2_LCH_MON: c_uint = 0x0bd0;
pub const AFE_DL12_RCH1_MON: c_uint = 0x0bd4;
pub const AFE_DL12_LCH1_MON: c_uint = 0x0bd8;
pub const AFE_DL12_RCH2_MON: c_uint = 0x0bdc;
pub const AFE_DL12_LCH2_MON: c_uint = 0x0be0;
pub const AFE_DL3_RCH_MON: c_uint = 0x0be4;
pub const AFE_DL3_LCH_MON: c_uint = 0x0be8;
pub const AFE_DL4_RCH_MON: c_uint = 0x0bec;
pub const AFE_DL4_LCH_MON: c_uint = 0x0bf0;
pub const AFE_DL5_RCH_MON: c_uint = 0x0bf4;
pub const AFE_DL5_LCH_MON: c_uint = 0x0bf8;
pub const AFE_DL6_RCH_MON: c_uint = 0x0bfc;
pub const AFE_DL6_LCH_MON: c_uint = 0x0c00;
pub const AFE_DL7_RCH_MON: c_uint = 0x0c04;
pub const AFE_DL7_LCH_MON: c_uint = 0x0c08;
pub const AFE_DL8_RCH_MON: c_uint = 0x0c0c;
pub const AFE_DL8_LCH_MON: c_uint = 0x0c10;
pub const AFE_VUL5_CON0: c_uint = 0x0c14;
pub const AFE_VUL5_BASE_MSB: c_uint = 0x0c18;
pub const AFE_VUL5_BASE: c_uint = 0x0c1c;
pub const AFE_VUL5_CUR_MSB: c_uint = 0x0c20;
pub const AFE_VUL5_CUR: c_uint = 0x0c24;
pub const AFE_VUL5_END_MSB: c_uint = 0x0c28;
pub const AFE_VUL5_END: c_uint = 0x0c2c;
pub const AFE_VUL6_CON0: c_uint = 0x0c30;
pub const AFE_VUL6_BASE_MSB: c_uint = 0x0c34;
pub const AFE_VUL6_BASE: c_uint = 0x0c38;
pub const AFE_VUL6_CUR_MSB: c_uint = 0x0c3c;
pub const AFE_VUL6_CUR: c_uint = 0x0c40;
pub const AFE_VUL6_END_MSB: c_uint = 0x0c44;
pub const AFE_VUL6_END: c_uint = 0x0c48;
pub const AFE_ADDA_DL_SDM_DCCOMP_CON: c_uint = 0x0c50;
pub const AFE_ADDA_DL_SDM_TEST: c_uint = 0x0c54;
pub const AFE_ADDA_DL_DC_COMP_CFG0: c_uint = 0x0c58;
pub const AFE_ADDA_DL_DC_COMP_CFG1: c_uint = 0x0c5c;
pub const AFE_ADDA_DL_SDM_FIFO_MON: c_uint = 0x0c60;
pub const AFE_ADDA_DL_SRC_LCH_MON: c_uint = 0x0c64;
pub const AFE_ADDA_DL_SRC_RCH_MON: c_uint = 0x0c68;
pub const AFE_ADDA_DL_SDM_OUT_MON: c_uint = 0x0c6c;
pub const AFE_ADDA_DL_SDM_DITHER_CON: c_uint = 0x0c70;
pub const AFE_ADDA_DL_SDM_AUTO_RESET_CON: c_uint = 0x0c74;
pub const AFE_CONNSYS_I2S_CON: c_uint = 0x0c78;
pub const AFE_CONNSYS_I2S_MON: c_uint = 0x0c7c;
pub const AFE_ASRC_2CH_CON0: c_uint = 0x0c80;
pub const AFE_ASRC_2CH_CON1: c_uint = 0x0c84;
pub const AFE_ASRC_2CH_CON2: c_uint = 0x0c88;
pub const AFE_ASRC_2CH_CON3: c_uint = 0x0c8c;
pub const AFE_ASRC_2CH_CON4: c_uint = 0x0c90;
pub const AFE_ASRC_2CH_CON5: c_uint = 0x0c94;
pub const AFE_ASRC_2CH_CON6: c_uint = 0x0c98;
pub const AFE_ASRC_2CH_CON7: c_uint = 0x0c9c;
pub const AFE_ASRC_2CH_CON8: c_uint = 0x0ca0;
pub const AFE_ASRC_2CH_CON9: c_uint = 0x0ca4;
pub const AFE_ASRC_2CH_CON10: c_uint = 0x0ca8;
pub const AFE_ASRC_2CH_CON12: c_uint = 0x0cb0;
pub const AFE_ASRC_2CH_CON13: c_uint = 0x0cb4;
pub const AFE_ADDA6_IIR_COEF_02_01: c_uint = 0x0ce0;
pub const AFE_ADDA6_IIR_COEF_04_03: c_uint = 0x0ce4;
pub const AFE_ADDA6_IIR_COEF_06_05: c_uint = 0x0ce8;
pub const AFE_ADDA6_IIR_COEF_08_07: c_uint = 0x0cec;
pub const AFE_ADDA6_IIR_COEF_10_09: c_uint = 0x0cf0;
pub const AFE_CONN67: c_uint = 0x0cf4;
pub const AFE_CONN68: c_uint = 0x0cf8;
pub const AFE_CONN69: c_uint = 0x0cfc;
pub const AFE_SE_PROT_SIDEBAND: c_uint = 0x0d38;
pub const AFE_SE_DOMAIN_SIDEBAND0: c_uint = 0x0d3c;
pub const AFE_ADDA_PREDIS_CON2: c_uint = 0x0d40;
pub const AFE_ADDA_PREDIS_CON3: c_uint = 0x0d44;
pub const AFE_SE_DOMAIN_SIDEBAND1: c_uint = 0x0d54;
pub const AFE_SE_DOMAIN_SIDEBAND2: c_uint = 0x0d58;
pub const AFE_SE_DOMAIN_SIDEBAND3: c_uint = 0x0d5c;
pub const AFE_CONN44: c_uint = 0x0d70;
pub const AFE_CONN45: c_uint = 0x0d74;
pub const AFE_CONN46: c_uint = 0x0d78;
pub const AFE_CONN47: c_uint = 0x0d7c;
pub const AFE_CONN44_1: c_uint = 0x0d80;
pub const AFE_CONN45_1: c_uint = 0x0d84;
pub const AFE_CONN46_1: c_uint = 0x0d88;
pub const AFE_CONN47_1: c_uint = 0x0d8c;
pub const AFE_HD_ENGEN_ENABLE: c_uint = 0x0dd0;
pub const AFE_ADDA_DL_NLE_FIFO_MON: c_uint = 0x0dfc;
pub const AFE_ADDA_MTKAIF_CFG0: c_uint = 0x0e00;
pub const AFE_CONN67_1: c_uint = 0x0e04;
pub const AFE_CONN68_1: c_uint = 0x0e08;
pub const AFE_CONN69_1: c_uint = 0x0e0c;
pub const AFE_ADDA_MTKAIF_SYNCWORD_CFG: c_uint = 0x0e14;
pub const AFE_ADDA_MTKAIF_RX_CFG0: c_uint = 0x0e20;
pub const AFE_ADDA_MTKAIF_RX_CFG1: c_uint = 0x0e24;
pub const AFE_ADDA_MTKAIF_RX_CFG2: c_uint = 0x0e28;
pub const AFE_ADDA_MTKAIF_MON0: c_uint = 0x0e34;
pub const AFE_ADDA_MTKAIF_MON1: c_uint = 0x0e38;
pub const AFE_AUD_PAD_TOP: c_uint = 0x0e40;
pub const AFE_DL_NLE_R_CFG0: c_uint = 0x0e44;
pub const AFE_DL_NLE_R_CFG1: c_uint = 0x0e48;
pub const AFE_DL_NLE_L_CFG0: c_uint = 0x0e4c;
pub const AFE_DL_NLE_L_CFG1: c_uint = 0x0e50;
pub const AFE_DL_NLE_R_MON0: c_uint = 0x0e54;
pub const AFE_DL_NLE_R_MON1: c_uint = 0x0e58;
pub const AFE_DL_NLE_R_MON2: c_uint = 0x0e5c;
pub const AFE_DL_NLE_L_MON0: c_uint = 0x0e60;
pub const AFE_DL_NLE_L_MON1: c_uint = 0x0e64;
pub const AFE_DL_NLE_L_MON2: c_uint = 0x0e68;
pub const AFE_DL_NLE_GAIN_CFG0: c_uint = 0x0e6c;
pub const AFE_ADDA6_MTKAIF_CFG0: c_uint = 0x0e70;
pub const AFE_ADDA6_MTKAIF_RX_CFG0: c_uint = 0x0e74;
pub const AFE_ADDA6_MTKAIF_RX_CFG1: c_uint = 0x0e78;
pub const AFE_ADDA6_MTKAIF_RX_CFG2: c_uint = 0x0e7c;
pub const AFE_GENERAL1_ASRC_2CH_CON0: c_uint = 0x0e80;
pub const AFE_GENERAL1_ASRC_2CH_CON1: c_uint = 0x0e84;
pub const AFE_GENERAL1_ASRC_2CH_CON2: c_uint = 0x0e88;
pub const AFE_GENERAL1_ASRC_2CH_CON3: c_uint = 0x0e8c;
pub const AFE_GENERAL1_ASRC_2CH_CON4: c_uint = 0x0e90;
pub const AFE_GENERAL1_ASRC_2CH_CON5: c_uint = 0x0e94;
pub const AFE_GENERAL1_ASRC_2CH_CON6: c_uint = 0x0e98;
pub const AFE_GENERAL1_ASRC_2CH_CON7: c_uint = 0x0e9c;
pub const AFE_GENERAL1_ASRC_2CH_CON8: c_uint = 0x0ea0;
pub const AFE_GENERAL1_ASRC_2CH_CON9: c_uint = 0x0ea4;
pub const AFE_GENERAL1_ASRC_2CH_CON10: c_uint = 0x0ea8;
pub const AFE_GENERAL1_ASRC_2CH_CON12: c_uint = 0x0eb0;
pub const AFE_GENERAL1_ASRC_2CH_CON13: c_uint = 0x0eb4;
pub const GENERAL_ASRC_MODE: c_uint = 0x0eb8;
pub const GENERAL_ASRC_EN_ON: c_uint = 0x0ebc;
pub const AFE_CONN48: c_uint = 0x0ec0;
pub const AFE_CONN49: c_uint = 0x0ec4;
pub const AFE_CONN50: c_uint = 0x0ec8;
pub const AFE_CONN51: c_uint = 0x0ecc;
pub const AFE_CONN52: c_uint = 0x0ed0;
pub const AFE_CONN53: c_uint = 0x0ed4;
pub const AFE_CONN54: c_uint = 0x0ed8;
pub const AFE_CONN55: c_uint = 0x0edc;
pub const AFE_CONN48_1: c_uint = 0x0ee0;
pub const AFE_CONN49_1: c_uint = 0x0ee4;
pub const AFE_CONN50_1: c_uint = 0x0ee8;
pub const AFE_CONN51_1: c_uint = 0x0eec;
pub const AFE_CONN52_1: c_uint = 0x0ef0;
pub const AFE_CONN53_1: c_uint = 0x0ef4;
pub const AFE_CONN54_1: c_uint = 0x0ef8;
pub const AFE_CONN55_1: c_uint = 0x0efc;
pub const AFE_GENERAL2_ASRC_2CH_CON0: c_uint = 0x0f00;
pub const AFE_GENERAL2_ASRC_2CH_CON1: c_uint = 0x0f04;
pub const AFE_GENERAL2_ASRC_2CH_CON2: c_uint = 0x0f08;
pub const AFE_GENERAL2_ASRC_2CH_CON3: c_uint = 0x0f0c;
pub const AFE_GENERAL2_ASRC_2CH_CON4: c_uint = 0x0f10;
pub const AFE_GENERAL2_ASRC_2CH_CON5: c_uint = 0x0f14;
pub const AFE_GENERAL2_ASRC_2CH_CON6: c_uint = 0x0f18;
pub const AFE_GENERAL2_ASRC_2CH_CON7: c_uint = 0x0f1c;
pub const AFE_GENERAL2_ASRC_2CH_CON8: c_uint = 0x0f20;
pub const AFE_GENERAL2_ASRC_2CH_CON9: c_uint = 0x0f24;
pub const AFE_GENERAL2_ASRC_2CH_CON10: c_uint = 0x0f28;
pub const AFE_GENERAL2_ASRC_2CH_CON12: c_uint = 0x0f30;
pub const AFE_GENERAL2_ASRC_2CH_CON13: c_uint = 0x0f34;
pub const AFE_DL5_CON0: c_uint = 0x0f4c;
pub const AFE_DL5_BASE_MSB: c_uint = 0x0f50;
pub const AFE_DL5_BASE: c_uint = 0x0f54;
pub const AFE_DL5_CUR_MSB: c_uint = 0x0f58;
pub const AFE_DL5_CUR: c_uint = 0x0f5c;
pub const AFE_DL5_END_MSB: c_uint = 0x0f60;
pub const AFE_DL5_END: c_uint = 0x0f64;
pub const AFE_DL6_CON0: c_uint = 0x0f68;
pub const AFE_DL6_BASE_MSB: c_uint = 0x0f6c;
pub const AFE_DL6_BASE: c_uint = 0x0f70;
pub const AFE_DL6_CUR_MSB: c_uint = 0x0f74;
pub const AFE_DL6_CUR: c_uint = 0x0f78;
pub const AFE_DL6_END_MSB: c_uint = 0x0f7c;
pub const AFE_DL6_END: c_uint = 0x0f80;
pub const AFE_DL7_CON0: c_uint = 0x0f84;
pub const AFE_DL7_BASE_MSB: c_uint = 0x0f88;
pub const AFE_DL7_BASE: c_uint = 0x0f8c;
pub const AFE_DL7_CUR_MSB: c_uint = 0x0f90;
pub const AFE_DL7_CUR: c_uint = 0x0f94;
pub const AFE_DL7_END_MSB: c_uint = 0x0f98;
pub const AFE_DL7_END: c_uint = 0x0f9c;
pub const AFE_DL8_CON0: c_uint = 0x0fa0;
pub const AFE_DL8_BASE_MSB: c_uint = 0x0fa4;
pub const AFE_DL8_BASE: c_uint = 0x0fa8;
pub const AFE_DL8_CUR_MSB: c_uint = 0x0fac;
pub const AFE_DL8_CUR: c_uint = 0x0fb0;
pub const AFE_DL8_END_MSB: c_uint = 0x0fb4;
pub const AFE_DL8_END: c_uint = 0x0fb8;
pub const AFE_SE_SECURE_CON: c_uint = 0x1004;
pub const AFE_PROT_SIDEBAND_MON: c_uint = 0x1008;
pub const AFE_DOMAIN_SIDEBAND0_MON: c_uint = 0x100c;
pub const AFE_DOMAIN_SIDEBAND1_MON: c_uint = 0x1010;
pub const AFE_DOMAIN_SIDEBAND2_MON: c_uint = 0x1014;
pub const AFE_DOMAIN_SIDEBAND3_MON: c_uint = 0x1018;
pub const AFE_SECURE_MASK_CONN0: c_uint = 0x1020;
pub const AFE_SECURE_MASK_CONN1: c_uint = 0x1024;
pub const AFE_SECURE_MASK_CONN2: c_uint = 0x1028;
pub const AFE_SECURE_MASK_CONN3: c_uint = 0x102c;
pub const AFE_SECURE_MASK_CONN4: c_uint = 0x1030;
pub const AFE_SECURE_MASK_CONN5: c_uint = 0x1034;
pub const AFE_SECURE_MASK_CONN6: c_uint = 0x1038;
pub const AFE_SECURE_MASK_CONN7: c_uint = 0x103c;
pub const AFE_SECURE_MASK_CONN8: c_uint = 0x1040;
pub const AFE_SECURE_MASK_CONN9: c_uint = 0x1044;
pub const AFE_SECURE_MASK_CONN10: c_uint = 0x1048;
pub const AFE_SECURE_MASK_CONN11: c_uint = 0x104c;
pub const AFE_SECURE_MASK_CONN12: c_uint = 0x1050;
pub const AFE_SECURE_MASK_CONN13: c_uint = 0x1054;
pub const AFE_SECURE_MASK_CONN14: c_uint = 0x1058;
pub const AFE_SECURE_MASK_CONN15: c_uint = 0x105c;
pub const AFE_SECURE_MASK_CONN16: c_uint = 0x1060;
pub const AFE_SECURE_MASK_CONN17: c_uint = 0x1064;
pub const AFE_SECURE_MASK_CONN18: c_uint = 0x1068;
pub const AFE_SECURE_MASK_CONN19: c_uint = 0x106c;
pub const AFE_SECURE_MASK_CONN20: c_uint = 0x1070;
pub const AFE_SECURE_MASK_CONN21: c_uint = 0x1074;
pub const AFE_SECURE_MASK_CONN22: c_uint = 0x1078;
pub const AFE_SECURE_MASK_CONN23: c_uint = 0x107c;
pub const AFE_SECURE_MASK_CONN24: c_uint = 0x1080;
pub const AFE_SECURE_MASK_CONN25: c_uint = 0x1084;
pub const AFE_SECURE_MASK_CONN26: c_uint = 0x1088;
pub const AFE_SECURE_MASK_CONN27: c_uint = 0x108c;
pub const AFE_SECURE_MASK_CONN28: c_uint = 0x1090;
pub const AFE_SECURE_MASK_CONN29: c_uint = 0x1094;
pub const AFE_SECURE_MASK_CONN30: c_uint = 0x1098;
pub const AFE_SECURE_MASK_CONN31: c_uint = 0x109c;
pub const AFE_SECURE_MASK_CONN32: c_uint = 0x10a0;
pub const AFE_SECURE_MASK_CONN33: c_uint = 0x10a4;
pub const AFE_SECURE_MASK_CONN34: c_uint = 0x10a8;
pub const AFE_SECURE_MASK_CONN35: c_uint = 0x10ac;
pub const AFE_SECURE_MASK_CONN36: c_uint = 0x10b0;
pub const AFE_SECURE_MASK_CONN37: c_uint = 0x10b4;
pub const AFE_SECURE_MASK_CONN38: c_uint = 0x10b8;
pub const AFE_SECURE_MASK_CONN39: c_uint = 0x10bc;
pub const AFE_SECURE_MASK_CONN40: c_uint = 0x10c0;
pub const AFE_SECURE_MASK_CONN41: c_uint = 0x10c4;
pub const AFE_SECURE_MASK_CONN42: c_uint = 0x10c8;
pub const AFE_SECURE_MASK_CONN43: c_uint = 0x10cc;
pub const AFE_SECURE_MASK_CONN44: c_uint = 0x10d0;
pub const AFE_SECURE_MASK_CONN45: c_uint = 0x10d4;
pub const AFE_SECURE_MASK_CONN46: c_uint = 0x10d8;
pub const AFE_SECURE_MASK_CONN47: c_uint = 0x10dc;
pub const AFE_SECURE_MASK_CONN48: c_uint = 0x10e0;
pub const AFE_SECURE_MASK_CONN49: c_uint = 0x10e4;
pub const AFE_SECURE_MASK_CONN50: c_uint = 0x10e8;
pub const AFE_SECURE_MASK_CONN51: c_uint = 0x10ec;
pub const AFE_SECURE_MASK_CONN52: c_uint = 0x10f0;
pub const AFE_SECURE_MASK_CONN53: c_uint = 0x10f4;
pub const AFE_SECURE_MASK_CONN54: c_uint = 0x10f8;
pub const AFE_SECURE_MASK_CONN55: c_uint = 0x10fc;
pub const AFE_SECURE_MASK_CONN56: c_uint = 0x1100;
pub const AFE_SECURE_MASK_CONN57: c_uint = 0x1104;
pub const AFE_SECURE_MASK_CONN0_1: c_uint = 0x1108;
pub const AFE_SECURE_MASK_CONN1_1: c_uint = 0x110c;
pub const AFE_SECURE_MASK_CONN2_1: c_uint = 0x1110;
pub const AFE_SECURE_MASK_CONN3_1: c_uint = 0x1114;
pub const AFE_SECURE_MASK_CONN4_1: c_uint = 0x1118;
pub const AFE_SECURE_MASK_CONN5_1: c_uint = 0x111c;
pub const AFE_SECURE_MASK_CONN6_1: c_uint = 0x1120;
pub const AFE_SECURE_MASK_CONN7_1: c_uint = 0x1124;
pub const AFE_SECURE_MASK_CONN8_1: c_uint = 0x1128;
pub const AFE_SECURE_MASK_CONN9_1: c_uint = 0x112c;
pub const AFE_SECURE_MASK_CONN10_1: c_uint = 0x1130;
pub const AFE_SECURE_MASK_CONN11_1: c_uint = 0x1134;
pub const AFE_SECURE_MASK_CONN12_1: c_uint = 0x1138;
pub const AFE_SECURE_MASK_CONN13_1: c_uint = 0x113c;
pub const AFE_SECURE_MASK_CONN14_1: c_uint = 0x1140;
pub const AFE_SECURE_MASK_CONN15_1: c_uint = 0x1144;
pub const AFE_SECURE_MASK_CONN16_1: c_uint = 0x1148;
pub const AFE_SECURE_MASK_CONN17_1: c_uint = 0x114c;
pub const AFE_SECURE_MASK_CONN18_1: c_uint = 0x1150;
pub const AFE_SECURE_MASK_CONN19_1: c_uint = 0x1154;
pub const AFE_SECURE_MASK_CONN20_1: c_uint = 0x1158;
pub const AFE_SECURE_MASK_CONN21_1: c_uint = 0x115c;
pub const AFE_SECURE_MASK_CONN22_1: c_uint = 0x1160;
pub const AFE_SECURE_MASK_CONN23_1: c_uint = 0x1164;
pub const AFE_SECURE_MASK_CONN24_1: c_uint = 0x1168;
pub const AFE_SECURE_MASK_CONN25_1: c_uint = 0x116c;
pub const AFE_SECURE_MASK_CONN26_1: c_uint = 0x1170;
pub const AFE_SECURE_MASK_CONN27_1: c_uint = 0x1174;
pub const AFE_SECURE_MASK_CONN28_1: c_uint = 0x1178;
pub const AFE_SECURE_MASK_CONN29_1: c_uint = 0x117c;
pub const AFE_SECURE_MASK_CONN30_1: c_uint = 0x1180;
pub const AFE_SECURE_MASK_CONN31_1: c_uint = 0x1184;
pub const AFE_SECURE_MASK_CONN32_1: c_uint = 0x1188;
pub const AFE_SECURE_MASK_CONN33_1: c_uint = 0x118c;
pub const AFE_SECURE_MASK_CONN34_1: c_uint = 0x1190;
pub const AFE_SECURE_MASK_CONN35_1: c_uint = 0x1194;
pub const AFE_SECURE_MASK_CONN36_1: c_uint = 0x1198;
pub const AFE_SECURE_MASK_CONN37_1: c_uint = 0x119c;
pub const AFE_SECURE_MASK_CONN38_1: c_uint = 0x11a0;
pub const AFE_SECURE_MASK_CONN39_1: c_uint = 0x11a4;
pub const AFE_SECURE_MASK_CONN40_1: c_uint = 0x11a8;
pub const AFE_SECURE_MASK_CONN41_1: c_uint = 0x11ac;
pub const AFE_SECURE_MASK_CONN42_1: c_uint = 0x11b0;
pub const AFE_SECURE_MASK_CONN43_1: c_uint = 0x11b4;
pub const AFE_SECURE_MASK_CONN44_1: c_uint = 0x11b8;
pub const AFE_SECURE_MASK_CONN45_1: c_uint = 0x11bc;
pub const AFE_SECURE_MASK_CONN46_1: c_uint = 0x11c0;
pub const AFE_SECURE_MASK_CONN47_1: c_uint = 0x11c4;
pub const AFE_SECURE_MASK_CONN48_1: c_uint = 0x11c8;
pub const AFE_SECURE_MASK_CONN49_1: c_uint = 0x11cc;
pub const AFE_SECURE_MASK_CONN50_1: c_uint = 0x11d0;
pub const AFE_SECURE_MASK_CONN51_1: c_uint = 0x11d4;
pub const AFE_SECURE_MASK_CONN52_1: c_uint = 0x11d8;
pub const AFE_SECURE_MASK_CONN53_1: c_uint = 0x11dc;
pub const AFE_SECURE_MASK_CONN54_1: c_uint = 0x11e0;
pub const AFE_SECURE_MASK_CONN55_1: c_uint = 0x11e4;
pub const AFE_SECURE_MASK_CONN56_1: c_uint = 0x11e8;
pub const AFE_CONN60_1: c_uint = 0x11f0;
pub const AFE_CONN61_1: c_uint = 0x11f4;
pub const AFE_CONN62_1: c_uint = 0x11f8;
pub const AFE_CONN63_1: c_uint = 0x11fc;
pub const AFE_CONN64_1: c_uint = 0x1220;
pub const AFE_CONN65_1: c_uint = 0x1224;
pub const AFE_CONN66_1: c_uint = 0x1228;
pub const FPGA_CFG4: c_uint = 0x1230;
pub const FPGA_CFG5: c_uint = 0x1234;
pub const FPGA_CFG6: c_uint = 0x1238;
pub const FPGA_CFG7: c_uint = 0x123c;
pub const FPGA_CFG8: c_uint = 0x1240;
pub const FPGA_CFG9: c_uint = 0x1244;
pub const FPGA_CFG10: c_uint = 0x1248;
pub const FPGA_CFG11: c_uint = 0x124c;
pub const FPGA_CFG12: c_uint = 0x1250;
pub const FPGA_CFG13: c_uint = 0x1254;
pub const ETDM_IN1_CON0: c_uint = 0x1430;
pub const ETDM_IN1_CON1: c_uint = 0x1434;
pub const ETDM_IN1_CON2: c_uint = 0x1438;
pub const ETDM_IN1_CON3: c_uint = 0x143c;
pub const ETDM_IN1_CON4: c_uint = 0x1440;
pub const ETDM_IN1_CON5: c_uint = 0x1444;
pub const ETDM_IN1_CON6: c_uint = 0x1448;
pub const ETDM_IN1_CON7: c_uint = 0x144c;
pub const ETDM_IN1_CON8: c_uint = 0x1450;
pub const ETDM_OUT1_CON0: c_uint = 0x1454;
pub const ETDM_OUT1_CON1: c_uint = 0x1458;
pub const ETDM_OUT1_CON2: c_uint = 0x145c;
pub const ETDM_OUT1_CON3: c_uint = 0x1460;
pub const ETDM_OUT1_CON4: c_uint = 0x1464;
pub const ETDM_OUT1_CON5: c_uint = 0x1468;
pub const ETDM_OUT1_CON6: c_uint = 0x146c;
pub const ETDM_OUT1_CON7: c_uint = 0x1470;
pub const ETDM_OUT1_CON8: c_uint = 0x1474;
pub const ETDM_IN1_MON: c_uint = 0x1478;
pub const ETDM_OUT1_MON: c_uint = 0x147c;
pub const ETDM_0_3_COWORK_CON0: c_uint = 0x18b0;
pub const ETDM_0_3_COWORK_CON1: c_uint = 0x18b4;
pub const ETDM_0_3_COWORK_CON3: c_uint = 0x18bc;

pub const AFE_IRQ_STATUS_BITS: c_uint = 0x87FFFFFF;
pub const AFE_IRQ_CNT_SHIFT: c_int = 0;
pub const AFE_IRQ_CNT_MASK: c_uint = 0x3ffff;
