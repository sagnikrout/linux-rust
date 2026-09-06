//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/regs.h
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
// File: regs.h
// $Revision: 1.8 $
// $Date: 2005/06/21 18:29:48 $
// Description:
// part of the Chelsio 10Gb Ethernet Driver.
//
// http://www.chelsio.com
//
// Copyright (c) 2003 - 2005 Chelsio Communications, Inc.
// All rights reserved.
//
// Maintainers: maintainers@chelsio.com
//
// Authors: Dimitrios Michailidis   <dm@chelsio.com>
// Tina Yang               <tainay@chelsio.com>
// Felix Marti             <felix@chelsio.com>
// Scott Bardone           <sbardone@chelsio.com>
// Kurt Ottaway            <kottaway@chelsio.com>
// Frank DiMambro          <frank@chelsio.com>
//
// History:
//
// SGE registers
pub const A_SG_CONTROL: c_uint = 0x0;
pub const S_CMDQ0_ENABLE: c_int = 0;

pub const S_CMDQ1_ENABLE: c_int = 1;

pub const S_FL0_ENABLE: c_int = 2;

pub const S_FL1_ENABLE: c_int = 3;

pub const S_CPL_ENABLE: c_int = 4;

pub const S_RESPONSE_QUEUE_ENABLE: c_int = 5;

pub const S_CMDQ_PRIORITY: c_int = 6;
pub const M_CMDQ_PRIORITY: c_uint = 0x3;

pub const S_DISABLE_CMDQ0_GTS: c_int = 8;

pub const S_DISABLE_CMDQ1_GTS: c_int = 9;

pub const S_DISABLE_FL0_GTS: c_int = 10;

pub const S_DISABLE_FL1_GTS: c_int = 11;

pub const S_ENABLE_BIG_ENDIAN: c_int = 12;

pub const S_FL_SELECTION_CRITERIA: c_int = 13;

pub const S_ISCSI_COALESCE: c_int = 14;

pub const S_RX_PKT_OFFSET: c_int = 15;
pub const M_RX_PKT_OFFSET: c_uint = 0x7;

pub const S_VLAN_XTRACT: c_int = 18;

pub const A_SG_DOORBELL: c_uint = 0x4;
pub const A_SG_CMD0BASELWR: c_uint = 0x8;
pub const A_SG_CMD0BASEUPR: c_uint = 0xc;
pub const A_SG_CMD1BASELWR: c_uint = 0x10;
pub const A_SG_CMD1BASEUPR: c_uint = 0x14;
pub const A_SG_FL0BASELWR: c_uint = 0x18;
pub const A_SG_FL0BASEUPR: c_uint = 0x1c;
pub const A_SG_FL1BASELWR: c_uint = 0x20;
pub const A_SG_FL1BASEUPR: c_uint = 0x24;
pub const A_SG_CMD0SIZE: c_uint = 0x28;
pub const S_CMDQ0_SIZE: c_int = 0;
pub const M_CMDQ0_SIZE: c_uint = 0x1ffff;

pub const A_SG_FL0SIZE: c_uint = 0x2c;
pub const S_FL0_SIZE: c_int = 0;
pub const M_FL0_SIZE: c_uint = 0x1ffff;

pub const A_SG_RSPSIZE: c_uint = 0x30;
pub const S_RESPQ_SIZE: c_int = 0;
pub const M_RESPQ_SIZE: c_uint = 0x1ffff;

pub const A_SG_RSPBASELWR: c_uint = 0x34;
pub const A_SG_RSPBASEUPR: c_uint = 0x38;
pub const A_SG_FLTHRESHOLD: c_uint = 0x3c;
pub const S_FL_THRESHOLD: c_int = 0;
pub const M_FL_THRESHOLD: c_uint = 0xffff;

pub const A_SG_RSPQUEUECREDIT: c_uint = 0x40;
pub const S_RESPQ_CREDIT: c_int = 0;
pub const M_RESPQ_CREDIT: c_uint = 0x1ffff;

pub const A_SG_SLEEPING: c_uint = 0x48;
pub const S_SLEEPING: c_int = 0;
pub const M_SLEEPING: c_uint = 0xffff;

pub const A_SG_INTRTIMER: c_uint = 0x4c;
pub const S_INTERRUPT_TIMER_COUNT: c_int = 0;
pub const M_INTERRUPT_TIMER_COUNT: c_uint = 0xffffff;

pub const A_SG_CMD0PTR: c_uint = 0x50;
pub const S_CMDQ0_POINTER: c_int = 0;
pub const M_CMDQ0_POINTER: c_uint = 0xffff;

pub const S_CURRENT_GENERATION_BIT: c_int = 16;

pub const A_SG_CMD1PTR: c_uint = 0x54;
pub const S_CMDQ1_POINTER: c_int = 0;
pub const M_CMDQ1_POINTER: c_uint = 0xffff;

pub const A_SG_FL0PTR: c_uint = 0x58;
pub const S_FL0_POINTER: c_int = 0;
pub const M_FL0_POINTER: c_uint = 0xffff;

pub const A_SG_FL1PTR: c_uint = 0x5c;
pub const S_FL1_POINTER: c_int = 0;
pub const M_FL1_POINTER: c_uint = 0xffff;

pub const A_SG_VERSION: c_uint = 0x6c;
pub const S_DAY: c_int = 0;
pub const M_DAY: c_uint = 0x1f;

pub const S_MONTH: c_int = 5;
pub const M_MONTH: c_uint = 0xf;

pub const A_SG_CMD1SIZE: c_uint = 0xb0;
pub const S_CMDQ1_SIZE: c_int = 0;
pub const M_CMDQ1_SIZE: c_uint = 0x1ffff;

pub const A_SG_FL1SIZE: c_uint = 0xb4;
pub const S_FL1_SIZE: c_int = 0;
pub const M_FL1_SIZE: c_uint = 0x1ffff;

pub const A_SG_INT_ENABLE: c_uint = 0xb8;
pub const S_RESPQ_EXHAUSTED: c_int = 0;

pub const S_RESPQ_OVERFLOW: c_int = 1;

pub const S_FL_EXHAUSTED: c_int = 2;

pub const S_PACKET_TOO_BIG: c_int = 3;

pub const S_PACKET_MISMATCH: c_int = 4;

pub const A_SG_INT_CAUSE: c_uint = 0xbc;
pub const A_SG_RESPACCUTIMER: c_uint = 0xc0;
// MC3 registers
pub const A_MC3_CFG: c_uint = 0x100;
pub const S_CLK_ENABLE: c_int = 0;

pub const S_READY: c_int = 1;

pub const S_READ_TO_WRITE_DELAY: c_int = 2;
pub const M_READ_TO_WRITE_DELAY: c_uint = 0x7;

pub const S_WRITE_TO_READ_DELAY: c_int = 5;
pub const M_WRITE_TO_READ_DELAY: c_uint = 0x7;

pub const S_MC3_BANK_CYCLE: c_int = 8;
pub const M_MC3_BANK_CYCLE: c_uint = 0xf;

pub const S_REFRESH_CYCLE: c_int = 12;
pub const M_REFRESH_CYCLE: c_uint = 0xf;

pub const S_PRECHARGE_CYCLE: c_int = 16;
pub const M_PRECHARGE_CYCLE: c_uint = 0x3;

pub const S_ACTIVE_TO_READ_WRITE_DELAY: c_int = 18;

pub const S_ACTIVE_TO_PRECHARGE_DELAY: c_int = 19;
pub const M_ACTIVE_TO_PRECHARGE_DELAY: c_uint = 0x7;

