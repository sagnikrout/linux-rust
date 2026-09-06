//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/mt6357.h
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
// mt6357.h  --  mt6357 ALSA SoC audio codec driver
//
// Copyright (c) 2024 Baylibre
// Author: Nicolas Belin <nbelin@baylibre.com>
//

// Reg bit defines
// MT6357_GPIO_DIR0

pub const MT6357_GPIO8_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO9_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO10_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO11_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO12_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO13_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO14_DIR_INPUT: c_int = 0;

pub const MT6357_GPIO15_DIR_INPUT: c_int = 0;

// MT6357_GPIO_MODE2

pub const MT6357_GPIO8_MODE_GPIO: c_int = 0;

pub const MT6357_GPIO9_MODE_GPIO: c_int = 0;

pub const MT6357_GPIO10_MODE_GPIO: c_int = 0;

pub const MT6357_GPIO11_MODE_GPIO: c_int = 0;
// MT6357_GPIO_MODE2_SET

// MT6357_GPIO_MODE2_CLR

// MT6357_GPIO_MODE3

pub const MT6357_GPIO12_MODE_GPIO: c_int = 0;

pub const MT6357_GPIO13_MODE_GPIO: c_int = 0;

pub const MT6357_GPIO14_MODE_GPIO: c_int = 0;

pub const MT6357_GPIO15_MODE_GPIO: c_int = 0;
// MT6357_GPIO_MODE3_SET

// MT6357_GPIO_MODE3_CLR

// MT6357_DCXO_CW14
pub const MT6357_XO_AUDIO_EN_M_SFT: c_int = 13;

pub const MT6357_XO_AUDIO_EN_M_DISABLE: c_int = 0;
// MT6357_AUD_TOP_CKPDN_CON0
pub const MT6357_AUDNCP_CK_PDN_SFT: c_int = 6;
pub const MT6357_ZCD13M_CK_PDN_SFT: c_int = 5;
pub const MT6357_AUDIF_CK_PDN_SFT: c_int = 2;
pub const MT6357_AUD_CK_PDN_SFT: c_int = 1;
// MT6357_AUDNCP_CLKDIV_CON0

// MT6357_AUDNCP_CLKDIV_CON1

// MT6357_AUDNCP_CLKDIV_CON3

pub const MT6357_DIVCKS_PWD_NCP_ENABLE: c_int = 0;
// MT6357_AUDNCP_CLKDIV_CON4

pub const MT6357_DIVCKS_PWD_NCP_ST_50US: c_int = 0;
pub const MT6357_DIVCKS_PWD_NCP_ST_100US: c_int = 1;
pub const MT6357_DIVCKS_PWD_NCP_ST_150US: c_int = 2;
pub const MT6357_DIVCKS_PWD_NCP_ST_200US: c_int = 3;
// MT6357_AFE_UL_DL_CON0
pub const MT6357_AFE_UL_LR_SWAP_SFT: c_int = 15;
pub const MT6357_AFE_ON_SFT: c_int = 0;
// MT6357_AFE_DL_SRC2_CON0_L
pub const MT6357_DL_2_SRC_ON_TMP_CTL_PRE_SFT: c_int = 0;
// MT6357_AFE_UL_SRC_CON0_H

pub const MT6357_C_TWO_DIGITAL_MIC_DISABLE: c_int = 0;
// MT6357_AFE_UL_SRC_CON0_L

pub const MT6357_UL_SDM_3_LEVEL_DESELECT: c_int = 0;

pub const MT6357_UL_SRC_DISABLE: c_int = 0;
// MT6357_AFE_TOP_CON0
pub const MT6357_UL_SINE_ON_SFT: c_int = 1;

pub const MT6357_DL_SINE_ON_SFT: c_int = 0;

// MT6357_AUDIO_TOP_CON0
pub const MT6357_PDN_LPBK_CTL_SFT: c_int = 15;
pub const MT6357_PDN_AFE_CTL_SFT: c_int = 7;
pub const MT6357_PDN_DAC_CTL_SFT: c_int = 6;
pub const MT6357_PDN_ADC_CTL_SFT: c_int = 5;
pub const MT6357_PDN_I2S_DL_CTL_SFT: c_int = 3;
pub const MT6357_PWR_CLK_DIS_CTL_SFT: c_int = 2;
pub const MT6357_PDN_AFE_TESTMODEL_CTL_SFT: c_int = 1;
pub const MT6357_PDN_RESERVED_SFT: c_int = 0;
// MT6357_AFUNC_AUD_CON0

