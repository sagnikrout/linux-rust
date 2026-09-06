//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/mvpp2/mvpp2.h
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
// Definitions for Marvell PPv2 network controller for Armada 375 SoC.
//
// Copyright (C) 2014 Marvell
//
// Marcin Wojtas <mw@semihalf.com>
//

// The PacketOffset field is measured in units of 32 bytes and is 3 bits wide,
// so the maximum offset is 7 * 32 = 224
//

pub const MVPP2_XDP_PASS: c_int = 0;

// Fifo Registers

pub const MVPP2_RX_MIN_PKT_SIZE_REG: c_uint = 0x60;
pub const MVPP2_RX_FIFO_INIT_REG: c_uint = 0x64;

// RX DMA Top Registers

pub const MVPP2_POOL_BUF_SIZE_OFFSET: c_int = 5;

pub const MVPP2_SNOOP_PKT_SIZE_MASK: c_uint = 0x1ff;

pub const MVPP2_RXQ_POOL_SHORT_OFFS: c_int = 20;
pub const MVPP21_RXQ_POOL_SHORT_MASK: c_uint = 0x700000;
pub const MVPP22_RXQ_POOL_SHORT_MASK: c_uint = 0xf00000;
pub const MVPP2_RXQ_POOL_LONG_OFFS: c_int = 24;
pub const MVPP21_RXQ_POOL_LONG_MASK: c_uint = 0x7000000;
pub const MVPP22_RXQ_POOL_LONG_MASK: c_uint = 0xf000000;
pub const MVPP2_RXQ_PACKET_OFFSET_OFFS: c_int = 28;
pub const MVPP2_RXQ_PACKET_OFFSET_MASK: c_uint = 0x70000000;

// Top Registers

pub const MVPP2_VER_ID_REG: c_uint = 0x50b0;
pub const MVPP2_VER_PP22: c_uint = 0x10;
pub const MVPP2_VER_PP23: c_uint = 0x11;
// Parser Registers
pub const MVPP2_PRS_INIT_LOOKUP_REG: c_uint = 0x1000;
pub const MVPP2_PRS_PORT_LU_MAX: c_uint = 0xf;

pub const MVPP2_PRS_TCAM_IDX_REG: c_uint = 0x1100;

pub const MVPP2_PRS_SRAM_IDX_REG: c_uint = 0x1200;

pub const MVPP2_PRS_TCAM_CTRL_REG: c_uint = 0x1230;

pub const MVPP2_PRS_TCAM_HIT_IDX_REG: c_uint = 0x1240;
pub const MVPP2_PRS_TCAM_HIT_CNT_REG: c_uint = 0x1244;

// RSS Registers
pub const MVPP22_RSS_INDEX: c_uint = 0x1500;

pub const MVPP22_RXQ2RSS_TABLE: c_uint = 0x1504;

pub const MVPP22_RSS_TABLE_ENTRY: c_uint = 0x1508;
pub const MVPP22_RSS_WIDTH: c_uint = 0x150c;
// Classifier Registers
pub const MVPP2_CLS_MODE_REG: c_uint = 0x1800;

pub const MVPP2_CLS_PORT_WAY_REG: c_uint = 0x1810;

pub const MVPP2_CLS_LKP_INDEX_REG: c_uint = 0x1814;
pub const MVPP2_CLS_LKP_INDEX_WAY_OFFS: c_int = 6;
pub const MVPP2_CLS_LKP_TBL_REG: c_uint = 0x1818;
pub const MVPP2_CLS_LKP_TBL_RXQ_MASK: c_uint = 0xff;

pub const MVPP2_CLS_FLOW_INDEX_REG: c_uint = 0x1820;
pub const MVPP2_CLS_FLOW_TBL0_REG: c_uint = 0x1824;

pub const MVPP2_CLS_FLOW_TBL0_ENG_MASK: c_uint = 0x7;
pub const MVPP2_CLS_FLOW_TBL0_OFFS: c_int = 1;

pub const MVPP2_CLS_FLOW_TBL0_PORT_ID_MASK: c_uint = 0xff;

pub const MVPP2_CLS_FLOW_TBL1_REG: c_uint = 0x1828;
pub const MVPP2_CLS_FLOW_TBL1_N_FIELDS_MASK: c_uint = 0x7;

pub const MVPP2_CLS_FLOW_TBL1_PRIO_MASK: c_uint = 0x3f;

pub const MVPP2_CLS_FLOW_TBL1_SEQ_MASK: c_uint = 0x7;

pub const MVPP2_CLS_FLOW_TBL2_REG: c_uint = 0x182c;
pub const MVPP2_CLS_FLOW_TBL2_FLD_MASK: c_uint = 0x3f;

pub const MVPP2_CLS_OVERSIZE_RXQ_LOW_BITS: c_int = 3;
pub const MVPP2_CLS_OVERSIZE_RXQ_LOW_MASK: c_uint = 0x7;

pub const MVPP2_CLS_SWFWD_PCTRL_REG: c_uint = 0x19d0;

// Classifier C2 engine Registers
pub const MVPP22_CLS_C2_TCAM_IDX: c_uint = 0x1b00;
pub const MVPP22_CLS_C2_TCAM_DATA0: c_uint = 0x1b10;
pub const MVPP22_CLS_C2_TCAM_DATA1: c_uint = 0x1b14;
pub const MVPP22_CLS_C2_TCAM_DATA2: c_uint = 0x1b18;
pub const MVPP22_CLS_C2_TCAM_DATA3: c_uint = 0x1b1c;
pub const MVPP22_CLS_C2_TCAM_DATA4: c_uint = 0x1b20;

pub const MVPP22_CLS_C2_TCAM_INV: c_uint = 0x1b24;

pub const MVPP22_CLS_C2_HIT_CTR: c_uint = 0x1b50;
pub const MVPP22_CLS_C2_ACT: c_uint = 0x1b60;

pub const MVPP22_CLS_C2_ATTR0: c_uint = 0x1b64;

pub const MVPP22_CLS_C2_ATTR0_QHIGH_MASK: c_uint = 0x1f;
pub const MVPP22_CLS_C2_ATTR0_QHIGH_OFFS: c_int = 24;

pub const MVPP22_CLS_C2_ATTR0_QLOW_MASK: c_uint = 0x7;
pub const MVPP22_CLS_C2_ATTR0_QLOW_OFFS: c_int = 21;
pub const MVPP22_CLS_C2_ATTR1: c_uint = 0x1b68;
pub const MVPP22_CLS_C2_ATTR2: c_uint = 0x1b6c;

pub const MVPP22_CLS_C2_ATTR3: c_uint = 0x1b70;
pub const MVPP22_CLS_C2_TCAM_CTRL: c_uint = 0x1b90;

// Descriptor Manager Top Registers
pub const MVPP2_RXQ_NUM_REG: c_uint = 0x2040;
pub const MVPP2_RXQ_DESC_ADDR_REG: c_uint = 0x2044;
pub const MVPP22_DESC_ADDR_OFFS: c_int = 8;
pub const MVPP2_RXQ_DESC_SIZE_REG: c_uint = 0x2048;
pub const MVPP2_RXQ_DESC_SIZE_MASK: c_uint = 0x3ff0;

pub const MVPP2_RXQ_NUM_PROCESSED_OFFSET: c_int = 0;
pub const MVPP2_RXQ_NUM_NEW_OFFSET: c_int = 16;

