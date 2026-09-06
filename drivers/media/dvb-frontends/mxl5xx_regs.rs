//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mxl5xx_regs.h
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
// Copyright (c) 2011-2013 MaxLinear, Inc. All rights reserved
//
// This program may alternatively be licensed under a proprietary license from
// MaxLinear, Inc.
//
pub const HYDRA_INTR_STATUS_REG: c_uint = 0x80030008;
pub const HYDRA_INTR_MASK_REG: c_uint = 0x8003000C;
pub const HYDRA_CRYSTAL_SETTING: c_uint = 0x3FFFC5F0 /* 0 - 24 MHz & 1 - 27 MHz */;
pub const HYDRA_CRYSTAL_CAP: c_uint = 0x3FFFEDA4 /* 0 - 24 MHz & 1 - 27 MHz */;
pub const HYDRA_CPU_RESET_REG: c_uint = 0x8003003C;
pub const HYDRA_CPU_RESET_DATA: c_uint = 0x00000400;
pub const HYDRA_RESET_TRANSPORT_FIFO_REG: c_uint = 0x80030028;
pub const HYDRA_RESET_TRANSPORT_FIFO_DATA: c_uint = 0x00000000;
pub const HYDRA_RESET_BBAND_REG: c_uint = 0x80030024;
pub const HYDRA_RESET_BBAND_DATA: c_uint = 0x00000000;
pub const HYDRA_RESET_XBAR_REG: c_uint = 0x80030020;
pub const HYDRA_RESET_XBAR_DATA: c_uint = 0x00000000;
pub const HYDRA_MODULES_CLK_1_REG: c_uint = 0x80030014;
pub const HYDRA_DISABLE_CLK_1: c_uint = 0x00000000;
pub const HYDRA_MODULES_CLK_2_REG: c_uint = 0x8003001C;
pub const HYDRA_DISABLE_CLK_2: c_uint = 0x0000000B;
pub const HYDRA_PRCM_ROOT_CLK_REG: c_uint = 0x80030018;
pub const HYDRA_PRCM_ROOT_CLK_DISABLE: c_uint = 0x00000000;
pub const HYDRA_CPU_RESET_CHECK_REG: c_uint = 0x80030008;
pub const HYDRA_CPU_RESET_CHECK_OFFSET: c_uint = 0x40000000  /* <bit 30> */;
pub const HYDRA_SKU_ID_REG: c_uint = 0x90000190;
pub const FW_DL_SIGN_ADDR: c_uint = 0x3FFFEAE0;
// Register to check if FW is running or not
pub const HYDRA_HEAR_BEAT: c_uint = 0x3FFFEDDC;
// Firmware version
pub const HYDRA_FIRMWARE_VERSION: c_uint = 0x3FFFEDB8;
pub const HYDRA_FW_RC_VERSION: c_uint = 0x3FFFCFAC;
// Firmware patch version
pub const HYDRA_FIRMWARE_PATCH_VERSION: c_uint = 0x3FFFEDC2;
// SOC operating temperature in C
pub const HYDRA_TEMPARATURE: c_uint = 0x3FFFEDB4;
// Demod & Tuner status registers
// Demod 0 status base address
pub const HYDRA_DEMOD_0_BASE_ADDR: c_uint = 0x3FFFC64C;
// Tuner 0 status base address
pub const HYDRA_TUNER_0_BASE_ADDR: c_uint = 0x3FFFCE4C;
pub const POWER_FROM_ADCRSSI_READBACK: c_uint = 0x3FFFEB6C;
// Macros to determine base address of respective demod or tuner

