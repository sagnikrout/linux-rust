//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7996/mcu.h
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
pub struct mt7996_mcu_rxd {
    pub rxd: [__le32; 8],
    pub len: __le16,
    pub pkt_type_id: __le16,
    pub eid: u8,
    pub seq: u8,
    pub option: u8,
    pub __rsv: u8,
    pub ext_eid: u8,
    pub __rsv1: [u8; 2],
    pub s2d_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_uni_event {
    pub cid: __le16,
    pub __rsv: [u8; 2],
    pub /: *mut *mut __le32 status; / 0: success, others: fail,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_thermal_ctrl {
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
pub struct mt7996_mcu_thermal_enable {
    pub trigger_temp: __le32,
    pub restore_temp: __le32,
    pub sustain_time: __le16,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_countdown_notify {
    pub omac_idx: u8,
    pub count: u8,
    pub /: *mut *mut u8 csa_failure_reason; / 0: success, 1: beacon disabled,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_rdd_report {
    pub rxd: mt7996_mcu_rxd,
    pub __rsv1: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
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
    pub __rsv2: u8,
    pub out_pri_const: __le32,
    pub out_pri_stg: [__le32; 3],
    pub out_pri_stg_dmin: __le32,
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
pub struct mt7996_mcu_background_chain_ctrl {
    pub _rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
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
pub struct mt7996_mcu_eeprom_update {
    pub _rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub buffer_mode: u8,
    pub format: u8,
    pub buf_len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union eeprom_data {
    pub data_len: __le32,
    pub data): DECLARE_FLEX_ARRAY(u8,,
    pub ext_eeprom: },
    pub efuse): DECLARE_FLEX_ARRAY(u8,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_eeprom_info {
    pub _rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub addr: __le32,
    pub valid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_eeprom_access {
    pub info: mt7996_mcu_eeprom_info,
    pub eeprom: eeprom_data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_eeprom_access_event {
    pub _rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub version: __le32,
    pub addr: __le32,
    pub valid: __le32,
    pub size: __le32,
    pub magic_no: __le32,
    pub type: __le32,
    pub rsv: [__le32; 4],
    pub eeprom: eeprom_data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_phy_rx_info {
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
pub struct mt7996_mcu_mib {
    pub tag: __le16,
    pub len: __le16,
    pub offs: __le32,
    pub data: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct all_sta_trx_rate {
    pub wlan_idx: __le16,
    pub __rsv1: [u8; 2],
    pub tx_mode: u8,
    pub flags: u8,
    pub tx_stbc: u8,
    pub tx_gi: u8,
    pub tx_bw: u8,
    pub tx_ldpc: u8,
    pub tx_mcs: u8,
    pub tx_nss: u8,
    pub rx_rate: u8,
    pub rx_mode: u8,
    pub rx_nsts: u8,
    pub rx_gi: u8,
    pub rx_coding: u8,
    pub rx_stbc: u8,
    pub rx_bw: u8,
    pub __rsv2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_all_sta_info_event {
    pub rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub more: u8,
    pub rsv2: u8,
    pub sta_num: __le16,
    pub rsv3: [u8; 4],
    pub rate): DECLARE_FLEX_ARRAY(struct all_sta_trx_rate,,
    pub wlan_idx: __le16,
    pub rsv: [u8; 2],
    pub tx_bytes: [__le32; IEEE80211_NUM_ACS],
    pub rx_bytes: [__le32; IEEE80211_NUM_ACS],
    pub adm_stat): } __packed,,
    pub wlan_idx: __le16,
    pub rsv: [u8; 2],
    pub tx_msdu_cnt: __le32,
    pub rx_msdu_cnt: __le32,
    pub msdu_cnt): } __packed,,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_wed_rro_event {
    pub rxd: mt7996_mcu_rxd,
    pub __rsv1: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_wed_rro_ba_event {
    pub tag: __le16,
    pub len: __le16,
    pub wlan_id: __le16,
    pub tid: u8,
    pub __rsv1: u8,
    pub status: __le32,
    pub id: __le16,
    pub __rsv2: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_wed_rro_ba_delete_event {
    pub tag: __le16,
    pub len: __le16,
    pub session_id: __le16,
    pub __rsv2: [u8; 2],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_ps_sync_event {
    pub rxd: mt7996_mcu_rxd,
    pub bss_idx: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_ps_client_info {
    pub tag: __le16,
    pub len: __le16,
    pub ps_bit: u8,
    pub __rsv: u8,
    pub wlan_idx: __le16,
    pub buffer_size: u8,
    pub __rsv2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_ps_multi_client_info {
    pub tag: __le16,
    pub len: __le16,
    pub sta_cnt: __le16,
    pub sta_ps_info: [__le16; ],
    pub __packed: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_mcu_thermal_notify {
    pub rxd: mt7996_mcu_rxd,
    pub __rsv1: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub event_id: u8,
    pub band_idx: u8,
    pub level_idx: u8,
    pub duty_percent: u8,
    pub restore_temp: __le32,
    pub __rsv2: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_chan_mib_offs {
    UNI_MIB_OBSS_AIRTIME = 26,
    UNI_MIB_NON_WIFI_TIME = 27,
    UNI_MIB_TX_TIME = 28,
    UNI_MIB_RX_TIME = 29
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edca {
    pub tag: __le16,
    pub len: __le16,
    pub queue: u8,
    pub set: u8,
    pub cw_min: u8,
    pub cw_max: u8,
    pub txop: __le16,
    pub aifs: u8,
    pub __rsv: u8,
}

pub const MCU_PKT_ID: c_uint = 0xa0;
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
pub struct bss_rate_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub __rsv1: [u8; 4],
    pub bc_trans: __le16,
    pub mc_trans: __le16,
    pub short_preamble: u8,
    pub bc_fixed_rate: u8,
    pub mc_fixed_rate: u8,
    pub __rsv2: [u8; 9],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_ra_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub short_preamble: u8,
    pub force_sgi: u8,
    pub force_gf: u8,
    pub ht_mode: u8,
    pub se_off: u8,
    pub antenna_idx: u8,
    pub max_phyrate: __le16,
    pub force_tx_streams: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_rlm_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub control_channel: u8,
    pub center_chan: u8,
    pub center_chan2: u8,
    pub bw: u8,
    pub tx_streams: u8,
    pub rx_streams: u8,
    pub ht_op_info: u8,
    pub sco: u8,
    pub band: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_color_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub enable: u8,
    pub color: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_inband_discovery_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub tx_type: u8,
    pub tx_mode: u8,
    pub tx_interval: u8,
    pub enable: u8,
    pub wcid: __le16,
    pub prob_rsp_len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_bcn_content_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub tim_ie_pos: __le16,
    pub csa_ie_pos: __le16,
    pub bcc_ie_pos: __le16,
    pub enable: u8,
    pub type: u8,
    pub pkt_len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_bcn_cntdwn_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub cnt: u8,
    pub static_pp: bool,
    pub abort: bool,
    pub csa: },
    pub abort: bool,
    pub cca: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_bcn_mbss_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub bitmap: __le32,
pub const MAX_BEACON_NUM: c_int = 32;
    pub offset: [__le16; MAX_BEACON_NUM],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_txcmd_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub txcmd_mode: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_sec_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub __rsv1: [u8; 2],
    pub cipher: u8,
    pub __rsv2: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_ifs_time_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub slot_valid: u8,
    pub sifs_valid: u8,
    pub rifs_valid: u8,
    pub eifs_valid: u8,
    pub slot_time: __le16,
    pub sifs_time: __le16,
    pub rifs_time: __le16,
    pub eifs_time: __le16,
    pub eifs_cck_valid: u8,
    pub rsv: u8,
    pub eifs_cck_time: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_power_save {
    pub tag: __le16,
    pub len: __le16,
    pub profile: u8,
    pub _rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_mld_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub group_mld_id: u8,
    pub own_mld_id: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub remap_idx: u8,
    pub link_id: u8,
    pub __rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_prot_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub prot_mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_mld_link_op_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub group_mld_id: u8,
    pub own_mld_id: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub remap_idx: u8,
    pub link_operation: u8,
    pub link_id: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ht_uni {
    pub tag: __le16,
    pub len: __le16,
    pub ht_cap: __le16,
    pub ht_cap_ext: __le16,
    pub ampdu_param: u8,
    pub _rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ba_uni {
    pub tag: __le16,
    pub len: __le16,
    pub tid: u8,
    pub ba_type: u8,
    pub amsdu: u8,
    pub ba_en: u8,
    pub ssn: __le16,
    pub winsize: __le16,
    pub ba_rdd_rro: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_eht {
    pub tag: __le16,
    pub len: __le16,
    pub tid_bitmap: u8,
    pub _rsv: u8,
    pub mac_cap: __le16,
    pub phy_cap: __le64,
    pub phy_cap_ext: __le64,
    pub mcs_map_bw20: [u8; 4],
    pub mcs_map_bw80: [u8; 3],
    pub mcs_map_bw160: [u8; 3],
    pub mcs_map_bw320: [u8; 3],
    pub _rsv2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_key_uni {
    pub wlan_idx: __le16,
    pub mgmt_prot: u8,
    pub cipher_id: u8,
    pub cipher_len: u8,
    pub key_id: u8,
    pub key_len: u8,
    pub need_resp: u8,
    pub key: [u8; 32],
    pub pn: [u8; 6],
    pub bcn_mode: u8,
    pub _rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_sec_uni {
    pub tag: __le16,
    pub len: __le16,
    pub add: u8,
    pub n_cipher: u8,
    pub rsv: [u8; 2],
    pub key: [sec_key_uni; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_phy_uni {
    pub type: u8,
    pub flag: u8,
    pub stbc: u8,
    pub sgi: u8,
    pub bw: u8,
    pub ldpc: u8,
    pub mcs: u8,
    pub nss: u8,
    pub he_ltf: u8,
    pub rsv: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ra_uni {
    pub tag: __le16,
    pub len: __le16,
    pub valid: u8,
    pub auto_rate: u8,
    pub phy_mode: u8,
    pub channel: u8,
    pub bw: u8,
    pub disable_cck: u8,
    pub ht_mcs32: u8,
    pub ht_gf: u8,
    pub ht_mcs: [u8; 4],
    pub mmps_mode: u8,
    pub gband_256: u8,
    pub af: u8,
    pub auth_wapi_mode: u8,
    pub rate_len: u8,
    pub supp_mode: u8,
    pub supp_cck_rate: u8,
    pub supp_ofdm_rate: u8,
    pub supp_ht_mcs: __le32,
    pub supp_vht_mcs: [__le16; 4],
    pub op_mode: u8,
    pub op_vht_chan_width: u8,
    pub op_vht_rx_nss: u8,
    pub op_vht_rx_nss_type: u8,
    pub sta_cap: __le32,
    pub phy: sta_phy_uni,
    pub rx_rcpi: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ra_fixed_uni {
    pub tag: __le16,
    pub len: __le16,
    pub field: __le32,
    pub op_mode: u8,
    pub op_vht_chan_width: u8,
    pub op_vht_rx_nss: u8,
    pub op_vht_rx_nss_type: u8,
    pub phy: sta_phy_uni,
    pub spe_idx: u8,
    pub short_preamble: u8,
    pub is_5g: u8,
    pub mmps_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_hdrt {
    pub tag: __le16,
    pub len: __le16,
    pub hdrt_mode: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_hdr_trans {
    pub tag: __le16,
    pub len: __le16,
    pub from_ds: u8,
    pub to_ds: u8,
    pub dis_rx_hdr_tran: u8,
    pub mesh: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ps_leave {
    pub tag: __le16,
    pub len: __le16,
    pub __rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_mld_setup {
    pub tag: __le16,
    pub len: __le16,
    pub mld_addr: [u8; ETH_ALEN],
    pub primary_id: __le16,
    pub seconed_id: __le16,
    pub setup_wcid: __le16,
    pub link_num: u8,
    pub info: u8,
    pub __rsv: [u8; 2],
    pub link_info: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_eht_mld {
    pub tag: __le16,
    pub len: __le16,
    pub nsep: u8,
    pub __rsv1: [u8; 2],
    pub str_cap: [u8; __MT_MAX_BAND],
    pub eml_cap: __le16,
    pub __rsv2: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld_setup_link {
    pub wcid: __le16,
    pub bss_idx: u8,
    pub __rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld_req_hdr {
    pub ver: u8,
    pub mld_addr: [u8; ETH_ALEN],
    pub mld_idx: u8,
    pub flag: u8,
    pub rsv: [u8; 3],
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld_reconf_stop_link {
    pub tag: __le16,
    pub len: __le16,
    pub link_bitmap: __le16,
    pub rsv: [u8; 2],
    pub bss_idx: [u8; 16],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_trans_en {
    pub tag: __le16,
    pub len: __le16,
    pub enable: u8,
    pub check_bssid: u8,
    pub mode: u8,
    pub __rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_trans_vlan {
    pub tag: __le16,
    pub len: __le16,
    pub insert_vlan: u8,
    pub remove_vlan: u8,
    pub tid: u8,
    pub __rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_trans_blacklist {
    pub tag: __le16,
    pub len: __le16,
    pub idx: u8,
    pub enable: u8,
    pub type: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_header {
    pub __rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vow_rx_airtime {
    pub tag: __le16,
    pub len: __le16,
    pub enable: u8,
    pub band: u8,
    pub __rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bf_sounding_on {
    pub tag: __le16,
    pub len: __le16,
    pub snd_mode: u8,
    pub sta_num: u8,
    pub __rsv: [u8; 2],
    pub wlan_id: [__le16; 4],
    pub snd_period: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bf_hw_en_status_update {
    pub tag: __le16,
    pub len: __le16,
    pub ebf: bool,
    pub ibf: bool,
    pub __rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bf_mod_en_ctrl {
    pub tag: __le16,
    pub len: __le16,
    pub bf_num: u8,
    pub bf_bitmap: u8,
    pub bf_sel: [u8; 8],
    pub __rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union bf_tag_tlv {
    pub bf_snd: bf_sounding_on,
    pub bf_hw_en: bf_hw_en_status_update,
    pub bf_mod_en: bf_mod_en_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ra_rate {
    pub wlan_idx: __le16,
    pub mode: u8,
    pub stbc: u8,
    pub gi: __le16,
    pub bw: u8,
    pub ldpc: u8,
    pub mcs: u8,
    pub nss: u8,
    pub ltf: __le16,
    pub spe: u8,
    pub preamble: u8,
    pub __rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ra_fixed_rate {
    pub tag: __le16,
    pub len: __le16,
    pub version: __le16,
    pub rate: ra_rate,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_req_hdr {
    pub bss_idx: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
}

pub const MT7996_MAX_BSS_OFFLOAD_SIZE: c_int = 2048;

// recovery
// action

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed_rate_table_ctrl {
    pub _rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub table_idx: u8,
    pub antenna_idx: u8,
    pub rate_idx: __le16,
    pub spe_idx_sel: u8,
    pub spe_idx: u8,
    pub gi: u8,
    pub he_ltf: u8,
    pub ldpc: bool,
    pub txbf: bool,
    pub dynamic_bw: bool,
    pub _rsv2: u8,
    pub __packed: },
