//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/mt6358.h
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
// mt6358.h  --  mt6358 ALSA SoC audio codec driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// Reg bit define
// MT6358_DCXO_CW14
pub const RG_XO_AUDIO_EN_M_SFT: c_int = 13;
// MT6358_DCXO_CW13
pub const RG_XO_VOW_EN_SFT: c_int = 8;
// MT6358_AUD_TOP_CKPDN_CON0
pub const RG_VOW13M_CK_PDN_SFT: c_int = 13;
pub const RG_VOW13M_CK_PDN_MASK: c_uint = 0x1;

pub const RG_VOW32K_CK_PDN_SFT: c_int = 12;
pub const RG_VOW32K_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUD_INTRP_CK_PDN_SFT: c_int = 8;
pub const RG_AUD_INTRP_CK_PDN_MASK: c_uint = 0x1;

pub const RG_PAD_AUD_CLK_MISO_CK_PDN_SFT: c_int = 7;
pub const RG_PAD_AUD_CLK_MISO_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUDNCP_CK_PDN_SFT: c_int = 6;
pub const RG_AUDNCP_CK_PDN_MASK: c_uint = 0x1;

pub const RG_ZCD13M_CK_PDN_SFT: c_int = 5;
pub const RG_ZCD13M_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUDIF_CK_PDN_SFT: c_int = 2;
pub const RG_AUDIF_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUD_CK_PDN_SFT: c_int = 1;
pub const RG_AUD_CK_PDN_MASK: c_uint = 0x1;

pub const RG_ACCDET_CK_PDN_SFT: c_int = 0;
pub const RG_ACCDET_CK_PDN_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_CKPDN_CON0_SET
pub const RG_AUD_TOP_CKPDN_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_TOP_CKPDN_CON0_SET_MASK: c_uint = 0x3fff;

// MT6358_AUD_TOP_CKPDN_CON0_CLR
pub const RG_AUD_TOP_CKPDN_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_TOP_CKPDN_CON0_CLR_MASK: c_uint = 0x3fff;

// MT6358_AUD_TOP_CKSEL_CON0
pub const RG_AUDIF_CK_CKSEL_SFT: c_int = 3;
pub const RG_AUDIF_CK_CKSEL_MASK: c_uint = 0x1;

pub const RG_AUD_CK_CKSEL_SFT: c_int = 2;
pub const RG_AUD_CK_CKSEL_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_CKSEL_CON0_SET
pub const RG_AUD_TOP_CKSEL_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_TOP_CKSEL_CON0_SET_MASK: c_uint = 0xf;

// MT6358_AUD_TOP_CKSEL_CON0_CLR
pub const RG_AUD_TOP_CKSEL_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_TOP_CKSEL_CON0_CLR_MASK: c_uint = 0xf;

// MT6358_AUD_TOP_CKTST_CON0
pub const RG_VOW13M_CK_TSTSEL_SFT: c_int = 9;
pub const RG_VOW13M_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_VOW13M_CK_TST_DIS_SFT: c_int = 8;
pub const RG_VOW13M_CK_TST_DIS_MASK: c_uint = 0x1;

pub const RG_AUD26M_CK_TSTSEL_SFT: c_int = 4;
pub const RG_AUD26M_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_AUDIF_CK_TSTSEL_SFT: c_int = 3;
pub const RG_AUDIF_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_AUD_CK_TSTSEL_SFT: c_int = 2;
pub const RG_AUD_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_AUD26M_CK_TST_DIS_SFT: c_int = 0;
pub const RG_AUD26M_CK_TST_DIS_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_CLK_HWEN_CON0
pub const RG_AUD_INTRP_CK_PDN_HWEN_SFT: c_int = 0;
pub const RG_AUD_INTRP_CK_PDN_HWEN_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_CLK_HWEN_CON0_SET
pub const RG_AUD_INTRP_CK_PND_HWEN_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_INTRP_CK_PND_HWEN_CON0_SET_MASK: c_uint = 0xffff;

// MT6358_AUD_TOP_CLK_HWEN_CON0_CLR
pub const RG_AUD_INTRP_CLK_PDN_HWEN_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_INTRP_CLK_PDN_HWEN_CON0_CLR_MASK: c_uint = 0xffff;

// MT6358_AUD_TOP_RST_CON0
pub const RG_AUDNCP_RST_SFT: c_int = 3;
pub const RG_AUDNCP_RST_MASK: c_uint = 0x1;

pub const RG_ZCD_RST_SFT: c_int = 2;
pub const RG_ZCD_RST_MASK: c_uint = 0x1;

pub const RG_ACCDET_RST_SFT: c_int = 1;
pub const RG_ACCDET_RST_MASK: c_uint = 0x1;

pub const RG_AUDIO_RST_SFT: c_int = 0;
pub const RG_AUDIO_RST_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_RST_CON0_SET
pub const RG_AUD_TOP_RST_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_TOP_RST_CON0_SET_MASK: c_uint = 0xf;

// MT6358_AUD_TOP_RST_CON0_CLR
pub const RG_AUD_TOP_RST_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_TOP_RST_CON0_CLR_MASK: c_uint = 0xf;

// MT6358_AUD_TOP_RST_BANK_CON0
pub const BANK_AUDZCD_SWRST_SFT: c_int = 2;
pub const BANK_AUDZCD_SWRST_MASK: c_uint = 0x1;

pub const BANK_AUDIO_SWRST_SFT: c_int = 1;
pub const BANK_AUDIO_SWRST_MASK: c_uint = 0x1;

pub const BANK_ACCDET_SWRST_SFT: c_int = 0;
pub const BANK_ACCDET_SWRST_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_INT_CON0
pub const RG_INT_EN_AUDIO_SFT: c_int = 0;
pub const RG_INT_EN_AUDIO_MASK: c_uint = 0x1;

pub const RG_INT_EN_ACCDET_SFT: c_int = 5;
pub const RG_INT_EN_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_EN_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_EN_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_EN_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_EN_ACCDET_EINT1_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_INT_CON0_SET
pub const RG_AUD_INT_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_INT_CON0_SET_MASK: c_uint = 0xffff;

// MT6358_AUD_TOP_INT_CON0_CLR
pub const RG_AUD_INT_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_INT_CON0_CLR_MASK: c_uint = 0xffff;

// MT6358_AUD_TOP_INT_MASK_CON0
pub const RG_INT_MASK_AUDIO_SFT: c_int = 0;
pub const RG_INT_MASK_AUDIO_MASK: c_uint = 0x1;

pub const RG_INT_MASK_ACCDET_SFT: c_int = 5;
pub const RG_INT_MASK_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_MASK_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_MASK_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_MASK_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_MASK_ACCDET_EINT1_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_INT_MASK_CON0_SET
pub const RG_AUD_INT_MASK_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_INT_MASK_CON0_SET_MASK: c_uint = 0xff;

// MT6358_AUD_TOP_INT_MASK_CON0_CLR
pub const RG_AUD_INT_MASK_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_INT_MASK_CON0_CLR_MASK: c_uint = 0xff;

// MT6358_AUD_TOP_INT_STATUS0
pub const RG_INT_STATUS_AUDIO_SFT: c_int = 0;
pub const RG_INT_STATUS_AUDIO_MASK: c_uint = 0x1;

pub const RG_INT_STATUS_ACCDET_SFT: c_int = 5;
pub const RG_INT_STATUS_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_STATUS_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_STATUS_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_STATUS_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_STATUS_ACCDET_EINT1_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_INT_RAW_STATUS0
pub const RG_INT_RAW_STATUS_AUDIO_SFT: c_int = 0;
pub const RG_INT_RAW_STATUS_AUDIO_MASK: c_uint = 0x1;

pub const RG_INT_RAW_STATUS_ACCDET_SFT: c_int = 5;
pub const RG_INT_RAW_STATUS_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_RAW_STATUS_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_RAW_STATUS_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_RAW_STATUS_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_RAW_STATUS_ACCDET_EINT1_MASK: c_uint = 0x1;

// MT6358_AUD_TOP_INT_MISC_CON0
pub const RG_AUD_TOP_INT_POLARITY_SFT: c_int = 0;
pub const RG_AUD_TOP_INT_POLARITY_MASK: c_uint = 0x1;

// MT6358_AUDNCP_CLKDIV_CON0
pub const RG_DIVCKS_CHG_SFT: c_int = 0;
pub const RG_DIVCKS_CHG_MASK: c_uint = 0x1;

// MT6358_AUDNCP_CLKDIV_CON1
pub const RG_DIVCKS_ON_SFT: c_int = 0;
pub const RG_DIVCKS_ON_MASK: c_uint = 0x1;

// MT6358_AUDNCP_CLKDIV_CON2
pub const RG_DIVCKS_PRG_SFT: c_int = 0;
pub const RG_DIVCKS_PRG_MASK: c_uint = 0x1ff;

// MT6358_AUDNCP_CLKDIV_CON3
pub const RG_DIVCKS_PWD_NCP_SFT: c_int = 0;
pub const RG_DIVCKS_PWD_NCP_MASK: c_uint = 0x1;

// MT6358_AUDNCP_CLKDIV_CON4
pub const RG_DIVCKS_PWD_NCP_ST_SEL_SFT: c_int = 0;
pub const RG_DIVCKS_PWD_NCP_ST_SEL_MASK: c_uint = 0x3;

// MT6358_AUD_TOP_MON_CON0
pub const RG_AUD_TOP_MON_SEL_SFT: c_int = 0;
pub const RG_AUD_TOP_MON_SEL_MASK: c_uint = 0x7;

pub const RG_AUD_CLK_INT_MON_FLAG_SEL_SFT: c_int = 3;
pub const RG_AUD_CLK_INT_MON_FLAG_SEL_MASK: c_uint = 0xff;

pub const RG_AUD_CLK_INT_MON_FLAG_EN_SFT: c_int = 11;
pub const RG_AUD_CLK_INT_MON_FLAG_EN_MASK: c_uint = 0x1;

// MT6358_AUDIO_DIG_DSN_ID
pub const AUDIO_DIG_ANA_ID_SFT: c_int = 0;
pub const AUDIO_DIG_ANA_ID_MASK: c_uint = 0xff;

pub const AUDIO_DIG_DIG_ID_SFT: c_int = 8;
pub const AUDIO_DIG_DIG_ID_MASK: c_uint = 0xff;

// MT6358_AUDIO_DIG_DSN_REV0
pub const AUDIO_DIG_ANA_MINOR_REV_SFT: c_int = 0;
pub const AUDIO_DIG_ANA_MINOR_REV_MASK: c_uint = 0xf;

pub const AUDIO_DIG_ANA_MAJOR_REV_SFT: c_int = 4;
pub const AUDIO_DIG_ANA_MAJOR_REV_MASK: c_uint = 0xf;

pub const AUDIO_DIG_DIG_MINOR_REV_SFT: c_int = 8;
pub const AUDIO_DIG_DIG_MINOR_REV_MASK: c_uint = 0xf;

pub const AUDIO_DIG_DIG_MAJOR_REV_SFT: c_int = 12;
pub const AUDIO_DIG_DIG_MAJOR_REV_MASK: c_uint = 0xf;

// MT6358_AUDIO_DIG_DSN_DBI
pub const AUDIO_DIG_DSN_CBS_SFT: c_int = 0;
pub const AUDIO_DIG_DSN_CBS_MASK: c_uint = 0x3;

pub const AUDIO_DIG_DSN_BIX_SFT: c_int = 2;
pub const AUDIO_DIG_DSN_BIX_MASK: c_uint = 0x3;

pub const AUDIO_DIG_ESP_SFT: c_int = 8;
pub const AUDIO_DIG_ESP_MASK: c_uint = 0xff;

// MT6358_AUDIO_DIG_DSN_DXI
pub const AUDIO_DIG_DSN_FPI_SFT: c_int = 0;
pub const AUDIO_DIG_DSN_FPI_MASK: c_uint = 0xff;

// MT6358_AFE_UL_DL_CON0
pub const AFE_UL_LR_SWAP_SFT: c_int = 15;
pub const AFE_UL_LR_SWAP_MASK: c_uint = 0x1;

pub const AFE_DL_LR_SWAP_SFT: c_int = 14;
pub const AFE_DL_LR_SWAP_MASK: c_uint = 0x1;

pub const AFE_ON_SFT: c_int = 0;
pub const AFE_ON_MASK: c_uint = 0x1;

// MT6358_AFE_DL_SRC2_CON0_L
pub const DL_2_SRC_ON_TMP_CTL_PRE_SFT: c_int = 0;
pub const DL_2_SRC_ON_TMP_CTL_PRE_MASK: c_uint = 0x1;

// MT6358_AFE_UL_SRC_CON0_H
pub const C_DIGMIC_PHASE_SEL_CH1_CTL_SFT: c_int = 11;
pub const C_DIGMIC_PHASE_SEL_CH1_CTL_MASK: c_uint = 0x7;

pub const C_DIGMIC_PHASE_SEL_CH2_CTL_SFT: c_int = 8;
pub const C_DIGMIC_PHASE_SEL_CH2_CTL_MASK: c_uint = 0x7;

pub const C_TWO_DIGITAL_MIC_CTL_SFT: c_int = 7;
pub const C_TWO_DIGITAL_MIC_CTL_MASK: c_uint = 0x1;

// MT6358_AFE_UL_SRC_CON0_L
pub const DMIC_LOW_POWER_MODE_CTL_SFT: c_int = 14;
pub const DMIC_LOW_POWER_MODE_CTL_MASK: c_uint = 0x3;

pub const DIGMIC_3P25M_1P625M_SEL_CTL_SFT: c_int = 5;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK: c_uint = 0x1;