// Demod status address offset from respective demod's base address
pub const HYDRA_DMD_AGC_DIG_LEVEL_ADDR_OFFSET: c_uint = 0x3FFFC64C;
pub const HYDRA_DMD_LOCK_STATUS_ADDR_OFFSET: c_uint = 0x3FFFC650;
pub const HYDRA_DMD_ACQ_STATUS_ADDR_OFFSET: c_uint = 0x3FFFC654;
pub const HYDRA_DMD_STANDARD_ADDR_OFFSET: c_uint = 0x3FFFC658;
pub const HYDRA_DMD_SPECTRUM_INVERSION_ADDR_OFFSET: c_uint = 0x3FFFC65C;
pub const HYDRA_DMD_SPECTRUM_ROLL_OFF_ADDR_OFFSET: c_uint = 0x3FFFC660;
pub const HYDRA_DMD_SYMBOL_RATE_ADDR_OFFSET: c_uint = 0x3FFFC664;
pub const HYDRA_DMD_MODULATION_SCHEME_ADDR_OFFSET: c_uint = 0x3FFFC668;
pub const HYDRA_DMD_FEC_CODE_RATE_ADDR_OFFSET: c_uint = 0x3FFFC66C;
pub const HYDRA_DMD_SNR_ADDR_OFFSET: c_uint = 0x3FFFC670;
pub const HYDRA_DMD_FREQ_OFFSET_ADDR_OFFSET: c_uint = 0x3FFFC674;
pub const HYDRA_DMD_CTL_FREQ_OFFSET_ADDR_OFFSET: c_uint = 0x3FFFC678;
pub const HYDRA_DMD_STR_FREQ_OFFSET_ADDR_OFFSET: c_uint = 0x3FFFC67C;
pub const HYDRA_DMD_FTL_FREQ_OFFSET_ADDR_OFFSET: c_uint = 0x3FFFC680;
pub const HYDRA_DMD_STR_NBC_SYNC_LOCK_ADDR_OFFSET: c_uint = 0x3FFFC684;
pub const HYDRA_DMD_CYCLE_SLIP_COUNT_ADDR_OFFSET: c_uint = 0x3FFFC688;
pub const HYDRA_DMD_DISPLAY_I_ADDR_OFFSET: c_uint = 0x3FFFC68C;
pub const HYDRA_DMD_DISPLAY_Q_ADDR_OFFSET: c_uint = 0x3FFFC68E;
pub const HYDRA_DMD_DVBS2_CRC_ERRORS_ADDR_OFFSET: c_uint = 0x3FFFC690;
pub const HYDRA_DMD_DVBS2_PER_COUNT_ADDR_OFFSET: c_uint = 0x3FFFC694;
pub const HYDRA_DMD_DVBS2_PER_WINDOW_ADDR_OFFSET: c_uint = 0x3FFFC698;
pub const HYDRA_DMD_DVBS_CORR_RS_ERRORS_ADDR_OFFSET: c_uint = 0x3FFFC69C;
pub const HYDRA_DMD_DVBS_UNCORR_RS_ERRORS_ADDR_OFFSET: c_uint = 0x3FFFC6A0;
pub const HYDRA_DMD_DVBS_BER_COUNT_ADDR_OFFSET: c_uint = 0x3FFFC6A4;
pub const HYDRA_DMD_DVBS_BER_WINDOW_ADDR_OFFSET: c_uint = 0x3FFFC6A8;
// Debug-purpose DVB-S DMD 0
pub const HYDRA_DMD_DVBS_1ST_CORR_RS_ERRORS_ADDR_OFFSET: c_uint = 0x3FFFC6C8  /* corrected RS Errors: 1st iteration */;
pub const HYDRA_DMD_DVBS_1ST_UNCORR_RS_ERRORS_ADDR_OFFSET: c_uint = 0x3FFFC6CC  /* uncorrected RS Errors: 1st iteration */;
pub const HYDRA_DMD_DVBS_BER_COUNT_1ST_ADDR_OFFSET: c_uint = 0x3FFFC6D0;
pub const HYDRA_DMD_DVBS_BER_WINDOW_1ST_ADDR_OFFSET: c_uint = 0x3FFFC6D4;
pub const HYDRA_DMD_TUNER_ID_ADDR_OFFSET: c_uint = 0x3FFFC6AC;
pub const HYDRA_DMD_DVBS2_PILOT_ON_OFF_ADDR_OFFSET: c_uint = 0x3FFFC6B0;
pub const HYDRA_DMD_FREQ_SEARCH_RANGE_KHZ_ADDR_OFFSET: c_uint = 0x3FFFC6B4;
pub const HYDRA_DMD_STATUS_LOCK_ADDR_OFFSET: c_uint = 0x3FFFC6B8;
pub const HYDRA_DMD_STATUS_CENTER_FREQ_IN_KHZ_ADDR: c_uint = 0x3FFFC704;
pub const HYDRA_DMD_STATUS_INPUT_POWER_ADDR: c_uint = 0x3FFFC708;
// DVB-S new scaled_BER_count for a new BER API, see HYDRA-1343 "DVB-S post viterbi information"
pub const DMD0_STATUS_DVBS_1ST_SCALED_BER_COUNT_ADDR: c_uint = 0x3FFFC710 /* DMD 0: 1st iteration BER count scaled by HYDRA_BER_COUNT_SCALING_FACTOR */;
pub const DMD0_STATUS_DVBS_SCALED_BER_COUNT_ADDR: c_uint = 0x3FFFC714 /* DMD 0: 2nd iteration BER count scaled by HYDRA_BER_COUNT_SCALING_FACTOR */;
pub const DMD0_SPECTRUM_MIN_GAIN_STATUS: c_uint = 0x3FFFC73C;
pub const DMD0_SPECTRUM_MIN_GAIN_WB_SAGC_VALUE: c_uint = 0x3FFFC740;
pub const DMD0_SPECTRUM_MIN_GAIN_NB_SAGC_VALUE: c_uint = 0x3FFFC744;
pub const HYDRA_DMD_STATUS_END_ADDR_OFFSET: c_uint = 0x3FFFC748;
// Tuner status address offset from respective tuners's base address
pub const HYDRA_TUNER_DEMOD_ID_ADDR_OFFSET: c_uint = 0x3FFFCE4C;
pub const HYDRA_TUNER_AGC_LOCK_OFFSET: c_uint = 0x3FFFCE50;
pub const HYDRA_TUNER_SPECTRUM_STATUS_OFFSET: c_uint = 0x3FFFCE54;
pub const HYDRA_TUNER_SPECTRUM_BIN_SIZE_OFFSET: c_uint = 0x3FFFCE58;
pub const HYDRA_TUNER_SPECTRUM_ADDRESS_OFFSET: c_uint = 0x3FFFCE5C;
pub const HYDRA_TUNER_ENABLE_COMPLETE: c_uint = 0x3FFFEB78;