pub const S_WRITE_RECOVERY_DELAY: c_int = 22;
pub const M_WRITE_RECOVERY_DELAY: c_uint = 0x3;

pub const S_DENSITY: c_int = 24;
pub const M_DENSITY: c_uint = 0x3;

pub const S_ORGANIZATION: c_int = 26;

pub const S_BANKS: c_int = 27;

pub const S_UNREGISTERED: c_int = 28;

pub const S_MC3_WIDTH: c_int = 29;
pub const M_MC3_WIDTH: c_uint = 0x3;

pub const S_MC3_SLOW: c_int = 31;

pub const A_MC3_MODE: c_uint = 0x104;
pub const S_MC3_MODE: c_int = 0;
pub const M_MC3_MODE: c_uint = 0x3fff;

pub const S_BUSY: c_int = 31;

pub const A_MC3_EXT_MODE: c_uint = 0x108;
pub const S_MC3_EXTENDED_MODE: c_int = 0;
pub const M_MC3_EXTENDED_MODE: c_uint = 0x3fff;

pub const A_MC3_PRECHARG: c_uint = 0x10c;
pub const A_MC3_REFRESH: c_uint = 0x110;
pub const S_REFRESH_ENABLE: c_int = 0;

pub const S_REFRESH_DIVISOR: c_int = 1;
pub const M_REFRESH_DIVISOR: c_uint = 0x3fff;

pub const A_MC3_STROBE: c_uint = 0x114;
pub const S_MASTER_DLL_RESET: c_int = 0;

pub const S_MASTER_DLL_TAP_COUNT: c_int = 1;
pub const M_MASTER_DLL_TAP_COUNT: c_uint = 0xff;

pub const S_MASTER_DLL_LOCKED: c_int = 9;

pub const S_MASTER_DLL_MAX_TAP_COUNT: c_int = 10;

pub const S_MASTER_DLL_TAP_COUNT_OFFSET: c_int = 11;
pub const M_MASTER_DLL_TAP_COUNT_OFFSET: c_uint = 0x3f;

pub const S_SLAVE_DLL_RESET: c_int = 11;

pub const S_SLAVE_DLL_DELTA: c_int = 12;
pub const M_SLAVE_DLL_DELTA: c_uint = 0xf;

pub const S_SLAVE_DELAY_LINE_MANUAL_TAP_COUNT: c_int = 17;
pub const M_SLAVE_DELAY_LINE_MANUAL_TAP_COUNT: c_uint = 0x3f;

pub const S_SLAVE_DELAY_LINE_MANUAL_TAP_COUNT_ENABLE: c_int = 23;

pub const S_SLAVE_DELAY_LINE_TAP_COUNT: c_int = 24;
pub const M_SLAVE_DELAY_LINE_TAP_COUNT: c_uint = 0x3f;

pub const A_MC3_ECC_CNTL: c_uint = 0x118;
pub const S_ECC_GENERATION_ENABLE: c_int = 0;

pub const S_ECC_CHECK_ENABLE: c_int = 1;

pub const S_CORRECTABLE_ERROR_COUNT: c_int = 2;
pub const M_CORRECTABLE_ERROR_COUNT: c_uint = 0xff;

pub const S_UNCORRECTABLE_ERROR_COUNT: c_int = 10;
pub const M_UNCORRECTABLE_ERROR_COUNT: c_uint = 0xff;

pub const A_MC3_CE_ADDR: c_uint = 0x11c;
pub const S_MC3_CE_ADDR: c_int = 4;
pub const M_MC3_CE_ADDR: c_uint = 0xfffffff;

pub const A_MC3_CE_DATA0: c_uint = 0x120;
pub const A_MC3_CE_DATA1: c_uint = 0x124;
pub const A_MC3_CE_DATA2: c_uint = 0x128;
pub const A_MC3_CE_DATA3: c_uint = 0x12c;
pub const A_MC3_CE_DATA4: c_uint = 0x130;
pub const A_MC3_UE_ADDR: c_uint = 0x134;
pub const S_MC3_UE_ADDR: c_int = 4;
pub const M_MC3_UE_ADDR: c_uint = 0xfffffff;

pub const A_MC3_UE_DATA0: c_uint = 0x138;
pub const A_MC3_UE_DATA1: c_uint = 0x13c;
pub const A_MC3_UE_DATA2: c_uint = 0x140;
pub const A_MC3_UE_DATA3: c_uint = 0x144;
pub const A_MC3_UE_DATA4: c_uint = 0x148;
pub const A_MC3_BD_ADDR: c_uint = 0x14c;
pub const A_MC3_BD_DATA0: c_uint = 0x150;
pub const A_MC3_BD_DATA1: c_uint = 0x154;
pub const A_MC3_BD_DATA2: c_uint = 0x158;
pub const A_MC3_BD_DATA3: c_uint = 0x15c;
pub const A_MC3_BD_DATA4: c_uint = 0x160;
pub const A_MC3_BD_OP: c_uint = 0x164;
pub const S_BACK_DOOR_OPERATION: c_int = 0;

pub const A_MC3_BIST_ADDR_BEG: c_uint = 0x168;
pub const A_MC3_BIST_ADDR_END: c_uint = 0x16c;
pub const A_MC3_BIST_DATA: c_uint = 0x170;
pub const A_MC3_BIST_OP: c_uint = 0x174;
pub const S_OP: c_int = 0;

pub const S_DATA_PATTERN: c_int = 1;
pub const M_DATA_PATTERN: c_uint = 0x3;

pub const S_CONTINUOUS: c_int = 3;

pub const A_MC3_INT_ENABLE: c_uint = 0x178;
pub const S_MC3_CORR_ERR: c_int = 0;

pub const S_MC3_UNCORR_ERR: c_int = 1;

pub const S_MC3_PARITY_ERR: c_int = 2;
pub const M_MC3_PARITY_ERR: c_uint = 0xff;

pub const S_MC3_ADDR_ERR: c_int = 10;

pub const A_MC3_INT_CAUSE: c_uint = 0x17c;
// MC4 registers
pub const A_MC4_CFG: c_uint = 0x180;
pub const S_POWER_UP: c_int = 0;

pub const S_MC4_BANK_CYCLE: c_int = 8;
pub const M_MC4_BANK_CYCLE: c_uint = 0x7;

pub const S_MC4_NARROW: c_int = 24;

pub const S_MC4_SLOW: c_int = 25;

pub const S_MC4A_WIDTH: c_int = 24;
pub const M_MC4A_WIDTH: c_uint = 0x3;

pub const S_MC4A_SLOW: c_int = 26;

pub const A_MC4_MODE: c_uint = 0x184;
pub const S_MC4_MODE: c_int = 0;
pub const M_MC4_MODE: c_uint = 0x7fff;

pub const A_MC4_EXT_MODE: c_uint = 0x188;
pub const S_MC4_EXTENDED_MODE: c_int = 0;
pub const M_MC4_EXTENDED_MODE: c_uint = 0x7fff;

pub const A_MC4_REFRESH: c_uint = 0x190;
pub const A_MC4_STROBE: c_uint = 0x194;
pub const A_MC4_ECC_CNTL: c_uint = 0x198;
pub const A_MC4_CE_ADDR: c_uint = 0x19c;
pub const S_MC4_CE_ADDR: c_int = 4;
pub const M_MC4_CE_ADDR: c_uint = 0xffffff;

