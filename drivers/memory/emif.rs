//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/emif.h
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
// Defines for the EMIF driver
//
// Copyright (C) 2012 Texas Instruments, Inc.
//
// Benoit Cousson (b-cousson@ti.com)
//
// Maximum number of different frequencies supported by EMIF driver
// Determines the number of entries in the pointer array for register
// cache
//
pub const EMIF_MAX_NUM_FREQUENCIES: c_int = 6;
// State of the core voltage
pub const DDR_VOLTAGE_STABLE: c_int = 0;
pub const DDR_VOLTAGE_RAMPING: c_int = 1;
// Defines for timing De-rating
pub const EMIF_NORMAL_TIMINGS: c_int = 0;
pub const EMIF_DERATED_TIMINGS: c_int = 1;
// Length of the forced read idle period in terms of cycles
pub const EMIF_READ_IDLE_LEN_VAL: c_int = 5;
//
// forced read idle interval to be used when voltage
// is changed as part of DVFS/DPS - 1ms
//

//
// Forced read idle interval to be used when voltage is stable
// 50us - or maximum value will do
//

// DLL calibration interval when voltage is NOT stable - 1us

pub const DLL_CALIB_ACK_WAIT_VAL: c_int = 5;
// Interval between ZQCS commands - hw team recommended value

