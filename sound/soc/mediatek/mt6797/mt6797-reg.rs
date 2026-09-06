//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt6797/mt6797-reg.h
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
// mt6797-reg.h  --  Mediatek 6797 audio driver reg definition
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
pub const AUDIO_TOP_CON0: c_uint = 0x0000;
pub const AUDIO_TOP_CON1: c_uint = 0x0004;
pub const AUDIO_TOP_CON3: c_uint = 0x000c;
pub const AFE_DAC_CON0: c_uint = 0x0010;
pub const AFE_DAC_CON1: c_uint = 0x0014;
pub const AFE_I2S_CON: c_uint = 0x0018;
pub const AFE_DAIBT_CON0: c_uint = 0x001c;
pub const AFE_CONN0: c_uint = 0x0020;
pub const AFE_CONN1: c_uint = 0x0024;
pub const AFE_CONN2: c_uint = 0x0028;
pub const AFE_CONN3: c_uint = 0x002c;
pub const AFE_CONN4: c_uint = 0x0030;
pub const AFE_I2S_CON1: c_uint = 0x0034;
pub const AFE_I2S_CON2: c_uint = 0x0038;
pub const AFE_MRGIF_CON: c_uint = 0x003c;
pub const AFE_DL1_BASE: c_uint = 0x0040;
pub const AFE_DL1_CUR: c_uint = 0x0044;
pub const AFE_DL1_END: c_uint = 0x0048;
pub const AFE_I2S_CON3: c_uint = 0x004c;
pub const AFE_DL2_BASE: c_uint = 0x0050;
pub const AFE_DL2_CUR: c_uint = 0x0054;
pub const AFE_DL2_END: c_uint = 0x0058;
pub const AFE_CONN5: c_uint = 0x005c;
pub const AFE_CONN_24BIT: c_uint = 0x006c;
pub const AFE_AWB_BASE: c_uint = 0x0070;
pub const AFE_AWB_END: c_uint = 0x0078;
pub const AFE_AWB_CUR: c_uint = 0x007c;
pub const AFE_VUL_BASE: c_uint = 0x0080;
pub const AFE_VUL_END: c_uint = 0x0088;
pub const AFE_VUL_CUR: c_uint = 0x008c;
pub const AFE_DAI_BASE: c_uint = 0x0090;
pub const AFE_DAI_END: c_uint = 0x0098;
pub const AFE_DAI_CUR: c_uint = 0x009c;
pub const AFE_CONN6: c_uint = 0x00bc;
pub const AFE_MEMIF_MSB: c_uint = 0x00cc;
pub const AFE_MEMIF_MON0: c_uint = 0x00d0;
pub const AFE_MEMIF_MON1: c_uint = 0x00d4;
pub const AFE_MEMIF_MON2: c_uint = 0x00d8;
pub const AFE_MEMIF_MON4: c_uint = 0x00e0;
pub const AFE_ADDA_DL_SRC2_CON0: c_uint = 0x0108;
pub const AFE_ADDA_DL_SRC2_CON1: c_uint = 0x010c;
pub const AFE_ADDA_UL_SRC_CON0: c_uint = 0x0114;
pub const AFE_ADDA_UL_SRC_CON1: c_uint = 0x0118;
pub const AFE_ADDA_TOP_CON0: c_uint = 0x0120;
pub const AFE_ADDA_UL_DL_CON0: c_uint = 0x0124;
pub const AFE_ADDA_SRC_DEBUG: c_uint = 0x012c;
pub const AFE_ADDA_SRC_DEBUG_MON0: c_uint = 0x0130;
pub const AFE_ADDA_SRC_DEBUG_MON1: c_uint = 0x0134;
pub const AFE_ADDA_NEWIF_CFG0: c_uint = 0x0138;
pub const AFE_ADDA_NEWIF_CFG1: c_uint = 0x013c;
pub const AFE_ADDA_NEWIF_CFG2: c_uint = 0x0140;
pub const AFE_DMA_CTL: c_uint = 0x0150;
pub const AFE_DMA_MON0: c_uint = 0x0154;
pub const AFE_DMA_MON1: c_uint = 0x0158;
pub const AFE_SIDETONE_DEBUG: c_uint = 0x01d0;
pub const AFE_SIDETONE_MON: c_uint = 0x01d4;
pub const AFE_SIDETONE_CON0: c_uint = 0x01e0;
pub const AFE_SIDETONE_COEFF: c_uint = 0x01e4;
pub const AFE_SIDETONE_CON1: c_uint = 0x01e8;
pub const AFE_SIDETONE_GAIN: c_uint = 0x01ec;
pub const AFE_SGEN_CON0: c_uint = 0x01f0;
pub const AFE_SINEGEN_CON_TDM: c_uint = 0x01fc;
pub const AFE_TOP_CON0: c_uint = 0x0200;
pub const AFE_ADDA_PREDIS_CON0: c_uint = 0x0260;
pub const AFE_ADDA_PREDIS_CON1: c_uint = 0x0264;
pub const AFE_MRGIF_MON0: c_uint = 0x0270;
pub const AFE_MRGIF_MON1: c_uint = 0x0274;
pub const AFE_MRGIF_MON2: c_uint = 0x0278;
pub const AFE_I2S_MON: c_uint = 0x027c;
pub const AFE_MOD_DAI_BASE: c_uint = 0x0330;
pub const AFE_MOD_DAI_END: c_uint = 0x0338;
pub const AFE_MOD_DAI_CUR: c_uint = 0x033c;
pub const AFE_VUL_D2_BASE: c_uint = 0x0350;
pub const AFE_VUL_D2_END: c_uint = 0x0358;
pub const AFE_VUL_D2_CUR: c_uint = 0x035c;
pub const AFE_DL3_BASE: c_uint = 0x0360;
pub const AFE_DL3_CUR: c_uint = 0x0364;
pub const AFE_DL3_END: c_uint = 0x0368;
pub const AFE_HDMI_OUT_CON0: c_uint = 0x0370;
pub const AFE_HDMI_BASE: c_uint = 0x0374;
pub const AFE_HDMI_CUR: c_uint = 0x0378;
pub const AFE_HDMI_END: c_uint = 0x037c;
pub const AFE_HDMI_CONN0: c_uint = 0x0390;
pub const AFE_IRQ3_MCU_CNT_MON: c_uint = 0x0398;
pub const AFE_IRQ4_MCU_CNT_MON: c_uint = 0x039c;
pub const AFE_IRQ_MCU_CON: c_uint = 0x03a0;
pub const AFE_IRQ_MCU_STATUS: c_uint = 0x03a4;
pub const AFE_IRQ_MCU_CLR: c_uint = 0x03a8;
pub const AFE_IRQ_MCU_CNT1: c_uint = 0x03ac;
pub const AFE_IRQ_MCU_CNT2: c_uint = 0x03b0;
pub const AFE_IRQ_MCU_EN: c_uint = 0x03b4;
pub const AFE_IRQ_MCU_MON2: c_uint = 0x03b8;
pub const AFE_IRQ_MCU_CNT5: c_uint = 0x03bc;
pub const AFE_IRQ1_MCU_CNT_MON: c_uint = 0x03c0;
pub const AFE_IRQ2_MCU_CNT_MON: c_uint = 0x03c4;
pub const AFE_IRQ1_MCU_EN_CNT_MON: c_uint = 0x03c8;
pub const AFE_IRQ5_MCU_CNT_MON: c_uint = 0x03cc;
pub const AFE_MEMIF_MINLEN: c_uint = 0x03d0;
pub const AFE_MEMIF_MAXLEN: c_uint = 0x03d4;
pub const AFE_MEMIF_PBUF_SIZE: c_uint = 0x03d8;
pub const AFE_IRQ_MCU_CNT7: c_uint = 0x03dc;
pub const AFE_IRQ7_MCU_CNT_MON: c_uint = 0x03e0;
pub const AFE_IRQ_MCU_CNT3: c_uint = 0x03e4;
pub const AFE_IRQ_MCU_CNT4: c_uint = 0x03e8;
pub const AFE_APLL1_TUNER_CFG: c_uint = 0x03f0;
pub const AFE_APLL2_TUNER_CFG: c_uint = 0x03f4;
pub const AFE_MEMIF_HD_MODE: c_uint = 0x03f8;
pub const AFE_MEMIF_HDALIGN: c_uint = 0x03fc;
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
pub const AFE_SRAM_DELSEL_CON0: c_uint = 0x04f0;
pub const AFE_SRAM_DELSEL_CON1: c_uint = 0x04f4;
pub const AFE_ASRC_CON0: c_uint = 0x0500;
pub const AFE_ASRC_CON1: c_uint = 0x0504;
pub const AFE_ASRC_CON2: c_uint = 0x0508;
pub const AFE_ASRC_CON3: c_uint = 0x050c;
pub const AFE_ASRC_CON4: c_uint = 0x0510;
pub const AFE_ASRC_CON5: c_uint = 0x0514;
pub const AFE_ASRC_CON6: c_uint = 0x0518;
pub const AFE_ASRC_CON7: c_uint = 0x051c;
pub const AFE_ASRC_CON8: c_uint = 0x0520;
pub const AFE_ASRC_CON9: c_uint = 0x0524;
pub const AFE_ASRC_CON10: c_uint = 0x0528;
pub const AFE_ASRC_CON11: c_uint = 0x052c;
pub const PCM_INTF_CON1: c_uint = 0x0530;
pub const PCM_INTF_CON2: c_uint = 0x0538;
pub const PCM2_INTF_CON: c_uint = 0x053c;
pub const AFE_TDM_CON1: c_uint = 0x0548;
pub const AFE_TDM_CON2: c_uint = 0x054c;
pub const AFE_ASRC_CON13: c_uint = 0x0550;
pub const AFE_ASRC_CON14: c_uint = 0x0554;
pub const AFE_ASRC_CON15: c_uint = 0x0558;
pub const AFE_ASRC_CON16: c_uint = 0x055c;
pub const AFE_ASRC_CON17: c_uint = 0x0560;
pub const AFE_ASRC_CON18: c_uint = 0x0564;
pub const AFE_ASRC_CON19: c_uint = 0x0568;
pub const AFE_ASRC_CON20: c_uint = 0x056c;
pub const AFE_ASRC_CON21: c_uint = 0x0570;
pub const CLK_AUDDIV_0: c_uint = 0x05a0;
pub const CLK_AUDDIV_1: c_uint = 0x05a4;
pub const CLK_AUDDIV_2: c_uint = 0x05a8;
pub const CLK_AUDDIV_3: c_uint = 0x05ac;
pub const AUDIO_TOP_DBG_CON: c_uint = 0x05c8;
pub const AUDIO_TOP_DBG_MON0: c_uint = 0x05cc;
pub const AUDIO_TOP_DBG_MON1: c_uint = 0x05d0;
pub const AUDIO_TOP_DBG_MON2: c_uint = 0x05d4;
pub const AFE_ADDA2_TOP_CON0: c_uint = 0x0600;
pub const AFE_ASRC4_CON0: c_uint = 0x06c0;
pub const AFE_ASRC4_CON1: c_uint = 0x06c4;
pub const AFE_ASRC4_CON2: c_uint = 0x06c8;
pub const AFE_ASRC4_CON3: c_uint = 0x06cc;
pub const AFE_ASRC4_CON4: c_uint = 0x06d0;
pub const AFE_ASRC4_CON5: c_uint = 0x06d4;
pub const AFE_ASRC4_CON6: c_uint = 0x06d8;
pub const AFE_ASRC4_CON7: c_uint = 0x06dc;
pub const AFE_ASRC4_CON8: c_uint = 0x06e0;
pub const AFE_ASRC4_CON9: c_uint = 0x06e4;
pub const AFE_ASRC4_CON10: c_uint = 0x06e8;
pub const AFE_ASRC4_CON11: c_uint = 0x06ec;
pub const AFE_ASRC4_CON12: c_uint = 0x06f0;
pub const AFE_ASRC4_CON13: c_uint = 0x06f4;
pub const AFE_ASRC4_CON14: c_uint = 0x06f8;
pub const AFE_ASRC2_CON0: c_uint = 0x0700;
pub const AFE_ASRC2_CON1: c_uint = 0x0704;
pub const AFE_ASRC2_CON2: c_uint = 0x0708;
pub const AFE_ASRC2_CON3: c_uint = 0x070c;
pub const AFE_ASRC2_CON4: c_uint = 0x0710;
pub const AFE_ASRC2_CON5: c_uint = 0x0714;
pub const AFE_ASRC2_CON6: c_uint = 0x0718;
pub const AFE_ASRC2_CON7: c_uint = 0x071c;
pub const AFE_ASRC2_CON8: c_uint = 0x0720;
pub const AFE_ASRC2_CON9: c_uint = 0x0724;
pub const AFE_ASRC2_CON10: c_uint = 0x0728;
pub const AFE_ASRC2_CON11: c_uint = 0x072c;
pub const AFE_ASRC2_CON12: c_uint = 0x0730;
pub const AFE_ASRC2_CON13: c_uint = 0x0734;
pub const AFE_ASRC2_CON14: c_uint = 0x0738;
pub const AFE_ASRC3_CON0: c_uint = 0x0740;
pub const AFE_ASRC3_CON1: c_uint = 0x0744;
pub const AFE_ASRC3_CON2: c_uint = 0x0748;
pub const AFE_ASRC3_CON3: c_uint = 0x074c;
pub const AFE_ASRC3_CON4: c_uint = 0x0750;
pub const AFE_ASRC3_CON5: c_uint = 0x0754;
pub const AFE_ASRC3_CON6: c_uint = 0x0758;
pub const AFE_ASRC3_CON7: c_uint = 0x075c;
pub const AFE_ASRC3_CON8: c_uint = 0x0760;
pub const AFE_ASRC3_CON9: c_uint = 0x0764;
pub const AFE_ASRC3_CON10: c_uint = 0x0768;
pub const AFE_ASRC3_CON11: c_uint = 0x076c;
pub const AFE_ASRC3_CON12: c_uint = 0x0770;
pub const AFE_ASRC3_CON13: c_uint = 0x0774;
pub const AFE_ASRC3_CON14: c_uint = 0x0778;
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

