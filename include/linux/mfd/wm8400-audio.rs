//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8400-audio.h
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
// wm8400 private definitions for audio
//
// Copyright 2008 Wolfson Microelectronics plc
//

//
// R2 (0x02) - Power Management (1)
//
pub const WM8400_CODEC_ENA: c_uint = 0x8000  /* CODEC_ENA */;
pub const WM8400_CODEC_ENA_MASK: c_uint = 0x8000  /* CODEC_ENA */;

pub const WM8400_SYSCLK_ENA: c_uint = 0x4000  /* SYSCLK_ENA */;
pub const WM8400_SYSCLK_ENA_MASK: c_uint = 0x4000  /* SYSCLK_ENA */;

pub const WM8400_SPK_MIX_ENA: c_uint = 0x2000  /* SPK_MIX_ENA */;
pub const WM8400_SPK_MIX_ENA_MASK: c_uint = 0x2000  /* SPK_MIX_ENA */;

pub const WM8400_SPK_ENA: c_uint = 0x1000  /* SPK_ENA */;
pub const WM8400_SPK_ENA_MASK: c_uint = 0x1000  /* SPK_ENA */;

pub const WM8400_OUT3_ENA: c_uint = 0x0800  /* OUT3_ENA */;
pub const WM8400_OUT3_ENA_MASK: c_uint = 0x0800  /* OUT3_ENA */;

pub const WM8400_OUT4_ENA: c_uint = 0x0400  /* OUT4_ENA */;
pub const WM8400_OUT4_ENA_MASK: c_uint = 0x0400  /* OUT4_ENA */;

pub const WM8400_LOUT_ENA: c_uint = 0x0200  /* LOUT_ENA */;
pub const WM8400_LOUT_ENA_MASK: c_uint = 0x0200  /* LOUT_ENA */;

pub const WM8400_ROUT_ENA: c_uint = 0x0100  /* ROUT_ENA */;
pub const WM8400_ROUT_ENA_MASK: c_uint = 0x0100  /* ROUT_ENA */;

pub const WM8400_MIC1BIAS_ENA: c_uint = 0x0010  /* MIC1BIAS_ENA */;
pub const WM8400_MIC1BIAS_ENA_MASK: c_uint = 0x0010  /* MIC1BIAS_ENA */;

pub const WM8400_VMID_MODE_MASK: c_uint = 0x0006  /* VMID_MODE - [2:1] */;

pub const WM8400_VREF_ENA: c_uint = 0x0001  /* VREF_ENA */;
pub const WM8400_VREF_ENA_MASK: c_uint = 0x0001  /* VREF_ENA */;

//
// R3 (0x03) - Power Management (2)
//
pub const WM8400_FLL_ENA: c_uint = 0x8000  /* FLL_ENA */;
pub const WM8400_FLL_ENA_MASK: c_uint = 0x8000  /* FLL_ENA */;

pub const WM8400_TSHUT_ENA: c_uint = 0x4000  /* TSHUT_ENA */;
pub const WM8400_TSHUT_ENA_MASK: c_uint = 0x4000  /* TSHUT_ENA */;

pub const WM8400_TSHUT_OPDIS: c_uint = 0x2000  /* TSHUT_OPDIS */;
pub const WM8400_TSHUT_OPDIS_MASK: c_uint = 0x2000  /* TSHUT_OPDIS */;

pub const WM8400_OPCLK_ENA: c_uint = 0x0800  /* OPCLK_ENA */;
pub const WM8400_OPCLK_ENA_MASK: c_uint = 0x0800  /* OPCLK_ENA */;

pub const WM8400_AINL_ENA: c_uint = 0x0200  /* AINL_ENA */;
pub const WM8400_AINL_ENA_MASK: c_uint = 0x0200  /* AINL_ENA */;

pub const WM8400_AINR_ENA: c_uint = 0x0100  /* AINR_ENA */;
pub const WM8400_AINR_ENA_MASK: c_uint = 0x0100  /* AINR_ENA */;

pub const WM8400_LIN34_ENA: c_uint = 0x0080  /* LIN34_ENA */;
pub const WM8400_LIN34_ENA_MASK: c_uint = 0x0080  /* LIN34_ENA */;

pub const WM8400_LIN12_ENA: c_uint = 0x0040  /* LIN12_ENA */;
pub const WM8400_LIN12_ENA_MASK: c_uint = 0x0040  /* LIN12_ENA */;

pub const WM8400_RIN34_ENA: c_uint = 0x0020  /* RIN34_ENA */;
pub const WM8400_RIN34_ENA_MASK: c_uint = 0x0020  /* RIN34_ENA */;

pub const WM8400_RIN12_ENA: c_uint = 0x0010  /* RIN12_ENA */;
pub const WM8400_RIN12_ENA_MASK: c_uint = 0x0010  /* RIN12_ENA */;

pub const WM8400_ADCL_ENA: c_uint = 0x0002  /* ADCL_ENA */;
pub const WM8400_ADCL_ENA_MASK: c_uint = 0x0002  /* ADCL_ENA */;

pub const WM8400_ADCR_ENA: c_uint = 0x0001  /* ADCR_ENA */;
pub const WM8400_ADCR_ENA_MASK: c_uint = 0x0001  /* ADCR_ENA */;

//
// R4 (0x04) - Power Management (3)
//
pub const WM8400_LON_ENA: c_uint = 0x2000  /* LON_ENA */;
pub const WM8400_LON_ENA_MASK: c_uint = 0x2000  /* LON_ENA */;

pub const WM8400_LOP_ENA: c_uint = 0x1000  /* LOP_ENA */;
pub const WM8400_LOP_ENA_MASK: c_uint = 0x1000  /* LOP_ENA */;

pub const WM8400_RON_ENA: c_uint = 0x0800  /* RON_ENA */;
pub const WM8400_RON_ENA_MASK: c_uint = 0x0800  /* RON_ENA */;

pub const WM8400_ROP_ENA: c_uint = 0x0400  /* ROP_ENA */;
pub const WM8400_ROP_ENA_MASK: c_uint = 0x0400  /* ROP_ENA */;