// Enable ZQ Calibration on exiting Self-refresh
pub const ZQ_SFEXITEN_ENABLE: c_int = 1;
//
// ZQ Calibration simultaneously on both chip-selects:
// Needs one calibration resistor per CS
//
pub const ZQ_DUALCALEN_DISABLE: c_int = 0;
pub const ZQ_DUALCALEN_ENABLE: c_int = 1;
pub const T_ZQCS_DEFAULT_NS: c_int = 90;
pub const T_ZQCL_DEFAULT_NS: c_int = 360;
pub const T_ZQINIT_DEFAULT_NS: c_int = 1000;
// DPD_EN
pub const DPD_DISABLE: c_int = 0;
pub const DPD_ENABLE: c_int = 1;
//
// Default values for the low-power entry to be used if not provided by user.
// OMAP4/5 has a hw bug(i735) due to which this value can not be less than 512
// Timeout values are in DDR clock 'cycles' and frequency threshold in Hz
//
pub const EMIF_LP_MODE_TIMEOUT_PERFORMANCE: c_int = 2048;
pub const EMIF_LP_MODE_TIMEOUT_POWER: c_int = 512;
pub const EMIF_LP_MODE_FREQ_THRESHOLD: c_int = 400000000;
// DDR_PHY_CTRL_1 values for EMIF4D - ATTILA PHY combination
pub const EMIF_DDR_PHY_CTRL_1_BASE_VAL_ATTILAPHY: c_uint = 0x049FF000;
pub const EMIF_DLL_SLAVE_DLY_CTRL_400_MHZ_ATTILAPHY: c_uint = 0x41;
pub const EMIF_DLL_SLAVE_DLY_CTRL_200_MHZ_ATTILAPHY: c_uint = 0x80;
pub const EMIF_DLL_SLAVE_DLY_CTRL_100_MHZ_AND_LESS_ATTILAPHY: c_uint = 0xFF;
// DDR_PHY_CTRL_1 values for EMIF4D5 INTELLIPHY combination
pub const EMIF_DDR_PHY_CTRL_1_BASE_VAL_INTELLIPHY: c_uint = 0x0E084200;
pub const EMIF_PHY_TOTAL_READ_LATENCY_INTELLIPHY_PS: c_int = 10000;
// TEMP_ALERT_CONFIG - corresponding to temp gradient 5 C/s
pub const TEMP_ALERT_POLL_INTERVAL_DEFAULT_MS: c_int = 360;
pub const EMIF_T_CSTA: c_int = 3;
pub const EMIF_T_PDLL_UL: c_int = 128;
// External PHY control registers magic values
pub const EMIF_EXT_PHY_CTRL_1_VAL: c_uint = 0x04020080;
pub const EMIF_EXT_PHY_CTRL_5_VAL: c_uint = 0x04010040;
pub const EMIF_EXT_PHY_CTRL_6_VAL: c_uint = 0x01004010;
pub const EMIF_EXT_PHY_CTRL_7_VAL: c_uint = 0x00001004;
pub const EMIF_EXT_PHY_CTRL_8_VAL: c_uint = 0x04010040;
pub const EMIF_EXT_PHY_CTRL_9_VAL: c_uint = 0x01004010;
pub const EMIF_EXT_PHY_CTRL_10_VAL: c_uint = 0x00001004;
pub const EMIF_EXT_PHY_CTRL_11_VAL: c_uint = 0x00000000;
pub const EMIF_EXT_PHY_CTRL_12_VAL: c_uint = 0x00000000;
pub const EMIF_EXT_PHY_CTRL_13_VAL: c_uint = 0x00000000;
pub const EMIF_EXT_PHY_CTRL_14_VAL: c_uint = 0x80080080;
pub const EMIF_EXT_PHY_CTRL_15_VAL: c_uint = 0x00800800;
pub const EMIF_EXT_PHY_CTRL_16_VAL: c_uint = 0x08102040;
pub const EMIF_EXT_PHY_CTRL_17_VAL: c_uint = 0x00000001;
pub const EMIF_EXT_PHY_CTRL_18_VAL: c_uint = 0x540A8150;
pub const EMIF_EXT_PHY_CTRL_19_VAL: c_uint = 0xA81502A0;
pub const EMIF_EXT_PHY_CTRL_20_VAL: c_uint = 0x002A0540;
pub const EMIF_EXT_PHY_CTRL_21_VAL: c_uint = 0x00000000;
pub const EMIF_EXT_PHY_CTRL_22_VAL: c_uint = 0x00000000;
pub const EMIF_EXT_PHY_CTRL_23_VAL: c_uint = 0x00000000;
pub const EMIF_EXT_PHY_CTRL_24_VAL: c_uint = 0x00000077;
pub const EMIF_INTELLI_PHY_DQS_GATE_OPENING_DELAY_PS: c_int = 1200;
// Registers offset
pub const EMIF_MODULE_ID_AND_REVISION: c_uint = 0x0000;
pub const EMIF_STATUS: c_uint = 0x0004;
pub const EMIF_SDRAM_CONFIG: c_uint = 0x0008;
pub const EMIF_SDRAM_CONFIG_2: c_uint = 0x000c;
pub const EMIF_SDRAM_REFRESH_CONTROL: c_uint = 0x0010;
pub const EMIF_SDRAM_REFRESH_CTRL_SHDW: c_uint = 0x0014;
pub const EMIF_SDRAM_TIMING_1: c_uint = 0x0018;
pub const EMIF_SDRAM_TIMING_1_SHDW: c_uint = 0x001c;
pub const EMIF_SDRAM_TIMING_2: c_uint = 0x0020;
pub const EMIF_SDRAM_TIMING_2_SHDW: c_uint = 0x0024;
pub const EMIF_SDRAM_TIMING_3: c_uint = 0x0028;
pub const EMIF_SDRAM_TIMING_3_SHDW: c_uint = 0x002c;
pub const EMIF_LPDDR2_NVM_TIMING: c_uint = 0x0030;
pub const EMIF_LPDDR2_NVM_TIMING_SHDW: c_uint = 0x0034;
pub const EMIF_POWER_MANAGEMENT_CONTROL: c_uint = 0x0038;
pub const EMIF_POWER_MANAGEMENT_CTRL_SHDW: c_uint = 0x003c;
pub const EMIF_LPDDR2_MODE_REG_DATA: c_uint = 0x0040;
pub const EMIF_LPDDR2_MODE_REG_CONFIG: c_uint = 0x0050;
pub const EMIF_OCP_CONFIG: c_uint = 0x0054;
pub const EMIF_OCP_CONFIG_VALUE_1: c_uint = 0x0058;
pub const EMIF_OCP_CONFIG_VALUE_2: c_uint = 0x005c;
pub const EMIF_IODFT_TEST_LOGIC_GLOBAL_CONTROL: c_uint = 0x0060;
pub const EMIF_IODFT_TEST_LOGIC_CTRL_MISR_RESULT: c_uint = 0x0064;
pub const EMIF_IODFT_TEST_LOGIC_ADDRESS_MISR_RESULT: c_uint = 0x0068;
pub const EMIF_IODFT_TEST_LOGIC_DATA_MISR_RESULT_1: c_uint = 0x006c;
pub const EMIF_IODFT_TEST_LOGIC_DATA_MISR_RESULT_2: c_uint = 0x0070;
pub const EMIF_IODFT_TEST_LOGIC_DATA_MISR_RESULT_3: c_uint = 0x0074;
pub const EMIF_PERFORMANCE_COUNTER_1: c_uint = 0x0080;
pub const EMIF_PERFORMANCE_COUNTER_2: c_uint = 0x0084;
pub const EMIF_PERFORMANCE_COUNTER_CONFIG: c_uint = 0x0088;
pub const EMIF_PERFORMANCE_COUNTER_MASTER_REGION_SELECT: c_uint = 0x008c;
pub const EMIF_PERFORMANCE_COUNTER_TIME: c_uint = 0x0090;
pub const EMIF_MISC_REG: c_uint = 0x0094;
pub const EMIF_DLL_CALIB_CTRL: c_uint = 0x0098;
pub const EMIF_DLL_CALIB_CTRL_SHDW: c_uint = 0x009c;
pub const EMIF_END_OF_INTERRUPT: c_uint = 0x00a0;
pub const EMIF_SYSTEM_OCP_INTERRUPT_RAW_STATUS: c_uint = 0x00a4;
pub const EMIF_LL_OCP_INTERRUPT_RAW_STATUS: c_uint = 0x00a8;
pub const EMIF_SYSTEM_OCP_INTERRUPT_STATUS: c_uint = 0x00ac;
pub const EMIF_LL_OCP_INTERRUPT_STATUS: c_uint = 0x00b0;
pub const EMIF_SYSTEM_OCP_INTERRUPT_ENABLE_SET: c_uint = 0x00b4;
pub const EMIF_LL_OCP_INTERRUPT_ENABLE_SET: c_uint = 0x00b8;
pub const EMIF_SYSTEM_OCP_INTERRUPT_ENABLE_CLEAR: c_uint = 0x00bc;
pub const EMIF_LL_OCP_INTERRUPT_ENABLE_CLEAR: c_uint = 0x00c0;
pub const EMIF_SDRAM_OUTPUT_IMPEDANCE_CALIBRATION_CONFIG: c_uint = 0x00c8;
pub const EMIF_TEMPERATURE_ALERT_CONFIG: c_uint = 0x00cc;
pub const EMIF_OCP_ERROR_LOG: c_uint = 0x00d0;
pub const EMIF_READ_WRITE_LEVELING_RAMP_WINDOW: c_uint = 0x00d4;
pub const EMIF_READ_WRITE_LEVELING_RAMP_CONTROL: c_uint = 0x00d8;
pub const EMIF_READ_WRITE_LEVELING_CONTROL: c_uint = 0x00dc;
pub const EMIF_DDR_PHY_CTRL_1: c_uint = 0x00e4;
pub const EMIF_DDR_PHY_CTRL_1_SHDW: c_uint = 0x00e8;
pub const EMIF_DDR_PHY_CTRL_2: c_uint = 0x00ec;
pub const EMIF_PRIORITY_TO_CLASS_OF_SERVICE_MAPPING: c_uint = 0x0100;
pub const EMIF_CONNECTION_ID_TO_CLASS_OF_SERVICE_1_MAPPING: c_uint = 0x0104;
pub const EMIF_CONNECTION_ID_TO_CLASS_OF_SERVICE_2_MAPPING: c_uint = 0x0108;
pub const EMIF_READ_WRITE_EXECUTION_THRESHOLD: c_uint = 0x0120;
pub const EMIF_COS_CONFIG: c_uint = 0x0124;
pub const EMIF_PHY_STATUS_1: c_uint = 0x0140;
pub const EMIF_PHY_STATUS_2: c_uint = 0x0144;
pub const EMIF_PHY_STATUS_3: c_uint = 0x0148;
pub const EMIF_PHY_STATUS_4: c_uint = 0x014c;
pub const EMIF_PHY_STATUS_5: c_uint = 0x0150;
pub const EMIF_PHY_STATUS_6: c_uint = 0x0154;
pub const EMIF_PHY_STATUS_7: c_uint = 0x0158;
pub const EMIF_PHY_STATUS_8: c_uint = 0x015c;
pub const EMIF_PHY_STATUS_9: c_uint = 0x0160;
pub const EMIF_PHY_STATUS_10: c_uint = 0x0164;
pub const EMIF_PHY_STATUS_11: c_uint = 0x0168;
pub const EMIF_PHY_STATUS_12: c_uint = 0x016c;
pub const EMIF_PHY_STATUS_13: c_uint = 0x0170;
pub const EMIF_PHY_STATUS_14: c_uint = 0x0174;
pub const EMIF_PHY_STATUS_15: c_uint = 0x0178;
pub const EMIF_PHY_STATUS_16: c_uint = 0x017c;
pub const EMIF_PHY_STATUS_17: c_uint = 0x0180;
pub const EMIF_PHY_STATUS_18: c_uint = 0x0184;
pub const EMIF_PHY_STATUS_19: c_uint = 0x0188;
pub const EMIF_PHY_STATUS_20: c_uint = 0x018c;
pub const EMIF_PHY_STATUS_21: c_uint = 0x0190;
pub const EMIF_EXT_PHY_CTRL_1: c_uint = 0x0200;
pub const EMIF_EXT_PHY_CTRL_1_SHDW: c_uint = 0x0204;
pub const EMIF_EXT_PHY_CTRL_2: c_uint = 0x0208;
pub const EMIF_EXT_PHY_CTRL_2_SHDW: c_uint = 0x020c;
pub const EMIF_EXT_PHY_CTRL_3: c_uint = 0x0210;
pub const EMIF_EXT_PHY_CTRL_3_SHDW: c_uint = 0x0214;
pub const EMIF_EXT_PHY_CTRL_4: c_uint = 0x0218;
pub const EMIF_EXT_PHY_CTRL_4_SHDW: c_uint = 0x021c;
pub const EMIF_EXT_PHY_CTRL_5: c_uint = 0x0220;
pub const EMIF_EXT_PHY_CTRL_5_SHDW: c_uint = 0x0224;
pub const EMIF_EXT_PHY_CTRL_6: c_uint = 0x0228;
pub const EMIF_EXT_PHY_CTRL_6_SHDW: c_uint = 0x022c;
pub const EMIF_EXT_PHY_CTRL_7: c_uint = 0x0230;
pub const EMIF_EXT_PHY_CTRL_7_SHDW: c_uint = 0x0234;
pub const EMIF_EXT_PHY_CTRL_8: c_uint = 0x0238;
pub const EMIF_EXT_PHY_CTRL_8_SHDW: c_uint = 0x023c;
pub const EMIF_EXT_PHY_CTRL_9: c_uint = 0x0240;
pub const EMIF_EXT_PHY_CTRL_9_SHDW: c_uint = 0x0244;
pub const EMIF_EXT_PHY_CTRL_10: c_uint = 0x0248;
pub const EMIF_EXT_PHY_CTRL_10_SHDW: c_uint = 0x024c;
pub const EMIF_EXT_PHY_CTRL_11: c_uint = 0x0250;
pub const EMIF_EXT_PHY_CTRL_11_SHDW: c_uint = 0x0254;
pub const EMIF_EXT_PHY_CTRL_12: c_uint = 0x0258;
pub const EMIF_EXT_PHY_CTRL_12_SHDW: c_uint = 0x025c;
pub const EMIF_EXT_PHY_CTRL_13: c_uint = 0x0260;
pub const EMIF_EXT_PHY_CTRL_13_SHDW: c_uint = 0x0264;
pub const EMIF_EXT_PHY_CTRL_14: c_uint = 0x0268;
pub const EMIF_EXT_PHY_CTRL_14_SHDW: c_uint = 0x026c;
pub const EMIF_EXT_PHY_CTRL_15: c_uint = 0x0270;
pub const EMIF_EXT_PHY_CTRL_15_SHDW: c_uint = 0x0274;
pub const EMIF_EXT_PHY_CTRL_16: c_uint = 0x0278;
pub const EMIF_EXT_PHY_CTRL_16_SHDW: c_uint = 0x027c;
pub const EMIF_EXT_PHY_CTRL_17: c_uint = 0x0280;
pub const EMIF_EXT_PHY_CTRL_17_SHDW: c_uint = 0x0284;
pub const EMIF_EXT_PHY_CTRL_18: c_uint = 0x0288;
pub const EMIF_EXT_PHY_CTRL_18_SHDW: c_uint = 0x028c;
pub const EMIF_EXT_PHY_CTRL_19: c_uint = 0x0290;
pub const EMIF_EXT_PHY_CTRL_19_SHDW: c_uint = 0x0294;
pub const EMIF_EXT_PHY_CTRL_20: c_uint = 0x0298;
pub const EMIF_EXT_PHY_CTRL_20_SHDW: c_uint = 0x029c;
pub const EMIF_EXT_PHY_CTRL_21: c_uint = 0x02a0;
pub const EMIF_EXT_PHY_CTRL_21_SHDW: c_uint = 0x02a4;
pub const EMIF_EXT_PHY_CTRL_22: c_uint = 0x02a8;
pub const EMIF_EXT_PHY_CTRL_22_SHDW: c_uint = 0x02ac;
pub const EMIF_EXT_PHY_CTRL_23: c_uint = 0x02b0;
pub const EMIF_EXT_PHY_CTRL_23_SHDW: c_uint = 0x02b4;
pub const EMIF_EXT_PHY_CTRL_24: c_uint = 0x02b8;
pub const EMIF_EXT_PHY_CTRL_24_SHDW: c_uint = 0x02bc;
pub const EMIF_EXT_PHY_CTRL_25: c_uint = 0x02c0;
pub const EMIF_EXT_PHY_CTRL_25_SHDW: c_uint = 0x02c4;
pub const EMIF_EXT_PHY_CTRL_26: c_uint = 0x02c8;
pub const EMIF_EXT_PHY_CTRL_26_SHDW: c_uint = 0x02cc;
pub const EMIF_EXT_PHY_CTRL_27: c_uint = 0x02d0;
pub const EMIF_EXT_PHY_CTRL_27_SHDW: c_uint = 0x02d4;
pub const EMIF_EXT_PHY_CTRL_28: c_uint = 0x02d8;
pub const EMIF_EXT_PHY_CTRL_28_SHDW: c_uint = 0x02dc;
pub const EMIF_EXT_PHY_CTRL_29: c_uint = 0x02e0;
pub const EMIF_EXT_PHY_CTRL_29_SHDW: c_uint = 0x02e4;
pub const EMIF_EXT_PHY_CTRL_30: c_uint = 0x02e8;
pub const EMIF_EXT_PHY_CTRL_30_SHDW: c_uint = 0x02ec;
// Registers shifts and masks
// EMIF_MODULE_ID_AND_REVISION
pub const SCHEME_SHIFT: c_int = 30;