pub const MVPP2_RXQ_OCCUPIED_MASK: c_uint = 0x3fff;
pub const MVPP2_RXQ_NON_OCCUPIED_OFFSET: c_int = 16;
pub const MVPP2_RXQ_NON_OCCUPIED_MASK: c_uint = 0x3fff0000;
pub const MVPP2_RXQ_THRESH_REG: c_uint = 0x204c;
pub const MVPP2_OCCUPIED_THRESH_OFFSET: c_int = 0;
pub const MVPP2_OCCUPIED_THRESH_MASK: c_uint = 0x3fff;
pub const MVPP2_RXQ_INDEX_REG: c_uint = 0x2050;
pub const MVPP2_TXQ_NUM_REG: c_uint = 0x2080;
pub const MVPP2_TXQ_DESC_ADDR_REG: c_uint = 0x2084;
pub const MVPP2_TXQ_DESC_SIZE_REG: c_uint = 0x2088;
pub const MVPP2_TXQ_DESC_SIZE_MASK: c_uint = 0x3ff0;
pub const MVPP2_TXQ_THRESH_REG: c_uint = 0x2094;
pub const MVPP2_TXQ_THRESH_OFFSET: c_int = 16;
pub const MVPP2_TXQ_THRESH_MASK: c_uint = 0x3fff;
pub const MVPP2_AGGR_TXQ_UPDATE_REG: c_uint = 0x2090;
pub const MVPP2_TXQ_INDEX_REG: c_uint = 0x2098;
pub const MVPP2_TXQ_PREF_BUF_REG: c_uint = 0x209c;

pub const MVPP2_TXQ_PENDING_REG: c_uint = 0x20a0;
pub const MVPP2_TXQ_PENDING_MASK: c_uint = 0x3fff;
pub const MVPP2_TXQ_INT_STATUS_REG: c_uint = 0x20a4;

pub const MVPP2_TRANSMITTED_COUNT_OFFSET: c_int = 16;
pub const MVPP2_TRANSMITTED_COUNT_MASK: c_uint = 0x3fff0000;
pub const MVPP2_TXQ_RSVD_REQ_REG: c_uint = 0x20b0;
pub const MVPP2_TXQ_RSVD_REQ_Q_OFFSET: c_int = 16;
pub const MVPP2_TXQ_RSVD_RSLT_REG: c_uint = 0x20b4;
pub const MVPP2_TXQ_RSVD_RSLT_MASK: c_uint = 0x3fff;
pub const MVPP2_TXQ_RSVD_CLR_REG: c_uint = 0x20b8;
pub const MVPP2_TXQ_RSVD_CLR_OFFSET: c_int = 16;

pub const MVPP22_AGGR_TXQ_DESC_ADDR_OFFS: c_int = 8;

pub const MVPP2_AGGR_TXQ_DESC_SIZE_MASK: c_uint = 0x3ff0;

pub const MVPP2_AGGR_TXQ_PENDING_MASK: c_uint = 0x3fff;

// MBUS bridge registers

pub const MVPP2_BASE_ADDR_ENABLE: c_uint = 0x4060;
// AXI Bridge Registers
pub const MVPP22_AXI_BM_WR_ATTR_REG: c_uint = 0x4100;
pub const MVPP22_AXI_BM_RD_ATTR_REG: c_uint = 0x4104;
pub const MVPP22_AXI_AGGRQ_DESCR_RD_ATTR_REG: c_uint = 0x4110;
pub const MVPP22_AXI_TXQ_DESCR_WR_ATTR_REG: c_uint = 0x4114;
pub const MVPP22_AXI_TXQ_DESCR_RD_ATTR_REG: c_uint = 0x4118;
pub const MVPP22_AXI_RXQ_DESCR_WR_ATTR_REG: c_uint = 0x411c;
pub const MVPP22_AXI_RX_DATA_WR_ATTR_REG: c_uint = 0x4120;
pub const MVPP22_AXI_TX_DATA_RD_ATTR_REG: c_uint = 0x4130;
pub const MVPP22_AXI_RD_NORMAL_CODE_REG: c_uint = 0x4150;
pub const MVPP22_AXI_RD_SNOOP_CODE_REG: c_uint = 0x4154;
pub const MVPP22_AXI_WR_NORMAL_CODE_REG: c_uint = 0x4160;
pub const MVPP22_AXI_WR_SNOOP_CODE_REG: c_uint = 0x4164;
// Values for AXI Bridge registers
pub const MVPP22_AXI_ATTR_CACHE_OFFS: c_int = 0;
pub const MVPP22_AXI_ATTR_DOMAIN_OFFS: c_int = 12;
pub const MVPP22_AXI_CODE_CACHE_OFFS: c_int = 0;
pub const MVPP22_AXI_CODE_DOMAIN_OFFS: c_int = 4;
pub const MVPP22_AXI_CODE_CACHE_NON_CACHE: c_uint = 0x3;
pub const MVPP22_AXI_CODE_CACHE_WR_CACHE: c_uint = 0x7;
pub const MVPP22_AXI_CODE_CACHE_RD_CACHE: c_uint = 0xb;
pub const MVPP22_AXI_CODE_DOMAIN_OUTER_DOM: c_int = 2;
pub const MVPP22_AXI_CODE_DOMAIN_SYSTEM: c_int = 3;
// Interrupt Cause and Mask registers

pub const MVPP2_MAX_ISR_TX_THRESHOLD: c_uint = 0xfffff0;

pub const MVPP2_MAX_ISR_RX_THRESHOLD: c_uint = 0xfffff0;

pub const MVPP22_ISR_RXQ_GROUP_INDEX_REG: c_uint = 0x5400;
pub const MVPP22_ISR_RXQ_GROUP_INDEX_SUBGROUP_MASK: c_uint = 0xf;
pub const MVPP22_ISR_RXQ_GROUP_INDEX_GROUP_MASK: c_uint = 0x380;
pub const MVPP22_ISR_RXQ_GROUP_INDEX_GROUP_OFFSET: c_int = 7;
pub const MVPP22_ISR_RXQ_GROUP_INDEX_SUBGROUP_MASK: c_uint = 0xf;
pub const MVPP22_ISR_RXQ_GROUP_INDEX_GROUP_MASK: c_uint = 0x380;
pub const MVPP22_ISR_RXQ_SUB_GROUP_CONFIG_REG: c_uint = 0x5404;
pub const MVPP22_ISR_RXQ_SUB_GROUP_STARTQ_MASK: c_uint = 0x1f;
pub const MVPP22_ISR_RXQ_SUB_GROUP_SIZE_MASK: c_uint = 0xf00;
pub const MVPP22_ISR_RXQ_SUB_GROUP_SIZE_OFFSET: c_int = 8;

pub const MVPP2_CAUSE_TXQ_OCCUP_DESC_ALL_MASK: c_uint = 0xff0000;
pub const MVPP2_CAUSE_TXQ_OCCUP_DESC_ALL_OFFSET: c_int = 16;

pub const MVPP2_ISR_PON_RX_TX_MASK_REG: c_uint = 0x54bc;
pub const MVPP2_PON_CAUSE_RXQ_OCCUP_DESC_ALL_MASK: c_uint = 0xffff;
pub const MVPP2_PON_CAUSE_TXP_OCCUP_DESC_ALL_MASK: c_uint = 0x3fc00000;

pub const MVPP2_ISR_MISC_CAUSE_REG: c_uint = 0x55b0;

pub const MVPP2_ISR_RX_ERR_CAUSE_NONOCC_MASK: c_uint = 0x00ff;
// Buffer Manager registers

pub const MVPP2_BM_POOL_BASE_ADDR_MASK: c_uint = 0xfffff80;

pub const MVPP2_BM_POOL_SIZE_MASK: c_uint = 0xfff0;

pub const MVPP2_BM_POOL_GET_READ_PTR_MASK: c_uint = 0xfff0;

pub const MVPP2_BM_POOL_PTRS_NUM_MASK: c_uint = 0xfff0;

pub const MVPP2_BM_BPPI_PTR_NUM_MASK: c_uint = 0x7ff;
pub const MVPP22_BM_POOL_PTRS_NUM_MASK: c_uint = 0xfff8;

pub const MVPP2_BM_LOW_THRESH_OFFS: c_int = 8;
pub const MVPP2_BM_LOW_THRESH_MASK: c_uint = 0x7f00;

pub const MVPP2_BM_HIGH_THRESH_OFFS: c_int = 16;
pub const MVPP2_BM_HIGH_THRESH_MASK: c_uint = 0x7f0000;

