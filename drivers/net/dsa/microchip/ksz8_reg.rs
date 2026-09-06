//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/ksz8_reg.h
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
// Microchip KSZ8XXX series register definitions
//
// The base for these definitions is KSZ8795 but unless indicated
// differently by their prefix, they apply to all KSZ8 series
// devices. Registers and masks that do change are defined in
// dedicated structures in ksz_common.c.
//
// Copyright (c) 2017 Microchip Technology Inc.
// Tristram Ha <Tristram.Ha@microchip.com>
//
pub const KS_PORT_M: c_uint = 0x1F;
pub const KS_PRIO_M: c_uint = 0x3;
pub const KS_PRIO_S: c_int = 2;
pub const SW_REVISION_M: c_uint = 0x0E;
pub const SW_REVISION_S: c_int = 1;
pub const KSZ8863_REG_SW_RESET: c_uint = 0x43;

pub const KSZ88X3_REG_FVID_AND_HOST_MODE: c_uint = 0xC6;

pub const REG_SW_CTRL_0: c_uint = 0x02;

pub const REG_SW_CTRL_1: c_uint = 0x03;

pub const REG_SW_CTRL_2: c_uint = 0x04;

pub const REG_SW_CTRL_3: c_uint = 0x05;

pub const REG_SW_CTRL_4: c_uint = 0x06;

pub const REG_SW_CTRL_5: c_uint = 0x07;
pub const REG_SW_CTRL_6: c_uint = 0x08;

pub const REG_SW_CTRL_9: c_uint = 0x0B;
pub const SPI_CLK_125_MHZ: c_uint = 0x80;
pub const SPI_CLK_62_5_MHZ: c_uint = 0x40;
pub const SPI_CLK_31_25_MHZ: c_uint = 0x00;
pub const SW_LED_MODE_M: c_uint = 0x3;
pub const SW_LED_MODE_S: c_int = 4;
pub const SW_LED_LINK_ACT_SPEED: c_int = 0;
pub const SW_LED_LINK_ACT: c_int = 1;
pub const SW_LED_LINK_ACT_DUPLEX: c_int = 2;
pub const SW_LED_LINK_DUPLEX: c_int = 3;
pub const REG_SW_CTRL_10: c_uint = 0x0C;

pub const REG_SW_CTRL_11: c_uint = 0x0D;
pub const REG_POWER_MANAGEMENT_1: c_uint = 0x0E;

pub const SW_POWER_MANAGEMENT_MODE_M: c_uint = 0x3;
pub const SW_POWER_MANAGEMENT_MODE_S: c_int = 3;
pub const SW_POWER_NORMAL: c_int = 0;
pub const SW_ENERGY_DETECTION: c_int = 1;
pub const SW_SOFTWARE_POWER_DOWN: c_int = 2;
pub const REG_POWER_MANAGEMENT_2: c_uint = 0x0F;
pub const REG_PORT_1_CTRL_0: c_uint = 0x10;
pub const REG_PORT_2_CTRL_0: c_uint = 0x20;
pub const REG_PORT_3_CTRL_0: c_uint = 0x30;
pub const REG_PORT_4_CTRL_0: c_uint = 0x40;
pub const REG_PORT_5_CTRL_0: c_uint = 0x50;

pub const PORT_BASED_PRIO_S: c_int = 3;

pub const PORT_BASED_PRIO_0: c_int = 0;
pub const PORT_BASED_PRIO_1: c_int = 1;
pub const PORT_BASED_PRIO_2: c_int = 2;
pub const PORT_BASED_PRIO_3: c_int = 3;

pub const REG_PORT_1_CTRL_1: c_uint = 0x11;
pub const REG_PORT_2_CTRL_1: c_uint = 0x21;
pub const REG_PORT_3_CTRL_1: c_uint = 0x31;
pub const REG_PORT_4_CTRL_1: c_uint = 0x41;
pub const REG_PORT_5_CTRL_1: c_uint = 0x51;

pub const REG_PORT_1_CTRL_2: c_uint = 0x12;
pub const REG_PORT_2_CTRL_2: c_uint = 0x22;
pub const REG_PORT_3_CTRL_2: c_uint = 0x32;
pub const REG_PORT_4_CTRL_2: c_uint = 0x42;
pub const REG_PORT_5_CTRL_2: c_uint = 0x52;