pub const HYDRA_VERSION: c_uint = 0x3FFFEDB8;
pub const HYDRA_DEMOD0_VERSION: c_uint = 0x3FFFEDBC;
pub const HYDRA_DEMOD1_VERSION: c_uint = 0x3FFFEDC0;
pub const HYDRA_DEMOD2_VERSION: c_uint = 0x3FFFEDC4;
pub const HYDRA_DEMOD3_VERSION: c_uint = 0x3FFFEDC8;
pub const HYDRA_DEMOD4_VERSION: c_uint = 0x3FFFEDCC;
pub const HYDRA_DEMOD5_VERSION: c_uint = 0x3FFFEDD0;
pub const HYDRA_DEMOD6_VERSION: c_uint = 0x3FFFEDD4;
pub const HYDRA_DEMOD7_VERSION: c_uint = 0x3FFFEDD8;
pub const HYDRA_HEAR_BEAT: c_uint = 0x3FFFEDDC;
pub const HYDRA_SKU_MGMT: c_uint = 0x3FFFEBC0;
pub const MXL_HYDRA_FPGA_A_ADDRESS: c_uint = 0x91C00000;
pub const MXL_HYDRA_FPGA_B_ADDRESS: c_uint = 0x91D00000;
// TS control base address
pub const HYDRA_TS_CTRL_BASE_ADDR: c_uint = 0x90700000;