pub const MVPP2_BM_BPPI_HIGH_THRESH: c_uint = 0x1E;
pub const MVPP2_BM_BPPI_LOW_THRESH: c_uint = 0x1C;
pub const MVPP23_BM_BPPI_HIGH_THRESH: c_uint = 0x34;
pub const MVPP23_BM_BPPI_LOW_THRESH: c_uint = 0x28;

pub const MVPP2_BM_VIRT_ALLOC_REG: c_uint = 0x6440;
pub const MVPP22_BM_ADDR_HIGH_ALLOC: c_uint = 0x6444;
pub const MVPP22_BM_ADDR_HIGH_PHYS_MASK: c_uint = 0xff;
pub const MVPP22_BM_ADDR_HIGH_VIRT_MASK: c_uint = 0xff00;
pub const MVPP22_BM_ADDR_HIGH_VIRT_SHIFT: c_int = 8;

pub const MVPP2_BM_VIRT_RLS_REG: c_uint = 0x64c0;
pub const MVPP22_BM_ADDR_HIGH_RLS_REG: c_uint = 0x64c4;
pub const MVPP22_BM_ADDR_HIGH_PHYS_RLS_MASK: c_uint = 0xff;
pub const MVPP22_BM_ADDR_HIGH_VIRT_RLS_MASK: c_uint = 0xff00;
pub const MVPP22_BM_ADDR_HIGH_VIRT_RLS_SHIFT: c_int = 8;
// Packet Processor per-port counters
pub const MVPP2_OVERRUN_ETH_DROP: c_uint = 0x7000;
pub const MVPP2_CLS_ETH_DROP: c_uint = 0x7020;
pub const MVPP22_BM_POOL_BASE_ADDR_HIGH_REG: c_uint = 0x6310;
pub const MVPP22_BM_POOL_BASE_ADDR_HIGH_MASK: c_uint = 0xff;

// Hit counters registers
pub const MVPP2_CTRS_IDX: c_uint = 0x7040;

pub const MVPP2_TX_DESC_ENQ_CTR: c_uint = 0x7100;
pub const MVPP2_TX_DESC_ENQ_TO_DDR_CTR: c_uint = 0x7104;
pub const MVPP2_TX_BUFF_ENQ_TO_DDR_CTR: c_uint = 0x7108;
pub const MVPP2_TX_DESC_ENQ_HW_FWD_CTR: c_uint = 0x710c;
pub const MVPP2_RX_DESC_ENQ_CTR: c_uint = 0x7120;
pub const MVPP2_TX_PKTS_DEQ_CTR: c_uint = 0x7130;
pub const MVPP2_TX_PKTS_FULL_QUEUE_DROP_CTR: c_uint = 0x7200;
pub const MVPP2_TX_PKTS_EARLY_DROP_CTR: c_uint = 0x7204;
pub const MVPP2_TX_PKTS_BM_DROP_CTR: c_uint = 0x7208;
pub const MVPP2_TX_PKTS_BM_MC_DROP_CTR: c_uint = 0x720c;
pub const MVPP2_RX_PKTS_FULL_QUEUE_DROP_CTR: c_uint = 0x7220;
pub const MVPP2_RX_PKTS_EARLY_DROP_CTR: c_uint = 0x7224;
pub const MVPP2_RX_PKTS_BM_DROP_CTR: c_uint = 0x7228;
pub const MVPP2_CLS_DEC_TBL_HIT_CTR: c_uint = 0x7700;
pub const MVPP2_CLS_FLOW_TBL_HIT_CTR: c_uint = 0x7704;
// TX Scheduler registers
pub const MVPP2_TXP_SCHED_PORT_INDEX_REG: c_uint = 0x8000;
pub const MVPP2_TXP_SCHED_Q_CMD_REG: c_uint = 0x8004;
pub const MVPP2_TXP_SCHED_ENQ_MASK: c_uint = 0xff;
pub const MVPP2_TXP_SCHED_DISQ_OFFSET: c_int = 8;
pub const MVPP2_TXP_SCHED_CMD_1_REG: c_uint = 0x8010;
pub const MVPP2_TXP_SCHED_FIXED_PRIO_REG: c_uint = 0x8014;
pub const MVPP2_TXP_SCHED_PERIOD_REG: c_uint = 0x8018;
pub const MVPP2_TXP_SCHED_MTU_REG: c_uint = 0x801c;
pub const MVPP2_TXP_MTU_MAX: c_uint = 0x7FFFF;
pub const MVPP2_TXP_SCHED_REFILL_REG: c_uint = 0x8020;
pub const MVPP2_TXP_REFILL_TOKENS_ALL_MASK: c_uint = 0x7ffff;
pub const MVPP2_TXP_REFILL_PERIOD_ALL_MASK: c_uint = 0x3ff00000;

pub const MVPP2_TXP_SCHED_TOKEN_SIZE_REG: c_uint = 0x8024;
pub const MVPP2_TXP_TOKEN_SIZE_MAX: c_uint = 0xffffffff;

pub const MVPP2_TXQ_REFILL_TOKENS_ALL_MASK: c_uint = 0x7ffff;
pub const MVPP2_TXQ_REFILL_PERIOD_ALL_MASK: c_uint = 0x3ff00000;

pub const MVPP2_TXQ_TOKEN_SIZE_MAX: c_uint = 0x7fffffff;

pub const MVPP2_TXQ_TOKEN_CNTR_MAX: c_uint = 0xffffffff;
// TX general registers
pub const MVPP2_TX_SNOOP_REG: c_uint = 0x8800;
pub const MVPP2_TX_PORT_FLUSH_REG: c_uint = 0x8810;

// LMS registers
pub const MVPP2_SRC_ADDR_MIDDLE: c_uint = 0x24;
pub const MVPP2_SRC_ADDR_HIGH: c_uint = 0x28;
pub const MVPP2_PHY_AN_CFG0_REG: c_uint = 0x34;

pub const MVPP2_MNG_EXTENDED_GLOBAL_CTRL_REG: c_uint = 0x305c;
pub const MVPP2_EXT_GLOBAL_CTRL_DEFAULT: c_uint = 0x27;
// Per-port registers
pub const MVPP2_GMAC_CTRL_0_REG: c_uint = 0x0;

pub const MVPP2_GMAC_MAX_RX_SIZE_OFFS: c_int = 2;
pub const MVPP2_GMAC_MAX_RX_SIZE_MASK: c_uint = 0x7ffc;

pub const MVPP2_GMAC_CTRL_1_REG: c_uint = 0x4;

pub const MVPP2_GMAC_PCS_LB_EN_BIT: c_int = 6;

pub const MVPP2_GMAC_SA_LOW_OFFS: c_int = 7;
pub const MVPP2_GMAC_CTRL_2_REG: c_uint = 0x8;

pub const MVPP2_GMAC_AUTONEG_CONFIG: c_uint = 0xc;

pub const MVPP2_GMAC_STATUS0: c_uint = 0x10;

pub const MVPP2_GMAC_PORT_FIFO_CFG_1_REG: c_uint = 0x1c;
pub const MVPP2_GMAC_TX_FIFO_MIN_TH_OFFS: c_int = 6;
pub const MVPP2_GMAC_TX_FIFO_MIN_TH_ALL_MASK: c_uint = 0x1fc0;

pub const MVPP22_GMAC_INT_STAT: c_uint = 0x20;

pub const MVPP22_GMAC_INT_MASK: c_uint = 0x24;

pub const MVPP22_GMAC_CTRL_4_REG: c_uint = 0x90;

pub const MVPP22_GMAC_INT_SUM_STAT: c_uint = 0xa0;

pub const MVPP22_GMAC_INT_SUM_MASK: c_uint = 0xa4;

pub const MVPP2_GMAC_LPI_CTRL0: c_uint = 0xc0;

pub const MVPP2_GMAC_LPI_CTRL1: c_uint = 0xc4;

// Per-port XGMAC registers. PPv2.2 and PPv2.3, only for GOP port 0,
// relative to port->base.
//
pub const MVPP22_XLG_CTRL0_REG: c_uint = 0x100;