pub const UL_LOOP_BACK_MODE_CTL_SFT: c_int = 2;
pub const UL_LOOP_BACK_MODE_CTL_MASK: c_uint = 0x1;

pub const UL_SDM_3_LEVEL_CTL_SFT: c_int = 1;
pub const UL_SDM_3_LEVEL_CTL_MASK: c_uint = 0x1;

pub const UL_SRC_ON_TMP_CTL_SFT: c_int = 0;
pub const UL_SRC_ON_TMP_CTL_MASK: c_uint = 0x1;

// MT6358_AFE_TOP_CON0
pub const MTKAIF_SINE_ON_SFT: c_int = 2;
pub const MTKAIF_SINE_ON_MASK: c_uint = 0x1;

pub const UL_SINE_ON_SFT: c_int = 1;
pub const UL_SINE_ON_MASK: c_uint = 0x1;

pub const DL_SINE_ON_SFT: c_int = 0;
pub const DL_SINE_ON_MASK: c_uint = 0x1;

// MT6358_AUDIO_TOP_CON0
pub const PDN_AFE_CTL_SFT: c_int = 7;
pub const PDN_AFE_CTL_MASK: c_uint = 0x1;

pub const PDN_DAC_CTL_SFT: c_int = 6;
pub const PDN_DAC_CTL_MASK: c_uint = 0x1;

pub const PDN_ADC_CTL_SFT: c_int = 5;
pub const PDN_ADC_CTL_MASK: c_uint = 0x1;

pub const PDN_I2S_DL_CTL_SFT: c_int = 3;
pub const PDN_I2S_DL_CTL_MASK: c_uint = 0x1;

pub const PWR_CLK_DIS_CTL_SFT: c_int = 2;
pub const PWR_CLK_DIS_CTL_MASK: c_uint = 0x1;

pub const PDN_AFE_TESTMODEL_CTL_SFT: c_int = 1;
pub const PDN_AFE_TESTMODEL_CTL_MASK: c_uint = 0x1;

pub const PDN_RESERVED_SFT: c_int = 0;
pub const PDN_RESERVED_MASK: c_uint = 0x1;

// MT6358_AFE_MON_DEBUG0
pub const AUDIO_SYS_TOP_MON_SWAP_SFT: c_int = 14;
pub const AUDIO_SYS_TOP_MON_SWAP_MASK: c_uint = 0x3;

pub const AUDIO_SYS_TOP_MON_SEL_SFT: c_int = 8;
pub const AUDIO_SYS_TOP_MON_SEL_MASK: c_uint = 0x1f;

pub const AFE_MON_SEL_SFT: c_int = 0;
pub const AFE_MON_SEL_MASK: c_uint = 0xff;

// MT6358_AFUNC_AUD_CON0
pub const CCI_AUD_ANACK_SEL_SFT: c_int = 15;
pub const CCI_AUD_ANACK_SEL_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_WPTR_SFT: c_int = 12;
pub const CCI_AUDIO_FIFO_WPTR_MASK: c_uint = 0x7;

pub const CCI_SCRAMBLER_CG_EN_SFT: c_int = 11;
pub const CCI_SCRAMBLER_CG_EN_MASK: c_uint = 0x1;

pub const CCI_LCH_INV_SFT: c_int = 10;
pub const CCI_LCH_INV_MASK: c_uint = 0x1;

pub const CCI_RAND_EN_SFT: c_int = 9;
pub const CCI_RAND_EN_MASK: c_uint = 0x1;

pub const CCI_SPLT_SCRMB_CLK_ON_SFT: c_int = 8;
pub const CCI_SPLT_SCRMB_CLK_ON_MASK: c_uint = 0x1;

pub const CCI_SPLT_SCRMB_ON_SFT: c_int = 7;
pub const CCI_SPLT_SCRMB_ON_MASK: c_uint = 0x1;

pub const CCI_AUD_IDAC_TEST_EN_SFT: c_int = 6;
pub const CCI_AUD_IDAC_TEST_EN_MASK: c_uint = 0x1;

pub const CCI_ZERO_PAD_DISABLE_SFT: c_int = 5;
pub const CCI_ZERO_PAD_DISABLE_MASK: c_uint = 0x1;

pub const CCI_AUD_SPLIT_TEST_EN_SFT: c_int = 4;
pub const CCI_AUD_SPLIT_TEST_EN_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_MUTEL_SFT: c_int = 3;
pub const CCI_AUD_SDM_MUTEL_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_MUTER_SFT: c_int = 2;
pub const CCI_AUD_SDM_MUTER_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_7BIT_SEL_SFT: c_int = 1;
pub const CCI_AUD_SDM_7BIT_SEL_MASK: c_uint = 0x1;

pub const CCI_SCRAMBLER_EN_SFT: c_int = 0;
pub const CCI_SCRAMBLER_EN_MASK: c_uint = 0x1;

// MT6358_AFUNC_AUD_CON1
pub const AUD_SDM_TEST_L_SFT: c_int = 8;
pub const AUD_SDM_TEST_L_MASK: c_uint = 0xff;

pub const AUD_SDM_TEST_R_SFT: c_int = 0;
pub const AUD_SDM_TEST_R_MASK: c_uint = 0xff;

// MT6358_AFUNC_AUD_CON2
pub const CCI_AUD_DAC_ANA_MUTE_SFT: c_int = 7;
pub const CCI_AUD_DAC_ANA_MUTE_MASK: c_uint = 0x1;

pub const CCI_AUD_DAC_ANA_RSTB_SEL_SFT: c_int = 6;
pub const CCI_AUD_DAC_ANA_RSTB_SEL_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_CLKIN_INV_SFT: c_int = 4;
pub const CCI_AUDIO_FIFO_CLKIN_INV_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_ENABLE_SFT: c_int = 3;
pub const CCI_AUDIO_FIFO_ENABLE_MASK: c_uint = 0x1;

pub const CCI_ACD_MODE_SFT: c_int = 2;
pub const CCI_ACD_MODE_MASK: c_uint = 0x1;

pub const CCI_AFIFO_CLK_PWDB_SFT: c_int = 1;
pub const CCI_AFIFO_CLK_PWDB_MASK: c_uint = 0x1;

pub const CCI_ACD_FUNC_RSTB_SFT: c_int = 0;
pub const CCI_ACD_FUNC_RSTB_MASK: c_uint = 0x1;

// MT6358_AFUNC_AUD_CON3
pub const SDM_ANA13M_TESTCK_SEL_SFT: c_int = 15;
pub const SDM_ANA13M_TESTCK_SEL_MASK: c_uint = 0x1;

pub const SDM_ANA13M_TESTCK_SRC_SEL_SFT: c_int = 12;
pub const SDM_ANA13M_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const SDM_TESTCK_SRC_SEL_SFT: c_int = 8;
pub const SDM_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const DIGMIC_TESTCK_SRC_SEL_SFT: c_int = 4;
pub const DIGMIC_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const DIGMIC_TESTCK_SEL_SFT: c_int = 0;
pub const DIGMIC_TESTCK_SEL_MASK: c_uint = 0x1;

// MT6358_AFUNC_AUD_CON4
pub const UL_FIFO_WCLK_INV_SFT: c_int = 8;
pub const UL_FIFO_WCLK_INV_MASK: c_uint = 0x1;

pub const UL_FIFO_DIGMIC_WDATA_TESTSRC_SEL_SFT: c_int = 6;
pub const UL_FIFO_DIGMIC_WDATA_TESTSRC_SEL_MASK: c_uint = 0x1;

pub const UL_FIFO_WDATA_TESTEN_SFT: c_int = 5;
pub const UL_FIFO_WDATA_TESTEN_MASK: c_uint = 0x1;

pub const UL_FIFO_WDATA_TESTSRC_SEL_SFT: c_int = 4;
pub const UL_FIFO_WDATA_TESTSRC_SEL_MASK: c_uint = 0x1;

pub const UL_FIFO_WCLK_6P5M_TESTCK_SEL_SFT: c_int = 3;
pub const UL_FIFO_WCLK_6P5M_TESTCK_SEL_MASK: c_uint = 0x1;

pub const UL_FIFO_WCLK_6P5M_TESTCK_SRC_SEL_SFT: c_int = 0;
pub const UL_FIFO_WCLK_6P5M_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

// MT6358_AFUNC_AUD_CON5
pub const R_AUD_DAC_POS_LARGE_MONO_SFT: c_int = 8;
pub const R_AUD_DAC_POS_LARGE_MONO_MASK: c_uint = 0xff;

pub const R_AUD_DAC_NEG_LARGE_MONO_SFT: c_int = 0;
pub const R_AUD_DAC_NEG_LARGE_MONO_MASK: c_uint = 0xff;

// MT6358_AFUNC_AUD_CON6
pub const R_AUD_DAC_POS_SMALL_MONO_SFT: c_int = 12;
pub const R_AUD_DAC_POS_SMALL_MONO_MASK: c_uint = 0xf;

pub const R_AUD_DAC_NEG_SMALL_MONO_SFT: c_int = 8;
pub const R_AUD_DAC_NEG_SMALL_MONO_MASK: c_uint = 0xf;

pub const R_AUD_DAC_POS_TINY_MONO_SFT: c_int = 6;
pub const R_AUD_DAC_POS_TINY_MONO_MASK: c_uint = 0x3;

pub const R_AUD_DAC_NEG_TINY_MONO_SFT: c_int = 4;
pub const R_AUD_DAC_NEG_TINY_MONO_MASK: c_uint = 0x3;

pub const R_AUD_DAC_MONO_SEL_SFT: c_int = 3;
pub const R_AUD_DAC_MONO_SEL_MASK: c_uint = 0x1;

pub const R_AUD_DAC_SW_RSTB_SFT: c_int = 0;
pub const R_AUD_DAC_SW_RSTB_MASK: c_uint = 0x1;

// MT6358_AFUNC_AUD_MON0
pub const AUD_SCR_OUT_L_SFT: c_int = 8;
pub const AUD_SCR_OUT_L_MASK: c_uint = 0xff;

pub const AUD_SCR_OUT_R_SFT: c_int = 0;
pub const AUD_SCR_OUT_R_MASK: c_uint = 0xff;

// MT6358_AUDRC_TUNE_MON0
pub const ASYNC_TEST_OUT_BCK_SFT: c_int = 15;
pub const ASYNC_TEST_OUT_BCK_MASK: c_uint = 0x1;

pub const RGS_AUDRCTUNE1READ_SFT: c_int = 8;
pub const RGS_AUDRCTUNE1READ_MASK: c_uint = 0x1f;

pub const RGS_AUDRCTUNE0READ_SFT: c_int = 0;
pub const RGS_AUDRCTUNE0READ_MASK: c_uint = 0x1f;

// MT6358_AFE_ADDA_MTKAIF_FIFO_CFG0
pub const AFE_RESERVED_SFT: c_int = 1;
pub const AFE_RESERVED_MASK: c_uint = 0x7fff;

pub const RG_MTKAIF_RXIF_FIFO_INTEN_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_FIFO_INTEN_MASK: c_uint = 0x1;

// MT6358_AFE_ADDA_MTKAIF_FIFO_LOG_MON1
pub const MTKAIF_RXIF_WR_FULL_STATUS_SFT: c_int = 1;
pub const MTKAIF_RXIF_WR_FULL_STATUS_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_RD_EMPTY_STATUS_SFT: c_int = 0;
pub const MTKAIF_RXIF_RD_EMPTY_STATUS_MASK: c_uint = 0x1;

// MT6358_AFE_ADDA_MTKAIF_MON0
pub const MTKAIFTX_V3_SYNC_OUT_SFT: c_int = 14;
pub const MTKAIFTX_V3_SYNC_OUT_MASK: c_uint = 0x1;

pub const MTKAIFTX_V3_SDATA_OUT2_SFT: c_int = 13;
pub const MTKAIFTX_V3_SDATA_OUT2_MASK: c_uint = 0x1;

pub const MTKAIFTX_V3_SDATA_OUT1_SFT: c_int = 12;
pub const MTKAIFTX_V3_SDATA_OUT1_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_FIFO_STATUS_SFT: c_int = 0;
pub const MTKAIF_RXIF_FIFO_STATUS_MASK: c_uint = 0xfff;

// MT6358_AFE_ADDA_MTKAIF_MON1
pub const MTKAIFRX_V3_SYNC_IN_SFT: c_int = 14;
pub const MTKAIFRX_V3_SYNC_IN_MASK: c_uint = 0x1;

pub const MTKAIFRX_V3_SDATA_IN2_SFT: c_int = 13;
pub const MTKAIFRX_V3_SDATA_IN2_MASK: c_uint = 0x1;

pub const MTKAIFRX_V3_SDATA_IN1_SFT: c_int = 12;
pub const MTKAIFRX_V3_SDATA_IN1_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_SEARCH_FAIL_FLAG_SFT: c_int = 11;
pub const MTKAIF_RXIF_SEARCH_FAIL_FLAG_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_INVALID_FLAG_SFT: c_int = 8;
pub const MTKAIF_RXIF_INVALID_FLAG_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_INVALID_CYCLE_SFT: c_int = 0;
pub const MTKAIF_RXIF_INVALID_CYCLE_MASK: c_uint = 0xff;

// MT6358_AFE_ADDA_MTKAIF_MON2
pub const MTKAIF_TXIF_IN_CH2_SFT: c_int = 8;
pub const MTKAIF_TXIF_IN_CH2_MASK: c_uint = 0xff;

pub const MTKAIF_TXIF_IN_CH1_SFT: c_int = 0;
pub const MTKAIF_TXIF_IN_CH1_MASK: c_uint = 0xff;