pub const MODULE_ID_SHIFT: c_int = 16;

pub const RTL_VERSION_SHIFT: c_int = 11;

pub const MAJOR_REVISION_SHIFT: c_int = 8;

pub const MINOR_REVISION_SHIFT: c_int = 0;

// STATUS
pub const BE_SHIFT: c_int = 31;

pub const DUAL_CLK_MODE_SHIFT: c_int = 30;

pub const FAST_INIT_SHIFT: c_int = 29;

pub const RDLVLGATETO_SHIFT: c_int = 6;

pub const RDLVLTO_SHIFT: c_int = 5;

pub const WRLVLTO_SHIFT: c_int = 4;

pub const PHY_DLL_READY_SHIFT: c_int = 2;

// SDRAM_CONFIG
pub const SDRAM_TYPE_SHIFT: c_int = 29;

pub const IBANK_POS_SHIFT: c_int = 27;

pub const DDR_TERM_SHIFT: c_int = 24;

pub const DDR2_DDQS_SHIFT: c_int = 23;

pub const DYN_ODT_SHIFT: c_int = 21;

pub const DDR_DISABLE_DLL_SHIFT: c_int = 20;

pub const SDRAM_DRIVE_SHIFT: c_int = 18;

pub const CWL_SHIFT: c_int = 16;