pub const MVPP22_XLG_CTRL1_REG: c_uint = 0x104;
pub const MVPP22_XLG_CTRL1_FRAMESIZELIMIT_OFFS: c_int = 0;
pub const MVPP22_XLG_CTRL1_FRAMESIZELIMIT_MASK: c_uint = 0x1fff;
pub const MVPP22_XLG_STATUS: c_uint = 0x10c;

pub const MVPP22_XLG_INT_STAT: c_uint = 0x114;

pub const MVPP22_XLG_INT_MASK: c_uint = 0x118;

pub const MVPP22_XLG_CTRL3_REG: c_uint = 0x11c;

pub const MVPP22_XLG_EXT_INT_STAT: c_uint = 0x158;

pub const MVPP22_XLG_EXT_INT_MASK: c_uint = 0x15c;

pub const MVPP22_XLG_CTRL4_REG: c_uint = 0x184;

// SMI registers. PPv2.2 and PPv2.3, relative to priv->iface_base.
pub const MVPP22_SMI_MISC_CFG_REG: c_uint = 0x1204;

// TAI registers, PPv2.2 only, relative to priv->iface_base
pub const MVPP22_TAI_INT_CAUSE: c_uint = 0x1400;
pub const MVPP22_TAI_INT_MASK: c_uint = 0x1404;
pub const MVPP22_TAI_CR0: c_uint = 0x1408;
pub const MVPP22_TAI_CR1: c_uint = 0x140c;
pub const MVPP22_TAI_TCFCR0: c_uint = 0x1410;
pub const MVPP22_TAI_TCFCR1: c_uint = 0x1414;
pub const MVPP22_TAI_TCFCR2: c_uint = 0x1418;
pub const MVPP22_TAI_FATWR: c_uint = 0x141c;
pub const MVPP22_TAI_TOD_STEP_NANO_CR: c_uint = 0x1420;
pub const MVPP22_TAI_TOD_STEP_FRAC_HIGH: c_uint = 0x1424;
pub const MVPP22_TAI_TOD_STEP_FRAC_LOW: c_uint = 0x1428;
pub const MVPP22_TAI_TAPDC_HIGH: c_uint = 0x142c;
pub const MVPP22_TAI_TAPDC_LOW: c_uint = 0x1430;
pub const MVPP22_TAI_TGTOD_SEC_HIGH: c_uint = 0x1434;
pub const MVPP22_TAI_TGTOD_SEC_MED: c_uint = 0x1438;
pub const MVPP22_TAI_TGTOD_SEC_LOW: c_uint = 0x143c;
pub const MVPP22_TAI_TGTOD_NANO_HIGH: c_uint = 0x1440;
pub const MVPP22_TAI_TGTOD_NANO_LOW: c_uint = 0x1444;
pub const MVPP22_TAI_TGTOD_FRAC_HIGH: c_uint = 0x1448;
pub const MVPP22_TAI_TGTOD_FRAC_LOW: c_uint = 0x144c;
pub const MVPP22_TAI_TLV_SEC_HIGH: c_uint = 0x1450;
pub const MVPP22_TAI_TLV_SEC_MED: c_uint = 0x1454;
pub const MVPP22_TAI_TLV_SEC_LOW: c_uint = 0x1458;
pub const MVPP22_TAI_TLV_NANO_HIGH: c_uint = 0x145c;
pub const MVPP22_TAI_TLV_NANO_LOW: c_uint = 0x1460;
pub const MVPP22_TAI_TLV_FRAC_HIGH: c_uint = 0x1464;
pub const MVPP22_TAI_TLV_FRAC_LOW: c_uint = 0x1468;
pub const MVPP22_TAI_TCV0_SEC_HIGH: c_uint = 0x146c;
pub const MVPP22_TAI_TCV0_SEC_MED: c_uint = 0x1470;
pub const MVPP22_TAI_TCV0_SEC_LOW: c_uint = 0x1474;
pub const MVPP22_TAI_TCV0_NANO_HIGH: c_uint = 0x1478;
pub const MVPP22_TAI_TCV0_NANO_LOW: c_uint = 0x147c;
pub const MVPP22_TAI_TCV0_FRAC_HIGH: c_uint = 0x1480;
pub const MVPP22_TAI_TCV0_FRAC_LOW: c_uint = 0x1484;
pub const MVPP22_TAI_TCV1_SEC_HIGH: c_uint = 0x1488;
pub const MVPP22_TAI_TCV1_SEC_MED: c_uint = 0x148c;
pub const MVPP22_TAI_TCV1_SEC_LOW: c_uint = 0x1490;
pub const MVPP22_TAI_TCV1_NANO_HIGH: c_uint = 0x1494;
pub const MVPP22_TAI_TCV1_NANO_LOW: c_uint = 0x1498;
pub const MVPP22_TAI_TCV1_FRAC_HIGH: c_uint = 0x149c;
pub const MVPP22_TAI_TCV1_FRAC_LOW: c_uint = 0x14a0;
pub const MVPP22_TAI_TCSR: c_uint = 0x14a4;
pub const MVPP22_TAI_TUC_LSB: c_uint = 0x14a8;
pub const MVPP22_TAI_GFM_SEC_HIGH: c_uint = 0x14ac;
pub const MVPP22_TAI_GFM_SEC_MED: c_uint = 0x14b0;
pub const MVPP22_TAI_GFM_SEC_LOW: c_uint = 0x14b4;
pub const MVPP22_TAI_GFM_NANO_HIGH: c_uint = 0x14b8;
pub const MVPP22_TAI_GFM_NANO_LOW: c_uint = 0x14bc;
pub const MVPP22_TAI_GFM_FRAC_HIGH: c_uint = 0x14c0;
pub const MVPP22_TAI_GFM_FRAC_LOW: c_uint = 0x14c4;
pub const MVPP22_TAI_PCLK_DA_HIGH: c_uint = 0x14c8;
pub const MVPP22_TAI_PCLK_DA_LOW: c_uint = 0x14cc;
pub const MVPP22_TAI_CTCR: c_uint = 0x14d0;
pub const MVPP22_TAI_PCLK_CCC_HIGH: c_uint = 0x14d4;
pub const MVPP22_TAI_PCLK_CCC_LOW: c_uint = 0x14d8;
pub const MVPP22_TAI_DTC_HIGH: c_uint = 0x14dc;
pub const MVPP22_TAI_DTC_LOW: c_uint = 0x14e0;
pub const MVPP22_TAI_CCC_HIGH: c_uint = 0x14e4;
pub const MVPP22_TAI_CCC_LOW: c_uint = 0x14e8;
pub const MVPP22_TAI_ICICE: c_uint = 0x14f4;
pub const MVPP22_TAI_ICICC_LOW: c_uint = 0x14f8;
pub const MVPP22_TAI_TUC_MSB: c_uint = 0x14fc;

pub const MVPP2_CAUSE_TXQ_SENT_DESC_ALL_MASK: c_uint = 0xff;
// Descriptor ring Macros

// XPCS registers.PPv2.2 and PPv2.3

pub const MVPP22_MPCS_CTRL: c_uint = 0x14;

pub const MVPP22_MPCS_CLK_RESET: c_uint = 0x14c;

// FCA registers. PPv2.2 and PPv2.3

pub const MVPP22_FCA_REG_SIZE: c_int = 16;
pub const MVPP22_FCA_REG_MASK: c_uint = 0xFFFF;
pub const MVPP22_FCA_CONTROL_REG: c_uint = 0x0;

// XPCS registers. PPv2.2 and PPv2.3

pub const MVPP22_XPCS_CFG0: c_uint = 0x0;

// PTP registers. PPv2.2 only

pub const MVPP22_PTP_INT_CAUSE: c_uint = 0x00;

pub const MVPP22_PTP_INT_MASK: c_uint = 0x04;

pub const MVPP22_PTP_GCR: c_uint = 0x08;