pub const REG_PORT_1_CTRL_3: c_uint = 0x13;
pub const REG_PORT_2_CTRL_3: c_uint = 0x23;
pub const REG_PORT_3_CTRL_3: c_uint = 0x33;
pub const REG_PORT_4_CTRL_3: c_uint = 0x43;
pub const REG_PORT_5_CTRL_3: c_uint = 0x53;
pub const REG_PORT_1_CTRL_4: c_uint = 0x14;
pub const REG_PORT_2_CTRL_4: c_uint = 0x24;
pub const REG_PORT_3_CTRL_4: c_uint = 0x34;
pub const REG_PORT_4_CTRL_4: c_uint = 0x44;
pub const REG_PORT_5_CTRL_4: c_uint = 0x54;
pub const PORT_DEFAULT_VID: c_uint = 0x0001;
pub const REG_PORT_1_CTRL_5: c_uint = 0x15;
pub const REG_PORT_2_CTRL_5: c_uint = 0x25;
pub const REG_PORT_3_CTRL_5: c_uint = 0x35;
pub const REG_PORT_4_CTRL_5: c_uint = 0x45;
pub const REG_PORT_5_CTRL_5: c_uint = 0x55;

pub const PORT_AUTHEN_MODE: c_uint = 0x3;
pub const PORT_AUTHEN_PASS: c_int = 0;
pub const PORT_AUTHEN_BLOCK: c_int = 1;
pub const PORT_AUTHEN_TRAP: c_int = 2;
pub const REG_PORT_5_CTRL_6: c_uint = 0x56;

pub const REG_PORT_1_CTRL_7: c_uint = 0x17;
pub const REG_PORT_2_CTRL_7: c_uint = 0x27;
pub const REG_PORT_3_CTRL_7: c_uint = 0x37;
pub const REG_PORT_4_CTRL_7: c_uint = 0x47;

pub const REG_PORT_1_STATUS_0: c_uint = 0x18;
pub const REG_PORT_2_STATUS_0: c_uint = 0x28;
pub const REG_PORT_3_STATUS_0: c_uint = 0x38;
pub const REG_PORT_4_STATUS_0: c_uint = 0x48;
// KSZ87xx LinkMD registers (TABLE_LINK_MD_V)
pub const KSZ87XX_REG_DSP_EQ: c_uint = 0x08   /* DSP EQ initial value */;
pub const KSZ87XX_REG_PHY_LPF: c_uint = 0x4C   /* RX LPF bandwidth */;
// For KSZ8765.

pub const REG_PORT_1_STATUS_1: c_uint = 0x19;
pub const REG_PORT_2_STATUS_1: c_uint = 0x29;
pub const REG_PORT_3_STATUS_1: c_uint = 0x39;
pub const REG_PORT_4_STATUS_1: c_uint = 0x49;

pub const REG_PORT_1_LINK_MD_CTRL: c_uint = 0x1A;
pub const REG_PORT_2_LINK_MD_CTRL: c_uint = 0x2A;
pub const REG_PORT_3_LINK_MD_CTRL: c_uint = 0x3A;
pub const REG_PORT_4_LINK_MD_CTRL: c_uint = 0x4A;

pub const PORT_CABLE_DIAG_RESULT_S: c_int = 5;
pub const PORT_CABLE_STAT_NORMAL: c_int = 0;
pub const PORT_CABLE_STAT_OPEN: c_int = 1;
pub const PORT_CABLE_STAT_SHORT: c_int = 2;
pub const PORT_CABLE_STAT_FAILED: c_int = 3;

pub const PORT_CABLE_FAULT_COUNTER_H: c_uint = 0x01;
pub const REG_PORT_1_LINK_MD_RESULT: c_uint = 0x1B;
pub const REG_PORT_2_LINK_MD_RESULT: c_uint = 0x2B;
pub const REG_PORT_3_LINK_MD_RESULT: c_uint = 0x3B;
pub const REG_PORT_4_LINK_MD_RESULT: c_uint = 0x4B;
pub const PORT_CABLE_FAULT_COUNTER_L: c_uint = 0xFF;
pub const PORT_CABLE_FAULT_COUNTER: c_uint = 0x1FF;
pub const REG_PORT_1_CTRL_9: c_uint = 0x1C;
pub const REG_PORT_2_CTRL_9: c_uint = 0x2C;
pub const REG_PORT_3_CTRL_9: c_uint = 0x3C;
pub const REG_PORT_4_CTRL_9: c_uint = 0x4C;