pub const WM8400_LOPGA_ENA: c_uint = 0x0080  /* LOPGA_ENA */;
pub const WM8400_LOPGA_ENA_MASK: c_uint = 0x0080  /* LOPGA_ENA */;

pub const WM8400_ROPGA_ENA: c_uint = 0x0040  /* ROPGA_ENA */;
pub const WM8400_ROPGA_ENA_MASK: c_uint = 0x0040  /* ROPGA_ENA */;

pub const WM8400_LOMIX_ENA: c_uint = 0x0020  /* LOMIX_ENA */;
pub const WM8400_LOMIX_ENA_MASK: c_uint = 0x0020  /* LOMIX_ENA */;

pub const WM8400_ROMIX_ENA: c_uint = 0x0010  /* ROMIX_ENA */;
pub const WM8400_ROMIX_ENA_MASK: c_uint = 0x0010  /* ROMIX_ENA */;

pub const WM8400_DACL_ENA: c_uint = 0x0002  /* DACL_ENA */;
pub const WM8400_DACL_ENA_MASK: c_uint = 0x0002  /* DACL_ENA */;

pub const WM8400_DACR_ENA: c_uint = 0x0001  /* DACR_ENA */;
pub const WM8400_DACR_ENA_MASK: c_uint = 0x0001  /* DACR_ENA */;

//
// R5 (0x05) - Audio Interface (1)
//
pub const WM8400_AIFADCL_SRC: c_uint = 0x8000  /* AIFADCL_SRC */;
pub const WM8400_AIFADCL_SRC_MASK: c_uint = 0x8000  /* AIFADCL_SRC */;

pub const WM8400_AIFADCR_SRC: c_uint = 0x4000  /* AIFADCR_SRC */;
pub const WM8400_AIFADCR_SRC_MASK: c_uint = 0x4000  /* AIFADCR_SRC */;

pub const WM8400_AIFADC_TDM: c_uint = 0x2000  /* AIFADC_TDM */;
pub const WM8400_AIFADC_TDM_MASK: c_uint = 0x2000  /* AIFADC_TDM */;

pub const WM8400_AIFADC_TDM_CHAN: c_uint = 0x1000  /* AIFADC_TDM_CHAN */;
pub const WM8400_AIFADC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIFADC_TDM_CHAN */;

pub const WM8400_AIF_BCLK_INV: c_uint = 0x0100  /* AIF_BCLK_INV */;
pub const WM8400_AIF_BCLK_INV_MASK: c_uint = 0x0100  /* AIF_BCLK_INV */;

pub const WM8400_AIF_LRCLK_INV: c_uint = 0x0080  /* AIF_LRCLK_INV */;
pub const WM8400_AIF_LRCLK_INV_MASK: c_uint = 0x0080  /* AIF_LRCLK_INV */;

pub const WM8400_AIF_WL_MASK: c_uint = 0x0060  /* AIF_WL - [6:5] */;

pub const WM8400_AIF_FMT_MASK: c_uint = 0x0018  /* AIF_FMT - [4:3] */;

//
// R6 (0x06) - Audio Interface (2)
//
pub const WM8400_DACL_SRC: c_uint = 0x8000  /* DACL_SRC */;
pub const WM8400_DACL_SRC_MASK: c_uint = 0x8000  /* DACL_SRC */;

pub const WM8400_DACR_SRC: c_uint = 0x4000  /* DACR_SRC */;
pub const WM8400_DACR_SRC_MASK: c_uint = 0x4000  /* DACR_SRC */;

pub const WM8400_AIFDAC_TDM: c_uint = 0x2000  /* AIFDAC_TDM */;
pub const WM8400_AIFDAC_TDM_MASK: c_uint = 0x2000  /* AIFDAC_TDM */;

pub const WM8400_AIFDAC_TDM_CHAN: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;
pub const WM8400_AIFDAC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;

pub const WM8400_DAC_BOOST_MASK: c_uint = 0x0C00  /* DAC_BOOST - [11:10] */;

pub const WM8400_DAC_COMP: c_uint = 0x0010  /* DAC_COMP */;
pub const WM8400_DAC_COMP_MASK: c_uint = 0x0010  /* DAC_COMP */;

pub const WM8400_DAC_COMPMODE: c_uint = 0x0008  /* DAC_COMPMODE */;
pub const WM8400_DAC_COMPMODE_MASK: c_uint = 0x0008  /* DAC_COMPMODE */;

pub const WM8400_ADC_COMP: c_uint = 0x0004  /* ADC_COMP */;
pub const WM8400_ADC_COMP_MASK: c_uint = 0x0004  /* ADC_COMP */;

pub const WM8400_ADC_COMPMODE: c_uint = 0x0002  /* ADC_COMPMODE */;
pub const WM8400_ADC_COMPMODE_MASK: c_uint = 0x0002  /* ADC_COMPMODE */;

pub const WM8400_LOOPBACK: c_uint = 0x0001  /* LOOPBACK */;
pub const WM8400_LOOPBACK_MASK: c_uint = 0x0001  /* LOOPBACK */;

//
// R7 (0x07) - Clocking (1)
//
pub const WM8400_TOCLK_RATE: c_uint = 0x8000  /* TOCLK_RATE */;
pub const WM8400_TOCLK_RATE_MASK: c_uint = 0x8000  /* TOCLK_RATE */;

pub const WM8400_TOCLK_ENA: c_uint = 0x4000  /* TOCLK_ENA */;
pub const WM8400_TOCLK_ENA_MASK: c_uint = 0x4000  /* TOCLK_ENA */;

pub const WM8400_OPCLKDIV_MASK: c_uint = 0x1E00  /* OPCLKDIV - [12:9] */;

pub const WM8400_DCLKDIV_MASK: c_uint = 0x01C0  /* DCLKDIV - [8:6] */;

pub const WM8400_BCLK_DIV_MASK: c_uint = 0x001E  /* BCLK_DIV - [4:1] */;

