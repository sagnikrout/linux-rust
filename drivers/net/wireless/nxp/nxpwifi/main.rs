//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/nxp/nxpwifi/main.h
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
// nxpwifi: main data structures and prototypes
//
// Copyright 2011-2024 NXP
//

// command type
pub const NXPWIFI_MAX_AP: c_int = 64;
pub const NXPWIFI_MAX_PKTS_TXQ: c_int = 16;

pub const NXPWIFI_TIMER_10S: c_int = 10000;
pub const NXPWIFI_TIMER_1S: c_int = 1000;
pub const MAX_TX_PENDING: c_int = 400;
pub const LOW_TX_PENDING: c_int = 380;
pub const HIGH_RX_PENDING: c_int = 50;
pub const LOW_RX_PENDING: c_int = 20;

pub const MAX_EVENT_SIZE: c_int = 2048;

pub const ARP_FILTER_MAX_BUF_SIZE: c_int = 68;
pub const NXPWIFI_KEY_BUFFER_SIZE: c_int = 16;
pub const NXPWIFI_DEFAULT_LISTEN_INTERVAL: c_int = 10;
pub const NXPWIFI_MAX_REGION_CODE: c_int = 9;
pub const DEFAULT_BCN_AVG_FACTOR: c_int = 8;
pub const DEFAULT_DATA_AVG_FACTOR: c_int = 8;
pub const FIRST_VALID_CHANNEL: c_uint = 0xff;
pub const DEFAULT_BCN_MISS_TIMEOUT: c_int = 5;
pub const MAX_SCAN_BEACON_BUFFER: c_int = 8000;
pub const SCAN_BEACON_ENTRY_PAD: c_int = 6;
pub const NXPWIFI_PASSIVE_SCAN_CHAN_TIME: c_int = 110;
pub const NXPWIFI_ACTIVE_SCAN_CHAN_TIME: c_int = 40;
pub const NXPWIFI_SPECIFIC_SCAN_CHAN_TIME: c_int = 40;
pub const NXPWIFI_DEF_SCAN_CHAN_GAP_TIME: c_int = 50;

pub const WPA_GTK_OUI_OFFSET: c_int = 2;
pub const RSN_GTK_OUI_OFFSET: c_int = 2;
pub const NXPWIFI_OUI_NOT_PRESENT: c_int = 0;
pub const NXPWIFI_OUI_PRESENT: c_int = 1;
pub const PKT_TYPE_MGMT: c_uint = 0xE5;
pub const PKT_TYPE_802DOT11: c_uint = 0x05;
// check if any data / resp / event is received from card

pub const NXPWIFI_TYPE_DATA: c_int = 0;
pub const NXPWIFI_TYPE_CMD: c_int = 1;
pub const NXPWIFI_TYPE_EVENT: c_int = 3;
pub const NXPWIFI_TYPE_VDLL: c_int = 4;
pub const NXPWIFI_TYPE_AGGR_DATA: c_int = 10;
pub const MAX_BITMAP_RATES_SIZE: c_int = 18;
pub const MAX_CHANNEL_BAND_BG: c_int = 14;
pub const MAX_CHANNEL_BAND_A: c_int = 165;
pub const MAX_FREQUENCY_BAND_BG: c_int = 2484;
pub const NXPWIFI_EVENT_HEADER_LEN: c_int = 4;
pub const NXPWIFI_UAP_EVENT_EXTRA_HEADER: c_int = 2;
pub const NXPWIFI_TYPE_LEN: c_int = 4;
pub const NXPWIFI_USB_TYPE_CMD: c_uint = 0xF00DFACE;
pub const NXPWIFI_USB_TYPE_DATA: c_uint = 0xBEADC0DE;
pub const NXPWIFI_USB_TYPE_EVENT: c_uint = 0xBEEFFACE;
// tx_timeout threshold to trigger card reset
pub const TX_TIMEOUT_THRESHOLD: c_int = 6;
pub const NXPWIFI_DRV_INFO_SIZE_MAX: c_uint = 0x40000;
// address alignment helper

pub const NXPWIFI_MAC_LOCAL_ADMIN_BIT: c_int = 41;
// bit helper

// enum nxpwifi_debug_level  -  nxp wifi debug level
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NXPWIFI_DEBUG_LEVEL {
    NXPWIFI_DBG_MSG = 0x00000001,
    NXPWIFI_DBG_FATAL = 0x00000002,
    NXPWIFI_DBG_ERROR = 0x00000004,
    NXPWIFI_DBG_DATA = 0x00000008,
    NXPWIFI_DBG_CMD = 0x00000010,
    NXPWIFI_DBG_EVENT = 0x00000020,
    NXPWIFI_DBG_INTR = 0x00000040,
    NXPWIFI_DBG_IOCTL = 0x00000080,
    NXPWIFI_DBG_MPA_D = 0x00008000,
    NXPWIFI_DBG_DAT_D = 0x00010000,
    NXPWIFI_DBG_CMD_D = 0x00020000,
    NXPWIFI_DBG_EVT_D = 0x00040000,
    NXPWIFI_DBG_FW_D = 0x00080000,
    NXPWIFI_DBG_IF_D = 0x00100000,
    NXPWIFI_DBG_ENTRY = 0x10000000,
    NXPWIFI_DBG_WARN = 0x20000000,
    NXPWIFI_DBG_INFO = 0x40000000,
    NXPWIFI_DBG_DUMP = 0x80000000,
    NXPWIFI_DBG_ANY = 0xffffffff
}

