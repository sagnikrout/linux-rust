//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/ioctl.h
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
// NXP Wireless LAN device driver: ioctl data structures & APIs
//
// Copyright 2011-2020 NXP
//
pub const NUM_WEP_KEYS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_user_scan {
    pub scan_cfg_len: u32,
    pub scan_cfg_buf: [u8; 1],
}

pub const MWIFIEX_PROMISC_MODE: c_int = 1;
pub const MWIFIEX_MULTICAST_MODE: c_int = 2;
pub const MWIFIEX_ALL_MULTI_MODE: c_int = 4;
pub const MWIFIEX_MAX_MULTICAST_LIST_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_multicast_list {
    pub mode: u32,
    pub num_multicast_addr: u32,
    pub mac_list: [u8; MWIFIEX_MAX_MULTICAST_LIST_SIZE][ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_chan_freq {
    pub channel: u32,
    pub freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ssid_bssid {
    pub ssid: cfg80211_ssid,
    pub bssid: [u8; ETH_ALEN],
}

pub const MWIFIEX_WPA_PASSHPHRASE_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpa_param {
    pub pairwise_cipher_wpa: u8,
    pub pairwise_cipher_wpa2: u8,
    pub group_cipher: u8,
    pub length: u32,
    pub passphrase: [u8; MWIFIEX_WPA_PASSHPHRASE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wep_key {
    pub key_index: u8,
    pub is_default: u8,
    pub length: u16,
    pub key: [u8; WLAN_KEY_LEN_WEP104],
}

pub const KEY_MGMT_ON_HOST: c_uint = 0x03;
pub const MWIFIEX_AUTH_MODE_AUTO: c_uint = 0xFF;
pub const BAND_CONFIG_BG: c_uint = 0x00;
pub const BAND_CONFIG_A: c_uint = 0x01;
pub const MWIFIEX_SEC_CHAN_BELOW: c_uint = 0x30;
pub const MWIFIEX_SEC_CHAN_ABOVE: c_uint = 0x10;
pub const MWIFIEX_SUPPORTED_RATES: c_int = 14;
pub const MWIFIEX_SUPPORTED_RATES_EXT: c_int = 32;
pub const MWIFIEX_TDLS_SUPPORTED_RATES: c_int = 8;
pub const MWIFIEX_TDLS_DEF_QOS_CAPAB: c_uint = 0xf;
pub const MWIFIEX_PRIO_BK: c_int = 2;
pub const MWIFIEX_PRIO_VI: c_int = 5;
pub const MWIFIEX_SUPPORTED_CHANNELS: c_int = 2;
pub const MWIFIEX_OPERATING_CLASSES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_uap_bss_param {
    pub channel: u8,
    pub band_cfg: u8,
    pub rts_threshold: u16,
    pub frag_threshold: u16,
    pub retry_limit: u8,
    pub ssid: mwifiex_802_11_ssid,
    pub bcast_ssid_ctl: u8,
    pub radio_ctl: u8,
    pub dtim_period: u8,
    pub beacon_period: u16,
    pub auth_mode: u16,
    pub protocol: u16,
    pub key_mgmt: u16,
    pub key_mgmt_operation: u16,
    pub wpa_cfg: wpa_param,
    pub wep_cfg: [wep_key; NUM_WEP_KEYS],
    pub ht_cap: ieee80211_ht_cap,
    pub vht_cap: ieee80211_vht_cap,
    pub rates: [u8; MWIFIEX_SUPPORTED_RATES],
    pub sta_ao_timer: u32,
    pub ps_sta_ao_timer: u32,
    pub qos_info: u8,
    pub power_constraint: u8,
    pub wmm_info: mwifiex_types_wmm_info,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_get_stats {
    pub mcast_tx_frame: u32,
    pub failed: u32,
    pub retry: u32,
    pub multi_retry: u32,
    pub frame_dup: u32,
    pub rts_success: u32,
    pub rts_failure: u32,
    pub ack_failure: u32,
    pub rx_frag: u32,
    pub mcast_rx_frame: u32,
    pub fcs_error: u32,
    pub tx_frame: u32,
    pub wep_icv_error: [u32; 4],
    pub bcn_rcv_cnt: u32,
    pub bcn_miss_cnt: u32,
}

pub const MWIFIEX_MAX_VER_STR_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ver_ext {
    pub version_str_sel: u32,
    pub version_str: [c_char; MWIFIEX_MAX_VER_STR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_bss_info {
    pub bss_mode: u32,
    pub ssid: cfg80211_ssid,
    pub bss_chan: u32,
    pub country_code: [u8; 3],
    pub media_connected: u32,
    pub max_power_level: u32,
    pub min_power_level: u32,
    pub adhoc_state: u32,
    pub bcn_nf_last: signed int,
    pub wep_status: u32,
    pub is_hs_configured: u32,
    pub is_deep_sleep: u32,
    pub bssid: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sta_info {
    pub peer_mac: [u8; ETH_ALEN],
    pub params: *mut station_parameters,
}

pub const MAX_NUM_TID: c_int = 8;
pub const MAX_RX_WINSIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_rx_reorder_tbl {
    pub tid: u16,
    pub ta: [u8; ETH_ALEN],
    pub start_win: u32,
    pub win_size: u32,
    pub buffer: [u32; MAX_RX_WINSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_tx_ba_stream_tbl {
    pub tid: u16,
    pub ra: [u8; ETH_ALEN],
    pub amsdu: u8,
}

pub const DBG_CMD_NUM: c_int = 5;
pub const MWIFIEX_DBG_SDIO_MP_NUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdls_peer_info {
    pub peer_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_debug_info {
    pub debug_mask: c_uint,
    pub int_counter: u32,
    pub packets_out: [u32; MAX_NUM_TID],
    pub tx_buf_size: u32,
    pub curr_tx_buf_size: u32,
    pub tx_tbl_num: u32,
    pub rx_tbl_num: u32,
    pub tdls_peer_num: u32,
    pub ps_mode: u16,
    pub ps_state: u32,
    pub is_deep_sleep: u8,
    pub pm_wakeup_card_req: u8,
    pub pm_wakeup_fw_try: u32,
    pub is_hs_configured: u8,
    pub hs_activated: u8,
    pub num_cmd_host_to_card_failure: u32,
    pub num_cmd_sleep_cfm_host_to_card_failure: u32,
    pub num_tx_host_to_card_failure: u32,
    pub num_event_deauth: u32,
    pub num_event_disassoc: u32,
    pub num_event_link_lost: u32,
    pub num_cmd_deauth: u32,
    pub num_cmd_assoc_success: u32,
    pub num_cmd_assoc_failure: u32,
    pub num_tx_timeout: u32,
    pub is_cmd_timedout: u8,
    pub timeout_cmd_id: u16,
    pub timeout_cmd_act: u16,
    pub last_cmd_id: [u16; DBG_CMD_NUM],
    pub last_cmd_act: [u16; DBG_CMD_NUM],
    pub last_cmd_index: u16,
    pub last_cmd_resp_id: [u16; DBG_CMD_NUM],
    pub last_cmd_resp_index: u16,
    pub last_event: [u16; DBG_CMD_NUM],
    pub last_event_index: u16,
    pub data_sent: u8,
    pub cmd_sent: u8,
    pub cmd_resp_received: u8,
    pub event_received: u8,
    pub last_mp_wr_bitmap: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_mp_wr_ports: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_mp_wr_len: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_mp_curr_wr_port: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_sdio_mp_index: u8,
}

pub const MWIFIEX_KEY_INDEX_UNICAST: c_uint = 0x40000000;
pub const PN_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_encrypt_key {
    pub key_disable: u32,
    pub key_index: u32,
    pub key_len: u32,
    pub key_material: [u8; WLAN_MAX_KEY_LEN],
    pub mac_addr: [u8; ETH_ALEN],
    pub is_wapi_key: u32,
    pub /: *mut *mut u8 pn[PN_LEN]; / packet number,
    pub pn_len: u8,
    pub is_igtk_key: u8,
    pub is_current_wep_key: u8,
    pub is_rx_seq_valid: u8,
    pub is_igtk_def_key: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_power_cfg {
    pub is_power_auto: u32,
    pub is_power_fixed: u32,
    pub power_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_hs_cfg {
    pub is_invoke_hostcmd: u32,
// Bit0: non-unicast data
// Bit1: unicast data
// Bit2: mac events
// Bit3: magic packet
//
    pub conditions: u32,
    pub gpio: u32,
    pub gap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_wakeup_reason {
    pub hs_wakeup_reason: u16,
}

pub const DEEP_SLEEP_ON: c_int = 1;
pub const DEEP_SLEEP_OFF: c_int = 0;
pub const DEEP_SLEEP_IDLE_TIME: c_int = 100;
pub const PS_MODE_AUTO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_auto_ds {
    pub auto_ds: u16,
    pub idle_time: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_pm_cfg {
    pub ps_mode: u32,
    pub hs_cfg: mwifiex_ds_hs_cfg,
    pub auto_deep_sleep: mwifiex_ds_auto_ds,
    pub sleep_period: u32,
    pub param: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_11ac_vht_cfg {
    pub band_config: u8,
    pub misc_config: u8,
    pub cap_info: u32,
    pub mcs_tx_set: u32,
    pub mcs_rx_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_11n_tx_cfg {
    pub tx_htcap: u16,
    pub tx_htinfo: u16,
    pub /: *mut *mut u16 misc_config; / Needed for 802.11AC cards only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_11n_amsdu_aggr_ctrl {
    pub enable: u16,
    pub curr_buf_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_ant_cfg {
    pub tx_ant: u32,
    pub rx_ant: u32,
}

pub const MWIFIEX_NUM_OF_CMD_BUFFER: c_int = 50;
pub const MWIFIEX_SIZE_OF_CMD_BUFFER: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_reg_rw {
    pub type: u32,
    pub offset: u32,
    pub value: u32,
}

pub const MAX_EEPROM_DATA: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_read_eeprom {
    pub offset: u16,
    pub byte_count: u16,
    pub value: [u8; MAX_EEPROM_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_mem_rw {
    pub addr: u32,
    pub value: u32,
}

pub const IEEE_MAX_IE_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_misc_gen_ie {
    pub type: u32,
    pub len: u32,
    pub ie_data: [u8; IEEE_MAX_IE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_misc_cmd {
    pub len: u32,
    pub cmd: [u8; MWIFIEX_SIZE_OF_CMD_BUFFER],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum subsc_evt_rssi_state {
    EVENT_HANDLED,
    RSSI_LOW_RECVD,
    RSSI_HIGH_RECVD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subsc_evt_cfg {
    pub abs_value: u8,
    pub evt_freq: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_misc_subsc_evt {
    pub action: u16,
    pub events: u16,
    pub bcn_l_rssi_cfg: subsc_evt_cfg,
    pub bcn_h_rssi_cfg: subsc_evt_cfg,
}

pub const MWIFIEX_MEF_MAX_FILTERS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_mef_filter {
    pub repeat: u16,
    pub offset: u16,
    pub 1]: s8 byte_seq[MWIFIEX_MEF_MAX_BYTESEQ +,
    pub filt_type: u8,
    pub filt_action: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_mef_entry {
    pub mode: u8,
    pub action: u8,
    pub filter: [mwifiex_mef_filter; MWIFIEX_MEF_MAX_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_mef_cfg {
    pub criteria: u32,
    pub num_entries: u16,
    pub mef_entry: *mut mwifiex_mef_entry,
}

pub const MWIFIEX_VSIE_MASK_CLEAR: c_uint = 0x00;
pub const MWIFIEX_VSIE_MASK_SCAN: c_uint = 0x01;
pub const MWIFIEX_VSIE_MASK_ASSOC: c_uint = 0x02;
pub const MWIFIEX_VSIE_MASK_ADHOC: c_uint = 0x04;
pub const MWIFIEX_VSIE_MASK_BGSCAN: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum COALESCE_OPERATION {
    RECV_FILTER_MATCH_TYPE_EQ = 0x80,
    RECV_FILTER_MATCH_TYPE_NE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum COALESCE_PACKET_TYPE {
    PACKET_TYPE_UNICAST = 1,
    PACKET_TYPE_MULTICAST = 2,
    PACKET_TYPE_BROADCAST = 3
}

pub const MWIFIEX_COALESCE_MAX_RULES: c_int = 8;

pub const MWIFIEX_COALESCE_MAX_FILTERS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filt_field_param {
    pub operation: u8,
    pub operand_len: u8,
    pub offset: u16,
    pub operand_byte_stream: [u8; MWIFIEX_COALESCE_MAX_BYTESEQ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_coalesce_rule {
    pub max_coalescing_delay: u16,
    pub num_of_fields: u8,
    pub pkt_type: u8,
    pub params: [filt_field_param; MWIFIEX_COALESCE_MAX_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_coalesce_cfg {
    pub num_of_rules: u16,
    pub rule: [mwifiex_coalesce_rule; MWIFIEX_COALESCE_MAX_RULES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_tdls_oper {
    pub tdls_action: u16,
    pub peer_mac: [u8; ETH_ALEN],
    pub capability: u16,
    pub qos_info: u8,
    pub ext_capab: *mut u8,
    pub ext_capab_len: u8,
    pub supp_rates: *mut u8,
    pub supp_rates_len: u8,
    pub ht_capab: *mut u8,
}