//
// R8 (0x08) - Clocking (2)
//
pub const WM8400_MCLK_SRC: c_uint = 0x8000  /* MCLK_SRC */;
pub const WM8400_MCLK_SRC_MASK: c_uint = 0x8000  /* MCLK_SRC */;

pub const WM8400_SYSCLK_SRC: c_uint = 0x4000  /* SYSCLK_SRC */;
pub const WM8400_SYSCLK_SRC_MASK: c_uint = 0x4000  /* SYSCLK_SRC */;

pub const WM8400_CLK_FORCE: c_uint = 0x2000  /* CLK_FORCE */;
pub const WM8400_CLK_FORCE_MASK: c_uint = 0x2000  /* CLK_FORCE */;

pub const WM8400_MCLK_DIV_MASK: c_uint = 0x1800  /* MCLK_DIV - [12:11] */;

pub const WM8400_MCLK_INV: c_uint = 0x0400  /* MCLK_INV */;
pub const WM8400_MCLK_INV_MASK: c_uint = 0x0400  /* MCLK_INV */;

pub const WM8400_ADC_CLKDIV_MASK: c_uint = 0x00E0  /* ADC_CLKDIV - [7:5] */;

pub const WM8400_DAC_CLKDIV_MASK: c_uint = 0x001C  /* DAC_CLKDIV - [4:2] */;

//
// R9 (0x09) - Audio Interface (3)
//
pub const WM8400_AIF_MSTR1: c_uint = 0x8000  /* AIF_MSTR1 */;
pub const WM8400_AIF_MSTR1_MASK: c_uint = 0x8000  /* AIF_MSTR1 */;

pub const WM8400_AIF_MSTR2: c_uint = 0x4000  /* AIF_MSTR2 */;
pub const WM8400_AIF_MSTR2_MASK: c_uint = 0x4000  /* AIF_MSTR2 */;

pub const WM8400_AIF_SEL: c_uint = 0x2000  /* AIF_SEL */;
pub const WM8400_AIF_SEL_MASK: c_uint = 0x2000  /* AIF_SEL */;

pub const WM8400_ADCLRC_DIR: c_uint = 0x0800  /* ADCLRC_DIR */;
pub const WM8400_ADCLRC_DIR_MASK: c_uint = 0x0800  /* ADCLRC_DIR */;

pub const WM8400_ADCLRC_RATE_MASK: c_uint = 0x07FF  /* ADCLRC_RATE - [10:0] */;

//
// R10 (0x0A) - Audio Interface (4)
//
pub const WM8400_ALRCGPIO1: c_uint = 0x8000  /* ALRCGPIO1 */;
pub const WM8400_ALRCGPIO1_MASK: c_uint = 0x8000  /* ALRCGPIO1 */;

pub const WM8400_ALRCBGPIO6: c_uint = 0x4000  /* ALRCBGPIO6 */;
pub const WM8400_ALRCBGPIO6_MASK: c_uint = 0x4000  /* ALRCBGPIO6 */;

pub const WM8400_AIF_TRIS: c_uint = 0x2000  /* AIF_TRIS */;
pub const WM8400_AIF_TRIS_MASK: c_uint = 0x2000  /* AIF_TRIS */;

pub const WM8400_DACLRC_DIR: c_uint = 0x0800  /* DACLRC_DIR */;
pub const WM8400_DACLRC_DIR_MASK: c_uint = 0x0800  /* DACLRC_DIR */;

pub const WM8400_DACLRC_RATE_MASK: c_uint = 0x07FF  /* DACLRC_RATE - [10:0] */;

//
// R11 (0x0B) - DAC CTRL
//
pub const WM8400_DAC_SDMCLK_RATE: c_uint = 0x2000  /* DAC_SDMCLK_RATE */;
pub const WM8400_DAC_SDMCLK_RATE_MASK: c_uint = 0x2000  /* DAC_SDMCLK_RATE */;

pub const WM8400_AIF_LRCLKRATE: c_uint = 0x0400  /* AIF_LRCLKRATE */;
pub const WM8400_AIF_LRCLKRATE_MASK: c_uint = 0x0400  /* AIF_LRCLKRATE */;

pub const WM8400_DAC_MONO: c_uint = 0x0200  /* DAC_MONO */;
pub const WM8400_DAC_MONO_MASK: c_uint = 0x0200  /* DAC_MONO */;

pub const WM8400_DAC_SB_FILT: c_uint = 0x0100  /* DAC_SB_FILT */;
pub const WM8400_DAC_SB_FILT_MASK: c_uint = 0x0100  /* DAC_SB_FILT */;

pub const WM8400_DAC_MUTERATE: c_uint = 0x0080  /* DAC_MUTERATE */;
pub const WM8400_DAC_MUTERATE_MASK: c_uint = 0x0080  /* DAC_MUTERATE */;

pub const WM8400_DAC_MUTEMODE: c_uint = 0x0040  /* DAC_MUTEMODE */;
pub const WM8400_DAC_MUTEMODE_MASK: c_uint = 0x0040  /* DAC_MUTEMODE */;

pub const WM8400_DEEMP_MASK: c_uint = 0x0030  /* DEEMP - [5:4] */;

pub const WM8400_DAC_MUTE: c_uint = 0x0004  /* DAC_MUTE */;
pub const WM8400_DAC_MUTE_MASK: c_uint = 0x0004  /* DAC_MUTE */;

pub const WM8400_DACL_DATINV: c_uint = 0x0002  /* DACL_DATINV */;
pub const WM8400_DACL_DATINV_MASK: c_uint = 0x0002  /* DACL_DATINV */;

pub const WM8400_DACR_DATINV: c_uint = 0x0001  /* DACR_DATINV */;
pub const WM8400_DACR_DATINV_MASK: c_uint = 0x0001  /* DACR_DATINV */;

//
// R12 (0x0C) - Left DAC Digital Volume
//
pub const WM8400_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8400_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8400_DACL_VOL_MASK: c_uint = 0x00FF  /* DACL_VOL - [7:0] */;

//
// R13 (0x0D) - Right DAC Digital Volume
//
pub const WM8400_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8400_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8400_DACR_VOL_MASK: c_uint = 0x00FF  /* DACR_VOL - [7:0] */;