pub const AFE_IRQ_STATUS_BITS: c_uint = 0x5f;
// AUDIO_TOP_CON0
pub const AHB_IDLE_EN_INT_SFT: c_int = 30;
pub const AHB_IDLE_EN_INT_MASK: c_uint = 0x1;

pub const AHB_IDLE_EN_EXT_SFT: c_int = 29;
pub const AHB_IDLE_EN_EXT_MASK: c_uint = 0x1;

pub const PDN_TML_SFT: c_int = 27;
pub const PDN_TML_MASK: c_uint = 0x1;

pub const PDN_DAC_PREDIS_SFT: c_int = 26;
pub const PDN_DAC_PREDIS_MASK: c_uint = 0x1;

pub const PDN_DAC_SFT: c_int = 25;
pub const PDN_DAC_MASK: c_uint = 0x1;

pub const PDN_ADC_SFT: c_int = 24;
pub const PDN_ADC_MASK: c_uint = 0x1;

pub const PDN_TDM_CK_SFT: c_int = 20;
pub const PDN_TDM_CK_MASK: c_uint = 0x1;

pub const PDN_APLL_TUNER_SFT: c_int = 19;
pub const PDN_APLL_TUNER_MASK: c_uint = 0x1;

pub const PDN_APLL2_TUNER_SFT: c_int = 18;
pub const PDN_APLL2_TUNER_MASK: c_uint = 0x1;

