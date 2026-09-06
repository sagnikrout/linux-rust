//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8183/mt8183-reg.h
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
// mt8183-reg.h  --  Mediatek 8183 audio driver reg definition
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
pub const AFE_CONN6: c_uint = 0x00bc;
pub const AFE_MEMIF_MSB: c_uint = 0x00cc;
pub const AFE_MEMIF_MON0: c_uint = 0x00d0;
pub const AFE_MEMIF_MON1: c_uint = 0x00d4;
pub const AFE_MEMIF_MON2: c_uint = 0x00d8;
pub const AFE_MEMIF_MON3: c_uint = 0x00dc;
pub const AFE_MEMIF_MON4: c_uint = 0x00e0;
pub const AFE_MEMIF_MON5: c_uint = 0x00e4;
pub const AFE_MEMIF_MON6: c_uint = 0x00e8;
pub const AFE_MEMIF_MON7: c_uint = 0x00ec;
pub const AFE_MEMIF_MON8: c_uint = 0x00f0;
pub const AFE_MEMIF_MON9: c_uint = 0x00f4;
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
pub const AFE_SIDETONE_DEBUG: c_uint = 0x01d0;
pub const AFE_SIDETONE_MON: c_uint = 0x01d4;
pub const AFE_SINEGEN_CON2: c_uint = 0x01dc;
pub const AFE_SIDETONE_CON0: c_uint = 0x01e0;
pub const AFE_SIDETONE_COEFF: c_uint = 0x01e4;
pub const AFE_SIDETONE_CON1: c_uint = 0x01e8;
pub const AFE_SIDETONE_GAIN: c_uint = 0x01ec;
pub const AFE_SINEGEN_CON0: c_uint = 0x01f0;
pub const AFE_TOP_CON0: c_uint = 0x0200;
pub const AFE_BUS_CFG: c_uint = 0x0240;
pub const AFE_BUS_MON0: c_uint = 0x0244;
pub const AFE_ADDA_PREDIS_CON0: c_uint = 0x0260;
pub const AFE_ADDA_PREDIS_CON1: c_uint = 0x0264;
pub const AFE_MRGIF_MON0: c_uint = 0x0270;
pub const AFE_MRGIF_MON1: c_uint = 0x0274;
pub const AFE_MRGIF_MON2: c_uint = 0x0278;
pub const AFE_I2S_MON: c_uint = 0x027c;
pub const AFE_ADDA_IIR_COEF_02_01: c_uint = 0x0290;
pub const AFE_ADDA_IIR_COEF_04_03: c_uint = 0x0294;
pub const AFE_ADDA_IIR_COEF_06_05: c_uint = 0x0298;
pub const AFE_ADDA_IIR_COEF_08_07: c_uint = 0x029c;
pub const AFE_ADDA_IIR_COEF_10_09: c_uint = 0x02a0;
pub const AFE_DAC_CON2: c_uint = 0x02e0;
pub const AFE_IRQ_MCU_CON1: c_uint = 0x02e4;
pub const AFE_IRQ_MCU_CON2: c_uint = 0x02e8;
pub const AFE_DAC_MON: c_uint = 0x02ec;
pub const AFE_VUL2_BASE: c_uint = 0x02f0;
pub const AFE_VUL2_END: c_uint = 0x02f8;
pub const AFE_VUL2_CUR: c_uint = 0x02fc;
pub const AFE_IRQ_MCU_CNT0: c_uint = 0x0300;
pub const AFE_IRQ_MCU_CNT6: c_uint = 0x0304;
pub const AFE_IRQ_MCU_CNT8: c_uint = 0x0308;
pub const AFE_IRQ_MCU_EN1: c_uint = 0x030c;
pub const AFE_IRQ0_MCU_CNT_MON: c_uint = 0x0310;
pub const AFE_IRQ6_MCU_CNT_MON: c_uint = 0x0314;
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
pub const AFE_HDMI_OUT_BASE: c_uint = 0x0374;
pub const AFE_HDMI_OUT_CUR: c_uint = 0x0378;
pub const AFE_HDMI_OUT_END: c_uint = 0x037c;
pub const AFE_HDMI_CONN0: c_uint = 0x0390;
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
pub const AFE_IRQ1_MCU_EN_CNT_MON: c_uint = 0x03c8;
pub const AFE_IRQ5_MCU_CNT_MON: c_uint = 0x03cc;
pub const AFE_MEMIF_MINLEN: c_uint = 0x03d0;
pub const AFE_MEMIF_MAXLEN: c_uint = 0x03d4;
pub const AFE_MEMIF_PBUF_SIZE: c_uint = 0x03d8;
pub const AFE_IRQ_MCU_CNT7: c_uint = 0x03dc;
pub const AFE_IRQ7_MCU_CNT_MON: c_uint = 0x03e0;
pub const AFE_IRQ_MCU_CNT3: c_uint = 0x03e4;
pub const AFE_IRQ_MCU_CNT4: c_uint = 0x03e8;
pub const AFE_IRQ_MCU_CNT11: c_uint = 0x03ec;
pub const AFE_APLL1_TUNER_CFG: c_uint = 0x03f0;
pub const AFE_APLL2_TUNER_CFG: c_uint = 0x03f4;
pub const AFE_MEMIF_HD_MODE: c_uint = 0x03f8;
pub const AFE_MEMIF_HDALIGN: c_uint = 0x03fc;
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
pub const AFE_SRAM_DELSEL_CON0: c_uint = 0x04f0;
pub const AFE_SRAM_DELSEL_CON2: c_uint = 0x04f8;
pub const AFE_SRAM_DELSEL_CON3: c_uint = 0x04fc;
pub const AFE_ASRC_2CH_CON12: c_uint = 0x0528;
pub const AFE_ASRC_2CH_CON13: c_uint = 0x052c;
pub const PCM_INTF_CON1: c_uint = 0x0530;
pub const PCM_INTF_CON2: c_uint = 0x0538;
pub const PCM2_INTF_CON: c_uint = 0x053c;
pub const AFE_TDM_CON1: c_uint = 0x0548;
pub const AFE_TDM_CON2: c_uint = 0x054c;
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
pub const AFE_ADDA6_TOP_CON0: c_uint = 0x0a80;
pub const AFE_ADDA6_UL_SRC_CON0: c_uint = 0x0a84;
pub const AFE_ADD6_UL_SRC_CON1: c_uint = 0x0a88;
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
pub const AFE_DL1_BASE_MSB: c_uint = 0x0b00;
pub const AFE_DL1_CUR_MSB: c_uint = 0x0b04;
pub const AFE_DL1_END_MSB: c_uint = 0x0b08;
pub const AFE_DL2_BASE_MSB: c_uint = 0x0b10;
pub const AFE_DL2_CUR_MSB: c_uint = 0x0b14;
pub const AFE_DL2_END_MSB: c_uint = 0x0b18;
pub const AFE_AWB_BASE_MSB: c_uint = 0x0b20;
pub const AFE_AWB_END_MSB: c_uint = 0x0b28;
pub const AFE_AWB_CUR_MSB: c_uint = 0x0b2c;
pub const AFE_VUL_BASE_MSB: c_uint = 0x0b30;
pub const AFE_VUL_END_MSB: c_uint = 0x0b38;
pub const AFE_VUL_CUR_MSB: c_uint = 0x0b3c;
pub const AFE_VUL2_BASE_MSB: c_uint = 0x0b50;
pub const AFE_VUL2_END_MSB: c_uint = 0x0b58;
pub const AFE_VUL2_CUR_MSB: c_uint = 0x0b5c;
pub const AFE_MOD_DAI_BASE_MSB: c_uint = 0x0b60;
pub const AFE_MOD_DAI_END_MSB: c_uint = 0x0b68;
pub const AFE_MOD_DAI_CUR_MSB: c_uint = 0x0b6c;
pub const AFE_VUL_D2_BASE_MSB: c_uint = 0x0b80;
pub const AFE_VUL_D2_END_MSB: c_uint = 0x0b88;
pub const AFE_VUL_D2_CUR_MSB: c_uint = 0x0b8c;
pub const AFE_DL3_BASE_MSB: c_uint = 0x0b90;
pub const AFE_DL3_CUR_MSB: c_uint = 0x0b94;
pub const AFE_DL3_END_MSB: c_uint = 0x0b98;
pub const AFE_HDMI_OUT_BASE_MSB: c_uint = 0x0ba4;
pub const AFE_HDMI_OUT_CUR_MSB: c_uint = 0x0ba8;
pub const AFE_HDMI_OUT_END_MSB: c_uint = 0x0bac;
pub const AFE_AWB2_BASE: c_uint = 0x0bd0;
pub const AFE_AWB2_END: c_uint = 0x0bd8;
pub const AFE_AWB2_CUR: c_uint = 0x0bdc;
pub const AFE_AWB2_BASE_MSB: c_uint = 0x0be0;
pub const AFE_AWB2_END_MSB: c_uint = 0x0be8;
pub const AFE_AWB2_CUR_MSB: c_uint = 0x0bec;
pub const AFE_ADDA_DL_SDM_DCCOMP_CON: c_uint = 0x0c50;
pub const AFE_ADDA_DL_SDM_TEST: c_uint = 0x0c54;
pub const AFE_ADDA_DL_DC_COMP_CFG0: c_uint = 0x0c58;
pub const AFE_ADDA_DL_DC_COMP_CFG1: c_uint = 0x0c5c;
pub const AFE_ADDA_DL_SDM_FIFO_MON: c_uint = 0x0c60;
pub const AFE_ADDA_DL_SRC_LCH_MON: c_uint = 0x0c64;
pub const AFE_ADDA_DL_SRC_RCH_MON: c_uint = 0x0c68;
pub const AFE_ADDA_DL_SDM_OUT_MON: c_uint = 0x0c6c;
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
pub const AFE_ADDA6_IIR_COEF_02_01: c_uint = 0x0ce0;
pub const AFE_ADDA6_IIR_COEF_04_03: c_uint = 0x0ce4;
pub const AFE_ADDA6_IIR_COEF_06_05: c_uint = 0x0ce8;
pub const AFE_ADDA6_IIR_COEF_08_07: c_uint = 0x0cec;
pub const AFE_ADDA6_IIR_COEF_10_09: c_uint = 0x0cf0;
pub const AFE_ADDA_PREDIS_CON2: c_uint = 0x0d40;
pub const AFE_ADDA_PREDIS_CON3: c_uint = 0x0d44;
pub const AFE_MEMIF_MON12: c_uint = 0x0d70;
pub const AFE_MEMIF_MON13: c_uint = 0x0d74;
pub const AFE_MEMIF_MON14: c_uint = 0x0d78;
pub const AFE_MEMIF_MON15: c_uint = 0x0d7c;
pub const AFE_MEMIF_MON16: c_uint = 0x0d80;
pub const AFE_MEMIF_MON17: c_uint = 0x0d84;
pub const AFE_MEMIF_MON18: c_uint = 0x0d88;
pub const AFE_MEMIF_MON19: c_uint = 0x0d8c;
pub const AFE_MEMIF_MON20: c_uint = 0x0d90;
pub const AFE_MEMIF_MON21: c_uint = 0x0d94;
pub const AFE_MEMIF_MON22: c_uint = 0x0d98;
pub const AFE_MEMIF_MON23: c_uint = 0x0d9c;
pub const AFE_MEMIF_MON24: c_uint = 0x0da0;
pub const AFE_HD_ENGEN_ENABLE: c_uint = 0x0dd0;
pub const AFE_ADDA_MTKAIF_CFG0: c_uint = 0x0e00;
pub const AFE_ADDA_MTKAIF_TX_CFG1: c_uint = 0x0e14;
pub const AFE_ADDA_MTKAIF_RX_CFG0: c_uint = 0x0e20;
pub const AFE_ADDA_MTKAIF_RX_CFG1: c_uint = 0x0e24;
pub const AFE_ADDA_MTKAIF_RX_CFG2: c_uint = 0x0e28;
pub const AFE_ADDA_MTKAIF_MON0: c_uint = 0x0e34;
pub const AFE_ADDA_MTKAIF_MON1: c_uint = 0x0e38;
pub const AFE_AUD_PAD_TOP: c_uint = 0x0e40;
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

