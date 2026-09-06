//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7915/mcu.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_thermal_ctrl {
    pub ctrl_id: u8,
    pub band_idx: u8,
    pub /: *mut *mut u8 protect_type; / 1: duty admit, 2: radio off,
    pub /: *mut *mut u8 trigger_type; / 0: low, 1: high,
    pub type: } __packed,
    pub /: *mut *mut u8 duty_level; / level 0~3,
    pub duty_cycle: u8,
    pub duty: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_thermal_notify {
    pub rxd: mt76_connac2_mcu_rxd_hdr,
    pub ctrl: mt7915_mcu_thermal_ctrl,
    pub temperature: __le32,
    pub rsv: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_csa_notify {
    pub rxd: mt76_connac2_mcu_rxd_hdr,
    pub omac_idx: u8,
    pub csa_count: u8,
    pub band_idx: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_bcc_notify {
    pub rxd: mt76_connac2_mcu_rxd_hdr,
    pub band_idx: u8,
    pub omac_idx: u8,
    pub cca_count: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_ps_notify {
    pub rxd: mt76_connac2_mcu_rxd_hdr,
    pub wtbl_lower: u8,
    pub ps_bit: u8,
    pub wtbl_higher: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_rdd_report {
    pub rxd: mt76_connac2_mcu_rxd_hdr,
    pub rdd_idx: u8,
    pub long_detected: u8,
    pub constant_prf_detected: u8,
    pub staggered_prf_detected: u8,
    pub radar_type_idx: u8,
    pub periodic_pulse_num: u8,
    pub long_pulse_num: u8,
    pub hw_pulse_num: u8,
    pub out_lpn: u8,
    pub out_spn: u8,
    pub out_crpn: u8,
    pub out_crpw: u8,
    pub out_crbn: u8,
    pub out_stgpn: u8,
    pub out_stgpw: u8,
    pub rsv: u8,
    pub out_pri_const: __le32,
    pub out_pri_stg: [__le32; 3],
    pub start: __le32,
    pub pulse_width: __le16,
    pub pulse_power: __le16,
    pub mdrdy_flag: u8,
    pub rsv: [u8; 3],
    pub long_pulse: [}; 32],
    pub start: __le32,
    pub pulse_width: __le16,
    pub pulse_power: __le16,
    pub mdrdy_flag: u8,
    pub rsv: [u8; 3],
    pub periodic_pulse: [}; 32],
    pub start: __le32,
    pub pulse_width: __le16,
    pub pulse_power: __le16,
    pub sc_pass: u8,
    pub sw_reset: u8,
    pub mdrdy_flag: u8,
    pub tx_active: u8,
    pub hw_pulse: [}; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_background_chain_ctrl {
    pub /: *mut *mut u8 chan; / primary channel,
    pub /: *mut *mut u8 central_chan; / central channel,
    pub bw: u8,
    pub tx_stream: u8,
    pub rx_stream: u8,
    pub /: *mut *mut u8 monitor_chan; / monitor channel,
    pub /: *mut *mut u8 monitor_central_chan;/ monitor central channel,
    pub monitor_bw: u8,
    pub monitor_tx_stream: u8,
    pub monitor_rx_stream: u8,
    pub ScanStop: *mut *mut u8 scan_mode; / 0:,
// 1: ScanStart
// 2: ScanRunning
//
    pub /: *mut *mut u8 band_idx; / DBDC,
    pub monitor_scan_type: u8,
    pub /: *mut *mut u8 band; / 0: 2.4GHz, 1: 5GHz,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_sr_ctrl {
    pub action: u8,
    pub argnum: u8,
    pub band_idx: u8,
    pub status: u8,
    pub drop_ta_idx: u8,
    pub /: *mut *mut u8 sta_idx; / 256 sta,
    pub rsv: [u8; 2],
    pub val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_eeprom {
    pub buffer_mode: u8,
    pub format: u8,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_eeprom_info {
    pub addr: __le32,
    pub valid: __le32,
    pub data: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_phy_rx_info {
    pub category: u8,
    pub rate: u8,
    pub mode: u8,
    pub nsts: u8,
    pub gi: u8,
    pub coding: u8,
    pub stbc: u8,
    pub bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_mib {
    pub band: __le32,
    pub offs: __le32,
    pub data: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_chan_mib_offs {
// mt7915
    MIB_TX_TIME = 81,
    MIB_RX_TIME,
    MIB_OBSS_AIRTIME = 86,
    MIB_NON_WIFI_TIME,
    MIB_TXOP_INIT_COUNT,

// mt7916
    MIB_TX_TIME_V2 = 6,
    MIB_RX_TIME_V2 = 8,
    MIB_OBSS_AIRTIME_V2 = 490,
    MIB_NON_WIFI_TIME_V2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_txpower_sku {
    pub format_id: u8,
    pub limit_type: u8,
    pub band_idx: u8,
    pub txpower_sku: [i8; MT7915_SKU_RATE_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edca {
    pub queue: u8,
    pub set: u8,
    pub aifs: u8,
    pub cw_min: u8,
    pub cw_max: __le16,
    pub txop: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_tx {
    pub total: u8,
    pub action: u8,
    pub valid: u8,
    pub mode: u8,
    pub edca: [edca; IEEE80211_NUM_ACS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_mcu_muru_stats {
    pub event_id: __le32,
    pub cck_cnt: __le32,
    pub ofdm_cnt: __le32,
    pub htmix_cnt: __le32,
    pub htgf_cnt: __le32,
    pub vht_su_cnt: __le32,
    pub vht_2mu_cnt: __le32,
    pub vht_3mu_cnt: __le32,
    pub vht_4mu_cnt: __le32,
    pub he_su_cnt: __le32,
    pub he_ext_su_cnt: __le32,
    pub he_2ru_cnt: __le32,
    pub he_2mu_cnt: __le32,
    pub he_3ru_cnt: __le32,
    pub he_3mu_cnt: __le32,
    pub he_4ru_cnt: __le32,
    pub he_4mu_cnt: __le32,
    pub he_5to8ru_cnt: __le32,
    pub he_9to16ru_cnt: __le32,
    pub he_gtr16ru_cnt: __le32,
    pub dl: },
    pub hetrig_su_cnt: __le32,
    pub hetrig_2ru_cnt: __le32,
    pub hetrig_3ru_cnt: __le32,
    pub hetrig_4ru_cnt: __le32,
    pub hetrig_5to8ru_cnt: __le32,
    pub hetrig_9to16ru_cnt: __le32,
    pub hetrig_gtr16ru_cnt: __le32,
    pub hetrig_2mu_cnt: __le32,
    pub hetrig_3mu_cnt: __le32,
    pub hetrig_4mu_cnt: __le32,
    pub ul: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_mmps_mode {
    MCU_MMPS_STATIC,
    MCU_MMPS_DYNAMIC,
    MCU_MMPS_RSV,
    MCU_MMPS_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_bmc_rate {
    pub tag: __le16,
    pub len: __le16,
    pub bc_trans: __le16,
    pub mc_trans: __le16,
    pub short_preamble: u8,
    pub rsv: [u8; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_ra {
    pub tag: __le16,
    pub len: __le16,
    pub op_mode: u8,
    pub adhoc_en: u8,
    pub short_preamble: u8,
    pub tx_streams: u8,
    pub rx_streams: u8,
    pub algo: u8,
    pub force_sgi: u8,
    pub force_gf: u8,
    pub ht_mode: u8,
    pub /: *mut *mut u8 has_20_sta; / Check if any sta support GF.,
    pub bss_width_trigger_events: u8,
    pub vht_nss_cap: u8,
    pub /: *mut *mut u8 vht_bw_signal; / not use,
    pub /: *mut *mut u8 vht_force_sgi; / not use,
    pub se_off: u8,
    pub antenna_idx: u8,
    pub train_up_rule: u8,
    pub rsv: [u8; 3],
    pub train_up_high_thres: __le16,
    pub train_up_rule_rssi: __le16,
    pub low_traffic_thres: __le16,
    pub max_phyrate: __le16,
    pub phy_cap: __le32,
    pub interval: __le32,
    pub fast_interval: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_hw_amsdu {
    pub tag: __le16,
    pub len: __le16,
    pub cmp_bitmap_0: __le32,
    pub cmp_bitmap_1: __le32,
    pub trig_thres: __le16,
    pub enable: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_color {
    pub tag: __le16,
    pub len: __le16,
    pub disable: u8,
    pub color: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_he {
    pub tag: __le16,
    pub len: __le16,
    pub he_pe_duration: u8,
    pub vht_op_info_present: u8,
    pub he_rts_thres: __le16,
    pub max_nss_mcs: [__le16; CMD_HE_MCS_BW_NUM],
    pub rsv: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_bcn {
    pub tag: __le16,
    pub len: __le16,
    pub ver: u8,
    pub enable: u8,
    pub sub_ntlv: __le16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_bcn_cntdwn {
    pub tag: __le16,
    pub len: __le16,
    pub cnt: u8,
    pub rsv: [u8; 3],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_bcn_mbss {
pub const MAX_BEACON_NUM: c_int = 32;
    pub tag: __le16,
    pub len: __le16,
    pub bitmap: __le32,
    pub offset: [__le16; MAX_BEACON_NUM],
    pub rsv: [u8; 8],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_bcn_cont {
    pub tag: __le16,
    pub len: __le16,
    pub tim_ofs: __le16,
    pub csa_ofs: __le16,
    pub bcc_ofs: __le16,
    pub pkt_len: __le16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_inband_discovery {
    pub tag: __le16,
    pub len: __le16,
    pub tx_type: u8,
    pub tx_mode: u8,
    pub tx_interval: u8,
    pub enable: u8,
    pub rsv: __le16,
    pub prob_rsp_len: __le16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_prot {
    pub tag: __le16,
    pub len: __le16,
    pub prot_type: __le32,
    pub prot_mode: __le32,
    pub rts_len_thres: __le32,
    pub he_rts_thres: __le16,
    pub rts_pkt_thres: u8,
    pub rsv: [u8; 5],
    pub __packed: },
}

// tx cmd tx statistics
// recovery
// action
pub const MT7915_MAX_BEACON_SIZE: c_int = 1308;

