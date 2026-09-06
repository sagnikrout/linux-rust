//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs42l42.h
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
// linux/sound/cs42l42.h -- Platform data for CS42L42 ALSA SoC audio driver header
//
// Copyright 2016-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>
// Author: Brian Austin <brian.austin@cirrus.com>
// Author: Michael White <michael.white@cirrus.com>
//
pub const CS42L42_PAGE_REGISTER: c_uint = 0x00	/* Page Select Register */;
pub const CS42L42_WIN_START: c_uint = 0x00;
pub const CS42L42_WIN_LEN: c_uint = 0x100;
pub const CS42L42_RANGE_MIN: c_uint = 0x00;
pub const CS42L42_RANGE_MAX: c_uint = 0x7F;
pub const CS42L42_PAGE_10: c_uint = 0x1000;
pub const CS42L42_PAGE_11: c_uint = 0x1100;
pub const CS42L42_PAGE_12: c_uint = 0x1200;
pub const CS42L42_PAGE_13: c_uint = 0x1300;
pub const CS42L42_PAGE_15: c_uint = 0x1500;
pub const CS42L42_PAGE_19: c_uint = 0x1900;
pub const CS42L42_PAGE_1B: c_uint = 0x1B00;
pub const CS42L42_PAGE_1C: c_uint = 0x1C00;
pub const CS42L42_PAGE_1D: c_uint = 0x1D00;
pub const CS42L42_PAGE_1F: c_uint = 0x1F00;
pub const CS42L42_PAGE_20: c_uint = 0x2000;
pub const CS42L42_PAGE_21: c_uint = 0x2100;
pub const CS42L42_PAGE_23: c_uint = 0x2300;
pub const CS42L42_PAGE_24: c_uint = 0x2400;
pub const CS42L42_PAGE_25: c_uint = 0x2500;
pub const CS42L42_PAGE_26: c_uint = 0x2600;
pub const CS42L42_PAGE_27: c_uint = 0x2700;
pub const CS42L42_PAGE_28: c_uint = 0x2800;
pub const CS42L42_PAGE_29: c_uint = 0x2900;
pub const CS42L42_PAGE_2A: c_uint = 0x2A00;
pub const CS42L42_PAGE_30: c_uint = 0x3000;
pub const CS42L42_CHIP_ID: c_uint = 0x42A42;
pub const CS42L83_CHIP_ID: c_uint = 0x42A83;
// Page 0x10 Global Registers

pub const CS42L42_SRC_BYPASS_DAC_SHIFT: c_int = 1;

pub const CS42L42_INTERNAL_FS_SHIFT: c_int = 1;

pub const CS42L42_SLOW_START_EN_SHIFT: c_int = 4;

// Page 0x11 Power and Headset Detect Registers

pub const CS42L42_ASP_DAO_PDN_SHIFT: c_int = 7;

pub const CS42L42_ASP_DAI_PDN_SHIFT: c_int = 6;

pub const CS42L42_MIXER_PDN_SHIFT: c_int = 5;

pub const CS42L42_EQ_PDN_SHIFT: c_int = 4;

pub const CS42L42_HP_PDN_SHIFT: c_int = 3;

pub const CS42L42_ADC_PDN_SHIFT: c_int = 2;

pub const CS42L42_PDN_ALL_SHIFT: c_int = 0;

pub const CS42L42_ADC_SRC_PDNB_SHIFT: c_int = 0;

pub const CS42L42_DAC_SRC_PDNB_SHIFT: c_int = 1;

pub const CS42L42_ASP_DAI1_PDN_SHIFT: c_int = 2;

pub const CS42L42_SRC_PDN_OVERRIDE_SHIFT: c_int = 3;

pub const CS42L42_DISCHARGE_FILT_SHIFT: c_int = 4;

pub const CS42L42_RING_SENSE_PDNB_SHIFT: c_int = 1;