pub const REG_PORT_1_CTRL_10: c_uint = 0x1D;
pub const REG_PORT_2_CTRL_10: c_uint = 0x2D;
pub const REG_PORT_3_CTRL_10: c_uint = 0x3D;
pub const REG_PORT_4_CTRL_10: c_uint = 0x4D;

pub const REG_PORT_1_STATUS_2: c_uint = 0x1E;
pub const REG_PORT_2_STATUS_2: c_uint = 0x2E;
pub const REG_PORT_3_STATUS_2: c_uint = 0x3E;
pub const REG_PORT_4_STATUS_2: c_uint = 0x4E;

pub const REG_PORT_1_STATUS_3: c_uint = 0x1F;
pub const REG_PORT_2_STATUS_3: c_uint = 0x2F;
pub const REG_PORT_3_STATUS_3: c_uint = 0x3F;
pub const REG_PORT_4_STATUS_3: c_uint = 0x4F;

pub const PORT_PHY_MODE_M: c_uint = 0x7;
pub const PHY_MODE_IN_AUTO_NEG: c_int = 1;
pub const PHY_MODE_10BT_HALF: c_int = 2;
pub const PHY_MODE_100BT_HALF: c_int = 3;
pub const PHY_MODE_10BT_FULL: c_int = 5;
pub const PHY_MODE_100BT_FULL: c_int = 6;
pub const PHY_MODE_ISOLDATE: c_int = 7;
pub const REG_PORT_CTRL_0: c_uint = 0x00;
pub const REG_PORT_CTRL_1: c_uint = 0x01;
pub const REG_PORT_CTRL_2: c_uint = 0x02;
pub const REG_PORT_CTRL_VID: c_uint = 0x03;
pub const REG_PORT_CTRL_5: c_uint = 0x05;
pub const REG_PORT_STATUS_1: c_uint = 0x09;
pub const REG_PORT_LINK_MD_CTRL: c_uint = 0x0A;
pub const REG_PORT_LINK_MD_RESULT: c_uint = 0x0B;
pub const REG_PORT_CTRL_9: c_uint = 0x0C;
pub const REG_PORT_CTRL_10: c_uint = 0x0D;
pub const REG_PORT_STATUS_3: c_uint = 0x0F;
pub const REG_PORT_CTRL_12: c_uint = 0xA0;
pub const REG_PORT_CTRL_13: c_uint = 0xA1;
pub const REG_PORT_RATE_CTRL_3: c_uint = 0xA2;
pub const REG_PORT_RATE_CTRL_2: c_uint = 0xA3;
pub const REG_PORT_RATE_CTRL_1: c_uint = 0xA4;
pub const REG_PORT_RATE_CTRL_0: c_uint = 0xA5;
pub const REG_PORT_RATE_LIMIT: c_uint = 0xA6;
pub const REG_PORT_IN_RATE_0: c_uint = 0xA7;
pub const REG_PORT_IN_RATE_1: c_uint = 0xA8;
pub const REG_PORT_IN_RATE_2: c_uint = 0xA9;
pub const REG_PORT_IN_RATE_3: c_uint = 0xAA;
pub const REG_PORT_OUT_RATE_0: c_uint = 0xAB;
pub const REG_PORT_OUT_RATE_1: c_uint = 0xAC;
pub const REG_PORT_OUT_RATE_2: c_uint = 0xAD;
pub const REG_PORT_OUT_RATE_3: c_uint = 0xAE;

pub const TABLE_EXT_SELECT_S: c_int = 5;
pub const TABLE_EEE_V: c_int = 1;
pub const TABLE_ACL_V: c_int = 2;
pub const TABLE_PME_V: c_int = 4;
pub const TABLE_LINK_MD_V: c_int = 5;

pub const TABLE_SELECT_S: c_int = 2;
pub const TABLE_STATIC_MAC_V: c_int = 0;
pub const TABLE_VLAN_V: c_int = 1;
pub const TABLE_DYNAMIC_MAC_V: c_int = 2;
pub const TABLE_MIB_V: c_int = 3;

pub const REG_IND_CTRL_1: c_uint = 0x6F;
pub const TABLE_ENTRY_MASK: c_uint = 0x03FF;
pub const TABLE_EXT_ENTRY_MASK: c_uint = 0x0FFF;
pub const REG_IND_DATA_5: c_uint = 0x73;
pub const REG_IND_DATA_2: c_uint = 0x76;
pub const REG_IND_DATA_1: c_uint = 0x77;
pub const REG_IND_DATA_0: c_uint = 0x78;
pub const REG_INT_STATUS: c_uint = 0x7C;
pub const REG_INT_ENABLE: c_uint = 0x7D;

