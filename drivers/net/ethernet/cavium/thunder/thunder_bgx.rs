//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/thunder/thunder_bgx.h
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
// Copyright (C) 2015 Cavium, Inc.
//
// PCI device ID
pub const PCI_DEVICE_ID_THUNDER_BGX: c_uint = 0xA026;
pub const PCI_DEVICE_ID_THUNDER_RGX: c_uint = 0xA054;
// Subsystem device IDs
pub const PCI_SUBSYS_DEVID_88XX_BGX: c_uint = 0xA126;
pub const PCI_SUBSYS_DEVID_81XX_BGX: c_uint = 0xA226;
pub const PCI_SUBSYS_DEVID_81XX_RGX: c_uint = 0xA254;
pub const PCI_SUBSYS_DEVID_83XX_BGX: c_uint = 0xA326;

pub const MAX_BGX_PER_CN88XX: c_int = 2;

pub const MAX_BGX_PER_CN83XX: c_int = 4;
pub const MAX_LMAC_PER_BGX: c_int = 4;
pub const MAX_BGX_CHANS_PER_LMAC: c_int = 16;
pub const MAX_DMAC_PER_LMAC: c_int = 8;
pub const MAX_FRAME_SIZE: c_int = 9216;
pub const DEFAULT_PAUSE_TIME: c_uint = 0xFFFF;
pub const BGX_ID_MASK: c_uint = 0x3;
pub const LMAC_ID_MASK: c_uint = 0x3;
pub const MAX_DMAC_PER_LMAC_TNS_BYPASS_MODE: c_int = 2;
// Registers
pub const BGX_CMRX_CFG: c_uint = 0x00;

pub const BGX_CMR_GLOBAL_CFG: c_uint = 0x08;

pub const BGX_CMRX_RX_ID_MAP: c_uint = 0x60;
pub const BGX_CMRX_RX_STAT0: c_uint = 0x70;
pub const BGX_CMRX_RX_STAT1: c_uint = 0x78;
pub const BGX_CMRX_RX_STAT2: c_uint = 0x80;
pub const BGX_CMRX_RX_STAT3: c_uint = 0x88;
pub const BGX_CMRX_RX_STAT4: c_uint = 0x90;
pub const BGX_CMRX_RX_STAT5: c_uint = 0x98;
pub const BGX_CMRX_RX_STAT6: c_uint = 0xA0;
pub const BGX_CMRX_RX_STAT7: c_uint = 0xA8;
pub const BGX_CMRX_RX_STAT8: c_uint = 0xB0;
pub const BGX_CMRX_RX_STAT9: c_uint = 0xB8;
pub const BGX_CMRX_RX_STAT10: c_uint = 0xC0;
pub const BGX_CMRX_RX_BP_DROP: c_uint = 0xC8;
pub const BGX_CMRX_RX_DMAC_CTL: c_uint = 0x0E8;
pub const BGX_CMRX_RX_FIFO_LEN: c_uint = 0x108;
pub const BGX_CMR_RX_DMACX_CAM: c_uint = 0x200;

pub const RX_DMAC_COUNT: c_int = 32;
pub const BGX_CMR_RX_STEERING: c_uint = 0x300;
pub const RX_TRAFFIC_STEER_RULE_COUNT: c_int = 8;
pub const BGX_CMR_CHAN_MSK_AND: c_uint = 0x450;
pub const BGX_CMR_BIST_STATUS: c_uint = 0x460;
pub const BGX_CMR_RX_LMACS: c_uint = 0x468;
pub const BGX_CMRX_TX_FIFO_LEN: c_uint = 0x518;
pub const BGX_CMRX_TX_STAT0: c_uint = 0x600;
pub const BGX_CMRX_TX_STAT1: c_uint = 0x608;
pub const BGX_CMRX_TX_STAT2: c_uint = 0x610;
pub const BGX_CMRX_TX_STAT3: c_uint = 0x618;
pub const BGX_CMRX_TX_STAT4: c_uint = 0x620;
pub const BGX_CMRX_TX_STAT5: c_uint = 0x628;
pub const BGX_CMRX_TX_STAT6: c_uint = 0x630;
pub const BGX_CMRX_TX_STAT7: c_uint = 0x638;
pub const BGX_CMRX_TX_STAT8: c_uint = 0x640;
pub const BGX_CMRX_TX_STAT9: c_uint = 0x648;
pub const BGX_CMRX_TX_STAT10: c_uint = 0x650;
pub const BGX_CMRX_TX_STAT11: c_uint = 0x658;
pub const BGX_CMRX_TX_STAT12: c_uint = 0x660;
pub const BGX_CMRX_TX_STAT13: c_uint = 0x668;
pub const BGX_CMRX_TX_STAT14: c_uint = 0x670;
pub const BGX_CMRX_TX_STAT15: c_uint = 0x678;
pub const BGX_CMRX_TX_STAT16: c_uint = 0x680;
pub const BGX_CMRX_TX_STAT17: c_uint = 0x688;
pub const BGX_CMR_TX_LMACS: c_uint = 0x1000;
pub const BGX_SPUX_CONTROL1: c_uint = 0x10000;

