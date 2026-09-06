//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7915/regs.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2020 MediaTek Inc.
// used to differentiate between generations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_reg_desc {
    pub reg_rev: *const u32,
    pub offs_rev: *const u32,
    pub map: *const mt76_connac_reg_map,
    pub map_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg_rev {
    INT_SOURCE_CSR,
    INT_MASK_CSR,
    INT1_SOURCE_CSR,
    INT1_MASK_CSR,
    INT_MCU_CMD_SOURCE,
    INT_MCU_CMD_EVENT,
    WFDMA0_ADDR,
    WFDMA0_PCIE1_ADDR,
    WFDMA_EXT_CSR_ADDR,
    CBTOP1_PHY_END,
    INFRA_MCU_ADDR_END,
    FW_ASSERT_STAT_ADDR,
    FW_EXCEPT_TYPE_ADDR,
    FW_EXCEPT_COUNT_ADDR,
    FW_CIRQ_COUNT_ADDR,
    FW_CIRQ_IDX_ADDR,
    FW_CIRQ_LISR_ADDR,
    FW_TASK_ID_ADDR,
    FW_TASK_IDX_ADDR,
    FW_TASK_QID1_ADDR,
    FW_TASK_QID2_ADDR,
    FW_TASK_START_ADDR,
    FW_TASK_END_ADDR,
    FW_TASK_SIZE_ADDR,
    FW_LAST_MSG_ID_ADDR,
    FW_EINT_INFO_ADDR,
    FW_SCHED_INFO_ADDR,
    SWDEF_BASE_ADDR,
    TXQ_WED_RING_BASE,
    RXQ_WED_RING_BASE,
    RXQ_WED_DATA_RING_BASE,
    __MT_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum offs_rev {
    TMAC_CDTR,
    TMAC_ODTR,
    TMAC_ATCR,
    TMAC_TRCR0,
    TMAC_ICR0,
    TMAC_ICR1,
    TMAC_CTCR0,
    TMAC_TFCR0,
    MDP_BNRCFR0,
    MDP_BNRCFR1,
    ARB_DRNGR0,
    ARB_SCR,
    RMAC_MIB_AIRTIME14,
    AGG_AWSCR0,
    AGG_PCR0,
    AGG_ACR0,
    AGG_ACR4,
    AGG_MRCR,
    AGG_ATCR0,
    AGG_ATCR1,
    AGG_ATCR3,
    LPON_UTTR0,
    LPON_UTTR1,
    LPON_FRCR,
    MIB_SDR3,
    MIB_SDR4,
    MIB_SDR5,
    MIB_SDR7,
    MIB_SDR8,
    MIB_SDR9,
    MIB_SDR10,
    MIB_SDR11,
    MIB_SDR12,
    MIB_SDR13,
    MIB_SDR14,
    MIB_SDR15,
    MIB_SDR16,
    MIB_SDR17,
    MIB_SDR18,
    MIB_SDR19,
    MIB_SDR20,
    MIB_SDR21,
    MIB_SDR22,
    MIB_SDR23,
    MIB_SDR24,
    MIB_SDR25,
    MIB_SDR27,
    MIB_SDR28,
    MIB_SDR29,
    MIB_SDRVEC,
    MIB_SDR31,
    MIB_SDR32,
    MIB_SDRMUBF,
    MIB_DR8,
    MIB_DR9,
    MIB_DR11,
    MIB_MB_SDR0,
    MIB_MB_SDR1,
    TX_AGG_CNT,
    TX_AGG_CNT2,
    MIB_ARNG,
    WTBLON_TOP_WDUCR,
    WTBL_UPDATE,
    PLE_FL_Q_EMPTY,
    PLE_FL_Q_CTRL,
    PLE_AC_QEMPTY,
    PLE_FREEPG_CNT,
    PLE_FREEPG_HEAD_TAIL,
    PLE_PG_HIF_GROUP,
    PLE_HIF_PG_INFO,
    AC_OFFSET,
    ETBF_PAR_RPT0,
    __MT_OFFS_MAX,
}

// MCU WFDMA0
pub const MT_MCU_WFDMA0_BASE: c_uint = 0x2000;

// MCU WFDMA1
pub const MT_MCU_WFDMA1_BASE: c_uint = 0x3000;

// PLE
pub const MT_PLE_BASE: c_uint = 0x820c0000;

pub const MT_PSE_BASE: c_uint = 0x820c8000;

// WF MDP TOP
pub const MT_MDP_BASE: c_uint = 0x820cd000;

pub const MT_MDP_TO_HIF: c_int = 0;
pub const MT_MDP_TO_WM: c_int = 1;
// TRB: band 0(0x820e1000), band 1(0x820f1000)

// TMAC: band 0(0x820e4000), band 1(0x820f4000)

// WF DMA TOP: band 0(0x820e7000),band 1(0x820f7000)

// WTBLOFF TOP: band 0(0x820e9000),band 1(0x820f9000)

// ETBF: band 0(0x820ea000), band 1(0x820fa000)

// LPON: band 0(0x820eb000), band 1(0x820fb000)

// MIB: band 0(0x820ed000), band 1(0x820fd000)
// These counters are (mostly?) clear-on-read.  So, some should not
// be read at all in case firmware is already reading them.  These
// are commented with 'DNR' below.  The DNR stats will be read by querying
// the firmware API for the appropriate message.  For counters the driver
// does read, the driver should accumulate the counters.
//

// rx mpdu counter, full 32 bits

// aka CCA_NAV_TX_TIME

// tx ampdu cnt, full 32 bits

// counts all mpdus in ampdu, regardless of success

// counts all successfully tx'd mpdus in ampdu

// in units of 'us'

// units are us

// rx ampdu count, 32-bit

// rx ampdu bytes count, 32-bit

// rx ampdu valid subframe count

// rx ampdu valid subframe bytes count, 32bits

// remaining windows protected stats

// rx blockack count, 32 bits

// 36, 37 both DNR

// WTBLON TOP
pub const MT_WTBLON_TOP_BASE: c_uint = 0x820d4000;

// WTBL
pub const MT_WTBL_BASE: c_uint = 0x820d8000;

// AGG: band 0(0x820e2000), band 1(0x820f2000)

// ARB: band 0(0x820e3000), band 1(0x820f3000)

// RMAC: band 0(0x820e5000), band 1(0x820f5000)

// WFDMA0

// WFDMA1
pub const MT_WFDMA1_BASE: c_uint = 0xd5000;

// WFDMA CSR

pub const MT_WFDMA_EXT_CSR_PHYS_BASE: c_uint = 0x18027000;

pub const MT_PCIE_RECOG_ID: c_uint = 0xd7090;

// WFDMA0 PCIE1

// WFDMA1 PCIE1
pub const MT_WFDMA1_PCIE1_BASE: c_uint = 0xd9000;

// WFDMA COMMON

// TOP RGU
pub const MT_TOP_RGU_BASE: c_uint = 0x18000000;

// l1/l2 remap
pub const MT_HIF_REMAP_L1: c_uint = 0xf11ac;
pub const MT_HIF_REMAP_L1_MT7916: c_uint = 0xfe260;

pub const MT_HIF_REMAP_BASE_L1: c_uint = 0xe0000;
pub const MT_HIF_REMAP_L2: c_uint = 0xf11b0;

pub const MT_HIF_REMAP_L2_MT7916: c_uint = 0x1b8;

pub const MT_HIF_REMAP_BASE_L2_MT7916: c_uint = 0x40000;
pub const MT_INFRA_BASE: c_uint = 0x18000000;
pub const MT_WFSYS0_PHY_START: c_uint = 0x18400000;
pub const MT_WFSYS1_PHY_START: c_uint = 0x18800000;
pub const MT_WFSYS1_PHY_END: c_uint = 0x18bfffff;
pub const MT_CBTOP1_PHY_START: c_uint = 0x70000000;

pub const MT_CBTOP2_PHY_START: c_uint = 0xf0000000;
pub const MT_INFRA_MCU_START: c_uint = 0x7c000000;

// CONN INFRA CFG
pub const MT_CONN_INFRA_BASE: c_uint = 0x18001000;

// AFE

// ADIE
pub const MT_ADIE_CHIP_ID: c_uint = 0x02c;

pub const MT_ADIE_RG_TOP_THADC_BG: c_uint = 0x034;

pub const MT_ADIE_RG_TOP_THADC: c_uint = 0x038;

pub const MT_AFE_RG_ENCAL_WBTAC_IF_SW: c_uint = 0x070;
pub const MT_ADIE_EFUSE_RDATA0: c_uint = 0x130;
pub const MT_ADIE_EFUSE2_CTRL: c_uint = 0x148;

pub const MT_ADIE_EFUSE_CFG: c_uint = 0x144;

pub const MT_ADIE_THADC_ANALOG: c_uint = 0x3a6;
pub const MT_ADIE_THADC_SLOP: c_uint = 0x3a7;

pub const MT_ADIE_7975_XTAL_CAL: c_uint = 0x3a1;

pub const MT_ADIE_7975_XO_TRIM2: c_uint = 0x3a2;
pub const MT_ADIE_7975_XO_TRIM3: c_uint = 0x3a3;
pub const MT_ADIE_7975_XO_TRIM4: c_uint = 0x3a4;
pub const MT_ADIE_7975_XTAL_EN: c_uint = 0x3a5;
pub const MT_ADIE_XO_TRIM_FLOW: c_uint = 0x3ac;
pub const MT_ADIE_XTAL_AXM_80M_OSC: c_uint = 0x390;
pub const MT_ADIE_XTAL_AXM_40M_OSC: c_uint = 0x391;
pub const MT_ADIE_XTAL_TRIM1_80M_OSC: c_uint = 0x398;
pub const MT_ADIE_XTAL_TRIM1_40M_OSC: c_uint = 0x399;
pub const MT_ADIE_WRI_CK_SEL: c_uint = 0x4ac;
pub const MT_ADIE_RG_STRAP_PIN_IN: c_uint = 0x4fc;
pub const MT_ADIE_XTAL_C1: c_uint = 0x654;
pub const MT_ADIE_XTAL_C2: c_uint = 0x658;
pub const MT_ADIE_RG_XO_01: c_uint = 0x65c;
pub const MT_ADIE_RG_XO_03: c_uint = 0x664;
pub const MT_ADIE_CLK_EN: c_uint = 0xa00;
pub const MT_ADIE_7975_XTAL: c_uint = 0xa18;

pub const MT_ADIE_7975_COCLK: c_uint = 0xa1c;
pub const MT_ADIE_7975_XO_2: c_uint = 0xa84;

pub const MT_ADIE_7975_XO_CTRL2: c_uint = 0xa94;

pub const MT_ADIE_7975_XO_CTRL6: c_uint = 0xaa4;

// TOP SPI

// CONN INFRA CKGEN
pub const MT_INFRA_CKGEN_BASE: c_uint = 0x18009000;

// CONN INFRA BUS
pub const MT_INFRA_BUS_BASE: c_uint = 0x1800e000;

// CONN_INFRA_SKU
pub const MT_CONNINFRA_SKU_DEC_ADDR: c_uint = 0x18050000;

// FW MODE SYNC

pub const MT_SWDEF_NORMAL_MODE: c_int = 0;
pub const MT_SWDEF_ICAP_MODE: c_int = 1;
pub const MT_SWDEF_SPECTRUM_MODE: c_int = 2;

pub const MT_DIC_CMD_REG_BASE: c_uint = 0x41f000;

pub const MT_CPU_UTIL_BASE: c_uint = 0x41f030;

// LED
pub const MT_LED_TOP_BASE: c_uint = 0x18013000;

pub const MT_LED_GPIO_MUX0: c_uint = 0x70005050 /* GPIO 1 and GPIO 2 */;
pub const MT_LED_GPIO_MUX1: c_uint = 0x70005054 /* GPIO 14 and 15 */;
pub const MT_LED_GPIO_MUX2: c_uint = 0x70005058 /* GPIO 18 */;
pub const MT_LED_GPIO_MUX3: c_uint = 0x7000505c /* GPIO 26 */;
// MT TOP
pub const MT_TOP_BASE: c_uint = 0x18060000;

// SEMA
pub const MT_SEMA_BASE: c_uint = 0x18070000;

// MCU BUS
pub const MT_MCU_BUS_BASE: c_uint = 0x18400000;

// TOP CFG
pub const MT_TOP_CFG_BASE: c_uint = 0x184b0000;

// TOP CFG ON
pub const MT_TOP_CFG_ON_BASE: c_uint = 0x184c1000;

// SLP CTRL
pub const MT_SLP_BASE: c_uint = 0x184c3000;

// MCU BUS DBG
pub const MT_MCU_BUS_DBG_BASE: c_uint = 0x18500000;

pub const MT_HW_BOUND: c_uint = 0x70010020;
pub const MT_HW_REV: c_uint = 0x70010204;
pub const MT_WF_SUBSYS_RST: c_uint = 0x70002600;
// PCIE MAC
pub const MT_PCIE_MAC_BASE: c_uint = 0x74030000;

pub const MT_PCIE1_MAC_INT_ENABLE: c_uint = 0x74020188;
pub const MT_PCIE1_MAC_INT_ENABLE_MT7916: c_uint = 0x74090188;
pub const MT_WM_MCU_PC: c_uint = 0x7c060204;
pub const MT_WA_MCU_PC: c_uint = 0x7c06020c;
// PP TOP
pub const MT_WF_PP_TOP_BASE: c_uint = 0x820cc000;

pub const MT_WF_IRPI_BASE: c_uint = 0x83000000;

// PHY
pub const MT_WF_PHY_BASE: c_uint = 0x83080000;

pub const MT_MCU_WM_CIRQ_BASE: c_uint = 0x89010000;