pub const APB3_SEL_SFT: c_int = 14;
pub const APB3_SEL_MASK: c_uint = 0x1;

pub const APB_R2T_SFT: c_int = 13;
pub const APB_R2T_MASK: c_uint = 0x1;

pub const APB_W2T_SFT: c_int = 12;
pub const APB_W2T_MASK: c_uint = 0x1;

pub const PDN_24M_SFT: c_int = 9;
pub const PDN_24M_MASK: c_uint = 0x1;

pub const PDN_22M_SFT: c_int = 8;
pub const PDN_22M_MASK: c_uint = 0x1;

pub const PDN_ADDA4_ADC_SFT: c_int = 7;
pub const PDN_ADDA4_ADC_MASK: c_uint = 0x1;

pub const PDN_I2S_SFT: c_int = 6;
pub const PDN_I2S_MASK: c_uint = 0x1;

pub const PDN_AFE_SFT: c_int = 2;
pub const PDN_AFE_MASK: c_uint = 0x1;

// AUDIO_TOP_CON1
pub const PDN_ADC_HIRES_TML_SFT: c_int = 17;
pub const PDN_ADC_HIRES_TML_MASK: c_uint = 0x1;

pub const PDN_ADC_HIRES_SFT: c_int = 16;
pub const PDN_ADC_HIRES_MASK: c_uint = 0x1;

pub const I2S4_BCLK_SW_CG_SFT: c_int = 7;
pub const I2S4_BCLK_SW_CG_MASK: c_uint = 0x1;

pub const I2S3_BCLK_SW_CG_SFT: c_int = 6;
pub const I2S3_BCLK_SW_CG_MASK: c_uint = 0x1;

pub const I2S2_BCLK_SW_CG_SFT: c_int = 5;
pub const I2S2_BCLK_SW_CG_MASK: c_uint = 0x1;

pub const I2S1_BCLK_SW_CG_SFT: c_int = 4;
pub const I2S1_BCLK_SW_CG_MASK: c_uint = 0x1;

pub const I2S_SOFT_RST2_SFT: c_int = 2;
pub const I2S_SOFT_RST2_MASK: c_uint = 0x1;

pub const I2S_SOFT_RST_SFT: c_int = 1;
pub const I2S_SOFT_RST_MASK: c_uint = 0x1;

// AFE_DAC_CON0
pub const AFE_AWB_RETM_SFT: c_int = 31;
pub const AFE_AWB_RETM_MASK: c_uint = 0x1;

pub const AFE_DL1_DATA2_RETM_SFT: c_int = 30;
pub const AFE_DL1_DATA2_RETM_MASK: c_uint = 0x1;

pub const AFE_DL2_RETM_SFT: c_int = 29;
pub const AFE_DL2_RETM_MASK: c_uint = 0x1;

pub const AFE_DL1_RETM_SFT: c_int = 28;
pub const AFE_DL1_RETM_MASK: c_uint = 0x1;