//
// R14 (0x0E) - Digital Side Tone
//
pub const WM8400_ADCL_DAC_SVOL_MASK: c_uint = 0x1E00  /*   ADCL_DAC_SVOL - [12:9] */;

pub const WM8400_ADCR_DAC_SVOL_MASK: c_uint = 0x01E0  /* ADCR_DAC_SVOL - [8:5] */;

pub const WM8400_ADC_TO_DACL_MASK: c_uint = 0x000C  /* ADC_TO_DACL - [3:2] */;

pub const WM8400_ADC_TO_DACR_MASK: c_uint = 0x0003  /* ADC_TO_DACR - [1:0] */;

//
// R15 (0x0F) - ADC CTRL
//
pub const WM8400_ADC_HPF_ENA: c_uint = 0x0100  /* ADC_HPF_ENA */;
pub const WM8400_ADC_HPF_ENA_MASK: c_uint = 0x0100  /* ADC_HPF_ENA */;

pub const WM8400_ADC_HPF_CUT_MASK: c_uint = 0x0060  /* ADC_HPF_CUT - [6:5] */;

pub const WM8400_ADCL_DATINV: c_uint = 0x0002  /* ADCL_DATINV */;
pub const WM8400_ADCL_DATINV_MASK: c_uint = 0x0002  /* ADCL_DATINV */;

pub const WM8400_ADCR_DATINV: c_uint = 0x0001  /* ADCR_DATINV */;
pub const WM8400_ADCR_DATINV_MASK: c_uint = 0x0001  /* ADCR_DATINV */;

//
// R16 (0x10) - Left ADC Digital Volume
//
pub const WM8400_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8400_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8400_ADCL_VOL_MASK: c_uint = 0x00FF  /* ADCL_VOL - [7:0] */;

//
// R17 (0x11) - Right ADC Digital Volume
//
pub const WM8400_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8400_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8400_ADCR_VOL_MASK: c_uint = 0x00FF  /* ADCR_VOL - [7:0] */;

//
// R24 (0x18) - Left Line Input 1&2 Volume
//
pub const WM8400_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8400_IPVU_MASK: c_uint = 0x0100  /* IPVU */;

pub const WM8400_LI12MUTE: c_uint = 0x0080  /* LI12MUTE */;
pub const WM8400_LI12MUTE_MASK: c_uint = 0x0080  /* LI12MUTE */;

pub const WM8400_LI12ZC: c_uint = 0x0040  /* LI12ZC */;
pub const WM8400_LI12ZC_MASK: c_uint = 0x0040  /* LI12ZC */;

pub const WM8400_LIN12VOL_MASK: c_uint = 0x001F  /* LIN12VOL - [4:0] */;

//
// R25 (0x19) - Left Line Input 3&4 Volume
//
pub const WM8400_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8400_IPVU_MASK: c_uint = 0x0100  /* IPVU */;

pub const WM8400_LI34MUTE: c_uint = 0x0080  /* LI34MUTE */;
pub const WM8400_LI34MUTE_MASK: c_uint = 0x0080  /* LI34MUTE */;

pub const WM8400_LI34ZC: c_uint = 0x0040  /* LI34ZC */;
pub const WM8400_LI34ZC_MASK: c_uint = 0x0040  /* LI34ZC */;

pub const WM8400_LIN34VOL_MASK: c_uint = 0x001F  /* LIN34VOL - [4:0] */;

//
// R26 (0x1A) - Right Line Input 1&2 Volume
//
pub const WM8400_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8400_IPVU_MASK: c_uint = 0x0100  /* IPVU */;

pub const WM8400_RI12MUTE: c_uint = 0x0080  /* RI12MUTE */;
pub const WM8400_RI12MUTE_MASK: c_uint = 0x0080  /* RI12MUTE */;

pub const WM8400_RI12ZC: c_uint = 0x0040  /* RI12ZC */;
pub const WM8400_RI12ZC_MASK: c_uint = 0x0040  /* RI12ZC */;

pub const WM8400_RIN12VOL_MASK: c_uint = 0x001F  /* RIN12VOL - [4:0] */;

//
// R27 (0x1B) - Right Line Input 3&4 Volume
//
pub const WM8400_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8400_IPVU_MASK: c_uint = 0x0100  /* IPVU */;

pub const WM8400_RI34MUTE: c_uint = 0x0080  /* RI34MUTE */;
pub const WM8400_RI34MUTE_MASK: c_uint = 0x0080  /* RI34MUTE */;

pub const WM8400_RI34ZC: c_uint = 0x0040  /* RI34ZC */;
pub const WM8400_RI34ZC_MASK: c_uint = 0x0040  /* RI34ZC */;

pub const WM8400_RIN34VOL_MASK: c_uint = 0x001F  /* RIN34VOL - [4:0] */;

//
// R28 (0x1C) - Left Output Volume
//
pub const WM8400_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8400_OPVU_MASK: c_uint = 0x0100  /* OPVU */;

pub const WM8400_LOZC: c_uint = 0x0080  /* LOZC */;
pub const WM8400_LOZC_MASK: c_uint = 0x0080  /* LOZC */;

pub const WM8400_LOUTVOL_MASK: c_uint = 0x007F  /* LOUTVOL - [6:0] */;

//
// R29 (0x1D) - Right Output Volume
//
pub const WM8400_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8400_OPVU_MASK: c_uint = 0x0100  /* OPVU */;

pub const WM8400_ROZC: c_uint = 0x0080  /* ROZC */;
pub const WM8400_ROZC_MASK: c_uint = 0x0080  /* ROZC */;

pub const WM8400_ROUTVOL_MASK: c_uint = 0x007F  /* ROUTVOL - [6:0] */;

//
// R30 (0x1E) - Line Outputs Volume
//
pub const WM8400_LONMUTE: c_uint = 0x0040  /* LONMUTE */;
pub const WM8400_LONMUTE_MASK: c_uint = 0x0040  /* LONMUTE */;

pub const WM8400_LOPMUTE: c_uint = 0x0020  /* LOPMUTE */;
pub const WM8400_LOPMUTE_MASK: c_uint = 0x0020  /* LOPMUTE */;