pub const REG_ACL_INT_STATUS: c_uint = 0x7E;
pub const REG_ACL_INT_ENABLE: c_uint = 0x7F;

pub const REG_SW_CTRL_12: c_uint = 0x80;
pub const REG_SW_CTRL_13: c_uint = 0x81;
pub const SWITCH_802_1P_MASK: c_int = 3;
pub const SWITCH_802_1P_BASE: c_int = 3;
pub const SWITCH_802_1P_SHIFT: c_int = 2;

pub const REG_SWITCH_CTRL_14: c_uint = 0x82;

pub const SW_PRIO_MAPPING_S: c_int = 6;
pub const SW_PRIO_MAP_3_HI: c_int = 0;
pub const SW_PRIO_MAP_2_HI: c_int = 2;
pub const SW_PRIO_MAP_0_LO: c_int = 3;
pub const REG_SW_CTRL_15: c_uint = 0x83;
pub const REG_SW_CTRL_16: c_uint = 0x84;
pub const REG_SW_CTRL_17: c_uint = 0x85;
pub const REG_SW_CTRL_18: c_uint = 0x86;

pub const REG_SW_UNK_UCAST_CTRL: c_uint = 0x83;
pub const REG_SW_UNK_MCAST_CTRL: c_uint = 0x84;
pub const REG_SW_UNK_VID_CTRL: c_uint = 0x85;
pub const REG_SW_UNK_IP_MCAST_CTRL: c_uint = 0x86;

pub const REG_SW_CTRL_19: c_uint = 0x87;
pub const SW_IN_RATE_LIMIT_PERIOD_M: c_uint = 0x3;
pub const SW_IN_RATE_LIMIT_PERIOD_S: c_int = 4;
pub const SW_IN_RATE_LIMIT_16_MS: c_int = 0;
pub const SW_IN_RATE_LIMIT_64_MS: c_int = 1;
pub const SW_IN_RATE_LIMIT_256_MS: c_int = 2;

pub const REG_TOS_PRIO_CTRL_0: c_uint = 0x90;
pub const REG_TOS_PRIO_CTRL_1: c_uint = 0x91;
pub const REG_TOS_PRIO_CTRL_2: c_uint = 0x92;
pub const REG_TOS_PRIO_CTRL_3: c_uint = 0x93;
pub const REG_TOS_PRIO_CTRL_4: c_uint = 0x94;
pub const REG_TOS_PRIO_CTRL_5: c_uint = 0x95;
pub const REG_TOS_PRIO_CTRL_6: c_uint = 0x96;
pub const REG_TOS_PRIO_CTRL_7: c_uint = 0x97;
pub const REG_TOS_PRIO_CTRL_8: c_uint = 0x98;
pub const REG_TOS_PRIO_CTRL_9: c_uint = 0x99;
pub const REG_TOS_PRIO_CTRL_10: c_uint = 0x9A;
pub const REG_TOS_PRIO_CTRL_11: c_uint = 0x9B;
pub const REG_TOS_PRIO_CTRL_12: c_uint = 0x9C;
pub const REG_TOS_PRIO_CTRL_13: c_uint = 0x9D;
pub const REG_TOS_PRIO_CTRL_14: c_uint = 0x9E;
pub const REG_TOS_PRIO_CTRL_15: c_uint = 0x9F;

pub const REG_SW_CTRL_21: c_uint = 0xA4;

pub const REG_PORT_1_CTRL_12: c_uint = 0xB0;
pub const REG_PORT_2_CTRL_12: c_uint = 0xC0;
pub const REG_PORT_3_CTRL_12: c_uint = 0xD0;
pub const REG_PORT_4_CTRL_12: c_uint = 0xE0;
pub const REG_PORT_5_CTRL_12: c_uint = 0xF0;

pub const PORT_INS_TAG_FOR_PORT_5_S: c_int = 3;

pub const REG_PORT_1_CTRL_13: c_uint = 0xB1;
pub const REG_PORT_2_CTRL_13: c_uint = 0xC1;
pub const REG_PORT_3_CTRL_13: c_uint = 0xD1;
pub const REG_PORT_4_CTRL_13: c_uint = 0xE1;
pub const REG_PORT_5_CTRL_13: c_uint = 0xF1;

