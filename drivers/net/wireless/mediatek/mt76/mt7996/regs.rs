//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7996/regs.h
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
//
// Copyright (C) 2022 MediaTek Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __map {
    pub phys: u32,
    pub mapped: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __base {
    pub band_base: [u32; __MT_MAX_BAND],
}

// used to differentiate between generations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_reg_desc {
    pub base: *const __base,
    pub offs_rev: *const u32,
    pub map: *const __map,
    pub map_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum base_rev {
    WF_AGG_BASE,
    WF_ARB_BASE,
    WF_TMAC_BASE,
    WF_RMAC_BASE,
    WF_DMA_BASE,
    WF_WTBLOFF_BASE,
    WF_ETBF_BASE,
    WF_LPON_BASE,
    WF_MIB_BASE,
    WF_RATE_BASE,
    __MT_REG_BASE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum offs_rev {
    MIB_RVSR0,
    MIB_RVSR1,
    MIB_BTSCR5,
    MIB_BTSCR6,
    MIB_RSCR1,
    MIB_RSCR27,
    MIB_RSCR28,
    MIB_RSCR29,
    MIB_RSCR30,
    MIB_RSCR31,
    MIB_RSCR33,
    MIB_RSCR35,
    MIB_RSCR36,
    MIB_BSCR0,
    MIB_BSCR1,
    MIB_BSCR2,
    MIB_BSCR3,
    MIB_BSCR4,
    MIB_BSCR5,
    MIB_BSCR6,
    MIB_BSCR7,
    MIB_BSCR17,
    MIB_TRDR1,
    MIB_TSCR0,
    MIB_TSCR1,
    MIB_TSCR2,
    MIB_TSCR3,
    MIB_TSCR4,
    MIB_TSCR5,
    MIB_TSCR6,
    MIB_TSCR7,
    HIF_REMAP_L1,
    HIF_REMAP_BASE_L1,
    HIF_REMAP_L2,
    HIF_REMAP_BASE_L2,
    CBTOP1_PHY_END,
    INFRA_MCU_END,
    WTBLON_WDUCR,
    WTBL_UPDATE,
    WTBL_ITCR,
    WTBL_ITCR0,
    WTBL_ITCR1,
    __MT_OFFS_MAX,
}

// RRO TOP
pub const MT_RRO_TOP_BASE: c_uint = 0xA000;

pub const MT_MCU_INT_EVENT: c_uint = 0x2108;

// PLE
pub const MT_PLE_BASE: c_uint = 0x820c0000;

// WF MDP TOP
pub const MT_MDP_BASE: c_uint = 0x820cc000;

// TMAC: band 0(0x820e4000), band 1(0x820f4000), band 2(0x830e4000)

// WF DMA TOP: band 0(0x820e7000), band 1(0x820f7000), band 2(0x830e7000)

// WTBLOFF TOP: band 0(0x820e9000), band 1(0x820f9000), band 2(0x830e9000)

// ETBF: band 0(0x820ea000), band 1(0x820fa000), band 2(0x830ea000)

// LPON: band 0(0x820eb000), band 1(0x820fb000), band 2(0x830eb000)

// MIB: band 0(0x820ed000), band 1(0x820fd000), band 2(0x830ed000)
// These counters are (mostly?) clear-on-read.  So, some should not
// be read at all in case firmware is already reading them.  These
// are commented with 'DNR' below. The DNR stats will be read by querying
// the firmware API for the appropriate message.  For counters the driver
// does read, the driver should accumulate the counters.
//

// rx mpdu counter, full 32 bits

// tx ampdu cnt, full 32 bits

// counts all mpdus in ampdu, regardless of success

// counts all successfully tx'd mpdus in ampdu

// rx ampdu count, 32-bit

// rx ampdu bytes count, 32-bit

// rx ampdu valid subframe count

// rx ampdu valid subframe bytes count, 32bits

// remaining windows protected stats

// rx blockack count, 32 bits

// UMIB
pub const MT_WF_UMIB_BASE: c_uint = 0x820cd000;

// WTBLON TOP
pub const MT_WTBLON_TOP_BASE: c_uint = 0x820d4000;

// WTBL
pub const MT_WTBL_BASE: c_uint = 0x820d8000;

// AGG: band 0(0x820e2000), band 1(0x820f2000), band 2(0x830e2000)

// ARB: band 0(0x820e3000), band 1(0x820f3000), band 2(0x830e3000)

// RMAC: band 0(0x820e5000), band 1(0x820f5000), band 2(0x830e5000),

// RATE: band 0(0x820ee000), band 1(0x820fe000), band 2(0x830ee000)

// WFDMA0
pub const MT_WFDMA0_BASE: c_uint = 0xd4000;

// WFDMA1
pub const MT_WFDMA1_BASE: c_uint = 0xd5000;
// WFDMA CSR
pub const MT_WFDMA_EXT_CSR_BASE: c_uint = 0xd7000;

pub const MT_PCIE_RECOG_ID: c_uint = 0xd7090;

// WFDMA0 PCIE1
pub const MT_WFDMA0_PCIE1_BASE: c_uint = 0xd8000;

// WFDMA COMMON

// l1/l2 remap
pub const CONN_BUS_CR_VON_BASE: c_uint = 0x155000;

// for mt7990 only
pub const MT_HIF_REMAP_CBTOP: c_uint = 0x1f6554;

pub const MT_HIF_REMAP_BASE_CBTOP: c_uint = 0x1c0000;
pub const MT_INFRA_BASE: c_uint = 0x18000000;
pub const MT_WFSYS0_PHY_START: c_uint = 0x18400000;
pub const MT_WFSYS1_PHY_START: c_uint = 0x18800000;
pub const MT_WFSYS1_PHY_END: c_uint = 0x18bfffff;
pub const MT_CBTOP1_PHY_START: c_uint = 0x70000000;

pub const MT_CBTOP2_PHY_START: c_uint = 0xf0000000;
pub const MT_INFRA_MCU_START: c_uint = 0x7c000000;

// FW MODE SYNC
pub const MT_FW_ASSERT_CNT: c_uint = 0x02208274;
pub const MT_FW_DUMP_STATE: c_uint = 0x02209e90;
pub const MT_SWDEF_BASE: c_uint = 0x00401400;

pub const MT_SWDEF_NORMAL_MODE: c_int = 0;

// LED
pub const MT_LED_TOP_BASE: c_uint = 0x18013000;

// CONN DBG
pub const MT_CONN_DBG_CTL_BASE: c_uint = 0x18023000;

pub const MT_LED_GPIO_MUX2: c_uint = 0x70005058 /* GPIO 18 */;
pub const MT_LED_GPIO_MUX3: c_uint = 0x7000505C /* GPIO 26 */;

// MT TOP
pub const MT_TOP_BASE: c_uint = 0xe0000;

// ADIE

pub const MT_PAD_GPIO: c_uint = 0x700056f0;

// for mt7992

pub const MT_HW_REV: c_uint = 0x70010204;
pub const MT_HW_REV1: c_uint = 0x8a00;
pub const MT_WF_L05_RST: c_uint = 0x70028550;

pub const MT_WF_SUBSYS_RST: c_uint = 0x70028600;

// PCIE MAC
pub const MT_PCIE_MAC_BASE: c_uint = 0x74030000;

pub const MT_PCIE1_MAC_BASE: c_uint = 0x74090000;

// PHYRX CSD
pub const MT_WF_PHYRX_CSD_BASE: c_uint = 0x83000000;

// PHYRX CTRL
pub const MT_WF_PHYRX_BAND_BASE: c_uint = 0x83080000;

// PHYRX CSD BAND

// CONN MCU EXCP CON
pub const MT_MCU_WM_EXCP_BASE: c_uint = 0x89050000;

// CONN AFE CTL CON
pub const MT_AFE_CTL_BASE: c_uint = 0x18043000;