pub const WM8400_LOATTN: c_uint = 0x0010  /* LOATTN */;
pub const WM8400_LOATTN_MASK: c_uint = 0x0010  /* LOATTN */;

pub const WM8400_RONMUTE: c_uint = 0x0004  /* RONMUTE */;
pub const WM8400_RONMUTE_MASK: c_uint = 0x0004  /* RONMUTE */;

pub const WM8400_ROPMUTE: c_uint = 0x0002  /* ROPMUTE */;
pub const WM8400_ROPMUTE_MASK: c_uint = 0x0002  /* ROPMUTE */;

pub const WM8400_ROATTN: c_uint = 0x0001  /* ROATTN */;
pub const WM8400_ROATTN_MASK: c_uint = 0x0001  /* ROATTN */;

//
// R31 (0x1F) - Out3/4 Volume
//
pub const WM8400_OUT3MUTE: c_uint = 0x0020  /* OUT3MUTE */;
pub const WM8400_OUT3MUTE_MASK: c_uint = 0x0020  /* OUT3MUTE */;

pub const WM8400_OUT3ATTN: c_uint = 0x0010  /* OUT3ATTN */;
pub const WM8400_OUT3ATTN_MASK: c_uint = 0x0010  /* OUT3ATTN */;

pub const WM8400_OUT4MUTE: c_uint = 0x0002  /* OUT4MUTE */;
pub const WM8400_OUT4MUTE_MASK: c_uint = 0x0002  /* OUT4MUTE */;

pub const WM8400_OUT4ATTN: c_uint = 0x0001  /* OUT4ATTN */;
pub const WM8400_OUT4ATTN_MASK: c_uint = 0x0001  /* OUT4ATTN */;

//
// R32 (0x20) - Left OPGA Volume
//
pub const WM8400_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8400_OPVU_MASK: c_uint = 0x0100  /* OPVU */;

pub const WM8400_LOPGAZC: c_uint = 0x0080  /* LOPGAZC */;
pub const WM8400_LOPGAZC_MASK: c_uint = 0x0080  /* LOPGAZC */;

pub const WM8400_LOPGAVOL_MASK: c_uint = 0x007F  /* LOPGAVOL - [6:0] */;

//
// R33 (0x21) - Right OPGA Volume
//
pub const WM8400_OPVU: c_uint = 0x0100  /* OPVU */;
pub const WM8400_OPVU_MASK: c_uint = 0x0100  /* OPVU */;

pub const WM8400_ROPGAZC: c_uint = 0x0080  /* ROPGAZC */;
pub const WM8400_ROPGAZC_MASK: c_uint = 0x0080  /* ROPGAZC */;

pub const WM8400_ROPGAVOL_MASK: c_uint = 0x007F  /* ROPGAVOL - [6:0] */;

//
// R34 (0x22) - Speaker Volume
//
pub const WM8400_SPKATTN_MASK: c_uint = 0x0003  /* SPKATTN - [1:0] */;

//
// R35 (0x23) - ClassD1
//
pub const WM8400_CDMODE: c_uint = 0x0100  /* CDMODE */;
pub const WM8400_CDMODE_MASK: c_uint = 0x0100  /* CDMODE */;

pub const WM8400_CLASSD_CLK_SEL: c_uint = 0x0080  /* CLASSD_CLK_SEL */;
pub const WM8400_CLASSD_CLK_SEL_MASK: c_uint = 0x0080  /* CLASSD_CLK_SEL */;

pub const WM8400_CD_SRCTRL: c_uint = 0x0040  /* CD_SRCTRL */;
pub const WM8400_CD_SRCTRL_MASK: c_uint = 0x0040  /* CD_SRCTRL */;

pub const WM8400_SPKNOPOP: c_uint = 0x0020  /* SPKNOPOP */;
pub const WM8400_SPKNOPOP_MASK: c_uint = 0x0020  /* SPKNOPOP */;

pub const WM8400_DBLERATE: c_uint = 0x0010  /* DBLERATE */;
pub const WM8400_DBLERATE_MASK: c_uint = 0x0010  /* DBLERATE */;

pub const WM8400_LOOPTEST: c_uint = 0x0008  /* LOOPTEST */;
pub const WM8400_LOOPTEST_MASK: c_uint = 0x0008  /* LOOPTEST */;

pub const WM8400_HALFABBIAS: c_uint = 0x0004  /* HALFABBIAS */;
pub const WM8400_HALFABBIAS_MASK: c_uint = 0x0004  /* HALFABBIAS */;

pub const WM8400_TRIDEL_MASK: c_uint = 0x0003  /* TRIDEL - [1:0] */;

//
// R37 (0x25) - ClassD3
//
pub const WM8400_DCGAIN_MASK: c_uint = 0x0038  /* DCGAIN - [5:3] */;

pub const WM8400_ACGAIN_MASK: c_uint = 0x0007  /* ACGAIN - [2:0] */;

//
// R39 (0x27) - Input Mixer1
//
pub const WM8400_AINLMODE_MASK: c_uint = 0x000C  /* AINLMODE - [3:2] */;

pub const WM8400_AINRMODE_MASK: c_uint = 0x0003  /* AINRMODE - [1:0] */;

//
// R40 (0x28) - Input Mixer2
//
pub const WM8400_LMP4: c_uint = 0x0080  /* LMP4 */;
pub const WM8400_LMP4_MASK: c_uint = 0x0080  /* LMP4 */;

pub const WM8400_LMN3: c_uint = 0x0040  /* LMN3 */;
pub const WM8400_LMN3_MASK: c_uint = 0x0040  /* LMN3 */;

pub const WM8400_LMP2: c_uint = 0x0020  /* LMP2 */;
pub const WM8400_LMP2_MASK: c_uint = 0x0020  /* LMP2 */;

pub const WM8400_LMN1: c_uint = 0x0010  /* LMN1 */;
pub const WM8400_LMN1_MASK: c_uint = 0x0010  /* LMN1 */;