// MT6358_AFE_ADDA_MTKAIF_MON3
pub const MTKAIF_RXIF_OUT_CH2_SFT: c_int = 8;
pub const MTKAIF_RXIF_OUT_CH2_MASK: c_uint = 0xff;

pub const MTKAIF_RXIF_OUT_CH1_SFT: c_int = 0;
pub const MTKAIF_RXIF_OUT_CH1_MASK: c_uint = 0xff;

// MT6358_AFE_ADDA_MTKAIF_CFG0
pub const RG_MTKAIF_RXIF_CLKINV_SFT: c_int = 15;
pub const RG_MTKAIF_RXIF_CLKINV_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_PROTOCOL2_SFT: c_int = 8;
pub const RG_MTKAIF_RXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_MTKAIF_BYPASS_SRC_MODE_SFT: c_int = 6;
pub const RG_MTKAIF_BYPASS_SRC_MODE_MASK: c_uint = 0x3;

pub const RG_MTKAIF_BYPASS_SRC_TEST_SFT: c_int = 5;
pub const RG_MTKAIF_BYPASS_SRC_TEST_MASK: c_uint = 0x1;

pub const RG_MTKAIF_TXIF_PROTOCOL2_SFT: c_int = 4;
pub const RG_MTKAIF_TXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_MTKAIF_PMIC_TXIF_8TO5_SFT: c_int = 2;
pub const RG_MTKAIF_PMIC_TXIF_8TO5_MASK: c_uint = 0x1;

pub const RG_MTKAIF_LOOPBACK_TEST2_SFT: c_int = 1;
pub const RG_MTKAIF_LOOPBACK_TEST2_MASK: c_uint = 0x1;

pub const RG_MTKAIF_LOOPBACK_TEST1_SFT: c_int = 0;
pub const RG_MTKAIF_LOOPBACK_TEST1_MASK: c_uint = 0x1;

// MT6358_AFE_ADDA_MTKAIF_RX_CFG0
pub const RG_MTKAIF_RXIF_VOICE_MODE_SFT: c_int = 12;
pub const RG_MTKAIF_RXIF_VOICE_MODE_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_DATA_BIT_SFT: c_int = 8;
pub const RG_MTKAIF_RXIF_DATA_BIT_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RXIF_FIFO_RSP_SFT: c_int = 4;
pub const RG_MTKAIF_RXIF_FIFO_RSP_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RXIF_DETECT_ON_SFT: c_int = 3;
pub const RG_MTKAIF_RXIF_DETECT_ON_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_DATA_MODE_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_DATA_MODE_MASK: c_uint = 0x1;

// MT6358_AFE_ADDA_MTKAIF_RX_CFG1
pub const RG_MTKAIF_RXIF_SYNC_SEARCH_TABLE_SFT: c_int = 12;
pub const RG_MTKAIF_RXIF_SYNC_SEARCH_TABLE_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_INVALID_SYNC_CHECK_ROUND_SFT: c_int = 8;
pub const RG_MTKAIF_RXIF_INVALID_SYNC_CHECK_ROUND_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_SYNC_CHECK_ROUND_SFT: c_int = 4;
pub const RG_MTKAIF_RXIF_SYNC_CHECK_ROUND_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_VOICE_MODE_PROTOCOL2_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_VOICE_MODE_PROTOCOL2_MASK: c_uint = 0xf;

// MT6358_AFE_ADDA_MTKAIF_RX_CFG2
pub const RG_MTKAIF_RXIF_CLEAR_SYNC_FAIL_SFT: c_int = 12;
pub const RG_MTKAIF_RXIF_CLEAR_SYNC_FAIL_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_SYNC_CNT_TABLE_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_SYNC_CNT_TABLE_MASK: c_uint = 0xfff;

// MT6358_AFE_ADDA_MTKAIF_RX_CFG3
pub const RG_MTKAIF_RXIF_LOOPBACK_USE_NLE_SFT: c_int = 7;
pub const RG_MTKAIF_RXIF_LOOPBACK_USE_NLE_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_SFT: c_int = 4;
pub const RG_MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RXIF_DETECT_ON_PROTOCOL2_SFT: c_int = 3;
pub const RG_MTKAIF_RXIF_DETECT_ON_PROTOCOL2_MASK: c_uint = 0x1;

// MT6358_AFE_ADDA_MTKAIF_TX_CFG1
pub const RG_MTKAIF_SYNC_WORD2_SFT: c_int = 4;
pub const RG_MTKAIF_SYNC_WORD2_MASK: c_uint = 0x7;

pub const RG_MTKAIF_SYNC_WORD1_SFT: c_int = 0;
pub const RG_MTKAIF_SYNC_WORD1_MASK: c_uint = 0x7;

// MT6358_AFE_SGEN_CFG0
pub const SGEN_AMP_DIV_CH1_CTL_SFT: c_int = 12;
pub const SGEN_AMP_DIV_CH1_CTL_MASK: c_uint = 0xf;

pub const SGEN_DAC_EN_CTL_SFT: c_int = 7;
pub const SGEN_DAC_EN_CTL_MASK: c_uint = 0x1;

pub const SGEN_MUTE_SW_CTL_SFT: c_int = 6;
pub const SGEN_MUTE_SW_CTL_MASK: c_uint = 0x1;

pub const R_AUD_SDM_MUTE_L_SFT: c_int = 5;
pub const R_AUD_SDM_MUTE_L_MASK: c_uint = 0x1;

pub const R_AUD_SDM_MUTE_R_SFT: c_int = 4;
pub const R_AUD_SDM_MUTE_R_MASK: c_uint = 0x1;

// MT6358_AFE_SGEN_CFG1
pub const C_SGEN_RCH_INV_5BIT_SFT: c_int = 15;
pub const C_SGEN_RCH_INV_5BIT_MASK: c_uint = 0x1;

pub const C_SGEN_RCH_INV_8BIT_SFT: c_int = 14;
pub const C_SGEN_RCH_INV_8BIT_MASK: c_uint = 0x1;

pub const SGEN_FREQ_DIV_CH1_CTL_SFT: c_int = 0;
pub const SGEN_FREQ_DIV_CH1_CTL_MASK: c_uint = 0x1f;

// MT6358_AFE_ADC_ASYNC_FIFO_CFG
pub const RG_UL_ASYNC_FIFO_SOFT_RST_EN_SFT: c_int = 5;
pub const RG_UL_ASYNC_FIFO_SOFT_RST_EN_MASK: c_uint = 0x1;

pub const RG_UL_ASYNC_FIFO_SOFT_RST_SFT: c_int = 4;
pub const RG_UL_ASYNC_FIFO_SOFT_RST_MASK: c_uint = 0x1;

pub const RG_AMIC_UL_ADC_CLK_SEL_SFT: c_int = 1;
pub const RG_AMIC_UL_ADC_CLK_SEL_MASK: c_uint = 0x1;

// MT6358_AFE_DCCLK_CFG0
pub const DCCLK_DIV_SFT: c_int = 5;
pub const DCCLK_DIV_MASK: c_uint = 0x7ff;

pub const DCCLK_INV_SFT: c_int = 4;
pub const DCCLK_INV_MASK: c_uint = 0x1;

pub const DCCLK_PDN_SFT: c_int = 1;
pub const DCCLK_PDN_MASK: c_uint = 0x1;

pub const DCCLK_GEN_ON_SFT: c_int = 0;
pub const DCCLK_GEN_ON_MASK: c_uint = 0x1;

// MT6358_AFE_DCCLK_CFG1
pub const RESYNC_SRC_SEL_SFT: c_int = 10;
pub const RESYNC_SRC_SEL_MASK: c_uint = 0x3;

pub const RESYNC_SRC_CK_INV_SFT: c_int = 9;
pub const RESYNC_SRC_CK_INV_MASK: c_uint = 0x1;

pub const DCCLK_RESYNC_BYPASS_SFT: c_int = 8;
pub const DCCLK_RESYNC_BYPASS_MASK: c_uint = 0x1;

pub const DCCLK_PHASE_SEL_SFT: c_int = 4;
pub const DCCLK_PHASE_SEL_MASK: c_uint = 0xf;

// MT6358_AUDIO_DIG_CFG
pub const RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_SFT: c_int = 15;
pub const RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_PHASE_MODE2_SFT: c_int = 8;
pub const RG_AUD_PAD_TOP_PHASE_MODE2_MASK: c_uint = 0x7f;

pub const RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_SFT: c_int = 7;
pub const RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_PHASE_MODE_SFT: c_int = 0;
pub const RG_AUD_PAD_TOP_PHASE_MODE_MASK: c_uint = 0x7f;

// MT6358_AFE_AUD_PAD_TOP
pub const RG_AUD_PAD_TOP_TX_FIFO_RSP_SFT: c_int = 12;
pub const RG_AUD_PAD_TOP_TX_FIFO_RSP_MASK: c_uint = 0x7;

pub const RG_AUD_PAD_TOP_MTKAIF_CLK_PROTOCOL2_SFT: c_int = 11;
pub const RG_AUD_PAD_TOP_MTKAIF_CLK_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_TX_FIFO_ON_SFT: c_int = 8;
pub const RG_AUD_PAD_TOP_TX_FIFO_ON_MASK: c_uint = 0x1;

// MT6358_AFE_AUD_PAD_TOP_MON
pub const ADDA_AUD_PAD_TOP_MON_SFT: c_int = 0;
pub const ADDA_AUD_PAD_TOP_MON_MASK: c_uint = 0xffff;

// MT6358_AFE_AUD_PAD_TOP_MON1
pub const ADDA_AUD_PAD_TOP_MON1_SFT: c_int = 0;
pub const ADDA_AUD_PAD_TOP_MON1_MASK: c_uint = 0xffff;

// MT6358_AFE_DL_NLE_CFG
pub const NLE_RCH_HPGAIN_SEL_SFT: c_int = 10;
pub const NLE_RCH_HPGAIN_SEL_MASK: c_uint = 0x1;

pub const NLE_RCH_CH_SEL_SFT: c_int = 9;
pub const NLE_RCH_CH_SEL_MASK: c_uint = 0x1;

pub const NLE_RCH_ON_SFT: c_int = 8;
pub const NLE_RCH_ON_MASK: c_uint = 0x1;

pub const NLE_LCH_HPGAIN_SEL_SFT: c_int = 2;
pub const NLE_LCH_HPGAIN_SEL_MASK: c_uint = 0x1;

pub const NLE_LCH_CH_SEL_SFT: c_int = 1;
pub const NLE_LCH_CH_SEL_MASK: c_uint = 0x1;

pub const NLE_LCH_ON_SFT: c_int = 0;
pub const NLE_LCH_ON_MASK: c_uint = 0x1;

// MT6358_AFE_DL_NLE_MON
pub const NLE_MONITOR_SFT: c_int = 0;
pub const NLE_MONITOR_MASK: c_uint = 0x3fff;

// MT6358_AFE_CG_EN_MON
pub const CK_CG_EN_MON_SFT: c_int = 0;
pub const CK_CG_EN_MON_MASK: c_uint = 0x3f;

// MT6358_AFE_VOW_TOP
pub const PDN_VOW_SFT: c_int = 15;
pub const PDN_VOW_MASK: c_uint = 0x1;

pub const VOW_1P6M_800K_SEL_SFT: c_int = 14;
pub const VOW_1P6M_800K_SEL_MASK: c_uint = 0x1;

pub const VOW_DIGMIC_ON_SFT: c_int = 13;
pub const VOW_DIGMIC_ON_MASK: c_uint = 0x1;

pub const VOW_CK_DIV_RST_SFT: c_int = 12;
pub const VOW_CK_DIV_RST_MASK: c_uint = 0x1;

pub const VOW_ON_SFT: c_int = 11;
pub const VOW_ON_MASK: c_uint = 0x1;

pub const VOW_DIGMIC_CK_PHASE_SEL_SFT: c_int = 8;
pub const VOW_DIGMIC_CK_PHASE_SEL_MASK: c_uint = 0x7;

pub const MAIN_DMIC_CK_VOW_SEL_SFT: c_int = 7;
pub const MAIN_DMIC_CK_VOW_SEL_MASK: c_uint = 0x1;

pub const VOW_SDM_3_LEVEL_SFT: c_int = 6;
pub const VOW_SDM_3_LEVEL_MASK: c_uint = 0x1;

pub const VOW_LOOP_BACK_MODE_SFT: c_int = 5;
pub const VOW_LOOP_BACK_MODE_MASK: c_uint = 0x1;

pub const VOW_INTR_SOURCE_SEL_SFT: c_int = 4;
pub const VOW_INTR_SOURCE_SEL_MASK: c_uint = 0x1;

pub const VOW_INTR_CLR_SFT: c_int = 3;
pub const VOW_INTR_CLR_MASK: c_uint = 0x1;

pub const S_N_VALUE_RST_SFT: c_int = 2;
pub const S_N_VALUE_RST_MASK: c_uint = 0x1;

pub const SAMPLE_BASE_MODE_SFT: c_int = 1;
pub const SAMPLE_BASE_MODE_MASK: c_uint = 0x1;

pub const VOW_INTR_FLAG_SFT: c_int = 0;
pub const VOW_INTR_FLAG_MASK: c_uint = 0x1;

// MT6358_AFE_VOW_CFG0
pub const AMPREF_SFT: c_int = 0;
pub const AMPREF_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_CFG1
pub const TIMERINI_SFT: c_int = 0;
pub const TIMERINI_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_CFG2
pub const B_DEFAULT_SFT: c_int = 12;
pub const B_DEFAULT_MASK: c_uint = 0x7;

pub const A_DEFAULT_SFT: c_int = 8;
pub const A_DEFAULT_MASK: c_uint = 0x7;