pub const REG_PORT_1_CTRL_14: c_uint = 0xB2;
pub const REG_PORT_2_CTRL_14: c_uint = 0xC2;
pub const REG_PORT_3_CTRL_14: c_uint = 0xD2;
pub const REG_PORT_4_CTRL_14: c_uint = 0xE2;
pub const REG_PORT_5_CTRL_14: c_uint = 0xF2;
pub const REG_PORT_1_CTRL_15: c_uint = 0xB3;
pub const REG_PORT_2_CTRL_15: c_uint = 0xC3;
pub const REG_PORT_3_CTRL_15: c_uint = 0xD3;
pub const REG_PORT_4_CTRL_15: c_uint = 0xE3;
pub const REG_PORT_5_CTRL_15: c_uint = 0xF3;
pub const REG_PORT_1_CTRL_16: c_uint = 0xB4;
pub const REG_PORT_2_CTRL_16: c_uint = 0xC4;
pub const REG_PORT_3_CTRL_16: c_uint = 0xD4;
pub const REG_PORT_4_CTRL_16: c_uint = 0xE4;
pub const REG_PORT_5_CTRL_16: c_uint = 0xF4;
pub const REG_PORT_1_CTRL_17: c_uint = 0xB5;
pub const REG_PORT_2_CTRL_17: c_uint = 0xC5;
pub const REG_PORT_3_CTRL_17: c_uint = 0xD5;
pub const REG_PORT_4_CTRL_17: c_uint = 0xE5;
pub const REG_PORT_5_CTRL_17: c_uint = 0xF5;
pub const REG_PORT_1_RATE_CTRL_3: c_uint = 0xB2;
pub const REG_PORT_1_RATE_CTRL_2: c_uint = 0xB3;
pub const REG_PORT_1_RATE_CTRL_1: c_uint = 0xB4;
pub const REG_PORT_1_RATE_CTRL_0: c_uint = 0xB5;
pub const REG_PORT_2_RATE_CTRL_3: c_uint = 0xC2;
pub const REG_PORT_2_RATE_CTRL_2: c_uint = 0xC3;
pub const REG_PORT_2_RATE_CTRL_1: c_uint = 0xC4;
pub const REG_PORT_2_RATE_CTRL_0: c_uint = 0xC5;
pub const REG_PORT_3_RATE_CTRL_3: c_uint = 0xD2;
pub const REG_PORT_3_RATE_CTRL_2: c_uint = 0xD3;
pub const REG_PORT_3_RATE_CTRL_1: c_uint = 0xD4;
pub const REG_PORT_3_RATE_CTRL_0: c_uint = 0xD5;
pub const REG_PORT_4_RATE_CTRL_3: c_uint = 0xE2;
pub const REG_PORT_4_RATE_CTRL_2: c_uint = 0xE3;
pub const REG_PORT_4_RATE_CTRL_1: c_uint = 0xE4;
pub const REG_PORT_4_RATE_CTRL_0: c_uint = 0xE5;
pub const REG_PORT_5_RATE_CTRL_3: c_uint = 0xF2;
pub const REG_PORT_5_RATE_CTRL_2: c_uint = 0xF3;
pub const REG_PORT_5_RATE_CTRL_1: c_uint = 0xF4;
pub const REG_PORT_5_RATE_CTRL_0: c_uint = 0xF5;

pub const REG_PORT_1_RATE_LIMIT: c_uint = 0xB6;
pub const REG_PORT_2_RATE_LIMIT: c_uint = 0xC6;
pub const REG_PORT_3_RATE_LIMIT: c_uint = 0xD6;
pub const REG_PORT_4_RATE_LIMIT: c_uint = 0xE6;
pub const REG_PORT_5_RATE_LIMIT: c_uint = 0xF6;
pub const PORT_IN_PORT_BASED_S: c_int = 6;
pub const PORT_RATE_PACKET_BASED_S: c_int = 5;
pub const PORT_IN_FLOW_CTRL_S: c_int = 4;
pub const PORT_IN_LIMIT_MODE_M: c_uint = 0x3;
pub const PORT_IN_LIMIT_MODE_S: c_int = 2;
pub const PORT_COUNT_IFG_S: c_int = 1;
pub const PORT_COUNT_PREAMBLE_S: c_int = 0;

pub const PORT_IN_ALL: c_int = 0;
pub const PORT_IN_UNICAST: c_int = 1;
pub const PORT_IN_MULTICAST: c_int = 2;
pub const PORT_IN_BROADCAST: c_int = 3;