pub const A_MC4_CE_DATA0: c_uint = 0x1a0;
pub const A_MC4_CE_DATA1: c_uint = 0x1a4;
pub const A_MC4_CE_DATA2: c_uint = 0x1a8;
pub const A_MC4_CE_DATA3: c_uint = 0x1ac;
pub const A_MC4_CE_DATA4: c_uint = 0x1b0;
pub const A_MC4_UE_ADDR: c_uint = 0x1b4;
pub const S_MC4_UE_ADDR: c_int = 4;
pub const M_MC4_UE_ADDR: c_uint = 0xffffff;

pub const A_MC4_UE_DATA0: c_uint = 0x1b8;
pub const A_MC4_UE_DATA1: c_uint = 0x1bc;
pub const A_MC4_UE_DATA2: c_uint = 0x1c0;
pub const A_MC4_UE_DATA3: c_uint = 0x1c4;
pub const A_MC4_UE_DATA4: c_uint = 0x1c8;
pub const A_MC4_BD_ADDR: c_uint = 0x1cc;
pub const S_MC4_BACK_DOOR_ADDR: c_int = 0;
pub const M_MC4_BACK_DOOR_ADDR: c_uint = 0xfffffff;

pub const A_MC4_BD_DATA0: c_uint = 0x1d0;
pub const A_MC4_BD_DATA1: c_uint = 0x1d4;
pub const A_MC4_BD_DATA2: c_uint = 0x1d8;
pub const A_MC4_BD_DATA3: c_uint = 0x1dc;
pub const A_MC4_BD_DATA4: c_uint = 0x1e0;
pub const A_MC4_BD_OP: c_uint = 0x1e4;
pub const S_OPERATION: c_int = 0;

pub const A_MC4_BIST_ADDR_BEG: c_uint = 0x1e8;
pub const A_MC4_BIST_ADDR_END: c_uint = 0x1ec;
pub const A_MC4_BIST_DATA: c_uint = 0x1f0;
pub const A_MC4_BIST_OP: c_uint = 0x1f4;
pub const A_MC4_INT_ENABLE: c_uint = 0x1f8;
pub const S_MC4_CORR_ERR: c_int = 0;

pub const S_MC4_UNCORR_ERR: c_int = 1;

pub const S_MC4_ADDR_ERR: c_int = 2;

pub const A_MC4_INT_CAUSE: c_uint = 0x1fc;
// TPI registers
pub const A_TPI_ADDR: c_uint = 0x280;
pub const S_TPI_ADDRESS: c_int = 0;
pub const M_TPI_ADDRESS: c_uint = 0xffffff;

pub const A_TPI_WR_DATA: c_uint = 0x284;
pub const A_TPI_RD_DATA: c_uint = 0x288;
pub const A_TPI_CSR: c_uint = 0x28c;
pub const S_TPIWR: c_int = 0;

pub const S_TPIRDY: c_int = 1;

pub const S_INT_DIR: c_int = 31;

pub const A_TPI_PAR: c_uint = 0x29c;
pub const S_TPIPAR: c_int = 0;
pub const M_TPIPAR: c_uint = 0x7f;

// TP registers
pub const A_TP_IN_CONFIG: c_uint = 0x300;
pub const S_TP_IN_CSPI_TUNNEL: c_int = 0;

pub const S_TP_IN_CSPI_ETHERNET: c_int = 1;

pub const S_TP_IN_CSPI_CPL: c_int = 3;

pub const S_TP_IN_CSPI_POS: c_int = 4;

pub const S_TP_IN_CSPI_CHECK_IP_CSUM: c_int = 5;

pub const S_TP_IN_CSPI_CHECK_TCP_CSUM: c_int = 6;

pub const S_TP_IN_ESPI_TUNNEL: c_int = 7;

pub const S_TP_IN_ESPI_ETHERNET: c_int = 8;

pub const S_TP_IN_ESPI_CPL: c_int = 10;

pub const S_TP_IN_ESPI_POS: c_int = 11;

pub const S_TP_IN_ESPI_CHECK_IP_CSUM: c_int = 12;

pub const S_TP_IN_ESPI_CHECK_TCP_CSUM: c_int = 13;

pub const S_OFFLOAD_DISABLE: c_int = 14;

pub const A_TP_OUT_CONFIG: c_uint = 0x304;
pub const S_TP_OUT_C_ETH: c_int = 0;

pub const S_TP_OUT_CSPI_CPL: c_int = 2;

pub const S_TP_OUT_CSPI_POS: c_int = 3;

pub const S_TP_OUT_CSPI_GENERATE_IP_CSUM: c_int = 4;

pub const S_TP_OUT_CSPI_GENERATE_TCP_CSUM: c_int = 5;

pub const S_TP_OUT_ESPI_ETHERNET: c_int = 6;

pub const S_TP_OUT_ESPI_TAG_ETHERNET: c_int = 7;

pub const S_TP_OUT_ESPI_CPL: c_int = 8;

pub const S_TP_OUT_ESPI_POS: c_int = 9;

pub const S_TP_OUT_ESPI_GENERATE_IP_CSUM: c_int = 10;

pub const S_TP_OUT_ESPI_GENERATE_TCP_CSUM: c_int = 11;

pub const A_TP_GLOBAL_CONFIG: c_uint = 0x308;
pub const S_IP_TTL: c_int = 0;
pub const M_IP_TTL: c_uint = 0xff;

pub const S_TCAM_SERVER_REGION_USAGE: c_int = 8;
pub const M_TCAM_SERVER_REGION_USAGE: c_uint = 0x3;

pub const S_QOS_MAPPING: c_int = 10;

pub const S_TCP_CSUM: c_int = 11;

pub const S_UDP_CSUM: c_int = 12;

pub const S_IP_CSUM: c_int = 13;

pub const S_IP_ID_SPLIT: c_int = 14;

pub const S_PATH_MTU: c_int = 15;

pub const S_5TUPLE_LOOKUP: c_int = 17;
pub const M_5TUPLE_LOOKUP: c_uint = 0x3;

pub const S_IP_FRAGMENT_DROP: c_int = 19;

pub const S_PING_DROP: c_int = 20;

pub const S_PROTECT_MODE: c_int = 21;

pub const S_SYN_COOKIE_ALGORITHM: c_int = 22;

pub const S_ATTACK_FILTER: c_int = 23;

pub const S_INTERFACE_TYPE: c_int = 24;

pub const S_DISABLE_RX_FLOW_CONTROL: c_int = 25;

pub const S_SYN_COOKIE_PARAMETER: c_int = 26;
pub const M_SYN_COOKIE_PARAMETER: c_uint = 0x3f;

pub const A_TP_GLOBAL_RX_CREDITS: c_uint = 0x30c;
pub const A_TP_CM_SIZE: c_uint = 0x310;
pub const A_TP_CM_MM_BASE: c_uint = 0x314;
pub const S_CM_MEMMGR_BASE: c_int = 0;
pub const M_CM_MEMMGR_BASE: c_uint = 0xfffffff;

pub const A_TP_CM_TIMER_BASE: c_uint = 0x318;
pub const S_CM_TIMER_BASE: c_int = 0;
pub const M_CM_TIMER_BASE: c_uint = 0xfffffff;