//
pub const PAD_MUX_GPIO_00_SYNC_BASEADDR: c_uint = 0x90000188;
pub const PAD_MUX_UART_RX_C_PINMUX_BASEADDR: c_uint = 0x9000001C;
pub const XPT_PACKET_GAP_MIN_BASEADDR: c_uint = 0x90700044;
pub const XPT_NCO_COUNT_BASEADDR: c_uint = 0x90700238;
pub const XPT_NCO_COUNT_BASEADDR1: c_uint = 0x9070023C;
// V2 DigRF status register
pub const XPT_PID_BASEADDR: c_uint = 0x90708000;
pub const XPT_PID_REMAP_BASEADDR: c_uint = 0x90708004;
pub const XPT_KNOWN_PID_BASEADDR: c_uint = 0x90709000;
pub const XPT_PID_BASEADDR1: c_uint = 0x9070A000;
pub const XPT_PID_REMAP_BASEADDR1: c_uint = 0x9070A004;
pub const XPT_KNOWN_PID_BASEADDR1: c_uint = 0x9070B000;
pub const XPT_BERT_LOCK_BASEADDR: c_uint = 0x907000B8;
pub const XPT_BERT_BASEADDR: c_uint = 0x907000BC;
pub const XPT_BERT_INVERT_BASEADDR: c_uint = 0x907000C0;
pub const XPT_BERT_HEADER_BASEADDR: c_uint = 0x907000C4;
pub const XPT_BERT_BASEADDR1: c_uint = 0x907000C8;
pub const XPT_BERT_BIT_COUNT0_BASEADDR: c_uint = 0x907000CC;
pub const XPT_BERT_BIT_COUNT0_BASEADDR1: c_uint = 0x907000D0;
pub const XPT_BERT_BIT_COUNT1_BASEADDR: c_uint = 0x907000D4;
pub const XPT_BERT_BIT_COUNT1_BASEADDR1: c_uint = 0x907000D8;
pub const XPT_BERT_BIT_COUNT2_BASEADDR: c_uint = 0x907000DC;
pub const XPT_BERT_BIT_COUNT2_BASEADDR1: c_uint = 0x907000E0;
pub const XPT_BERT_BIT_COUNT3_BASEADDR: c_uint = 0x907000E4;
pub const XPT_BERT_BIT_COUNT3_BASEADDR1: c_uint = 0x907000E8;
pub const XPT_BERT_BIT_COUNT4_BASEADDR: c_uint = 0x907000EC;
pub const XPT_BERT_BIT_COUNT4_BASEADDR1: c_uint = 0x907000F0;
pub const XPT_BERT_BIT_COUNT5_BASEADDR: c_uint = 0x907000F4;
pub const XPT_BERT_BIT_COUNT5_BASEADDR1: c_uint = 0x907000F8;
pub const XPT_BERT_BIT_COUNT6_BASEADDR: c_uint = 0x907000FC;
pub const XPT_BERT_BIT_COUNT6_BASEADDR1: c_uint = 0x90700100;
pub const XPT_BERT_BIT_COUNT7_BASEADDR: c_uint = 0x90700104;
pub const XPT_BERT_BIT_COUNT7_BASEADDR1: c_uint = 0x90700108;
pub const XPT_BERT_ERR_COUNT0_BASEADDR: c_uint = 0x9070010C;
pub const XPT_BERT_ERR_COUNT0_BASEADDR1: c_uint = 0x90700110;
pub const XPT_BERT_ERR_COUNT1_BASEADDR: c_uint = 0x90700114;
pub const XPT_BERT_ERR_COUNT1_BASEADDR1: c_uint = 0x90700118;
pub const XPT_BERT_ERR_COUNT2_BASEADDR: c_uint = 0x9070011C;
pub const XPT_BERT_ERR_COUNT2_BASEADDR1: c_uint = 0x90700120;
pub const XPT_BERT_ERR_COUNT3_BASEADDR: c_uint = 0x90700124;
pub const XPT_BERT_ERR_COUNT3_BASEADDR1: c_uint = 0x90700128;
pub const XPT_BERT_ERR_COUNT4_BASEADDR: c_uint = 0x9070012C;
pub const XPT_BERT_ERR_COUNT4_BASEADDR1: c_uint = 0x90700130;
pub const XPT_BERT_ERR_COUNT5_BASEADDR: c_uint = 0x90700134;
pub const XPT_BERT_ERR_COUNT5_BASEADDR1: c_uint = 0x90700138;
pub const XPT_BERT_ERR_COUNT6_BASEADDR: c_uint = 0x9070013C;
pub const XPT_BERT_ERR_COUNT6_BASEADDR1: c_uint = 0x90700140;
pub const XPT_BERT_ERR_COUNT7_BASEADDR: c_uint = 0x90700144;
pub const XPT_BERT_ERR_COUNT7_BASEADDR1: c_uint = 0x90700148;
pub const XPT_BERT_ERROR_BASEADDR: c_uint = 0x9070014C;
pub const XPT_BERT_ANALYZER_BASEADDR: c_uint = 0x90700150;
pub const XPT_BERT_ANALYZER_BASEADDR1: c_uint = 0x90700154;
pub const XPT_BERT_ANALYZER_BASEADDR2: c_uint = 0x90700158;
pub const XPT_BERT_ANALYZER_BASEADDR3: c_uint = 0x9070015C;
pub const XPT_BERT_ANALYZER_BASEADDR4: c_uint = 0x90700160;
pub const XPT_BERT_ANALYZER_BASEADDR5: c_uint = 0x90700164;
pub const XPT_BERT_ANALYZER_BASEADDR6: c_uint = 0x90700168;
pub const XPT_BERT_ANALYZER_BASEADDR7: c_uint = 0x9070016C;
pub const XPT_BERT_ANALYZER_BASEADDR8: c_uint = 0x90700170;
pub const XPT_BERT_ANALYZER_BASEADDR9: c_uint = 0x90700174;
pub const XPT_DMD0_BASEADDR: c_uint = 0x9070024C;
// V2 AGC Gain Freeze & step