pub const CS42L42_VPMON_PDNB_SHIFT: c_int = 2;

pub const CS42L42_SW_CLK_STP_STAT_SEL_SHIFT: c_int = 5;

pub const CS42L42_RS_TRIM_R_SHIFT: c_int = 0;

pub const CS42L42_RS_TRIM_T_SHIFT: c_int = 1;

pub const CS42L42_HPREF_RS_SHIFT: c_int = 2;

pub const CS42L42_HSBIAS_FILT_REF_RS_SHIFT: c_int = 3;

pub const CS42L42_RING_SENSE_PU_HIZ_SHIFT: c_int = 6;

pub const CS42L42_TS_RS_GATE_SHIFT: c_int = 7;

pub const CS42L42_SCLK_PRESENT_SHIFT: c_int = 0;

pub const CS42L42_OSC_SW_SEL_STAT_SHIFT: c_int = 0;

pub const CS42L42_OSC_PDNB_STAT_SHIFT: c_int = 2;

pub const CS42L42_RS_RISE_DBNCE_TIME_SHIFT: c_int = 0;

pub const CS42L42_RS_FALL_DBNCE_TIME_SHIFT: c_int = 3;

pub const CS42L42_RS_PU_EN_SHIFT: c_int = 6;

pub const CS42L42_RS_INV_SHIFT: c_int = 7;

pub const CS42L42_TS_RISE_DBNCE_TIME_SHIFT: c_int = 0;

pub const CS42L42_TS_FALL_DBNCE_TIME_SHIFT: c_int = 3;

pub const CS42L42_TS_INV_SHIFT: c_int = 7;

pub const CS42L42_D_RS_PLUG_DBNC_SHIFT: c_int = 0;

pub const CS42L42_D_RS_UNPLUG_DBNC_SHIFT: c_int = 1;

pub const CS42L42_D_TS_PLUG_DBNC_SHIFT: c_int = 2;

pub const CS42L42_D_TS_UNPLUG_DBNC_SHIFT: c_int = 3;

pub const CS42L42_RS_PLUG_DBNC_SHIFT: c_int = 0;

pub const CS42L42_RS_UNPLUG_DBNC_SHIFT: c_int = 1;

pub const CS42L42_TS_PLUG_DBNC_SHIFT: c_int = 2;

pub const CS42L42_TS_UNPLUG_DBNC_SHIFT: c_int = 3;

pub const CS42L42_HSDET_COMP1_LVL_SHIFT: c_int = 0;

pub const CS42L42_HSDET_COMP2_LVL_SHIFT: c_int = 4;

pub const CS42L42_HSDET_AUTO_TIME_SHIFT: c_int = 0;

pub const CS42L42_HSBIAS_REF_SHIFT: c_int = 3;

pub const CS42L42_HSDET_SET_SHIFT: c_int = 4;

pub const CS42L42_HSDET_CTRL_SHIFT: c_int = 6;

pub const CS42L42_SW_GNDHS_HS4_SHIFT: c_int = 0;

pub const CS42L42_SW_GNDHS_HS3_SHIFT: c_int = 1;

pub const CS42L42_SW_HSB_HS4_SHIFT: c_int = 2;

pub const CS42L42_SW_HSB_HS3_SHIFT: c_int = 3;

pub const CS42L42_SW_HSB_FILT_HS4_SHIFT: c_int = 4;

pub const CS42L42_SW_HSB_FILT_HS3_SHIFT: c_int = 5;

pub const CS42L42_SW_REF_HS4_SHIFT: c_int = 6;

pub const CS42L42_SW_REF_HS3_SHIFT: c_int = 7;

pub const CS42L42_HSDET_TYPE_SHIFT: c_int = 0;

pub const CS42L42_HSDET_COMP1_OUT_SHIFT: c_int = 6;

pub const CS42L42_HSDET_COMP2_OUT_SHIFT: c_int = 7;