pub const NARROW_MODE_SHIFT: c_int = 14;

pub const CL_SHIFT: c_int = 10;

pub const ROWSIZE_SHIFT: c_int = 7;

pub const IBANK_SHIFT: c_int = 4;

pub const EBANK_SHIFT: c_int = 3;

pub const PAGESIZE_SHIFT: c_int = 0;

// SDRAM_CONFIG_2
pub const CS1NVMEN_SHIFT: c_int = 30;

pub const EBANK_POS_SHIFT: c_int = 27;

pub const RDBNUM_SHIFT: c_int = 4;

pub const RDBSIZE_SHIFT: c_int = 0;

// SDRAM_REFRESH_CONTROL
pub const INITREF_DIS_SHIFT: c_int = 31;

pub const SRT_SHIFT: c_int = 29;

pub const ASR_SHIFT: c_int = 28;

pub const PASR_SHIFT: c_int = 24;

pub const REFRESH_RATE_SHIFT: c_int = 0;

// SDRAM_TIMING_1
pub const T_RTW_SHIFT: c_int = 29;

pub const T_RP_SHIFT: c_int = 25;

pub const T_RCD_SHIFT: c_int = 21;

pub const T_WR_SHIFT: c_int = 17;

pub const T_RAS_SHIFT: c_int = 12;