pub const AFE_IRQ_STATUS_BITS: c_uint = 0x1fff;
// AUDIO_TOP_CON3
pub const BCK_INVERSE_SFT: c_int = 3;
pub const BCK_INVERSE_MASK: c_uint = 0x1;

// AFE_DAC_CON0
pub const AWB2_ON_SFT: c_int = 29;
pub const AWB2_ON_MASK: c_uint = 0x1;

pub const VUL2_ON_SFT: c_int = 27;
pub const VUL2_ON_MASK: c_uint = 0x1;

pub const MOD_DAI_DUP_WR_SFT: c_int = 26;
pub const MOD_DAI_DUP_WR_MASK: c_uint = 0x1;

pub const VUL12_MODE_SFT: c_int = 20;
pub const VUL12_MODE_MASK: c_uint = 0xf;

pub const VUL12_R_MONO_SFT: c_int = 11;
pub const VUL12_R_MONO_MASK: c_uint = 0x1;

pub const VUL12_MONO_SFT: c_int = 10;
pub const VUL12_MONO_MASK: c_uint = 0x1;

pub const VUL12_ON_SFT: c_int = 9;
pub const VUL12_ON_MASK: c_uint = 0x1;

pub const MOD_DAI_ON_SFT: c_int = 7;
pub const MOD_DAI_ON_MASK: c_uint = 0x1;