pub const MVPP22_PTP_TX_Q0_R0: c_uint = 0x0c;
pub const MVPP22_PTP_TX_Q0_R1: c_uint = 0x10;
pub const MVPP22_PTP_TX_Q0_R2: c_uint = 0x14;
pub const MVPP22_PTP_TX_Q1_R0: c_uint = 0x18;
pub const MVPP22_PTP_TX_Q1_R1: c_uint = 0x1c;
pub const MVPP22_PTP_TX_Q1_R2: c_uint = 0x20;
pub const MVPP22_PTP_TPCR: c_uint = 0x24;
pub const MVPP22_PTP_V1PCR: c_uint = 0x28;
pub const MVPP22_PTP_V2PCR: c_uint = 0x2c;
pub const MVPP22_PTP_Y1731PCR: c_uint = 0x30;
pub const MVPP22_PTP_NTPTSPCR: c_uint = 0x34;
pub const MVPP22_PTP_NTPRXPCR: c_uint = 0x38;
pub const MVPP22_PTP_NTPTXPCR: c_uint = 0x3c;
pub const MVPP22_PTP_WAMPPCR: c_uint = 0x40;
pub const MVPP22_PTP_NAPCR: c_uint = 0x44;
pub const MVPP22_PTP_FAPCR: c_uint = 0x48;
pub const MVPP22_PTP_CAPCR: c_uint = 0x50;
pub const MVPP22_PTP_ATAPCR: c_uint = 0x54;
pub const MVPP22_PTP_ACTAPCR: c_uint = 0x58;
pub const MVPP22_PTP_CATAPCR: c_uint = 0x5c;
pub const MVPP22_PTP_CACTAPCR: c_uint = 0x60;
pub const MVPP22_PTP_AITAPCR: c_uint = 0x64;
pub const MVPP22_PTP_CAITAPCR: c_uint = 0x68;
pub const MVPP22_PTP_CITAPCR: c_uint = 0x6c;
pub const MVPP22_PTP_NTP_OFF_HIGH: c_uint = 0x70;
pub const MVPP22_PTP_NTP_OFF_LOW: c_uint = 0x74;
pub const MVPP22_PTP_TX_PIPE_STATUS_DELAY: c_uint = 0x78;
// System controller registers. Accessed through a regmap.
pub const GENCONF_SOFT_RESET1: c_uint = 0x1108;

pub const GENCONF_PORT_CTRL0: c_uint = 0x1110;

pub const GENCONF_PORT_CTRL1: c_uint = 0x1114;

pub const GENCONF_CTRL0: c_uint = 0x1120;

// Various constants
// Coalescing
pub const MVPP2_TXDONE_COAL_PKTS_THRESH: c_int = 64;

pub const MVPP2_TXDONE_COAL_USEC: c_int = 1000;
pub const MVPP2_RX_COAL_PKTS: c_int = 32;
pub const MVPP2_RX_COAL_USEC: c_int = 64;
// The two bytes Marvell header. Either contains a special value used
// by Marvell switches when a specific hardware mode is enabled (not
// supported by this driver) or is filled automatically by zeroes on
// the RX side. Those two bytes being at the front of the Ethernet
// header, they allow to have the IP header aligned on a 4 bytes
// boundary automatically: the hardware skips those two bytes on its
// own.
//
pub const MVPP2_MH_SIZE: c_int = 2;
pub const MVPP2_ETH_TYPE_LEN: c_int = 2;
pub const MVPP2_PPPOE_HDR_SIZE: c_int = 8;
pub const MVPP2_VLAN_TAG_LEN: c_int = 4;
pub const MVPP2_VLAN_TAG_EDSA_LEN: c_int = 8;
// Lbtd 802.3 type
pub const MVPP2_IP_LBDT_TYPE: c_uint = 0xfffa;
pub const MVPP2_TX_CSUM_MAX_SIZE: c_int = 9800;
// Timeout constants
pub const MVPP2_TX_DISABLE_TIMEOUT_MSEC: c_int = 1000;
pub const MVPP2_TX_PENDING_TIMEOUT_MSEC: c_int = 1000;
pub const MVPP2_TX_MTU_MAX: c_uint = 0x7ffff;
// Maximum number of T-CONTs of PON port
pub const MVPP2_MAX_TCONT: c_int = 16;
// Maximum number of supported ports
pub const MVPP2_MAX_PORTS: c_int = 4;
// Loopback port index
pub const MVPP2_LOOPBACK_PORT_INDEX: c_int = 3;
// Maximum number of TXQs used by single port
pub const MVPP2_MAX_TXQ: c_int = 8;
// MVPP2_MAX_TSO_SEGS is the maximum number of fragments to allow in the GSO
// skb. As we need a maxium of two descriptors per fragments (1 header, 1 data),
// multiply this value by two to count the maximum number of skb descs needed.
//
pub const MVPP2_MAX_TSO_SEGS: c_int = 300;

// Max number of RXQs per port
pub const MVPP2_PORT_MAX_RXQ: c_int = 32;
// Max number of Rx descriptors
pub const MVPP2_MAX_RXD_MAX: c_int = 2048;
pub const MVPP2_MAX_RXD_DFLT: c_int = 1024;
// Max number of Tx descriptors
pub const MVPP2_MAX_TXD_MAX: c_int = 2048;
pub const MVPP2_MAX_TXD_DFLT: c_int = 1024;
// Amount of Tx descriptors that can be reserved at once by CPU
pub const MVPP2_CPU_DESC_CHUNK: c_int = 64;
// Max number of Tx descriptors in each aggregated queue
pub const MVPP2_AGGR_TXQ_SIZE: c_int = 256;
// Descriptor aligned size
pub const MVPP2_DESC_ALIGNED_SIZE: c_int = 32;
// Descriptor alignment mask

// RX FIFO constants
pub const MVPP2_RX_FIFO_PORT_DATA_SIZE_44KB: c_uint = 0xb000;
pub const MVPP2_RX_FIFO_PORT_DATA_SIZE_32KB: c_uint = 0x8000;
pub const MVPP2_RX_FIFO_PORT_DATA_SIZE_8KB: c_uint = 0x2000;
pub const MVPP2_RX_FIFO_PORT_DATA_SIZE_4KB: c_uint = 0x1000;

pub const MVPP2_RX_FIFO_PORT_ATTR_SIZE_4KB: c_uint = 0x40;
pub const MVPP2_RX_FIFO_PORT_MIN_PKT: c_uint = 0x80;
// TX FIFO constants
pub const MVPP22_TX_FIFO_DATA_SIZE_18KB: c_int = 18;
pub const MVPP22_TX_FIFO_DATA_SIZE_10KB: c_int = 10;
pub const MVPP22_TX_FIFO_DATA_SIZE_1KB: c_int = 1;

// RX FIFO threshold in 1KB granularity

// RX Flow Control Registers

pub const MVPP2_RX_FC_TRSH_OFFS: c_int = 16;

pub const MVPP2_RX_FC_TRSH_UNIT: c_int = 256;
// MSS Flow control
pub const MSS_FC_COM_REG: c_int = 0;

pub const FC_QUANTA: c_uint = 0xFFFF;
pub const FC_CLK_DIVIDER: c_int = 100;
pub const MSS_RXQ_TRESH_BASE: c_uint = 0x200;
pub const MSS_RXQ_TRESH_OFFS: c_int = 4;

// MSS_RXQ_TRESH_OFFS))
pub const MSS_BUF_POOL_BASE: c_uint = 0x40;
pub const MSS_BUF_POOL_OFFS: c_int = 4;

pub const MSS_BUF_POOL_STOP_MASK: c_uint = 0xFFF;

pub const MSS_BUF_POOL_START_OFFS: c_int = 12;

pub const MSS_BUF_POOL_PORTS_OFFS: c_int = 24;

pub const MSS_RXQ_TRESH_START_MASK: c_uint = 0xFFFF;

