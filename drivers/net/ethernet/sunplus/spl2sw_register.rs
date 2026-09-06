//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sunplus/spl2sw_register.h
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
// Copyright Sunplus Technology Co., Ltd.
// All rights reserved.
//
// Register L2SW
pub const L2SW_SW_INT_STATUS_0: c_uint = 0x0;
pub const L2SW_SW_INT_MASK_0: c_uint = 0x4;
pub const L2SW_FL_CNTL_TH: c_uint = 0x8;
pub const L2SW_CPU_FL_CNTL_TH: c_uint = 0xc;
pub const L2SW_PRI_FL_CNTL: c_uint = 0x10;
pub const L2SW_VLAN_PRI_TH: c_uint = 0x14;
pub const L2SW_EN_TOS_BUS: c_uint = 0x18;
pub const L2SW_TOS_MAP0: c_uint = 0x1c;
pub const L2SW_TOS_MAP1: c_uint = 0x20;
pub const L2SW_TOS_MAP2: c_uint = 0x24;
pub const L2SW_TOS_MAP3: c_uint = 0x28;
pub const L2SW_TOS_MAP4: c_uint = 0x2c;
pub const L2SW_TOS_MAP5: c_uint = 0x30;
pub const L2SW_TOS_MAP6: c_uint = 0x34;
pub const L2SW_TOS_MAP7: c_uint = 0x38;
pub const L2SW_GLOBAL_QUE_STATUS: c_uint = 0x3c;
pub const L2SW_ADDR_TBL_SRCH: c_uint = 0x40;
pub const L2SW_ADDR_TBL_ST: c_uint = 0x44;
pub const L2SW_MAC_AD_SER0: c_uint = 0x48;
pub const L2SW_MAC_AD_SER1: c_uint = 0x4c;
pub const L2SW_WT_MAC_AD0: c_uint = 0x50;
pub const L2SW_W_MAC_15_0: c_uint = 0x54;
pub const L2SW_W_MAC_47_16: c_uint = 0x58;
pub const L2SW_PVID_CONFIG0: c_uint = 0x5c;
pub const L2SW_PVID_CONFIG1: c_uint = 0x60;
pub const L2SW_VLAN_MEMSET_CONFIG0: c_uint = 0x64;
pub const L2SW_VLAN_MEMSET_CONFIG1: c_uint = 0x68;
pub const L2SW_PORT_ABILITY: c_uint = 0x6c;
pub const L2SW_PORT_ST: c_uint = 0x70;
pub const L2SW_CPU_CNTL: c_uint = 0x74;
pub const L2SW_PORT_CNTL0: c_uint = 0x78;
pub const L2SW_PORT_CNTL1: c_uint = 0x7c;
pub const L2SW_PORT_CNTL2: c_uint = 0x80;
pub const L2SW_SW_GLB_CNTL: c_uint = 0x84;
pub const L2SW_L2SW_SW_RESET: c_uint = 0x88;
pub const L2SW_LED_PORT0: c_uint = 0x8c;
pub const L2SW_LED_PORT1: c_uint = 0x90;
pub const L2SW_LED_PORT2: c_uint = 0x94;
pub const L2SW_LED_PORT3: c_uint = 0x98;
pub const L2SW_LED_PORT4: c_uint = 0x9c;
pub const L2SW_WATCH_DOG_TRIG_RST: c_uint = 0xa0;
pub const L2SW_WATCH_DOG_STOP_CPU: c_uint = 0xa4;
pub const L2SW_PHY_CNTL_REG0: c_uint = 0xa8;
pub const L2SW_PHY_CNTL_REG1: c_uint = 0xac;
pub const L2SW_MAC_FORCE_MODE: c_uint = 0xb0;
pub const L2SW_VLAN_GROUP_CONFIG0: c_uint = 0xb4;
pub const L2SW_VLAN_GROUP_CONFIG1: c_uint = 0xb8;
pub const L2SW_FLOW_CTRL_TH3: c_uint = 0xbc;
pub const L2SW_QUEUE_STATUS_0: c_uint = 0xc0;
pub const L2SW_DEBUG_CNTL: c_uint = 0xc4;
pub const L2SW_RESERVED_1: c_uint = 0xc8;
pub const L2SW_MEM_TEST_INFO: c_uint = 0xcc;
pub const L2SW_SW_INT_STATUS_1: c_uint = 0xd0;
pub const L2SW_SW_INT_MASK_1: c_uint = 0xd4;
pub const L2SW_SW_GLOBAL_SIGNAL: c_uint = 0xd8;
pub const L2SW_CPU_TX_TRIG: c_uint = 0x208;
pub const L2SW_TX_HBASE_ADDR_0: c_uint = 0x20c;
pub const L2SW_TX_LBASE_ADDR_0: c_uint = 0x210;
pub const L2SW_RX_HBASE_ADDR_0: c_uint = 0x214;
pub const L2SW_RX_LBASE_ADDR_0: c_uint = 0x218;
pub const L2SW_TX_HW_ADDR_0: c_uint = 0x21c;
pub const L2SW_TX_LW_ADDR_0: c_uint = 0x220;
pub const L2SW_RX_HW_ADDR_0: c_uint = 0x224;
pub const L2SW_RX_LW_ADDR_0: c_uint = 0x228;
pub const L2SW_CPU_PORT_CNTL_REG_0: c_uint = 0x22c;
pub const L2SW_TX_HBASE_ADDR_1: c_uint = 0x230;
pub const L2SW_TX_LBASE_ADDR_1: c_uint = 0x234;
pub const L2SW_RX_HBASE_ADDR_1: c_uint = 0x238;
pub const L2SW_RX_LBASE_ADDR_1: c_uint = 0x23c;
pub const L2SW_TX_HW_ADDR_1: c_uint = 0x240;
pub const L2SW_TX_LW_ADDR_1: c_uint = 0x244;
pub const L2SW_RX_HW_ADDR_1: c_uint = 0x248;
pub const L2SW_RX_LW_ADDR_1: c_uint = 0x24c;
pub const L2SW_CPU_PORT_CNTL_REG_1: c_uint = 0x250;