pub const AWB_ON_SFT: c_int = 6;
pub const AWB_ON_MASK: c_uint = 0x1;

pub const DL3_ON_SFT: c_int = 5;
pub const DL3_ON_MASK: c_uint = 0x1;

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

pub const VUL_R_MONO_SFT: c_int = 28;
pub const VUL_R_MONO_MASK: c_uint = 0x1;

pub const VUL_DATA_SFT: c_int = 27;
pub const VUL_DATA_MASK: c_uint = 0x1;

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

// AFE_DAC_CON2
pub const AWB2_R_MONO_SFT: c_int = 21;
pub const AWB2_R_MONO_MASK: c_uint = 0x1;

pub const AWB2_DATA_SFT: c_int = 20;
pub const AWB2_DATA_MASK: c_uint = 0x1;

pub const AWB2_MODE_SFT: c_int = 16;
pub const AWB2_MODE_MASK: c_uint = 0xf;

pub const DL3_MODE_SFT: c_int = 8;
pub const DL3_MODE_MASK: c_uint = 0xf;

pub const VUL2_MODE_SFT: c_int = 4;
pub const VUL2_MODE_MASK: c_uint = 0xf;

pub const VUL2_R_MONO_SFT: c_int = 1;
pub const VUL2_R_MONO_MASK: c_uint = 0x1;

pub const VUL2_DATA_SFT: c_int = 0;
pub const VUL2_DATA_MASK: c_uint = 0x1;

// AFE_DAC_MON
pub const AFE_ON_RETM_SFT: c_int = 0;
pub const AFE_ON_RETM_MASK: c_uint = 0x1;

// AFE_I2S_CON
pub const BCK_NEG_EG_LATCH_SFT: c_int = 30;
pub const BCK_NEG_EG_LATCH_MASK: c_uint = 0x1;

pub const BCK_INV_SFT: c_int = 29;
pub const BCK_INV_MASK: c_uint = 0x1;

pub const I2SIN_PAD_SEL_SFT: c_int = 28;
pub const I2SIN_PAD_SEL_MASK: c_uint = 0x1;

pub const I2S_LOOPBACK_SFT: c_int = 20;
pub const I2S_LOOPBACK_MASK: c_uint = 0x1;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;
pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_MASK: c_uint = 0x1;

pub const I2S1_HD_EN_SFT: c_int = 12;
pub const I2S1_HD_EN_MASK: c_uint = 0x1;

pub const INV_PAD_CTRL_SFT: c_int = 7;
pub const INV_PAD_CTRL_MASK: c_uint = 0x1;

pub const I2S_BYPSRC_SFT: c_int = 6;
pub const I2S_BYPSRC_MASK: c_uint = 0x1;

pub const INV_LRCK_SFT: c_int = 5;
pub const INV_LRCK_MASK: c_uint = 0x1;

pub const I2S_FMT_SFT: c_int = 3;
pub const I2S_FMT_MASK: c_uint = 0x1;

pub const I2S_SRC_SFT: c_int = 2;
pub const I2S_SRC_MASK: c_uint = 0x1;

pub const I2S_WLEN_SFT: c_int = 1;
pub const I2S_WLEN_MASK: c_uint = 0x1;

pub const I2S_EN_SFT: c_int = 0;
pub const I2S_EN_MASK: c_uint = 0x1;

// AFE_I2S_CON1
pub const I2S2_LR_SWAP_SFT: c_int = 31;
pub const I2S2_LR_SWAP_MASK: c_uint = 0x1;

pub const I2S2_SEL_O19_O20_SFT: c_int = 18;
pub const I2S2_SEL_O19_O20_MASK: c_uint = 0x1;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;
pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_MASK: c_uint = 0x1;

pub const I2S2_SEL_O03_O04_SFT: c_int = 16;
pub const I2S2_SEL_O03_O04_MASK: c_uint = 0x1;

pub const I2S2_32BIT_EN_SFT: c_int = 13;
pub const I2S2_32BIT_EN_MASK: c_uint = 0x1;

pub const I2S2_HD_EN_SFT: c_int = 12;
pub const I2S2_HD_EN_MASK: c_uint = 0x1;

pub const I2S2_OUT_MODE_SFT: c_int = 8;
pub const I2S2_OUT_MODE_MASK: c_uint = 0xf;

pub const INV_LRCK_SFT: c_int = 5;
pub const INV_LRCK_MASK: c_uint = 0x1;

pub const I2S2_FMT_SFT: c_int = 3;
pub const I2S2_FMT_MASK: c_uint = 0x1;