pub const REG_PORT_1_IN_RATE_0: c_uint = 0xB7;
pub const REG_PORT_2_IN_RATE_0: c_uint = 0xC7;
pub const REG_PORT_3_IN_RATE_0: c_uint = 0xD7;
pub const REG_PORT_4_IN_RATE_0: c_uint = 0xE7;
pub const REG_PORT_5_IN_RATE_0: c_uint = 0xF7;
pub const REG_PORT_1_IN_RATE_1: c_uint = 0xB8;
pub const REG_PORT_2_IN_RATE_1: c_uint = 0xC8;
pub const REG_PORT_3_IN_RATE_1: c_uint = 0xD8;
pub const REG_PORT_4_IN_RATE_1: c_uint = 0xE8;
pub const REG_PORT_5_IN_RATE_1: c_uint = 0xF8;
pub const REG_PORT_1_IN_RATE_2: c_uint = 0xB9;
pub const REG_PORT_2_IN_RATE_2: c_uint = 0xC9;
pub const REG_PORT_3_IN_RATE_2: c_uint = 0xD9;
pub const REG_PORT_4_IN_RATE_2: c_uint = 0xE9;
pub const REG_PORT_5_IN_RATE_2: c_uint = 0xF9;
pub const REG_PORT_1_IN_RATE_3: c_uint = 0xBA;
pub const REG_PORT_2_IN_RATE_3: c_uint = 0xCA;
pub const REG_PORT_3_IN_RATE_3: c_uint = 0xDA;
pub const REG_PORT_4_IN_RATE_3: c_uint = 0xEA;
pub const REG_PORT_5_IN_RATE_3: c_uint = 0xFA;

pub const REG_PORT_1_OUT_RATE_0: c_uint = 0xBB;
pub const REG_PORT_2_OUT_RATE_0: c_uint = 0xCB;
pub const REG_PORT_3_OUT_RATE_0: c_uint = 0xDB;
pub const REG_PORT_4_OUT_RATE_0: c_uint = 0xEB;
pub const REG_PORT_5_OUT_RATE_0: c_uint = 0xFB;
pub const REG_PORT_1_OUT_RATE_1: c_uint = 0xBC;
pub const REG_PORT_2_OUT_RATE_1: c_uint = 0xCC;
pub const REG_PORT_3_OUT_RATE_1: c_uint = 0xDC;
pub const REG_PORT_4_OUT_RATE_1: c_uint = 0xEC;
pub const REG_PORT_5_OUT_RATE_1: c_uint = 0xFC;
pub const REG_PORT_1_OUT_RATE_2: c_uint = 0xBD;
pub const REG_PORT_2_OUT_RATE_2: c_uint = 0xCD;
pub const REG_PORT_3_OUT_RATE_2: c_uint = 0xDD;
pub const REG_PORT_4_OUT_RATE_2: c_uint = 0xED;
pub const REG_PORT_5_OUT_RATE_2: c_uint = 0xFD;
pub const REG_PORT_1_OUT_RATE_3: c_uint = 0xBE;
pub const REG_PORT_2_OUT_RATE_3: c_uint = 0xCE;
pub const REG_PORT_3_OUT_RATE_3: c_uint = 0xDE;
pub const REG_PORT_4_OUT_RATE_3: c_uint = 0xEE;
pub const REG_PORT_5_OUT_RATE_3: c_uint = 0xFE;
// 88x3 specific
pub const REG_SW_INSERT_SRC_PVID: c_uint = 0xC2;
// PME

// ACL
pub const ACL_FIRST_RULE_M: c_uint = 0xF;
pub const ACL_MODE_M: c_uint = 0x3;
pub const ACL_MODE_S: c_int = 4;
pub const ACL_MODE_DISABLE: c_int = 0;
pub const ACL_MODE_LAYER_2: c_int = 1;
pub const ACL_MODE_LAYER_3: c_int = 2;
pub const ACL_MODE_LAYER_4: c_int = 3;
pub const ACL_ENABLE_M: c_uint = 0x3;
pub const ACL_ENABLE_S: c_int = 2;
pub const ACL_ENABLE_2_COUNT: c_int = 0;
pub const ACL_ENABLE_2_TYPE: c_int = 1;
pub const ACL_ENABLE_2_MAC: c_int = 2;
pub const ACL_ENABLE_2_BOTH: c_int = 3;
pub const ACL_ENABLE_3_IP: c_int = 1;
pub const ACL_ENABLE_3_SRC_DST_COMP: c_int = 2;
pub const ACL_ENABLE_4_PROTOCOL: c_int = 0;
pub const ACL_ENABLE_4_TCP_PORT_COMP: c_int = 1;
pub const ACL_ENABLE_4_UDP_PORT_COMP: c_int = 2;
pub const ACL_ENABLE_4_TCP_SEQN_COMP: c_int = 3;

