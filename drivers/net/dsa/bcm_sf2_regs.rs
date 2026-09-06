//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/bcm_sf2_regs.h
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
// Broadcom Starfighter 2 switch register defines
//
// Copyright (C) 2014, Broadcom Corporation
//
// Register set relative to 'REG'
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm_sf2_reg_offs {
    REG_SWITCH_CNTRL = 0,
    REG_SWITCH_STATUS,
    REG_DIR_DATA_WRITE,
    REG_DIR_DATA_READ,
    REG_SWITCH_REVISION,
    REG_PHY_REVISION,
    REG_SPHY_CNTRL,
    REG_CROSSBAR,
    REG_RGMII_0_CNTRL,
    REG_RGMII_1_CNTRL,
    REG_RGMII_2_CNTRL,
    REG_RGMII_11_CNTRL,
    REG_LED_0_CNTRL,
    REG_LED_1_CNTRL,
    REG_LED_2_CNTRL,
    REG_LED_3_CNTRL,
    REG_LED_4_CNTRL,
    REG_LED_5_CNTRL,
    REG_LED_AGGREGATE_CTRL,
    REG_SWITCH_REG_MAX,
}

// Relative to REG_SWITCH_CNTRL

// Relative to REG_SWITCH_REVISION
pub const SF2_REV_MASK: c_uint = 0xffff;
pub const SWITCH_TOP_REV_SHIFT: c_int = 16;
pub const SWITCH_TOP_REV_MASK: c_uint = 0xffff;
// Relative to REG_PHY_REVISION
pub const PHY_REVISION_MASK: c_uint = 0xffff;
// Relative to REG_SPHY_CNTRL

pub const PHY_PHYAD_SHIFT: c_int = 8;
pub const PHY_PHYAD_MASK: c_uint = 0x1F;
// Relative to REG_CROSSBAR
pub const CROSSBAR_BCM4908_INT_P7: c_int = 0;
pub const CROSSBAR_BCM4908_INT_RUNNER: c_int = 1;
pub const CROSSBAR_BCM4908_EXT_SERDES: c_int = 0;
pub const CROSSBAR_BCM4908_EXT_GPHY4: c_int = 1;
pub const CROSSBAR_BCM4908_EXT_RGMII: c_int = 2;
// Relative to REG_LED_*_CNTRL (BCM7278, BCM7445)
pub const LED_CNTRL_NO_LINK_ENCODE_SHIFT: c_int = 0;
pub const LED_CNTRL_M10_ENCODE_SHIFT: c_int = 2;
pub const LED_CNTRL_M100_ENCODE_SHIFT: c_int = 4;
pub const LED_CNTRL_M1000_ENCODE_SHIFT: c_int = 6;
pub const LED_CNTRL_SEL_NO_LINK_ENCODE_SHIFT: c_int = 8;
pub const LED_CNTRL_SEL_10M_ENCODE_SHIFT: c_int = 10;
pub const LED_CNTRL_SEL_100M_ENCODE_SHIFT: c_int = 12;
pub const LED_CNTRL_SEL_1000M_ENCODE_SHIFT: c_int = 14;

pub const LED_CNTRL_SPDLNK_LED0_ACT_SEL_SHIFT: c_int = 18;
pub const LED_CNTRL_SPDLNK_LED1_ACT_SEL_SHIFT: c_int = 20;
pub const LED_CNTRL_ACT_LED_ACT_SEL_SHIFT: c_int = 22;