pub const T_RC_SHIFT: c_int = 6;

pub const T_RRD_SHIFT: c_int = 3;

pub const T_WTR_SHIFT: c_int = 0;

// SDRAM_TIMING_2
pub const T_XP_SHIFT: c_int = 28;

pub const T_ODT_SHIFT: c_int = 25;

pub const T_XSNR_SHIFT: c_int = 16;

pub const T_XSRD_SHIFT: c_int = 6;

pub const T_RTP_SHIFT: c_int = 3;

pub const T_CKE_SHIFT: c_int = 0;

// SDRAM_TIMING_3
pub const T_PDLL_UL_SHIFT: c_int = 28;

pub const T_CSTA_SHIFT: c_int = 24;

pub const T_CKESR_SHIFT: c_int = 21;

pub const ZQ_ZQCS_SHIFT: c_int = 15;

pub const T_TDQSCKMAX_SHIFT: c_int = 13;

pub const T_RFC_SHIFT: c_int = 4;

pub const T_RAS_MAX_SHIFT: c_int = 0;

// POWER_MANAGEMENT_CONTROL
pub const PD_TIM_SHIFT: c_int = 12;

pub const DPD_EN_SHIFT: c_int = 11;

pub const LP_MODE_SHIFT: c_int = 8;

pub const SR_TIM_SHIFT: c_int = 4;