pub const ACL_MAX_PORT: c_uint = 0xFFFF;
pub const ACL_MIN_PORT: c_uint = 0xFFFF;
pub const ACL_IP_ADDR: c_uint = 0xFFFFFFFF;
pub const ACL_TCP_SEQNUM: c_uint = 0xFFFFFFFF;
pub const ACL_RESERVED: c_uint = 0xF8;
pub const ACL_PORT_MODE_M: c_uint = 0x3;
pub const ACL_PORT_MODE_S: c_int = 1;
pub const ACL_PORT_MODE_DISABLE: c_int = 0;
pub const ACL_PORT_MODE_EITHER: c_int = 1;
pub const ACL_PORT_MODE_IN_RANGE: c_int = 2;
pub const ACL_PORT_MODE_OUT_OF_RANGE: c_int = 3;

pub const ACL_TCP_FLAG_M: c_uint = 0xFF;
pub const ACL_TCP_FLAG: c_uint = 0xFF;
pub const ACL_ETH_TYPE: c_uint = 0xFFFF;
pub const ACL_IP_M: c_uint = 0xFFFFFFFF;
pub const ACL_PRIO_MODE_M: c_uint = 0x3;
pub const ACL_PRIO_MODE_S: c_int = 6;
pub const ACL_PRIO_MODE_DISABLE: c_int = 0;
pub const ACL_PRIO_MODE_HIGHER: c_int = 1;
pub const ACL_PRIO_MODE_LOWER: c_int = 2;
pub const ACL_PRIO_MODE_REPLACE: c_int = 3;
pub const ACL_PRIO_M: c_uint = 0x7;
pub const ACL_PRIO_S: c_int = 3;

pub const ACL_VLAN_PRIO_M: c_uint = 0x7;
pub const ACL_VLAN_PRIO_HI_M: c_uint = 0x3;
pub const ACL_VLAN_PRIO_LO_M: c_uint = 0x8;
pub const ACL_VLAN_PRIO_S: c_int = 7;
pub const ACL_MAP_MODE_M: c_uint = 0x3;
pub const ACL_MAP_MODE_S: c_int = 5;
pub const ACL_MAP_MODE_DISABLE: c_int = 0;
pub const ACL_MAP_MODE_OR: c_int = 1;
pub const ACL_MAP_MODE_AND: c_int = 2;
pub const ACL_MAP_MODE_REPLACE: c_int = 3;
pub const ACL_MAP_PORT_M: c_uint = 0x1F;

pub const ACL_CNT_S: c_int = 5;

pub const REG_PORT_ACL_BYTE_EN_MSB: c_uint = 0x10;
pub const ACL_BYTE_EN_MSB_M: c_uint = 0x3F;
pub const REG_PORT_ACL_BYTE_EN_LSB: c_uint = 0x11;
pub const ACL_ACTION_START: c_uint = 0xA;
pub const ACL_ACTION_LEN: c_int = 2;
pub const ACL_INTR_CNT_START: c_uint = 0xB;
pub const ACL_RULESET_START: c_uint = 0xC;
pub const ACL_RULESET_LEN: c_int = 2;
pub const ACL_TABLE_LEN: c_int = 14;
pub const ACL_ACTION_ENABLE: c_uint = 0x000C;
pub const ACL_MATCH_ENABLE: c_uint = 0x1FF0;
pub const ACL_RULESET_ENABLE: c_uint = 0x2003;

pub const REG_PORT_ACL_CTRL_0: c_uint = 0x12;

pub const PORT_ACL_INDEX_M: c_uint = 0xF;
pub const REG_PORT_ACL_CTRL_1: c_uint = 0x13;

pub const KSZ8795_ID_HI: c_uint = 0x0022;
pub const KSZ8795_ID_LO: c_uint = 0x1550;
pub const KSZ8863_ID_LO: c_uint = 0x1430;
pub const PHY_REG_LINK_MD: c_uint = 0x1D;

pub const PHY_CABLE_DIAG_RESULT: c_uint = 0x6000;
pub const PHY_CABLE_STAT_NORMAL: c_uint = 0x0000;
pub const PHY_CABLE_STAT_OPEN: c_uint = 0x2000;
pub const PHY_CABLE_STAT_SHORT: c_uint = 0x4000;
pub const PHY_CABLE_STAT_FAILED: c_uint = 0x6000;