pub const CS42L42_PLUG_CTIA: c_int = 0;
pub const CS42L42_PLUG_OMTP: c_int = 1;
pub const CS42L42_PLUG_HEADPHONE: c_int = 2;
pub const CS42L42_PLUG_INVALID: c_int = 3;

pub const CS42L42_HSDET_COMP_TYPE1: c_int = 1;
pub const CS42L42_HSDET_COMP_TYPE2: c_int = 2;
pub const CS42L42_HSDET_COMP_TYPE3: c_int = 0;
pub const CS42L42_HSDET_COMP_TYPE4: c_int = 3;

pub const CS42L42_HS_CLAMP_DISABLE_SHIFT: c_int = 0;

// Page 0x12 Clocking Registers

pub const CS42L42_MCLKDIV_SHIFT: c_int = 1;

pub const CS42L42_MCLK_SRC_SEL_SHIFT: c_int = 0;

pub const CS42L42_FSYNC_PULSE_WIDTH_SHIFT: c_int = 0;

pub const CS42L42_FSYNC_PERIOD_SHIFT: c_int = 0;

pub const CS42L42_ASP_SCLK_EN_SHIFT: c_int = 5;

pub const CS42L42_ASP_MASTER_MODE: c_uint = 0x01;
pub const CS42L42_ASP_SLAVE_MODE: c_uint = 0x00;
pub const CS42L42_ASP_MODE_SHIFT: c_int = 4;

pub const CS42L42_ASP_SCPOL_SHIFT: c_int = 2;

pub const CS42L42_ASP_SCPOL_NOR: c_int = 3;
pub const CS42L42_ASP_LCPOL_SHIFT: c_int = 0;

pub const CS42L42_ASP_LCPOL_INV: c_int = 3;

pub const CS42L42_ASP_STP_SHIFT: c_int = 4;

pub const CS42L42_ASP_5050_SHIFT: c_int = 3;

pub const CS42L42_ASP_FSD_SHIFT: c_int = 0;

pub const CS42L42_ASP_FSD_0_5: c_int = 1;
pub const CS42L42_ASP_FSD_1_0: c_int = 2;
pub const CS42L42_ASP_FSD_1_5: c_int = 3;
pub const CS42L42_ASP_FSD_2_0: c_int = 4;

pub const CS42L42_FS_EN_SHIFT: c_int = 0;

pub const CS42L42_FS_EN_IASRC_96K: c_uint = 0x1;
pub const CS42L42_FS_EN_OASRC_96K: c_uint = 0x2;

pub const CS42L42_CLK_IASRC_SEL_SHIFT: c_int = 0;

pub const CS42L42_CLK_IASRC_SEL_6: c_int = 0;
pub const CS42L42_CLK_IASRC_SEL_12: c_int = 1;

pub const CS42L42_CLK_OASRC_SEL_SHIFT: c_int = 0;

pub const CS42L42_CLK_OASRC_SEL_12: c_int = 1;

pub const CS42L42_SCLK_PREDIV_SHIFT: c_int = 0;

// Page 0x13 Interrupt Registers
// Interrupts

// Masks

pub const CS42L42_ADC_OVFL_SHIFT: c_int = 0;

pub const CS42L42_MIX_CHB_OVFL_SHIFT: c_int = 0;

pub const CS42L42_MIX_CHA_OVFL_SHIFT: c_int = 1;

pub const CS42L42_EQ_OVFL_SHIFT: c_int = 2;

pub const CS42L42_EQ_BIQUAD_OVFL_SHIFT: c_int = 3;

pub const CS42L42_SRC_ILK_SHIFT: c_int = 0;

pub const CS42L42_SRC_OLK_SHIFT: c_int = 1;

pub const CS42L42_SRC_IUNLK_SHIFT: c_int = 2;

pub const CS42L42_SRC_OUNLK_SHIFT: c_int = 3;

pub const CS42L42_ASPRX_NOLRCK_SHIFT: c_int = 0;

