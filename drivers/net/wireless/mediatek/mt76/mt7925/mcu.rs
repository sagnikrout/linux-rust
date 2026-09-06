//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7925/mcu.h
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
// Copyright (C) 2023 MediaTek Inc.

// ext event table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_eeprom_info {
    pub addr: __le32,
    pub valid: __le32,
    pub data: [u8; MT7925_EEPROM_BLOCK_SIZE],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_rxd {
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
    pub tlv: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_uni_event {
    pub cid: u8,
    pub pad: [u8; 3],
    pub /: *mut *mut __le32 status; / 0: success, others: fail,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_reg_event {
    pub reg: __le32,
    pub val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_ant_id_config {
    pub ant_id: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_txpwr_req {
    pub _rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub format_id: u8,
    pub catg: u8,
    pub band_idx: u8,
    pub _rsv1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_txpwr_event {
    pub rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
    pub catg: u8,
    pub band_idx: u8,
    pub ch_band: u8,
    pub /: *mut *mut u8 format; / 0:Legacy, 1:HE,
// Rate power info
    pub txpwr: mt7925_txpwr,
    pub pwr_max: i8,
    pub pwr_min: i8,
    pub rsv1: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_rftest_evt {
    pub param0: __le32,
    pub param1: __le32,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connac3_mcu_cipher_type {
    CONNAC3_CIPHER_NONE = 0,
    CONNAC3_CIPHER_WEP40 = 1,
    CONNAC3_CIPHER_TKIP = 2,
    CONNAC3_CIPHER_AES_CCMP = 4,
    CONNAC3_CIPHER_WEP104 = 5,
    CONNAC3_CIPHER_BIP_CMAC_128 = 6,
    CONNAC3_CIPHER_WEP128 = 7,
    CONNAC3_CIPHER_WAPI = 8,
    CONNAC3_CIPHER_CCMP_256 = 10,
    CONNAC3_CIPHER_GCMP = 11,
    CONNAC3_CIPHER_GCMP_256 = 12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DMASHDL_GROUP_IDX {
    DMASHDL_GROUP_0 = 0,
    DMASHDL_GROUP_1,
    DMASHDL_GROUP_2,
    DMASHDL_GROUP_3,
    DMASHDL_GROUP_4,
    DMASHDL_GROUP_5,
    DMASHDL_GROUP_6,
    DMASHDL_GROUP_7,
    DMASHDL_GROUP_8,
    DMASHDL_GROUP_9,
    DMASHDL_GROUP_10,
    DMASHDL_GROUP_11,
    DMASHDL_GROUP_12,
    DMASHDL_GROUP_13,
    DMASHDL_GROUP_14,
    DMASHDL_GROUP_15,
    DMASHDL_GROUP_NUM,
    DMASHDL_LITE_GROUP_NUM = 64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_scan_chinfo_event {
    pub nr_chan: u8,
    pub alpha2: [u8; 3],
    pub __packed: },
}

pub const MT7925_RNR_SCAN_MAX_BSSIDS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_hdr_tlv {
// fixed field
    pub seq_num: u8,
    pub bss_idx: u8,
    pub pad: [u8; 2],
// tlv
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_req_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub SCAN: *mut *mut u8 scan_type; / 0: PASSIVE,
// 1: ACTIVE SCAN
//
    pub /: *mut *mut u8 probe_req_num; / Number of probe request for each SSID,
    pub scan: *mut *mut u8 scan_func; / BIT(0) Enable random MAC,
// BIT(1) Disable DBDC scan type 1~3.
// BIT(2) Use DBDC scan type 3 (dedicated one RF to scan).
//
    pub src_mask: u8,
    pub channel_min_dwell_time: __le16,
    pub /: *mut *mut __le16 channel_dwell_time; / channel Dwell interval,
    pub timeout_value: __le16,
    pub probe_delay_time: __le16,
    pub func_mask_ext: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_ssid_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub SSID: *mut *mut u8 ssid_type; / BIT(0) wildcard,
// BIT(1) P2P wildcard SSID
// BIT(2) specified SSID + wildcard SSID
// BIT(2) + ssid_type_ext BIT(0) specified SSID only
//
    pub ssids_num: u8,
    pub is_short_ssid: u8,
    pub pad: u8,
    pub ssids: [mt76_connac_mcu_scan_ssid; MT7925_RNR_SCAN_MAX_BSSIDS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_bssid_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub match_ch: u8,
    pub match_ssid_ind: u8,
    pub rcpi: u8,
    pub match_short_ssid_ind: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_chan_info_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub channels: *mut *mut u8 channel_type; / 0: Full,
// 1: Only 2.4GHz channels
// 2: Only 5GHz channels
// 3: P2P social channel only (channel #1, #6 and #11)
// 4: Specified channels
// Others: Reserved
//
    pub /: *mut *mut u8 channels_num; / valid when channel_type is 4,
    pub pad: [u8; 2],
    pub channels: [mt76_connac_mcu_scan_channel; 64],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_ie_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub ies_len: __le16,
    pub band: u8,
    pub pad: u8,
    pub ies: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_misc_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub random_mac: [u8; ETH_ALEN],
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_sched_req {
    pub tag: __le16,
    pub len: __le16,
    pub version: u8,
    pub stop_on_match: u8,
    pub intervals_num: u8,
    pub scan_func: u8,
    pub intervals: [__le16; MT76_CONNAC_MAX_NUM_SCHED_SCAN_INTERVAL],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_sched_ssid_match_sets {
    pub tag: __le16,
    pub len: __le16,
    pub match_num: u8,
    pub rsv: [u8; 3],
    pub match: [mt76_connac_mcu_scan_match; MT76_CONNAC_MAX_SCAN_MATCH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_sched_enable {
    pub tag: __le16,
    pub len: __le16,
    pub active: u8,
    pub rsv: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbmc_set_req {
    pub pad: [u8; 4],
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbmc_conf_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub mbmc_en: u8,
    pub band: u8,
    pub pad: [u8; 2],
    pub __packed: },
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_req_hdr {
    pub bss_idx: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_rate_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub __rsv1: [u8; 2],
    pub basic_rate: __le16,
    pub bc_trans: __le16,
    pub mc_trans: __le16,
    pub short_preamble: u8,
    pub bc_fixed_rate: u8,
    pub mc_fixed_rate: u8,
    pub __rsv2: u8,
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
    pub eml_enable: u8,
    pub max_link_num: u8,
    pub hybrid_mode: u8,
    pub __rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_eht_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub is_eht_op_present: u8,
    pub is_eth_dscb_present: u8,
    pub eht_ctrl: u8,
    pub eht_ccfs0: u8,
    pub eht_ccfs1: u8,
    pub pad1: u8,
    pub eht_dis_sub_chan_bitmap: __le16,
    pub pad2: [u8; 4],
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
pub struct sta_rec_sec_uni {
    pub tag: __le16,
    pub len: __le16,
    pub add: u8,
    pub tx_key: u8,
    pub key_type: u8,
    pub is_authenticator: u8,
    pub peer_addr: [u8; 6],
    pub bss_idx: u8,
    pub cipher_id: u8,
    pub key_id: u8,
    pub key_len: u8,
    pub wlan_idx: u8,
    pub mgmt_prot: u8,
    pub key: [u8; 32],
    pub key_rsc: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_hdr_trans {
    pub tag: __le16,
    pub len: __le16,
    pub from_ds: u8,
    pub to_ds: u8,
    pub dis_rx_hdr_tran: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_mld {
    pub tag: __le16,
    pub len: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub primary_id: __le16,
    pub secondary_id: __le16,
    pub wlan_id: __le16,
    pub link_num: u8,
    pub rsv: [u8; 3],
    pub wlan_id: __le16,
    pub bss_idx: u8,
    pub rsv: u8,
    pub link: [} __packed; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_eht_mld {
    pub tag: __le16,
    pub len: __le16,
    pub nsep: u8,
    pub mld_type: u8,
    pub __rsv1: [u8; 1],
    pub str_cap: [u8; 3],
    pub eml_cap: [u8; 3],
    pub __rsv2: [u8; 3],
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
    pub pad: [u8; 3],
    pub __packed: },

pub const MT_CONNAC3_SKU_POWER_LIMIT: c_int = 449;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_sku_tlv {
    pub channel: u8,
    pub pwr_limit: [i8; MT_CONNAC3_SKU_POWER_LIMIT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_tx_power_limit_tlv {
    pub rsv: [u8; 4],
    pub tag: __le16,
    pub len: __le16,
// DW0 - common info
    pub ver: u8,
    pub pad0: u8,
    pub rsv1: __le16,
// DW1 - cmd hint
    pub /: *mut *mut u8 n_chan; / # channel,
    pub /: *mut *mut u8 band; / 2.4GHz - 5GHz - 6GHz,
    pub last_msg: u8,
    pub limit_type: u8,
// DW3
    pub /: *mut *mut u8 alpha2[4]; / regulatory_request.alpha2,
    pub pad2: [u8; 32],
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_arpns_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub enable: u8,
    pub ips_num: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_wow_pattern_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub bss_idx: u8,
    pub /: *mut *mut u8 index; / pattern index,
    pub disable: *mut *mut u8 enable; / 0:,
// 1: enable
//
    pub /: *mut *mut u8 data_len; / pattern length,
    pub offset: u8,
    pub mask: [u8; MT76_CONNAC_WOW_MASK_MAX_LEN],
    pub pattern: [u8; MT76_CONNAC_WOW_PATTEN_MAX_LEN],
    pub rsv: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct roc_acquire_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub bss_idx: u8,
    pub tokenid: u8,
    pub control_channel: u8,
    pub sco: u8,
    pub band: u8,
    pub bw: u8,
    pub center_chan: u8,
    pub center_chan2: u8,
    pub bw_from_ap: u8,
    pub center_chan_from_ap: u8,
    pub center_chan2_from_ap: u8,
    pub reqtype: u8,
    pub maxinterval: __le32,
    pub dbdcband: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ENUM_CMD_TEST_CTRL_ACT {
    CMD_TEST_CTRL_ACT_SWITCH_MODE = 0,
    CMD_TEST_CTRL_ACT_SET_AT = 1,
    CMD_TEST_CTRL_ACT_GET_AT = 2,
    CMD_TEST_CTRL_ACT_SET_AT_ENG = 3,
    CMD_TEST_CTRL_ACT_GET_AT_ENG = 4,
    CMD_TEST_CTRL_ACT_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ENUM_CMD_TEST_CTRL_ACT_SWITCH_MODE_OP {
    CMD_TEST_CTRL_ACT_SWITCH_MODE_NORMAL = 0,
    CMD_TEST_CTRL_ACT_SWITCH_MODE_RF_TEST = 1,
    CMD_TEST_CTRL_ACT_SWITCH_MODE_ICAP = 2,
    CMD_TEST_CTRL_ACT_SWITCH_MODE_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union testmode_data {
    pub op_mode: __le32,
    pub channel_freq: __le32,
    pub rf_at_info: [u8; 84],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union testmode_evt {
    pub op_mode: __le32,
    pub channel_freq: __le32,
    pub rf_at_info: [u8; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_cmd_testmode_ctrl {
    pub tag: u16,
    pub length: u16,
    pub action: u8,
    pub reserved: [u8; 3],
    pub data: testmode_data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_rftest_cmd {
    pub padding: [u8; 4],
    pub ctrl: uni_cmd_testmode_ctrl,
    pub __packed: },
    pub CONNAC3_CIPHER_WEP40: return,
    pub CONNAC3_CIPHER_WEP104: return,
    pub CONNAC3_CIPHER_TKIP: return,
    pub CONNAC3_CIPHER_BIP_CMAC_128: return,
    pub CONNAC3_CIPHER_AES_CCMP: return,
    pub CONNAC3_CIPHER_CCMP_256: return,
    pub CONNAC3_CIPHER_GCMP: return,
    pub CONNAC3_CIPHER_GCMP_256: return,
    pub CONNAC3_CIPHER_WAPI: return,
    pub CONNAC3_CIPHER_NONE: return,
    pub len: u16,
    pub false: return,
    pub le16_to_cpu(tlv->len): len =,
// a length below the header size would not advance the cursor
    pub rem: *mut *mut return len >= sizeof(tlv) && len <=,

    pub \: for (; mt7925_mcu_tlv_valid(tlv, rem);,
    pub enable): *mut *mut int mt7925_mcu_set_dbdc(struct mt76_phy phy, bool,
    pub scan_req): *mut ieee80211_scan_request,
    pub vif): *mut ieee80211_vif,
    pub ies): *mut ieee80211_scan_ies,
    pub enable): bool,
    pub vif): *mut ieee80211_vif,
    pub enable): c_int,
    pub enable): c_int,
    pub link_conf): *mut ieee80211_bss_conf,
    pub enable): *mut *mut int mt7925_mcu_set_deep_sleep(struct mt792x_dev dev, bool,
    pub dev): *mut int mt7925_mcu_set_thermal_protect(struct mt792x_dev,
    pub phy): *mut int mt7925_mcu_set_channel_domain(struct mt76_phy,
    pub enable): *mut *mut int mt7925_mcu_set_radio_en(struct mt792x_phy phy, bool,
    pub ctx): *mut ieee80211_chanctx_conf,
    pub ctx): *mut ieee80211_chanctx_conf,
    pub phy): *mut int mt7925_mcu_set_rate_txpower(struct mt76_phy,
    pub link_conf): *mut ieee80211_bss_conf,
    pub enable): *mut *mut ieee80211_bss_conf link_conf, bool,