pub const B_INI_SFT: c_int = 4;
pub const B_INI_MASK: c_uint = 0x7;

pub const A_INI_SFT: c_int = 0;
pub const A_INI_MASK: c_uint = 0x7;

// MT6358_AFE_VOW_CFG3
pub const K_BETA_RISE_SFT: c_int = 12;
pub const K_BETA_RISE_MASK: c_uint = 0xf;

pub const K_BETA_FALL_SFT: c_int = 8;
pub const K_BETA_FALL_MASK: c_uint = 0xf;

pub const K_ALPHA_RISE_SFT: c_int = 4;
pub const K_ALPHA_RISE_MASK: c_uint = 0xf;

pub const K_ALPHA_FALL_SFT: c_int = 0;
pub const K_ALPHA_FALL_MASK: c_uint = 0xf;

// MT6358_AFE_VOW_CFG4
pub const VOW_TXIF_SCK_INV_SFT: c_int = 15;
pub const VOW_TXIF_SCK_INV_MASK: c_uint = 0x1;

pub const VOW_ADC_TESTCK_SRC_SEL_SFT: c_int = 12;
pub const VOW_ADC_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const VOW_ADC_TESTCK_SEL_SFT: c_int = 11;
pub const VOW_ADC_TESTCK_SEL_MASK: c_uint = 0x1;

pub const VOW_ADC_CLK_INV_SFT: c_int = 10;
pub const VOW_ADC_CLK_INV_MASK: c_uint = 0x1;

pub const VOW_TXIF_MONO_SFT: c_int = 9;
pub const VOW_TXIF_MONO_MASK: c_uint = 0x1;

pub const VOW_TXIF_SCK_DIV_SFT: c_int = 4;
pub const VOW_TXIF_SCK_DIV_MASK: c_uint = 0x1f;

pub const K_GAMMA_SFT: c_int = 0;
pub const K_GAMMA_MASK: c_uint = 0xf;

// MT6358_AFE_VOW_CFG5
pub const N_MIN_SFT: c_int = 0;
pub const N_MIN_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_CFG6
pub const RG_WINDOW_SIZE_SEL_SFT: c_int = 12;
pub const RG_WINDOW_SIZE_SEL_MASK: c_uint = 0x1;

pub const RG_FLR_BYPASS_SFT: c_int = 11;
pub const RG_FLR_BYPASS_MASK: c_uint = 0x1;

pub const RG_FLR_RATIO_SFT: c_int = 8;
pub const RG_FLR_RATIO_MASK: c_uint = 0x7;

pub const RG_BUCK_DVFS_DONE_SW_CTL_SFT: c_int = 7;
pub const RG_BUCK_DVFS_DONE_SW_CTL_MASK: c_uint = 0x1;

pub const RG_BUCK_DVFS_DONE_HW_MODE_SFT: c_int = 6;
pub const RG_BUCK_DVFS_DONE_HW_MODE_MASK: c_uint = 0x1;

pub const RG_BUCK_DVFS_HW_CNT_THR_SFT: c_int = 0;
pub const RG_BUCK_DVFS_HW_CNT_THR_MASK: c_uint = 0x3f;

// MT6358_AFE_VOW_MON0
pub const VOW_DOWNCNT_SFT: c_int = 0;
pub const VOW_DOWNCNT_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_MON1
pub const K_TMP_MON_SFT: c_int = 10;
pub const K_TMP_MON_MASK: c_uint = 0xf;

pub const SLT_COUNTER_MON_SFT: c_int = 7;
pub const SLT_COUNTER_MON_MASK: c_uint = 0x7;

pub const VOW_B_SFT: c_int = 4;
pub const VOW_B_MASK: c_uint = 0x7;

pub const VOW_A_SFT: c_int = 1;
pub const VOW_A_MASK: c_uint = 0x7;

pub const SECOND_CNT_START_SFT: c_int = 0;
pub const SECOND_CNT_START_MASK: c_uint = 0x1;

// MT6358_AFE_VOW_MON2
pub const VOW_S_L_SFT: c_int = 0;
pub const VOW_S_L_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_MON3
pub const VOW_S_H_SFT: c_int = 0;
pub const VOW_S_H_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_MON4
pub const VOW_N_L_SFT: c_int = 0;
pub const VOW_N_L_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_MON5
pub const VOW_N_H_SFT: c_int = 0;
pub const VOW_N_H_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_SN_INI_CFG
pub const VOW_SN_INI_CFG_EN_SFT: c_int = 15;
pub const VOW_SN_INI_CFG_EN_MASK: c_uint = 0x1;

pub const VOW_SN_INI_CFG_VAL_SFT: c_int = 0;
pub const VOW_SN_INI_CFG_VAL_MASK: c_uint = 0x7fff;

// MT6358_AFE_VOW_TGEN_CFG0
pub const VOW_TGEN_EN_SFT: c_int = 15;
pub const VOW_TGEN_EN_MASK: c_uint = 0x1;

pub const VOW_TGEN_MUTE_SW_SFT: c_int = 14;
pub const VOW_TGEN_MUTE_SW_MASK: c_uint = 0x1;

pub const VOW_TGEN_FREQ_DIV_SFT: c_int = 0;
pub const VOW_TGEN_FREQ_DIV_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_POSDIV_CFG0
pub const BUCK_DVFS_DONE_SFT: c_int = 15;
pub const BUCK_DVFS_DONE_MASK: c_uint = 0x1;

pub const VOW_32K_MODE_SFT: c_int = 13;
pub const VOW_32K_MODE_MASK: c_uint = 0x1;

pub const RG_BUCK_CLK_DIV_SFT: c_int = 8;
pub const RG_BUCK_CLK_DIV_MASK: c_uint = 0x1f;

pub const RG_A1P6M_EN_SEL_SFT: c_int = 7;
pub const RG_A1P6M_EN_SEL_MASK: c_uint = 0x1;

pub const VOW_CLK_SEL_SFT: c_int = 6;
pub const VOW_CLK_SEL_MASK: c_uint = 0x1;

pub const VOW_INTR_SW_MODE_SFT: c_int = 5;
pub const VOW_INTR_SW_MODE_MASK: c_uint = 0x1;

pub const VOW_INTR_SW_VAL_SFT: c_int = 4;
pub const VOW_INTR_SW_VAL_MASK: c_uint = 0x1;

pub const VOW_CIC_MODE_SEL_SFT: c_int = 2;
pub const VOW_CIC_MODE_SEL_MASK: c_uint = 0x3;

pub const RG_VOW_POSDIV_SFT: c_int = 0;
pub const RG_VOW_POSDIV_MASK: c_uint = 0x3;

// MT6358_AFE_VOW_HPF_CFG0
pub const VOW_HPF_DC_TEST_SFT: c_int = 12;
pub const VOW_HPF_DC_TEST_MASK: c_uint = 0xf;

pub const VOW_IRQ_LATCH_SNR_EN_SFT: c_int = 10;
pub const VOW_IRQ_LATCH_SNR_EN_MASK: c_uint = 0x1;

pub const VOW_DMICCLK_PDN_SFT: c_int = 9;
pub const VOW_DMICCLK_PDN_MASK: c_uint = 0x1;

pub const VOW_POSDIVCLK_PDN_SFT: c_int = 8;
pub const VOW_POSDIVCLK_PDN_MASK: c_uint = 0x1;

pub const RG_BASELINE_ALPHA_ORDER_SFT: c_int = 4;
pub const RG_BASELINE_ALPHA_ORDER_MASK: c_uint = 0xf;

pub const RG_MTKAIF_HPF_BYPASS_SFT: c_int = 2;
pub const RG_MTKAIF_HPF_BYPASS_MASK: c_uint = 0x1;

pub const RG_SNRDET_HPF_BYPASS_SFT: c_int = 1;
pub const RG_SNRDET_HPF_BYPASS_MASK: c_uint = 0x1;

pub const RG_HPF_ON_SFT: c_int = 0;
pub const RG_HPF_ON_MASK: c_uint = 0x1;

// MT6358_AFE_VOW_PERIODIC_CFG0
pub const RG_PERIODIC_EN_SFT: c_int = 15;
pub const RG_PERIODIC_EN_MASK: c_uint = 0x1;

pub const RG_PERIODIC_CNT_CLR_SFT: c_int = 14;
pub const RG_PERIODIC_CNT_CLR_MASK: c_uint = 0x1;

pub const RG_PERIODIC_CNT_PERIOD_SFT: c_int = 0;
pub const RG_PERIODIC_CNT_PERIOD_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG1
pub const RG_PERIODIC_CNT_SET_SFT: c_int = 15;
pub const RG_PERIODIC_CNT_SET_MASK: c_uint = 0x1;

pub const RG_PERIODIC_CNT_PAUSE_SFT: c_int = 14;
pub const RG_PERIODIC_CNT_PAUSE_MASK: c_uint = 0x1;

pub const RG_PERIODIC_CNT_SET_VALUE_SFT: c_int = 0;
pub const RG_PERIODIC_CNT_SET_VALUE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG2
pub const AUDPREAMPLON_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDPREAMPLON_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDPREAMPLON_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDPREAMPLON_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDPREAMPLON_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDPREAMPLON_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG3
pub const AUDPREAMPLDCPRECHARGE_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDPREAMPLDCPRECHARGE_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDPREAMPLDCPRECHARGE_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDPREAMPLDCPRECHARGE_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDPREAMPLDCPRECHARGE_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDPREAMPLDCPRECHARGE_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG4
pub const AUDADCLPWRUP_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDADCLPWRUP_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDADCLPWRUP_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDADCLPWRUP_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDADCLPWRUP_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDADCLPWRUP_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG5
pub const AUDGLBVOWLPWEN_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDGLBVOWLPWEN_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDGLBVOWLPWEN_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDGLBVOWLPWEN_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDGLBVOWLPWEN_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDGLBVOWLPWEN_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG6
pub const AUDDIGMICEN_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDDIGMICEN_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDDIGMICEN_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDDIGMICEN_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDDIGMICEN_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDDIGMICEN_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG7
pub const AUDPWDBMICBIAS0_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDPWDBMICBIAS0_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDPWDBMICBIAS0_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDPWDBMICBIAS0_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDPWDBMICBIAS0_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDPWDBMICBIAS0_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG8
pub const AUDPWDBMICBIAS1_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDPWDBMICBIAS1_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDPWDBMICBIAS1_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDPWDBMICBIAS1_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDPWDBMICBIAS1_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDPWDBMICBIAS1_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG9
pub const XO_VOW_CK_EN_PERIODIC_MODE_SFT: c_int = 15;
pub const XO_VOW_CK_EN_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const XO_VOW_CK_EN_PERIODIC_INVERSE_SFT: c_int = 14;
pub const XO_VOW_CK_EN_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const XO_VOW_CK_EN_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const XO_VOW_CK_EN_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG10
pub const AUDGLB_PWRDN_PERIODIC_MODE_SFT: c_int = 15;
pub const AUDGLB_PWRDN_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const AUDGLB_PWRDN_PERIODIC_INVERSE_SFT: c_int = 14;
pub const AUDGLB_PWRDN_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const AUDGLB_PWRDN_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const AUDGLB_PWRDN_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG11
pub const VOW_ON_PERIODIC_MODE_SFT: c_int = 15;
pub const VOW_ON_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const VOW_ON_PERIODIC_INVERSE_SFT: c_int = 14;
pub const VOW_ON_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const VOW_ON_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const VOW_ON_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG12
pub const DMIC_ON_PERIODIC_MODE_SFT: c_int = 15;
pub const DMIC_ON_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const DMIC_ON_PERIODIC_INVERSE_SFT: c_int = 14;
pub const DMIC_ON_PERIODIC_INVERSE_MASK: c_uint = 0x1;

pub const DMIC_ON_PERIODIC_ON_CYCLE_SFT: c_int = 0;
pub const DMIC_ON_PERIODIC_ON_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG13
pub const PDN_VOW_F32K_CK_SFT: c_int = 15;
pub const PDN_VOW_F32K_CK_MASK: c_uint = 0x1;

pub const AUDPREAMPLON_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDPREAMPLON_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG14
pub const VOW_SNRDET_PERIODIC_CFG_SFT: c_int = 15;
pub const VOW_SNRDET_PERIODIC_CFG_MASK: c_uint = 0x1;

pub const AUDPREAMPLDCPRECHARGE_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDPREAMPLDCPRECHARGE_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG15
pub const AUDADCLPWRUP_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDADCLPWRUP_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG16
pub const AUDGLBVOWLPWEN_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDGLBVOWLPWEN_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG17
pub const AUDDIGMICEN_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDDIGMICEN_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG18
pub const AUDPWDBMICBIAS0_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDPWDBMICBIAS0_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG19
pub const AUDPWDBMICBIAS1_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDPWDBMICBIAS1_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG20
pub const CLKSQ_EN_VOW_PERIODIC_MODE_SFT: c_int = 15;
pub const CLKSQ_EN_VOW_PERIODIC_MODE_MASK: c_uint = 0x1;

pub const XO_VOW_CK_EN_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const XO_VOW_CK_EN_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG21
pub const AUDGLB_PWRDN_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const AUDGLB_PWRDN_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG22
pub const VOW_ON_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const VOW_ON_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_CFG23
pub const DMIC_ON_PERIODIC_OFF_CYCLE_SFT: c_int = 0;
pub const DMIC_ON_PERIODIC_OFF_CYCLE_MASK: c_uint = 0x3fff;

// MT6358_AFE_VOW_PERIODIC_MON0
pub const VOW_PERIODIC_MON_SFT: c_int = 0;
pub const VOW_PERIODIC_MON_MASK: c_uint = 0xffff;

