//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwxgmac2.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 Synopsys, Inc. and/or its affiliates.
// stmmac XGMAC definitions.
//

// Misc
pub const XGMAC_JUMBO_LEN: c_int = 16368;
// MAC Registers
pub const XGMAC_TX_CONFIG: c_uint = 0x00000000;
pub const XGMAC_CONFIG_SS_OFF: c_int = 29;

pub const XGMAC_RX_CONFIG: c_uint = 0x00000004;

pub const XGMAC_CONFIG_HDSMS_SHIFT: c_int = 12;

pub const XGMAC_PACKET_FILTER: c_uint = 0x00000008;

pub const XGMAC_MAX_HASH_TABLE: c_int = 8;
pub const XGMAC_RXQ_CTRL0: c_uint = 0x000000a0;

pub const XGMAC_RXQ_CTRL1: c_uint = 0x000000a4;

pub const XGMAC_AVCPQ_SHIFT: c_int = 28;

pub const XGMAC_PTPQ_SHIFT: c_int = 24;

pub const XGMAC_DCBCPQ_SHIFT: c_int = 16;

pub const XGMAC_MCBCQ_SHIFT: c_int = 8;

pub const XGMAC_UPQ_SHIFT: c_int = 0;
pub const XGMAC_RXQ_CTRL2: c_uint = 0x000000a8;
pub const XGMAC_RXQ_CTRL3: c_uint = 0x000000ac;

pub const XGMAC_INT_STATUS: c_uint = 0x000000b0;

pub const XGMAC_INT_EN: c_uint = 0x000000b4;

pub const XGMAC_RX_FLOW_CTRL: c_uint = 0x00000090;

pub const XGMAC_PMT: c_uint = 0x000000c0;

pub const XGMAC_LPI_CTRL: c_uint = 0x000000d0;
// For definitions, see LPI_CTRL_STATUS_xxx in common.h
pub const XGMAC_LPI_TIMER_CTRL: c_uint = 0x000000d4;
pub const XGMAC_HW_FEATURE0: c_uint = 0x0000011c;

pub const XGMAC_HW_FEATURE1: c_uint = 0x00000120;

pub const XGMAC_HW_FEATURE2: c_uint = 0x00000124;

pub const XGMAC_HW_FEATURE3: c_uint = 0x00000128;

pub const XGMAC_HW_FEATURE4: c_uint = 0x0000012c;

pub const XGMAC_MAC_DPP_FSM_INT_STATUS: c_uint = 0x00000150;
pub const XGMAC_MAC_FSM_CONTROL: c_uint = 0x00000158;

pub const XGMAC_MDIO_ADDR: c_uint = 0x00000200;
pub const XGMAC_MDIO_DATA: c_uint = 0x00000204;
pub const XGMAC_MDIO_C22P: c_uint = 0x00000220;
pub const XGMAC_GPIO_STATUS: c_uint = 0x0000027c;

pub const XGMAC_ADDR_MAX: c_int = 32;

pub const XGMAC_L3L4_ADDR_CTRL: c_uint = 0x00000c00;

pub const XGMAC_L3L4_DATA: c_uint = 0x00000c04;
pub const XGMAC_L3L4_CTRL: c_uint = 0x0;

pub const XGMAC_L4_ADDR: c_uint = 0x1;

pub const XGMAC_L3_ADDR0: c_uint = 0x4;
pub const XGMAC_L3_ADDR1: c_uint = 0x5;
pub const XGMAC_L3_ADDR2: c_uint = 0x6;
pub const XMGAC_L3_ADDR3: c_uint = 0x7;
pub const XGMAC_ARP_ADDR: c_uint = 0x00000c10;
pub const XGMAC_RSS_CTRL: c_uint = 0x00000c80;

pub const XGMAC_RSS_ADDR: c_uint = 0x00000c88;
pub const XGMAC_RSSIA_SHIFT: c_int = 8;

pub const XGMAC_RSS_DATA: c_uint = 0x00000c8c;
pub const XGMAC_TIMESTAMP_STATUS: c_uint = 0x00000d20;