pub const I2S2_WLEN_SFT: c_int = 1;
pub const I2S2_WLEN_MASK: c_uint = 0x1;

pub const I2S2_EN_SFT: c_int = 0;
pub const I2S2_EN_MASK: c_uint = 0x1;

// AFE_I2S_CON2
pub const I2S3_LR_SWAP_SFT: c_int = 31;
pub const I2S3_LR_SWAP_MASK: c_uint = 0x1;

pub const I2S3_UPDATE_WORD_SFT: c_int = 24;
pub const I2S3_UPDATE_WORD_MASK: c_uint = 0x1f;

pub const I2S3_BCK_INV_SFT: c_int = 23;
pub const I2S3_BCK_INV_MASK: c_uint = 0x1;

pub const I2S3_FPGA_BIT_TEST_SFT: c_int = 22;
pub const I2S3_FPGA_BIT_TEST_MASK: c_uint = 0x1;

pub const I2S3_FPGA_BIT_SFT: c_int = 21;
pub const I2S3_FPGA_BIT_MASK: c_uint = 0x1;

pub const I2S3_LOOPBACK_SFT: c_int = 20;
pub const I2S3_LOOPBACK_MASK: c_uint = 0x1;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;
pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_MASK: c_uint = 0x1;

pub const I2S3_HD_EN_SFT: c_int = 12;
pub const I2S3_HD_EN_MASK: c_uint = 0x1;

pub const I2S3_OUT_MODE_SFT: c_int = 8;
pub const I2S3_OUT_MODE_MASK: c_uint = 0xf;

pub const I2S3_FMT_SFT: c_int = 3;
pub const I2S3_FMT_MASK: c_uint = 0x1;

pub const I2S3_WLEN_SFT: c_int = 1;
pub const I2S3_WLEN_MASK: c_uint = 0x1;

pub const I2S3_EN_SFT: c_int = 0;
pub const I2S3_EN_MASK: c_uint = 0x1;

// AFE_I2S_CON3
pub const I2S4_LR_SWAP_SFT: c_int = 31;
pub const I2S4_LR_SWAP_MASK: c_uint = 0x1;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;
pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_MASK: c_uint = 0x1;

pub const I2S4_32BIT_EN_SFT: c_int = 13;
pub const I2S4_32BIT_EN_MASK: c_uint = 0x1;

pub const I2S4_HD_EN_SFT: c_int = 12;
pub const I2S4_HD_EN_MASK: c_uint = 0x1;

pub const I2S4_OUT_MODE_SFT: c_int = 8;
pub const I2S4_OUT_MODE_MASK: c_uint = 0xf;

pub const INV_LRCK_SFT: c_int = 5;
pub const INV_LRCK_MASK: c_uint = 0x1;

pub const I2S4_FMT_SFT: c_int = 3;
pub const I2S4_FMT_MASK: c_uint = 0x1;

pub const I2S4_WLEN_SFT: c_int = 1;
pub const I2S4_WLEN_MASK: c_uint = 0x1;

pub const I2S4_EN_SFT: c_int = 0;
pub const I2S4_EN_MASK: c_uint = 0x1;

// AFE_I2S_CON4
pub const I2S5_LR_SWAP_SFT: c_int = 31;
pub const I2S5_LR_SWAP_MASK: c_uint = 0x1;

pub const I2S_LOOPBACK_SFT: c_int = 20;
pub const I2S_LOOPBACK_MASK: c_uint = 0x1;

pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_SFT: c_int = 17;
pub const I2S_ONOFF_NOT_RESET_CK_ENABLE_MASK: c_uint = 0x1;

pub const I2S5_32BIT_EN_SFT: c_int = 13;
pub const I2S5_32BIT_EN_MASK: c_uint = 0x1;

pub const I2S5_HD_EN_SFT: c_int = 12;
pub const I2S5_HD_EN_MASK: c_uint = 0x1;

pub const I2S5_OUT_MODE_SFT: c_int = 8;
pub const I2S5_OUT_MODE_MASK: c_uint = 0xf;

pub const INV_LRCK_SFT: c_int = 5;
pub const INV_LRCK_MASK: c_uint = 0x1;

pub const I2S5_FMT_SFT: c_int = 3;
pub const I2S5_FMT_MASK: c_uint = 0x1;

pub const I2S5_WLEN_SFT: c_int = 1;
pub const I2S5_WLEN_MASK: c_uint = 0x1;

pub const I2S5_EN_SFT: c_int = 0;
pub const I2S5_EN_MASK: c_uint = 0x1;

// AFE_GAIN1_CON0
pub const GAIN1_SAMPLE_PER_STEP_SFT: c_int = 8;
pub const GAIN1_SAMPLE_PER_STEP_MASK: c_uint = 0xff;

pub const GAIN1_MODE_SFT: c_int = 4;
pub const GAIN1_MODE_MASK: c_uint = 0xf;

pub const GAIN1_ON_SFT: c_int = 0;
pub const GAIN1_ON_MASK: c_uint = 0x1;

// AFE_GAIN1_CON1
pub const GAIN1_TARGET_SFT: c_int = 0;
pub const GAIN1_TARGET_MASK: c_uint = 0xfffff;

// AFE_GAIN2_CON0
pub const GAIN2_SAMPLE_PER_STEP_SFT: c_int = 8;
pub const GAIN2_SAMPLE_PER_STEP_MASK: c_uint = 0xff;

pub const GAIN2_MODE_SFT: c_int = 4;
pub const GAIN2_MODE_MASK: c_uint = 0xf;

pub const GAIN2_ON_SFT: c_int = 0;
pub const GAIN2_ON_MASK: c_uint = 0x1;

// AFE_GAIN2_CON1
pub const GAIN2_TARGET_SFT: c_int = 0;
pub const GAIN2_TARGET_MASK: c_uint = 0xfffff;

// AFE_GAIN1_CUR
pub const AFE_GAIN1_CUR_SFT: c_int = 0;
pub const AFE_GAIN1_CUR_MASK: c_uint = 0xfffff;

// AFE_GAIN2_CUR
pub const AFE_GAIN2_CUR_SFT: c_int = 0;
pub const AFE_GAIN2_CUR_MASK: c_uint = 0xfffff;