pub const A_TP_PM_SIZE: c_uint = 0x31c;
pub const A_TP_PM_TX_BASE: c_uint = 0x320;
pub const A_TP_PM_DEFRAG_BASE: c_uint = 0x324;
pub const A_TP_PM_RX_BASE: c_uint = 0x328;
pub const A_TP_PM_RX_PG_SIZE: c_uint = 0x32c;
pub const A_TP_PM_RX_MAX_PGS: c_uint = 0x330;
pub const A_TP_PM_TX_PG_SIZE: c_uint = 0x334;
pub const A_TP_PM_TX_MAX_PGS: c_uint = 0x338;
pub const A_TP_TCP_OPTIONS: c_uint = 0x340;
pub const S_TIMESTAMP: c_int = 0;
pub const M_TIMESTAMP: c_uint = 0x3;

pub const S_WINDOW_SCALE: c_int = 2;
pub const M_WINDOW_SCALE: c_uint = 0x3;

pub const S_SACK: c_int = 4;
pub const M_SACK: c_uint = 0x3;

pub const S_ECN: c_int = 6;
pub const M_ECN: c_uint = 0x3;

pub const S_SACK_ALGORITHM: c_int = 8;
pub const M_SACK_ALGORITHM: c_uint = 0x3;

pub const S_MSS: c_int = 10;

pub const S_DEFAULT_PEER_MSS: c_int = 16;
pub const M_DEFAULT_PEER_MSS: c_uint = 0xffff;

pub const A_TP_DACK_CONFIG: c_uint = 0x344;
pub const S_DACK_MODE: c_int = 0;

pub const S_DACK_AUTO_MGMT: c_int = 1;

pub const S_DACK_AUTO_CAREFUL: c_int = 2;

pub const S_DACK_MSS_SELECTOR: c_int = 3;
pub const M_DACK_MSS_SELECTOR: c_uint = 0x3;

pub const S_DACK_BYTE_THRESHOLD: c_int = 5;
pub const M_DACK_BYTE_THRESHOLD: c_uint = 0xfffff;

pub const A_TP_PC_CONFIG: c_uint = 0x348;
pub const S_TP_ACCESS_LATENCY: c_int = 0;
pub const M_TP_ACCESS_LATENCY: c_uint = 0xf;

pub const S_HELD_FIN_DISABLE: c_int = 4;

pub const S_DDP_FC_ENABLE: c_int = 5;

pub const S_RDMA_ERR_ENABLE: c_int = 6;

pub const S_FAST_PDU_DELIVERY: c_int = 7;

pub const S_CLEAR_FIN: c_int = 8;

pub const S_DIS_TX_FILL_WIN_PUSH: c_int = 12;

pub const S_TP_PC_REV: c_int = 30;
pub const M_TP_PC_REV: c_uint = 0x3;

pub const A_TP_BACKOFF0: c_uint = 0x350;
pub const S_ELEMENT0: c_int = 0;
pub const M_ELEMENT0: c_uint = 0xff;

pub const S_ELEMENT1: c_int = 8;
pub const M_ELEMENT1: c_uint = 0xff;

pub const S_ELEMENT2: c_int = 16;
pub const M_ELEMENT2: c_uint = 0xff;

pub const S_ELEMENT3: c_int = 24;
pub const M_ELEMENT3: c_uint = 0xff;

pub const A_TP_BACKOFF1: c_uint = 0x354;
pub const A_TP_BACKOFF2: c_uint = 0x358;
pub const A_TP_BACKOFF3: c_uint = 0x35c;
pub const A_TP_PARA_REG0: c_uint = 0x360;
pub const S_VAR_MULT: c_int = 0;
pub const M_VAR_MULT: c_uint = 0xf;

pub const S_VAR_GAIN: c_int = 4;
pub const M_VAR_GAIN: c_uint = 0xf;

pub const S_SRTT_GAIN: c_int = 8;
pub const M_SRTT_GAIN: c_uint = 0xf;

pub const S_RTTVAR_INIT: c_int = 12;
pub const M_RTTVAR_INIT: c_uint = 0xf;

pub const S_DUP_THRESH: c_int = 20;
pub const M_DUP_THRESH: c_uint = 0xf;

pub const S_INIT_CONG_WIN: c_int = 24;
pub const M_INIT_CONG_WIN: c_uint = 0x7;

pub const A_TP_PARA_REG1: c_uint = 0x364;
pub const S_INITIAL_SLOW_START_THRESHOLD: c_int = 0;
pub const M_INITIAL_SLOW_START_THRESHOLD: c_uint = 0xffff;

pub const S_RECEIVE_BUFFER_SIZE: c_int = 16;
pub const M_RECEIVE_BUFFER_SIZE: c_uint = 0xffff;

pub const A_TP_PARA_REG2: c_uint = 0x368;
pub const S_RX_COALESCE_SIZE: c_int = 0;
pub const M_RX_COALESCE_SIZE: c_uint = 0xffff;

pub const S_MAX_RX_SIZE: c_int = 16;
pub const M_MAX_RX_SIZE: c_uint = 0xffff;

pub const A_TP_PARA_REG3: c_uint = 0x36c;
pub const S_RX_COALESCING_PSH_DELIVER: c_int = 0;

pub const S_RX_COALESCING_ENABLE: c_int = 1;

pub const S_TAHOE_ENABLE: c_int = 2;

pub const S_MAX_REORDER_FRAGMENTS: c_int = 12;
pub const M_MAX_REORDER_FRAGMENTS: c_uint = 0x7;

pub const A_TP_TIMER_RESOLUTION: c_uint = 0x390;
pub const S_DELAYED_ACK_TIMER_RESOLUTION: c_int = 0;
pub const M_DELAYED_ACK_TIMER_RESOLUTION: c_uint = 0x3f;

pub const S_GENERIC_TIMER_RESOLUTION: c_int = 16;
pub const M_GENERIC_TIMER_RESOLUTION: c_uint = 0x3f;

pub const A_TP_2MSL: c_uint = 0x394;
pub const S_2MSL: c_int = 0;
pub const M_2MSL: c_uint = 0x3fffffff;

pub const A_TP_RXT_MIN: c_uint = 0x398;
pub const S_RETRANSMIT_TIMER_MIN: c_int = 0;
pub const M_RETRANSMIT_TIMER_MIN: c_uint = 0xffff;

pub const A_TP_RXT_MAX: c_uint = 0x39c;
pub const S_RETRANSMIT_TIMER_MAX: c_int = 0;
pub const M_RETRANSMIT_TIMER_MAX: c_uint = 0x3fffffff;

pub const A_TP_PERS_MIN: c_uint = 0x3a0;
pub const S_PERSIST_TIMER_MIN: c_int = 0;
pub const M_PERSIST_TIMER_MIN: c_uint = 0xffff;

pub const A_TP_PERS_MAX: c_uint = 0x3a4;
pub const S_PERSIST_TIMER_MAX: c_int = 0;
pub const M_PERSIST_TIMER_MAX: c_uint = 0x3fffffff;

pub const A_TP_KEEP_IDLE: c_uint = 0x3ac;
pub const S_KEEP_ALIVE_IDLE_TIME: c_int = 0;
pub const M_KEEP_ALIVE_IDLE_TIME: c_uint = 0x3fffffff;

pub const A_TP_KEEP_INTVL: c_uint = 0x3b0;
pub const S_KEEP_ALIVE_INTERVAL_TIME: c_int = 0;
pub const M_KEEP_ALIVE_INTERVAL_TIME: c_uint = 0x3fffffff;

pub const A_TP_INIT_SRTT: c_uint = 0x3b4;
pub const S_INITIAL_SRTT: c_int = 0;
pub const M_INITIAL_SRTT: c_uint = 0xffff;

pub const A_TP_DACK_TIME: c_uint = 0x3b8;
pub const S_DELAYED_ACK_TIME: c_int = 0;
pub const M_DELAYED_ACK_TIME: c_uint = 0x7ff;