pub const WB_DFE0_DFE_FB_RF1_BASEADDR: c_uint = 0x903004A4;
pub const WB_DFE1_DFE_FB_RF1_BASEADDR: c_uint = 0x904004A4;
pub const WB_DFE2_DFE_FB_RF1_BASEADDR: c_uint = 0x905004A4;
pub const WB_DFE3_DFE_FB_RF1_BASEADDR: c_uint = 0x906004A4;
pub const AFE_REG_D2A_TA_RFFE_LNA_BO_1P8_BASEADDR: c_uint = 0x90200104;
pub const AFE_REG_AFE_REG_SPARE_BASEADDR: c_uint = 0x902000A0;
pub const AFE_REG_AFE_REG_SPARE_BASEADDR1: c_uint = 0x902000B4;
pub const AFE_REG_AFE_REG_SPARE_BASEADDR2: c_uint = 0x902000C4;
pub const AFE_REG_AFE_REG_SPARE_BASEADDR3: c_uint = 0x902000D4;
pub const WB_DFE0_DFE_FB_AGC_BASEADDR: c_uint = 0x90300498;
pub const WB_DFE1_DFE_FB_AGC_BASEADDR: c_uint = 0x90400498;
pub const WB_DFE2_DFE_FB_AGC_BASEADDR: c_uint = 0x90500498;
pub const WB_DFE3_DFE_FB_AGC_BASEADDR: c_uint = 0x90600498;
pub const WDT_WD_INT_BASEADDR: c_uint = 0x8002000C;
pub const FSK_TX_FTM_BASEADDR: c_uint = 0x80090000;
pub const FSK_TX_FTM_TX_CNT_BASEADDR: c_uint = 0x80090018;
pub const AFE_REG_D2A_FSK_BIAS_BASEADDR: c_uint = 0x90200040;
pub const DMD_TEI_BASEADDR: c_uint = 0x3FFFEBE0;