pub const LED_CNTRL_MASK: c_uint = 0x3;
// Register relative to REG_LED_*_CNTRL (BCM4908)
pub const REG_LED_CTRL: c_uint = 0x0;
pub const LED_CTRL_RX_ACT_EN: c_uint = 0x00000001;
pub const LED_CTRL_TX_ACT_EN: c_uint = 0x00000002;
pub const LED_CTRL_SPDLNK_LED0_ACT_SEL: c_uint = 0x00000004;
pub const LED_CTRL_SPDLNK_LED1_ACT_SEL: c_uint = 0x00000008;
pub const LED_CTRL_SPDLNK_LED2_ACT_SEL: c_uint = 0x00000010;
pub const LED_CTRL_ACT_LED_ACT_SEL: c_uint = 0x00000020;
pub const LED_CTRL_SPDLNK_LED0_ACT_POL_SEL: c_uint = 0x00000040;
pub const LED_CTRL_SPDLNK_LED1_ACT_POL_SEL: c_uint = 0x00000080;
pub const LED_CTRL_SPDLNK_LED2_ACT_POL_SEL: c_uint = 0x00000100;
pub const LED_CTRL_ACT_LED_POL_SEL: c_uint = 0x00000200;
pub const LED_CTRL_LED_SPD_OVRD: c_uint = 0x00001c00;
pub const LED_CTRL_LNK_STATUS_OVRD: c_uint = 0x00002000;
pub const LED_CTRL_SPD_OVRD_EN: c_uint = 0x00004000;
pub const LED_CTRL_LNK_OVRD_EN: c_uint = 0x00008000;
// Register relative to REG_LED_*_CNTRL (BCM4908)
pub const REG_LED_LINK_SPEED_ENC_SEL: c_uint = 0x4;
pub const LED_LINK_SPEED_ENC_SEL_NO_LINK_SHIFT: c_int = 0;
pub const LED_LINK_SPEED_ENC_SEL_10M_SHIFT: c_int = 3;
pub const LED_LINK_SPEED_ENC_SEL_100M_SHIFT: c_int = 6;
pub const LED_LINK_SPEED_ENC_SEL_1000M_SHIFT: c_int = 9;
pub const LED_LINK_SPEED_ENC_SEL_2500M_SHIFT: c_int = 12;
pub const LED_LINK_SPEED_ENC_SEL_10G_SHIFT: c_int = 15;
pub const LED_LINK_SPEED_ENC_SEL_MASK: c_uint = 0x7;
// Register relative to REG_LED_*_CNTRL (BCM4908)
pub const REG_LED_LINK_SPEED_ENC: c_uint = 0x8;
pub const LED_LINK_SPEED_ENC_NO_LINK_SHIFT: c_int = 0;
pub const LED_LINK_SPEED_ENC_M10_SHIFT: c_int = 3;
pub const LED_LINK_SPEED_ENC_M100_SHIFT: c_int = 6;
pub const LED_LINK_SPEED_ENC_M1000_SHIFT: c_int = 9;
pub const LED_LINK_SPEED_ENC_M2500_SHIFT: c_int = 12;
pub const LED_LINK_SPEED_ENC_M10G_SHIFT: c_int = 15;
pub const LED_LINK_SPEED_ENC_MASK: c_uint = 0x7;
// Relative to REG_RGMII_CNTRL

pub const PORT_MODE_SHIFT: c_int = 2;

pub const PORT_MODE_MASK: c_uint = 0x7;

pub const LPI_COUNT_SHIFT: c_int = 9;
pub const LPI_COUNT_MASK: c_uint = 0x3F;
// Register set relative to 'INTRL2_0' and 'INTRL2_1'
pub const INTRL2_CPU_STATUS: c_uint = 0x00;
pub const INTRL2_CPU_SET: c_uint = 0x04;
pub const INTRL2_CPU_CLEAR: c_uint = 0x08;
pub const INTRL2_CPU_MASK_STATUS: c_uint = 0x0c;
pub const INTRL2_CPU_MASK_SET: c_uint = 0x10;
pub const INTRL2_CPU_MASK_CLEAR: c_uint = 0x14;
// Shared INTRL2_0 and INTRL2_ interrupt sources macros

pub const P_NUM_IRQ: c_int = 5;

// INTRL2_0 interrupt sources
pub const P0_IRQ_OFF: c_int = 0;

// INTRL2_1 interrupt sources
pub const P7_IRQ_OFF: c_int = 0;

// Register set relative to 'ACB'
pub const ACB_CONTROL: c_uint = 0x00;

pub const ACB_FLUSH_SHIFT: c_int = 2;
pub const ACB_FLUSH_MASK: c_uint = 0x3;
pub const ACB_QUEUE_0_CFG: c_uint = 0x08;
pub const XOFF_THRESHOLD_MASK: c_uint = 0x7ff;

pub const TOTAL_XOFF_THRESHOLD_SHIFT: c_int = 12;
pub const TOTAL_XOFF_THRESHOLD_MASK: c_uint = 0x7ff;

pub const PKTLEN_SHIFT: c_int = 25;
pub const PKTLEN_MASK: c_uint = 0x3f;

// Register set relative to 'CORE'
pub const CORE_G_PCTL_PORT0: c_uint = 0x00000;

pub const CORE_IMP_CTL: c_uint = 0x00020;

pub const CORE_SWMODE: c_uint = 0x0002c;

pub const CORE_STS_OVERRIDE_IMP: c_uint = 0x00038;

// Alternate layout for e.g: 7278
pub const CORE_STS_OVERRIDE_IMP2: c_uint = 0x39040;
pub const CORE_NEW_CTRL: c_uint = 0x00084;