// MT6358_AFE_VOW_PERIODIC_MON1
pub const VOW_PERIODIC_COUNT_MON_SFT: c_int = 0;
pub const VOW_PERIODIC_COUNT_MON_MASK: c_uint = 0xffff;

// MT6358_AUDENC_DSN_ID
pub const AUDENC_ANA_ID_SFT: c_int = 0;
pub const AUDENC_ANA_ID_MASK: c_uint = 0xff;

pub const AUDENC_DIG_ID_SFT: c_int = 8;
pub const AUDENC_DIG_ID_MASK: c_uint = 0xff;

// MT6358_AUDENC_DSN_REV0
pub const AUDENC_ANA_MINOR_REV_SFT: c_int = 0;
pub const AUDENC_ANA_MINOR_REV_MASK: c_uint = 0xf;

pub const AUDENC_ANA_MAJOR_REV_SFT: c_int = 4;
pub const AUDENC_ANA_MAJOR_REV_MASK: c_uint = 0xf;

pub const AUDENC_DIG_MINOR_REV_SFT: c_int = 8;
pub const AUDENC_DIG_MINOR_REV_MASK: c_uint = 0xf;

pub const AUDENC_DIG_MAJOR_REV_SFT: c_int = 12;
pub const AUDENC_DIG_MAJOR_REV_MASK: c_uint = 0xf;

// MT6358_AUDENC_DSN_DBI
pub const AUDENC_DSN_CBS_SFT: c_int = 0;
pub const AUDENC_DSN_CBS_MASK: c_uint = 0x3;

pub const AUDENC_DSN_BIX_SFT: c_int = 2;
pub const AUDENC_DSN_BIX_MASK: c_uint = 0x3;

pub const AUDENC_DSN_ESP_SFT: c_int = 8;
pub const AUDENC_DSN_ESP_MASK: c_uint = 0xff;

// MT6358_AUDENC_DSN_FPI
pub const AUDENC_DSN_FPI_SFT: c_int = 0;
pub const AUDENC_DSN_FPI_MASK: c_uint = 0xff;

// MT6358_AUDENC_ANA_CON0
pub const RG_AUDPREAMPLON_SFT: c_int = 0;
pub const RG_AUDPREAMPLON_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLDCCEN_SFT: c_int = 1;
pub const RG_AUDPREAMPLDCCEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLDCPRECHARGE_SFT: c_int = 2;
pub const RG_AUDPREAMPLDCPRECHARGE_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLPGATEST_SFT: c_int = 3;
pub const RG_AUDPREAMPLPGATEST_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLVSCALE_SFT: c_int = 4;
pub const RG_AUDPREAMPLVSCALE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPLINPUTSEL_SFT: c_int = 6;
pub const RG_AUDPREAMPLINPUTSEL_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPLGAIN_SFT: c_int = 8;
pub const RG_AUDPREAMPLGAIN_MASK: c_uint = 0x7;

pub const RG_AUDADCLPWRUP_SFT: c_int = 12;
pub const RG_AUDADCLPWRUP_MASK: c_uint = 0x1;

pub const RG_AUDADCLINPUTSEL_SFT: c_int = 13;
pub const RG_AUDADCLINPUTSEL_MASK: c_uint = 0x3;

// MT6358_AUDENC_ANA_CON1
pub const RG_AUDPREAMPRON_SFT: c_int = 0;
pub const RG_AUDPREAMPRON_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRDCCEN_SFT: c_int = 1;
pub const RG_AUDPREAMPRDCCEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRDCPRECHARGE_SFT: c_int = 2;
pub const RG_AUDPREAMPRDCPRECHARGE_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRPGATEST_SFT: c_int = 3;
pub const RG_AUDPREAMPRPGATEST_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRVSCALE_SFT: c_int = 4;
pub const RG_AUDPREAMPRVSCALE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPRINPUTSEL_SFT: c_int = 6;
pub const RG_AUDPREAMPRINPUTSEL_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPRGAIN_SFT: c_int = 8;
pub const RG_AUDPREAMPRGAIN_MASK: c_uint = 0x7;

pub const RG_AUDIO_VOW_EN_SFT: c_int = 11;
pub const RG_AUDIO_VOW_EN_MASK: c_uint = 0x1;

pub const RG_AUDADCRPWRUP_SFT: c_int = 12;
pub const RG_AUDADCRPWRUP_MASK: c_uint = 0x1;

pub const RG_AUDADCRINPUTSEL_SFT: c_int = 13;
pub const RG_AUDADCRINPUTSEL_MASK: c_uint = 0x3;

pub const RG_CLKSQ_EN_VOW_SFT: c_int = 15;
pub const RG_CLKSQ_EN_VOW_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON2
pub const RG_AUDULHALFBIAS_SFT: c_int = 0;
pub const RG_AUDULHALFBIAS_MASK: c_uint = 0x1;

pub const RG_AUDGLBVOWLPWEN_SFT: c_int = 1;
pub const RG_AUDGLBVOWLPWEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLPEN_SFT: c_int = 2;
pub const RG_AUDPREAMPLPEN_MASK: c_uint = 0x1;

pub const RG_AUDADC1STSTAGELPEN_SFT: c_int = 3;
pub const RG_AUDADC1STSTAGELPEN_MASK: c_uint = 0x1;

pub const RG_AUDADC2NDSTAGELPEN_SFT: c_int = 4;
pub const RG_AUDADC2NDSTAGELPEN_MASK: c_uint = 0x1;

pub const RG_AUDADCFLASHLPEN_SFT: c_int = 5;
pub const RG_AUDADCFLASHLPEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPIDDTEST_SFT: c_int = 6;
pub const RG_AUDPREAMPIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADC1STSTAGEIDDTEST_SFT: c_int = 8;
pub const RG_AUDADC1STSTAGEIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADC2NDSTAGEIDDTEST_SFT: c_int = 10;
pub const RG_AUDADC2NDSTAGEIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADCREFBUFIDDTEST_SFT: c_int = 12;
pub const RG_AUDADCREFBUFIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADCFLASHIDDTEST_SFT: c_int = 14;
pub const RG_AUDADCFLASHIDDTEST_MASK: c_uint = 0x3;

// MT6358_AUDENC_ANA_CON3
pub const RG_AUDADCDAC0P25FS_SFT: c_int = 0;
pub const RG_AUDADCDAC0P25FS_MASK: c_uint = 0x1;

pub const RG_AUDADCCLKSEL_SFT: c_int = 1;
pub const RG_AUDADCCLKSEL_MASK: c_uint = 0x1;

pub const RG_AUDADCCLKSOURCE_SFT: c_int = 2;
pub const RG_AUDADCCLKSOURCE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPAAFEN_SFT: c_int = 8;
pub const RG_AUDPREAMPAAFEN_MASK: c_uint = 0x1;

pub const RG_DCCVCMBUFLPMODSEL_SFT: c_int = 9;
pub const RG_DCCVCMBUFLPMODSEL_MASK: c_uint = 0x1;

pub const RG_DCCVCMBUFLPSWEN_SFT: c_int = 10;
pub const RG_DCCVCMBUFLPSWEN_MASK: c_uint = 0x1;

pub const RG_CMSTBENH_SFT: c_int = 11;
pub const RG_CMSTBENH_MASK: c_uint = 0x1;

pub const RG_PGABODYSW_SFT: c_int = 12;
pub const RG_PGABODYSW_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON4
pub const RG_AUDADC1STSTAGESDENB_SFT: c_int = 0;
pub const RG_AUDADC1STSTAGESDENB_MASK: c_uint = 0x1;

pub const RG_AUDADC2NDSTAGERESET_SFT: c_int = 1;
pub const RG_AUDADC2NDSTAGERESET_MASK: c_uint = 0x1;

pub const RG_AUDADC3RDSTAGERESET_SFT: c_int = 2;
pub const RG_AUDADC3RDSTAGERESET_MASK: c_uint = 0x1;

pub const RG_AUDADCFSRESET_SFT: c_int = 3;
pub const RG_AUDADCFSRESET_MASK: c_uint = 0x1;

pub const RG_AUDADCWIDECM_SFT: c_int = 4;
pub const RG_AUDADCWIDECM_MASK: c_uint = 0x1;

pub const RG_AUDADCNOPATEST_SFT: c_int = 5;
pub const RG_AUDADCNOPATEST_MASK: c_uint = 0x1;

pub const RG_AUDADCBYPASS_SFT: c_int = 6;
pub const RG_AUDADCBYPASS_MASK: c_uint = 0x1;

pub const RG_AUDADCFFBYPASS_SFT: c_int = 7;
pub const RG_AUDADCFFBYPASS_MASK: c_uint = 0x1;

pub const RG_AUDADCDACFBCURRENT_SFT: c_int = 8;
pub const RG_AUDADCDACFBCURRENT_MASK: c_uint = 0x1;

pub const RG_AUDADCDACIDDTEST_SFT: c_int = 9;
pub const RG_AUDADCDACIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADCDACNRZ_SFT: c_int = 11;
pub const RG_AUDADCDACNRZ_MASK: c_uint = 0x1;

pub const RG_AUDADCNODEM_SFT: c_int = 12;
pub const RG_AUDADCNODEM_MASK: c_uint = 0x1;

pub const RG_AUDADCDACTEST_SFT: c_int = 13;
pub const RG_AUDADCDACTEST_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON5
pub const RG_AUDRCTUNEL_SFT: c_int = 0;
pub const RG_AUDRCTUNEL_MASK: c_uint = 0x1f;

pub const RG_AUDRCTUNELSEL_SFT: c_int = 5;
pub const RG_AUDRCTUNELSEL_MASK: c_uint = 0x1;

pub const RG_AUDRCTUNER_SFT: c_int = 8;
pub const RG_AUDRCTUNER_MASK: c_uint = 0x1f;

pub const RG_AUDRCTUNERSEL_SFT: c_int = 13;
pub const RG_AUDRCTUNERSEL_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON6
pub const RG_CLKSQ_EN_SFT: c_int = 0;
pub const RG_CLKSQ_EN_MASK: c_uint = 0x1;

pub const RG_CLKSQ_IN_SEL_TEST_SFT: c_int = 1;
pub const RG_CLKSQ_IN_SEL_TEST_MASK: c_uint = 0x1;

pub const RG_CM_REFGENSEL_SFT: c_int = 2;
pub const RG_CM_REFGENSEL_MASK: c_uint = 0x1;

pub const RG_AUDSPARE_SFT: c_int = 4;
pub const RG_AUDSPARE_MASK: c_uint = 0xf;

pub const RG_AUDENCSPARE_SFT: c_int = 8;
pub const RG_AUDENCSPARE_MASK: c_uint = 0x3f;

// MT6358_AUDENC_ANA_CON7
pub const RG_AUDENCSPARE2_SFT: c_int = 0;
pub const RG_AUDENCSPARE2_MASK: c_uint = 0xff;

// MT6358_AUDENC_ANA_CON8
pub const RG_AUDDIGMICEN_SFT: c_int = 0;
pub const RG_AUDDIGMICEN_MASK: c_uint = 0x1;

pub const RG_AUDDIGMICBIAS_SFT: c_int = 1;
pub const RG_AUDDIGMICBIAS_MASK: c_uint = 0x3;

pub const RG_DMICHPCLKEN_SFT: c_int = 3;
pub const RG_DMICHPCLKEN_MASK: c_uint = 0x1;

pub const RG_AUDDIGMICPDUTY_SFT: c_int = 4;
pub const RG_AUDDIGMICPDUTY_MASK: c_uint = 0x3;

pub const RG_AUDDIGMICNDUTY_SFT: c_int = 6;
pub const RG_AUDDIGMICNDUTY_MASK: c_uint = 0x3;

pub const RG_DMICMONEN_SFT: c_int = 8;
pub const RG_DMICMONEN_MASK: c_uint = 0x1;

pub const RG_DMICMONSEL_SFT: c_int = 9;
pub const RG_DMICMONSEL_MASK: c_uint = 0x7;

pub const RG_AUDSPAREVMIC_SFT: c_int = 12;
pub const RG_AUDSPAREVMIC_MASK: c_uint = 0xf;

// MT6358_AUDENC_ANA_CON9
pub const RG_AUDPWDBMICBIAS0_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS0_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0BYPASSEN_SFT: c_int = 1;
pub const RG_AUDMICBIAS0BYPASSEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0LOWPEN_SFT: c_int = 2;
pub const RG_AUDMICBIAS0LOWPEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0VREF_SFT: c_int = 4;
pub const RG_AUDMICBIAS0VREF_MASK: c_uint = 0x7;

pub const RG_AUDMICBIAS0DCSW0P1EN_SFT: c_int = 8;
pub const RG_AUDMICBIAS0DCSW0P1EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW0P2EN_SFT: c_int = 9;
pub const RG_AUDMICBIAS0DCSW0P2EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW0NEN_SFT: c_int = 10;
pub const RG_AUDMICBIAS0DCSW0NEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW2P1EN_SFT: c_int = 12;
pub const RG_AUDMICBIAS0DCSW2P1EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW2P2EN_SFT: c_int = 13;
pub const RG_AUDMICBIAS0DCSW2P2EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW2NEN_SFT: c_int = 14;
pub const RG_AUDMICBIAS0DCSW2NEN_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON10
pub const RG_AUDPWDBMICBIAS1_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS1_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1BYPASSEN_SFT: c_int = 1;
pub const RG_AUDMICBIAS1BYPASSEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1LOWPEN_SFT: c_int = 2;
pub const RG_AUDMICBIAS1LOWPEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1VREF_SFT: c_int = 4;
pub const RG_AUDMICBIAS1VREF_MASK: c_uint = 0x7;