pub const A_TP_FINWAIT2_TIME: c_uint = 0x3bc;
pub const S_FINWAIT2_TIME: c_int = 0;
pub const M_FINWAIT2_TIME: c_uint = 0x3fffffff;

pub const A_TP_FAST_FINWAIT2_TIME: c_uint = 0x3c0;
pub const S_FAST_FINWAIT2_TIME: c_int = 0;
pub const M_FAST_FINWAIT2_TIME: c_uint = 0x3fffffff;

pub const A_TP_SHIFT_CNT: c_uint = 0x3c4;
pub const S_KEEPALIVE_MAX: c_int = 0;
pub const M_KEEPALIVE_MAX: c_uint = 0xff;

pub const S_WINDOWPROBE_MAX: c_int = 8;
pub const M_WINDOWPROBE_MAX: c_uint = 0xff;

pub const S_RETRANSMISSION_MAX: c_int = 16;
pub const M_RETRANSMISSION_MAX: c_uint = 0xff;

pub const S_SYN_MAX: c_int = 24;
pub const M_SYN_MAX: c_uint = 0xff;

pub const A_TP_QOS_REG0: c_uint = 0x3e0;
pub const S_L3_VALUE: c_int = 0;
pub const M_L3_VALUE: c_uint = 0x3f;

pub const A_TP_QOS_REG1: c_uint = 0x3e4;
pub const A_TP_QOS_REG2: c_uint = 0x3e8;
pub const A_TP_QOS_REG3: c_uint = 0x3ec;
pub const A_TP_QOS_REG4: c_uint = 0x3f0;
pub const A_TP_QOS_REG5: c_uint = 0x3f4;
pub const A_TP_QOS_REG6: c_uint = 0x3f8;
pub const A_TP_QOS_REG7: c_uint = 0x3fc;
pub const A_TP_MTU_REG0: c_uint = 0x404;
pub const A_TP_MTU_REG1: c_uint = 0x408;
pub const A_TP_MTU_REG2: c_uint = 0x40c;
pub const A_TP_MTU_REG3: c_uint = 0x410;
pub const A_TP_MTU_REG4: c_uint = 0x414;
pub const A_TP_MTU_REG5: c_uint = 0x418;
pub const A_TP_MTU_REG6: c_uint = 0x41c;
pub const A_TP_MTU_REG7: c_uint = 0x420;
pub const A_TP_RESET: c_uint = 0x44c;
pub const S_TP_RESET: c_int = 0;

pub const S_CM_MEMMGR_INIT: c_int = 1;

pub const A_TP_MIB_INDEX: c_uint = 0x450;
pub const A_TP_MIB_DATA: c_uint = 0x454;
pub const A_TP_SYNC_TIME_HI: c_uint = 0x458;
pub const A_TP_SYNC_TIME_LO: c_uint = 0x45c;
pub const A_TP_CM_MM_RX_FLST_BASE: c_uint = 0x460;
pub const S_CM_MEMMGR_RX_FREE_LIST_BASE: c_int = 0;
pub const M_CM_MEMMGR_RX_FREE_LIST_BASE: c_uint = 0xfffffff;

pub const A_TP_CM_MM_TX_FLST_BASE: c_uint = 0x464;
pub const S_CM_MEMMGR_TX_FREE_LIST_BASE: c_int = 0;
pub const M_CM_MEMMGR_TX_FREE_LIST_BASE: c_uint = 0xfffffff;

pub const A_TP_CM_MM_P_FLST_BASE: c_uint = 0x468;
pub const S_CM_MEMMGR_PSTRUCT_FREE_LIST_BASE: c_int = 0;
pub const M_CM_MEMMGR_PSTRUCT_FREE_LIST_BASE: c_uint = 0xfffffff;

pub const A_TP_CM_MM_MAX_P: c_uint = 0x46c;
pub const S_CM_MEMMGR_MAX_PSTRUCT: c_int = 0;
pub const M_CM_MEMMGR_MAX_PSTRUCT: c_uint = 0xfffffff;

pub const A_TP_INT_ENABLE: c_uint = 0x470;
pub const S_TX_FREE_LIST_EMPTY: c_int = 0;

pub const S_RX_FREE_LIST_EMPTY: c_int = 1;

pub const A_TP_INT_CAUSE: c_uint = 0x474;
pub const A_TP_TIMER_SEPARATOR: c_uint = 0x4a4;
pub const S_DISABLE_PAST_TIMER_INSERTION: c_int = 0;

pub const S_MODULATION_TIMER_SEPARATOR: c_int = 1;
pub const M_MODULATION_TIMER_SEPARATOR: c_uint = 0x7fff;

pub const S_GLOBAL_TIMER_SEPARATOR: c_int = 16;
pub const M_GLOBAL_TIMER_SEPARATOR: c_uint = 0xffff;

pub const A_TP_CM_FC_MODE: c_uint = 0x4b0;
pub const A_TP_PC_CONGESTION_CNTL: c_uint = 0x4b4;
pub const A_TP_TX_DROP_CONFIG: c_uint = 0x4b8;
pub const S_ENABLE_TX_DROP: c_int = 31;

pub const S_ENABLE_TX_ERROR: c_int = 30;

pub const S_DROP_TICKS_CNT: c_int = 4;
pub const M_DROP_TICKS_CNT: c_uint = 0x3ffffff;

pub const S_NUM_PKTS_DROPPED: c_int = 0;
pub const M_NUM_PKTS_DROPPED: c_uint = 0xf;

pub const A_TP_TX_DROP_COUNT: c_uint = 0x4bc;
// RAT registers
pub const A_RAT_ROUTE_CONTROL: c_uint = 0x580;
pub const S_USE_ROUTE_TABLE: c_int = 0;

pub const S_ENABLE_CSPI: c_int = 1;

pub const S_ENABLE_PCIX: c_int = 2;

pub const A_RAT_ROUTE_TABLE_INDEX: c_uint = 0x584;
pub const S_ROUTE_TABLE_INDEX: c_int = 0;
pub const M_ROUTE_TABLE_INDEX: c_uint = 0xf;

pub const A_RAT_ROUTE_TABLE_DATA: c_uint = 0x588;
pub const A_RAT_NO_ROUTE: c_uint = 0x58c;
pub const S_CPL_OPCODE: c_int = 0;
pub const M_CPL_OPCODE: c_uint = 0xff;

pub const A_RAT_INTR_ENABLE: c_uint = 0x590;
pub const S_ZEROROUTEERROR: c_int = 0;

pub const S_CSPIFRAMINGERROR: c_int = 1;

pub const S_SGEFRAMINGERROR: c_int = 2;

pub const S_TPFRAMINGERROR: c_int = 3;

pub const A_RAT_INTR_CAUSE: c_uint = 0x594;
// CSPI registers
pub const A_CSPI_RX_AE_WM: c_uint = 0x810;
pub const A_CSPI_RX_AF_WM: c_uint = 0x814;
pub const A_CSPI_CALENDAR_LEN: c_uint = 0x818;
pub const S_CALENDARLENGTH: c_int = 0;
pub const M_CALENDARLENGTH: c_uint = 0xffff;

pub const A_CSPI_FIFO_STATUS_ENABLE: c_uint = 0x820;
pub const S_FIFOSTATUSENABLE: c_int = 0;

pub const A_CSPI_MAXBURST1_MAXBURST2: c_uint = 0x828;
pub const S_MAXBURST1: c_int = 0;
pub const M_MAXBURST1: c_uint = 0xffff;

