//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/suni1x10gexp_regs.h
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
// File: suni1x10gexp_regs.h
// $Revision: 1.9 $
// $Date: 2005/06/22 00:17:04 $
// Description:
// PMC/SIERRA (pm3393) MAC-PHY functionality.
// part of the Chelsio 10Gb Ethernet Driver.
//
// http://www.chelsio.com
//
// Maintainers: maintainers@chelsio.com
//
// Authors: PMC/SIERRA
//
// History:
//

// Macro flag: #define _CXGB_SUNI1x10GEXP_REGS_H_
//
// Space allocated for each Exact Match Filter
// There are 8 filter configurations
//
pub const SUNI1x10GEXP_REG_SIZEOF_MAC_FILTER: c_uint = 0x0003;

//
// Space allocated for VLAN-Id Filter
// There are 8 filter configurations
//
pub const SUNI1x10GEXP_REG_SIZEOF_MAC_VID_FILTER: c_uint = 0x0001;

//
// Space allocated for each MSTAT Counter
//
pub const SUNI1x10GEXP_REG_SIZEOF_MSTAT_COUNT: c_uint = 0x0004;

//
// S/UNI-1x10GE-XP REGISTER ADDRESS MAP
//
// Refer to the Register Bit Masks bellow for the naming of each register and
// to the S/UNI-1x10GE-XP Data Sheet for the signification of each bit
//
pub const SUNI1x10GEXP_REG_IDENTIFICATION: c_uint = 0x0000;
pub const SUNI1x10GEXP_REG_PRODUCT_REVISION: c_uint = 0x0001;
pub const SUNI1x10GEXP_REG_CONFIG_AND_RESET_CONTROL: c_uint = 0x0002;
pub const SUNI1x10GEXP_REG_LOOPBACK_MISC_CTRL: c_uint = 0x0003;
pub const SUNI1x10GEXP_REG_DEVICE_STATUS: c_uint = 0x0004;
pub const SUNI1x10GEXP_REG_GLOBAL_PERFORMANCE_MONITOR_UPDATE: c_uint = 0x0005;
pub const SUNI1x10GEXP_REG_MDIO_COMMAND: c_uint = 0x0006;
pub const SUNI1x10GEXP_REG_MDIO_INTERRUPT_ENABLE: c_uint = 0x0007;
pub const SUNI1x10GEXP_REG_MDIO_INTERRUPT_STATUS: c_uint = 0x0008;
pub const SUNI1x10GEXP_REG_MMD_PHY_ADDRESS: c_uint = 0x0009;
pub const SUNI1x10GEXP_REG_MMD_CONTROL_ADDRESS_DATA: c_uint = 0x000A;
pub const SUNI1x10GEXP_REG_MDIO_READ_STATUS_DATA: c_uint = 0x000B;
pub const SUNI1x10GEXP_REG_OAM_INTF_CTRL: c_uint = 0x000C;
pub const SUNI1x10GEXP_REG_MASTER_INTERRUPT_STATUS: c_uint = 0x000D;
pub const SUNI1x10GEXP_REG_GLOBAL_INTERRUPT_ENABLE: c_uint = 0x000E;
pub const SUNI1x10GEXP_REG_FREE: c_uint = 0x000F;
pub const SUNI1x10GEXP_REG_XTEF_MISC_CTRL: c_uint = 0x0010;
pub const SUNI1x10GEXP_REG_XRF_MISC_CTRL: c_uint = 0x0011;
pub const SUNI1x10GEXP_REG_SERDES_3125_CONFIG_1: c_uint = 0x0100;
pub const SUNI1x10GEXP_REG_SERDES_3125_CONFIG_2: c_uint = 0x0101;
pub const SUNI1x10GEXP_REG_SERDES_3125_INTERRUPT_ENABLE: c_uint = 0x0102;
pub const SUNI1x10GEXP_REG_SERDES_3125_INTERRUPT_VISIBLE: c_uint = 0x0103;
pub const SUNI1x10GEXP_REG_SERDES_3125_INTERRUPT_STATUS: c_uint = 0x0104;
pub const SUNI1x10GEXP_REG_SERDES_3125_TEST_CONFIG: c_uint = 0x0107;
pub const SUNI1x10GEXP_REG_RXXG_CONFIG_1: c_uint = 0x2040;
pub const SUNI1x10GEXP_REG_RXXG_CONFIG_2: c_uint = 0x2041;
pub const SUNI1x10GEXP_REG_RXXG_CONFIG_3: c_uint = 0x2042;
pub const SUNI1x10GEXP_REG_RXXG_INTERRUPT: c_uint = 0x2043;
pub const SUNI1x10GEXP_REG_RXXG_MAX_FRAME_LENGTH: c_uint = 0x2045;
pub const SUNI1x10GEXP_REG_RXXG_SA_15_0: c_uint = 0x2046;
pub const SUNI1x10GEXP_REG_RXXG_SA_31_16: c_uint = 0x2047;
pub const SUNI1x10GEXP_REG_RXXG_SA_47_32: c_uint = 0x2048;
pub const SUNI1x10GEXP_REG_RXXG_RECEIVE_FIFO_THRESHOLD: c_uint = 0x2049;

pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_0_LOW: c_uint = 0x204A;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_0_MID: c_uint = 0x204B;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_0_HIGH: c_uint = 0x204C;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_1_LOW: c_uint = 0x204D;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_1_MID: c_uint = 0x204E;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_1_HIGH: c_uint = 0x204F;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_2_LOW: c_uint = 0x2050;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_2_MID: c_uint = 0x2051;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_2_HIGH: c_uint = 0x2052;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_3_LOW: c_uint = 0x2053;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_3_MID: c_uint = 0x2054;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_3_HIGH: c_uint = 0x2055;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_4_LOW: c_uint = 0x2056;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_4_MID: c_uint = 0x2057;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_4_HIGH: c_uint = 0x2058;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_5_LOW: c_uint = 0x2059;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_5_MID: c_uint = 0x205A;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_5_HIGH: c_uint = 0x205B;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_6_LOW: c_uint = 0x205C;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_6_MID: c_uint = 0x205D;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_6_HIGH: c_uint = 0x205E;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_7_LOW: c_uint = 0x205F;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_7_MID: c_uint = 0x2060;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_ADDR_7_HIGH: c_uint = 0x2061;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_0: c_uint = 0x2062;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_1: c_uint = 0x2063;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_2: c_uint = 0x2064;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_3: c_uint = 0x2065;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_4: c_uint = 0x2066;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_5: c_uint = 0x2067;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_6: c_uint = 0x2068;
pub const SUNI1x10GEXP_REG_RXXG_EXACT_MATCH_VID_7: c_uint = 0x2069;
pub const SUNI1x10GEXP_REG_RXXG_MULTICAST_HASH_LOW: c_uint = 0x206A;
pub const SUNI1x10GEXP_REG_RXXG_MULTICAST_HASH_MIDLOW: c_uint = 0x206B;
pub const SUNI1x10GEXP_REG_RXXG_MULTICAST_HASH_MIDHIGH: c_uint = 0x206C;
pub const SUNI1x10GEXP_REG_RXXG_MULTICAST_HASH_HIGH: c_uint = 0x206D;
pub const SUNI1x10GEXP_REG_RXXG_ADDRESS_FILTER_CONTROL_0: c_uint = 0x206E;
pub const SUNI1x10GEXP_REG_RXXG_ADDRESS_FILTER_CONTROL_1: c_uint = 0x206F;
pub const SUNI1x10GEXP_REG_RXXG_ADDRESS_FILTER_CONTROL_2: c_uint = 0x2070;
pub const SUNI1x10GEXP_REG_XRF_PATTERN_GEN_CTRL: c_uint = 0x2081;
pub const SUNI1x10GEXP_REG_XRF_8BTB_ERR_COUNT_LANE_0: c_uint = 0x2084;
pub const SUNI1x10GEXP_REG_XRF_8BTB_ERR_COUNT_LANE_1: c_uint = 0x2085;
pub const SUNI1x10GEXP_REG_XRF_8BTB_ERR_COUNT_LANE_2: c_uint = 0x2086;
pub const SUNI1x10GEXP_REG_XRF_8BTB_ERR_COUNT_LANE_3: c_uint = 0x2087;
pub const SUNI1x10GEXP_REG_XRF_INTERRUPT_ENABLE: c_uint = 0x2088;
pub const SUNI1x10GEXP_REG_XRF_INTERRUPT_STATUS: c_uint = 0x2089;
pub const SUNI1x10GEXP_REG_XRF_ERR_STATUS: c_uint = 0x208A;
pub const SUNI1x10GEXP_REG_XRF_DIAG_INTERRUPT_ENABLE: c_uint = 0x208B;
pub const SUNI1x10GEXP_REG_XRF_DIAG_INTERRUPT_STATUS: c_uint = 0x208C;
pub const SUNI1x10GEXP_REG_XRF_CODE_ERR_THRES: c_uint = 0x2092;
pub const SUNI1x10GEXP_REG_RXOAM_CONFIG: c_uint = 0x20C0;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_1_CONFIG: c_uint = 0x20C1;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_2_CONFIG: c_uint = 0x20C2;
pub const SUNI1x10GEXP_REG_RXOAM_CONFIG_2: c_uint = 0x20C3;
pub const SUNI1x10GEXP_REG_RXOAM_HEC_CONFIG: c_uint = 0x20C4;
pub const SUNI1x10GEXP_REG_RXOAM_HEC_ERR_THRES: c_uint = 0x20C5;
pub const SUNI1x10GEXP_REG_RXOAM_INTERRUPT_ENABLE: c_uint = 0x20C7;
pub const SUNI1x10GEXP_REG_RXOAM_INTERRUPT_STATUS: c_uint = 0x20C8;
pub const SUNI1x10GEXP_REG_RXOAM_STATUS: c_uint = 0x20C9;
pub const SUNI1x10GEXP_REG_RXOAM_HEC_ERR_COUNT: c_uint = 0x20CA;
pub const SUNI1x10GEXP_REG_RXOAM_FIFO_OVERFLOW_COUNT: c_uint = 0x20CB;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_MISMATCH_COUNT_LSB: c_uint = 0x20CC;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_MISMATCH_COUNT_MSB: c_uint = 0x20CD;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_1_MISMATCH_COUNT_LSB: c_uint = 0x20CE;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_1_MISMATCH_COUNT_MSB: c_uint = 0x20CF;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_2_MISMATCH_COUNT_LSB: c_uint = 0x20D0;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_2_MISMATCH_COUNT_MSB: c_uint = 0x20D1;
pub const SUNI1x10GEXP_REG_RXOAM_OAM_EXTRACT_COUNT_LSB: c_uint = 0x20D2;
pub const SUNI1x10GEXP_REG_RXOAM_OAM_EXTRACT_COUNT_MSB: c_uint = 0x20D3;
pub const SUNI1x10GEXP_REG_RXOAM_MINI_PACKET_COUNT_LSB: c_uint = 0x20D4;
pub const SUNI1x10GEXP_REG_RXOAM_MINI_PACKET_COUNT_MSB: c_uint = 0x20D5;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_MISMATCH_THRES_LSB: c_uint = 0x20D6;
pub const SUNI1x10GEXP_REG_RXOAM_FILTER_MISMATCH_THRES_MSB: c_uint = 0x20D7;
pub const SUNI1x10GEXP_REG_MSTAT_CONTROL: c_uint = 0x2100;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_ROLLOVER_0: c_uint = 0x2101;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_ROLLOVER_1: c_uint = 0x2102;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_ROLLOVER_2: c_uint = 0x2103;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_ROLLOVER_3: c_uint = 0x2104;
pub const SUNI1x10GEXP_REG_MSTAT_INTERRUPT_MASK_0: c_uint = 0x2105;
pub const SUNI1x10GEXP_REG_MSTAT_INTERRUPT_MASK_1: c_uint = 0x2106;
pub const SUNI1x10GEXP_REG_MSTAT_INTERRUPT_MASK_2: c_uint = 0x2107;
pub const SUNI1x10GEXP_REG_MSTAT_INTERRUPT_MASK_3: c_uint = 0x2108;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_WRITE_ADDRESS: c_uint = 0x2109;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_WRITE_DATA_LOW: c_uint = 0x210A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_WRITE_DATA_MIDDLE: c_uint = 0x210B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_WRITE_DATA_HIGH: c_uint = 0x210C;

pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_0_LOW: c_uint = 0x2110;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_0_MID: c_uint = 0x2111;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_0_HIGH: c_uint = 0x2112;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_0_RESVD: c_uint = 0x2113;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_1_LOW: c_uint = 0x2114;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_1_MID: c_uint = 0x2115;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_1_HIGH: c_uint = 0x2116;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_1_RESVD: c_uint = 0x2117;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_2_LOW: c_uint = 0x2118;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_2_MID: c_uint = 0x2119;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_2_HIGH: c_uint = 0x211A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_2_RESVD: c_uint = 0x211B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_3_LOW: c_uint = 0x211C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_3_MID: c_uint = 0x211D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_3_HIGH: c_uint = 0x211E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_3_RESVD: c_uint = 0x211F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_4_LOW: c_uint = 0x2120;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_4_MID: c_uint = 0x2121;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_4_HIGH: c_uint = 0x2122;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_4_RESVD: c_uint = 0x2123;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_5_LOW: c_uint = 0x2124;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_5_MID: c_uint = 0x2125;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_5_HIGH: c_uint = 0x2126;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_5_RESVD: c_uint = 0x2127;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_6_LOW: c_uint = 0x2128;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_6_MID: c_uint = 0x2129;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_6_HIGH: c_uint = 0x212A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_6_RESVD: c_uint = 0x212B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_7_LOW: c_uint = 0x212C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_7_MID: c_uint = 0x212D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_7_HIGH: c_uint = 0x212E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_7_RESVD: c_uint = 0x212F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_8_LOW: c_uint = 0x2130;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_8_MID: c_uint = 0x2131;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_8_HIGH: c_uint = 0x2132;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_8_RESVD: c_uint = 0x2133;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_9_LOW: c_uint = 0x2134;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_9_MID: c_uint = 0x2135;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_9_HIGH: c_uint = 0x2136;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_9_RESVD: c_uint = 0x2137;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_10_LOW: c_uint = 0x2138;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_10_MID: c_uint = 0x2139;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_10_HIGH: c_uint = 0x213A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_10_RESVD: c_uint = 0x213B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_11_LOW: c_uint = 0x213C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_11_MID: c_uint = 0x213D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_11_HIGH: c_uint = 0x213E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_11_RESVD: c_uint = 0x213F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_12_LOW: c_uint = 0x2140;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_12_MID: c_uint = 0x2141;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_12_HIGH: c_uint = 0x2142;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_12_RESVD: c_uint = 0x2143;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_13_LOW: c_uint = 0x2144;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_13_MID: c_uint = 0x2145;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_13_HIGH: c_uint = 0x2146;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_13_RESVD: c_uint = 0x2147;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_14_LOW: c_uint = 0x2148;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_14_MID: c_uint = 0x2149;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_14_HIGH: c_uint = 0x214A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_14_RESVD: c_uint = 0x214B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_15_LOW: c_uint = 0x214C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_15_MID: c_uint = 0x214D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_15_HIGH: c_uint = 0x214E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_15_RESVD: c_uint = 0x214F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_16_LOW: c_uint = 0x2150;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_16_MID: c_uint = 0x2151;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_16_HIGH: c_uint = 0x2152;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_16_RESVD: c_uint = 0x2153;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_17_LOW: c_uint = 0x2154;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_17_MID: c_uint = 0x2155;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_17_HIGH: c_uint = 0x2156;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_17_RESVD: c_uint = 0x2157;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_18_LOW: c_uint = 0x2158;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_18_MID: c_uint = 0x2159;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_18_HIGH: c_uint = 0x215A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_18_RESVD: c_uint = 0x215B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_19_LOW: c_uint = 0x215C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_19_MID: c_uint = 0x215D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_19_HIGH: c_uint = 0x215E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_19_RESVD: c_uint = 0x215F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_20_LOW: c_uint = 0x2160;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_20_MID: c_uint = 0x2161;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_20_HIGH: c_uint = 0x2162;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_20_RESVD: c_uint = 0x2163;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_21_LOW: c_uint = 0x2164;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_21_MID: c_uint = 0x2165;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_21_HIGH: c_uint = 0x2166;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_21_RESVD: c_uint = 0x2167;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_22_LOW: c_uint = 0x2168;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_22_MID: c_uint = 0x2169;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_22_HIGH: c_uint = 0x216A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_22_RESVD: c_uint = 0x216B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_23_LOW: c_uint = 0x216C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_23_MID: c_uint = 0x216D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_23_HIGH: c_uint = 0x216E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_23_RESVD: c_uint = 0x216F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_24_LOW: c_uint = 0x2170;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_24_MID: c_uint = 0x2171;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_24_HIGH: c_uint = 0x2172;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_24_RESVD: c_uint = 0x2173;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_25_LOW: c_uint = 0x2174;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_25_MID: c_uint = 0x2175;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_25_HIGH: c_uint = 0x2176;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_25_RESVD: c_uint = 0x2177;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_26_LOW: c_uint = 0x2178;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_26_MID: c_uint = 0x2179;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_26_HIGH: c_uint = 0x217a;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_26_RESVD: c_uint = 0x217b;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_27_LOW: c_uint = 0x217c;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_27_MID: c_uint = 0x217d;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_27_HIGH: c_uint = 0x217e;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_27_RESVD: c_uint = 0x217f;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_28_LOW: c_uint = 0x2180;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_28_MID: c_uint = 0x2181;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_28_HIGH: c_uint = 0x2182;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_28_RESVD: c_uint = 0x2183;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_29_LOW: c_uint = 0x2184;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_29_MID: c_uint = 0x2185;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_29_HIGH: c_uint = 0x2186;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_29_RESVD: c_uint = 0x2187;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_30_LOW: c_uint = 0x2188;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_30_MID: c_uint = 0x2189;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_30_HIGH: c_uint = 0x218A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_30_RESVD: c_uint = 0x218B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_31_LOW: c_uint = 0x218C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_31_MID: c_uint = 0x218D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_31_HIGH: c_uint = 0x218E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_31_RESVD: c_uint = 0x218F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_32_LOW: c_uint = 0x2190;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_32_MID: c_uint = 0x2191;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_32_HIGH: c_uint = 0x2192;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_32_RESVD: c_uint = 0x2193;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_33_LOW: c_uint = 0x2194;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_33_MID: c_uint = 0x2195;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_33_HIGH: c_uint = 0x2196;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_33_RESVD: c_uint = 0x2197;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_34_LOW: c_uint = 0x2198;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_34_MID: c_uint = 0x2199;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_34_HIGH: c_uint = 0x219A;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_34_RESVD: c_uint = 0x219B;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_35_LOW: c_uint = 0x219C;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_35_MID: c_uint = 0x219D;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_35_HIGH: c_uint = 0x219E;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_35_RESVD: c_uint = 0x219F;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_36_LOW: c_uint = 0x21A0;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_36_MID: c_uint = 0x21A1;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_36_HIGH: c_uint = 0x21A2;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_36_RESVD: c_uint = 0x21A3;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_37_LOW: c_uint = 0x21A4;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_37_MID: c_uint = 0x21A5;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_37_HIGH: c_uint = 0x21A6;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_37_RESVD: c_uint = 0x21A7;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_38_LOW: c_uint = 0x21A8;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_38_MID: c_uint = 0x21A9;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_38_HIGH: c_uint = 0x21AA;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_38_RESVD: c_uint = 0x21AB;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_39_LOW: c_uint = 0x21AC;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_39_MID: c_uint = 0x21AD;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_39_HIGH: c_uint = 0x21AE;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_39_RESVD: c_uint = 0x21AF;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_40_LOW: c_uint = 0x21B0;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_40_MID: c_uint = 0x21B1;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_40_HIGH: c_uint = 0x21B2;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_40_RESVD: c_uint = 0x21B3;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_41_LOW: c_uint = 0x21B4;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_41_MID: c_uint = 0x21B5;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_41_HIGH: c_uint = 0x21B6;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_41_RESVD: c_uint = 0x21B7;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_42_LOW: c_uint = 0x21B8;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_42_MID: c_uint = 0x21B9;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_42_HIGH: c_uint = 0x21BA;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_42_RESVD: c_uint = 0x21BB;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_43_LOW: c_uint = 0x21BC;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_43_MID: c_uint = 0x21BD;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_43_HIGH: c_uint = 0x21BE;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_43_RESVD: c_uint = 0x21BF;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_44_LOW: c_uint = 0x21C0;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_44_MID: c_uint = 0x21C1;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_44_HIGH: c_uint = 0x21C2;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_44_RESVD: c_uint = 0x21C3;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_45_LOW: c_uint = 0x21C4;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_45_MID: c_uint = 0x21C5;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_45_HIGH: c_uint = 0x21C6;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_45_RESVD: c_uint = 0x21C7;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_46_LOW: c_uint = 0x21C8;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_46_MID: c_uint = 0x21C9;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_46_HIGH: c_uint = 0x21CA;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_46_RESVD: c_uint = 0x21CB;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_47_LOW: c_uint = 0x21CC;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_47_MID: c_uint = 0x21CD;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_47_HIGH: c_uint = 0x21CE;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_47_RESVD: c_uint = 0x21CF;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_48_LOW: c_uint = 0x21D0;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_48_MID: c_uint = 0x21D1;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_48_HIGH: c_uint = 0x21D2;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_48_RESVD: c_uint = 0x21D3;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_49_LOW: c_uint = 0x21D4;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_49_MID: c_uint = 0x21D5;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_49_HIGH: c_uint = 0x21D6;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_49_RESVD: c_uint = 0x21D7;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_50_LOW: c_uint = 0x21D8;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_50_MID: c_uint = 0x21D9;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_50_HIGH: c_uint = 0x21DA;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_50_RESVD: c_uint = 0x21DB;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_51_LOW: c_uint = 0x21DC;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_51_MID: c_uint = 0x21DD;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_51_HIGH: c_uint = 0x21DE;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_51_RESVD: c_uint = 0x21DF;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_52_LOW: c_uint = 0x21E0;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_52_MID: c_uint = 0x21E1;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_52_HIGH: c_uint = 0x21E2;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_52_RESVD: c_uint = 0x21E3;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_53_LOW: c_uint = 0x21E4;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_53_MID: c_uint = 0x21E5;
pub const SUNI1x10GEXP_REG_MSTAT_COUNTER_53_HIGH: c_uint = 0x21E6;
pub const SUNI1x10GEXP_CNTR_MAC_ETHERNET_NUM: c_int = 51;
pub const SUNI1x10GEXP_REG_IFLX_GLOBAL_CONFIG: c_uint = 0x2200;
pub const SUNI1x10GEXP_REG_IFLX_CHANNEL_PROVISION: c_uint = 0x2201;
pub const SUNI1x10GEXP_REG_IFLX_FIFO_OVERFLOW_ENABLE: c_uint = 0x2209;
pub const SUNI1x10GEXP_REG_IFLX_FIFO_OVERFLOW_INTERRUPT: c_uint = 0x220A;
pub const SUNI1x10GEXP_REG_IFLX_INDIR_CHANNEL_ADDRESS: c_uint = 0x220D;
pub const SUNI1x10GEXP_REG_IFLX_INDIR_LOGICAL_FIFO_LOW_LIMIT_PROVISION: c_uint = 0x220E;
pub const SUNI1x10GEXP_REG_IFLX_INDIR_LOGICAL_FIFO_HIGH_LIMIT: c_uint = 0x220F;
pub const SUNI1x10GEXP_REG_IFLX_INDIR_FULL_ALMOST_FULL_STATUS_LIMIT: c_uint = 0x2210;
pub const SUNI1x10GEXP_REG_IFLX_INDIR_EMPTY_ALMOST_EMPTY_STATUS_LIMIT: c_uint = 0x2211;
pub const SUNI1x10GEXP_REG_PL4MOS_CONFIG: c_uint = 0x2240;
pub const SUNI1x10GEXP_REG_PL4MOS_MASK: c_uint = 0x2241;
pub const SUNI1x10GEXP_REG_PL4MOS_FAIRNESS_MASKING: c_uint = 0x2242;
pub const SUNI1x10GEXP_REG_PL4MOS_MAXBURST1: c_uint = 0x2243;
pub const SUNI1x10GEXP_REG_PL4MOS_MAXBURST2: c_uint = 0x2244;
pub const SUNI1x10GEXP_REG_PL4MOS_TRANSFER_SIZE: c_uint = 0x2245;
pub const SUNI1x10GEXP_REG_PL4ODP_CONFIG: c_uint = 0x2280;
pub const SUNI1x10GEXP_REG_PL4ODP_INTERRUPT_MASK: c_uint = 0x2282;
pub const SUNI1x10GEXP_REG_PL4ODP_INTERRUPT: c_uint = 0x2283;
pub const SUNI1x10GEXP_REG_PL4ODP_CONFIG_MAX_T: c_uint = 0x2284;
pub const SUNI1x10GEXP_REG_PL4IO_LOCK_DETECT_STATUS: c_uint = 0x2300;
pub const SUNI1x10GEXP_REG_PL4IO_LOCK_DETECT_CHANGE: c_uint = 0x2301;
pub const SUNI1x10GEXP_REG_PL4IO_LOCK_DETECT_MASK: c_uint = 0x2302;
pub const SUNI1x10GEXP_REG_PL4IO_LOCK_DETECT_LIMITS: c_uint = 0x2303;
pub const SUNI1x10GEXP_REG_PL4IO_CALENDAR_REPETITIONS: c_uint = 0x2304;
pub const SUNI1x10GEXP_REG_PL4IO_CONFIG: c_uint = 0x2305;
pub const SUNI1x10GEXP_REG_TXXG_CONFIG_1: c_uint = 0x3040;
pub const SUNI1x10GEXP_REG_TXXG_CONFIG_2: c_uint = 0x3041;
pub const SUNI1x10GEXP_REG_TXXG_CONFIG_3: c_uint = 0x3042;
pub const SUNI1x10GEXP_REG_TXXG_INTERRUPT: c_uint = 0x3043;
pub const SUNI1x10GEXP_REG_TXXG_STATUS: c_uint = 0x3044;
pub const SUNI1x10GEXP_REG_TXXG_MAX_FRAME_SIZE: c_uint = 0x3045;
pub const SUNI1x10GEXP_REG_TXXG_MIN_FRAME_SIZE: c_uint = 0x3046;
pub const SUNI1x10GEXP_REG_TXXG_SA_15_0: c_uint = 0x3047;
pub const SUNI1x10GEXP_REG_TXXG_SA_31_16: c_uint = 0x3048;
pub const SUNI1x10GEXP_REG_TXXG_SA_47_32: c_uint = 0x3049;
pub const SUNI1x10GEXP_REG_TXXG_PAUSE_TIMER: c_uint = 0x304D;
pub const SUNI1x10GEXP_REG_TXXG_PAUSE_TIMER_INTERVAL: c_uint = 0x304E;
pub const SUNI1x10GEXP_REG_TXXG_FILTER_ERROR_COUNTER: c_uint = 0x3051;
pub const SUNI1x10GEXP_REG_TXXG_PAUSE_QUANTUM_CONFIG: c_uint = 0x3052;
pub const SUNI1x10GEXP_REG_XTEF_CTRL: c_uint = 0x3080;
pub const SUNI1x10GEXP_REG_XTEF_INTERRUPT_STATUS: c_uint = 0x3084;
pub const SUNI1x10GEXP_REG_XTEF_INTERRUPT_ENABLE: c_uint = 0x3085;
pub const SUNI1x10GEXP_REG_XTEF_VISIBILITY: c_uint = 0x3086;
pub const SUNI1x10GEXP_REG_TXOAM_OAM_CONFIG: c_uint = 0x30C0;
pub const SUNI1x10GEXP_REG_TXOAM_MINI_RATE_CONFIG: c_uint = 0x30C1;
pub const SUNI1x10GEXP_REG_TXOAM_MINI_GAP_FIFO_CONFIG: c_uint = 0x30C2;
pub const SUNI1x10GEXP_REG_TXOAM_P1P2_STATIC_VALUES: c_uint = 0x30C3;
pub const SUNI1x10GEXP_REG_TXOAM_P3P4_STATIC_VALUES: c_uint = 0x30C4;
pub const SUNI1x10GEXP_REG_TXOAM_P5P6_STATIC_VALUES: c_uint = 0x30C5;
pub const SUNI1x10GEXP_REG_TXOAM_INTERRUPT_ENABLE: c_uint = 0x30C6;
pub const SUNI1x10GEXP_REG_TXOAM_INTERRUPT_STATUS: c_uint = 0x30C7;
pub const SUNI1x10GEXP_REG_TXOAM_INSERT_COUNT_LSB: c_uint = 0x30C8;
pub const SUNI1x10GEXP_REG_TXOAM_INSERT_COUNT_MSB: c_uint = 0x30C9;
pub const SUNI1x10GEXP_REG_TXOAM_OAM_MINI_COUNT_LSB: c_uint = 0x30CA;
pub const SUNI1x10GEXP_REG_TXOAM_OAM_MINI_COUNT_MSB: c_uint = 0x30CB;
pub const SUNI1x10GEXP_REG_TXOAM_P1P2_MINI_MASK: c_uint = 0x30CC;
pub const SUNI1x10GEXP_REG_TXOAM_P3P4_MINI_MASK: c_uint = 0x30CD;
pub const SUNI1x10GEXP_REG_TXOAM_P5P6_MINI_MASK: c_uint = 0x30CE;
pub const SUNI1x10GEXP_REG_TXOAM_COSET: c_uint = 0x30CF;
pub const SUNI1x10GEXP_REG_TXOAM_EMPTY_FIFO_INS_OP_CNT_LSB: c_uint = 0x30D0;
pub const SUNI1x10GEXP_REG_TXOAM_EMPTY_FIFO_INS_OP_CNT_MSB: c_uint = 0x30D1;
pub const SUNI1x10GEXP_REG_TXOAM_STATIC_VALUE_MINI_COUNT_LSB: c_uint = 0x30D2;
pub const SUNI1x10GEXP_REG_TXOAM_STATIC_VALUE_MINI_COUNT_MSB: c_uint = 0x30D3;
pub const SUNI1x10GEXP_REG_EFLX_GLOBAL_CONFIG: c_uint = 0x3200;
pub const SUNI1x10GEXP_REG_EFLX_ERCU_GLOBAL_STATUS: c_uint = 0x3201;
pub const SUNI1x10GEXP_REG_EFLX_INDIR_CHANNEL_ADDRESS: c_uint = 0x3202;
pub const SUNI1x10GEXP_REG_EFLX_INDIR_FIFO_LOW_LIMIT: c_uint = 0x3203;
pub const SUNI1x10GEXP_REG_EFLX_INDIR_FIFO_HIGH_LIMIT: c_uint = 0x3204;
pub const SUNI1x10GEXP_REG_EFLX_INDIR_FULL_ALMOST_FULL_STATUS_AND_LIMIT: c_uint = 0x3205;
pub const SUNI1x10GEXP_REG_EFLX_INDIR_EMPTY_ALMOST_EMPTY_STATUS_AND_LIMIT: c_uint = 0x3206;
pub const SUNI1x10GEXP_REG_EFLX_INDIR_FIFO_CUT_THROUGH_THRESHOLD: c_uint = 0x3207;
pub const SUNI1x10GEXP_REG_EFLX_FIFO_OVERFLOW_ERROR_ENABLE: c_uint = 0x320C;
pub const SUNI1x10GEXP_REG_EFLX_FIFO_OVERFLOW_ERROR_INDICATION: c_uint = 0x320D;
pub const SUNI1x10GEXP_REG_EFLX_CHANNEL_PROVISION: c_uint = 0x3210;
pub const SUNI1x10GEXP_REG_PL4IDU_CONFIG: c_uint = 0x3280;
pub const SUNI1x10GEXP_REG_PL4IDU_INTERRUPT_MASK: c_uint = 0x3282;
pub const SUNI1x10GEXP_REG_PL4IDU_INTERRUPT: c_uint = 0x3283;
// ----------------------------------------
pub const SUNI1x10GEXP_REG_MAX_OFFSET: c_uint = 0x3480;
//
// -- End register offset definitions --
//
// SUNI-1x10GE-XP REGISTER BIT MASKS
//
pub const SUNI1x10GEXP_BITMSK_BITS_1: c_uint = 0x00001;
pub const SUNI1x10GEXP_BITMSK_BITS_2: c_uint = 0x00003;
pub const SUNI1x10GEXP_BITMSK_BITS_3: c_uint = 0x00007;
pub const SUNI1x10GEXP_BITMSK_BITS_4: c_uint = 0x0000f;
pub const SUNI1x10GEXP_BITMSK_BITS_5: c_uint = 0x0001f;
pub const SUNI1x10GEXP_BITMSK_BITS_6: c_uint = 0x0003f;
pub const SUNI1x10GEXP_BITMSK_BITS_7: c_uint = 0x0007f;
pub const SUNI1x10GEXP_BITMSK_BITS_8: c_uint = 0x000ff;
pub const SUNI1x10GEXP_BITMSK_BITS_9: c_uint = 0x001ff;
pub const SUNI1x10GEXP_BITMSK_BITS_10: c_uint = 0x003ff;
pub const SUNI1x10GEXP_BITMSK_BITS_11: c_uint = 0x007ff;
pub const SUNI1x10GEXP_BITMSK_BITS_12: c_uint = 0x00fff;
pub const SUNI1x10GEXP_BITMSK_BITS_13: c_uint = 0x01fff;
pub const SUNI1x10GEXP_BITMSK_BITS_14: c_uint = 0x03fff;
pub const SUNI1x10GEXP_BITMSK_BITS_15: c_uint = 0x07fff;
pub const SUNI1x10GEXP_BITMSK_BITS_16: c_uint = 0x0ffff;