pub const WM8400_RMP4: c_uint = 0x0008  /* RMP4 */;
pub const WM8400_RMP4_MASK: c_uint = 0x0008  /* RMP4 */;

pub const WM8400_RMN3: c_uint = 0x0004  /* RMN3 */;
pub const WM8400_RMN3_MASK: c_uint = 0x0004  /* RMN3 */;

pub const WM8400_RMP2: c_uint = 0x0002  /* RMP2 */;
pub const WM8400_RMP2_MASK: c_uint = 0x0002  /* RMP2 */;

pub const WM8400_RMN1: c_uint = 0x0001  /* RMN1 */;
pub const WM8400_RMN1_MASK: c_uint = 0x0001  /* RMN1 */;

//
// R41 (0x29) - Input Mixer3
//
pub const WM8400_L34MNB: c_uint = 0x0100  /* L34MNB */;
pub const WM8400_L34MNB_MASK: c_uint = 0x0100  /* L34MNB */;

pub const WM8400_L34MNBST: c_uint = 0x0080  /* L34MNBST */;
pub const WM8400_L34MNBST_MASK: c_uint = 0x0080  /* L34MNBST */;

pub const WM8400_L12MNB: c_uint = 0x0020  /* L12MNB */;
pub const WM8400_L12MNB_MASK: c_uint = 0x0020  /* L12MNB */;

pub const WM8400_L12MNBST: c_uint = 0x0010  /* L12MNBST */;
pub const WM8400_L12MNBST_MASK: c_uint = 0x0010  /* L12MNBST */;

pub const WM8400_LDBVOL_MASK: c_uint = 0x0007  /* LDBVOL - [2:0] */;

//
// R42 (0x2A) - Input Mixer4
//
pub const WM8400_R34MNB: c_uint = 0x0100  /* R34MNB */;
pub const WM8400_R34MNB_MASK: c_uint = 0x0100  /* R34MNB */;

pub const WM8400_R34MNBST: c_uint = 0x0080  /* R34MNBST */;
pub const WM8400_R34MNBST_MASK: c_uint = 0x0080  /* R34MNBST */;

pub const WM8400_R12MNB: c_uint = 0x0020  /* R12MNB */;
pub const WM8400_R12MNB_MASK: c_uint = 0x0020  /* R12MNB */;

pub const WM8400_R12MNBST: c_uint = 0x0010  /* R12MNBST */;
pub const WM8400_R12MNBST_MASK: c_uint = 0x0010  /* R12MNBST */;

pub const WM8400_RDBVOL_MASK: c_uint = 0x0007  /* RDBVOL - [2:0] */;

//
// R43 (0x2B) - Input Mixer5
//
pub const WM8400_LI2BVOL_MASK: c_uint = 0x01C0  /* LI2BVOL - [8:6] */;

pub const WM8400_LR4BVOL_MASK: c_uint = 0x0038  /* LR4BVOL - [5:3] */;

pub const WM8400_LL4BVOL_MASK: c_uint = 0x0007  /* LL4BVOL - [2:0] */;

//
// R44 (0x2C) - Input Mixer6
//
pub const WM8400_RI2BVOL_MASK: c_uint = 0x01C0  /* RI2BVOL - [8:6] */;

pub const WM8400_RL4BVOL_MASK: c_uint = 0x0038  /* RL4BVOL - [5:3] */;

pub const WM8400_RR4BVOL_MASK: c_uint = 0x0007  /* RR4BVOL - [2:0] */;

//
// R45 (0x2D) - Output Mixer1
//
pub const WM8400_LRBLO: c_uint = 0x0080  /* LRBLO */;
pub const WM8400_LRBLO_MASK: c_uint = 0x0080  /* LRBLO */;

pub const WM8400_LLBLO: c_uint = 0x0040  /* LLBLO */;
pub const WM8400_LLBLO_MASK: c_uint = 0x0040  /* LLBLO */;

pub const WM8400_LRI3LO: c_uint = 0x0020  /* LRI3LO */;
pub const WM8400_LRI3LO_MASK: c_uint = 0x0020  /* LRI3LO */;

pub const WM8400_LLI3LO: c_uint = 0x0010  /* LLI3LO */;
pub const WM8400_LLI3LO_MASK: c_uint = 0x0010  /* LLI3LO */;

pub const WM8400_LR12LO: c_uint = 0x0008  /* LR12LO */;
pub const WM8400_LR12LO_MASK: c_uint = 0x0008  /* LR12LO */;

pub const WM8400_LL12LO: c_uint = 0x0004  /* LL12LO */;
pub const WM8400_LL12LO_MASK: c_uint = 0x0004  /* LL12LO */;

pub const WM8400_LDLO: c_uint = 0x0001  /* LDLO */;
pub const WM8400_LDLO_MASK: c_uint = 0x0001  /* LDLO */;

//
// R46 (0x2E) - Output Mixer2
//
pub const WM8400_RLBRO: c_uint = 0x0080  /* RLBRO */;
pub const WM8400_RLBRO_MASK: c_uint = 0x0080  /* RLBRO */;

pub const WM8400_RRBRO: c_uint = 0x0040  /* RRBRO */;
pub const WM8400_RRBRO_MASK: c_uint = 0x0040  /* RRBRO */;

pub const WM8400_RLI3RO: c_uint = 0x0020  /* RLI3RO */;
pub const WM8400_RLI3RO_MASK: c_uint = 0x0020  /* RLI3RO */;

pub const WM8400_RRI3RO: c_uint = 0x0010  /* RRI3RO */;
pub const WM8400_RRI3RO_MASK: c_uint = 0x0010  /* RRI3RO */;

pub const WM8400_RL12RO: c_uint = 0x0008  /* RL12RO */;
pub const WM8400_RL12RO_MASK: c_uint = 0x0008  /* RL12RO */;

pub const WM8400_RR12RO: c_uint = 0x0004  /* RR12RO */;
pub const WM8400_RR12RO_MASK: c_uint = 0x0004  /* RR12RO */;

pub const WM8400_RDRO: c_uint = 0x0001  /* RDRO */;
pub const WM8400_RDRO_MASK: c_uint = 0x0001  /* RDRO */;

