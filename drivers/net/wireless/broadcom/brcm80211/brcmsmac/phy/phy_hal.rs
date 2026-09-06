//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/phy/phy_hal.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//
// phy_hal.h:  functionality exported from the phy to higher layers
//

pub const IDCODE_VER_MASK: c_uint = 0x0000000f;
pub const IDCODE_VER_SHIFT: c_int = 0;
pub const IDCODE_MFG_MASK: c_uint = 0x00000fff;
pub const IDCODE_MFG_SHIFT: c_int = 0;
pub const IDCODE_ID_MASK: c_uint = 0x0ffff000;
pub const IDCODE_ID_SHIFT: c_int = 12;
pub const IDCODE_REV_MASK: c_uint = 0xf0000000;
pub const IDCODE_REV_SHIFT: c_int = 28;
pub const NORADIO_ID: c_uint = 0xe4f5;
pub const NORADIO_IDCODE: c_uint = 0x4e4f5246;
pub const BCM2055_ID: c_uint = 0x2055;
pub const BCM2055_IDCODE: c_uint = 0x02055000;
pub const BCM2055A0_IDCODE: c_uint = 0x1205517f;
pub const BCM2056_ID: c_uint = 0x2056;
pub const BCM2056_IDCODE: c_uint = 0x02056000;
pub const BCM2056A0_IDCODE: c_uint = 0x1205617f;
pub const BCM2057_ID: c_uint = 0x2057;
pub const BCM2057_IDCODE: c_uint = 0x02057000;
pub const BCM2057A0_IDCODE: c_uint = 0x1205717f;
pub const BCM2064_ID: c_uint = 0x2064;
pub const BCM2064_IDCODE: c_uint = 0x02064000;
pub const BCM2064A0_IDCODE: c_uint = 0x0206417f;

pub const PHY_PERICAL_DRIVERUP: c_int = 1;
pub const PHY_PERICAL_WATCHDOG: c_int = 2;
pub const PHY_PERICAL_PHYINIT: c_int = 3;
pub const PHY_PERICAL_JOIN_BSS: c_int = 4;
pub const PHY_PERICAL_START_IBSS: c_int = 5;
pub const PHY_PERICAL_UP_BSS: c_int = 6;
pub const PHY_PERICAL_CHAN: c_int = 7;
pub const PHY_FULLCAL: c_int = 8;
pub const PHY_PERICAL_DISABLE: c_int = 0;
pub const PHY_PERICAL_SPHASE: c_int = 1;
pub const PHY_PERICAL_MPHASE: c_int = 2;
pub const PHY_PERICAL_MANUAL: c_int = 3;
pub const PHY_HOLD_FOR_ASSOC: c_int = 1;
pub const PHY_HOLD_FOR_SCAN: c_int = 2;
pub const PHY_HOLD_FOR_RM: c_int = 4;
pub const PHY_HOLD_FOR_PLT: c_int = 8;
pub const PHY_HOLD_FOR_MUTE: c_int = 16;
pub const PHY_HOLD_FOR_NOT_ASSOC: c_uint = 0x20;
pub const PHY_MUTE_FOR_PREISM: c_int = 1;
pub const PHY_MUTE_ALL: c_uint = 0xffffffff;

pub const PHY_MODE_CAL: c_uint = 0x0002;
pub const PHY_MODE_NOISEM: c_uint = 0x0004;
pub const BRCMS_TXPWR_DB_FACTOR: c_int = 4;
// a large TX Power as an init value to factor out of min() calculations,
// keep low enough to fit in an s8, units are .25 dBm
//