pub const CORE_SWITCH_CTRL: c_uint = 0x00088;

pub const CORE_DIS_LEARN: c_uint = 0x000f0;
pub const CORE_SFT_LRN_CTRL: c_uint = 0x000f8;

pub const SPEED_SHIFT: c_int = 2;
pub const SPEED_MASK: c_uint = 0x3;

pub const CORE_WATCHDOG_CTRL: c_uint = 0x001e4;

pub const CORE_FAST_AGE_CTRL: c_uint = 0x00220;

pub const CORE_FAST_AGE_PORT: c_uint = 0x00224;
pub const AGE_PORT_MASK: c_uint = 0xf;
pub const CORE_FAST_AGE_VID: c_uint = 0x00228;
pub const AGE_VID_MASK: c_uint = 0x3fff;
pub const CORE_LNKSTS: c_uint = 0x00400;
pub const LNK_STS_MASK: c_uint = 0x1ff;
pub const CORE_SPDSTS: c_uint = 0x00410;
pub const SPDSTS_10: c_int = 0;
pub const SPDSTS_100: c_int = 1;
pub const SPDSTS_1000: c_int = 2;
pub const SPDSTS_SHIFT: c_int = 2;
pub const SPDSTS_MASK: c_uint = 0x3;
pub const CORE_DUPSTS: c_uint = 0x00420;
pub const CORE_DUPSTS_MASK: c_uint = 0x1ff;
pub const CORE_PAUSESTS: c_uint = 0x00428;
pub const PAUSESTS_TX_PAUSE_SHIFT: c_int = 9;
pub const CORE_GMNCFGCFG: c_uint = 0x0800;

pub const CORE_IMP0_PRT_ID: c_uint = 0x0804;
pub const CORE_RST_MIB_CNT_EN: c_uint = 0x0950;
pub const CORE_ARLA_VTBL_RWCTRL: c_uint = 0x1600;
pub const ARLA_VTBL_CMD_WRITE: c_int = 0;
pub const ARLA_VTBL_CMD_READ: c_int = 1;
pub const ARLA_VTBL_CMD_CLEAR: c_int = 2;

pub const CORE_ARLA_VTBL_ADDR: c_uint = 0x1604;
pub const VTBL_ADDR_INDEX_MASK: c_uint = 0xfff;
pub const CORE_ARLA_VTBL_ENTRY: c_uint = 0x160c;
pub const FWD_MAP_MASK: c_uint = 0x1ff;
pub const UNTAG_MAP_MASK: c_uint = 0x1ff;
pub const UNTAG_MAP_SHIFT: c_int = 9;
pub const MSTP_INDEX_MASK: c_uint = 0x7;
pub const MSTP_INDEX_SHIFT: c_int = 18;

pub const CORE_MEM_PSM_VDD_CTRL: c_uint = 0x2380;
pub const P_TXQ_PSM_VDD_SHIFT: c_int = 2;
pub const P_TXQ_PSM_VDD_MASK: c_uint = 0x3;

pub const PRT_TO_QID_MASK: c_uint = 0x3;
pub const PRT_TO_QID_SHIFT: c_int = 3;

pub const PORT_VLAN_CTRL_MASK: c_uint = 0x1ff;
pub const CORE_TXQ_THD_PAUSE_QN_PORT_0: c_uint = 0x2c80;
pub const TXQ_PAUSE_THD_MASK: c_uint = 0x7ff;

pub const CFI_SHIFT: c_int = 12;
pub const PRI_SHIFT: c_int = 13;
pub const PRI_MASK: c_uint = 0x7;
pub const CORE_JOIN_ALL_VLAN_EN: c_uint = 0xd140;
pub const CORE_CFP_ACC: c_uint = 0x28000;

pub const OP_SEL_SHIFT: c_int = 1;

pub const RAM_SEL_SHIFT: c_int = 10;

pub const XCESS_ADDR_SHIFT: c_int = 16;
pub const XCESS_ADDR_MASK: c_uint = 0xff;

pub const RD_STS_SHIFT: c_int = 28;

pub const CORE_CFP_RATE_METER_GLOBAL_CTL: c_uint = 0x28010;
pub const CORE_CFP_DATA_PORT_0: c_uint = 0x28040;

// UDF_DATA7
pub const L3_FRAMING_SHIFT: c_int = 24;

pub const IPTOS_SHIFT: c_int = 16;
pub const IPTOS_MASK: c_uint = 0xff;
pub const IPPROTO_SHIFT: c_int = 8;

pub const IP_FRAG_SHIFT: c_int = 7;