pub const MSS_RXQ_TRESH_STOP_OFFS: c_int = 16;
pub const MSS_RXQ_ASS_BASE: c_uint = 0x80;
pub const MSS_RXQ_ASS_OFFS: c_int = 4;
pub const MSS_RXQ_ASS_PER_REG: c_int = 4;
pub const MSS_RXQ_ASS_PER_OFFS: c_int = 8;
pub const MSS_RXQ_ASS_PORTID_OFFS: c_int = 0;
pub const MSS_RXQ_ASS_PORTID_MASK: c_uint = 0x3;
pub const MSS_RXQ_ASS_HOSTID_OFFS: c_int = 2;
pub const MSS_RXQ_ASS_HOSTID_MASK: c_uint = 0x3F;

// MSS_RXQ_ASS_PER_OFFS)

// MSS_RXQ_ASS_OFFS)

pub const MSS_THRESHOLD_STOP: c_int = 768;
pub const MSS_THRESHOLD_START: c_int = 1024;
pub const MSS_FC_MAX_TIMEOUT: c_int = 5000;
// RX buffer constants

pub const MVPP2_N_PRS_FLOWS: c_int = 52;
pub const MVPP2_N_RFS_ENTRIES_PER_FLOW: c_int = 4;
// There are 7 supported high-level flows

// RSS constants
pub const MVPP22_N_RSS_TABLES: c_int = 8;
pub const MVPP22_RSS_TABLE_ENTRIES: c_int = 32;
// IPv6 max L3 address size
pub const MVPP2_MAX_L3_ADDR_SIZE: c_int = 16;
// Port flags

// Marvell tag types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_tag_type {
    MVPP2_TAG_TYPE_NONE = 0,
    MVPP2_TAG_TYPE_MH   = 1,
    MVPP2_TAG_TYPE_DSA  = 2,
    MVPP2_TAG_TYPE_EDSA = 3,
    MVPP2_TAG_TYPE_VLAN = 4,
    MVPP2_TAG_TYPE_LAST = 5
}

// L2 cast enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_prs_l2_cast {
    MVPP2_PRS_L2_UNI_CAST,
    MVPP2_PRS_L2_MULTI_CAST,
}

// L3 cast enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_prs_l3_cast {
    MVPP2_PRS_L3_UNI_CAST,
    MVPP2_PRS_L3_MULTI_CAST,
    MVPP2_PRS_L3_BROAD_CAST
}

// PTP descriptor constants. The low bits of the descriptor are stored
// separately from the high bits.
//
pub const MVPP22_PTP_DESC_MASK_LOW: c_uint = 0xfff;
// PTPAction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp22_ptp_action {
    MVPP22_PTP_ACTION_NONE = 0,
    MVPP22_PTP_ACTION_FORWARD = 1,
    MVPP22_PTP_ACTION_CAPTURE = 3,
// The following have not been verified
    MVPP22_PTP_ACTION_ADDTIME = 4,
    MVPP22_PTP_ACTION_ADDCORRECTEDTIME = 5,
    MVPP22_PTP_ACTION_CAPTUREADDTIME = 6,
    MVPP22_PTP_ACTION_CAPTUREADDCORRECTEDTIME = 7,
    MVPP22_PTP_ACTION_ADDINGRESSTIME = 8,
    MVPP22_PTP_ACTION_CAPTUREADDINGRESSTIME = 9,
    MVPP22_PTP_ACTION_CAPTUREINGRESSTIME = 10,
}

// PTPPacketFormat
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp22_ptp_packet_format {
    MVPP22_PTP_PKT_FMT_PTPV2 = 0,
    MVPP22_PTP_PKT_FMT_PTPV1 = 1,
    MVPP22_PTP_PKT_FMT_Y1731 = 2,
    MVPP22_PTP_PKT_FMT_NTPTS = 3,
    MVPP22_PTP_PKT_FMT_NTPRX = 4,
    MVPP22_PTP_PKT_FMT_NTPTX = 5,
    MVPP22_PTP_PKT_FMT_TWAMP = 6,
}

// BM constants
pub const MVPP2_BM_JUMBO_BUF_NUM: c_int = 2048;
pub const MVPP2_BM_LONG_BUF_NUM: c_int = 2048;
pub const MVPP2_BM_SHORT_BUF_NUM: c_int = 2048;

pub const MVPP2_BM_POOL_PTR_ALIGN: c_int = 128;
pub const MVPP2_BM_MAX_POOLS: c_int = 8;
// BM cookie (32 bits) definition
pub const MVPP2_BM_COOKIE_POOL_OFFS: c_int = 8;
pub const MVPP2_BM_COOKIE_CPU_OFFS: c_int = 24;

// BM short pool packet size
// These value assure that for SWF the total number
// of bytes allocated for each buffer will be 512
//

pub const MVPP21_ADDR_SPACE_SZ: c_int = 0;

pub const MVPP2_MAX_THREADS: c_int = 9;

// GMAC MIB Counters register definitions
pub const MVPP21_MIB_COUNTERS_OFFSET: c_uint = 0x1000;
pub const MVPP21_MIB_COUNTERS_PORT_SZ: c_uint = 0x400;
pub const MVPP22_MIB_COUNTERS_OFFSET: c_uint = 0x0;
pub const MVPP22_MIB_COUNTERS_PORT_SZ: c_uint = 0x100;
pub const MVPP2_MIB_GOOD_OCTETS_RCVD: c_uint = 0x0;
pub const MVPP2_MIB_BAD_OCTETS_RCVD: c_uint = 0x8;
pub const MVPP2_MIB_CRC_ERRORS_SENT: c_uint = 0xc;
pub const MVPP2_MIB_UNICAST_FRAMES_RCVD: c_uint = 0x10;
pub const MVPP2_MIB_BROADCAST_FRAMES_RCVD: c_uint = 0x18;
pub const MVPP2_MIB_MULTICAST_FRAMES_RCVD: c_uint = 0x1c;
pub const MVPP2_MIB_FRAMES_64_OCTETS: c_uint = 0x20;
pub const MVPP2_MIB_FRAMES_65_TO_127_OCTETS: c_uint = 0x24;
pub const MVPP2_MIB_FRAMES_128_TO_255_OCTETS: c_uint = 0x28;
pub const MVPP2_MIB_FRAMES_256_TO_511_OCTETS: c_uint = 0x2c;
pub const MVPP2_MIB_FRAMES_512_TO_1023_OCTETS: c_uint = 0x30;
pub const MVPP2_MIB_FRAMES_1024_TO_MAX_OCTETS: c_uint = 0x34;
pub const MVPP2_MIB_GOOD_OCTETS_SENT: c_uint = 0x38;
pub const MVPP2_MIB_UNICAST_FRAMES_SENT: c_uint = 0x40;
pub const MVPP2_MIB_MULTICAST_FRAMES_SENT: c_uint = 0x48;
pub const MVPP2_MIB_BROADCAST_FRAMES_SENT: c_uint = 0x4c;
pub const MVPP2_MIB_FC_SENT: c_uint = 0x54;
pub const MVPP2_MIB_FC_RCVD: c_uint = 0x58;
pub const MVPP2_MIB_RX_FIFO_OVERRUN: c_uint = 0x5c;
pub const MVPP2_MIB_UNDERSIZE_RCVD: c_uint = 0x60;
pub const MVPP2_MIB_FRAGMENTS_RCVD: c_uint = 0x64;
pub const MVPP2_MIB_OVERSIZE_RCVD: c_uint = 0x68;
pub const MVPP2_MIB_JABBER_RCVD: c_uint = 0x6c;
pub const MVPP2_MIB_MAC_RCV_ERROR: c_uint = 0x70;
pub const MVPP2_MIB_BAD_CRC_EVENT: c_uint = 0x74;
pub const MVPP2_MIB_COLLISION: c_uint = 0x78;
pub const MVPP2_MIB_LATE_COLLISION: c_uint = 0x7c;

// Buffer header info bits
pub const MVPP2_B_HDR_INFO_MC_ID_MASK: c_uint = 0xfff;

pub const MVPP2_B_HDR_INFO_LAST_OFFS: c_int = 12;

// Definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_rss_table {
    pub indir: [u32; MVPP22_RSS_TABLE_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_buff_hdr {
    pub next_phys_addr: __le32,
    pub next_dma_addr: __le32,
    pub byte_count: __le16,
    pub info: __le16,
    pub /: *mut *mut __le16 reserved1; / bm_qset (for future use, BM),
    pub next_phys_addr_high: u8,
    pub next_dma_addr_high: u8,
    pub reserved2: __le16,
    pub reserved3: __le16,
    pub reserved4: __le16,
    pub reserved5: __le16,
}

// Shared Packet Processor resources
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2 {
// Shared registers' base addresses
    pub lms_base: *mut void __iomem,
    pub iface_base: *mut void __iomem,
    pub cm3_base: *mut void __iomem,
// On PPv2.2 and PPv2.3, each "software thread" can access the base
// register through a separate address space, each 64 KB apart
// from each other. Typically, such address spaces will be
// used per CPU.
//
    pub swth_base: [*mut void __iomem; MVPP2_MAX_THREADS],
// On PPv2.2 and PPv2.3, some port control registers are located into
// the system controller space. These registers are accessible
// through a regmap.
//
    pub sysctrl_base: *mut regmap,
// Common clocks
    pub pp_clk: *mut clk,
    pub gop_clk: *mut clk,
    pub mg_clk: *mut clk,
    pub mg_core_clk: *mut clk,
    pub axi_clk: *mut clk,
// List of pointers to port structures
    pub port_count: c_int,
    pub port_list: [*mut mvpp2_port; MVPP2_MAX_PORTS],
// Map of enabled ports
    pub port_map: c_ulong,
    pub tai: *mut mvpp2_tai,
// Number of Tx threads used
    pub nthreads: c_uint,
// Map of threads needing locking
    pub lock_map: c_ulong,
// Aggregated TXQs
    pub aggr_txqs: *mut mvpp2_tx_queue,
// Are we using page_pool with per-cpu pools?
    pub percpu_pools: c_int,
// BM pools
    pub bm_pools: *mut mvpp2_bm_pool,
// PRS shadow table
    pub prs_shadow: *mut mvpp2_prs_shadow,
// PRS auxiliary table for double vlan entries control
    pub prs_double_vlans: *mut bool,
// Tclk value
    pub tclk: u32,
// HW version
    pub hw_version: { MVPP21, MVPP22, MVPP23 },
// Maximum number of RXQs per port
    pub max_port_rxqs: c_uint,
// Workqueue to gather hardware statistics
    pub queue_name: [c_char; 31],
    pub stats_queue: *mut workqueue_struct,
// Debugfs root entry
    pub dbgfs_dir: *mut dentry,
// Debugfs entries private data
    pub dbgfs_entries: *mut mvpp2_dbgfs_entries,
// RSS Indirection tables
    pub rss_tables: [*mut mvpp2_rss_table; MVPP22_N_RSS_TABLES],
// page_pool allocator
    pub page_pool: [*mut page_pool; MVPP2_PORT_MAX_RXQ],
// Global TX Flow Control config
    pub global_tx_fc: bool,
// Spinlocks for CM3 shared memory configuration
    pub mss_spinlock: spinlock_t,
// Spinlock for shared PRS parser memory and shadow table
    pub prs_spinlock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_pcpu_stats {
    pub syncp: u64_stats_sync,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
// XDP
    pub xdp_redirect: u64,
    pub xdp_pass: u64,
    pub xdp_drop: u64,
    pub xdp_xmit: u64,
    pub xdp_xmit_err: u64,
    pub xdp_tx: u64,
    pub xdp_tx_err: u64,
}

// Per-CPU port control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_port_pcpu {
    pub tx_done_timer: hrtimer,
    pub dev: *mut net_device,
    pub timer_scheduled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_queue_vector {
    pub irq: c_int,
    pub napi: napi_struct,
    pub type: { MVPP2_QUEUE_VECTOR_SHARED, MVPP2_QUEUE_VECTOR_PRIVATE },
    pub sw_thread_id: c_int,
    pub sw_thread_mask: u16,
    pub first_rxq: c_int,
    pub nrxqs: c_int,
    pub pending_cause_rx: u32,
    pub port: *mut mvpp2_port,
    pub mask: *mut cpumask,
}

// Internal represention of a Flow Steering rule
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_rfs_rule {
// Rule location inside the flow
    pub loc: c_int,
// Flow type, such as TCP_V4_FLOW, IP6_FLOW, etc.
    pub flow_type: c_int,
// Index of the C2 TCAM entry handling this rule
    pub c2_index: c_int,
// Header fields that needs to be extracted to match this flow
    pub hek_fields: u16,
// CLS engine : only c2 is supported for now.
    pub engine: u8,
// TCAM key and mask for C2-based steering. These fields should be
// encapsulated in a union should we add more engines.
//
    pub c2_tcam: u64,
    pub c2_tcam_mask: u64,
    pub flow: *mut flow_rule,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_ethtool_fs {
    pub rule: mvpp2_rfs_rule,
    pub rxnfc: ethtool_rxnfc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_hwtstamp_queue {
    pub skb: [*mut sk_buff; 32],
    pub next: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_port {
    pub id: u8,
// Index of the port from the "group of ports" complex point
// of view. This is specific to PPv2.2.
//
    pub gop_id: c_int,
    pub port_irq: c_int,
    pub priv: *mut mvpp2,
// Firmware node associated to the port
    pub fwnode: *mut fwnode_handle,
// Per-port registers' base address
    pub base: *mut void __iomem,
    pub stats_base: *mut void __iomem,
    pub rxqs: *mut mvpp2_rx_queue,
    pub nrxqs: c_uint,
    pub txqs: *mut mvpp2_tx_queue,
    pub ntxqs: c_uint,
    pub dev: *mut net_device,
    pub xdp_prog: *mut bpf_prog,
    pub pkt_size: c_int,
// Per-CPU port control
    pub pcpu: *mut mvpp2_port_pcpu __percpu,
// Protect the BM refills and the Tx paths when a thread is used on more
// than a single CPU.
//
    pub bm_lock: [spinlock_t; MVPP2_MAX_THREADS],
    pub tx_lock: [spinlock_t; MVPP2_MAX_THREADS],
// Flags
    pub flags: c_ulong,
    pub tx_ring_size: u16,
    pub rx_ring_size: u16,
    pub stats: *mut mvpp2_pcpu_stats __percpu,
    pub ethtool_stats: *mut u64,
    pub state: c_ulong,
// Per-port work and its lock to gather hardware statistics
    pub gather_stats_lock: mutex,
    pub stats_work: delayed_work,
    pub of_node: *mut device_node,
    pub phy_interface: phy_interface_t,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub pcs_gmac: phylink_pcs,
    pub pcs_xlg: phylink_pcs,
    pub comphy: *mut phy,
    pub pool_long: *mut mvpp2_bm_pool,
    pub pool_short: *mut mvpp2_bm_pool,
// Index of first port's physical RXQ
    pub first_rxq: u8,
    pub qvecs: [mvpp2_queue_vector; MVPP2_MAX_QVECS],
    pub nqvecs: c_uint,
    pub has_tx_irqs: bool,
    pub tx_time_coal: u32,
// List of steering rules active on that port
    pub rfs_rules: [*mut mvpp2_ethtool_fs; MVPP2_N_RFS_ENTRIES_PER_FLOW],
    pub n_rfs_rules: c_int,
// Each port has its own view of the rss contexts, so that it can number
// them from 0
//
    pub rss_ctx: [c_int; MVPP22_N_RSS_TABLES],
    pub hwtstamp: bool,
    pub rx_hwtstamp: bool,
    pub tx_hwtstamp_type: hwtstamp_tx_types,
    pub tx_hwtstamp_queue: [mvpp2_hwtstamp_queue; 2],
// Firmware TX flow control
    pub tx_fc: bool,
}

// The mvpp2_tx_desc and mvpp2_rx_desc structures describe the
// layout of the transmit and reception DMA descriptors, and their
// layout is therefore defined by the hardware design
//
pub const MVPP2_TXD_L3_OFF_SHIFT: c_int = 0;
pub const MVPP2_TXD_IP_HLEN_SHIFT: c_int = 8;

pub const MVPP2_RXD_ERR_CRC: c_uint = 0x0;

pub const MVPP2_RXD_BM_POOL_ID_OFFS: c_int = 16;

// HW TX descriptor for PPv2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp21_tx_desc {
    pub transmitting.*/: *mut *mut __le32 command; / Options used by HW for packet,
    pub /: *mut *mut u8 packet_offset; / the offset from the buffer beginning,
    pub /: *mut *mut u8 phys_txq; / destination queue ID,
    pub /: *mut *mut __le16 data_size; / data size of transmitted packet in bytes,
    pub /: *mut *mut __le32 buf_dma_addr; / physical addr of transmitted buffer,
    pub /: *mut *mut __le32 buf_cookie; / cookie for access to TX buffer in tx path,
    pub /: *mut *mut __le32 reserved1[3]; / hw_cmd (for future use, BM, PON, PNC),
    pub /: *mut *mut __le32 reserved2; / reserved (for future use),
}

// HW RX descriptor for PPv2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp21_rx_desc {
    pub /: *mut *mut __le32 status; / info about received packet,
    pub /: *mut *mut __le16 reserved1; / parser_info (for future use, PnC),
    pub /: *mut *mut __le16 data_size; / size of received packet in bytes,
    pub /: *mut *mut __le32 buf_dma_addr; / physical address of the buffer,
    pub /: *mut *mut __le32 buf_cookie; / cookie for access to RX buffer in rx path,
    pub /: *mut *mut __le16 reserved2; / gem_port_id (for future use, PON),
    pub /: *mut *mut __le16 reserved3; / csum_l4 (for future use, PnC),
    pub /: *mut *mut u8 reserved4; / bm_qset (for future use, BM),
    pub reserved5: u8,
    pub /: *mut *mut __le16 reserved6; / classify_info (for future use, PnC),
    pub /: *mut *mut __le32 reserved7; / flow_id (for future use, PnC),
    pub reserved8: __le32,
}

// HW TX descriptor for PPv2.2 and PPv2.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp22_tx_desc {
    pub command: __le32,
    pub packet_offset: u8,
    pub phys_txq: u8,
    pub data_size: __le16,
    pub ptp_descriptor: __le32,
    pub reserved2: __le32,
    pub buf_dma_addr_ptp: __le64,
    pub buf_cookie_misc: __le64,
}

// HW RX descriptor for PPv2.2 and PPv2.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp22_rx_desc {
    pub status: __le32,
    pub reserved1: __le16,
    pub data_size: __le16,
    pub reserved2: __le32,
    pub timestamp: __le32,
    pub buf_dma_addr_key_hash: __le64,
    pub buf_cookie_misc: __le64,
}

// Opaque type used by the driver to manipulate the HW TX and RX
// descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_tx_desc {
    pub pp21: mvpp21_tx_desc,
    pub pp22: mvpp22_tx_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_rx_desc {
    pub pp21: mvpp21_rx_desc,
    pub pp22: mvpp22_rx_desc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_tx_buf_type {
    MVPP2_TYPE_SKB,
    MVPP2_TYPE_XDP_TX,
    MVPP2_TYPE_XDP_NDO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_txq_pcpu_buf {
    pub type: mvpp2_tx_buf_type,
// Transmitted SKB
    pub xdpf: *mut xdp_frame,
    pub skb: *mut sk_buff,
}

// Physical address of transmitted buffer
// Size transmitted
// Per-CPU Tx queue control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_txq_pcpu {
    pub thread: c_uint,
// Number of Tx DMA descriptors in the descriptor ring
    pub size: c_int,
// Number of currently used Tx DMA descriptor in the
// descriptor ring
//
    pub count: c_int,
    pub wake_threshold: c_int,
    pub stop_threshold: c_int,
// Number of Tx DMA descriptors reserved for each CPU
    pub reserved_num: c_int,
// Infos about transmitted buffers
    pub buffs: *mut mvpp2_txq_pcpu_buf,
// Index of last TX DMA descriptor that was inserted
    pub txq_put_index: c_int,
// Index of the TX DMA descriptor to be cleaned up
    pub txq_get_index: c_int,
// DMA buffer for TSO headers
    pub tso_headers: *mut c_char,
    pub tso_headers_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_tx_queue {
// Physical number of this Tx queue
    pub id: u8,
// Logical number of this Tx queue
    pub log_id: u8,
// Number of Tx DMA descriptors in the descriptor ring
    pub size: c_int,
// Number of currently used Tx DMA descriptor in the descriptor ring
    pub count: c_int,
// Per-CPU control of physical Tx queues
    pub pcpu: *mut mvpp2_txq_pcpu __percpu,
    pub done_pkts_coal: u32,
// Virtual address of thex Tx DMA descriptors array
    pub descs: *mut mvpp2_tx_desc,
// DMA address of the Tx DMA descriptors array
    pub descs_dma: dma_addr_t,
// Index of the last Tx DMA descriptor
    pub last_desc: c_int,
// Index of the next Tx DMA descriptor to process
    pub next_desc_to_proc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_rx_queue {
// RX queue number, in the range 0-31 for physical RXQs
    pub id: u8,
// Num of rx descriptors in the rx descriptor ring
    pub size: c_int,
    pub pkts_coal: u32,
    pub time_coal: u32,
// Virtual address of the RX DMA descriptors array
    pub descs: *mut mvpp2_rx_desc,
// DMA address of the RX DMA descriptors array
    pub descs_dma: dma_addr_t,
// Index of the last RX DMA descriptor
    pub last_desc: c_int,
// Index of the next RX DMA descriptor to process
    pub next_desc_to_proc: c_int,
// ID of port to which physical RXQ is mapped
    pub port: c_int,
// Port's logic RXQ number to which physical RXQ is mapped
    pub logic_rxq: c_int,
// XDP memory accounting
    pub xdp_rxq_short: xdp_rxq_info,
    pub xdp_rxq_long: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_bm_pool {
// Pool number in the range 0-7
    pub id: c_int,
// Buffer Pointers Pool External (BPPE) size
    pub size: c_int,
// BPPE size in bytes
    pub size_bytes: c_int,
// Number of buffers for this pool
    pub buf_num: c_int,
// Pool buffer size
    pub buf_size: c_int,
// Packet size
    pub pkt_size: c_int,
    pub frag_size: c_int,
// BPPE virtual base address
    pub virt_addr: *mut u32,
// BPPE DMA base address
    pub dma_addr: dma_addr_t,
// Ports using BM pool
    pub port_map: u32,
}

extern "C" {
    pub fn mvpp2_write(priv: *mut mvpp2, offset: u32, data: u32);
}
extern "C" {
    pub fn mvpp2_read(priv: *mut mvpp2, offset: u32) -> u32;
}
extern "C" {
    pub fn mvpp2_dbgfs_init(priv: *mut mvpp2, name: *const c_char);
}
extern "C" {
    pub fn mvpp2_dbgfs_cleanup(priv: *mut mvpp2);
}
extern "C" {
    pub fn mvpp2_dbgfs_exit();
}
extern "C" {
    pub fn mvpp23_rx_fifo_fc_en(priv: *mut mvpp2, port: c_int, en: bool);
}

extern "C" {
    pub fn mvpp22_tai_probe(dev: *mut device, priv: *mut mvpp2) -> c_int;
}
extern "C" {
    pub fn mvpp22_tai_start(tai: *mut mvpp2_tai);
}
extern "C" {
    pub fn mvpp22_tai_stop(tai: *mut mvpp2_tai);
}
extern "C" {
    pub fn mvpp22_tai_ptp_clock_index(tai: *mut mvpp2_tai) -> c_int;
}

