//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/phy/phy_lcn.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_phy_lcnphy {
    pub lcnphy_txrf_sp_9_override: c_int,
    pub lcnphy_full_cal_channel: u8,
    pub lcnphy_cal_counter: u8,
    pub lcnphy_cal_temper: u16,
    pub lcnphy_recal: bool,
    pub lcnphy_rc_cap: u8,
    pub lcnphy_mcs20_po: u32,
    pub lcnphy_tr_isolation_mid: u8,
    pub lcnphy_tr_isolation_low: u8,
    pub lcnphy_tr_isolation_hi: u8,
    pub lcnphy_bx_arch: u8,
    pub lcnphy_rx_power_offset: u8,
    pub lcnphy_rssi_vf: u8,
    pub lcnphy_rssi_vc: u8,
    pub lcnphy_rssi_gs: u8,
    pub lcnphy_tssi_val: u8,
    pub lcnphy_rssi_vf_lowtemp: u8,
    pub lcnphy_rssi_vc_lowtemp: u8,
    pub lcnphy_rssi_gs_lowtemp: u8,
    pub lcnphy_rssi_vf_hightemp: u8,
    pub lcnphy_rssi_vc_hightemp: u8,
    pub lcnphy_rssi_gs_hightemp: u8,
    pub lcnphy_pa0b0: i16,
    pub lcnphy_pa0b1: i16,
    pub lcnphy_pa0b2: i16,
    pub lcnphy_rawtempsense: u16,
    pub lcnphy_measPower: u8,
    pub lcnphy_tempsense_slope: u8,
    pub lcnphy_freqoffset_corr: u8,
    pub lcnphy_tempsense_option: u8,
    pub lcnphy_tempcorrx: u8,
    pub lcnphy_iqcal_swp_dis: bool,
    pub lcnphy_hw_iqcal_en: bool,
    pub lcnphy_bandedge_corr: c_uint,
    pub lcnphy_spurmod: bool,
    pub lcnphy_tssi_tx_cnt: u16,
    pub lcnphy_tssi_idx: u16,
    pub lcnphy_tssi_npt: u16,
    pub lcnphy_target_tx_freq: u16,
    pub lcnphy_tx_power_idx_override: i8,
    pub lcnphy_noise_samples: u16,
    pub lcnphy_papdRxGnIdx: u32,
    pub lcnphy_papd_rxGnCtrl_init: u32,
    pub lcnphy_gain_idx_14_lowword: u32,
    pub lcnphy_gain_idx_14_hiword: u32,
    pub lcnphy_gain_idx_27_lowword: u32,
    pub lcnphy_gain_idx_27_hiword: u32,
    pub lcnphy_ofdmgainidxtableoffset: i16,
    pub lcnphy_dsssgainidxtableoffset: i16,
    pub lcnphy_tr_R_gain_val: u32,
    pub lcnphy_tr_T_gain_val: u32,
    pub lcnphy_input_pwr_offset_db: i8,
    pub lcnphy_Med_Low_Gain_db: u16,
    pub lcnphy_Very_Low_Gain_db: u16,
    pub lcnphy_lastsensed_temperature: i8,
    pub lcnphy_pkteng_rssi_slope: i8,
    pub lcnphy_saved_tx_user_target: [u8; TXP_NUM_RATES],
    pub lcnphy_volt_winner: u8,
    pub lcnphy_volt_low: u8,
    pub lcnphy_54_48_36_24mbps_backoff: u8,
    pub lcnphy_11n_backoff: u8,
    pub lcnphy_lowerofdm: u8,
    pub lcnphy_cck: u8,
    pub lcnphy_psat_2pt3_detected: u8,
    pub lcnphy_lowest_Re_div_Im: i32,
    pub lcnphy_final_papd_cal_idx: i8,
    pub lcnphy_extstxctrl4: u16,
    pub lcnphy_extstxctrl0: u16,
    pub lcnphy_extstxctrl1: u16,
    pub lcnphy_cck_dig_filt_type: i16,
    pub lcnphy_ofdm_dig_filt_type: i16,
    pub lcnphy_cal_results: lcnphy_cal_results,
    pub lcnphy_psat_pwr: u8,
    pub lcnphy_psat_indx: u8,
    pub lcnphy_min_phase: i32,
    pub lcnphy_final_idx: u8,
    pub lcnphy_start_idx: u8,
    pub lcnphy_current_index: u8,
    pub lcnphy_logen_buf_1: u16,
    pub lcnphy_local_ovr_2: u16,
    pub lcnphy_local_oval_6: u16,
    pub lcnphy_local_oval_5: u16,
    pub lcnphy_logen_mixer_1: u16,
    pub lcnphy_aci_stat: u8,
    pub lcnphy_aci_start_time: c_uint,
    pub lcnphy_tx_power_offset: [i8; TXP_NUM_RATES],
}