pub const BGX_SPUX_STATUS1: c_uint = 0x10008;

pub const BGX_SPUX_STATUS2: c_uint = 0x10020;

pub const BGX_SPUX_BX_STATUS: c_uint = 0x10028;

pub const BGX_SPUX_BR_STATUS1: c_uint = 0x10030;

pub const BGX_SPUX_BR_PMD_CRTL: c_uint = 0x10068;

pub const BGX_SPUX_BR_PMD_LP_CUP: c_uint = 0x10078;
pub const BGX_SPUX_BR_PMD_LD_CUP: c_uint = 0x10088;
pub const BGX_SPUX_BR_PMD_LD_REP: c_uint = 0x10090;
pub const BGX_SPUX_FEC_CONTROL: c_uint = 0x100A0;

pub const BGX_SPUX_AN_CONTROL: c_uint = 0x100C8;

pub const BGX_SPUX_AN_ADV: c_uint = 0x100D8;
pub const BGX_SPUX_MISC_CONTROL: c_uint = 0x10218;

pub const BGX_SPUX_INT: c_uint = 0x10220	/* +(0..3) << 20 */;
pub const BGX_SPUX_INT_W1S: c_uint = 0x10228;
pub const BGX_SPUX_INT_ENA_W1C: c_uint = 0x10230;
pub const BGX_SPUX_INT_ENA_W1S: c_uint = 0x10238;
pub const BGX_SPU_DBG_CONTROL: c_uint = 0x10300;

pub const BGX_SMUX_RX_INT: c_uint = 0x20000;
pub const BGX_SMUX_RX_FRM_CTL: c_uint = 0x20020;

pub const BGX_SMUX_RX_JABBER: c_uint = 0x20030;
pub const BGX_SMUX_RX_CTL: c_uint = 0x20048;

pub const BGX_SMUX_TX_APPEND: c_uint = 0x20100;

pub const BGX_SMUX_TX_PAUSE_PKT_TIME: c_uint = 0x20110;
pub const BGX_SMUX_TX_MIN_PKT: c_uint = 0x20118;
pub const BGX_SMUX_TX_PAUSE_PKT_INTERVAL: c_uint = 0x20120;
pub const BGX_SMUX_TX_PAUSE_ZERO: c_uint = 0x20138;
pub const BGX_SMUX_TX_INT: c_uint = 0x20140;
pub const BGX_SMUX_TX_CTL: c_uint = 0x20178;

pub const BGX_SMUX_TX_THRESH: c_uint = 0x20180;
pub const BGX_SMUX_CTL: c_uint = 0x20200;

pub const BGX_SMUX_CBFC_CTL: c_uint = 0x20218;

pub const BGX_GMP_PCS_MRX_CTL: c_uint = 0x30000;

pub const BGX_GMP_PCS_MRX_STATUS: c_uint = 0x30008;

pub const BGX_GMP_PCS_ANX_ADV: c_uint = 0x30010;
pub const BGX_GMP_PCS_ANX_AN_RESULTS: c_uint = 0x30020;
pub const BGX_GMP_PCS_LINKX_TIMER: c_uint = 0x30040;
pub const PCS_LINKX_TIMER_COUNT: c_uint = 0x1E84;
pub const BGX_GMP_PCS_SGM_AN_ADV: c_uint = 0x30068;
pub const BGX_GMP_PCS_MISCX_CTL: c_uint = 0x30078;

pub const PCS_MISC_CTL_SAMP_PT_MASK: c_uint = 0x7Full;
pub const BGX_GMP_GMI_PRTX_CFG: c_uint = 0x38020;

pub const BGX_GMP_GMI_RXX_FRM_CTL: c_uint = 0x38028;
pub const BGX_GMP_GMI_RXX_JABBER: c_uint = 0x38038;
pub const BGX_GMP_GMI_TXX_THRESH: c_uint = 0x38210;
pub const BGX_GMP_GMI_TXX_APPEND: c_uint = 0x38218;
pub const BGX_GMP_GMI_TXX_SLOT: c_uint = 0x38220;
pub const BGX_GMP_GMI_TXX_BURST: c_uint = 0x38228;
pub const BGX_GMP_GMI_TXX_MIN_PKT: c_uint = 0x38240;
pub const BGX_GMP_GMI_TXX_SGMII_CTL: c_uint = 0x38300;
pub const BGX_GMP_GMI_TXX_INT: c_uint = 0x38500;
pub const BGX_GMP_GMI_TXX_INT_W1S: c_uint = 0x38508;
pub const BGX_GMP_GMI_TXX_INT_ENA_W1C: c_uint = 0x38510;
pub const BGX_GMP_GMI_TXX_INT_ENA_W1S: c_uint = 0x38518;