pub const BRCMS_NUM_RATES_CCK: c_int = 4;
pub const BRCMS_NUM_RATES_OFDM: c_int = 8;
pub const BRCMS_NUM_RATES_MCS_1_STREAM: c_int = 8;
pub const BRCMS_NUM_RATES_MCS_2_STREAM: c_int = 8;
pub const BRCMS_NUM_RATES_MCS_3_STREAM: c_int = 8;
pub const BRCMS_NUM_RATES_MCS_4_STREAM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpwr_limits {
    pub cck: [u8; BRCMS_NUM_RATES_CCK],
    pub ofdm: [u8; BRCMS_NUM_RATES_OFDM],
    pub ofdm_cdd: [u8; BRCMS_NUM_RATES_OFDM],
    pub ofdm_40_siso: [u8; BRCMS_NUM_RATES_OFDM],
    pub ofdm_40_cdd: [u8; BRCMS_NUM_RATES_OFDM],
    pub mcs_20_siso: [u8; BRCMS_NUM_RATES_MCS_1_STREAM],
    pub mcs_20_cdd: [u8; BRCMS_NUM_RATES_MCS_1_STREAM],
    pub mcs_20_stbc: [u8; BRCMS_NUM_RATES_MCS_1_STREAM],
    pub mcs_20_mimo: [u8; BRCMS_NUM_RATES_MCS_2_STREAM],
    pub mcs_40_siso: [u8; BRCMS_NUM_RATES_MCS_1_STREAM],
    pub mcs_40_cdd: [u8; BRCMS_NUM_RATES_MCS_1_STREAM],
    pub mcs_40_stbc: [u8; BRCMS_NUM_RATES_MCS_1_STREAM],
    pub mcs_40_mimo: [u8; BRCMS_NUM_RATES_MCS_2_STREAM],
    pub mcs32: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_power {
    pub flags: u32,
    pub /: *mut *mut u16 chanspec; / txpwr report for this channel,
    pub /: *mut *mut u16 local_chanspec; / channel on which we are associated,
    pub /: *mut *mut u8 local_max; / local max according to the AP,
    pub /: *mut *mut u8 local_constraint; / local constraint according to the AP,
    pub /: *mut *mut s8 antgain[2]; / Ant gain for each band - from SROM,
    pub /: *mut *mut u8 rf_cores; / count of RF Cores being reported,
    pub /: *mut *mut u8 est_Pout[4]; / Latest tx power out estimate per RF chain,
    pub chain: *mut *mut u8 est_Pout_act[4]; / Latest tx power out estimate per RF,
// without adjustment
    pub /: *mut *mut u8 est_Pout_cck; / Latest CCK tx power out estimate,
    pub /: *mut *mut u8 tx_power_max[4]; / Maximum target power among all rates,
// Index of the rate with the max target power
    pub tx_power_max_rate_ind: [u8; 4],
// User limit
    pub user_limit: [u8; WL_TX_POWER_RATES],
// Regulatory power limit
    pub reg_limit: [u8; WL_TX_POWER_RATES],
// Max power board can support (SROM)
    pub board_limit: [u8; WL_TX_POWER_RATES],
// Latest target power
    pub target: [u8; WL_TX_POWER_RATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_chanvec {
    pub NBBY]: u8 vec[MAXCHANNEL /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_phy_params {
    pub sih: *mut si_pub,
    pub physhim: *mut phy_shim_info,
    pub unit: c_uint,
    pub corerev: c_uint,
    pub vid: u16,
    pub did: u16,
    pub chip: c_uint,
    pub chiprev: c_uint,
    pub chippkg: c_uint,
    pub sromrev: c_uint,
    pub boardtype: c_uint,
    pub boardrev: c_uint,
    pub boardflags: u32,
    pub boardflags2: u32,
}

extern "C" {
    pub fn wlc_phy_detach(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_get_encore(pih: *mut brcms_phy_pub) -> bool;
}
extern "C" {
    pub fn wlc_phy_get_coreflags(pih: *mut brcms_phy_pub) -> u32;
}
extern "C" {
    pub fn wlc_phy_hw_clk_state_upd(ppi: *mut brcms_phy_pub, newstate: bool);
}
extern "C" {
    pub fn wlc_phy_hw_state_upd(ppi: *mut brcms_phy_pub, newstate: bool);
}
extern "C" {
    pub fn wlc_phy_init(ppi: *mut brcms_phy_pub, chanspec: u16);
}
extern "C" {
    pub fn wlc_phy_watchdog(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_down(ppi: *mut brcms_phy_pub) -> c_int;
}
extern "C" {
    pub fn wlc_phy_clk_bwbits(pih: *mut brcms_phy_pub) -> u32;
}
extern "C" {
    pub fn wlc_phy_cal_init(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_antsel_init(ppi: *mut brcms_phy_pub, lut_init: bool);
}
extern "C" {
    pub fn wlc_phy_chanspec_set(ppi: *mut brcms_phy_pub, chanspec: u16);
}
extern "C" {
    pub fn wlc_phy_chanspec_get(ppi: *mut brcms_phy_pub) -> u16;
}
extern "C" {
    pub fn wlc_phy_chanspec_radio_set(ppi: *mut brcms_phy_pub, newch: u16);
}
extern "C" {
    pub fn wlc_phy_bw_state_set(ppi: *mut brcms_phy_pub, bw: u16);
}
extern "C" {
    pub fn wlc_phy_rssi_compute(pih: *mut brcms_phy_pub, rxh: *mut d11rxhdr) -> c_int;
}
extern "C" {
    pub fn wlc_phy_por_inform(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_noise_sample_intr(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_bist_check_phy(ppi: *mut brcms_phy_pub) -> bool;
}
extern "C" {
    pub fn wlc_phy_switch_radio(ppi: *mut brcms_phy_pub, on: bool);
}
extern "C" {
    pub fn wlc_phy_anacore(ppi: *mut brcms_phy_pub, on: bool);
}
extern "C" {
    pub fn wlc_phy_txpower_get(ppi: *mut brcms_phy_pub, qdbm: *mut c_uint, override: *mut bool) -> c_int;
}
extern "C" {
    pub fn wlc_phy_txpower_set(ppi: *mut brcms_phy_pub, qdbm: c_uint, override: bool) -> c_int;
}
extern "C" {
    pub fn wlc_phy_txpower_hw_ctrl_get(ppi: *mut brcms_phy_pub) -> bool;
}
extern "C" {
    pub fn wlc_phy_stf_chain_init(pih: *mut brcms_phy_pub, txchain: u8, rxchain: u8);
}
extern "C" {
    pub fn wlc_phy_stf_chain_set(pih: *mut brcms_phy_pub, txchain: u8, rxchain: u8);
}
extern "C" {
    pub fn wlc_phy_stf_chain_active_get(pih: *mut brcms_phy_pub) -> u8;
}
extern "C" {
    pub fn wlc_phy_ldpc_override_set(ppi: *mut brcms_phy_pub, val: bool);
}
extern "C" {
    pub fn wlc_phy_cal_perical(ppi: *mut brcms_phy_pub, reason: u8);
}
extern "C" {
    pub fn wlc_phy_cal_papd_recal(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_ant_rxdiv_set(ppi: *mut brcms_phy_pub, val: u8);
}
extern "C" {
    pub fn wlc_phy_hold_upd(ppi: *mut brcms_phy_pub, id: u32, val: bool);
}
extern "C" {
    pub fn wlc_phy_mute_upd(ppi: *mut brcms_phy_pub, val: bool, flags: u32);
}
extern "C" {
    pub fn wlc_phy_antsel_type_set(ppi: *mut brcms_phy_pub, antsel_type: u8);
}
extern "C" {
    pub fn wlc_phy_initcal_enable(pih: *mut brcms_phy_pub, initcal: bool);
}
extern "C" {
    pub fn wlc_phy_ofdm_rateset_war(pih: *mut brcms_phy_pub, war: bool);
}
extern "C" {
    pub fn wlc_phy_bf_preempt_enable(pih: *mut brcms_phy_pub, bf_preempt: bool);
}
extern "C" {
    pub fn wlc_phy_machwcap_set(ppi: *mut brcms_phy_pub, machwcap: u32);
}
extern "C" {
    pub fn wlc_phy_get_tx_power_offset(ppi: *mut brcms_phy_pub, tbl_offset: u8) -> i8;
}