// AFE_MEMIF_HD_MODE
pub const AWB2_HD_SFT: c_int = 28;
pub const AWB2_HD_MASK: c_uint = 0x3;

pub const HDMI_HD_SFT: c_int = 20;
pub const HDMI_HD_MASK: c_uint = 0x3;

pub const MOD_DAI_HD_SFT: c_int = 18;
pub const MOD_DAI_HD_MASK: c_uint = 0x3;

pub const DAI_HD_SFT: c_int = 16;
pub const DAI_HD_MASK: c_uint = 0x3;

pub const VUL2_HD_SFT: c_int = 14;
pub const VUL2_HD_MASK: c_uint = 0x3;

pub const VUL12_HD_SFT: c_int = 12;
pub const VUL12_HD_MASK: c_uint = 0x3;

pub const VUL_HD_SFT: c_int = 10;
pub const VUL_HD_MASK: c_uint = 0x3;

pub const AWB_HD_SFT: c_int = 8;
pub const AWB_HD_MASK: c_uint = 0x3;

pub const DL3_HD_SFT: c_int = 6;
pub const DL3_HD_MASK: c_uint = 0x3;

pub const DL2_HD_SFT: c_int = 4;
pub const DL2_HD_MASK: c_uint = 0x3;

pub const DL1_HD_SFT: c_int = 0;
pub const DL1_HD_MASK: c_uint = 0x3;

// AFE_MEMIF_HDALIGN
pub const AWB2_NORMAL_MODE_SFT: c_int = 30;
pub const AWB2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const HDMI_NORMAL_MODE_SFT: c_int = 26;
pub const HDMI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const MOD_DAI_NORMAL_MODE_SFT: c_int = 25;
pub const MOD_DAI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DAI_NORMAL_MODE_SFT: c_int = 24;
pub const DAI_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL2_NORMAL_MODE_SFT: c_int = 23;
pub const VUL2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL12_NORMAL_MODE_SFT: c_int = 22;
pub const VUL12_NORMAL_MODE_MASK: c_uint = 0x1;

pub const VUL_NORMAL_MODE_SFT: c_int = 21;
pub const VUL_NORMAL_MODE_MASK: c_uint = 0x1;

pub const AWB_NORMAL_MODE_SFT: c_int = 20;
pub const AWB_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL3_NORMAL_MODE_SFT: c_int = 19;
pub const DL3_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL2_NORMAL_MODE_SFT: c_int = 18;
pub const DL2_NORMAL_MODE_MASK: c_uint = 0x1;

pub const DL1_NORMAL_MODE_SFT: c_int = 16;
pub const DL1_NORMAL_MODE_MASK: c_uint = 0x1;

pub const RESERVED1_SFT: c_int = 15;
pub const RESERVED1_MASK: c_uint = 0x1;

pub const AWB2_ALIGN_SFT: c_int = 14;
pub const AWB2_ALIGN_MASK: c_uint = 0x1;

pub const HDMI_HD_ALIGN_SFT: c_int = 10;
pub const HDMI_HD_ALIGN_MASK: c_uint = 0x1;

pub const MOD_DAI_HD_ALIGN_SFT: c_int = 9;
pub const MOD_DAI_HD_ALIGN_MASK: c_uint = 0x1;

pub const VUL2_HD_ALIGN_SFT: c_int = 7;
pub const VUL2_HD_ALIGN_MASK: c_uint = 0x1;

pub const VUL12_HD_ALIGN_SFT: c_int = 6;
pub const VUL12_HD_ALIGN_MASK: c_uint = 0x1;

pub const VUL_HD_ALIGN_SFT: c_int = 5;
pub const VUL_HD_ALIGN_MASK: c_uint = 0x1;

pub const AWB_HD_ALIGN_SFT: c_int = 4;
pub const AWB_HD_ALIGN_MASK: c_uint = 0x1;

pub const DL3_HD_ALIGN_SFT: c_int = 3;
pub const DL3_HD_ALIGN_MASK: c_uint = 0x1;

pub const DL2_HD_ALIGN_SFT: c_int = 2;
pub const DL2_HD_ALIGN_MASK: c_uint = 0x1;

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

pub const TX3_RCH_DBG_MODE_SFT: c_int = 17;
pub const TX3_RCH_DBG_MODE_MASK: c_uint = 0x1;

pub const PCM1_PCM2_LOOPBACK_SFT: c_int = 16;
pub const PCM1_PCM2_LOOPBACK_MASK: c_uint = 0x1;

pub const DAI_PCM_LOOPBACK_CH_SFT: c_int = 14;
pub const DAI_PCM_LOOPBACK_CH_MASK: c_uint = 0x3;

pub const I2S_PCM_LOOPBACK_CH_SFT: c_int = 12;
pub const I2S_PCM_LOOPBACK_CH_MASK: c_uint = 0x3;

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

// AFE_ADDA_MTKAIF_CFG0
pub const MTKAIF_RXIF_CLKINV_ADC_SFT: c_int = 31;
pub const MTKAIF_RXIF_CLKINV_ADC_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_BYPASS_SRC_SFT: c_int = 17;
pub const MTKAIF_RXIF_BYPASS_SRC_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_PROTOCOL2_SFT: c_int = 16;
pub const MTKAIF_RXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const MTKAIF_TXIF_BYPASS_SRC_SFT: c_int = 5;
pub const MTKAIF_TXIF_BYPASS_SRC_MASK: c_uint = 0x1;

pub const MTKAIF_TXIF_PROTOCOL2_SFT: c_int = 4;
pub const MTKAIF_TXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const MTKAIF_TXIF_8TO5_SFT: c_int = 2;
pub const MTKAIF_TXIF_8TO5_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_8TO5_SFT: c_int = 1;
pub const MTKAIF_RXIF_8TO5_MASK: c_uint = 0x1;

pub const MTKAIF_IF_LOOPBACK1_SFT: c_int = 0;
pub const MTKAIF_IF_LOOPBACK1_MASK: c_uint = 0x1;