pub const AFE_ON_RETM_SFT: c_int = 27;
pub const AFE_ON_RETM_MASK: c_uint = 0x1;

pub const MOD_DAI_DUP_WR_SFT: c_int = 26;
pub const MOD_DAI_DUP_WR_MASK: c_uint = 0x1;

pub const DAI_MODE_SFT: c_int = 24;
pub const DAI_MODE_MASK: c_uint = 0x3;

pub const VUL_DATA2_MODE_SFT: c_int = 20;
pub const VUL_DATA2_MODE_MASK: c_uint = 0xf;

pub const DL1_DATA2_MODE_SFT: c_int = 16;
pub const DL1_DATA2_MODE_MASK: c_uint = 0xf;

pub const DL3_MODE_SFT: c_int = 12;
pub const DL3_MODE_MASK: c_uint = 0xf;

pub const VUL_DATA2_R_MONO_SFT: c_int = 11;
pub const VUL_DATA2_R_MONO_MASK: c_uint = 0x1;

pub const VUL_DATA2_DATA_SFT: c_int = 10;
pub const VUL_DATA2_DATA_MASK: c_uint = 0x1;

pub const VUL_DATA2_ON_SFT: c_int = 9;
pub const VUL_DATA2_ON_MASK: c_uint = 0x1;

pub const DL1_DATA2_ON_SFT: c_int = 8;
pub const DL1_DATA2_ON_MASK: c_uint = 0x1;

pub const MOD_DAI_ON_SFT: c_int = 7;
pub const MOD_DAI_ON_MASK: c_uint = 0x1;

pub const AWB_ON_SFT: c_int = 6;
pub const AWB_ON_MASK: c_uint = 0x1;

pub const DL3_ON_SFT: c_int = 5;
pub const DL3_ON_MASK: c_uint = 0x1;

pub const DAI_ON_SFT: c_int = 4;
pub const DAI_ON_MASK: c_uint = 0x1;

pub const VUL_ON_SFT: c_int = 3;
pub const VUL_ON_MASK: c_uint = 0x1;

pub const DL2_ON_SFT: c_int = 2;
pub const DL2_ON_MASK: c_uint = 0x1;

pub const DL1_ON_SFT: c_int = 1;
pub const DL1_ON_MASK: c_uint = 0x1;

pub const AFE_ON_SFT: c_int = 0;
pub const AFE_ON_MASK: c_uint = 0x1;

// AFE_DAC_CON1
pub const MOD_DAI_MODE_SFT: c_int = 30;
pub const MOD_DAI_MODE_MASK: c_uint = 0x3;

pub const DAI_DUP_WR_SFT: c_int = 29;
pub const DAI_DUP_WR_MASK: c_uint = 0x1;

pub const VUL_R_MONO_SFT: c_int = 28;
pub const VUL_R_MONO_MASK: c_uint = 0x1;

pub const VUL_DATA_SFT: c_int = 27;
pub const VUL_DATA_MASK: c_uint = 0x1;

pub const AXI_2X1_CG_DISABLE_SFT: c_int = 26;
pub const AXI_2X1_CG_DISABLE_MASK: c_uint = 0x1;

pub const AWB_R_MONO_SFT: c_int = 25;
pub const AWB_R_MONO_MASK: c_uint = 0x1;

pub const AWB_DATA_SFT: c_int = 24;
pub const AWB_DATA_MASK: c_uint = 0x1;

pub const DL3_DATA_SFT: c_int = 23;
pub const DL3_DATA_MASK: c_uint = 0x1;

pub const DL2_DATA_SFT: c_int = 22;
pub const DL2_DATA_MASK: c_uint = 0x1;

pub const DL1_DATA_SFT: c_int = 21;
pub const DL1_DATA_MASK: c_uint = 0x1;

pub const DL1_DATA2_DATA_SFT: c_int = 20;
pub const DL1_DATA2_DATA_MASK: c_uint = 0x1;

pub const VUL_MODE_SFT: c_int = 16;
pub const VUL_MODE_MASK: c_uint = 0xf;

pub const AWB_MODE_SFT: c_int = 12;
pub const AWB_MODE_MASK: c_uint = 0xf;

pub const I2S_MODE_SFT: c_int = 8;
pub const I2S_MODE_MASK: c_uint = 0xf;

pub const DL2_MODE_SFT: c_int = 4;
pub const DL2_MODE_MASK: c_uint = 0xf;

pub const DL1_MODE_SFT: c_int = 0;
pub const DL1_MODE_MASK: c_uint = 0xf;

// AFE_ADDA_DL_SRC2_CON0
pub const DL_2_INPUT_MODE_CTL_SFT: c_int = 28;
pub const DL_2_INPUT_MODE_CTL_MASK: c_uint = 0xf;

pub const DL_2_CH1_SATURATION_EN_CTL_SFT: c_int = 27;
pub const DL_2_CH1_SATURATION_EN_CTL_MASK: c_uint = 0x1;

pub const DL_2_CH2_SATURATION_EN_CTL_SFT: c_int = 26;
pub const DL_2_CH2_SATURATION_EN_CTL_MASK: c_uint = 0x1;

pub const DL_2_OUTPUT_SEL_CTL_SFT: c_int = 24;
pub const DL_2_OUTPUT_SEL_CTL_MASK: c_uint = 0x3;

pub const DL_2_FADEIN_0START_EN_SFT: c_int = 16;
pub const DL_2_FADEIN_0START_EN_MASK: c_uint = 0x3;

pub const DL_DISABLE_HW_CG_CTL_SFT: c_int = 15;
pub const DL_DISABLE_HW_CG_CTL_MASK: c_uint = 0x1;

pub const C_DATA_EN_SEL_CTL_PRE_SFT: c_int = 14;
pub const C_DATA_EN_SEL_CTL_PRE_MASK: c_uint = 0x1;

pub const DL_2_SIDE_TONE_ON_CTL_PRE_SFT: c_int = 13;
pub const DL_2_SIDE_TONE_ON_CTL_PRE_MASK: c_uint = 0x1;

pub const DL_2_MUTE_CH1_OFF_CTL_PRE_SFT: c_int = 12;
pub const DL_2_MUTE_CH1_OFF_CTL_PRE_MASK: c_uint = 0x1;