pub const BGX_MSIX_VEC_0_29_ADDR: c_uint = 0x400000 /* +(0..29) << 4 */;
pub const BGX_MSIX_VEC_0_29_CTL: c_uint = 0x400008;
pub const BGX_MSIX_PBA_0: c_uint = 0x4F0000;
// MSI-X interrupts
pub const BGX_MSIX_VECTORS: c_int = 30;
pub const BGX_LMAC_VEC_OFFSET: c_int = 7;
pub const BGX_MSIX_VEC_SHIFT: c_int = 4;
pub const CMRX_INT: c_int = 0;
pub const SPUX_INT: c_int = 1;
pub const SMUX_RX_INT: c_int = 2;
pub const SMUX_TX_INT: c_int = 3;
pub const GMPX_PCS_INT: c_int = 4;
pub const GMPX_GMI_RX_INT: c_int = 5;
pub const GMPX_GMI_TX_INT: c_int = 6;
pub const CMR_MEM_INT: c_int = 28;
pub const SPU_MEM_INT: c_int = 29;

extern "C" {
    pub fn bgx_set_dmac_cam_filter(node: c_int, bgx_idx: c_int, lmacid: c_int, mac: u64, vf: u8);
}
extern "C" {
    pub fn bgx_reset_xcast_mode(node: c_int, bgx_idx: c_int, lmacid: c_int, vf: u8);
}
extern "C" {
    pub fn bgx_set_xcast_mode(node: c_int, bgx_idx: c_int, lmacid: c_int, mode: u8);
}
extern "C" {
    pub fn bgx_lmac_rx_tx_enable(node: c_int, bgx_idx: c_int, lmacid: c_int, enable: bool);
}
extern "C" {
    pub fn bgx_get_map(node: c_int) -> unsigned;
}
extern "C" {
    pub fn bgx_get_lmac_count(node: c_int, bgx: c_int) -> c_int;
}
extern "C" {
    pub fn bgx_set_lmac_mac(node: c_int, bgx_idx: c_int, lmacid: c_int, mac: *const u8);
}
extern "C" {
    pub fn bgx_get_lmac_link_state(node: c_int, bgx_idx: c_int, lmacid: c_int, status: *mut c_void);
}
extern "C" {
    pub fn bgx_config_timestamping(node: c_int, bgx_idx: c_int, lmacid: c_int, enable: bool);
}
extern "C" {
    pub fn bgx_lmac_get_pfc(node: c_int, bgx_idx: c_int, lmacid: c_int, pause: *mut c_void);
}
extern "C" {
    pub fn bgx_lmac_set_pfc(node: c_int, bgx_idx: c_int, lmacid: c_int, pause: *mut c_void);
}
extern "C" {
    pub fn xcv_init_hw();
}
extern "C" {
    pub fn xcv_setup_link(link_up: bool, link_speed: c_int);
}
extern "C" {
    pub fn bgx_get_rx_stats(node: c_int, bgx_idx: c_int, lmac: c_int, idx: c_int) -> u64;
}
extern "C" {
    pub fn bgx_get_tx_stats(node: c_int, bgx_idx: c_int, lmac: c_int, idx: c_int) -> u64;
}
pub const BGX_RX_STATS_COUNT: c_int = 11;
pub const BGX_TX_STATS_COUNT: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bgx_stats {
    pub rx_stats: [u64; BGX_RX_STATS_COUNT],
    pub tx_stats: [u64; BGX_TX_STATS_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LMAC_TYPE {
    BGX_MODE_SGMII = 0, /* 1 lane, 1.250 Gbaud */
    BGX_MODE_XAUI = 1,  /* 4 lanes, 3.125 Gbaud */
    BGX_MODE_DXAUI = 1, /* 4 lanes, 6.250 Gbaud */
    BGX_MODE_RXAUI = 2, /* 2 lanes, 6.250 Gbaud */
    BGX_MODE_XFI = 3,   /* 1 lane, 10.3125 Gbaud */
    BGX_MODE_XLAUI = 4, /* 4 lanes, 10.3125 Gbaud */
    BGX_MODE_10G_KR = 3,/* 1 lane, 10.3125 Gbaud */
    BGX_MODE_40G_KR = 4,/* 4 lanes, 10.3125 Gbaud */
    BGX_MODE_RGMII = 5,
    BGX_MODE_QSGMII = 6,
    BGX_MODE_INVALID = 7,
}
