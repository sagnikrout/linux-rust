//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rtw8703b.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright Fiona Klute <fiona.klute@gmx.de>

// phy status parsing

// masks for assembling LNA index from high and low bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_rx_agc_info {

    pub 7: u8 gain:,
    pub 1: u8 trsw:,

    pub 1: u8 trsw:,
    pub 7: u8 gain:,

    pub __packed: },
// This struct is called phy_status_rpt_8192cd in the vendor driver,
// there might be potential to share it with drivers for other chips
// of the same generation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_status_8703b {
    pub path_agc: [phy_rx_agc_info; 2],
    pub ch_corr: [u8; 2],
    pub cck_sig_qual_ofdm_pwdb_all: u8,
// for CCK: bits 0:4: VGA index, bits 5:7: LNA index (low)
    pub cck_agc_rpt_ofdm_cfosho_a: u8,
// for CCK: bit 7 is high bit of LNA index if long report type
    pub cck_rpt_b_ofdm_cfosho_b: u8,
    pub reserved_1: u8,
    pub noise_power_db_msb: u8,
    pub path_cfotail: [i8; 2],
    pub pcts_mask: [u8; 2],
    pub stream_rxevm: [i8; 2],
    pub path_rxsnr: [u8; 2],
    pub noise_power_db_lsb: u8,
    pub reserved_2: [u8; 3],
    pub stream_csi: [u8; 2],
    pub stream_target_csi: [u8; 2],
    pub sig_evm: i8,
    pub reserved_3: u8,

    pub 1: u8 antsel_rx_keep_2:,
    pub 1: u8 sgi_en:,
    pub 2: u8 rxsc:,
    pub 1: u8 idle_long:,
    pub 1: u8 r_ant_train_en:,
    pub 1: u8 ant_sel_b:,
    pub 1: u8 ant_sel:,

    pub 1: u8 ant_sel:,
    pub 1: u8 ant_sel_b:,
    pub 1: u8 r_ant_train_en:,
    pub 1: u8 idle_long:,
    pub 2: u8 rxsc:,
    pub 1: u8 sgi_en:,
    pub 1: u8 antsel_rx_keep_2:,

    pub __packed: },
// Baseband registers
pub const REG_BB_PWR_SAV5_11N: c_uint = 0x0818;
// BIT(11) should be 1 for 8703B *and* 8723D, which means LNA uses 4
// bit for CCK rates in report, not 3. Vendor driver logs a warning if
// it's 0, but handles the case.
//
// Purpose of other parts of this register is unknown, 8723cs driver
// code indicates some other chips use certain bits for antenna
// diversity.
//
pub const REG_BB_AMP: c_uint = 0x0950;

// 0xaXX: 40MHz channel settings
pub const REG_CCK_TXSF2: c_uint = 0x0a24  /* CCK TX filter 2 */;
pub const REG_CCK_DBG: c_uint = 0x0a28  /* debug port */;
pub const REG_OFDM0_A_TX_AFE: c_uint = 0x0c84;
pub const REG_TXIQK_MATRIXB_LSB2_11N: c_uint = 0x0c9c;
pub const REG_OFDM0_TX_PSD_NOISE: c_uint = 0x0ce4  /* TX pseudo noise weighting */;
pub const REG_IQK_RDY: c_uint = 0x0e90  /* is != 0 when IQK is done */;
// RF registers
pub const RF_RCK1: c_uint = 0x1E;
pub const AGG_BURST_NUM: c_int = 3;