pub const CS42L42_ASPRX_EARLY_SHIFT: c_int = 1;

pub const CS42L42_ASPRX_LATE_SHIFT: c_int = 2;

pub const CS42L42_ASPRX_ERROR_SHIFT: c_int = 3;

pub const CS42L42_ASPRX_OVLD_SHIFT: c_int = 4;

pub const CS42L42_ASPTX_NOLRCK_SHIFT: c_int = 0;

pub const CS42L42_ASPTX_EARLY_SHIFT: c_int = 1;

pub const CS42L42_ASPTX_LATE_SHIFT: c_int = 2;

pub const CS42L42_ASPTX_SMERROR_SHIFT: c_int = 3;

pub const CS42L42_PDN_DONE_SHIFT: c_int = 0;

pub const CS42L42_HSDET_AUTO_DONE_SHIFT: c_int = 1;

pub const CS42L42_SRCPL_ADC_LK_SHIFT: c_int = 0;

pub const CS42L42_SRCPL_DAC_LK_SHIFT: c_int = 2;

pub const CS42L42_SRCPL_ADC_UNLK_SHIFT: c_int = 5;

pub const CS42L42_SRCPL_DAC_UNLK_SHIFT: c_int = 6;

pub const CS42L42_VPMON_SHIFT: c_int = 0;

pub const CS42L42_PLL_LOCK_SHIFT: c_int = 0;

pub const CS42L42_RS_PLUG_SHIFT: c_int = 0;

pub const CS42L42_RS_UNPLUG_SHIFT: c_int = 1;

pub const CS42L42_TS_PLUG_SHIFT: c_int = 2;

pub const CS42L42_TS_UNPLUG_SHIFT: c_int = 3;

pub const CS42L42_TS_PLUG: c_int = 3;
pub const CS42L42_TS_UNPLUG: c_int = 0;
pub const CS42L42_TS_TRANS: c_int = 1;
//
// NOTE: PLL_START must be 0 while both ADC_PDN=1 and HP_PDN=1.
// Otherwise it will prevent FILT+ from charging properly.
//

pub const CS42L42_PLL_START_SHIFT: c_int = 0;

pub const CS42L42_PLL_DIV_FRAC_SHIFT: c_int = 0;

pub const CS42L42_PLL_DIV_INT_SHIFT: c_int = 0;

pub const CS42L42_PLL_DIVOUT_SHIFT: c_int = 0;

pub const CS42L42_PLL_CAL_RATIO_SHIFT: c_int = 0;

pub const CS42L42_PLL_MODE_SHIFT: c_int = 0;

// Page 0x19 HP Load Detect Registers

pub const CS42L42_RLA_STAT_SHIFT: c_int = 0;

pub const CS42L42_RLA_STAT_15_OHM: c_int = 0;

pub const CS42L42_HPLOAD_DET_DONE_SHIFT: c_int = 0;

pub const CS42L42_HP_LD_EN_SHIFT: c_int = 0;

// Page 0x1B Headset Interface Registers

pub const CS42L42_HSBIAS_SENSE_TRIP_SHIFT: c_int = 0;

pub const CS42L42_TIP_SENSE_EN_SHIFT: c_int = 5;

pub const CS42L42_AUTO_HSBIAS_HIZ_SHIFT: c_int = 6;

pub const CS42L42_HSBIAS_SENSE_EN_SHIFT: c_int = 7;

pub const CS42L42_WAKEB_CLEAR_SHIFT: c_int = 0;

pub const CS42L42_WAKEB_MODE_SHIFT: c_int = 5;

pub const CS42L42_M_HP_WAKE_SHIFT: c_int = 6;

pub const CS42L42_M_MIC_WAKE_SHIFT: c_int = 7;

pub const CS42L42_ADC_DISABLE_S0_MUTE_SHIFT: c_int = 7;