// AFE_ADDA_MTKAIF_RX_CFG2
pub const MTKAIF_RXIF_DETECT_ON_PROTOCOL2_SFT: c_int = 16;
pub const MTKAIF_RXIF_DETECT_ON_PROTOCOL2_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_DELAY_CYCLE_SFT: c_int = 12;
pub const MTKAIF_RXIF_DELAY_CYCLE_MASK: c_uint = 0xf;

pub const MTKAIF_RXIF_DELAY_DATA_SFT: c_int = 8;
pub const MTKAIF_RXIF_DELAY_DATA_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_SFT: c_int = 4;
pub const MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_MASK: c_uint = 0x7;

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
pub const ULCF_CFG_EN_CTL_SFT: c_int = 31;
pub const ULCF_CFG_EN_CTL_MASK: c_uint = 0x1;

pub const UL_MODE_3P25M_CH2_CTL_SFT: c_int = 22;
pub const UL_MODE_3P25M_CH2_CTL_MASK: c_uint = 0x1;

pub const UL_MODE_3P25M_CH1_CTL_SFT: c_int = 21;
pub const UL_MODE_3P25M_CH1_CTL_MASK: c_uint = 0x1;

pub const UL_VOICE_MODE_CH1_CH2_CTL_SFT: c_int = 17;
pub const UL_VOICE_MODE_CH1_CH2_CTL_MASK: c_uint = 0x7;

pub const DMIC_LOW_POWER_MODE_CTL_SFT: c_int = 14;
pub const DMIC_LOW_POWER_MODE_CTL_MASK: c_uint = 0x3;

pub const UL_DISABLE_HW_CG_CTL_SFT: c_int = 12;
pub const UL_DISABLE_HW_CG_CTL_MASK: c_uint = 0x1;

pub const UL_IIR_ON_TMP_CTL_SFT: c_int = 10;
pub const UL_IIR_ON_TMP_CTL_MASK: c_uint = 0x1;

pub const UL_IIRMODE_CTL_SFT: c_int = 7;
pub const UL_IIRMODE_CTL_MASK: c_uint = 0x7;

pub const DIGMIC_3P25M_1P625M_SEL_CTL_SFT: c_int = 5;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK: c_uint = 0x1;

pub const UL_LOOP_BACK_MODE_CTL_SFT: c_int = 2;
pub const UL_LOOP_BACK_MODE_CTL_MASK: c_uint = 0x1;

pub const UL_SDM_3_LEVEL_CTL_SFT: c_int = 1;
pub const UL_SDM_3_LEVEL_CTL_MASK: c_uint = 0x1;

pub const UL_SRC_ON_TMP_CTL_SFT: c_int = 0;
pub const UL_SRC_ON_TMP_CTL_MASK: c_uint = 0x1;

// AFE_ADDA_UL_SRC_CON1
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
pub const AFE_ADDA6_UL_LR_SWAP_SFT: c_int = 15;
pub const AFE_ADDA6_UL_LR_SWAP_MASK: c_uint = 0x1;

pub const AFE_ADDA6_CKDIV_RST_SFT: c_int = 14;
pub const AFE_ADDA6_CKDIV_RST_MASK: c_uint = 0x1;

pub const AFE_ADDA6_FIFO_AUTO_RST_SFT: c_int = 13;
pub const AFE_ADDA6_FIFO_AUTO_RST_MASK: c_uint = 0x1;

pub const UL_FIFO_DIGMIC_TESTIN_SFT: c_int = 5;
pub const UL_FIFO_DIGMIC_TESTIN_MASK: c_uint = 0x3;

pub const UL_FIFO_DIGMIC_WDATA_TESTEN_SFT: c_int = 4;
pub const UL_FIFO_DIGMIC_WDATA_TESTEN_MASK: c_uint = 0x1;

pub const ADDA_AFE_ON_SFT: c_int = 0;
pub const ADDA_AFE_ON_MASK: c_uint = 0x1;

// AFE_SIDETONE_CON0
pub const R_RDY_SFT: c_int = 30;
pub const R_RDY_MASK: c_uint = 0x1;

pub const W_RDY_SFT: c_int = 29;
pub const W_RDY_MASK: c_uint = 0x1;

pub const R_W_EN_SFT: c_int = 25;
pub const R_W_EN_MASK: c_uint = 0x1;

pub const R_W_SEL_SFT: c_int = 24;
pub const R_W_SEL_MASK: c_uint = 0x1;

pub const SEL_CH2_SFT: c_int = 23;
pub const SEL_CH2_MASK: c_uint = 0x1;

pub const SIDE_TONE_COEFFICIENT_ADDR_SFT: c_int = 16;
pub const SIDE_TONE_COEFFICIENT_ADDR_MASK: c_uint = 0x1f;

pub const SIDE_TONE_COEFFICIENT_SFT: c_int = 0;
pub const SIDE_TONE_COEFFICIENT_MASK: c_uint = 0xffff;

// AFE_SIDETONE_COEFF
pub const SIDE_TONE_COEFF_SFT: c_int = 0;
pub const SIDE_TONE_COEFF_MASK: c_uint = 0xffff;

// AFE_SIDETONE_CON1
pub const STF_BYPASS_MODE_SFT: c_int = 31;
pub const STF_BYPASS_MODE_MASK: c_uint = 0x1;

pub const STF_BYPASS_MODE_O28_O29_SFT: c_int = 30;
pub const STF_BYPASS_MODE_O28_O29_MASK: c_uint = 0x1;

pub const STF_BYPASS_MODE_I2S4_SFT: c_int = 29;
pub const STF_BYPASS_MODE_I2S4_MASK: c_uint = 0x1;

pub const STF_BYPASS_MODE_I2S5_SFT: c_int = 28;
pub const STF_BYPASS_MODE_I2S5_MASK: c_uint = 0x1;

pub const STF_INPUT_EN_SEL_SFT: c_int = 13;
pub const STF_INPUT_EN_SEL_MASK: c_uint = 0x1;

pub const STF_SOURCE_FROM_O19O20_SFT: c_int = 12;
pub const STF_SOURCE_FROM_O19O20_MASK: c_uint = 0x1;