pub const S_MAXBURST2: c_int = 16;
pub const M_MAXBURST2: c_uint = 0xffff;

pub const A_CSPI_TRAIN: c_uint = 0x82c;
pub const S_CSPI_TRAIN_ALPHA: c_int = 0;
pub const M_CSPI_TRAIN_ALPHA: c_uint = 0xffff;

pub const S_CSPI_TRAIN_DATA_MAXT: c_int = 16;
pub const M_CSPI_TRAIN_DATA_MAXT: c_uint = 0xffff;

pub const A_CSPI_INTR_STATUS: c_uint = 0x848;
pub const S_DIP4ERR: c_int = 0;

pub const S_RXDROP: c_int = 1;

pub const S_TXDROP: c_int = 2;

pub const S_RXOVERFLOW: c_int = 3;

pub const S_RAMPARITYERR: c_int = 4;

pub const A_CSPI_INTR_ENABLE: c_uint = 0x84c;
// ESPI registers
pub const A_ESPI_SCH_TOKEN0: c_uint = 0x880;
pub const S_SCHTOKEN0: c_int = 0;
pub const M_SCHTOKEN0: c_uint = 0xffff;

pub const A_ESPI_SCH_TOKEN1: c_uint = 0x884;
pub const S_SCHTOKEN1: c_int = 0;
pub const M_SCHTOKEN1: c_uint = 0xffff;

pub const A_ESPI_SCH_TOKEN2: c_uint = 0x888;
pub const S_SCHTOKEN2: c_int = 0;
pub const M_SCHTOKEN2: c_uint = 0xffff;

pub const A_ESPI_SCH_TOKEN3: c_uint = 0x88c;
pub const S_SCHTOKEN3: c_int = 0;
pub const M_SCHTOKEN3: c_uint = 0xffff;

pub const A_ESPI_RX_FIFO_ALMOST_EMPTY_WATERMARK: c_uint = 0x890;
pub const S_ALMOSTEMPTY: c_int = 0;
pub const M_ALMOSTEMPTY: c_uint = 0xffff;

pub const A_ESPI_RX_FIFO_ALMOST_FULL_WATERMARK: c_uint = 0x894;
pub const S_ALMOSTFULL: c_int = 0;
pub const M_ALMOSTFULL: c_uint = 0xffff;

pub const A_ESPI_CALENDAR_LENGTH: c_uint = 0x898;
pub const A_PORT_CONFIG: c_uint = 0x89c;
pub const S_RX_NPORTS: c_int = 0;
pub const M_RX_NPORTS: c_uint = 0xff;

pub const S_TX_NPORTS: c_int = 8;
pub const M_TX_NPORTS: c_uint = 0xff;

pub const A_ESPI_FIFO_STATUS_ENABLE: c_uint = 0x8a0;
pub const S_RXSTATUSENABLE: c_int = 0;

pub const S_TXDROPENABLE: c_int = 1;

pub const S_RXENDIANMODE: c_int = 2;

pub const S_TXENDIANMODE: c_int = 3;

pub const S_INTEL1010MODE: c_int = 4;

pub const A_ESPI_MAXBURST1_MAXBURST2: c_uint = 0x8a8;
pub const A_ESPI_TRAIN: c_uint = 0x8ac;
pub const S_MAXTRAINALPHA: c_int = 0;
pub const M_MAXTRAINALPHA: c_uint = 0xffff;

pub const S_MAXTRAINDATA: c_int = 16;
pub const M_MAXTRAINDATA: c_uint = 0xffff;

pub const A_RAM_STATUS: c_uint = 0x8b0;
pub const S_RXFIFOPARITYERROR: c_int = 0;
pub const M_RXFIFOPARITYERROR: c_uint = 0x3ff;

pub const S_TXFIFOPARITYERROR: c_int = 10;
pub const M_TXFIFOPARITYERROR: c_uint = 0x3ff;

pub const S_RXFIFOOVERFLOW: c_int = 20;
pub const M_RXFIFOOVERFLOW: c_uint = 0x3ff;

pub const A_TX_DROP_COUNT0: c_uint = 0x8b4;
pub const S_TXPORT0DROPCNT: c_int = 0;
pub const M_TXPORT0DROPCNT: c_uint = 0xffff;

pub const S_TXPORT1DROPCNT: c_int = 16;
pub const M_TXPORT1DROPCNT: c_uint = 0xffff;

pub const A_TX_DROP_COUNT1: c_uint = 0x8b8;
pub const S_TXPORT2DROPCNT: c_int = 0;
pub const M_TXPORT2DROPCNT: c_uint = 0xffff;

pub const S_TXPORT3DROPCNT: c_int = 16;
pub const M_TXPORT3DROPCNT: c_uint = 0xffff;

pub const A_RX_DROP_COUNT0: c_uint = 0x8bc;
pub const S_RXPORT0DROPCNT: c_int = 0;
pub const M_RXPORT0DROPCNT: c_uint = 0xffff;

pub const S_RXPORT1DROPCNT: c_int = 16;
pub const M_RXPORT1DROPCNT: c_uint = 0xffff;

pub const A_RX_DROP_COUNT1: c_uint = 0x8c0;
pub const S_RXPORT2DROPCNT: c_int = 0;
pub const M_RXPORT2DROPCNT: c_uint = 0xffff;

pub const S_RXPORT3DROPCNT: c_int = 16;
pub const M_RXPORT3DROPCNT: c_uint = 0xffff;

pub const A_DIP4_ERROR_COUNT: c_uint = 0x8c4;
pub const S_DIP4ERRORCNT: c_int = 0;
pub const M_DIP4ERRORCNT: c_uint = 0xfff;

pub const S_DIP4ERRORCNTSHADOW: c_int = 12;
pub const M_DIP4ERRORCNTSHADOW: c_uint = 0xfff;

pub const S_TRICN_RX_TRAIN_ERR: c_int = 24;

pub const S_TRICN_RX_TRAINING: c_int = 25;

pub const S_TRICN_RX_TRAIN_OK: c_int = 26;

pub const A_ESPI_INTR_STATUS: c_uint = 0x8c8;
pub const S_DIP2PARITYERR: c_int = 5;

pub const A_ESPI_INTR_ENABLE: c_uint = 0x8cc;
pub const A_RX_DROP_THRESHOLD: c_uint = 0x8d0;
pub const A_ESPI_RX_RESET: c_uint = 0x8ec;
pub const S_ESPI_RX_LNK_RST: c_int = 0;

pub const S_ESPI_RX_CORE_RST: c_int = 1;

pub const S_RX_CLK_STATUS: c_int = 2;

pub const A_ESPI_MISC_CONTROL: c_uint = 0x8f0;
pub const S_OUT_OF_SYNC_COUNT: c_int = 0;
pub const M_OUT_OF_SYNC_COUNT: c_uint = 0xf;

pub const S_DIP2_COUNT_MODE_ENABLE: c_int = 4;

pub const S_DIP2_PARITY_ERR_THRES: c_int = 5;
pub const M_DIP2_PARITY_ERR_THRES: c_uint = 0xf;

pub const S_DIP4_THRES: c_int = 9;
pub const M_DIP4_THRES: c_uint = 0xfff;

pub const S_DIP4_THRES_ENABLE: c_int = 21;

pub const S_FORCE_DISABLE_STATUS: c_int = 22;

pub const S_DYNAMIC_DESKEW: c_int = 23;

pub const S_MONITORED_PORT_NUM: c_int = 25;
pub const M_MONITORED_PORT_NUM: c_uint = 0x3;