pub const PHY_REG_PHY_CTRL: c_uint = 0x1F;
pub const PHY_MODE_M: c_uint = 0x7;
pub const PHY_MODE_S: c_int = 8;

// Vendor-specific Clause 22 PHY registers (virtualized)
pub const PHY_REG_KSZ87XX_SHORT_CABLE: c_uint = 0x1A;
pub const PHY_REG_KSZ87XX_LPF_BW: c_uint = 0x1B;
pub const PHY_REG_KSZ87XX_EQ_INIT: c_uint = 0x1C;
// LPF bandwidth bits [7:6]: 00 = 90MHz (default), 01 = 62MHz, 10 = 55MHz, 11 = 44MHz

// Low-loss workaround DSP EQ INIT VALUE

pub const KSZ87XX_DSP_EQ_INIT_LOW_LOSS: c_uint = 0x00;
pub const KSZ87XX_DSP_EQ_INIT_FACTORY: c_uint = 0x0F;
// KSZ8463 specific registers.
pub const P1MBCR: c_uint = 0x4C;
pub const P1MBSR: c_uint = 0x4E;
pub const PHY1ILR: c_uint = 0x50;
pub const PHY1IHR: c_uint = 0x52;
pub const P1ANAR: c_uint = 0x54;
pub const P1ANLPR: c_uint = 0x56;
pub const P2MBCR: c_uint = 0x58;
pub const P2MBSR: c_uint = 0x5A;
pub const PHY2ILR: c_uint = 0x5C;
pub const PHY2IHR: c_uint = 0x5E;
pub const P2ANAR: c_uint = 0x60;
pub const P2ANLPR: c_uint = 0x62;
pub const P1CR1: c_uint = 0x6C;
pub const P1CR2: c_uint = 0x6E;
pub const P1CR3: c_uint = 0x72;
pub const P1CR4: c_uint = 0x7E;
pub const P1SR: c_uint = 0x80;
pub const KSZ8463_FLUSH_TABLE_CTRL: c_uint = 0xAD;

pub const KSZ8463_REG_SW_CTRL_9: c_uint = 0xAE;
pub const KSZ8463_REG_CFG_CTRL: c_uint = 0xD8;

pub const PORT_COPPER_MODE_S: c_int = 6;
pub const KSZ8463_REG_SW_RESET: c_uint = 0x126;

pub const KSZ8463_PTP_CLK_CTRL: c_uint = 0x600;

pub const KSZ8463_PTP_MSG_CONF1: c_uint = 0x620;

pub const KSZ8463_REG_DSP_CTRL_6: c_uint = 0x734;

// Chip resource
pub const PRIO_QUEUES: c_int = 4;
pub const KS_PRIO_IN_REG: c_int = 4;
pub const MIB_COUNTER_NUM: c_uint = 0x20;
// Common names used by other drivers

//
pub const REG_IND_EEE_GLOB2_LO: c_uint = 0x34;
pub const REG_IND_EEE_GLOB2_HI: c_uint = 0x35;
//
// MIB_COUNTER_VALUE			00-00000000-3FFFFFFF
// MIB_TOTAL_BYTES			00-0000000F-FFFFFFFF
// MIB_PACKET_DROPPED			00-00000000-0000FFFF
// MIB_COUNTER_VALID			00-00000020-00000000
// MIB_COUNTER_OVERFLOW			00-00000040-00000000
//
pub const MIB_COUNTER_VALUE: c_uint = 0x3FFFFFFF;
pub const KSZ8795_MIB_TOTAL_RX_0: c_uint = 0x100;
pub const KSZ8795_MIB_TOTAL_TX_0: c_uint = 0x101;
pub const KSZ8795_MIB_TOTAL_RX_1: c_uint = 0x104;
pub const KSZ8795_MIB_TOTAL_TX_1: c_uint = 0x105;
pub const KSZ8863_MIB_PACKET_DROPPED_TX_0: c_uint = 0x100;
pub const KSZ8863_MIB_PACKET_DROPPED_RX_0: c_uint = 0x103;
pub const KSZ8895_MIB_PACKET_DROPPED_RX_0: c_uint = 0x105;
pub const MIB_PACKET_DROPPED: c_uint = 0x0000FFFF;
pub const MIB_TOTAL_BYTES_H: c_uint = 0x0000000F;

pub const FID_ENTRIES: c_int = 128;
pub const KSZ8_DYN_MAC_ENTRIES: c_int = 1024;