// UDF_DATA0
pub const SLICE_VALID: c_int = 3;
pub const SLICE_NUM_SHIFT: c_int = 2;

pub const SLICE_NUM_MASK: c_uint = 0x3;
pub const CORE_CFP_MASK_PORT_0: c_uint = 0x280c0;

pub const CORE_ACT_POL_DATA0: c_uint = 0x28140;

pub const REASON_CODE_SHIFT: c_int = 3;
pub const REASON_CODE_MASK: c_uint = 0x3f;

pub const NEW_TC_SHIFT: c_int = 10;
pub const NEW_TC_MASK: c_uint = 0x7;

pub const DST_MAP_IB_SHIFT: c_int = 14;
pub const DST_MAP_IB_MASK: c_uint = 0x1ff;
pub const CHANGE_FWRD_MAP_IB_SHIFT: c_int = 24;
pub const CHANGE_FWRD_MAP_IB_MASK: c_uint = 0x3;

pub const NEW_DSCP_IB_SHIFT: c_int = 26;
pub const NEW_DSCP_IB_MASK: c_uint = 0x3f;
pub const CORE_ACT_POL_DATA1: c_uint = 0x28150;

pub const DST_MAP_OB_SHIFT: c_int = 1;
pub const DST_MAP_OB_MASK: c_uint = 0x3ff;
pub const CHANGE_FWRD_MAP_OB_SHIT: c_int = 11;
pub const CHANGE_FWRD_MAP_OB_MASK: c_uint = 0x3;
pub const NEW_DSCP_OB_SHIFT: c_int = 13;
pub const NEW_DSCP_OB_MASK: c_uint = 0x3f;

pub const CHAIN_ID_SHIFT: c_int = 20;
pub const CHAIN_ID_MASK: c_uint = 0xff;

pub const NEW_COLOR_SHIFT: c_int = 29;
pub const NEW_COLOR_MASK: c_uint = 0x3;

pub const CORE_ACT_POL_DATA2: c_uint = 0x28160;

pub const NEW_TC_O_SHIFT: c_int = 2;
pub const NEW_TC_O_MASK: c_uint = 0x7;

pub const CORE_RATE_METER0: c_uint = 0x28180;

pub const POLICER_MODE_SHIFT: c_int = 3;
pub const POLICER_MODE_MASK: c_uint = 0x3;

pub const CORE_RATE_METER1: c_uint = 0x28190;
pub const EIR_TK_BKT_MASK: c_uint = 0x7fffff;
pub const CORE_RATE_METER2: c_uint = 0x281a0;
pub const EIR_BKT_SIZE_MASK: c_uint = 0xfffff;
pub const CORE_RATE_METER3: c_uint = 0x281b0;
pub const EIR_REF_CNT_MASK: c_uint = 0x7ffff;
pub const CORE_RATE_METER4: c_uint = 0x281c0;
pub const CIR_TK_BKT_MASK: c_uint = 0x7fffff;
pub const CORE_RATE_METER5: c_uint = 0x281d0;
pub const CIR_BKT_SIZE_MASK: c_uint = 0xfffff;
pub const CORE_RATE_METER6: c_uint = 0x281e0;
pub const CIR_REF_CNT_MASK: c_uint = 0x7ffff;
pub const CORE_STAT_GREEN_CNTR: c_uint = 0x28200;
pub const CORE_STAT_YELLOW_CNTR: c_uint = 0x28210;
pub const CORE_STAT_RED_CNTR: c_uint = 0x28220;
pub const CORE_CFP_CTL_REG: c_uint = 0x28400;
pub const CFP_EN_MAP_MASK: c_uint = 0x1ff;
// IPv4 slices, 3 of them
pub const CORE_UDF_0_A_0_8_PORT_0: c_uint = 0x28440;
pub const CFG_UDF_OFFSET_MASK: c_uint = 0x1f;
pub const CFG_UDF_OFFSET_BASE_SHIFT: c_int = 5;

// IPv6 slices
pub const CORE_UDF_0_B_0_8_PORT_0: c_uint = 0x28500;
// IPv6 chained slices
pub const CORE_UDF_0_D_0_11_PORT_0: c_uint = 0x28680;
// Number of slices for IPv4, IPv6 and non-IP
pub const UDF_NUM_SLICES: c_int = 4;
pub const UDFS_PER_SLICE: c_int = 9;
// Spacing between different slices
pub const UDF_SLICE_OFFSET: c_uint = 0x40;
pub const CFP_NUM_RULES: c_int = 256;
// Number of egress queues per port
pub const SF2_NUM_EGRESS_QUEUES: c_int = 8;