pub const S_MONITORED_DIRECTION: c_int = 27;

pub const S_MONITORED_INTERFACE: c_int = 28;

pub const A_ESPI_DIP2_ERR_COUNT: c_uint = 0x8f4;
pub const S_DIP2_ERR_CNT: c_int = 0;
pub const M_DIP2_ERR_CNT: c_uint = 0xf;

pub const A_ESPI_CMD_ADDR: c_uint = 0x8f8;
pub const S_WRITE_DATA: c_int = 0;
pub const M_WRITE_DATA: c_uint = 0xff;

pub const S_REGISTER_OFFSET: c_int = 8;
pub const M_REGISTER_OFFSET: c_uint = 0xf;

pub const S_CHANNEL_ADDR: c_int = 12;
pub const M_CHANNEL_ADDR: c_uint = 0xf;

pub const S_MODULE_ADDR: c_int = 16;
pub const M_MODULE_ADDR: c_uint = 0x3;

pub const S_BUNDLE_ADDR: c_int = 20;
pub const M_BUNDLE_ADDR: c_uint = 0x3;

pub const S_SPI4_COMMAND: c_int = 24;
pub const M_SPI4_COMMAND: c_uint = 0xff;

pub const A_ESPI_GOSTAT: c_uint = 0x8fc;
pub const S_READ_DATA: c_int = 0;
pub const M_READ_DATA: c_uint = 0xff;

pub const S_ESPI_CMD_BUSY: c_int = 8;

pub const S_ERROR_ACK: c_int = 9;

pub const S_UNMAPPED_ERR: c_int = 10;

pub const S_TRANSACTION_TIMER: c_int = 16;
pub const M_TRANSACTION_TIMER: c_uint = 0xff;

// ULP registers
pub const A_ULP_ULIMIT: c_uint = 0x980;
pub const A_ULP_TAGMASK: c_uint = 0x984;
pub const A_ULP_HREG_INDEX: c_uint = 0x988;
pub const A_ULP_HREG_DATA: c_uint = 0x98c;
pub const A_ULP_INT_ENABLE: c_uint = 0x990;
pub const A_ULP_INT_CAUSE: c_uint = 0x994;
pub const S_HREG_PAR_ERR: c_int = 0;

pub const S_EGRS_DATA_PAR_ERR: c_int = 1;

pub const S_INGRS_DATA_PAR_ERR: c_int = 2;

pub const S_PM_INTR: c_int = 3;

pub const S_PM_E2C_SYNC_ERR: c_int = 4;

pub const S_PM_C2E_SYNC_ERR: c_int = 5;

pub const S_PM_E2C_EMPTY_ERR: c_int = 6;

pub const S_PM_C2E_EMPTY_ERR: c_int = 7;

pub const S_PM_PAR_ERR: c_int = 8;
pub const M_PM_PAR_ERR: c_uint = 0xffff;

pub const S_PM_E2C_WRT_FULL: c_int = 24;

pub const S_PM_C2E_WRT_FULL: c_int = 25;

pub const A_ULP_PIO_CTRL: c_uint = 0x998;
// PL registers
pub const A_PL_ENABLE: c_uint = 0xa00;
pub const S_PL_INTR_SGE_ERR: c_int = 0;

pub const S_PL_INTR_SGE_DATA: c_int = 1;

pub const S_PL_INTR_MC3: c_int = 2;

pub const S_PL_INTR_MC4: c_int = 3;

pub const S_PL_INTR_MC5: c_int = 4;

pub const S_PL_INTR_RAT: c_int = 5;

pub const S_PL_INTR_TP: c_int = 6;

pub const S_PL_INTR_ULP: c_int = 7;

pub const S_PL_INTR_ESPI: c_int = 8;

pub const S_PL_INTR_CSPI: c_int = 9;

pub const S_PL_INTR_PCIX: c_int = 10;

pub const S_PL_INTR_EXT: c_int = 11;

pub const A_PL_CAUSE: c_uint = 0xa04;
// MC5 registers
pub const A_MC5_CONFIG: c_uint = 0xc04;
pub const S_MODE: c_int = 0;

pub const S_TCAM_RESET: c_int = 1;

pub const S_TCAM_READY: c_int = 2;

pub const S_DBGI_ENABLE: c_int = 4;

pub const S_M_BUS_ENABLE: c_int = 5;

pub const S_PARITY_ENABLE: c_int = 6;

pub const S_SYN_ISSUE_MODE: c_int = 7;
pub const M_SYN_ISSUE_MODE: c_uint = 0x3;

pub const S_BUILD: c_int = 16;

pub const S_COMPRESSION_ENABLE: c_int = 17;

pub const S_NUM_LIP: c_int = 18;
pub const M_NUM_LIP: c_uint = 0x3f;

pub const S_TCAM_PART_CNT: c_int = 24;
pub const M_TCAM_PART_CNT: c_uint = 0x3;

pub const S_TCAM_PART_TYPE: c_int = 26;
pub const M_TCAM_PART_TYPE: c_uint = 0x3;

pub const S_TCAM_PART_SIZE: c_int = 28;
pub const M_TCAM_PART_SIZE: c_uint = 0x3;

pub const S_TCAM_PART_TYPE_HI: c_int = 30;

pub const A_MC5_SIZE: c_uint = 0xc08;
pub const S_SIZE: c_int = 0;
pub const M_SIZE: c_uint = 0x3fffff;

pub const A_MC5_ROUTING_TABLE_INDEX: c_uint = 0xc0c;
pub const S_START_OF_ROUTING_TABLE: c_int = 0;
pub const M_START_OF_ROUTING_TABLE: c_uint = 0x3fffff;

pub const A_MC5_SERVER_INDEX: c_uint = 0xc14;
pub const S_START_OF_SERVER_INDEX: c_int = 0;
pub const M_START_OF_SERVER_INDEX: c_uint = 0x3fffff;

pub const A_MC5_LIP_RAM_ADDR: c_uint = 0xc18;
pub const S_LOCAL_IP_RAM_ADDR: c_int = 0;
pub const M_LOCAL_IP_RAM_ADDR: c_uint = 0x3f;

pub const S_RAM_WRITE_ENABLE: c_int = 8;

pub const A_MC5_LIP_RAM_DATA: c_uint = 0xc1c;
pub const A_MC5_RSP_LATENCY: c_uint = 0xc20;
pub const S_SEARCH_RESPONSE_LATENCY: c_int = 0;
pub const M_SEARCH_RESPONSE_LATENCY: c_uint = 0x1f;

pub const S_LEARN_RESPONSE_LATENCY: c_int = 8;
pub const M_LEARN_RESPONSE_LATENCY: c_uint = 0x1f;

pub const A_MC5_PARITY_LATENCY: c_uint = 0xc24;
pub const S_SRCHLAT: c_int = 0;
pub const M_SRCHLAT: c_uint = 0x1f;

pub const S_PARLAT: c_int = 8;
pub const M_PARLAT: c_uint = 0x1f;

pub const A_MC5_WR_LRN_VERIFY: c_uint = 0xc28;
pub const S_POVEREN: c_int = 0;

pub const S_LRNVEREN: c_int = 1;

pub const S_VWVEREN: c_int = 2;

pub const A_MC5_PART_ID_INDEX: c_uint = 0xc2c;
pub const S_IDINDEX: c_int = 0;
pub const M_IDINDEX: c_uint = 0xf;