pub const CS_TIM_SHIFT: c_int = 0;

// LPDDR2_MODE_REG_DATA
pub const VALUE_0_SHIFT: c_int = 0;

// LPDDR2_MODE_REG_CONFIG
pub const CS_SHIFT: c_int = 31;

pub const REFRESH_EN_SHIFT: c_int = 30;

pub const ADDRESS_SHIFT: c_int = 0;

// OCP_CONFIG
pub const SYS_THRESH_MAX_SHIFT: c_int = 24;

pub const MPU_THRESH_MAX_SHIFT: c_int = 20;

pub const LL_THRESH_MAX_SHIFT: c_int = 16;

// PERFORMANCE_COUNTER_1
pub const COUNTER1_SHIFT: c_int = 0;

// PERFORMANCE_COUNTER_2
pub const COUNTER2_SHIFT: c_int = 0;

// PERFORMANCE_COUNTER_CONFIG
pub const CNTR2_MCONNID_EN_SHIFT: c_int = 31;

pub const CNTR2_REGION_EN_SHIFT: c_int = 30;

pub const CNTR2_CFG_SHIFT: c_int = 16;

pub const CNTR1_MCONNID_EN_SHIFT: c_int = 15;

pub const CNTR1_REGION_EN_SHIFT: c_int = 14;

pub const CNTR1_CFG_SHIFT: c_int = 0;

// PERFORMANCE_COUNTER_MASTER_REGION_SELECT
pub const MCONNID2_SHIFT: c_int = 24;

pub const REGION_SEL2_SHIFT: c_int = 16;

pub const MCONNID1_SHIFT: c_int = 8;

pub const REGION_SEL1_SHIFT: c_int = 0;

// PERFORMANCE_COUNTER_TIME
pub const TOTAL_TIME_SHIFT: c_int = 0;

// DLL_CALIB_CTRL
pub const ACK_WAIT_SHIFT: c_int = 16;

pub const DLL_CALIB_INTERVAL_SHIFT: c_int = 0;

// END_OF_INTERRUPT
pub const EOI_SHIFT: c_int = 0;

// SYSTEM_OCP_INTERRUPT_RAW_STATUS
pub const DNV_SYS_SHIFT: c_int = 2;

pub const TA_SYS_SHIFT: c_int = 1;

pub const ERR_SYS_SHIFT: c_int = 0;