pub const RG_AUDMICBIAS1DCSW1PEN_SFT: c_int = 8;
pub const RG_AUDMICBIAS1DCSW1PEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1DCSW1NEN_SFT: c_int = 9;
pub const RG_AUDMICBIAS1DCSW1NEN_MASK: c_uint = 0x1;

pub const RG_BANDGAPGEN_SFT: c_int = 12;
pub const RG_BANDGAPGEN_MASK: c_uint = 0x1;

pub const RG_MTEST_EN_SFT: c_int = 13;
pub const RG_MTEST_EN_MASK: c_uint = 0x1;

pub const RG_MTEST_SEL_SFT: c_int = 14;
pub const RG_MTEST_SEL_MASK: c_uint = 0x1;

pub const RG_MTEST_CURRENT_SFT: c_int = 15;
pub const RG_MTEST_CURRENT_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON11
pub const RG_AUDACCDETMICBIAS0PULLLOW_SFT: c_int = 0;
pub const RG_AUDACCDETMICBIAS0PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS1PULLLOW_SFT: c_int = 1;
pub const RG_AUDACCDETMICBIAS1PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVIN1PULLLOW_SFT: c_int = 2;
pub const RG_AUDACCDETVIN1PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVTHACAL_SFT: c_int = 4;
pub const RG_AUDACCDETVTHACAL_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVTHBCAL_SFT: c_int = 5;
pub const RG_AUDACCDETVTHBCAL_MASK: c_uint = 0x1;

pub const RG_AUDACCDETTVDET_SFT: c_int = 6;
pub const RG_AUDACCDETTVDET_MASK: c_uint = 0x1;

pub const RG_ACCDETSEL_SFT: c_int = 7;
pub const RG_ACCDETSEL_MASK: c_uint = 0x1;

pub const RG_SWBUFMODSEL_SFT: c_int = 8;
pub const RG_SWBUFMODSEL_MASK: c_uint = 0x1;

pub const RG_SWBUFSWEN_SFT: c_int = 9;
pub const RG_SWBUFSWEN_MASK: c_uint = 0x1;

pub const RG_EINTCOMPVTH_SFT: c_int = 10;
pub const RG_EINTCOMPVTH_MASK: c_uint = 0x1;

pub const RG_EINTCONFIGACCDET_SFT: c_int = 11;
pub const RG_EINTCONFIGACCDET_MASK: c_uint = 0x1;

pub const RG_EINTHIRENB_SFT: c_int = 12;
pub const RG_EINTHIRENB_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXRESBYPASS_SFT: c_int = 13;
pub const RG_ACCDET2AUXRESBYPASS_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXBUFFERBYPASS_SFT: c_int = 14;
pub const RG_ACCDET2AUXBUFFERBYPASS_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXSWEN_SFT: c_int = 15;
pub const RG_ACCDET2AUXSWEN_MASK: c_uint = 0x1;

// MT6358_AUDENC_ANA_CON12
pub const RGS_AUDRCTUNELREAD_SFT: c_int = 0;
pub const RGS_AUDRCTUNELREAD_MASK: c_uint = 0x1f;

pub const RGS_AUDRCTUNERREAD_SFT: c_int = 8;
pub const RGS_AUDRCTUNERREAD_MASK: c_uint = 0x1f;

// MT6358_AUDDEC_DSN_ID
pub const AUDDEC_ANA_ID_SFT: c_int = 0;
pub const AUDDEC_ANA_ID_MASK: c_uint = 0xff;

pub const AUDDEC_DIG_ID_SFT: c_int = 8;
pub const AUDDEC_DIG_ID_MASK: c_uint = 0xff;

// MT6358_AUDDEC_DSN_REV0
pub const AUDDEC_ANA_MINOR_REV_SFT: c_int = 0;
pub const AUDDEC_ANA_MINOR_REV_MASK: c_uint = 0xf;

pub const AUDDEC_ANA_MAJOR_REV_SFT: c_int = 4;
pub const AUDDEC_ANA_MAJOR_REV_MASK: c_uint = 0xf;

pub const AUDDEC_DIG_MINOR_REV_SFT: c_int = 8;
pub const AUDDEC_DIG_MINOR_REV_MASK: c_uint = 0xf;

pub const AUDDEC_DIG_MAJOR_REV_SFT: c_int = 12;
pub const AUDDEC_DIG_MAJOR_REV_MASK: c_uint = 0xf;

// MT6358_AUDDEC_DSN_DBI
pub const AUDDEC_DSN_CBS_SFT: c_int = 0;
pub const AUDDEC_DSN_CBS_MASK: c_uint = 0x3;

pub const AUDDEC_DSN_BIX_SFT: c_int = 2;
pub const AUDDEC_DSN_BIX_MASK: c_uint = 0x3;

pub const AUDDEC_DSN_ESP_SFT: c_int = 8;
pub const AUDDEC_DSN_ESP_MASK: c_uint = 0xff;

// MT6358_AUDDEC_DSN_FPI
pub const AUDDEC_DSN_FPI_SFT: c_int = 0;
pub const AUDDEC_DSN_FPI_MASK: c_uint = 0xff;

// MT6358_AUDDEC_ANA_CON0
pub const RG_AUDDACLPWRUP_VAUDP15_SFT: c_int = 0;
pub const RG_AUDDACLPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDDACRPWRUP_VAUDP15_SFT: c_int = 1;
pub const RG_AUDDACRPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUD_DAC_PWR_UP_VA28_SFT: c_int = 2;
pub const RG_AUD_DAC_PWR_UP_VA28_MASK: c_uint = 0x1;

pub const RG_AUD_DAC_PWL_UP_VA28_SFT: c_int = 3;
pub const RG_AUD_DAC_PWL_UP_VA28_MASK: c_uint = 0x1;

pub const RG_AUDHPLPWRUP_VAUDP15_SFT: c_int = 4;
pub const RG_AUDHPLPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPRPWRUP_VAUDP15_SFT: c_int = 5;
pub const RG_AUDHPRPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPLPWRUP_IBIAS_VAUDP15_SFT: c_int = 6;
pub const RG_AUDHPLPWRUP_IBIAS_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPRPWRUP_IBIAS_VAUDP15_SFT: c_int = 7;
pub const RG_AUDHPRPWRUP_IBIAS_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPLMUXINPUTSEL_VAUDP15_SFT: c_int = 8;
pub const RG_AUDHPLMUXINPUTSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDHPRMUXINPUTSEL_VAUDP15_SFT: c_int = 10;
pub const RG_AUDHPRMUXINPUTSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDHPLSCDISABLE_VAUDP15_SFT: c_int = 12;
pub const RG_AUDHPLSCDISABLE_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPRSCDISABLE_VAUDP15_SFT: c_int = 13;
pub const RG_AUDHPRSCDISABLE_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPLBSCCURRENT_VAUDP15_SFT: c_int = 14;
pub const RG_AUDHPLBSCCURRENT_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPRBSCCURRENT_VAUDP15_SFT: c_int = 15;
pub const RG_AUDHPRBSCCURRENT_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON1
pub const RG_AUDHPLOUTPWRUP_VAUDP15_SFT: c_int = 0;
pub const RG_AUDHPLOUTPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPROUTPWRUP_VAUDP15_SFT: c_int = 1;
pub const RG_AUDHPROUTPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPLOUTAUXPWRUP_VAUDP15_SFT: c_int = 2;
pub const RG_AUDHPLOUTAUXPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPROUTAUXPWRUP_VAUDP15_SFT: c_int = 3;
pub const RG_AUDHPROUTAUXPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPLAUXFBRSW_EN_VAUDP15_SFT: c_int = 4;
pub const RG_HPLAUXFBRSW_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPRAUXFBRSW_EN_VAUDP15_SFT: c_int = 5;
pub const RG_HPRAUXFBRSW_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPLSHORT2HPLAUX_EN_VAUDP15_SFT: c_int = 6;
pub const RG_HPLSHORT2HPLAUX_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPRSHORT2HPRAUX_EN_VAUDP15_SFT: c_int = 7;
pub const RG_HPRSHORT2HPRAUX_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPLOUTSTGCTRL_VAUDP15_SFT: c_int = 8;
pub const RG_HPLOUTSTGCTRL_VAUDP15_MASK: c_uint = 0x7;

pub const RG_HPROUTSTGCTRL_VAUDP15_SFT: c_int = 11;
pub const RG_HPROUTSTGCTRL_VAUDP15_MASK: c_uint = 0x7;

// MT6358_AUDDEC_ANA_CON2
pub const RG_HPLOUTPUTSTBENH_VAUDP15_SFT: c_int = 0;
pub const RG_HPLOUTPUTSTBENH_VAUDP15_MASK: c_uint = 0x7;

pub const RG_HPROUTPUTSTBENH_VAUDP15_SFT: c_int = 4;
pub const RG_HPROUTPUTSTBENH_VAUDP15_MASK: c_uint = 0x7;

pub const RG_AUDHPSTARTUP_VAUDP15_SFT: c_int = 13;
pub const RG_AUDHPSTARTUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDREFN_DERES_EN_VAUDP15_SFT: c_int = 14;
pub const RG_AUDREFN_DERES_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPPSHORT2VCM_VAUDP15_SFT: c_int = 15;
pub const RG_HPPSHORT2VCM_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON3
pub const RG_HPINPUTSTBENH_VAUDP15_SFT: c_int = 13;
pub const RG_HPINPUTSTBENH_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPINPUTRESET0_VAUDP15_SFT: c_int = 14;
pub const RG_HPINPUTRESET0_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HPOUTPUTRESET0_VAUDP15_SFT: c_int = 15;
pub const RG_HPOUTPUTRESET0_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON4
pub const RG_ABIDEC_RSVD0_VAUDP28_SFT: c_int = 0;
pub const RG_ABIDEC_RSVD0_VAUDP28_MASK: c_uint = 0xff;

// MT6358_AUDDEC_ANA_CON5
pub const RG_AUDHPDECMGAINADJ_VAUDP15_SFT: c_int = 0;
pub const RG_AUDHPDECMGAINADJ_VAUDP15_MASK: c_uint = 0x7;

pub const RG_AUDHPDEDMGAINADJ_VAUDP15_SFT: c_int = 4;
pub const RG_AUDHPDEDMGAINADJ_VAUDP15_MASK: c_uint = 0x7;

// MT6358_AUDDEC_ANA_CON6
pub const RG_AUDHSPWRUP_VAUDP15_SFT: c_int = 0;
pub const RG_AUDHSPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHSPWRUP_IBIAS_VAUDP15_SFT: c_int = 1;
pub const RG_AUDHSPWRUP_IBIAS_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHSMUXINPUTSEL_VAUDP15_SFT: c_int = 2;
pub const RG_AUDHSMUXINPUTSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDHSSCDISABLE_VAUDP15_SFT: c_int = 4;
pub const RG_AUDHSSCDISABLE_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHSBSCCURRENT_VAUDP15_SFT: c_int = 5;
pub const RG_AUDHSBSCCURRENT_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHSSTARTUP_VAUDP15_SFT: c_int = 6;
pub const RG_AUDHSSTARTUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HSOUTPUTSTBENH_VAUDP15_SFT: c_int = 7;
pub const RG_HSOUTPUTSTBENH_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HSINPUTSTBENH_VAUDP15_SFT: c_int = 8;
pub const RG_HSINPUTSTBENH_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HSINPUTRESET0_VAUDP15_SFT: c_int = 9;
pub const RG_HSINPUTRESET0_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HSOUTPUTRESET0_VAUDP15_SFT: c_int = 10;
pub const RG_HSOUTPUTRESET0_VAUDP15_MASK: c_uint = 0x1;

pub const RG_HSOUT_SHORTVCM_VAUDP15_SFT: c_int = 11;
pub const RG_HSOUT_SHORTVCM_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON7
pub const RG_AUDLOLPWRUP_VAUDP15_SFT: c_int = 0;
pub const RG_AUDLOLPWRUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDLOLPWRUP_IBIAS_VAUDP15_SFT: c_int = 1;
pub const RG_AUDLOLPWRUP_IBIAS_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDLOLMUXINPUTSEL_VAUDP15_SFT: c_int = 2;
pub const RG_AUDLOLMUXINPUTSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDLOLSCDISABLE_VAUDP15_SFT: c_int = 4;
pub const RG_AUDLOLSCDISABLE_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDLOLBSCCURRENT_VAUDP15_SFT: c_int = 5;
pub const RG_AUDLOLBSCCURRENT_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDLOSTARTUP_VAUDP15_SFT: c_int = 6;
pub const RG_AUDLOSTARTUP_VAUDP15_MASK: c_uint = 0x1;

pub const RG_LOINPUTSTBENH_VAUDP15_SFT: c_int = 7;
pub const RG_LOINPUTSTBENH_VAUDP15_MASK: c_uint = 0x1;

pub const RG_LOOUTPUTSTBENH_VAUDP15_SFT: c_int = 8;
pub const RG_LOOUTPUTSTBENH_VAUDP15_MASK: c_uint = 0x1;

pub const RG_LOINPUTRESET0_VAUDP15_SFT: c_int = 9;
pub const RG_LOINPUTRESET0_VAUDP15_MASK: c_uint = 0x1;

pub const RG_LOOUTPUTRESET0_VAUDP15_SFT: c_int = 10;
pub const RG_LOOUTPUTRESET0_VAUDP15_MASK: c_uint = 0x1;

pub const RG_LOOUT_SHORTVCM_VAUDP15_SFT: c_int = 11;
pub const RG_LOOUT_SHORTVCM_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON8
pub const RG_AUDTRIMBUF_INPUTMUXSEL_VAUDP15_SFT: c_int = 0;
pub const RG_AUDTRIMBUF_INPUTMUXSEL_VAUDP15_MASK: c_uint = 0xf;