pub const MT6357_CCI_AUD_ANACK_NORMAL: c_int = 0;
pub const MT6357_CCI_AUDIO_FIFO_WPTR_SFT: c_int = 12;

pub const MT6357_CCI_SCRAMBLER_CG_DISABLE: c_int = 0;

pub const MT6357_CCI_LCK_INV_IN_PHASE: c_int = 0;

pub const MT6357_CCI_RAND_DISABLE: c_int = 0;

pub const MT6357_CCI_SPLT_SCRMB_CLK_OFF: c_int = 0;

pub const MT6357_CCI_SPLT_SCRMB_OFF: c_int = 0;

pub const MT6357_CCI_AUD_IDAC_TEST_EN_NORMAL_PATH: c_int = 0;

pub const MT6357_CCI_ZERO_PADDING_ENABLE: c_int = 0;

pub const MT6357_CCI_AUD_SPLIT_TEST_EN_NORMAL_PATH: c_int = 0;

pub const MT6357_CCI_AUD_SDM_MUTE_L_NO_CTL: c_int = 0;

pub const MT6357_CCI_AUD_SDM_MUTE_R_NO_CTL: c_int = 0;

pub const MT6357_CCI_AUD_SDM_7BIT_FROM_SPLITTER1: c_int = 0;

pub const MT6357_CCI_SCRAMBLER_DISABLE: c_int = 0;
// MT6357_AFUNC_AUD_CON2

pub const MT6357_CCI_AUDIO_FIFO_DISABLE: c_int = 0;

pub const MT6357_CCI_ACD_MODE_TEST_PATH: c_int = 0;

pub const MT6357_CCI_AFIFO_CLK_PWDB_DOWN: c_int = 0;

pub const MT6357_CCI_ACD_FUNC_RSTB_RESET: c_int = 0;
// MT6357_AFE_ADDA_MTKAIF_CFG0

pub const MT6357_ADDA_MTKAIF_LPBK_DISABLE: c_int = 0;
// MT6357_AFE_SGEN_CFG0
pub const MT6357_SGEN_DAC_EN_CTL_SFT: c_int = 7;

pub const MT6357_SGEN_MUTE_SW_CTL_SFT: c_int = 6;
pub const MT6357_SGEN_MUTE_SW_DISABLE: c_int = 0;
// MT6357_AFE_DCCLK_CFG0

pub const MT6357_DCCLK_DIV_SFT: c_int = 5;

pub const MT6357_DCCLK_OUTPUT: c_int = 0;

pub const MT6357_DCCLK_GEN_OFF: c_int = 0;
// MT6357_AFE_DCCLK_CFG1

// MT6357_AFE_AUD_PAD_TOP

pub const MT6357_AUD_PAD_TX_FIFO_LPBK_DISABLE: c_int = 0;
// MT6357_AUDENC_ANA_CON0

pub const MT6357_AUDADCLINPUTSEL_IDLE: c_int = 0;
pub const MT6357_AUDADCLPWRUP_SFT: c_int = 12;

pub const MT6357_AUDADCLPWRDOWN: c_int = 0;
pub const MT6357_AUDPREAMPLGAIN_SFT: c_int = 8;

pub const MT6357_AUDPREAMPLGAIN_MAX: c_int = 4;
pub const MT6357_AUDPREAMPLINPUTSEL_SFT: c_int = 6;

pub const MT6357_AUDPREAMPLDCPRECHARGE_DISABLE: c_int = 0;

pub const MT6357_AUDPREAMPLDCCEN_AC: c_int = 0;

pub const MT6357_AUDPREAMPLON_DISABLE: c_int = 0;
// MT6357_AUDENC_ANA_CON1

pub const MT6357_AUDADCRINPUTSEL_IDLE: c_int = 0;
pub const MT6357_AUDADCRPWRUP_SFT: c_int = 12;