//
// R47 (0x2F) - Output Mixer3
//
pub const WM8400_LLI3LOVOL_MASK: c_uint = 0x01C0  /* LLI3LOVOL - [8:6] */;

pub const WM8400_LR12LOVOL_MASK: c_uint = 0x0038  /* LR12LOVOL - [5:3] */;

pub const WM8400_LL12LOVOL_MASK: c_uint = 0x0007  /* LL12LOVOL - [2:0] */;

//
// R48 (0x30) - Output Mixer4
//
pub const WM8400_RRI3ROVOL_MASK: c_uint = 0x01C0  /* RRI3ROVOL - [8:6] */;

pub const WM8400_RL12ROVOL_MASK: c_uint = 0x0038  /* RL12ROVOL - [5:3] */;

pub const WM8400_RR12ROVOL_MASK: c_uint = 0x0007  /* RR12ROVOL - [2:0] */;

//
// R49 (0x31) - Output Mixer5
//
pub const WM8400_LRI3LOVOL_MASK: c_uint = 0x01C0  /* LRI3LOVOL - [8:6] */;

pub const WM8400_LRBLOVOL_MASK: c_uint = 0x0038  /* LRBLOVOL - [5:3] */;

pub const WM8400_LLBLOVOL_MASK: c_uint = 0x0007  /* LLBLOVOL - [2:0] */;

//
// R50 (0x32) - Output Mixer6
//
pub const WM8400_RLI3ROVOL_MASK: c_uint = 0x01C0  /* RLI3ROVOL - [8:6] */;

pub const WM8400_RLBROVOL_MASK: c_uint = 0x0038  /* RLBROVOL - [5:3] */;

pub const WM8400_RRBROVOL_MASK: c_uint = 0x0007  /* RRBROVOL - [2:0] */;

//
// R51 (0x33) - Out3/4 Mixer
//
pub const WM8400_VSEL_MASK: c_uint = 0x0180  /* VSEL - [8:7] */;

pub const WM8400_LI4O3: c_uint = 0x0020  /* LI4O3 */;
pub const WM8400_LI4O3_MASK: c_uint = 0x0020  /* LI4O3 */;

pub const WM8400_LPGAO3: c_uint = 0x0010  /* LPGAO3 */;
pub const WM8400_LPGAO3_MASK: c_uint = 0x0010  /* LPGAO3 */;

pub const WM8400_RI4O4: c_uint = 0x0002  /* RI4O4 */;
pub const WM8400_RI4O4_MASK: c_uint = 0x0002  /* RI4O4 */;

pub const WM8400_RPGAO4: c_uint = 0x0001  /* RPGAO4 */;
pub const WM8400_RPGAO4_MASK: c_uint = 0x0001  /* RPGAO4 */;

//
// R52 (0x34) - Line Mixer1
//
pub const WM8400_LLOPGALON: c_uint = 0x0040  /* LLOPGALON */;
pub const WM8400_LLOPGALON_MASK: c_uint = 0x0040  /* LLOPGALON */;

pub const WM8400_LROPGALON: c_uint = 0x0020  /* LROPGALON */;
pub const WM8400_LROPGALON_MASK: c_uint = 0x0020  /* LROPGALON */;

pub const WM8400_LOPLON: c_uint = 0x0010  /* LOPLON */;
pub const WM8400_LOPLON_MASK: c_uint = 0x0010  /* LOPLON */;

pub const WM8400_LR12LOP: c_uint = 0x0004  /* LR12LOP */;
pub const WM8400_LR12LOP_MASK: c_uint = 0x0004  /* LR12LOP */;

pub const WM8400_LL12LOP: c_uint = 0x0002  /* LL12LOP */;
pub const WM8400_LL12LOP_MASK: c_uint = 0x0002  /* LL12LOP */;

pub const WM8400_LLOPGALOP: c_uint = 0x0001  /* LLOPGALOP */;
pub const WM8400_LLOPGALOP_MASK: c_uint = 0x0001  /* LLOPGALOP */;

//
// R53 (0x35) - Line Mixer2
//
pub const WM8400_RROPGARON: c_uint = 0x0040  /* RROPGARON */;
pub const WM8400_RROPGARON_MASK: c_uint = 0x0040  /* RROPGARON */;

pub const WM8400_RLOPGARON: c_uint = 0x0020  /* RLOPGARON */;
pub const WM8400_RLOPGARON_MASK: c_uint = 0x0020  /* RLOPGARON */;

pub const WM8400_ROPRON: c_uint = 0x0010  /* ROPRON */;
pub const WM8400_ROPRON_MASK: c_uint = 0x0010  /* ROPRON */;

pub const WM8400_RL12ROP: c_uint = 0x0004  /* RL12ROP */;
pub const WM8400_RL12ROP_MASK: c_uint = 0x0004  /* RL12ROP */;

pub const WM8400_RR12ROP: c_uint = 0x0002  /* RR12ROP */;
pub const WM8400_RR12ROP_MASK: c_uint = 0x0002  /* RR12ROP */;

pub const WM8400_RROPGAROP: c_uint = 0x0001  /* RROPGAROP */;
pub const WM8400_RROPGAROP_MASK: c_uint = 0x0001  /* RROPGAROP */;

//
// R54 (0x36) - Speaker Mixer
//
pub const WM8400_LB2SPK: c_uint = 0x0080  /* LB2SPK */;
pub const WM8400_LB2SPK_MASK: c_uint = 0x0080  /* LB2SPK */;

pub const WM8400_RB2SPK: c_uint = 0x0040  /* RB2SPK */;
pub const WM8400_RB2SPK_MASK: c_uint = 0x0040  /* RB2SPK */;

pub const WM8400_LI2SPK: c_uint = 0x0020  /* LI2SPK */;
pub const WM8400_LI2SPK_MASK: c_uint = 0x0020  /* LI2SPK */;

pub const WM8400_RI2SPK: c_uint = 0x0010  /* RI2SPK */;
pub const WM8400_RI2SPK_MASK: c_uint = 0x0010  /* RI2SPK */;