pub const RG_AUDTRIMBUF_GAINSEL_VAUDP15_SFT: c_int = 4;
pub const RG_AUDTRIMBUF_GAINSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDTRIMBUF_EN_VAUDP15_SFT: c_int = 6;
pub const RG_AUDTRIMBUF_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDHPSPKDET_INPUTMUXSEL_VAUDP15_SFT: c_int = 8;
pub const RG_AUDHPSPKDET_INPUTMUXSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDHPSPKDET_OUTPUTMUXSEL_VAUDP15_SFT: c_int = 10;
pub const RG_AUDHPSPKDET_OUTPUTMUXSEL_VAUDP15_MASK: c_uint = 0x3;

pub const RG_AUDHPSPKDET_EN_VAUDP15_SFT: c_int = 12;
pub const RG_AUDHPSPKDET_EN_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON9
pub const RG_ABIDEC_RSVD0_VA28_SFT: c_int = 0;
pub const RG_ABIDEC_RSVD0_VA28_MASK: c_uint = 0xff;

pub const RG_ABIDEC_RSVD0_VAUDP15_SFT: c_int = 8;
pub const RG_ABIDEC_RSVD0_VAUDP15_MASK: c_uint = 0xff;

// MT6358_AUDDEC_ANA_CON10
pub const RG_ABIDEC_RSVD1_VAUDP15_SFT: c_int = 0;
pub const RG_ABIDEC_RSVD1_VAUDP15_MASK: c_uint = 0xff;

pub const RG_ABIDEC_RSVD2_VAUDP15_SFT: c_int = 8;
pub const RG_ABIDEC_RSVD2_VAUDP15_MASK: c_uint = 0xff;

// MT6358_AUDDEC_ANA_CON11
pub const RG_AUDZCDMUXSEL_VAUDP15_SFT: c_int = 0;
pub const RG_AUDZCDMUXSEL_VAUDP15_MASK: c_uint = 0x7;

pub const RG_AUDZCDCLKSEL_VAUDP15_SFT: c_int = 3;
pub const RG_AUDZCDCLKSEL_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDBIASADJ_0_VAUDP15_SFT: c_int = 7;
pub const RG_AUDBIASADJ_0_VAUDP15_MASK: c_uint = 0x1ff;

// MT6358_AUDDEC_ANA_CON12
pub const RG_AUDBIASADJ_1_VAUDP15_SFT: c_int = 0;
pub const RG_AUDBIASADJ_1_VAUDP15_MASK: c_uint = 0xff;

pub const RG_AUDIBIASPWRDN_VAUDP15_SFT: c_int = 8;
pub const RG_AUDIBIASPWRDN_VAUDP15_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON13
pub const RG_RSTB_DECODER_VA28_SFT: c_int = 0;
pub const RG_RSTB_DECODER_VA28_MASK: c_uint = 0x1;

pub const RG_SEL_DECODER_96K_VA28_SFT: c_int = 1;
pub const RG_SEL_DECODER_96K_VA28_MASK: c_uint = 0x1;

pub const RG_SEL_DELAY_VCORE_SFT: c_int = 2;
pub const RG_SEL_DELAY_VCORE_MASK: c_uint = 0x1;

pub const RG_AUDGLB_PWRDN_VA28_SFT: c_int = 4;
pub const RG_AUDGLB_PWRDN_VA28_MASK: c_uint = 0x1;

pub const RG_RSTB_ENCODER_VA28_SFT: c_int = 5;
pub const RG_RSTB_ENCODER_VA28_MASK: c_uint = 0x1;

pub const RG_SEL_ENCODER_96K_VA28_SFT: c_int = 6;
pub const RG_SEL_ENCODER_96K_VA28_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON14
pub const RG_HCLDO_EN_VA18_SFT: c_int = 0;
pub const RG_HCLDO_EN_VA18_MASK: c_uint = 0x1;

pub const RG_HCLDO_PDDIS_EN_VA18_SFT: c_int = 1;
pub const RG_HCLDO_PDDIS_EN_VA18_MASK: c_uint = 0x1;

pub const RG_HCLDO_REMOTE_SENSE_VA18_SFT: c_int = 2;
pub const RG_HCLDO_REMOTE_SENSE_VA18_MASK: c_uint = 0x1;

pub const RG_LCLDO_EN_VA18_SFT: c_int = 4;
pub const RG_LCLDO_EN_VA18_MASK: c_uint = 0x1;

pub const RG_LCLDO_PDDIS_EN_VA18_SFT: c_int = 5;
pub const RG_LCLDO_PDDIS_EN_VA18_MASK: c_uint = 0x1;

pub const RG_LCLDO_REMOTE_SENSE_VA18_SFT: c_int = 6;
pub const RG_LCLDO_REMOTE_SENSE_VA18_MASK: c_uint = 0x1;

pub const RG_LCLDO_ENC_EN_VA28_SFT: c_int = 8;
pub const RG_LCLDO_ENC_EN_VA28_MASK: c_uint = 0x1;

pub const RG_LCLDO_ENC_PDDIS_EN_VA28_SFT: c_int = 9;
pub const RG_LCLDO_ENC_PDDIS_EN_VA28_MASK: c_uint = 0x1;

pub const RG_LCLDO_ENC_REMOTE_SENSE_VA28_SFT: c_int = 10;
pub const RG_LCLDO_ENC_REMOTE_SENSE_VA28_MASK: c_uint = 0x1;

pub const RG_VA33REFGEN_EN_VA18_SFT: c_int = 12;
pub const RG_VA33REFGEN_EN_VA18_MASK: c_uint = 0x1;

pub const RG_VA28REFGEN_EN_VA28_SFT: c_int = 13;
pub const RG_VA28REFGEN_EN_VA28_MASK: c_uint = 0x1;

pub const RG_HCLDO_VOSEL_VA18_SFT: c_int = 14;
pub const RG_HCLDO_VOSEL_VA18_MASK: c_uint = 0x1;

pub const RG_LCLDO_VOSEL_VA18_SFT: c_int = 15;
pub const RG_LCLDO_VOSEL_VA18_MASK: c_uint = 0x1;

// MT6358_AUDDEC_ANA_CON15
pub const RG_NVREG_EN_VAUDP15_SFT: c_int = 0;
pub const RG_NVREG_EN_VAUDP15_MASK: c_uint = 0x1;

pub const RG_NVREG_PULL0V_VAUDP15_SFT: c_int = 1;
pub const RG_NVREG_PULL0V_VAUDP15_MASK: c_uint = 0x1;

pub const RG_AUDPMU_RSD0_VAUDP15_SFT: c_int = 4;
pub const RG_AUDPMU_RSD0_VAUDP15_MASK: c_uint = 0xf;

pub const RG_AUDPMU_RSD0_VA18_SFT: c_int = 8;
pub const RG_AUDPMU_RSD0_VA18_MASK: c_uint = 0xf;

pub const RG_AUDPMU_RSD0_VA28_SFT: c_int = 12;
pub const RG_AUDPMU_RSD0_VA28_MASK: c_uint = 0xf;

// MT6358_ZCD_CON0
pub const RG_AUDZCDENABLE_SFT: c_int = 0;
pub const RG_AUDZCDENABLE_MASK: c_uint = 0x1;

pub const RG_AUDZCDGAINSTEPTIME_SFT: c_int = 1;
pub const RG_AUDZCDGAINSTEPTIME_MASK: c_uint = 0x7;

pub const RG_AUDZCDGAINSTEPSIZE_SFT: c_int = 4;
pub const RG_AUDZCDGAINSTEPSIZE_MASK: c_uint = 0x3;

pub const RG_AUDZCDTIMEOUTMODESEL_SFT: c_int = 6;
pub const RG_AUDZCDTIMEOUTMODESEL_MASK: c_uint = 0x1;

// MT6358_ZCD_CON1
pub const RG_AUDLOLGAIN_SFT: c_int = 0;
pub const RG_AUDLOLGAIN_MASK: c_uint = 0x1f;

pub const RG_AUDLORGAIN_SFT: c_int = 7;
pub const RG_AUDLORGAIN_MASK: c_uint = 0x1f;

// MT6358_ZCD_CON2
pub const RG_AUDHPLGAIN_SFT: c_int = 0;
pub const RG_AUDHPLGAIN_MASK: c_uint = 0x1f;

pub const RG_AUDHPRGAIN_SFT: c_int = 7;
pub const RG_AUDHPRGAIN_MASK: c_uint = 0x1f;

// MT6358_ZCD_CON3
pub const RG_AUDHSGAIN_SFT: c_int = 0;
pub const RG_AUDHSGAIN_MASK: c_uint = 0x1f;

// MT6358_ZCD_CON4
pub const RG_AUDIVLGAIN_SFT: c_int = 0;
pub const RG_AUDIVLGAIN_MASK: c_uint = 0x7;

pub const RG_AUDIVRGAIN_SFT: c_int = 8;
pub const RG_AUDIVRGAIN_MASK: c_uint = 0x7;

// MT6358_ZCD_CON5
pub const RG_AUDINTGAIN1_SFT: c_int = 0;
pub const RG_AUDINTGAIN1_MASK: c_uint = 0x3f;

pub const RG_AUDINTGAIN2_SFT: c_int = 8;
pub const RG_AUDINTGAIN2_MASK: c_uint = 0x3f;