pub const MT6357_AUDADCRPWRDOWN: c_int = 0;
pub const MT6357_AUDPREAMPRGAIN_SFT: c_int = 8;

pub const MT6357_AUDPREAMPRGAIN_MAX: c_int = 4;
pub const MT6357_AUDPREAMPRINPUTSEL_SFT: c_int = 6;

pub const MT6357_AUDPREAMPRDCPRECHARGE_DISABLE: c_int = 0;

pub const MT6357_AUDPREAMPRDCCEN_AC: c_int = 0;

pub const MT6357_AUDPREAMPRON_DISABLE: c_int = 0;
// MT6357_AUDENC_ANA_CON6
pub const MT6357_CLKSQ_EN_SFT: c_int = 0;
// MT6357_AUDENC_ANA_CON7

pub const MT6357_AUDDIGMICBIAS_OFF: c_int = 0;

pub const MT6357_AUDDIGMICEN_DISABLE: c_int = 0;
// MT6357_AUDENC_ANA_CON8

pub const MT6357_AUD_MICBIAS0_DCSW2N_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS0_DCSW2P2_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS0_DCSW2P1_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS0_DCSWN_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS0_DCSW0P2_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS0_DCSW0P1_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS0_VREF_SFT: c_int = 4;
pub const MT6357_AUD_MICBIAS0_PWD_SFT: c_int = 0;

pub const MT6357_AUD_MICBIAS0_DC_DISABLE_ALL: c_int = 0;
// MT6357_AUDENC_ANA_CON9

pub const MT6357_AUD_MICBIAS1_DCSW1P_DISABLE: c_int = 0;

pub const MT6357_AUD_MICBIAS1_VREF_SFT: c_int = 4;
pub const MT6357_AUD_MICBIAS1_PWD_SFT: c_int = 0;
// MT6357_AUDDEC_ANA_CON0

pub const MT6357_AUD_HPR_SC_VAUDP15_ENABLE: c_int = 0;

pub const MT6357_AUD_HPL_SC_VAUDP15_ENABLE: c_int = 0;

pub const MT6357_AUD_HPR_MUX_INPUT_VAUDP15_SFT: c_int = 10;

pub const MT6357_AUD_HPL_MUX_INPUT_VAUDP15_SFT: c_int = 8;

pub const MT6357_AUD_HPR_BIAS_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_HPL_BIAS_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_HPR_PWRUP_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_HPL_PWRUP_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_DACL_PWRUP_VA28_DISABLE: c_int = 0;

pub const MT6357_AUD_DACR_PWRUP_VA28_DISABLE: c_int = 0;

pub const MT6357_AUD_DACR_PWRUP_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_DACL_PWRUP_VAUDP15_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON1

pub const MT6357_HPROUT_STG_CTRL_VAUDP15_SFT: c_int = 12;

pub const MT6357_HPLOUT_STG_CTRL_VAUDP15_SFT: c_int = 8;
pub const MT6357_HPLOUT_STG_CTRL_VAUDP15_MAX: c_int = 7;

pub const MT6357_HPR_SHORT2HPR_AUX_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPL_SHORT2HPR_AUX_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPR_AUX_FBRSW_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPL_AUX_FBRSW_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPROUT_AUX_PWRUP_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPLOUT_AUX_PWRUP_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPROUT_PWRUP_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPLOUT_PWRUP_VAUDP15_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON2

pub const MT6357_HPP_SHORT_2VCM_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_REFN_DERES_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_HPROUT_STB_ENH_VAUDP15_OPEN: c_int = 0;

pub const MT6357_HPLOUT_STB_ENH_VAUDP15_OPEN: c_int = 0;

// MT6357_AUDDEC_ANA_CON3

pub const MT6357_AUD_HSOUT_STB_ENH_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_HS_SC_VAUDP15_ENABLE: c_int = 0;

pub const MT6357_AUD_HS_MUX_INPUT_VAUDP15_SFT: c_int = 2;

pub const MT6357_AUD_HS_PWRUP_BIAS_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_HS_PWRUP_VAUDP15_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON4

pub const MT6357_AUD_LOLOUT_STB_ENH_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_LOL_SC_VAUDP15_ENABLE: c_int = 0;