pub const CS42L42_TIP_SENSE_DEBOUNCE_SHIFT: c_int = 0;

pub const CS42L42_TIP_SENSE_INV_SHIFT: c_int = 5;

pub const CS42L42_TIP_SENSE_CTRL_SHIFT: c_int = 6;

//
// NOTE: DETECT_MODE must be 0 while both ADC_PDN=1 and HP_PDN=1.
// Otherwise it will prevent FILT+ from charging properly.
//

pub const CS42L42_PDN_MIC_LVL_DET_SHIFT: c_int = 0;

pub const CS42L42_HSBIAS_CTL_SHIFT: c_int = 1;

pub const CS42L42_DETECT_MODE_SHIFT: c_int = 3;

pub const CS42L42_HS_DET_LEVEL_SHIFT: c_int = 0;

pub const CS42L42_EVENT_STAT_SEL_SHIFT: c_int = 6;

pub const CS42L42_LATCH_TO_VP_SHIFT: c_int = 7;

pub const CS42L42_DEBOUNCE_TIME_SHIFT: c_int = 5;

pub const CS42L42_HSBIAS_HIZ_MODE_SHIFT: c_int = 6;

pub const CS42L42_TIP_SENSE_SHIFT: c_int = 7;

pub const CS42L42_SHORT_TRUE_SHIFT: c_int = 0;

pub const CS42L42_HS_TRUE_SHIFT: c_int = 1;

pub const CS42L42_TIP_SENSE_UNPLUG_SHIFT: c_int = 5;

pub const CS42L42_TIP_SENSE_PLUG_SHIFT: c_int = 6;

pub const CS42L42_HSBIAS_SENSE_SHIFT: c_int = 7;

pub const CS42L42_M_SHORT_DET_SHIFT: c_int = 0;

pub const CS42L42_M_SHORT_RLS_SHIFT: c_int = 1;

pub const CS42L42_M_HSBIAS_HIZ_SHIFT: c_int = 2;

pub const CS42L42_M_DETECT_FT_SHIFT: c_int = 6;

pub const CS42L42_M_DETECT_TF_SHIFT: c_int = 7;

// Page 0x1C Headset Bias Registers

pub const CS42L42_HSBIAS_RAMP_SHIFT: c_int = 0;

pub const CS42L42_HSBIAS_PD_SHIFT: c_int = 4;

pub const CS42L42_HSBIAS_CAPLESS_SHIFT: c_int = 7;

// Page 0x1D ADC Registers

pub const CS42L42_ADC_NOTCH_DIS_SHIFT: c_int = 5;
pub const CS42L42_ADC_FORCE_WEAK_VCM_SHIFT: c_int = 4;
pub const CS42L42_ADC_INV_SHIFT: c_int = 2;
pub const CS42L42_ADC_DIG_BOOST_SHIFT: c_int = 0;

pub const CS42L42_ADC_VOL_SHIFT: c_int = 0;

pub const CS42L42_ADC_WNF_CF_SHIFT: c_int = 4;
pub const CS42L42_ADC_WNF_EN_SHIFT: c_int = 3;
pub const CS42L42_ADC_HPF_CF_SHIFT: c_int = 1;
pub const CS42L42_ADC_HPF_EN_SHIFT: c_int = 0;
// Page 0x1F DAC Registers

pub const CS42L42_DACB_INV_SHIFT: c_int = 1;
pub const CS42L42_DACA_INV_SHIFT: c_int = 0;

pub const CS42L42_HPOUT_PULLDOWN_SHIFT: c_int = 4;

pub const CS42L42_HPOUT_LOAD_SHIFT: c_int = 3;

pub const CS42L42_HPOUT_CLAMP_SHIFT: c_int = 2;

pub const CS42L42_DAC_HPF_EN_SHIFT: c_int = 1;

pub const CS42L42_DAC_MON_EN_SHIFT: c_int = 0;

// Page 0x20 HP CTL Registers