// audio register
pub const MT6358_DRV_CON3: c_uint = 0x3c;
pub const MT6358_GPIO_DIR0: c_uint = 0x88;
pub const MT6358_GPIO_MODE2: c_uint = 0xd8	/* mosi */;
pub const MT6358_GPIO_MODE2_SET: c_uint = 0xda;
pub const MT6358_GPIO_MODE2_CLR: c_uint = 0xdc;
pub const MT6358_GPIO_MODE3: c_uint = 0xde	/* miso */;
pub const MT6358_GPIO_MODE3_SET: c_uint = 0xe0;
pub const MT6358_GPIO_MODE3_CLR: c_uint = 0xe2;
pub const MT6358_TOP_CKPDN_CON0: c_uint = 0x10c;
pub const MT6358_TOP_CKPDN_CON0_SET: c_uint = 0x10e;
pub const MT6358_TOP_CKPDN_CON0_CLR: c_uint = 0x110;
pub const MT6358_TOP_CKHWEN_CON0: c_uint = 0x12a;
pub const MT6358_TOP_CKHWEN_CON0_SET: c_uint = 0x12c;
pub const MT6358_TOP_CKHWEN_CON0_CLR: c_uint = 0x12e;
pub const MT6358_OTP_CON0: c_uint = 0x38a;
pub const MT6358_OTP_CON8: c_uint = 0x39a;
pub const MT6358_OTP_CON11: c_uint = 0x3a0;
pub const MT6358_OTP_CON12: c_uint = 0x3a2;
pub const MT6358_OTP_CON13: c_uint = 0x3a4;
pub const MT6358_DCXO_CW13: c_uint = 0x7aa;
pub const MT6358_DCXO_CW14: c_uint = 0x7ac;
pub const MT6358_AUXADC_CON10: c_uint = 0x11a0;
// audio register
pub const MT6358_AUD_TOP_ID: c_uint = 0x2200;
pub const MT6358_AUD_TOP_REV0: c_uint = 0x2202;
pub const MT6358_AUD_TOP_DBI: c_uint = 0x2204;
pub const MT6358_AUD_TOP_DXI: c_uint = 0x2206;
pub const MT6358_AUD_TOP_CKPDN_TPM0: c_uint = 0x2208;
pub const MT6358_AUD_TOP_CKPDN_TPM1: c_uint = 0x220a;
pub const MT6358_AUD_TOP_CKPDN_CON0: c_uint = 0x220c;
pub const MT6358_AUD_TOP_CKPDN_CON0_SET: c_uint = 0x220e;
pub const MT6358_AUD_TOP_CKPDN_CON0_CLR: c_uint = 0x2210;
pub const MT6358_AUD_TOP_CKSEL_CON0: c_uint = 0x2212;
pub const MT6358_AUD_TOP_CKSEL_CON0_SET: c_uint = 0x2214;
pub const MT6358_AUD_TOP_CKSEL_CON0_CLR: c_uint = 0x2216;
pub const MT6358_AUD_TOP_CKTST_CON0: c_uint = 0x2218;
pub const MT6358_AUD_TOP_CLK_HWEN_CON0: c_uint = 0x221a;
pub const MT6358_AUD_TOP_CLK_HWEN_CON0_SET: c_uint = 0x221c;
pub const MT6358_AUD_TOP_CLK_HWEN_CON0_CLR: c_uint = 0x221e;
pub const MT6358_AUD_TOP_RST_CON0: c_uint = 0x2220;
pub const MT6358_AUD_TOP_RST_CON0_SET: c_uint = 0x2222;
pub const MT6358_AUD_TOP_RST_CON0_CLR: c_uint = 0x2224;
pub const MT6358_AUD_TOP_RST_BANK_CON0: c_uint = 0x2226;
pub const MT6358_AUD_TOP_INT_CON0: c_uint = 0x2228;
pub const MT6358_AUD_TOP_INT_CON0_SET: c_uint = 0x222a;
pub const MT6358_AUD_TOP_INT_CON0_CLR: c_uint = 0x222c;
pub const MT6358_AUD_TOP_INT_MASK_CON0: c_uint = 0x222e;
pub const MT6358_AUD_TOP_INT_MASK_CON0_SET: c_uint = 0x2230;
pub const MT6358_AUD_TOP_INT_MASK_CON0_CLR: c_uint = 0x2232;
pub const MT6358_AUD_TOP_INT_STATUS0: c_uint = 0x2234;
pub const MT6358_AUD_TOP_INT_RAW_STATUS0: c_uint = 0x2236;
pub const MT6358_AUD_TOP_INT_MISC_CON0: c_uint = 0x2238;
pub const MT6358_AUDNCP_CLKDIV_CON0: c_uint = 0x223a;
pub const MT6358_AUDNCP_CLKDIV_CON1: c_uint = 0x223c;
pub const MT6358_AUDNCP_CLKDIV_CON2: c_uint = 0x223e;
pub const MT6358_AUDNCP_CLKDIV_CON3: c_uint = 0x2240;
pub const MT6358_AUDNCP_CLKDIV_CON4: c_uint = 0x2242;
pub const MT6358_AUD_TOP_MON_CON0: c_uint = 0x2244;
pub const MT6358_AUDIO_DIG_DSN_ID: c_uint = 0x2280;
pub const MT6358_AUDIO_DIG_DSN_REV0: c_uint = 0x2282;
pub const MT6358_AUDIO_DIG_DSN_DBI: c_uint = 0x2284;
pub const MT6358_AUDIO_DIG_DSN_DXI: c_uint = 0x2286;
pub const MT6358_AFE_UL_DL_CON0: c_uint = 0x2288;
pub const MT6358_AFE_DL_SRC2_CON0_L: c_uint = 0x228a;
pub const MT6358_AFE_UL_SRC_CON0_H: c_uint = 0x228c;
pub const MT6358_AFE_UL_SRC_CON0_L: c_uint = 0x228e;
pub const MT6358_AFE_TOP_CON0: c_uint = 0x2290;
pub const MT6358_AUDIO_TOP_CON0: c_uint = 0x2292;
pub const MT6358_AFE_MON_DEBUG0: c_uint = 0x2294;
pub const MT6358_AFUNC_AUD_CON0: c_uint = 0x2296;
pub const MT6358_AFUNC_AUD_CON1: c_uint = 0x2298;
pub const MT6358_AFUNC_AUD_CON2: c_uint = 0x229a;
pub const MT6358_AFUNC_AUD_CON3: c_uint = 0x229c;
pub const MT6358_AFUNC_AUD_CON4: c_uint = 0x229e;
pub const MT6358_AFUNC_AUD_CON5: c_uint = 0x22a0;
pub const MT6358_AFUNC_AUD_CON6: c_uint = 0x22a2;
pub const MT6358_AFUNC_AUD_MON0: c_uint = 0x22a4;
pub const MT6358_AUDRC_TUNE_MON0: c_uint = 0x22a6;
pub const MT6358_AFE_ADDA_MTKAIF_FIFO_CFG0: c_uint = 0x22a8;
pub const MT6358_AFE_ADDA_MTKAIF_FIFO_LOG_MON1: c_uint = 0x22aa;
pub const MT6358_AFE_ADDA_MTKAIF_MON0: c_uint = 0x22ac;
pub const MT6358_AFE_ADDA_MTKAIF_MON1: c_uint = 0x22ae;
pub const MT6358_AFE_ADDA_MTKAIF_MON2: c_uint = 0x22b0;
pub const MT6358_AFE_ADDA_MTKAIF_MON3: c_uint = 0x22b2;
pub const MT6358_AFE_ADDA_MTKAIF_CFG0: c_uint = 0x22b4;
pub const MT6358_AFE_ADDA_MTKAIF_RX_CFG0: c_uint = 0x22b6;
pub const MT6358_AFE_ADDA_MTKAIF_RX_CFG1: c_uint = 0x22b8;
pub const MT6358_AFE_ADDA_MTKAIF_RX_CFG2: c_uint = 0x22ba;
pub const MT6358_AFE_ADDA_MTKAIF_RX_CFG3: c_uint = 0x22bc;
pub const MT6358_AFE_ADDA_MTKAIF_TX_CFG1: c_uint = 0x22be;
pub const MT6358_AFE_SGEN_CFG0: c_uint = 0x22c0;
pub const MT6358_AFE_SGEN_CFG1: c_uint = 0x22c2;
pub const MT6358_AFE_ADC_ASYNC_FIFO_CFG: c_uint = 0x22c4;
pub const MT6358_AFE_DCCLK_CFG0: c_uint = 0x22c6;
pub const MT6358_AFE_DCCLK_CFG1: c_uint = 0x22c8;
pub const MT6358_AUDIO_DIG_CFG: c_uint = 0x22ca;
pub const MT6358_AFE_AUD_PAD_TOP: c_uint = 0x22cc;
pub const MT6358_AFE_AUD_PAD_TOP_MON: c_uint = 0x22ce;
pub const MT6358_AFE_AUD_PAD_TOP_MON1: c_uint = 0x22d0;
pub const MT6358_AFE_DL_NLE_CFG: c_uint = 0x22d2;
pub const MT6358_AFE_DL_NLE_MON: c_uint = 0x22d4;
pub const MT6358_AFE_CG_EN_MON: c_uint = 0x22d6;
pub const MT6358_AUDIO_DIG_2ND_DSN_ID: c_uint = 0x2300;
pub const MT6358_AUDIO_DIG_2ND_DSN_REV0: c_uint = 0x2302;
pub const MT6358_AUDIO_DIG_2ND_DSN_DBI: c_uint = 0x2304;
pub const MT6358_AUDIO_DIG_2ND_DSN_DXI: c_uint = 0x2306;
pub const MT6358_AFE_PMIC_NEWIF_CFG3: c_uint = 0x2308;
pub const MT6358_AFE_VOW_TOP: c_uint = 0x230a;
pub const MT6358_AFE_VOW_CFG0: c_uint = 0x230c;
pub const MT6358_AFE_VOW_CFG1: c_uint = 0x230e;
pub const MT6358_AFE_VOW_CFG2: c_uint = 0x2310;
pub const MT6358_AFE_VOW_CFG3: c_uint = 0x2312;
pub const MT6358_AFE_VOW_CFG4: c_uint = 0x2314;
pub const MT6358_AFE_VOW_CFG5: c_uint = 0x2316;
pub const MT6358_AFE_VOW_CFG6: c_uint = 0x2318;
pub const MT6358_AFE_VOW_MON0: c_uint = 0x231a;
pub const MT6358_AFE_VOW_MON1: c_uint = 0x231c;
pub const MT6358_AFE_VOW_MON2: c_uint = 0x231e;
pub const MT6358_AFE_VOW_MON3: c_uint = 0x2320;
pub const MT6358_AFE_VOW_MON4: c_uint = 0x2322;
pub const MT6358_AFE_VOW_MON5: c_uint = 0x2324;
pub const MT6358_AFE_VOW_SN_INI_CFG: c_uint = 0x2326;
pub const MT6358_AFE_VOW_TGEN_CFG0: c_uint = 0x2328;
pub const MT6358_AFE_VOW_POSDIV_CFG0: c_uint = 0x232a;
pub const MT6358_AFE_VOW_HPF_CFG0: c_uint = 0x232c;
pub const MT6358_AFE_VOW_PERIODIC_CFG0: c_uint = 0x232e;
pub const MT6358_AFE_VOW_PERIODIC_CFG1: c_uint = 0x2330;
pub const MT6358_AFE_VOW_PERIODIC_CFG2: c_uint = 0x2332;
pub const MT6358_AFE_VOW_PERIODIC_CFG3: c_uint = 0x2334;
pub const MT6358_AFE_VOW_PERIODIC_CFG4: c_uint = 0x2336;
pub const MT6358_AFE_VOW_PERIODIC_CFG5: c_uint = 0x2338;
pub const MT6358_AFE_VOW_PERIODIC_CFG6: c_uint = 0x233a;
pub const MT6358_AFE_VOW_PERIODIC_CFG7: c_uint = 0x233c;
pub const MT6358_AFE_VOW_PERIODIC_CFG8: c_uint = 0x233e;
pub const MT6358_AFE_VOW_PERIODIC_CFG9: c_uint = 0x2340;
pub const MT6358_AFE_VOW_PERIODIC_CFG10: c_uint = 0x2342;
pub const MT6358_AFE_VOW_PERIODIC_CFG11: c_uint = 0x2344;
pub const MT6358_AFE_VOW_PERIODIC_CFG12: c_uint = 0x2346;
pub const MT6358_AFE_VOW_PERIODIC_CFG13: c_uint = 0x2348;
pub const MT6358_AFE_VOW_PERIODIC_CFG14: c_uint = 0x234a;
pub const MT6358_AFE_VOW_PERIODIC_CFG15: c_uint = 0x234c;
pub const MT6358_AFE_VOW_PERIODIC_CFG16: c_uint = 0x234e;
pub const MT6358_AFE_VOW_PERIODIC_CFG17: c_uint = 0x2350;
pub const MT6358_AFE_VOW_PERIODIC_CFG18: c_uint = 0x2352;
pub const MT6358_AFE_VOW_PERIODIC_CFG19: c_uint = 0x2354;
pub const MT6358_AFE_VOW_PERIODIC_CFG20: c_uint = 0x2356;
pub const MT6358_AFE_VOW_PERIODIC_CFG21: c_uint = 0x2358;
pub const MT6358_AFE_VOW_PERIODIC_CFG22: c_uint = 0x235a;
pub const MT6358_AFE_VOW_PERIODIC_CFG23: c_uint = 0x235c;
pub const MT6358_AFE_VOW_PERIODIC_MON0: c_uint = 0x235e;
pub const MT6358_AFE_VOW_PERIODIC_MON1: c_uint = 0x2360;
pub const MT6358_AUDENC_DSN_ID: c_uint = 0x2380;
pub const MT6358_AUDENC_DSN_REV0: c_uint = 0x2382;
pub const MT6358_AUDENC_DSN_DBI: c_uint = 0x2384;
pub const MT6358_AUDENC_DSN_FPI: c_uint = 0x2386;
pub const MT6358_AUDENC_ANA_CON0: c_uint = 0x2388;
pub const MT6358_AUDENC_ANA_CON1: c_uint = 0x238a;
pub const MT6358_AUDENC_ANA_CON2: c_uint = 0x238c;
pub const MT6358_AUDENC_ANA_CON3: c_uint = 0x238e;
pub const MT6358_AUDENC_ANA_CON4: c_uint = 0x2390;
pub const MT6358_AUDENC_ANA_CON5: c_uint = 0x2392;
pub const MT6358_AUDENC_ANA_CON6: c_uint = 0x2394;
pub const MT6358_AUDENC_ANA_CON7: c_uint = 0x2396;
pub const MT6358_AUDENC_ANA_CON8: c_uint = 0x2398;
pub const MT6358_AUDENC_ANA_CON9: c_uint = 0x239a;
pub const MT6358_AUDENC_ANA_CON10: c_uint = 0x239c;
pub const MT6358_AUDENC_ANA_CON11: c_uint = 0x239e;
pub const MT6358_AUDENC_ANA_CON12: c_uint = 0x23a0;
pub const MT6358_AUDDEC_DSN_ID: c_uint = 0x2400;
pub const MT6358_AUDDEC_DSN_REV0: c_uint = 0x2402;
pub const MT6358_AUDDEC_DSN_DBI: c_uint = 0x2404;
pub const MT6358_AUDDEC_DSN_FPI: c_uint = 0x2406;
pub const MT6358_AUDDEC_ANA_CON0: c_uint = 0x2408;
pub const MT6358_AUDDEC_ANA_CON1: c_uint = 0x240a;
pub const MT6358_AUDDEC_ANA_CON2: c_uint = 0x240c;
pub const MT6358_AUDDEC_ANA_CON3: c_uint = 0x240e;
pub const MT6358_AUDDEC_ANA_CON4: c_uint = 0x2410;
pub const MT6358_AUDDEC_ANA_CON5: c_uint = 0x2412;
pub const MT6358_AUDDEC_ANA_CON6: c_uint = 0x2414;
pub const MT6358_AUDDEC_ANA_CON7: c_uint = 0x2416;
pub const MT6358_AUDDEC_ANA_CON8: c_uint = 0x2418;
pub const MT6358_AUDDEC_ANA_CON9: c_uint = 0x241a;
pub const MT6358_AUDDEC_ANA_CON10: c_uint = 0x241c;
pub const MT6358_AUDDEC_ANA_CON11: c_uint = 0x241e;
pub const MT6358_AUDDEC_ANA_CON12: c_uint = 0x2420;
pub const MT6358_AUDDEC_ANA_CON13: c_uint = 0x2422;
pub const MT6358_AUDDEC_ANA_CON14: c_uint = 0x2424;
pub const MT6358_AUDDEC_ANA_CON15: c_uint = 0x2426;
pub const MT6358_AUDDEC_ELR_NUM: c_uint = 0x2428;
pub const MT6358_AUDDEC_ELR_0: c_uint = 0x242a;
pub const MT6358_AUDZCD_DSN_ID: c_uint = 0x2480;
pub const MT6358_AUDZCD_DSN_REV0: c_uint = 0x2482;
pub const MT6358_AUDZCD_DSN_DBI: c_uint = 0x2484;
pub const MT6358_AUDZCD_DSN_FPI: c_uint = 0x2486;
pub const MT6358_ZCD_CON0: c_uint = 0x2488;
pub const MT6358_ZCD_CON1: c_uint = 0x248a;
pub const MT6358_ZCD_CON2: c_uint = 0x248c;
pub const MT6358_ZCD_CON3: c_uint = 0x248e;
pub const MT6358_ZCD_CON4: c_uint = 0x2490;
pub const MT6358_ZCD_CON5: c_uint = 0x2492;
pub const MT6358_ACCDET_CON13: c_uint = 0x2522;

// set only during init