pub const DL_2_MUTE_CH2_OFF_CTL_PRE_SFT: c_int = 11;
pub const DL_2_MUTE_CH2_OFF_CTL_PRE_MASK: c_uint = 0x1;

pub const DL2_ARAMPSP_CTL_PRE_SFT: c_int = 9;
pub const DL2_ARAMPSP_CTL_PRE_MASK: c_uint = 0x3;

pub const DL_2_IIRMODE_CTL_PRE_SFT: c_int = 6;
pub const DL_2_IIRMODE_CTL_PRE_MASK: c_uint = 0x7;

pub const DL_2_VOICE_MODE_CTL_PRE_SFT: c_int = 5;
pub const DL_2_VOICE_MODE_CTL_PRE_MASK: c_uint = 0x1;

pub const D2_2_MUTE_CH1_ON_CTL_PRE_SFT: c_int = 4;
pub const D2_2_MUTE_CH1_ON_CTL_PRE_MASK: c_uint = 0x1;

pub const D2_2_MUTE_CH2_ON_CTL_PRE_SFT: c_int = 3;
pub const D2_2_MUTE_CH2_ON_CTL_PRE_MASK: c_uint = 0x1;

pub const DL_2_IIR_ON_CTL_PRE_SFT: c_int = 2;
pub const DL_2_IIR_ON_CTL_PRE_MASK: c_uint = 0x1;

pub const DL_2_GAIN_ON_CTL_PRE_SFT: c_int = 1;
pub const DL_2_GAIN_ON_CTL_PRE_MASK: c_uint = 0x1;

pub const DL_2_SRC_ON_TMP_CTL_PRE_SFT: c_int = 0;
pub const DL_2_SRC_ON_TMP_CTL_PRE_MASK: c_uint = 0x1;

// AFE_ADDA_DL_SRC2_CON1
pub const DL_2_GAIN_CTL_PRE_SFT: c_int = 16;
pub const DL_2_GAIN_CTL_PRE_MASK: c_uint = 0xffff;

pub const DL_2_GAIN_MODE_CTL_SFT: c_int = 0;
pub const DL_2_GAIN_MODE_CTL_MASK: c_uint = 0x1;

// AFE_ADDA_UL_SRC_CON0
pub const C_COMB_OUT_SIN_GEN_CTL_SFT: c_int = 31;
pub const C_COMB_OUT_SIN_GEN_CTL_MASK: c_uint = 0x1;

pub const C_BASEBAND_SIN_GEN_CTL_SFT: c_int = 30;
pub const C_BASEBAND_SIN_GEN_CTL_MASK: c_uint = 0x1;

pub const C_DIGMIC_PHASE_SEL_CH1_CTL_SFT: c_int = 27;
pub const C_DIGMIC_PHASE_SEL_CH1_CTL_MASK: c_uint = 0x7;

pub const C_DIGMIC_PHASE_SEL_CH2_CTL_SFT: c_int = 24;
pub const C_DIGMIC_PHASE_SEL_CH2_CTL_MASK: c_uint = 0x7;

pub const C_TWO_DIGITAL_MIC_CTL_SFT: c_int = 23;
pub const C_TWO_DIGITAL_MIC_CTL_MASK: c_uint = 0x1;

pub const UL_MODE_3P25M_CH2_CTL_SFT: c_int = 22;
pub const UL_MODE_3P25M_CH2_CTL_MASK: c_uint = 0x1;

pub const UL_MODE_3P25M_CH1_CTL_SFT: c_int = 21;
pub const UL_MODE_3P25M_CH1_CTL_MASK: c_uint = 0x1;

pub const UL_SRC_USE_CIC_OUT_CTL_SFT: c_int = 20;
pub const UL_SRC_USE_CIC_OUT_CTL_MASK: c_uint = 0x1;

pub const UL_VOICE_MODE_CH1_CH2_CTL_SFT: c_int = 17;
pub const UL_VOICE_MODE_CH1_CH2_CTL_MASK: c_uint = 0x7;

pub const DMIC_LOW_POWER_MODE_CTL_SFT: c_int = 14;
pub const DMIC_LOW_POWER_MODE_CTL_MASK: c_uint = 0x3;

pub const DMIC_48K_SEL_CTL_SFT: c_int = 13;
pub const DMIC_48K_SEL_CTL_MASK: c_uint = 0x1;

pub const UL_DISABLE_HW_CG_CTL_SFT: c_int = 12;
pub const UL_DISABLE_HW_CG_CTL_MASK: c_uint = 0x1;

pub const UL_IIR_ON_TMP_CTL_SFT: c_int = 10;
pub const UL_IIR_ON_TMP_CTL_MASK: c_uint = 0x1;

pub const UL_IIRMODE_CTL_SFT: c_int = 7;
pub const UL_IIRMODE_CTL_MASK: c_uint = 0x7;

pub const DIGMIC_3P25M_1P625M_SEL_CTL_SFT: c_int = 5;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK: c_uint = 0x1;

pub const AGC_260K_SEL_CH2_CTL_SFT: c_int = 4;
pub const AGC_260K_SEL_CH2_CTL_MASK: c_uint = 0x1;

pub const AGC_260K_SEL_CH1_CTL_SFT: c_int = 3;
pub const AGC_260K_SEL_CH1_CTL_MASK: c_uint = 0x1;

pub const UL_LOOP_BACK_MODE_CTL_SFT: c_int = 2;
pub const UL_LOOP_BACK_MODE_CTL_MASK: c_uint = 0x1;

pub const UL_SDM_3_LEVEL_CTL_SFT: c_int = 1;
pub const UL_SDM_3_LEVEL_CTL_MASK: c_uint = 0x1;

pub const UL_SRC_ON_TMP_CTL_SFT: c_int = 0;
pub const UL_SRC_ON_TMP_CTL_MASK: c_uint = 0x1;

// AFE_ADDA_UL_SRC_CON1
pub const C_SDM_RESET_CTL_SFT: c_int = 31;
pub const C_SDM_RESET_CTL_MASK: c_uint = 0x1;

pub const ADITHON_CTL_SFT: c_int = 30;
pub const ADITHON_CTL_MASK: c_uint = 0x1;