pub const MT6357_AUD_LOL_MUX_INPUT_VAUDP15_SFT: c_int = 2;

pub const MT6357_AUD_LOL_PWRUP_BIAS_VAUDP15_DISABLE: c_int = 0;

pub const MT6357_AUD_LOL_PWRUP_VAUDP15_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON6

pub const MT6357_HP_AUX_LOOP_GAIN_SFT: c_int = 12;
pub const MT6357_HP_AUX_LOOP_GAIN_MAX: c_uint = 0x0f;

pub const MT6357_HPR_AUX_CMFB_LOOP_DISABLE: c_int = 0;

pub const MT6357_HPL_AUX_CMFB_LOOP_DISABLE: c_int = 0;

pub const MT6357_HPRL_MAIN_CMFB_LOOP_DISABLE: c_int = 0;

pub const MT6357_HP_CMFB_RST_RESET: c_int = 0;

pub const MT6357_DAC_LOW_NOISE_MODE_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON7
pub const MT6357_HP_IVBUF_DEGAIN_SFT: c_int = 2;
pub const MT6357_HP_IVBUF_DEGAIN_MAX: c_int = 1;
// MT6357_AUDDEC_ANA_CON10

pub const MT6357_AUD_IBIAS_PWRDN_VAUDP15_ENABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON11

pub const MT6357_RSTB_ENCODER_VA28_DISABLE: c_int = 0;
pub const MT6357_AUDGLB_PWRDN_VA28_SFT: c_int = 4;

pub const MT6357_RSTB_DECODER_VA28_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON12

pub const MT6357_VA28REFGEN_EN_VA28_DISABLE: c_int = 0;

pub const MT6357_VA33REFGEN_EN_VA18_DISABLE: c_int = 0;

pub const MT6357_LCLDO_ENC_REMOTE_SENSE_VA28_DISABLE: c_int = 0;

pub const MT6357_LCLDO_ENC_EN_VA28_DISABLE: c_int = 0;

pub const MT6357_LCLDO_REMOTE_SENSE_VA18_DISABLE: c_int = 0;

pub const MT6357_LCLDO_EN_VA18_DISABLE: c_int = 0;

pub const MT6357_HCLDO_REMOTE_SENSE_VA18_DISABLE: c_int = 0;

pub const MT6357_HCLDO_EN_VA18_DISABLE: c_int = 0;
// MT6357_AUDDEC_ANA_CON13

pub const MT6357_NVREG_EN_VAUDP15_DISABLE: c_int = 0;
// MT6357_AUDDEC_ELR_0

pub const MT6357_AUD_HP_TRIM_EN_VAUDP15_DISABLE: c_int = 0;
// MT6357_ZCD_CON1

pub const MT6357_AUD_LOL_GAIN_SFT: c_int = 0;

pub const MT6357_AUD_LOR_GAIN_SFT: c_int = 7;
pub const MT6357_AUD_LO_GAIN_MAX: c_uint = 0x12;
// MT6357_ZCD_CON2

pub const MT6357_AUD_HPL_GAIN_SFT: c_int = 0;

pub const MT6357_AUD_HPR_GAIN_SFT: c_int = 7;
pub const MT6357_AUD_HP_GAIN_MAX: c_uint = 0x12;
// MT6357_ZCD_CON3