pub const DEBUG_DUMP_DATA_MAX_LEN: c_int = 128;

// Min BGSCAN interval 15 second
pub const NXPWIFI_BGSCAN_INTERVAL: c_int = 15000;
// bgscan interval (ms) and default repeat count
pub const NXPWIFI_BGSCAN_REPEAT_COUNT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_dbg {
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
    pub timeout_cmd_id: u16,
    pub timeout_cmd_act: u16,
    pub last_cmd_id: [u16; DBG_CMD_NUM],
    pub last_cmd_act: [u16; DBG_CMD_NUM],
    pub last_cmd_index: u16,
    pub last_cmd_resp_id: [u16; DBG_CMD_NUM],
    pub last_cmd_resp_index: u16,
    pub last_event: [u16; DBG_CMD_NUM],
    pub last_event_index: u16,
    pub last_mp_wr_bitmap: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_mp_wr_ports: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_mp_wr_len: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_mp_curr_wr_port: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_sdio_mp_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NXPWIFI_HARDWARE_STATUS {
    NXPWIFI_HW_STATUS_READY,
    NXPWIFI_HW_STATUS_INITIALIZING,
    NXPWIFI_HW_STATUS_RESET,
    NXPWIFI_HW_STATUS_NOT_READY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NXPWIFI_802_11_POWER_MODE {
    NXPWIFI_802_11_POWER_MODE_CAM,
    NXPWIFI_802_11_POWER_MODE_PSP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_tx_param {
    pub next_pkt_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NXPWIFI_PS_STATE {
    PS_STATE_AWAKE,
    PS_STATE_PRE_SLEEP,
    PS_STATE_SLEEP_CFM,
    PS_STATE_SLEEP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_iface_type {
    NXPWIFI_SDIO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_add_ba_param {
    pub tx_win_size: u32,
    pub rx_win_size: u32,
    pub timeout: u32,
    pub tx_amsdu: u8,
    pub rx_amsdu: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_tx_aggr {
    pub ampdu_user: u8,
    pub ampdu_ap: u8,
    pub amsdu: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_ba_status {
    BA_SETUP_NONE = 0,
    BA_SETUP_INPROGRESS,
    BA_SETUP_COMPLETE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ra_list_tbl {
    pub list: list_head,
    pub skb_head: sk_buff_head,
    pub ra: [u8; ETH_ALEN],
    pub is_11n_enabled: u32,
    pub max_amsdu: u16,
    pub ba_pkt_count: u16,
    pub ba_packet_thr: u8,
    pub ba_status: nxpwifi_ba_status,
    pub amsdu_in_ampdu: u8,
    pub total_pkt_count: u16,
    pub tx_paused: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_tid_tbl {
    pub ra_list: list_head,
}

pub const WMM_HIGHEST_PRIORITY: c_int = 7;
pub const HIGH_PRIO_TID: c_int = 7;
pub const LOW_PRIO_TID: c_int = 0;

pub const NXPWIFI_WMM_DRV_DELAY_MAX: c_int = 510;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_wmm_desc {
    pub tid_tbl_ptr: [nxpwifi_tid_tbl; MAX_NUM_TID],
    pub packets_out: [u32; MAX_NUM_TID],
    pub pkts_paused: [u32; MAX_NUM_TID],
// protects ra_list
    pub ra_list_spinlock: spinlock_t,
    pub ac_status: [nxpwifi_wmm_ac_status; IEEE80211_NUM_ACS],
    pub ac_down_graded_vals: [nxpwifi_wmm_ac_e; IEEE80211_NUM_ACS],
    pub drv_pkt_delay_max: u32,
    pub queue_priority: [u8; IEEE80211_NUM_ACS],
    pub /: *mut *mut u32 user_pri_pkt_tx_ctrl[WMM_HIGHEST_PRIORITY + 1]; / UP: 0 to 7,
// number of queued TX packets
    pub tx_pkts_queued: core::sync::atomic::AtomicI32,
// highest priority currently queued
    pub highest_queued_prio: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_802_11_security {
    pub wpa_enabled: u8,
    pub wpa2_enabled: u8,
    pub wep_enabled: u8,
    pub authentication_mode: u32,
    pub is_authtype_auto: u8,
    pub encryption_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_vendor_specific {
    pub vend_hdr: ieee80211_vendor_ie,
    pub ieee80211_vendor_ie)]: u8 data[IEEE_MAX_IE_SIZE - sizeof(struct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_bssdescriptor {
    pub mac_address: [u8; ETH_ALEN],
    pub ssid: cfg80211_ssid,
    pub privacy: u32,
    pub rssi: i32,
    pub channel: u32,
    pub freq: u32,
    pub beacon_period: u16,
    pub erp_flags: u8,
    pub bss_mode: u32,
    pub supported_rates: [u8; NXPWIFI_SUPPORTED_RATES],
    pub data_rates: [u8; NXPWIFI_SUPPORTED_RATES],
    pub bss_band: u16,
    pub fw_tsf: u64,
    pub timestamp: u64,
    pub phy_param_set: ieee_types_phy_param_set,
    pub cf_param_set: ieee_types_cf_param_set,
    pub cap_info_bitmap: u16,
    pub wmm_ie: ieee80211_wmm_param_ie,
    pub disable_11n: u8,
    pub bcn_ht_cap: *mut ieee80211_ht_cap,
    pub ht_cap_offset: u16,
    pub bcn_ht_oper: *mut ieee80211_ht_operation,
    pub ht_info_offset: u16,
    pub bcn_bss_co_2040: *mut u8,
    pub bss_co_2040_offset: u16,
    pub bcn_ext_cap: *mut u8,
    pub ext_cap_offset: u16,
    pub bcn_vht_cap: *mut ieee80211_vht_cap,
    pub vht_cap_offset: u16,
    pub bcn_vht_oper: *mut ieee80211_vht_operation,
    pub vht_info_offset: u16,
    pub oper_mode: *mut ieee_types_oper_mode_ntf,
    pub oper_mode_offset: u16,
    pub disable_11ac: u8,
    pub bcn_he_cap: *mut ieee80211_he_cap_elem,
    pub he_cap_offset: u16,
    pub bcn_he_oper: *mut ieee80211_he_operation,
    pub he_info_offset: u16,
    pub disable_11ax: u8,
    pub bcn_wpa_ie: *mut ieee_types_vendor_specific,
    pub wpa_offset: u16,
    pub bcn_rsn_ie: *mut element,
    pub rsn_offset: u16,
    pub bcn_rsnx_ie: *mut element,
    pub rsnx_offset: u16,
    pub beacon_buf: *mut u8,
    pub beacon_buf_size: u32,
    pub sensed_11h: u8,
    pub local_constraint: u8,
    pub chan_sw_ie_present: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_current_bss_params {
    pub bss_descriptor: nxpwifi_bssdescriptor,
    pub wmm_enabled: bool,
    pub wmm_uapsd_enabled: bool,
    pub band: u8,
    pub num_of_rates: u32,
    pub data_rates: [u8; NXPWIFI_SUPPORTED_RATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_sleep_period {
    pub period: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_wep_key {
    pub length: u32,
    pub key_index: u32,
    pub key_length: u32,
    pub key_material: [u8; NXPWIFI_KEY_BUFFER_SIZE],
}

pub const MAX_REGION_CHANNEL_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_chan_freq_power {
    pub channel: u16,
    pub freq: u32,
    pub max_tx_power: u16,
    pub unsupported: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum state_11d_t {
    DISABLE_11D = 0,
    ENABLE_11D = 1,
}

pub const NXPWIFI_MAX_TRIPLET_802_11D: c_int = 83;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_802_11d_domain_reg {
    pub dfs_region: u8,
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub no_of_triplet: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_vendor_spec_cfg_ie {
    pub mask: u16,
    pub flag: u16,
    pub ie: [u8; NXPWIFI_MAX_VSIE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wps {
    pub session_enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_roc_cfg {
    pub cookie: u64,
    pub chan: ieee80211_channel,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_iface_work_flags {
    NXPWIFI_IFACE_WORK_DEVICE_DUMP,
    NXPWIFI_IFACE_WORK_CARD_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_adapter_work_flags {
    NXPWIFI_SURPRISE_REMOVED,
    NXPWIFI_IS_CMD_TIMEDOUT,
    NXPWIFI_IS_SUSPENDED,
    NXPWIFI_IS_HS_CONFIGURED,
    NXPWIFI_IS_HS_ENABLING,
    NXPWIFI_IS_REQUESTING_FW_VEREXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_band_config {
    pub chan_band:2: u8,
    pub chan_width:2: u8,
    pub chan2_offset:2: u8,
    pub scan_mode:2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_channel_band {
    pub band_config: nxpwifi_band_config,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_private {
    pub adapter: *mut nxpwifi_adapter,
    pub bss_type: u8,
    pub bss_role: u8,
    pub bss_priority: u8,
    pub bss_num: u8,
    pub bss_started: u8,
    pub auth_flag: u8,
    pub auth_alg: u16,
    pub frame_type: u8,
    pub curr_addr: [u8; ETH_ALEN],
    pub media_connected: u8,
    pub port_open: u8,
    pub usb_port: u8,
    pub num_tx_timeout: u32,
// track consecutive timeout
    pub tx_timeout_cnt: u8,
    pub netdev: *mut net_device,
    pub stats: net_device_stats,
    pub curr_pkt_filter: u32,
    pub bss_mode: u32,
    pub pkt_tx_ctrl: u32,
    pub tx_power_level: u16,
    pub max_tx_power_level: u8,
    pub min_tx_power_level: u8,
    pub tx_ant: u32,
    pub rx_ant: u32,
    pub tx_rate: u8,
    pub tx_htinfo: u8,
    pub rxpd_htinfo: u8,
    pub rxpd_rate: u8,
    pub rate_bitmap: u16,
    pub bitmap_rates: [u16; MAX_BITMAP_RATES_SIZE],
    pub data_rate: u32,
    pub is_data_rate_auto: u8,
    pub bcn_avg_factor: u16,
    pub data_avg_factor: u16,
    pub data_rssi_last: i16,
    pub data_nf_last: i16,
    pub data_rssi_avg: i16,
    pub data_nf_avg: i16,
    pub bcn_rssi_last: i16,
    pub bcn_nf_last: i16,
    pub bcn_rssi_avg: i16,
    pub bcn_nf_avg: i16,
    pub attempted_bss_desc: *mut nxpwifi_bssdescriptor,
    pub prev_ssid: cfg80211_ssid,
    pub prev_bssid: [u8; ETH_ALEN],
    pub curr_bss_params: nxpwifi_current_bss_params,
    pub beacon_period: u16,
    pub dtim_period: u8,
    pub listen_interval: u16,
    pub atim_window: u16,
    pub sec_info: nxpwifi_802_11_security,
    pub wep_key: [nxpwifi_wep_key; NUM_WEP_KEYS],
    pub wep_key_curr_index: u16,
    pub wpa_ie: [u8; 256],
    pub wpa_ie_len: u16,
    pub wpa_is_gtk_set: u8,
    pub aes_key: host_cmd_ds_802_11_key_material,
    pub wps_ie: *mut u8,
    pub wps_ie_len: u16,
    pub wmm_required: u8,
    pub wmm_enabled: bool,
    pub wmm_qosinfo: u8,
    pub wmm: nxpwifi_wmm_desc,
    pub wmm_tx_pending: [core::sync::atomic::AtomicI32; IEEE80211_NUM_ACS],
    pub sta_list: list_head,
// spin lock for associated station list
    pub sta_list_spinlock: spinlock_t,
    pub tx_ba_stream_tbl_ptr: [list_head; MAX_NUM_TID],
// spin lock for tx_ba_stream_tbl_ptr queue
    pub tx_ba_stream_tbl_lock: [spinlock; MAX_NUM_TID],
    pub aggr_prio_tbl: [nxpwifi_tx_aggr; MAX_NUM_TID],
    pub add_ba_param: nxpwifi_add_ba_param,
    pub rx_seq: [u16; MAX_NUM_TID],
    pub tos_to_tid_inv: [u8; MAX_NUM_TID],
    pub rx_reorder_tbl_ptr: [list_head; MAX_NUM_TID],
// spin lock for rx_reorder_tbl_ptr queue
    pub rx_reorder_tbl_lock: [spinlock; MAX_NUM_TID],
pub const NXPWIFI_ASSOC_RSP_BUF_SIZE: c_int = 500;
    pub assoc_rsp_buf: [u8; NXPWIFI_ASSOC_RSP_BUF_SIZE],
    pub assoc_rsp_size: u32,
    pub req_bss: *mut cfg80211_bss,
pub const NXPWIFI_GENIE_BUF_SIZE: c_int = 256;
    pub gen_ie_buf: [u8; NXPWIFI_GENIE_BUF_SIZE],
    pub gen_ie_buf_len: u8,
    pub vs_ie: [nxpwifi_vendor_spec_cfg_ie; NXPWIFI_MAX_VSIE_NUM],
pub const NXPWIFI_ASSOC_TLV_BUF_SIZE: c_int = 256;
    pub assoc_tlv_buf: [u8; NXPWIFI_ASSOC_TLV_BUF_SIZE],
    pub assoc_tlv_buf_len: u8,
    pub curr_bcn_buf: *mut u8,
    pub curr_bcn_size: u32,
// spin lock for beacon buffer
    pub curr_bcn_buf_lock: spinlock_t,
    pub wdev: wireless_dev,
    pub cfp: nxpwifi_chan_freq_power,
    pub versionstrsel: u32,
    pub version_str: [c_char; NXPWIFI_VERSION_STR_LENGTH],
    pub dfs_dev_dir: *mut dentry,

    pub current_key_index: u16,
    pub scan_request: *mut cfg80211_scan_request,
    pub cfg_bssid: [u8; 6],
    pub wps: wps,
    pub scan_block: u8,
    pub cqm_rssi_thold: i32,
    pub cqm_rssi_hyst: u32,
    pub subsc_evt_rssi_state: u8,
    pub async_subsc_evt_storage: nxpwifi_ds_misc_subsc_evt,
    pub mgmt_ie: [nxpwifi_ie; MAX_MGMT_IE_INDEX],
    pub beacon_idx: u16,
    pub proberesp_idx: u16,
    pub assocresp_idx: u16,
    pub gen_idx: u16,
    pub ap_11n_enabled: u8,
    pub ap_11ac_enabled: u8,
    pub ap_11ax_enabled: u8,
    pub config_bands: u16,
// 11AX
    pub user_he_cap_len: u8,
    pub user_he_cap: [u8; HE_CAP_MAX_SIZE],
    pub user_2g_he_cap_len: u8,
    pub user_2g_he_cap: [u8; HE_CAP_MAX_SIZE],
    pub host_mlme_reg: bool,
    pub mgmt_frame_mask: u32,
    pub roc_cfg: nxpwifi_roc_cfg,
    pub scan_aborting: bool,
    pub sched_scanning: u8,
    pub csa_chan: u8,
    pub csa_expire_time: c_ulong,
    pub del_list_idx: u8,
    pub hs2_enabled: bool,
    pub bss_cfg: nxpwifi_uap_bss_param,
    pub bss_chandef: cfg80211_chan_def,
    pub sta_params: *mut station_parameters,
    pub ack_status_frames: xarray,
// spin lock for ack status
    pub ack_status_lock: spinlock_t,
// rx histogram data
    pub hist_data: *mut nxpwifi_histogram_data,
    pub dfs_chandef: cfg80211_chan_def,
    pub reset_conn_state_work: wiphy_work,
    pub dfs_cac_work: wiphy_delayed_work,
    pub dfs_chan_sw_work: wiphy_delayed_work,
    pub uap_stop_tx: bool,
    pub ap_update_info: cfg80211_ap_update,
    pub state_11h: nxpwifi_11h_intf_state,
    pub mem_rw: nxpwifi_ds_mem_rw,
    pub bypass_txq: sk_buff_head,
    pub hidden_chan: [nxpwifi_user_scan_chan; NXPWIFI_USER_SCAN_CHAN_MAX],
    pub assoc_resp_ht_param: u8,
    pub ht_param_present: bool,
    pub last_deauth_reason: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_tx_ba_stream_tbl {
    pub list: list_head,
    pub rcu: rcu_head,
    pub tid: c_int,
    pub ra: [u8; ETH_ALEN],
    pub ba_status: nxpwifi_ba_status,
    pub amsdu: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reorder_tmr_cnxt {
    pub timer: timer_list,
    pub ptr: *mut nxpwifi_rx_reorder_tbl,
    pub priv: *mut nxpwifi_private,
    pub timer_is_set: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_rx_reorder_tbl {
    pub list: list_head,
    pub tmp_list: list_head,
    pub rcu: rcu_head,
    pub tid: c_int,
    pub ta: [u8; ETH_ALEN],
    pub init_win: c_int,
    pub start_win: c_int,
    pub win_size: c_int,
    pub timer_context: reorder_tmr_cnxt,
    pub amsdu: u8,
    pub flags: u8,
    pub __counted_by(win_size): *mut *mut sk_buff rx_reorder_ptr[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_bss_prio_node {
    pub list: list_head,
    pub priv: *mut nxpwifi_private,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_bss_prio_tbl {
    pub bss_prio_head: list_head,
    pub /: *mut *mut spinlock_t bss_prio_lock; / protects BSS priority,
    pub bss_prio_cur: *mut nxpwifi_bss_prio_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ctrl_node {
    pub list: list_head,
    pub priv: *mut nxpwifi_private,
    pub cmd_no: u32,
    pub cmd_flag: u32,
    pub cmd_skb: *mut sk_buff,
    pub resp_skb: *mut sk_buff,
    pub data_buf: *mut c_void,
    pub wait_q_enabled: u32,
    pub skb: *mut sk_buff,
    pub condition: *mut u8,
    pub cmd_wait_q_woken: u8,
    pub data_buf): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_bss_priv {
    pub band: u16,
    pub fw_tsf: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_station_stats {
    pub last_rx: u64,
    pub rssi: i8,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub tx_failed: u32,
    pub last_tx_rate: u8,
    pub last_tx_htinfo: u8,
}

// AP - side structure tracking associated STA info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_sta_node {
    pub list: list_head,
    pub rcu: rcu_head,
    pub mac_addr: [u8; ETH_ALEN],
    pub is_wmm_enabled: u8,
    pub is_11n_enabled: u8,
    pub is_11ac_enabled: u8,
    pub is_11ax_enabled: u8,
    pub ampdu_sta: [u8; MAX_NUM_TID],
    pub rx_seq: [u16; MAX_NUM_TID],
    pub max_amsdu: u16,
    pub stats: nxpwifi_station_stats,
    pub tx_pause: u8,
}

pub const NXPWIFI_TYPE_AGGR_DATA_V2: c_int = 11;

pub const NXPWIFI_BUS_AGGR_MAX_LEN: c_int = 16000;
pub const NXPWIFI_BUS_AGGR_MAX_NUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_aggr_params {
    pub enable: u16,
    pub mode: u16,
    pub tx_aggr_max_size: u16,
    pub tx_aggr_max_num: u16,
    pub tx_aggr_align: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdll_dnld_ctrl {
    pub pending_block: *mut u8,
    pub pending_block_len: u16,
    pub vdll_mem: *mut u8,
    pub vdll_len: u32,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_if_ops {
    pub adapter): *mut *mut int (init_if)(struct nxpwifi_adapter,
    pub adapter): *mut *mut void (cleanup_if)(struct nxpwifi_adapter,
    pub poll_num): *mut *mut *mut int (check_fw_status)(struct nxpwifi_adapter adapter, u32,
    pub adapter): *mut *mut int (check_winner_status)(struct nxpwifi_adapter,
    pub fw): *mut nxpwifi_fw_image,
    pub adapter): *mut *mut int (register_dev)(struct nxpwifi_adapter,
    pub adapter): *mut *mut void (unregister_dev)(struct nxpwifi_adapter,
    pub adapter): *mut *mut int (enable_int)(struct nxpwifi_adapter,
    pub adapter): *mut *mut void (disable_int)(struct nxpwifi_adapter,
    pub istat): *mut *mut *mut int (process_int_status)(struct nxpwifi_adapter adapter, u8,
    pub tx_param): *mut nxpwifi_tx_param,
    pub adapter): *mut *mut int (wakeup)(struct nxpwifi_adapter,
    pub adapter): *mut *mut int (wakeup_complete)(struct nxpwifi_adapter,
// interface-specific operations
    pub port): *mut *mut *mut void (update_mp_end_port)(struct nxpwifi_adapter adapter, u16,
    pub adapter): *mut *mut void (cleanup_mpa_buf)(struct nxpwifi_adapter,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub fw): *mut nxpwifi_fw_image,
    pub adapter): *mut *mut void (card_reset)(struct nxpwifi_adapter,
    pub drv_buf): *mut *mut *mut int (reg_dump)(struct nxpwifi_adapter adapter, char,
    pub adapter): *mut *mut void (device_dump)(struct nxpwifi_adapter,
    pub skb): *mut sk_buff,
    pub adapter): *mut *mut void (up_dev)(struct nxpwifi_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_adapter {
    pub iface_type: u8,
    pub debug_mask: c_uint,
    pub iface_limit: nxpwifi_iface_comb,
    pub curr_iface_comb: nxpwifi_iface_comb,
    pub priv: [*mut nxpwifi_private; NXPWIFI_MAX_BSS_NUM],
    pub priv_num: u8,
    pub firmware: *const firmware,
    pub fw_name: [c_char; 32],
    pub winner: c_int,
    pub dev: *mut device,
    pub wiphy: *mut wiphy,
    pub perm_addr: [u8; ETH_ALEN],
    pub work_flags: c_ulong,
    pub fw_release_number: u32,
    pub intf_hdr_len: u8,
    pub card: *mut c_void,
    pub if_ops: nxpwifi_if_ops,
    pub bypass_tx_pending: core::sync::atomic::AtomicI32,
    pub rx_pending: core::sync::atomic::AtomicI32,
    pub tx_pending: core::sync::atomic::AtomicI32,
    pub cmd_pending: core::sync::atomic::AtomicI32,
    pub tx_hw_pending: core::sync::atomic::AtomicI32,
    pub workqueue: *mut workqueue_struct,
    pub main_work: work_struct,
    pub rx_workqueue: *mut workqueue_struct,
    pub rx_work: work_struct,
    pub host_mlme_work: wiphy_work,
    pub rx_work_enabled: bool,
    pub rx_processing: bool,
    pub delay_main_work: bool,
    pub rx_ba_teardown_pending: core::sync::atomic::AtomicI32,
    pub iface_changing: core::sync::atomic::AtomicI32,
    pub bss_prio_tbl: [nxpwifi_bss_prio_tbl; NXPWIFI_MAX_BSS_NUM],
    pub nxpwifi_processing: u32,
    pub tx_buf_size: u16,
    pub curr_tx_buf_size: u16,
// SDIO single port rx aggregation capability
    pub host_disable_sdio_rx_aggr: bool,
    pub sdio_rx_aggr_enable: bool,
    pub sdio_rx_block_size: u16,
    pub ioport: u32,
    pub hw_status: NXPWIFI_HARDWARE_STATUS,
    pub number_of_antenna: u16,
    pub fw_cap_info: u32,
    pub fw_cap_ext: u32,
    pub user_htstream: u16,
    pub uuid_lo: u64,
    pub uuid_hi: u64,
// interrupt lock
    pub int_lock: spinlock_t,
    pub int_status: u8,
    pub event_cause: u32,
    pub event_skb: *mut sk_buff,
    pub upld_buf: [u8; NXPWIFI_UPLD_SIZE],
    pub data_sent: u8,
    pub cmd_sent: u8,
    pub cmd_resp_received: u8,
    pub event_received: bool,
    pub data_received: u8,
    pub assoc_resp_received: u8,
    pub priv_link_lost: *mut nxpwifi_private,
    pub host_mlme_link_lost: u8,
    pub seq_num: u16,
    pub cmd_pool: *mut cmd_ctrl_node,
    pub curr_cmd: *mut cmd_ctrl_node,
// spin lock for command
    pub nxpwifi_cmd_lock: spinlock_t,
    pub cmd_timer: timer_list,
    pub cmd_free_q: list_head,
    pub /: *mut *mut spinlock_t cmd_free_q_lock; / protects cmd_free_q,
    pub cmd_pending_q: list_head,
    pub /: *mut *mut spinlock_t cmd_pending_q_lock; / protects cmd_pending_q,
    pub scan_pending_q: list_head,
    pub /: *mut *mut spinlock_t scan_pending_q_lock; / protects scan_pending_q,
    pub tx_data_q: sk_buff_head,
    pub tx_queued: core::sync::atomic::AtomicI32,
    pub scan_processing: u32,
    pub region_code: nxpwifi_region_code,
    pub domain_reg: nxpwifi_802_11d_domain_reg,
    pub scan_probes: u16,
    pub scan_mode: u32,
    pub specific_scan_time: u16,
    pub active_scan_time: u16,
    pub passive_scan_time: u16,
    pub scan_chan_gap_time: u16,
    pub fw_bands: u16,
    pub tx_lock_flag: u8,
    pub sleep_period: nxpwifi_sleep_period,
    pub ps_mode: u16,
    pub ps_state: u32,
    pub need_to_wakeup: u8,
    pub multiple_dtim: u16,
    pub local_listen_interval: u16,
    pub null_pkt_interval: u16,
    pub sleep_cfm: *mut sk_buff,
    pub bcn_miss_time_out: u16,
    pub is_deep_sleep: u8,
    pub delay_null_pkt: u8,
    pub delay_to_ps: u16,
    pub enhanced_ps_mode: u16,
    pub pm_wakeup_card_req: u8,
    pub gen_null_pkt: u16,
    pub pps_uapsd_mode: u16,
    pub pm_wakeup_fw_try: u32,
    pub wakeup_timer: timer_list,
    pub hs_cfg: nxpwifi_hs_config_param,
    pub hs_activated: u8,
    pub hs_activated_manually: u8,
    pub hs_activate_wait_q_woken: u16,
    pub hs_activate_wait_q: wait_queue_head_t,
    pub event_body: [u8; MAX_EVENT_SIZE],
    pub hw_dot_11n_dev_cap: u32,
    pub hw_dev_mcs_support: u8,
    pub hw_mpdu_density: u8,
    pub user_dev_mcs_support: u8,
    pub sec_chan_offset: u8,
    pub dbg: nxpwifi_dbg,
    pub arp_filter: [u8; ARP_FILTER_MAX_BUF_SIZE],
    pub arp_filter_size: u32,
    pub cmd_wait_q: nxpwifi_wait_queue,
    pub scan_wait_q_woken: u8,
    pub /: *mut *mut spinlock_t queue_lock; / protects TX queues,
    pub dfs_region: u8,
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub max_mgmt_ie_index: u16,
    pub cal_data: *const firmware,
// 11AC capability fields
    pub is_hw_11ac_capable: u32,
    pub hw_dot_11ac_dev_cap: u32,
    pub hw_dot_11ac_mcs_support: u32,
    pub usr_dot_11ac_dev_cap_bg: u32,
    pub usr_dot_11ac_dev_cap_a: u32,
    pub usr_dot_11ac_mcs_support: u32,
// 11AX capability fields
    pub is_hw_11ax_capable: u8,
    pub hw_he_cap_len: u8,
    pub hw_he_cap: [u8; HE_CAP_MAX_SIZE],
    pub hw_2g_he_cap_len: u8,
    pub hw_2g_he_cap: [u8; HE_CAP_MAX_SIZE],
    pub pending_bridged_pkts: core::sync::atomic::AtomicI32,
    pub /: *mut *mut *mut completion fw_done; / FW init completion,
    pub is_up: bool,
    pub ext_scan: bool,
    pub fw_api_ver: u8,
    pub fw_hotfix_ver: u8,
    pub key_api_minor_ver: u8 key_api_major_ver,,
    pub max_sta_conn: u8,
    pub mem_type_mapping_tbl: *mut memory_type_mapping,
    pub num_mem_types: u8,
    pub scan_chan_gap_enabled: bool,
    pub rx_mlme_q: sk_buff_head,
    pub rx_data_q: sk_buff_head,
    pub chan_stats: *mut nxpwifi_chan_stats,
    pub num_in_chan_stats: u32,
    pub survey_idx: c_int,
    pub coex_scan: u8,
    pub coex_min_scan_time: u8,
    pub coex_max_scan_time: u8,
    pub coex_win_size: u8,
    pub coex_tx_win_size: u8,
    pub coex_rx_win_size: u8,
    pub active_scan_triggered: u8,
    pub usb_mc_status: bool,
    pub usb_mc_setup: bool,
    pub nd_info: *mut cfg80211_wowlan_nd_info,
    pub regd: *mut ieee80211_regdomain,
// Aggregation parameters
    pub bus_aggr: bus_aggr_params,
    pub /: *mut *mut *mut void devdump_data; / device dump storage,
    pub /: *mut *mut int devdump_len; / device dump length,
    pub ignore_btcoex_events: bool,
    pub vdll_ctrl: vdll_dnld_ctrl,
    pub roc_cookie_counter: u64,
    pub enable_net_mon: u32,
    pub wowlan_enabled: bool,
    pub chandef_valid: bool,
    pub chandef: cfg80211_chan_def,
    pub uap_count: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn nxpwifi_process_tx_queue(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_init_lock_list(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_set_trans_start(dev: *mut net_device);
}
extern "C" {
    pub fn nxpwifi_init_priv(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_free_priv(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_init_fw(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_shutdown_drv(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_recv_packet(priv: *mut nxpwifi_private, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nxpwifi_cmd_timeout_func(t: *mut timer_list);
}
extern "C" {
    pub fn nxpwifi_alloc_cmd_buffer(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_free_cmd_buffer(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_free_cmd_buffers(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_cancel_all_pending_cmd(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_cancel_pending_scan_cmd(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_cancel_scan(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_exec_next_cmd(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_process_cmdresp(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_process_assoc_resp(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_send_null_packet(priv: *mut nxpwifi_private, flags: u8) -> c_int;
}
extern "C" {
    pub fn nxpwifi_clean_txrx(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_check_last_packet_indication(priv: *mut nxpwifi_private) -> u8;
}
extern "C" {
    pub fn nxpwifi_check_ps_cond(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_process_hs_config(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_delete_all_station_list(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_band_to_radio_type(config_bands: u16) -> u8;
}
extern "C" {
    pub fn nxpwifi_deauthenticate(priv: *mut nxpwifi_private, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn nxpwifi_deauthenticate_all(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_cmd_802_11_bg_scan_query(cmd: *mut host_cmd_ds_command) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_supported_rates(priv: *mut nxpwifi_private, rates: *mut u8) -> u32;
}
extern "C" {
    pub fn nxpwifi_is_rate_auto(priv: *mut nxpwifi_private) -> u8;
}
extern "C" {
    pub fn nxpwifi_save_curr_bcn(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_free_curr_bcn(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_is_command_pending(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_ba_params(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_update_ampdu_txwinsize(pmadapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_set_11ac_ba_params(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_stop_bg_scan(priv: *mut nxpwifi_private) -> c_int;
}
// check if RA-based queuing
// In STA mode DA==RA; subject to future revision
// copy rates from src to dest
// return priv matching the given BSS type and number
// return first priv matching BSS role
// find unused BSS number for new interface
// bss_num = j;
// return unused private entry for requested bss type
// return private structure attached to netdev
// return true if skb contains a management frame
// channel closed by CSA
// clear CSA if DFS switch timeout expired
extern "C" {
    pub fn nxpwifi_remove_card(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_cancel_hs(priv: *mut nxpwifi_private, cmd_type: c_int) -> c_int;
}
extern "C" {
    pub fn nxpwifi_enable_hs(adapter: *mut nxpwifi_adapter) -> bool;
}
extern "C" {
    pub fn nxpwifi_disable_auto_ds(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_drv_get_data_rate(priv: *mut nxpwifi_private, rate: *mut u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_radio(priv: *mut nxpwifi_private, option: u8) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_gen_ie(priv: *mut nxpwifi_private, ie: *const u8, ie_len: c_int) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_ver_ext(priv: *mut nxpwifi_private, version_str_sel: u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_11n_httx_cfg(priv: *mut nxpwifi_private, data: c_int) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_11n_httx_cfg(priv: *mut nxpwifi_private, data: *mut c_int) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_tx_rate_cfg(priv: *mut nxpwifi_private, tx_rate_index: c_int) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_tx_rate_cfg(priv: *mut nxpwifi_private, tx_rate_index: *mut c_int) -> c_int;
}
extern "C" {
    pub fn nxpwifi_drv_set_power(priv: *mut nxpwifi_private, ps_mode: *mut u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_main_process(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_queue_tx_pkt(priv: *mut nxpwifi_private, skb: *mut sk_buff);
}
extern "C" {
    pub fn nxpwifi_chan_type_to_sec_chan_offset(chan_type: nl80211_channel_type) -> u8;
}
extern "C" {
    pub fn nxpwifi_get_chan_type(priv: *mut nxpwifi_private) -> u8;
}
extern "C" {
    pub fn nxpwifi_del_virtual_intf(wiphy: *mut wiphy, wdev: *mut wireless_dev) -> c_int;
}
extern "C" {
    pub fn nxpwifi_add_wowlan_magic_pkt_filter(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_del_mgmt_ies(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_init_11h_params(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_is_11h_active(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_11h_activate(priv: *mut nxpwifi_private, flag: bool) -> c_int;
}
extern "C" {
    pub fn nxpwifi_11h_handle_event_chanswann(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_del_all_sta_list(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_del_sta_entry(priv: *mut nxpwifi_private, mac: *const u8);
}
extern "C" {
    pub fn nxpwifi_init_channel_scan_gap(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_reset_conn_state_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn nxpwifi_dfs_cac_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn nxpwifi_dfs_chan_sw_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn nxpwifi_abort_cac(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_hist_data_reset(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_drv_info_dump(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_prepare_fw_dump_info(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_upload_device_dump(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_fw_dump_event(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_coex_ampdu_rxwinsize(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_11n_delba(priv: *mut nxpwifi_private, tid: c_int);
}
extern "C" {
    pub fn nxpwifi_send_domain_info_cmd_fw(wiphy: *mut wiphy, band: nl80211_band) -> c_int;
}
extern "C" {
    pub fn nxpwifi_devdump_tmo_func(function_context: c_ulong);
}

extern "C" {
    pub fn nxpwifi_debugfs_init();
}
extern "C" {
    pub fn nxpwifi_debugfs_remove();
}
extern "C" {
    pub fn nxpwifi_dev_debugfs_init(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_dev_debugfs_remove(priv: *mut nxpwifi_private);
}

extern "C" {
    pub fn nxpwifi_reinit_sw(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_shutdown_sw(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_is_valid_region_code(code: nxpwifi_region_code) -> bool;
}