pub const A_MC5_RESET_MAX: c_uint = 0xc30;
pub const S_RSTMAX: c_int = 0;
pub const M_RSTMAX: c_uint = 0x1ff;

pub const A_MC5_INT_ENABLE: c_uint = 0xc40;
pub const S_MC5_INT_HIT_OUT_ACTIVE_REGION_ERR: c_int = 0;

pub const S_MC5_INT_HIT_IN_ACTIVE_REGION_ERR: c_int = 1;

pub const S_MC5_INT_HIT_IN_RT_REGION_ERR: c_int = 2;

pub const S_MC5_INT_MISS_ERR: c_int = 3;

pub const S_MC5_INT_LIP0_ERR: c_int = 4;

pub const S_MC5_INT_LIP_MISS_ERR: c_int = 5;

pub const S_MC5_INT_PARITY_ERR: c_int = 6;

pub const S_MC5_INT_ACTIVE_REGION_FULL: c_int = 7;

pub const S_MC5_INT_NFA_SRCH_ERR: c_int = 8;

pub const S_MC5_INT_SYN_COOKIE: c_int = 9;

pub const S_MC5_INT_SYN_COOKIE_BAD: c_int = 10;

pub const S_MC5_INT_SYN_COOKIE_OFF: c_int = 11;

pub const S_MC5_INT_UNKNOWN_CMD: c_int = 15;

pub const S_MC5_INT_REQUESTQ_PARITY_ERR: c_int = 16;

pub const S_MC5_INT_DISPATCHQ_PARITY_ERR: c_int = 17;

pub const S_MC5_INT_DEL_ACT_EMPTY: c_int = 18;

pub const A_MC5_INT_CAUSE: c_uint = 0xc44;
pub const A_MC5_INT_TID: c_uint = 0xc48;
pub const A_MC5_INT_PTID: c_uint = 0xc4c;
pub const A_MC5_DBGI_CONFIG: c_uint = 0xc74;
pub const A_MC5_DBGI_REQ_CMD: c_uint = 0xc78;
pub const S_CMDMODE: c_int = 0;
pub const M_CMDMODE: c_uint = 0x7;

pub const S_SADRSEL: c_int = 4;

pub const S_WRITE_BURST_SIZE: c_int = 22;
pub const M_WRITE_BURST_SIZE: c_uint = 0x3ff;

pub const A_MC5_DBGI_REQ_ADDR0: c_uint = 0xc7c;
pub const A_MC5_DBGI_REQ_ADDR1: c_uint = 0xc80;
pub const A_MC5_DBGI_REQ_ADDR2: c_uint = 0xc84;
pub const A_MC5_DBGI_REQ_DATA0: c_uint = 0xc88;
pub const A_MC5_DBGI_REQ_DATA1: c_uint = 0xc8c;
pub const A_MC5_DBGI_REQ_DATA2: c_uint = 0xc90;
pub const A_MC5_DBGI_REQ_DATA3: c_uint = 0xc94;
pub const A_MC5_DBGI_REQ_DATA4: c_uint = 0xc98;
pub const A_MC5_DBGI_REQ_MASK0: c_uint = 0xc9c;
pub const A_MC5_DBGI_REQ_MASK1: c_uint = 0xca0;
pub const A_MC5_DBGI_REQ_MASK2: c_uint = 0xca4;
pub const A_MC5_DBGI_REQ_MASK3: c_uint = 0xca8;
pub const A_MC5_DBGI_REQ_MASK4: c_uint = 0xcac;
pub const A_MC5_DBGI_RSP_STATUS: c_uint = 0xcb0;
pub const S_DBGI_RSP_VALID: c_int = 0;

pub const S_DBGI_RSP_HIT: c_int = 1;

pub const S_DBGI_RSP_ERR: c_int = 2;

pub const S_DBGI_RSP_ERR_REASON: c_int = 8;
pub const M_DBGI_RSP_ERR_REASON: c_uint = 0x7;

pub const A_MC5_DBGI_RSP_DATA0: c_uint = 0xcb4;
pub const A_MC5_DBGI_RSP_DATA1: c_uint = 0xcb8;
pub const A_MC5_DBGI_RSP_DATA2: c_uint = 0xcbc;
pub const A_MC5_DBGI_RSP_DATA3: c_uint = 0xcc0;
pub const A_MC5_DBGI_RSP_DATA4: c_uint = 0xcc4;
pub const A_MC5_DBGI_RSP_LAST_CMD: c_uint = 0xcc8;
pub const A_MC5_POPEN_DATA_WR_CMD: c_uint = 0xccc;
pub const A_MC5_POPEN_MASK_WR_CMD: c_uint = 0xcd0;
pub const A_MC5_AOPEN_SRCH_CMD: c_uint = 0xcd4;
pub const A_MC5_AOPEN_LRN_CMD: c_uint = 0xcd8;
pub const A_MC5_SYN_SRCH_CMD: c_uint = 0xcdc;
pub const A_MC5_SYN_LRN_CMD: c_uint = 0xce0;
pub const A_MC5_ACK_SRCH_CMD: c_uint = 0xce4;
pub const A_MC5_ACK_LRN_CMD: c_uint = 0xce8;
pub const A_MC5_ILOOKUP_CMD: c_uint = 0xcec;
pub const A_MC5_ELOOKUP_CMD: c_uint = 0xcf0;
pub const A_MC5_DATA_WRITE_CMD: c_uint = 0xcf4;
pub const A_MC5_DATA_READ_CMD: c_uint = 0xcf8;
pub const A_MC5_MASK_WRITE_CMD: c_uint = 0xcfc;
// PCICFG registers
pub const A_PCICFG_PM_CSR: c_uint = 0x44;
pub const A_PCICFG_VPD_ADDR: c_uint = 0x4a;
pub const S_VPD_ADDR: c_int = 0;
pub const M_VPD_ADDR: c_uint = 0x7fff;

pub const S_VPD_OP_FLAG: c_int = 15;

pub const A_PCICFG_VPD_DATA: c_uint = 0x4c;
pub const A_PCICFG_PCIX_CMD: c_uint = 0x60;
pub const A_PCICFG_INTR_ENABLE: c_uint = 0xf4;
pub const S_MASTER_PARITY_ERR: c_int = 0;

pub const S_SIG_TARGET_ABORT: c_int = 1;

pub const S_RCV_TARGET_ABORT: c_int = 2;

pub const S_RCV_MASTER_ABORT: c_int = 3;

pub const S_SIG_SYS_ERR: c_int = 4;

pub const S_DET_PARITY_ERR: c_int = 5;

pub const S_PIO_PARITY_ERR: c_int = 6;

pub const S_WF_PARITY_ERR: c_int = 7;

pub const S_RF_PARITY_ERR: c_int = 8;
pub const M_RF_PARITY_ERR: c_uint = 0x3;

pub const S_CF_PARITY_ERR: c_int = 10;
pub const M_CF_PARITY_ERR: c_uint = 0x3;

pub const A_PCICFG_INTR_CAUSE: c_uint = 0xf8;
pub const A_PCICFG_MODE: c_uint = 0xfc;
pub const S_PCI_MODE_64BIT: c_int = 0;

pub const S_PCI_MODE_66MHZ: c_int = 1;

pub const S_PCI_MODE_PCIX_INITPAT: c_int = 2;
pub const M_PCI_MODE_PCIX_INITPAT: c_uint = 0x7;

pub const S_PCI_MODE_PCIX: c_int = 5;

pub const S_PCI_MODE_CLK: c_int = 6;
pub const M_PCI_MODE_CLK: c_uint = 0x3;