pub const ADITHVAL_CTL_SFT: c_int = 28;
pub const ADITHVAL_CTL_MASK: c_uint = 0x3;

pub const C_DAC_EN_CTL_SFT: c_int = 27;
pub const C_DAC_EN_CTL_MASK: c_uint = 0x1;

pub const C_MUTE_SW_CTL_SFT: c_int = 26;
pub const C_MUTE_SW_CTL_MASK: c_uint = 0x1;

pub const ASDM_SRC_SEL_CTL_SFT: c_int = 25;
pub const ASDM_SRC_SEL_CTL_MASK: c_uint = 0x1;

pub const C_AMP_DIV_CH2_CTL_SFT: c_int = 21;
pub const C_AMP_DIV_CH2_CTL_MASK: c_uint = 0x7;

pub const C_FREQ_DIV_CH2_CTL_SFT: c_int = 16;
pub const C_FREQ_DIV_CH2_CTL_MASK: c_uint = 0x1f;

pub const C_SINE_MODE_CH2_CTL_SFT: c_int = 12;
pub const C_SINE_MODE_CH2_CTL_MASK: c_uint = 0xf;

pub const C_AMP_DIV_CH1_CTL_SFT: c_int = 9;
pub const C_AMP_DIV_CH1_CTL_MASK: c_uint = 0x7;

pub const C_FREQ_DIV_CH1_CTL_SFT: c_int = 4;
pub const C_FREQ_DIV_CH1_CTL_MASK: c_uint = 0x1f;

pub const C_SINE_MODE_CH1_CTL_SFT: c_int = 0;
pub const C_SINE_MODE_CH1_CTL_MASK: c_uint = 0xf;

// AFE_ADDA_TOP_CON0
pub const C_LOOP_BACK_MODE_CTL_SFT: c_int = 12;
pub const C_LOOP_BACK_MODE_CTL_MASK: c_uint = 0xf;

pub const C_EXT_ADC_CTL_SFT: c_int = 0;
pub const C_EXT_ADC_CTL_MASK: c_uint = 0x1;

// AFE_ADDA_UL_DL_CON0
pub const AFE_UL_DL_CON0_RESERVED_SFT: c_int = 1;
pub const AFE_UL_DL_CON0_RESERVED_MASK: c_uint = 0x3fff;

pub const ADDA_AFE_ON_SFT: c_int = 0;
pub const ADDA_AFE_ON_MASK: c_uint = 0x1;

// AFE_IRQ_MCU_CON
pub const IRQ7_MCU_MODE_SFT: c_int = 24;
pub const IRQ7_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ4_MCU_MODE_SFT: c_int = 20;
pub const IRQ4_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ3_MCU_MODE_SFT: c_int = 16;
pub const IRQ3_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ7_MCU_ON_SFT: c_int = 14;
pub const IRQ7_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ5_MCU_ON_SFT: c_int = 12;
pub const IRQ5_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ2_MCU_MODE_SFT: c_int = 8;
pub const IRQ2_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ1_MCU_MODE_SFT: c_int = 4;
pub const IRQ1_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ4_MCU_ON_SFT: c_int = 3;
pub const IRQ4_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ3_MCU_ON_SFT: c_int = 2;
pub const IRQ3_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ2_MCU_ON_SFT: c_int = 1;
pub const IRQ2_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ1_MCU_ON_SFT: c_int = 0;
pub const IRQ1_MCU_ON_MASK: c_uint = 0x1;

// AFE_IRQ_MCU_EN
pub const AFE_IRQ_CM4_EN_SFT: c_int = 16;
pub const AFE_IRQ_CM4_EN_MASK: c_uint = 0x7f;

pub const AFE_IRQ_MD32_EN_SFT: c_int = 8;
pub const AFE_IRQ_MD32_EN_MASK: c_uint = 0x7f;

pub const AFE_IRQ_MCU_EN_SFT: c_int = 0;
pub const AFE_IRQ_MCU_EN_MASK: c_uint = 0x7f;

// AFE_IRQ_MCU_CLR
pub const IRQ7_MCU_CLR_SFT: c_int = 6;
pub const IRQ7_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ5_MCU_CLR_SFT: c_int = 4;
pub const IRQ5_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ4_MCU_CLR_SFT: c_int = 3;
pub const IRQ4_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ3_MCU_CLR_SFT: c_int = 2;
pub const IRQ3_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ2_MCU_CLR_SFT: c_int = 1;
pub const IRQ2_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ1_MCU_CLR_SFT: c_int = 0;
pub const IRQ1_MCU_CLR_MASK: c_uint = 0x1;

// AFE_IRQ_MCU_CNT1
pub const AFE_IRQ_MCU_CNT1_SFT: c_int = 0;
pub const AFE_IRQ_MCU_CNT1_MASK: c_uint = 0x3ffff;

// AFE_IRQ_MCU_CNT2
pub const AFE_IRQ_MCU_CNT2_SFT: c_int = 0;
pub const AFE_IRQ_MCU_CNT2_MASK: c_uint = 0x3ffff;

// AFE_IRQ_MCU_CNT3
pub const AFE_IRQ_MCU_CNT3_SFT: c_int = 0;
pub const AFE_IRQ_MCU_CNT3_MASK: c_uint = 0x3ffff;

// AFE_IRQ_MCU_CNT4
pub const AFE_IRQ_MCU_CNT4_SFT: c_int = 0;
pub const AFE_IRQ_MCU_CNT4_MASK: c_uint = 0x3ffff;

// AFE_IRQ_MCU_CNT5
pub const AFE_IRQ_MCU_CNT5_SFT: c_int = 0;
pub const AFE_IRQ_MCU_CNT5_MASK: c_uint = 0x3ffff;

// AFE_IRQ_MCU_CNT7
pub const AFE_IRQ_MCU_CNT7_SFT: c_int = 0;
pub const AFE_IRQ_MCU_CNT7_MASK: c_uint = 0x3ffff;

// AFE_MEMIF_MSB
pub const CPU_COMPACT_MODE_SFT: c_int = 23;
pub const CPU_COMPACT_MODE_MASK: c_uint = 0x1;

pub const CPU_HD_ALIGN_SFT: c_int = 22;
pub const CPU_HD_ALIGN_MASK: c_uint = 0x1;