pub const CS42L42_HP_ANA_BMUTE_SHIFT: c_int = 3;

pub const CS42L42_HP_ANA_AMUTE_SHIFT: c_int = 2;

pub const CS42L42_HP_FULL_SCALE_VOL_SHIFT: c_int = 1;

// Page 0x21 Class H Registers

// Page 0x23 Mixer Volume Registers

pub const CS42L42_MIXER_CH_VOL_SHIFT: c_int = 0;

// Page 0x24 EQ Registers

// Page 0x25 Audio Port Registers

pub const CS42L42_SP_RX_CHB_SEL_SHIFT: c_int = 2;

pub const CS42L42_SP_RX_RSYNC_SHIFT: c_int = 6;

pub const CS42L42_SP_RX_NSB_POS_SHIFT: c_int = 3;

pub const CS42L42_SP_RX_NFS_NSBB_SHIFT: c_int = 2;

pub const CS42L42_SP_RX_ISOC_MODE_SHIFT: c_int = 0;

// Page 0x26 SRC Registers

pub const CS42L42_SRC_SDIN_FS_SHIFT: c_int = 0;

// Page 0x27 DMA

// Page 0x28 S/PDIF Registers

// Page 0x29 Serial Port TX Registers

pub const CS42L42_ASP_TX_EN_SHIFT: c_int = 0;

pub const CS42L42_ASP_TX0_CH2_SHIFT: c_int = 1;
pub const CS42L42_ASP_TX0_CH1_SHIFT: c_int = 0;

pub const CS42L42_ASP_TX_CH1_AP_SHIFT: c_int = 7;

pub const CS42L42_ASP_TX_CH2_AP_SHIFT: c_int = 6;

pub const CS42L42_ASP_TX_CH2_RES_SHIFT: c_int = 2;

pub const CS42L42_ASP_TX_CH1_RES_SHIFT: c_int = 0;

// Page 0x2A Serial Port RX Registers

pub const CS42L42_ASP_RX0_CH_EN_SHIFT: c_int = 2;

pub const CS42L42_ASP_RX0_CH1_SHIFT: c_int = 2;
pub const CS42L42_ASP_RX0_CH2_SHIFT: c_int = 3;
pub const CS42L42_ASP_RX0_CH3_SHIFT: c_int = 4;
pub const CS42L42_ASP_RX0_CH4_SHIFT: c_int = 5;

pub const CS42L42_ASP_RX_CH_AP_SHIFT: c_int = 6;

pub const CS42L42_ASP_RX_CH_AP_LOW: c_int = 0;
pub const CS42L42_ASP_RX_CH_AP_HI: c_int = 1;
pub const CS42L42_ASP_RX_CH_RES_SHIFT: c_int = 0;

pub const CS42L42_ASP_RX_CH_RES_32: c_int = 3;
pub const CS42L42_ASP_RX_CH_RES_16: c_int = 1;
pub const CS42L42_ASP_RX_CH_BIT_ST_SHIFT: c_int = 0;

// Page 0x30 ID Registers

// Defines for fracturing values spread across multiple registers

pub const CS42L42_NUM_SUPPLIES: c_int = 5;
pub const CS42L42_BOOT_TIME_US: c_int = 3000;
pub const CS42L42_PLL_DIVOUT_TIME_US: c_int = 800;
pub const CS42L42_CLOCK_SWITCH_DELAY_US: c_int = 150;
pub const CS42L42_PLL_LOCK_POLL_US: c_int = 250;
pub const CS42L42_PLL_LOCK_TIMEOUT_US: c_int = 1250;
pub const CS42L42_HP_ADC_EN_TIME_US: c_int = 20000;
pub const CS42L42_PDN_DONE_POLL_US: c_int = 1000;
pub const CS42L42_PDN_DONE_TIMEOUT_US: c_int = 235000;
pub const CS42L42_PDN_DONE_TIME_MS: c_int = 65;