// LOW_LATENCY_OCP_INTERRUPT_RAW_STATUS
pub const DNV_LL_SHIFT: c_int = 2;

pub const TA_LL_SHIFT: c_int = 1;

pub const ERR_LL_SHIFT: c_int = 0;

// SYSTEM_OCP_INTERRUPT_ENABLE_SET
pub const EN_DNV_SYS_SHIFT: c_int = 2;

pub const EN_TA_SYS_SHIFT: c_int = 1;

pub const EN_ERR_SYS_SHIFT: c_int = 0;

// LOW_LATENCY_OCP_INTERRUPT_ENABLE_SET
pub const EN_DNV_LL_SHIFT: c_int = 2;

pub const EN_TA_LL_SHIFT: c_int = 1;

pub const EN_ERR_LL_SHIFT: c_int = 0;

// SDRAM_OUTPUT_IMPEDANCE_CALIBRATION_CONFIG
pub const ZQ_CS1EN_SHIFT: c_int = 31;

pub const ZQ_CS0EN_SHIFT: c_int = 30;

pub const ZQ_DUALCALEN_SHIFT: c_int = 29;

pub const ZQ_SFEXITEN_SHIFT: c_int = 28;

pub const ZQ_ZQINIT_MULT_SHIFT: c_int = 18;

pub const ZQ_ZQCL_MULT_SHIFT: c_int = 16;

pub const ZQ_REFINTERVAL_SHIFT: c_int = 0;

// TEMPERATURE_ALERT_CONFIG
pub const TA_CS1EN_SHIFT: c_int = 31;

pub const TA_CS0EN_SHIFT: c_int = 30;

pub const TA_SFEXITEN_SHIFT: c_int = 28;

pub const TA_DEVWDT_SHIFT: c_int = 26;

pub const TA_DEVCNT_SHIFT: c_int = 24;

pub const TA_REFINTERVAL_SHIFT: c_int = 0;

// OCP_ERROR_LOG
pub const MADDRSPACE_SHIFT: c_int = 14;

pub const MBURSTSEQ_SHIFT: c_int = 11;

pub const MCMD_SHIFT: c_int = 8;

pub const MCONNID_SHIFT: c_int = 0;

// READ_WRITE_LEVELING_CONTROL
pub const RDWRLVLFULL_START: c_uint = 0x80000000;
// DDR_PHY_CTRL_1 - EMIF4D
pub const DLL_SLAVE_DLY_CTRL_SHIFT_4D: c_int = 4;

pub const READ_LATENCY_SHIFT_4D: c_int = 0;

// DDR_PHY_CTRL_1 - EMIF4D5
pub const DLL_HALF_DELAY_SHIFT_4D5: c_int = 21;

pub const READ_LATENCY_SHIFT_4D5: c_int = 0;

// DDR_PHY_CTRL_1_SHDW
pub const DDR_PHY_CTRL_1_SHDW_SHIFT: c_int = 5;

pub const READ_LATENCY_SHDW_SHIFT: c_int = 0;

pub const EMIF_SRAM_AM33_REG_LAYOUT: c_uint = 0x00000000;
pub const EMIF_SRAM_AM43_REG_LAYOUT: c_uint = 0x00000001;
//
// Structure containing shadow of important registers in EMIF
// The calculation function fills in this structure to be later used for
// initialisation and DVFS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emif_regs {
    pub freq: u32,
    pub ref_ctrl_shdw: u32,
    pub ref_ctrl_shdw_derated: u32,
    pub sdram_tim1_shdw: u32,
    pub sdram_tim1_shdw_derated: u32,
    pub sdram_tim2_shdw: u32,
    pub sdram_tim3_shdw: u32,
    pub sdram_tim3_shdw_derated: u32,
    pub pwr_mgmt_ctrl_shdw: u32,
    pub read_idle_ctrl_shdw_normal: u32,
    pub dll_calib_ctrl_shdw_normal: u32,
}

extern "C" {
    pub fn ti_emif_save_context();
}
extern "C" {
    pub fn ti_emif_restore_context();
}
extern "C" {
    pub fn ti_emif_run_hw_leveling();
}
extern "C" {
    pub fn ti_emif_enter_sr();
}
extern "C" {
    pub fn ti_emif_exit_sr();
}
extern "C" {
    pub fn ti_emif_abort_sr();
}