pub const SIDE_TONE_ON_SFT: c_int = 8;
pub const SIDE_TONE_ON_MASK: c_uint = 0x1;

pub const SIDE_TONE_HALF_TAP_NUM_SFT: c_int = 0;
pub const SIDE_TONE_HALF_TAP_NUM_MASK: c_uint = 0x3f;

// AFE_SIDETONE_GAIN
pub const POSITIVE_GAIN_SFT: c_int = 16;
pub const POSITIVE_GAIN_MASK: c_uint = 0x7;

pub const SIDE_TONE_GAIN_SFT: c_int = 0;
pub const SIDE_TONE_GAIN_MASK: c_uint = 0xffff;

// AFE_ADDA_DL_SDM_DCCOMP_CON
pub const AUD_DC_COMP_EN_SFT: c_int = 8;
pub const AUD_DC_COMP_EN_MASK: c_uint = 0x1;

pub const ATTGAIN_CTL_SFT: c_int = 0;
pub const ATTGAIN_CTL_MASK: c_uint = 0x3f;

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
pub const INNER_LOOP_BACK_MODE_MASK: c_uint = 0x3f;

// AFE_MEMIF_MINLEN
pub const HDMI_MINLEN_SFT: c_int = 24;
pub const HDMI_MINLEN_MASK: c_uint = 0xf;

pub const DL3_MINLEN_SFT: c_int = 12;
pub const DL3_MINLEN_MASK: c_uint = 0xf;

pub const DL2_MINLEN_SFT: c_int = 8;
pub const DL2_MINLEN_MASK: c_uint = 0xf;

pub const DL1_DATA2_MINLEN_SFT: c_int = 4;
pub const DL1_DATA2_MINLEN_MASK: c_uint = 0xf;

pub const DL1_MINLEN_SFT: c_int = 0;
pub const DL1_MINLEN_MASK: c_uint = 0xf;

// AFE_MEMIF_MAXLEN
pub const HDMI_MAXLEN_SFT: c_int = 24;
pub const HDMI_MAXLEN_MASK: c_uint = 0xf;

pub const DL3_MAXLEN_SFT: c_int = 8;
pub const DL3_MAXLEN_MASK: c_uint = 0xf;

pub const DL2_MAXLEN_SFT: c_int = 4;
pub const DL2_MAXLEN_MASK: c_uint = 0xf;

pub const DL1_MAXLEN_SFT: c_int = 0;
pub const DL1_MAXLEN_MASK: c_uint = 0x3;

// AFE_MEMIF_PBUF_SIZE
pub const VUL12_4CH_SFT: c_int = 17;
pub const VUL12_4CH_MASK: c_uint = 0x1;

pub const DL3_PBUF_SIZE_SFT: c_int = 10;
pub const DL3_PBUF_SIZE_MASK: c_uint = 0x3;

pub const HDMI_PBUF_SIZE_SFT: c_int = 4;
pub const HDMI_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL2_PBUF_SIZE_SFT: c_int = 2;
pub const DL2_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL1_PBUF_SIZE_SFT: c_int = 0;
pub const DL1_PBUF_SIZE_MASK: c_uint = 0x3;

// AFE_HD_ENGEN_ENABLE
pub const AFE_24M_ON_SFT: c_int = 1;
pub const AFE_24M_ON_MASK: c_uint = 0x1;

pub const AFE_22M_ON_SFT: c_int = 0;
pub const AFE_22M_ON_MASK: c_uint = 0x1;

// AFE_IRQ_MCU_CON0
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
pub const IRQ12_MCU_MODE_SFT: c_int = 4;
pub const IRQ12_MCU_MODE_MASK: c_uint = 0xf;

pub const IRQ11_MCU_MODE_SFT: c_int = 0;
pub const IRQ11_MCU_MODE_MASK: c_uint = 0xf;

// AFE_IRQ_MCU_CLR
pub const IRQ12_MCU_MISS_CNT_CLR_SFT: c_int = 28;
pub const IRQ12_MCU_MISS_CNT_CLR_MASK: c_uint = 0x1;

pub const IRQ11_MCU_MISS_CNT_CLR_SFT: c_int = 27;
pub const IRQ11_MCU_MISS_CNT_CLR_MASK: c_uint = 0x1;

pub const IRQ10_MCU_MISS_CLR_SFT: c_int = 26;
pub const IRQ10_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ9_MCU_MISS_CLR_SFT: c_int = 25;
pub const IRQ9_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ8_MCU_MISS_CLR_SFT: c_int = 24;
pub const IRQ8_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ7_MCU_MISS_CLR_SFT: c_int = 23;
pub const IRQ7_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ6_MCU_MISS_CLR_SFT: c_int = 22;
pub const IRQ6_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ5_MCU_MISS_CLR_SFT: c_int = 21;
pub const IRQ5_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ4_MCU_MISS_CLR_SFT: c_int = 20;
pub const IRQ4_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ3_MCU_MISS_CLR_SFT: c_int = 19;
pub const IRQ3_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ2_MCU_MISS_CLR_SFT: c_int = 18;
pub const IRQ2_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ1_MCU_MISS_CLR_SFT: c_int = 17;
pub const IRQ1_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ0_MCU_MISS_CLR_SFT: c_int = 16;
pub const IRQ0_MCU_MISS_CLR_MASK: c_uint = 0x1;

pub const IRQ12_MCU_CLR_SFT: c_int = 12;
pub const IRQ12_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ11_MCU_CLR_SFT: c_int = 11;
pub const IRQ11_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ10_MCU_CLR_SFT: c_int = 10;
pub const IRQ10_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ9_MCU_CLR_SFT: c_int = 9;
pub const IRQ9_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ8_MCU_CLR_SFT: c_int = 8;
pub const IRQ8_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ7_MCU_CLR_SFT: c_int = 7;
pub const IRQ7_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ6_MCU_CLR_SFT: c_int = 6;
pub const IRQ6_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ5_MCU_CLR_SFT: c_int = 5;
pub const IRQ5_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ4_MCU_CLR_SFT: c_int = 4;
pub const IRQ4_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ3_MCU_CLR_SFT: c_int = 3;
pub const IRQ3_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ2_MCU_CLR_SFT: c_int = 2;
pub const IRQ2_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ1_MCU_CLR_SFT: c_int = 1;
pub const IRQ1_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ0_MCU_CLR_SFT: c_int = 0;
pub const IRQ0_MCU_CLR_MASK: c_uint = 0x1;