// AFE_MEMIF_HD_MODE
pub const HDMI_HD_SFT: c_int = 20;
pub const HDMI_HD_MASK: c_uint = 0x3;

pub const MOD_DAI_HD_SFT: c_int = 18;
pub const MOD_DAI_HD_MASK: c_uint = 0x3;

pub const DAI_HD_SFT: c_int = 16;
pub const DAI_HD_MASK: c_uint = 0x3;

pub const VUL_DATA2_HD_SFT: c_int = 12;
pub const VUL_DATA2_HD_MASK: c_uint = 0x3;

pub const VUL_HD_SFT: c_int = 10;
pub const VUL_HD_MASK: c_uint = 0x3;

pub const AWB_HD_SFT: c_int = 8;
pub const AWB_HD_MASK: c_uint = 0x3;

pub const DL3_HD_SFT: c_int = 6;
pub const DL3_HD_MASK: c_uint = 0x3;

pub const DL2_HD_SFT: c_int = 4;
pub const DL2_HD_MASK: c_uint = 0x3;

pub const DL1_DATA2_HD_SFT: c_int = 2;
pub const DL1_DATA2_HD_MASK: c_uint = 0x3;

pub const DL1_HD_SFT: c_int = 0;
pub const DL1_HD_MASK: c_uint = 0x3;

// AFE_MEMIF_HDALIGN
pub const HDMI_NORMAL_MODE_SFT: c_int = 26;
pub const HDMI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const MOD_DAI_NORMAL_MODE_SFT: c_int = 25;
pub const MOD_DAI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DAI_NORMAL_MODE_SFT: c_int = 24;
pub const DAI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL_DATA2_NORMAL_MODE_SFT: c_int = 22;
pub const VUL_DATA2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL_NORMAL_MODE_SFT: c_int = 21;
pub const VUL_NORMAL_MODE_MASK: c_uint = 0x1;

pub const AWB_NORMAL_MODE_SFT: c_int = 20;
pub const AWB_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL3_NORMAL_MODE_SFT: c_int = 19;
pub const DL3_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL2_NORMAL_MODE_SFT: c_int = 18;
pub const DL2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL1_DATA2_NORMAL_MODE_SFT: c_int = 17;
pub const DL1_DATA2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL1_NORMAL_MODE_SFT: c_int = 16;
pub const DL1_NORMAL_MODE_MASK: c_uint = 0x1;

pub const HDMI_HD_ALIGN_SFT: c_int = 10;
pub const HDMI_HD_ALIGN_MASK: c_uint = 0x1;

pub const MOD_DAI_HD_ALIGN_SFT: c_int = 9;
pub const MOD_DAI_HD_ALIGN_MASK: c_uint = 0x1;

pub const DAI_ALIGN_SFT: c_int = 8;
pub const DAI_ALIGN_MASK: c_uint = 0x1;

pub const VUL2_HD_ALIGN_SFT: c_int = 7;
pub const VUL2_HD_ALIGN_MASK: c_uint = 0x1;

pub const VUL_DATA2_HD_ALIGN_SFT: c_int = 6;
pub const VUL_DATA2_HD_ALIGN_MASK: c_uint = 0x1;

pub const VUL_HD_ALIGN_SFT: c_int = 5;
pub const VUL_HD_ALIGN_MASK: c_uint = 0x1;

pub const AWB_HD_ALIGN_SFT: c_int = 4;
pub const AWB_HD_ALIGN_MASK: c_uint = 0x1;

pub const DL3_HD_ALIGN_SFT: c_int = 3;
pub const DL3_HD_ALIGN_MASK: c_uint = 0x1;

pub const DL2_HD_ALIGN_SFT: c_int = 2;
pub const DL2_HD_ALIGN_MASK: c_uint = 0x1;

pub const DL1_DATA2_HD_ALIGN_SFT: c_int = 1;
pub const DL1_DATA2_HD_ALIGN_MASK: c_uint = 0x1;

pub const DL1_HD_ALIGN_SFT: c_int = 0;
pub const DL1_HD_ALIGN_MASK: c_uint = 0x1;

// PCM_INTF_CON1
pub const PCM_FIX_VALUE_SEL_SFT: c_int = 31;
pub const PCM_FIX_VALUE_SEL_MASK: c_uint = 0x1;

pub const PCM_BUFFER_LOOPBACK_SFT: c_int = 30;
pub const PCM_BUFFER_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM_PARALLEL_LOOPBACK_SFT: c_int = 29;
pub const PCM_PARALLEL_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM_SERIAL_LOOPBACK_SFT: c_int = 28;
pub const PCM_SERIAL_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM_DAI_PCM_LOOPBACK_SFT: c_int = 27;
pub const PCM_DAI_PCM_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM_I2S_PCM_LOOPBACK_SFT: c_int = 26;
pub const PCM_I2S_PCM_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM_SYNC_DELSEL_SFT: c_int = 25;
pub const PCM_SYNC_DELSEL_MASK: c_uint = 0x1;

pub const PCM_TX_LR_SWAP_SFT: c_int = 24;
pub const PCM_TX_LR_SWAP_MASK: c_uint = 0x1;

pub const PCM_SYNC_OUT_INV_SFT: c_int = 23;
pub const PCM_SYNC_OUT_INV_MASK: c_uint = 0x1;

pub const PCM_BCLK_OUT_INV_SFT: c_int = 22;
pub const PCM_BCLK_OUT_INV_MASK: c_uint = 0x1;

pub const PCM_SYNC_IN_INV_SFT: c_int = 21;
pub const PCM_SYNC_IN_INV_MASK: c_uint = 0x1;

pub const PCM_BCLK_IN_INV_SFT: c_int = 20;
pub const PCM_BCLK_IN_INV_MASK: c_uint = 0x1;

pub const PCM_TX_LCH_RPT_SFT: c_int = 19;
pub const PCM_TX_LCH_RPT_MASK: c_uint = 0x1;

pub const PCM_VBT_16K_MODE_SFT: c_int = 18;
pub const PCM_VBT_16K_MODE_MASK: c_uint = 0x1;

pub const PCM_EXT_MODEM_SFT: c_int = 17;
pub const PCM_EXT_MODEM_MASK: c_uint = 0x1;