// ----------------------------------------------------------------------------
// Register 0x0001: S/UNI-1x10GE-XP Product Revision
// Bit 3-0  REVISION
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_REVISION: c_uint = 0x000F;
// ----------------------------------------------------------------------------
// Register 0x0002: S/UNI-1x10GE-XP Configuration and Reset Control
// Bit 2  XAUI_ARESETB
// Bit 1  PL4_ARESETB
// Bit 0  DRESETB
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_XAUI_ARESET: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_PL4_ARESET: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_DRESETB: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0003: S/UNI-1x10GE-XP Loop Back and Miscellaneous Control
// Bit 11  PL4IO_OUTCLKSEL
// Bit 9   SYSPCSLB
// Bit 8   LINEPCSLB
// Bit 7   MSTAT_BYPASS
// Bit 6   RXXG_BYPASS
// Bit 5   TXXG_BYPASS
// Bit 4   SOP_PAD_EN
// Bit 1   LOS_INV
// Bit 0   OVERRIDE_LOS
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_OUTCLKSEL: c_uint = 0x0800;
pub const SUNI1x10GEXP_BITMSK_SYSPCSLB: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_LINEPCSLB: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_MSTAT_BYPASS: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_RXXG_BYPASS: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_TXXG_BYPASS: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_SOP_PAD_EN: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_LOS_INV: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_OVERRIDE_LOS: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0004: S/UNI-1x10GE-XP Device Status
// Bit 9 TOP_SXRA_EXPIRED
// Bit 8 TOP_MDIO_BUSY
// Bit 7 TOP_DTRB
// Bit 6 TOP_EXPIRED
// Bit 5 TOP_PAUSED
// Bit 4 TOP_PL4_ID_DOOL
// Bit 3 TOP_PL4_IS_DOOL
// Bit 2 TOP_PL4_ID_ROOL
// Bit 1 TOP_PL4_IS_ROOL
// Bit 0 TOP_PL4_OUT_ROOL
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TOP_SXRA_EXPIRED: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_TOP_MDIO_BUSY: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_TOP_DTRB: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_TOP_EXPIRED: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_TOP_PAUSED: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4_ID_DOOL: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4_IS_DOOL: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4_ID_ROOL: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4_IS_ROOL: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4_OUT_ROOL: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0005: Global Performance Update and Clock Monitors
// Bit 15 TIP
// Bit 8  XAUI_REF_CLKA
// Bit 7  RXLANE3CLKA
// Bit 6  RXLANE2CLKA
// Bit 5  RXLANE1CLKA
// Bit 4  RXLANE0CLKA
// Bit 3  CSUCLKA
// Bit 2  TDCLKA
// Bit 1  RSCLKA
// Bit 0  RDCLKA
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TIP: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_XAUI_REF_CLKA: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXLANE3CLKA: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_RXLANE2CLKA: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_RXLANE1CLKA: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_RXLANE0CLKA: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_CSUCLKA: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_TDCLKA: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_RSCLKA: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_RDCLKA: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0006: MDIO Command
// Bit 4 MDIO_RDINC
// Bit 3 MDIO_RSTAT
// Bit 2 MDIO_LCTLD
// Bit 1 MDIO_LCTLA
// Bit 0 MDIO_SPRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MDIO_RDINC: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_MDIO_RSTAT: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_MDIO_LCTLD: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_MDIO_LCTLA: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_MDIO_SPRE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0007: MDIO Interrupt Enable
// Bit 0 MDIO_BUSY_EN
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MDIO_BUSY_EN: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0008: MDIO Interrupt Status
// Bit 0 MDIO_BUSYI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MDIO_BUSYI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0009: MMD PHY Address
// Bit 12-8 MDIO_DEVADR
// Bit 4-0 MDIO_PRTADR
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MDIO_DEVADR: c_uint = 0x1F00;
pub const SUNI1x10GEXP_BITOFF_MDIO_DEVADR: c_int = 8;
pub const SUNI1x10GEXP_BITMSK_MDIO_PRTADR: c_uint = 0x001F;
pub const SUNI1x10GEXP_BITOFF_MDIO_PRTADR: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x000C: OAM Interface Control
// Bit 6 MDO_OD_ENB
// Bit 5 MDI_INV
// Bit 4 MDI_SEL
// Bit 3 RXOAMEN
// Bit 2 RXOAMCLKEN
// Bit 1 TXOAMEN
// Bit 0 TXOAMCLKEN
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MDO_OD_ENB: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_MDI_INV: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_MDI_SEL: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_RXOAMEN: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_RXOAMCLKEN: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_TXOAMEN: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TXOAMCLKEN: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x000D: S/UNI-1x10GE-XP Master Interrupt Status
// Bit 15 TOP_PL4IO_INT
// Bit 14 TOP_IRAM_INT
// Bit 13 TOP_ERAM_INT
// Bit 12 TOP_XAUI_INT
// Bit 11 TOP_MSTAT_INT
// Bit 10 TOP_RXXG_INT
// Bit 9 TOP_TXXG_INT
// Bit 8 TOP_XRF_INT
// Bit 7 TOP_XTEF_INT
// Bit 6 TOP_MDIO_BUSY_INT
// Bit 5 TOP_RXOAM_INT
// Bit 4 TOP_TXOAM_INT
// Bit 3 TOP_IFLX_INT
// Bit 2 TOP_EFLX_INT
// Bit 1 TOP_PL4ODP_INT
// Bit 0 TOP_PL4IDU_INT
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TOP_PL4IO_INT: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_TOP_IRAM_INT: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_TOP_ERAM_INT: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_TOP_XAUI_INT: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_TOP_MSTAT_INT: c_uint = 0x0800;
pub const SUNI1x10GEXP_BITMSK_TOP_RXXG_INT: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_TOP_TXXG_INT: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_TOP_XRF_INT: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_TOP_XTEF_INT: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_TOP_MDIO_BUSY_INT: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_TOP_RXOAM_INT: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_TOP_TXOAM_INT: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_TOP_IFLX_INT: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_TOP_EFLX_INT: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4ODP_INT: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TOP_PL4IDU_INT: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x000E:PM3393 Global interrupt enable
// Bit 15 TOP_INTE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TOP_INTE: c_uint = 0x8000;
// ----------------------------------------------------------------------------
// Register 0x0010: XTEF Miscellaneous Control
// Bit 7 RF_VAL
// Bit 6 RF_OVERRIDE
// Bit 5 LF_VAL
// Bit 4 LF_OVERRIDE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RF_VAL: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_RF_OVERRIDE: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_LF_VAL: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_LF_OVERRIDE: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_LFRF_OVERRIDE_VAL: c_uint = 0x00F0;
// ----------------------------------------------------------------------------
// Register 0x0011: XRF Miscellaneous Control
// Bit 6-4 EN_IDLE_REP
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EN_IDLE_REP: c_uint = 0x0070;
// ----------------------------------------------------------------------------
// Register 0x0100: SERDES 3125 Configuration Register 1
// Bit 10 RXEQB_3
// Bit 8  RXEQB_2
// Bit 6  RXEQB_1
// Bit 4  RXEQB_0
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXEQB: c_uint = 0x0FF0;
pub const SUNI1x10GEXP_BITOFF_RXEQB_3: c_int = 10;
pub const SUNI1x10GEXP_BITOFF_RXEQB_2: c_int = 8;
pub const SUNI1x10GEXP_BITOFF_RXEQB_1: c_int = 6;
pub const SUNI1x10GEXP_BITOFF_RXEQB_0: c_int = 4;
// ----------------------------------------------------------------------------
// Register 0x0101: SERDES 3125 Configuration Register 2
// Bit 12 YSEL
// Bit  7 PRE_EMPH_3
// Bit  6 PRE_EMPH_2
// Bit  5 PRE_EMPH_1
// Bit  4 PRE_EMPH_0
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_YSEL: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_PRE_EMPH: c_uint = 0x00F0;
pub const SUNI1x10GEXP_BITMSK_PRE_EMPH_3: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_PRE_EMPH_2: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_PRE_EMPH_1: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_PRE_EMPH_0: c_uint = 0x0010;
// ----------------------------------------------------------------------------
// Register 0x0102: SERDES 3125 Interrupt Enable Register
// Bit 3 LASIE
// Bit 2 SPLL_RAE
// Bit 1 MPLL_RAE
// Bit 0 PLL_LOCKE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LASIE: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_SPLL_RAE: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_MPLL_RAE: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_PLL_LOCKE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0103: SERDES 3125 Interrupt Visibility Register
// Bit 3 LASIV
// Bit 2 SPLL_RAV
// Bit 1 MPLL_RAV
// Bit 0 PLL_LOCKV
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LASIV: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_SPLL_RAV: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_MPLL_RAV: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_PLL_LOCKV: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0104: SERDES 3125 Interrupt Status Register
// Bit 3 LASII
// Bit 2 SPLL_RAI
// Bit 1 MPLL_RAI
// Bit 0 PLL_LOCKI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LASII: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_SPLL_RAI: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_MPLL_RAI: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_PLL_LOCKI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x0107: SERDES 3125 Test Configuration
// Bit 12 DUALTX
// Bit 10 HC_1
// Bit  9 HC_0
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_DUALTX: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_HC: c_uint = 0x0600;
pub const SUNI1x10GEXP_BITOFF_HC_0: c_int = 9;
// ----------------------------------------------------------------------------
// Register 0x2040: RXXG Configuration 1
// Bit 15  RXXG_RXEN
// Bit 14  RXXG_ROCF
// Bit 13  RXXG_PAD_STRIP
// Bit 10  RXXG_PUREP
// Bit 9   RXXG_LONGP
// Bit 8   RXXG_PARF
// Bit 7   RXXG_FLCHK
// Bit 5   RXXG_PASS_CTRL
// Bit 3   RXXG_CRC_STRIP
// Bit 2-0 RXXG_MIFG
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_RXEN: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_RXXG_ROCF: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_RXXG_PAD_STRIP: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_RXXG_PUREP: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_RXXG_LONGP: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_RXXG_PARF: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXXG_FLCHK: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_RXXG_PASS_CTRL: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_RXXG_CRC_STRIP: c_uint = 0x0008;
// ----------------------------------------------------------------------------
// Register 0x02041: RXXG Configuration 2
// Bit 7-0 RXXG_HDRSIZE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_HDRSIZE: c_uint = 0x00FF;
// ----------------------------------------------------------------------------
// Register 0x2042: RXXG Configuration 3
// Bit 15 RXXG_MIN_LERRE
// Bit 14 RXXG_MAX_LERRE
// Bit 12 RXXG_LINE_ERRE
// Bit 10 RXXG_RX_OVRE
// Bit 9  RXXG_ADR_FILTERE
// Bit 8  RXXG_ERR_FILTERE
// Bit 5  RXXG_PRMB_ERRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_MIN_LERRE: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_RXXG_MAX_LERRE: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_RXXG_LINE_ERRE: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_RXXG_RX_OVRE: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_RXXG_ADR_FILTERE: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_RXXG_ERR_FILTERRE: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXXG_PRMB_ERRE: c_uint = 0x0020;
// ----------------------------------------------------------------------------
// Register 0x2043: RXXG Interrupt
// Bit 15 RXXG_MIN_LERRI
// Bit 14 RXXG_MAX_LERRI
// Bit 12 RXXG_LINE_ERRI
// Bit 10 RXXG_RX_OVRI
// Bit 9  RXXG_ADR_FILTERI
// Bit 8  RXXG_ERR_FILTERI
// Bit 5  RXXG_PRMB_ERRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_MIN_LERRI: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_RXXG_MAX_LERRI: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_RXXG_LINE_ERRI: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_RXXG_RX_OVRI: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_RXXG_ADR_FILTERI: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_RXXG_ERR_FILTERI: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXXG_PRMB_ERRE: c_uint = 0x0020;
// ----------------------------------------------------------------------------
// Register 0x2049: RXXG Receive FIFO Threshold
// Bit 2-0 RXXG_CUT_THRU
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_CUT_THRU: c_uint = 0x0007;
pub const SUNI1x10GEXP_BITOFF_RXXG_CUT_THRU: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2062H - 0x2069: RXXG Exact Match VID
// Bit 11-0 RXXG_VID_MATCH
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_VID_MATCH: c_uint = 0x0FFF;
pub const SUNI1x10GEXP_BITOFF_RXXG_VID_MATCH: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x206EH - 0x206F: RXXG Address Filter Control
// Bit 3 RXXG_FORWARD_ENABLE
// Bit 2 RXXG_VLAN_ENABLE
// Bit 1 RXXG_SRC_ADDR
// Bit 0 RXXG_MATCH_ENABLE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_FORWARD_ENABLE: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_RXXG_VLAN_ENABLE: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_RXXG_SRC_ADDR: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_RXXG_MATCH_ENABLE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2070: RXXG Address Filter Control 2
// Bit 1 RXXG_PMODE
// Bit 0 RXXG_MHASH_EN
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXXG_PMODE: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_RXXG_MHASH_EN: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2081: XRF Control Register 2
// Bit 6   EN_PKT_GEN
// Bit 4-2 PATT
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EN_PKT_GEN: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_PATT: c_uint = 0x001C;
pub const SUNI1x10GEXP_BITOFF_PATT: c_int = 2;
// ----------------------------------------------------------------------------
// Register 0x2088: XRF Interrupt Enable
// Bit 12-9 LANE_HICERE
// Bit 8-5  HS_SD_LANEE
// Bit 4    ALIGN_STATUS_ERRE
// Bit 3-0  LANE_SYNC_STAT_ERRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LANE_HICERE: c_uint = 0x1E00;
pub const SUNI1x10GEXP_BITOFF_LANE_HICERE: c_int = 9;
pub const SUNI1x10GEXP_BITMSK_HS_SD_LANEE: c_uint = 0x01E0;
pub const SUNI1x10GEXP_BITOFF_HS_SD_LANEE: c_int = 5;
pub const SUNI1x10GEXP_BITMSK_ALIGN_STATUS_ERRE: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_LANE_SYNC_STAT_ERRE: c_uint = 0x000F;
pub const SUNI1x10GEXP_BITOFF_LANE_SYNC_STAT_ERRE: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2089: XRF Interrupt Status
// Bit 12-9 LANE_HICERI
// Bit 8-5  HS_SD_LANEI
// Bit 4    ALIGN_STATUS_ERRI
// Bit 3-0  LANE_SYNC_STAT_ERRI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LANE_HICERI: c_uint = 0x1E00;
pub const SUNI1x10GEXP_BITOFF_LANE_HICERI: c_int = 9;
pub const SUNI1x10GEXP_BITMSK_HS_SD_LANEI: c_uint = 0x01E0;
pub const SUNI1x10GEXP_BITOFF_HS_SD_LANEI: c_int = 5;
pub const SUNI1x10GEXP_BITMSK_ALIGN_STATUS_ERRI: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_LANE_SYNC_STAT_ERRI: c_uint = 0x000F;
pub const SUNI1x10GEXP_BITOFF_LANE_SYNC_STAT_ERRI: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x208A: XRF Error Status
// Bit 8-5  HS_SD_LANE
// Bit 4    ALIGN_STATUS_ERR
// Bit 3-0  LANE_SYNC_STAT_ERR
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_HS_SD_LANE3: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_HS_SD_LANE2: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_HS_SD_LANE1: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_HS_SD_LANE0: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_ALIGN_STATUS_ERR: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_LANE3_SYNC_STAT_ERR: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_LANE2_SYNC_STAT_ERR: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_LANE1_SYNC_STAT_ERR: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_LANE0_SYNC_STAT_ERR: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x208B: XRF Diagnostic Interrupt Enable
// Bit 7-4 LANE_OVERRUNE
// Bit 3-0 LANE_UNDERRUNE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LANE_OVERRUNE: c_uint = 0x00F0;
pub const SUNI1x10GEXP_BITOFF_LANE_OVERRUNE: c_int = 4;
pub const SUNI1x10GEXP_BITMSK_LANE_UNDERRUNE: c_uint = 0x000F;
pub const SUNI1x10GEXP_BITOFF_LANE_UNDERRUNE: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x208C: XRF Diagnostic Interrupt Status
// Bit 7-4 LANE_OVERRUNI
// Bit 3-0 LANE_UNDERRUNI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_LANE_OVERRUNI: c_uint = 0x00F0;
pub const SUNI1x10GEXP_BITOFF_LANE_OVERRUNI: c_int = 4;
pub const SUNI1x10GEXP_BITMSK_LANE_UNDERRUNI: c_uint = 0x000F;
pub const SUNI1x10GEXP_BITOFF_LANE_UNDERRUNI: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x20C0: RXOAM Configuration
// Bit 15    RXOAM_BUSY
// Bit 14-12 RXOAM_F2_SEL
// Bit 10-8  RXOAM_F1_SEL
// Bit 7-6   RXOAM_FILTER_CTRL
// Bit 5-0   RXOAM_PX_EN
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_BUSY: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_RXOAM_F2_SEL: c_uint = 0x7000;
pub const SUNI1x10GEXP_BITOFF_RXOAM_F2_SEL: c_int = 12;
pub const SUNI1x10GEXP_BITMSK_RXOAM_F1_SEL: c_uint = 0x0700;
pub const SUNI1x10GEXP_BITOFF_RXOAM_F1_SEL: c_int = 8;
pub const SUNI1x10GEXP_BITMSK_RXOAM_FILTER_CTRL: c_uint = 0x00C0;
pub const SUNI1x10GEXP_BITOFF_RXOAM_FILTER_CTRL: c_int = 6;
pub const SUNI1x10GEXP_BITMSK_RXOAM_PX_EN: c_uint = 0x003F;
pub const SUNI1x10GEXP_BITOFF_RXOAM_PX_EN: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x20C1,0x20C2: RXOAM Filter Configuration
// Bit 15-8 RXOAM_FX_MASK
// Bit 7-0  RXOAM_FX_VAL
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_FX_MASK: c_uint = 0xFF00;
pub const SUNI1x10GEXP_BITOFF_RXOAM_FX_MASK: c_int = 8;
pub const SUNI1x10GEXP_BITMSK_RXOAM_FX_VAL: c_uint = 0x00FF;
pub const SUNI1x10GEXP_BITOFF_RXOAM_FX_VAl: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x20C3: RXOAM Configuration Register 2
// Bit 13    RXOAM_REC_BYTE_VAL
// Bit 11-10 RXOAM_BYPASS_MODE
// Bit 5-0   RXOAM_PX_CLEAR
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_REC_BYTE_VAL: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_RXOAM_BYPASS_MODE: c_uint = 0x0C00;
pub const SUNI1x10GEXP_BITOFF_RXOAM_BYPASS_MODE: c_int = 10;
pub const SUNI1x10GEXP_BITMSK_RXOAM_PX_CLEAR: c_uint = 0x003F;
pub const SUNI1x10GEXP_BITOFF_RXOAM_PX_CLEAR: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x20C4: RXOAM HEC Configuration
// Bit 15-8 RXOAM_COSET
// Bit 2    RXOAM_HEC_ERR_PKT
// Bit 0    RXOAM_HEC_EN
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_COSET: c_uint = 0xFF00;
pub const SUNI1x10GEXP_BITOFF_RXOAM_COSET: c_int = 8;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HEC_ERR_PKT: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HEC_EN: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x20C7: RXOAM Interrupt Enable
// Bit 10 RXOAM_FILTER_THRSHE
// Bit 9  RXOAM_OAM_ERRE
// Bit 8  RXOAM_HECE_THRSHE
// Bit 7  RXOAM_SOPE
// Bit 6  RXOAM_RFE
// Bit 5  RXOAM_LFE
// Bit 4  RXOAM_DV_ERRE
// Bit 3  RXOAM_DATA_INVALIDE
// Bit 2  RXOAM_FILTER_DROPE
// Bit 1  RXOAM_HECE
// Bit 0  RXOAM_OFLE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_FILTER_THRSHE: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_RXOAM_OAM_ERRE: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HECE_THRSHE: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXOAM_SOPE: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_RXOAM_RFE: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_RXOAM_LFE: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_RXOAM_DV_ERRE: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_RXOAM_DATA_INVALIDE: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_RXOAM_FILTER_DROPE: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HECE: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_RXOAM_OFLE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x20C8: RXOAM Interrupt Status
// Bit 10 RXOAM_FILTER_THRSHI
// Bit 9  RXOAM_OAM_ERRI
// Bit 8  RXOAM_HECE_THRSHI
// Bit 7  RXOAM_SOPI
// Bit 6  RXOAM_RFI
// Bit 5  RXOAM_LFI
// Bit 4  RXOAM_DV_ERRI
// Bit 3  RXOAM_DATA_INVALIDI
// Bit 2  RXOAM_FILTER_DROPI
// Bit 1  RXOAM_HECI
// Bit 0  RXOAM_OFLI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_FILTER_THRSHI: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_RXOAM_OAM_ERRI: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HECE_THRSHI: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXOAM_SOPI: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_RXOAM_RFI: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_RXOAM_LFI: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_RXOAM_DV_ERRI: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_RXOAM_DATA_INVALIDI: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_RXOAM_FILTER_DROPI: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HECI: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_RXOAM_OFLI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x20C9: RXOAM Status
// Bit 10 RXOAM_FILTER_THRSHV
// Bit 8  RXOAM_HECE_THRSHV
// Bit 6  RXOAM_RFV
// Bit 5  RXOAM_LFV
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_RXOAM_FILTER_THRSHV: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_RXOAM_HECE_THRSHV: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_RXOAM_RFV: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_RXOAM_LFV: c_uint = 0x0020;
// ----------------------------------------------------------------------------
// Register 0x2100: MSTAT Control
// Bit 2 MSTAT_WRITE
// Bit 1 MSTAT_CLEAR
// Bit 0 MSTAT_SNAP
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MSTAT_WRITE: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_MSTAT_CLEAR: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_MSTAT_SNAP: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2109: MSTAT Counter Write Address
// Bit 5-0 MSTAT_WRITE_ADDRESS
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_MSTAT_WRITE_ADDRESS: c_uint = 0x003F;
pub const SUNI1x10GEXP_BITOFF_MSTAT_WRITE_ADDRESS: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2200: IFLX Global Configuration Register
// Bit 15   IFLX_IRCU_ENABLE
// Bit 14   IFLX_IDSWT_ENABLE
// Bit 13-0 IFLX_IFD_CNT
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_IRCU_ENABLE: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_IFLX_IDSWT_ENABLE: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_IFLX_IFD_CNT: c_uint = 0x3FFF;
pub const SUNI1x10GEXP_BITOFF_IFLX_IFD_CNT: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2209: IFLX FIFO Overflow Enable
// Bit 0 IFLX_OVFE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_OVFE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x220A: IFLX FIFO Overflow Interrupt
// Bit 0 IFLX_OVFI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_OVFI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x220D: IFLX Indirect Channel Address
// Bit 15 IFLX_BUSY
// Bit 14 IFLX_RWB
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_BUSY: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_IFLX_RWB: c_uint = 0x4000;
// ----------------------------------------------------------------------------
// Register 0x220E: IFLX Indirect Logical FIFO Low Limit & Provision
// Bit 9-0 IFLX_LOLIM
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_LOLIM: c_uint = 0x03FF;
pub const SUNI1x10GEXP_BITOFF_IFLX_LOLIM: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x220F: IFLX Indirect Logical FIFO High Limit
// Bit 9-0 IFLX_HILIM
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_HILIM: c_uint = 0x03FF;
pub const SUNI1x10GEXP_BITOFF_IFLX_HILIM: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2210: IFLX Indirect Full/Almost Full Status & Limit
// Bit 15   IFLX_FULL
// Bit 14   IFLX_AFULL
// Bit 13-0 IFLX_AFTH
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_FULL: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_IFLX_AFULL: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_IFLX_AFTH: c_uint = 0x3FFF;
pub const SUNI1x10GEXP_BITOFF_IFLX_AFTH: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2211: IFLX Indirect Empty/Almost Empty Status & Limit
// Bit 15   IFLX_EMPTY
// Bit 14   IFLX_AEMPTY
// Bit 13-0 IFLX_AETH
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_IFLX_EMPTY: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_IFLX_AEMPTY: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_IFLX_AETH: c_uint = 0x3FFF;
pub const SUNI1x10GEXP_BITOFF_IFLX_AETH: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2240: PL4MOS Configuration Register
// Bit 3 PL4MOS_RE_INIT
// Bit 2 PL4MOS_EN
// Bit 1 PL4MOS_NO_STATUS
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4MOS_RE_INIT: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_PL4MOS_EN: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_PL4MOS_NO_STATUS: c_uint = 0x0002;
// ----------------------------------------------------------------------------
// Register 0x2243: PL4MOS MaxBurst1 Register
// Bit 11-0 PL4MOS_MAX_BURST1
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4MOS_MAX_BURST1: c_uint = 0x0FFF;
pub const SUNI1x10GEXP_BITOFF_PL4MOS_MAX_BURST1: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2244: PL4MOS MaxBurst2 Register
// Bit 11-0 PL4MOS_MAX_BURST2
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4MOS_MAX_BURST2: c_uint = 0x0FFF;
pub const SUNI1x10GEXP_BITOFF_PL4MOS_MAX_BURST2: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2245: PL4MOS Transfer Size Register
// Bit 7-0 PL4MOS_MAX_TRANSFER
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4MOS_MAX_TRANSFER: c_uint = 0x00FF;
pub const SUNI1x10GEXP_BITOFF_PL4MOS_MAX_TRANSFER: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2280: PL4ODP Configuration
// Bit 15-12 PL4ODP_REPEAT_T
// Bit 8     PL4ODP_SOP_RULE
// Bit 1     PL4ODP_EN_PORTS
// Bit 0     PL4ODP_EN_DFWD
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4ODP_REPEAT_T: c_uint = 0xF000;
pub const SUNI1x10GEXP_BITOFF_PL4ODP_REPEAT_T: c_int = 12;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_SOP_RULE: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_EN_PORTS: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_EN_DFWD: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2282: PL4ODP Interrupt Mask
// Bit 0 PL4ODP_OUT_DISE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4ODP_OUT_DISE: c_uint = 0x0001;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_EOPEOBE: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_ERREOPE: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_MEOPE: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_MSOPE: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_ES_OVRE: c_uint = 0x0002;
// ----------------------------------------------------------------------------
// Register 0x2283: PL4ODP Interrupt
// Bit 0 PL4ODP_OUT_DISI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4ODP_OUT_DISI: c_uint = 0x0001;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_EOPEOBI: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_ERREOPI: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_MEOPI: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_PPE_MSOPI: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_PL4ODP_ES_OVRI: c_uint = 0x0002;
// ----------------------------------------------------------------------------
// Register 0x2300:  PL4IO Lock Detect Status
// Bit 15 PL4IO_OUT_ROOLV
// Bit 12 PL4IO_IS_ROOLV
// Bit 11 PL4IO_DIP2_ERRV
// Bit 8  PL4IO_ID_ROOLV
// Bit 4  PL4IO_IS_DOOLV
// Bit 0  PL4IO_ID_DOOLV
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_OUT_ROOLV: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_IS_ROOLV: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_DIP2_ERRV: c_uint = 0x0800;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ID_ROOLV: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_PL4IO_IS_DOOLV: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ID_DOOLV: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2301:  PL4IO Lock Detect Change
// Bit 15 PL4IO_OUT_ROOLI
// Bit 12 PL4IO_IS_ROOLI
// Bit 11 PL4IO_DIP2_ERRI
// Bit 8  PL4IO_ID_ROOLI
// Bit 4  PL4IO_IS_DOOLI
// Bit 0  PL4IO_ID_DOOLI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_OUT_ROOLI: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_IS_ROOLI: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_DIP2_ERRI: c_uint = 0x0800;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ID_ROOLI: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_PL4IO_IS_DOOLI: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ID_DOOLI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2302:  PL4IO Lock Detect Mask
// Bit 15 PL4IO_OUT_ROOLE
// Bit 12 PL4IO_IS_ROOLE
// Bit 11 PL4IO_DIP2_ERRE
// Bit 8  PL4IO_ID_ROOLE
// Bit 4  PL4IO_IS_DOOLE
// Bit 0  PL4IO_ID_DOOLE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_OUT_ROOLE: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_IS_ROOLE: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_DIP2_ERRE: c_uint = 0x0800;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ID_ROOLE: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_PL4IO_IS_DOOLE: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ID_DOOLE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x2303:  PL4IO Lock Detect Limits
// Bit 15-8 PL4IO_REF_LIMIT
// Bit 7-0  PL4IO_TRAN_LIMIT
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_REF_LIMIT: c_uint = 0xFF00;
pub const SUNI1x10GEXP_BITOFF_PL4IO_REF_LIMIT: c_int = 8;
pub const SUNI1x10GEXP_BITMSK_PL4IO_TRAN_LIMIT: c_uint = 0x00FF;
pub const SUNI1x10GEXP_BITOFF_PL4IO_TRAN_LIMIT: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2304:  PL4IO Calendar Repetitions
// Bit 15-8 PL4IO_IN_MUL
// Bit 7-0  PL4IO_OUT_MUL
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_IN_MUL: c_uint = 0xFF00;
pub const SUNI1x10GEXP_BITOFF_PL4IO_IN_MUL: c_int = 8;
pub const SUNI1x10GEXP_BITMSK_PL4IO_OUT_MUL: c_uint = 0x00FF;
pub const SUNI1x10GEXP_BITOFF_PL4IO_OUT_MUL: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x2305:  PL4IO Configuration
// Bit 15  PL4IO_DIP2_ERR_CHK
// Bit 11  PL4IO_ODAT_DIS
// Bit 10  PL4IO_TRAIN_DIS
// Bit 9   PL4IO_OSTAT_DIS
// Bit 8   PL4IO_ISTAT_DIS
// Bit 7   PL4IO_NO_ISTAT
// Bit 6   PL4IO_STAT_OUTSEL
// Bit 5   PL4IO_INSEL
// Bit 4   PL4IO_DLSEL
// Bit 1-0 PL4IO_OUTSEL
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IO_DIP2_ERR_CHK: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ODAT_DIS: c_uint = 0x0800;
pub const SUNI1x10GEXP_BITMSK_PL4IO_TRAIN_DIS: c_uint = 0x0400;
pub const SUNI1x10GEXP_BITMSK_PL4IO_OSTAT_DIS: c_uint = 0x0200;
pub const SUNI1x10GEXP_BITMSK_PL4IO_ISTAT_DIS: c_uint = 0x0100;
pub const SUNI1x10GEXP_BITMSK_PL4IO_NO_ISTAT: c_uint = 0x0080;
pub const SUNI1x10GEXP_BITMSK_PL4IO_STAT_OUTSEL: c_uint = 0x0040;
pub const SUNI1x10GEXP_BITMSK_PL4IO_INSEL: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_PL4IO_DLSEL: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_PL4IO_OUTSEL: c_uint = 0x0003;
pub const SUNI1x10GEXP_BITOFF_PL4IO_OUTSEL: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3040: TXXG Configuration Register 1
// Bit 15   TXXG_TXEN0
// Bit 13   TXXG_HOSTPAUSE
// Bit 12-7 TXXG_IPGT
// Bit 5    TXXG_32BIT_ALIGN
// Bit 4    TXXG_CRCEN
// Bit 3    TXXG_FCTX
// Bit 2    TXXG_FCRX
// Bit 1    TXXG_PADEN
// Bit 0    TXXG_SPRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_TXEN0: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_TXXG_HOSTPAUSE: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_TXXG_IPGT: c_uint = 0x1F80;
pub const SUNI1x10GEXP_BITOFF_TXXG_IPGT: c_int = 7;
pub const SUNI1x10GEXP_BITMSK_TXXG_32BIT_ALIGN: c_uint = 0x0020;
pub const SUNI1x10GEXP_BITMSK_TXXG_CRCEN: c_uint = 0x0010;
pub const SUNI1x10GEXP_BITMSK_TXXG_FCTX: c_uint = 0x0008;
pub const SUNI1x10GEXP_BITMSK_TXXG_FCRX: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_TXXG_PADEN: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TXXG_SPRE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3041: TXXG Configuration Register 2
// Bit 7-0   TXXG_HDRSIZE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_HDRSIZE: c_uint = 0x00FF;
// ----------------------------------------------------------------------------
// Register 0x3042: TXXG Configuration Register 3
// Bit 15 TXXG_FIFO_ERRE
// Bit 14 TXXG_FIFO_UDRE
// Bit 13 TXXG_MAX_LERRE
// Bit 12 TXXG_MIN_LERRE
// Bit 11 TXXG_XFERE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_FIFO_ERRE: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_TXXG_FIFO_UDRE: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_TXXG_MAX_LERRE: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_TXXG_MIN_LERRE: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_TXXG_XFERE: c_uint = 0x0800;
// ----------------------------------------------------------------------------
// Register 0x3043: TXXG Interrupt
// Bit 15 TXXG_FIFO_ERRI
// Bit 14 TXXG_FIFO_UDRI
// Bit 13 TXXG_MAX_LERRI
// Bit 12 TXXG_MIN_LERRI
// Bit 11 TXXG_XFERI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_FIFO_ERRI: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_TXXG_FIFO_UDRI: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_TXXG_MAX_LERRI: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_TXXG_MIN_LERRI: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_TXXG_XFERI: c_uint = 0x0800;
// ----------------------------------------------------------------------------
// Register 0x3044: TXXG Status Register
// Bit 1 TXXG_TXACTIVE
// Bit 0 TXXG_PAUSED
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_TXACTIVE: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TXXG_PAUSED: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3046: TXXG TX_MINFR -  Transmit Min Frame Size Register
// Bit 7-0 TXXG_TX_MINFR
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_TX_MINFR: c_uint = 0x00FF;
pub const SUNI1x10GEXP_BITOFF_TXXG_TX_MINFR: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3052: TXXG Pause Quantum Value Configuration Register
// Bit 7-0 TXXG_FC_PAUSE_QNTM
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXXG_FC_PAUSE_QNTM: c_uint = 0x00FF;
pub const SUNI1x10GEXP_BITOFF_TXXG_FC_PAUSE_QNTM: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3080: XTEF Control
// Bit 3-0 XTEF_FORCE_PARITY_ERR
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_XTEF_FORCE_PARITY_ERR: c_uint = 0x000F;
pub const SUNI1x10GEXP_BITOFF_XTEF_FORCE_PARITY_ERR: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3084: XTEF Interrupt Event Register
// Bit 0 XTEF_LOST_SYNCI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_XTEF_LOST_SYNCI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3085: XTEF Interrupt Enable Register
// Bit 0 XTEF_LOST_SYNCE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_XTEF_LOST_SYNCE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3086: XTEF Visibility Register
// Bit 0 XTEF_LOST_SYNCV
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_XTEF_LOST_SYNCV: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x30C0: TXOAM OAM Configuration
// Bit 15   TXOAM_HEC_EN
// Bit 14   TXOAM_EMPTYCODE_EN
// Bit 13   TXOAM_FORCE_IDLE
// Bit 12   TXOAM_IGNORE_IDLE
// Bit 11-6 TXOAM_PX_OVERWRITE
// Bit 5-0  TXOAM_PX_SEL
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXOAM_HEC_EN: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_EMPTYCODE_EN: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_FORCE_IDLE: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_IGNORE_IDLE: c_uint = 0x1000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_PX_OVERWRITE: c_uint = 0x0FC0;
pub const SUNI1x10GEXP_BITOFF_TXOAM_PX_OVERWRITE: c_int = 6;
pub const SUNI1x10GEXP_BITMSK_TXOAM_PX_SEL: c_uint = 0x003F;
pub const SUNI1x10GEXP_BITOFF_TXOAM_PX_SEL: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x30C1: TXOAM Mini-Packet Rate Configuration
// Bit 15   TXOAM_MINIDIS
// Bit 14   TXOAM_BUSY
// Bit 13   TXOAM_TRANS_EN
// Bit 10-0 TXOAM_MINIRATE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXOAM_MINIDIS: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_BUSY: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_TRANS_EN: c_uint = 0x2000;
pub const SUNI1x10GEXP_BITMSK_TXOAM_MINIRATE: c_uint = 0x07FF;
// ----------------------------------------------------------------------------
// Register 0x30C2: TXOAM Mini-Packet Gap and FIFO Configuration
// Bit 13-10 TXOAM_FTHRESH
// Bit 9-6   TXOAM_MINIPOST
// Bit 5-0   TXOAM_MINIPRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXOAM_FTHRESH: c_uint = 0x3C00;
pub const SUNI1x10GEXP_BITOFF_TXOAM_FTHRESH: c_int = 10;
pub const SUNI1x10GEXP_BITMSK_TXOAM_MINIPOST: c_uint = 0x03C0;
pub const SUNI1x10GEXP_BITOFF_TXOAM_MINIPOST: c_int = 6;
pub const SUNI1x10GEXP_BITMSK_TXOAM_MINIPRE: c_uint = 0x003F;
// ----------------------------------------------------------------------------
// Register 0x30C6: TXOAM Interrupt Enable
// Bit 2 TXOAM_SOP_ERRE
// Bit 1 TXOAM_OFLE
// Bit 0 TXOAM_ERRE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXOAM_SOP_ERRE: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_TXOAM_OFLE: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TXOAM_ERRE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x30C7: TXOAM Interrupt Status
// Bit 2 TXOAM_SOP_ERRI
// Bit 1 TXOAM_OFLI
// Bit 0 TXOAM_ERRI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXOAM_SOP_ERRI: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_TXOAM_OFLI: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_TXOAM_ERRI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x30CF: TXOAM Coset
// Bit 7-0 TXOAM_COSET
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_TXOAM_COSET: c_uint = 0x00FF;
// ----------------------------------------------------------------------------
// Register 0x3200: EFLX Global Configuration
// Bit 15 EFLX_ERCU_EN
// Bit 7  EFLX_EN_EDSWT
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_ERCU_EN: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_EFLX_EN_EDSWT: c_uint = 0x0080;
// ----------------------------------------------------------------------------
// Register 0x3201: EFLX ERCU Global Status
// Bit 13 EFLX_OVF_ERR
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_OVF_ERR: c_uint = 0x2000;
// ----------------------------------------------------------------------------
// Register 0x3202: EFLX Indirect Channel Address
// Bit 15 EFLX_BUSY
// Bit 14 EFLX_RDWRB
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_BUSY: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_EFLX_RDWRB: c_uint = 0x4000;
// ----------------------------------------------------------------------------
// Register 0x3203: EFLX Indirect Logical FIFO Low Limit
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_LOLIM: c_uint = 0x03FF;
pub const SUNI1x10GEXP_BITOFF_EFLX_LOLIM: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3204: EFLX Indirect Logical FIFO High Limit
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_HILIM: c_uint = 0x03FF;
pub const SUNI1x10GEXP_BITOFF_EFLX_HILIM: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3205: EFLX Indirect Full/Almost-Full Status and Limit
// Bit 15   EFLX_FULL
// Bit 14   EFLX_AFULL
// Bit 13-0 EFLX_AFTH
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_FULL: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_EFLX_AFULL: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_EFLX_AFTH: c_uint = 0x3FFF;
pub const SUNI1x10GEXP_BITOFF_EFLX_AFTH: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3206: EFLX Indirect Empty/Almost-Empty Status and Limit
// Bit 15   EFLX_EMPTY
// Bit 14   EFLX_AEMPTY
// Bit 13-0 EFLX_AETH
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_EMPTY: c_uint = 0x8000;
pub const SUNI1x10GEXP_BITMSK_EFLX_AEMPTY: c_uint = 0x4000;
pub const SUNI1x10GEXP_BITMSK_EFLX_AETH: c_uint = 0x3FFF;
pub const SUNI1x10GEXP_BITOFF_EFLX_AETH: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x3207: EFLX Indirect FIFO Cut-Through Threshold
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_CUT_THRU: c_uint = 0x3FFF;
pub const SUNI1x10GEXP_BITOFF_EFLX_CUT_THRU: c_int = 0;
// ----------------------------------------------------------------------------
// Register 0x320C: EFLX FIFO Overflow Error Enable
// Bit 0 EFLX_OVFE
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_OVFE: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x320D: EFLX FIFO Overflow Error Indication
// Bit 0 EFLX_OVFI
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_OVFI: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3210: EFLX Channel Provision
// Bit 0 EFLX_PROV
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_EFLX_PROV: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3280: PL4IDU Configuration
// Bit 2 PL4IDU_SYNCH_ON_TRAIN
// Bit 1 PL4IDU_EN_PORTS
// Bit 0 PL4IDU_EN_DFWD
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IDU_SYNCH_ON_TRAIN: c_uint = 0x0004;
pub const SUNI1x10GEXP_BITMSK_PL4IDU_EN_PORTS: c_uint = 0x0002;
pub const SUNI1x10GEXP_BITMSK_PL4IDU_EN_DFWD: c_uint = 0x0001;
// ----------------------------------------------------------------------------
// Register 0x3282: PL4IDU Interrupt Mask
// Bit 1 PL4IDU_DIP4E
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IDU_DIP4E: c_uint = 0x0002;
// ----------------------------------------------------------------------------
// Register 0x3283: PL4IDU Interrupt
// Bit 1 PL4IDU_DIP4I
// ----------------------------------------------------------------------------
pub const SUNI1x10GEXP_BITMSK_PL4IDU_DIP4I: c_uint = 0x0002;