pub const XGMAC_TXTIMESTAMP_NSEC: c_uint = 0x00000d30;

pub const XGMAC_TXTIMESTAMP_SEC: c_uint = 0x00000d34;
pub const XGMAC_PPS_CONTROL: c_uint = 0x00000d70;

pub const XGMAC_PPSCMD_START: c_uint = 0x2;
pub const XGMAC_PPSCMD_STOP: c_uint = 0x5;

// MTL Registers
pub const XGMAC_MTL_OPMODE: c_uint = 0x00001000;

pub const XGMAC_MTL_INT_STATUS: c_uint = 0x00001020;
pub const XGMAC_MTL_RXQ_DMA_MAP0: c_uint = 0x00001030;
pub const XGMAC_MTL_RXQ_DMA_MAP1: c_uint = 0x00001034;

pub const XGMAC_TC_PRTY_MAP0: c_uint = 0x00001040;
pub const XGMAC_TC_PRTY_MAP1: c_uint = 0x00001044;

pub const XGMAC_MTL_RXP_CONTROL_STATUS: c_uint = 0x000010a0;

pub const XGMAC_MTL_RXP_IACC_CTRL_ST: c_uint = 0x000010b0;

pub const XGMAC_MTL_RXP_IACC_DATA: c_uint = 0x000010b4;
pub const XGMAC_MTL_ECC_CONTROL: c_uint = 0x000010c0;
pub const XGMAC_MTL_SAFETY_INT_STATUS: c_uint = 0x000010c4;

pub const XGMAC_MTL_ECC_INT_ENABLE: c_uint = 0x000010c8;

pub const XGMAC_MTL_ECC_INT_STATUS: c_uint = 0x000010cc;
pub const XGMAC_MTL_DPP_CONTROL: c_uint = 0x000010e0;

// DMA Registers
pub const XGMAC_DMA_MODE: c_uint = 0x00003000;

pub const XGMAC_INTM_MODE1: c_uint = 0x1;
pub const XGMAC_DMA_SYSBUS_MODE: c_uint = 0x00003004;

// XGMAC_BLEN* are now defined as DMA_AXI_BLEN* in common.h

pub const XGMAC_TX_EDMA_CTRL: c_uint = 0x00003040;

pub const XGMAC_RX_EDMA_CTRL: c_uint = 0x00003044;

pub const XGMAC_DMA_TBS_CTRL0: c_uint = 0x00003054;
pub const XGMAC_DMA_TBS_CTRL1: c_uint = 0x00003058;
pub const XGMAC_DMA_TBS_CTRL2: c_uint = 0x0000305c;
pub const XGMAC_DMA_TBS_CTRL3: c_uint = 0x00003060;

pub const XGMAC_DMA_SAFETY_INT_STATUS: c_uint = 0x00003064;

pub const XGMAC_DMA_ECC_INT_ENABLE: c_uint = 0x00003068;

pub const XGMAC_DMA_ECC_INT_STATUS: c_uint = 0x0000306c;
pub const XGMAC_DMA_DPP_INT_STATUS: c_uint = 0x00003074;

// Descriptors

pub const XGMAC_L34T_IP4TCP: c_uint = 0x1;
pub const XGMAC_L34T_IP4UDP: c_uint = 0x2;
pub const XGMAC_L34T_IP6TCP: c_uint = 0x9;
pub const XGMAC_L34T_IP6UDP: c_uint = 0xA;

// RDES0 (write back format)

// Error Type or L2 Type(ET/LT) Field Number
pub const XGMAC_ET_LT_VLAN_STAG: c_int = 8;
pub const XGMAC_ET_LT_VLAN_CTAG: c_int = 9;
pub const XGMAC_ET_LT_DVLAN_CTAG_CTAG: c_int = 10;
pub const XGMAC_ET_LT_DVLAN_STAG_STAG: c_int = 11;
pub const XGMAC_ET_LT_DVLAN_CTAG_STAG: c_int = 12;
pub const XGMAC_ET_LT_DVLAN_STAG_CTAG: c_int = 13;