pub const MT6357_AUD_HS_GAIN_SFT: c_int = 0;
pub const MT6357_AUD_HS_GAIN_MAX: c_uint = 0x12;
// Registers list
// gpio direction
pub const MT6357_GPIO_DIR0: c_uint = 0x0088;
// mosi
pub const MT6357_GPIO_MODE2: c_uint = 0x00B6;
pub const MT6357_GPIO_MODE2_SET: c_uint = 0x00B8;
pub const MT6357_GPIO_MODE2_CLR: c_uint = 0x00BA;
// miso
pub const MT6357_GPIO_MODE3: c_uint = 0x00BC;
pub const MT6357_GPIO_MODE3_SET: c_uint = 0x00BE;
pub const MT6357_GPIO_MODE3_CLR: c_uint = 0x00C0;
pub const MT6357_DCXO_CW14: c_uint = 0x07AC;
pub const MT6357_AUD_TOP_CKPDN_CON0: c_uint = 0x208C;
pub const MT6357_AUDNCP_CLKDIV_CON0: c_uint = 0x20B4;
pub const MT6357_AUDNCP_CLKDIV_CON1: c_uint = 0x20B6;
pub const MT6357_AUDNCP_CLKDIV_CON2: c_uint = 0x20B8;
pub const MT6357_AUDNCP_CLKDIV_CON3: c_uint = 0x20BA;
pub const MT6357_AUDNCP_CLKDIV_CON4: c_uint = 0x20BC;
pub const MT6357_AFE_UL_DL_CON0: c_uint = 0x2108;
pub const MT6357_AFE_DL_SRC2_CON0_L: c_uint = 0x210A;
pub const MT6357_AFE_UL_SRC_CON0_H: c_uint = 0x210C;
pub const MT6357_AFE_UL_SRC_CON0_L: c_uint = 0x210E;
pub const MT6357_AFE_TOP_CON0: c_uint = 0x2110;
pub const MT6357_AUDIO_TOP_CON0: c_uint = 0x2112;
pub const MT6357_AFUNC_AUD_CON0: c_uint = 0x2116;
pub const MT6357_AFUNC_AUD_CON2: c_uint = 0x211A;
pub const MT6357_AFE_ADDA_MTKAIF_CFG0: c_uint = 0x2134;
pub const MT6357_AFE_SGEN_CFG0: c_uint = 0x2140;
pub const MT6357_AFE_DCCLK_CFG0: c_uint = 0x2146;
pub const MT6357_AFE_DCCLK_CFG1: c_uint = 0x2148;
pub const MT6357_AFE_AUD_PAD_TOP: c_uint = 0x214C;
pub const MT6357_AUDENC_ANA_CON0: c_uint = 0x2188;
pub const MT6357_AUDENC_ANA_CON1: c_uint = 0x218A;
pub const MT6357_AUDENC_ANA_CON6: c_uint = 0x2194;
pub const MT6357_AUDENC_ANA_CON7: c_uint = 0x2196;
pub const MT6357_AUDENC_ANA_CON8: c_uint = 0x2198;
pub const MT6357_AUDENC_ANA_CON9: c_uint = 0x219A;
pub const MT6357_AUDDEC_ANA_CON0: c_uint = 0x2208;
pub const MT6357_AUDDEC_ANA_CON1: c_uint = 0x220A;
pub const MT6357_AUDDEC_ANA_CON2: c_uint = 0x220C;
pub const MT6357_AUDDEC_ANA_CON3: c_uint = 0x220E;
pub const MT6357_AUDDEC_ANA_CON4: c_uint = 0x2210;
pub const MT6357_AUDDEC_ANA_CON6: c_uint = 0x2214;
pub const MT6357_AUDDEC_ANA_CON7: c_uint = 0x2216;
pub const MT6357_AUDDEC_ANA_CON10: c_uint = 0x221C;
pub const MT6357_AUDDEC_ANA_CON11: c_uint = 0x221E;
pub const MT6357_AUDDEC_ANA_CON12: c_uint = 0x2220;
pub const MT6357_AUDDEC_ANA_CON13: c_uint = 0x2222;
pub const MT6357_AUDDEC_ELR_0: c_uint = 0x2226;
pub const MT6357_ZCD_CON1: c_uint = 0x228A;
pub const MT6357_ZCD_CON2: c_uint = 0x228C;
pub const MT6357_ZCD_CON3: c_uint = 0x228E;

pub const MT6357_DL_GAIN_REG_LEFT_MASK: c_uint = 0x001f;
pub const MT6357_DL_GAIN_REG_LEFT_SHIFT: c_int = 0;
pub const MT6357_DL_GAIN_REG_RIGHT_MASK: c_uint = 0x0f80;
pub const MT6357_DL_GAIN_REG_RIGHT_SHIFT: c_int = 7;
pub const MT6357_DL_GAIN_REG_MASK: c_uint = 0x0f9f;

// codec private structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6357_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pull_down_needed: bool,
    pub hp_channel_number: c_int,
}