pub const PCM_24BIT_SFT: c_int = 16;
pub const PCM_24BIT_MASK: c_uint = 0x1;

pub const PCM_WLEN_SFT: c_int = 14;
pub const PCM_WLEN_MASK: c_uint = 0x3;

pub const PCM_SYNC_LENGTH_SFT: c_int = 9;
pub const PCM_SYNC_LENGTH_MASK: c_uint = 0x1f;

pub const PCM_SYNC_TYPE_SFT: c_int = 8;
pub const PCM_SYNC_TYPE_MASK: c_uint = 0x1;

pub const PCM_BT_MODE_SFT: c_int = 7;
pub const PCM_BT_MODE_MASK: c_uint = 0x1;

pub const PCM_BYP_ASRC_SFT: c_int = 6;
pub const PCM_BYP_ASRC_MASK: c_uint = 0x1;

pub const PCM_SLAVE_SFT: c_int = 5;
pub const PCM_SLAVE_MASK: c_uint = 0x1;

pub const PCM_MODE_SFT: c_int = 3;
pub const PCM_MODE_MASK: c_uint = 0x3;

pub const PCM_FMT_SFT: c_int = 1;
pub const PCM_FMT_MASK: c_uint = 0x3;

pub const PCM_EN_SFT: c_int = 0;
pub const PCM_EN_MASK: c_uint = 0x1;

// PCM_INTF_CON2
pub const PCM1_TX_FIFO_OV_SFT: c_int = 31;
pub const PCM1_TX_FIFO_OV_MASK: c_uint = 0x1;

pub const PCM1_RX_FIFO_OV_SFT: c_int = 30;
pub const PCM1_RX_FIFO_OV_MASK: c_uint = 0x1;

pub const PCM2_TX_FIFO_OV_SFT: c_int = 29;
pub const PCM2_TX_FIFO_OV_MASK: c_uint = 0x1;

pub const PCM2_RX_FIFO_OV_SFT: c_int = 28;
pub const PCM2_RX_FIFO_OV_MASK: c_uint = 0x1;

pub const PCM1_SYNC_GLITCH_SFT: c_int = 27;
pub const PCM1_SYNC_GLITCH_MASK: c_uint = 0x1;

pub const PCM2_SYNC_GLITCH_SFT: c_int = 26;
pub const PCM2_SYNC_GLITCH_MASK: c_uint = 0x1;

pub const PCM1_PCM2_LOOPBACK_SFT: c_int = 15;
pub const PCM1_PCM2_LOOPBACK_MASK: c_uint = 0x1;

pub const DAI_PCM_LOOPBACK_CH_SFT: c_int = 13;
pub const DAI_PCM_LOOPBACK_CH_MASK: c_uint = 0x1;

pub const I2S_PCM_LOOPBACK_CH_SFT: c_int = 12;
pub const I2S_PCM_LOOPBACK_CH_MASK: c_uint = 0x1;

pub const PCM_USE_MD3_SFT: c_int = 8;
pub const PCM_USE_MD3_MASK: c_uint = 0x1;

pub const TX_FIX_VALUE_SFT: c_int = 0;
pub const TX_FIX_VALUE_MASK: c_uint = 0xff;

// PCM2_INTF_CON
pub const PCM2_TX_FIX_VALUE_SFT: c_int = 24;
pub const PCM2_TX_FIX_VALUE_MASK: c_uint = 0xff;

pub const PCM2_FIX_VALUE_SEL_SFT: c_int = 23;
pub const PCM2_FIX_VALUE_SEL_MASK: c_uint = 0x1;

pub const PCM2_BUFFER_LOOPBACK_SFT: c_int = 22;
pub const PCM2_BUFFER_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM2_PARALLEL_LOOPBACK_SFT: c_int = 21;
pub const PCM2_PARALLEL_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM2_SERIAL_LOOPBACK_SFT: c_int = 20;
pub const PCM2_SERIAL_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM2_DAI_PCM_LOOPBACK_SFT: c_int = 19;
pub const PCM2_DAI_PCM_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM2_I2S_PCM_LOOPBACK_SFT: c_int = 18;
pub const PCM2_I2S_PCM_LOOPBACK_MASK: c_uint = 0x1;

pub const PCM2_SYNC_DELSEL_SFT: c_int = 17;
pub const PCM2_SYNC_DELSEL_MASK: c_uint = 0x1;

pub const PCM2_TX_LR_SWAP_SFT: c_int = 16;
pub const PCM2_TX_LR_SWAP_MASK: c_uint = 0x1;

pub const PCM2_SYNC_IN_INV_SFT: c_int = 15;
pub const PCM2_SYNC_IN_INV_MASK: c_uint = 0x1;

pub const PCM2_BCLK_IN_INV_SFT: c_int = 14;
pub const PCM2_BCLK_IN_INV_MASK: c_uint = 0x1;

pub const PCM2_TX_LCH_RPT_SFT: c_int = 13;
pub const PCM2_TX_LCH_RPT_MASK: c_uint = 0x1;

pub const PCM2_VBT_16K_MODE_SFT: c_int = 12;
pub const PCM2_VBT_16K_MODE_MASK: c_uint = 0x1;

pub const PCM2_LOOPBACK_CH_SEL_SFT: c_int = 10;
pub const PCM2_LOOPBACK_CH_SEL_MASK: c_uint = 0x3;

pub const PCM2_TX2_BT_MODE_SFT: c_int = 8;
pub const PCM2_TX2_BT_MODE_MASK: c_uint = 0x1;

pub const PCM2_BT_MODE_SFT: c_int = 7;
pub const PCM2_BT_MODE_MASK: c_uint = 0x1;

pub const PCM2_AFIFO_SFT: c_int = 6;
pub const PCM2_AFIFO_MASK: c_uint = 0x1;

pub const PCM2_WLEN_SFT: c_int = 5;
pub const PCM2_WLEN_MASK: c_uint = 0x1;

pub const PCM2_MODE_SFT: c_int = 3;
pub const PCM2_MODE_MASK: c_uint = 0x3;

pub const PCM2_FMT_SFT: c_int = 1;
pub const PCM2_FMT_MASK: c_uint = 0x3;

pub const PCM2_EN_SFT: c_int = 0;
pub const PCM2_EN_MASK: c_uint = 0x1;