pub const WM8400_LOPGASPK: c_uint = 0x0008  /* LOPGASPK */;
pub const WM8400_LOPGASPK_MASK: c_uint = 0x0008  /* LOPGASPK */;

pub const WM8400_ROPGASPK: c_uint = 0x0004  /* ROPGASPK */;
pub const WM8400_ROPGASPK_MASK: c_uint = 0x0004  /* ROPGASPK */;

pub const WM8400_LDSPK: c_uint = 0x0002  /* LDSPK */;
pub const WM8400_LDSPK_MASK: c_uint = 0x0002  /* LDSPK */;

pub const WM8400_RDSPK: c_uint = 0x0001  /* RDSPK */;
pub const WM8400_RDSPK_MASK: c_uint = 0x0001  /* RDSPK */;

//
// R55 (0x37) - Additional Control
//
pub const WM8400_VROI: c_uint = 0x0001  /* VROI */;
pub const WM8400_VROI_MASK: c_uint = 0x0001  /* VROI */;

//
// R56 (0x38) - AntiPOP1
//
pub const WM8400_DIS_LLINE: c_uint = 0x0020  /* DIS_LLINE */;
pub const WM8400_DIS_LLINE_MASK: c_uint = 0x0020  /* DIS_LLINE */;

pub const WM8400_DIS_RLINE: c_uint = 0x0010  /* DIS_RLINE */;
pub const WM8400_DIS_RLINE_MASK: c_uint = 0x0010  /* DIS_RLINE */;

pub const WM8400_DIS_OUT3: c_uint = 0x0008  /* DIS_OUT3 */;
pub const WM8400_DIS_OUT3_MASK: c_uint = 0x0008  /* DIS_OUT3 */;

pub const WM8400_DIS_OUT4: c_uint = 0x0004  /* DIS_OUT4 */;
pub const WM8400_DIS_OUT4_MASK: c_uint = 0x0004  /* DIS_OUT4 */;

pub const WM8400_DIS_LOUT: c_uint = 0x0002  /* DIS_LOUT */;
pub const WM8400_DIS_LOUT_MASK: c_uint = 0x0002  /* DIS_LOUT */;

pub const WM8400_DIS_ROUT: c_uint = 0x0001  /* DIS_ROUT */;
pub const WM8400_DIS_ROUT_MASK: c_uint = 0x0001  /* DIS_ROUT */;

//
// R57 (0x39) - AntiPOP2
//
pub const WM8400_SOFTST: c_uint = 0x0040  /* SOFTST */;
pub const WM8400_SOFTST_MASK: c_uint = 0x0040  /* SOFTST */;

pub const WM8400_BUFIOEN: c_uint = 0x0008  /* BUFIOEN */;
pub const WM8400_BUFIOEN_MASK: c_uint = 0x0008  /* BUFIOEN */;

pub const WM8400_BUFDCOPEN: c_uint = 0x0004  /* BUFDCOPEN */;
pub const WM8400_BUFDCOPEN_MASK: c_uint = 0x0004  /* BUFDCOPEN */;

pub const WM8400_POBCTRL: c_uint = 0x0002  /* POBCTRL */;
pub const WM8400_POBCTRL_MASK: c_uint = 0x0002  /* POBCTRL */;

pub const WM8400_VMIDTOG: c_uint = 0x0001  /* VMIDTOG */;
pub const WM8400_VMIDTOG_MASK: c_uint = 0x0001  /* VMIDTOG */;

//
// R58 (0x3A) - MICBIAS
//
pub const WM8400_MCDSCTH_MASK: c_uint = 0x00C0  /* MCDSCTH - [7:6] */;

pub const WM8400_MCDTHR_MASK: c_uint = 0x0038  /* MCDTHR - [5:3] */;

pub const WM8400_MCD: c_uint = 0x0004  /* MCD */;
pub const WM8400_MCD_MASK: c_uint = 0x0004  /* MCD */;

pub const WM8400_MBSEL: c_uint = 0x0001  /* MBSEL */;
pub const WM8400_MBSEL_MASK: c_uint = 0x0001  /* MBSEL */;

//
// R60 (0x3C) - FLL Control 1
//
pub const WM8400_FLL_REF_FREQ: c_uint = 0x1000  /* FLL_REF_FREQ */;
pub const WM8400_FLL_REF_FREQ_MASK: c_uint = 0x1000  /* FLL_REF_FREQ */;

pub const WM8400_FLL_CLK_SRC_MASK: c_uint = 0x0C00  /* FLL_CLK_SRC - [11:10] */;

pub const WM8400_FLL_FRAC: c_uint = 0x0200  /* FLL_FRAC */;
pub const WM8400_FLL_FRAC_MASK: c_uint = 0x0200  /* FLL_FRAC */;

pub const WM8400_FLL_OSC_ENA: c_uint = 0x0100  /* FLL_OSC_ENA */;
pub const WM8400_FLL_OSC_ENA_MASK: c_uint = 0x0100  /* FLL_OSC_ENA */;

pub const WM8400_FLL_CTRL_RATE_MASK: c_uint = 0x00E0  /* FLL_CTRL_RATE - [7:5] */;

pub const WM8400_FLL_FRATIO_MASK: c_uint = 0x001F  /* FLL_FRATIO - [4:0] */;

//
// R61 (0x3D) - FLL Control 2
//
pub const WM8400_FLL_K_MASK: c_uint = 0xFFFF  /* FLL_K - [15:0] */;

//
// R62 (0x3E) - FLL Control 3
//
pub const WM8400_FLL_N_MASK: c_uint = 0x03FF  /* FLL_N - [9:0] */;

//
// R63 (0x3F) - FLL Control 4
//
pub const WM8400_FLL_TRK_GAIN_MASK: c_uint = 0x0078  /* FLL_TRK_GAIN - [6:3] */;

pub const WM8400_FLL_OUTDIV_MASK: c_uint = 0x0007  /* FLL_OUTDIV - [2:0] */;

extern "C" {
    pub fn wm8400_reset_codec_reg_cache(wm8400: *mut wm8400);
}