// AFE_MEMIF_MSB
pub const CPU_COMPACT_MODE_SFT: c_int = 29;
pub const CPU_COMPACT_MODE_MASK: c_uint = 0x1;

pub const CPU_HD_ALIGN_SFT: c_int = 28;
pub const CPU_HD_ALIGN_MASK: c_uint = 0x1;

pub const AWB2_AXI_WR_SIGN_SFT: c_int = 24;
pub const AWB2_AXI_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL2_AXI_WR_SIGN_SFT: c_int = 22;
pub const VUL2_AXI_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL12_AXI_WR_SIGN_SFT: c_int = 21;
pub const VUL12_AXI_WR_SIGN_MASK: c_uint = 0x1;

pub const VUL_AXI_WR_SIGN_SFT: c_int = 20;
pub const VUL_AXI_WR_SIGN_MASK: c_uint = 0x1;

pub const MOD_DAI_AXI_WR_SIGN_SFT: c_int = 18;
pub const MOD_DAI_AXI_WR_SIGN_MASK: c_uint = 0x1;

pub const AWB_MSTR_SIGN_SFT: c_int = 17;
pub const AWB_MSTR_SIGN_MASK: c_uint = 0x1;

pub const SYSRAM_SIGN_SFT: c_int = 16;
pub const SYSRAM_SIGN_MASK: c_uint = 0x1;

// AFE_HDMI_CONN0
pub const HDMI_O_7_SFT: c_int = 21;
pub const HDMI_O_7_MASK: c_uint = 0x7;

pub const HDMI_O_6_SFT: c_int = 18;
pub const HDMI_O_6_MASK: c_uint = 0x7;

pub const HDMI_O_5_SFT: c_int = 15;
pub const HDMI_O_5_MASK: c_uint = 0x7;

pub const HDMI_O_4_SFT: c_int = 12;
pub const HDMI_O_4_MASK: c_uint = 0x7;

pub const HDMI_O_3_SFT: c_int = 9;
pub const HDMI_O_3_MASK: c_uint = 0x7;

pub const HDMI_O_2_SFT: c_int = 6;
pub const HDMI_O_2_MASK: c_uint = 0x7;

pub const HDMI_O_1_SFT: c_int = 3;
pub const HDMI_O_1_MASK: c_uint = 0x7;

pub const HDMI_O_0_SFT: c_int = 0;
pub const HDMI_O_0_MASK: c_uint = 0x7;

// AFE_TDM_CON1
pub const TDM_EN_SFT: c_int = 0;
pub const TDM_EN_MASK: c_uint = 0x1;

pub const LRCK_INVERSE_SFT: c_int = 2;
pub const LRCK_INVERSE_MASK: c_uint = 0x1;

pub const DELAY_DATA_SFT: c_int = 3;
pub const DELAY_DATA_MASK: c_uint = 0x1;

pub const LEFT_ALIGN_SFT: c_int = 4;
pub const LEFT_ALIGN_MASK: c_uint = 0x1;

pub const WLEN_SFT: c_int = 8;
pub const WLEN_MASK: c_uint = 0x3;

pub const CHANNEL_NUM_SFT: c_int = 10;
pub const CHANNEL_NUM_MASK: c_uint = 0x3;

pub const CHANNEL_BCK_CYCLES_SFT: c_int = 12;
pub const CHANNEL_BCK_CYCLES_MASK: c_uint = 0x3;

pub const DAC_BIT_NUM_SFT: c_int = 16;
pub const DAC_BIT_NUM_MASK: c_uint = 0x1f;

pub const LRCK_TDM_WIDTH_SFT: c_int = 24;
pub const LRCK_TDM_WIDTH_MASK: c_uint = 0xff;

// AFE_TDM_CON2
pub const ST_CH_PAIR_SOUT0_SFT: c_int = 0;
pub const ST_CH_PAIR_SOUT0_MASK: c_uint = 0x7;

pub const ST_CH_PAIR_SOUT1_SFT: c_int = 4;
pub const ST_CH_PAIR_SOUT1_MASK: c_uint = 0x7;

pub const ST_CH_PAIR_SOUT2_SFT: c_int = 8;
pub const ST_CH_PAIR_SOUT2_MASK: c_uint = 0x7;

pub const ST_CH_PAIR_SOUT3_SFT: c_int = 12;
pub const ST_CH_PAIR_SOUT3_MASK: c_uint = 0x7;

pub const TDM_FIX_VALUE_SEL_SFT: c_int = 16;
pub const TDM_FIX_VALUE_SEL_MASK: c_uint = 0x1;

pub const TDM_I2S_LOOPBACK_SFT: c_int = 20;
pub const TDM_I2S_LOOPBACK_MASK: c_uint = 0x1;

pub const TDM_I2S_LOOPBACK_CH_SFT: c_int = 21;
pub const TDM_I2S_LOOPBACK_CH_MASK: c_uint = 0x3;

pub const TDM_FIX_VALUE_SFT: c_int = 24;
pub const TDM_FIX_VALUE_MASK: c_uint = 0xff;

// AFE_HDMI_OUT_CON0
pub const AFE_HDMI_OUT_ON_RETM_SFT: c_int = 8;
pub const AFE_HDMI_OUT_ON_RETM_MASK: c_uint = 0x1;

pub const AFE_HDMI_OUT_CH_NUM_SFT: c_int = 4;
pub const AFE_HDMI_OUT_CH_NUM_MASK: c_uint = 0xf;

pub const AFE_HDMI_OUT_BIT_WIDTH_SFT: c_int = 1;
pub const AFE_HDMI_OUT_BIT_WIDTH_MASK: c_uint = 0x1;

pub const AFE_HDMI_OUT_ON_SFT: c_int = 0;
pub const AFE_HDMI_OUT_ON_MASK: c_uint = 0x1;

