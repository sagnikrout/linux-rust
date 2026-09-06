//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/main.h
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
// NXP Wireless LAN device driver: major data structures and prototypes
//
// Copyright 2011-2020 NXP
//

pub const MWIFIEX_MAX_AP: c_int = 64;
pub const MWIFIEX_MAX_PKTS_TXQ: c_int = 16;

pub const MWIFIEX_TIMER_10S: c_int = 10000;
pub const MWIFIEX_TIMER_1S: c_int = 1000;
pub const MAX_TX_PENDING: c_int = 400;
pub const LOW_TX_PENDING: c_int = 380;
pub const HIGH_RX_PENDING: c_int = 50;
pub const LOW_RX_PENDING: c_int = 20;

pub const MAX_EVENT_SIZE: c_int = 2048;

pub const ARP_FILTER_MAX_BUF_SIZE: c_int = 68;
pub const MWIFIEX_KEY_BUFFER_SIZE: c_int = 16;
pub const MWIFIEX_DEFAULT_LISTEN_INTERVAL: c_int = 10;
pub const MWIFIEX_MAX_REGION_CODE: c_int = 9;
pub const DEFAULT_BCN_AVG_FACTOR: c_int = 8;
pub const DEFAULT_DATA_AVG_FACTOR: c_int = 8;
pub const FIRST_VALID_CHANNEL: c_uint = 0xff;
pub const DEFAULT_AD_HOC_CHANNEL: c_int = 6;
pub const DEFAULT_AD_HOC_CHANNEL_A: c_int = 36;
pub const DEFAULT_BCN_MISS_TIMEOUT: c_int = 5;
pub const MAX_SCAN_BEACON_BUFFER: c_int = 8000;
pub const SCAN_BEACON_ENTRY_PAD: c_int = 6;
pub const MWIFIEX_PASSIVE_SCAN_CHAN_TIME: c_int = 110;
pub const MWIFIEX_ACTIVE_SCAN_CHAN_TIME: c_int = 40;
pub const MWIFIEX_SPECIFIC_SCAN_CHAN_TIME: c_int = 40;
pub const MWIFIEX_DEF_SCAN_CHAN_GAP_TIME: c_int = 50;

pub const WPA_GTK_OUI_OFFSET: c_int = 2;
pub const RSN_GTK_OUI_OFFSET: c_int = 2;
pub const MWIFIEX_OUI_NOT_PRESENT: c_int = 0;
pub const MWIFIEX_OUI_PRESENT: c_int = 1;
pub const PKT_TYPE_MGMT: c_uint = 0xE5;
//
// Do not check for data_received for USB, as data_received
// is handled in mwifiex_usb_recv for USB
//

pub const MWIFIEX_TYPE_CMD: c_int = 1;
pub const MWIFIEX_TYPE_DATA: c_int = 0;
pub const MWIFIEX_TYPE_AGGR_DATA: c_int = 10;
pub const MWIFIEX_TYPE_EVENT: c_int = 3;
pub const MAX_BITMAP_RATES_SIZE: c_int = 18;
pub const MAX_CHANNEL_BAND_BG: c_int = 14;
pub const MAX_CHANNEL_BAND_A: c_int = 165;
pub const MAX_FREQUENCY_BAND_BG: c_int = 2484;
pub const MWIFIEX_EVENT_HEADER_LEN: c_int = 4;
pub const MWIFIEX_UAP_EVENT_EXTRA_HEADER: c_int = 2;
pub const MWIFIEX_TYPE_LEN: c_int = 4;
pub const MWIFIEX_USB_TYPE_CMD: c_uint = 0xF00DFACE;
pub const MWIFIEX_USB_TYPE_DATA: c_uint = 0xBEADC0DE;
pub const MWIFIEX_USB_TYPE_EVENT: c_uint = 0xBEEFFACE;
// Threshold for tx_timeout_cnt before we trigger a card reset
pub const TX_TIMEOUT_THRESHOLD: c_int = 6;
pub const MWIFIEX_DRV_INFO_SIZE_MAX: c_uint = 0x40000;
// Address alignment

pub const MWIFIEX_MAC_LOCAL_ADMIN_BIT: c_int = 41;
//
// enum mwifiex_debug_level  -  marvell wifi debug level
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MWIFIEX_DEBUG_LEVEL {
    MWIFIEX_DBG_MSG		= 0x00000001,
    MWIFIEX_DBG_FATAL	= 0x00000002,
    MWIFIEX_DBG_ERROR	= 0x00000004,
    MWIFIEX_DBG_DATA	= 0x00000008,
    MWIFIEX_DBG_CMD		= 0x00000010,
    MWIFIEX_DBG_EVENT	= 0x00000020,
    MWIFIEX_DBG_INTR	= 0x00000040,
    MWIFIEX_DBG_IOCTL	= 0x00000080,

    MWIFIEX_DBG_MPA_D	= 0x00008000,
    MWIFIEX_DBG_DAT_D	= 0x00010000,
    MWIFIEX_DBG_CMD_D	= 0x00020000,
    MWIFIEX_DBG_EVT_D	= 0x00040000,
    MWIFIEX_DBG_FW_D	= 0x00080000,
    MWIFIEX_DBG_IF_D	= 0x00100000,

    MWIFIEX_DBG_ENTRY	= 0x10000000,
    MWIFIEX_DBG_WARN	= 0x20000000,
    MWIFIEX_DBG_INFO	= 0x40000000,
    MWIFIEX_DBG_DUMP	= 0x80000000,

    MWIFIEX_DBG_ANY		= 0xffffffff
}

pub const DEBUG_DUMP_DATA_MAX_LEN: c_int = 128;

// Min BGSCAN interval 15 second
pub const MWIFIEX_BGSCAN_INTERVAL: c_int = 15000;
// default repeat count
pub const MWIFIEX_BGSCAN_REPEAT_COUNT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_dbg {
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
    pub last_mp_wr_bitmap: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_mp_wr_ports: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_mp_wr_len: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_mp_curr_wr_port: [u32; MWIFIEX_DBG_SDIO_MP_NUM],
    pub last_sdio_mp_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MWIFIEX_HARDWARE_STATUS {
    MWIFIEX_HW_STATUS_READY,
    MWIFIEX_HW_STATUS_INITIALIZING,
    MWIFIEX_HW_STATUS_RESET,
    MWIFIEX_HW_STATUS_NOT_READY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MWIFIEX_802_11_POWER_MODE {
    MWIFIEX_802_11_POWER_MODE_CAM,
    MWIFIEX_802_11_POWER_MODE_PSP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tx_param {
    pub next_pkt_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MWIFIEX_PS_STATE {
    PS_STATE_AWAKE,
    PS_STATE_PRE_SLEEP,
    PS_STATE_SLEEP_CFM,
    PS_STATE_SLEEP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_iface_type {
    MWIFIEX_SDIO,
    MWIFIEX_PCIE,
    MWIFIEX_USB
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_add_ba_param {
    pub tx_win_size: u32,
    pub rx_win_size: u32,
    pub timeout: u32,
    pub tx_amsdu: u8,
    pub rx_amsdu: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tx_aggr {
    pub ampdu_user: u8,
    pub ampdu_ap: u8,
    pub amsdu: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_ba_status {
    BA_SETUP_NONE = 0,
    BA_SETUP_INPROGRESS,
    BA_SETUP_COMPLETE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ra_list_tbl {
    pub list: list_head,
    pub skb_head: sk_buff_head,
    pub ra: [u8; ETH_ALEN],
    pub is_11n_enabled: u32,
    pub max_amsdu: u16,
    pub ba_pkt_count: u16,
    pub ba_packet_thr: u8,
    pub ba_status: mwifiex_ba_status,
    pub amsdu_in_ampdu: u8,
    pub total_pkt_count: u16,
    pub tdls_link: bool,
    pub tx_paused: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tid_tbl {
    pub ra_list: list_head,
}

pub const WMM_HIGHEST_PRIORITY: c_int = 7;
pub const HIGH_PRIO_TID: c_int = 7;
pub const LOW_PRIO_TID: c_int = 0;

pub const MWIFIEX_WMM_DRV_DELAY_MAX: c_int = 510;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_wmm_desc {
    pub tid_tbl_ptr: [mwifiex_tid_tbl; MAX_NUM_TID],
    pub packets_out: [u32; MAX_NUM_TID],
    pub pkts_paused: [u32; MAX_NUM_TID],
// spin lock to protect ra_list
    pub ra_list_spinlock: spinlock_t,
    pub ac_status: [mwifiex_wmm_ac_status; IEEE80211_NUM_ACS],
    pub ac_down_graded_vals: [mwifiex_wmm_ac_e; IEEE80211_NUM_ACS],
    pub drv_pkt_delay_max: u32,
    pub queue_priority: [u8; IEEE80211_NUM_ACS],
    pub /: *mut *mut u32 user_pri_pkt_tx_ctrl[WMM_HIGHEST_PRIORITY + 1]; / UP: 0 to 7,
// Number of transmit packets queued
    pub tx_pkts_queued: core::sync::atomic::AtomicI32,
// Tracks highest priority with a packet queued
    pub highest_queued_prio: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_802_11_security {
    pub wpa_enabled: u8,
    pub wpa2_enabled: u8,
    pub wapi_enabled: u8,
    pub wapi_key_on: u8,
    pub wep_enabled: u8,
    pub authentication_mode: u32,
    pub is_authtype_auto: u8,
    pub encryption_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_header {
    pub element_id: u8,
    pub len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_vendor_specific {
    pub vend_hdr: ieee_types_vendor_header,
    pub ieee_types_vendor_header)]: u8 data[IEEE_MAX_IE_SIZE - sizeof(struct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_generic {
    pub ieee_hdr: ieee_types_header,
    pub ieee_types_header)]: u8 data[IEEE_MAX_IE_SIZE - sizeof(struct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_bss_co_2040 {
    pub ieee_hdr: ieee_types_header,
    pub bss_2040co: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_extcap {
    pub ieee_hdr: ieee_types_header,
    pub ext_capab: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_vht_cap {
    pub ieee_hdr: ieee_types_header,
    pub vhtcap: ieee80211_vht_cap,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_vht_oper {
    pub ieee_hdr: ieee_types_header,
    pub vhtoper: ieee80211_vht_operation,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_aid {
    pub ieee_hdr: ieee_types_header,
    pub aid: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_bssdescriptor {
    pub mac_address: [u8; ETH_ALEN],
    pub ssid: cfg80211_ssid,
    pub privacy: u32,
    pub rssi: i32,
    pub channel: u32,
    pub freq: u32,
    pub beacon_period: u16,
    pub erp_flags: u8,
    pub bss_mode: u32,
    pub supported_rates: [u8; MWIFIEX_SUPPORTED_RATES],
    pub data_rates: [u8; MWIFIEX_SUPPORTED_RATES],
// Network band.
// BAND_B(0x01): 'b' band
// BAND_G(0x02): 'g' band
// BAND_A(0X04): 'a' band
//
    pub bss_band: u16,
    pub fw_tsf: u64,
    pub timestamp: u64,
    pub phy_param_set: ieee_types_phy_param_set,
    pub ss_param_set: ieee_types_ss_param_set,
    pub cap_info_bitmap: u16,
    pub wmm_ie: ieee_types_wmm_parameter,
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
    pub bcn_wpa_ie: *mut ieee_types_vendor_specific,
    pub wpa_offset: u16,
    pub bcn_rsn_ie: *mut ieee_types_generic,
    pub rsn_offset: u16,
    pub bcn_rsnx_ie: *mut ieee_types_generic,
    pub rsnx_offset: u16,
    pub bcn_wapi_ie: *mut ieee_types_generic,
    pub wapi_offset: u16,
    pub beacon_buf: *mut u8,
    pub beacon_buf_size: u32,
    pub sensed_11h: u8,
    pub local_constraint: u8,
    pub chan_sw_ie_present: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_current_bss_params {
    pub bss_descriptor: mwifiex_bssdescriptor,
    pub wmm_enabled: u8,
    pub wmm_uapsd_enabled: u8,
    pub band: u8,
    pub num_of_rates: u32,
    pub data_rates: [u8; MWIFIEX_SUPPORTED_RATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sleep_period {
    pub period: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_wep_key {
    pub length: u32,
    pub key_index: u32,
    pub key_length: u32,
    pub key_material: [u8; MWIFIEX_KEY_BUFFER_SIZE],
}

pub const MAX_REGION_CHANNEL_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_chan_freq_power {
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

pub const MWIFIEX_MAX_TRIPLET_802_11D: c_int = 83;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_802_11d_domain_reg {
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub no_of_triplet: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_vendor_spec_cfg_ie {
    pub mask: u16,
    pub flag: u16,
    pub ie: [u8; MWIFIEX_MAX_VSIE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wps {
    pub session_enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_roc_cfg {
    pub cookie: u64,
    pub chan: ieee80211_channel,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_iface_work_flags {
    MWIFIEX_IFACE_WORK_DEVICE_DUMP,
    MWIFIEX_IFACE_WORK_CARD_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_adapter_work_flags {
    MWIFIEX_SURPRISE_REMOVED,
    MWIFIEX_IS_CMD_TIMEDOUT,
    MWIFIEX_IS_SUSPENDED,
    MWIFIEX_IS_HS_CONFIGURED,
    MWIFIEX_IS_HS_ENABLING,
    MWIFIEX_IS_REQUESTING_FW_VEREXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_band_config {
    pub chan_band:2: u8,
    pub chan_width:2: u8,
    pub chan2_offset:2: u8,
    pub scan_mode:2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_channel_band {
    pub band_config: mwifiex_band_config,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_private {
    pub adapter: *mut mwifiex_adapter,
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
    pub attempted_bss_desc: *mut mwifiex_bssdescriptor,
    pub prev_ssid: cfg80211_ssid,
    pub prev_bssid: [u8; ETH_ALEN],
    pub curr_bss_params: mwifiex_current_bss_params,
    pub beacon_period: u16,
    pub dtim_period: u8,
    pub listen_interval: u16,
    pub atim_window: u16,
    pub adhoc_channel: u8,
    pub adhoc_state: u8,
    pub sec_info: mwifiex_802_11_security,
    pub wep_key: [mwifiex_wep_key; NUM_WEP_KEYS],
    pub wep_key_curr_index: u16,
    pub wpa_ie: [u8; 256],
    pub wpa_ie_len: u16,
    pub wpa_is_gtk_set: u8,
    pub aes_key: host_cmd_ds_802_11_key_material,
    pub aes_key_v2: host_cmd_ds_802_11_key_material_v2,
    pub wapi_ie: [u8; 256],
    pub wapi_ie_len: u16,
    pub wps_ie: *mut u8,
    pub wps_ie_len: u16,
    pub wmm_required: u8,
    pub wmm_enabled: u8,
    pub wmm_qosinfo: u8,
    pub wmm: mwifiex_wmm_desc,
    pub wmm_tx_pending: [core::sync::atomic::AtomicI32; IEEE80211_NUM_ACS],
    pub sta_list: list_head,
// spin lock for associated station/TDLS peers list
    pub sta_list_spinlock: spinlock_t,
    pub auto_tdls_list: list_head,
// spin lock for auto TDLS peer list
    pub auto_tdls_lock: spinlock_t,
    pub tx_ba_stream_tbl_ptr: list_head,
// spin lock for tx_ba_stream_tbl_ptr queue
    pub tx_ba_stream_tbl_lock: spinlock_t,
    pub aggr_prio_tbl: [mwifiex_tx_aggr; MAX_NUM_TID],
    pub add_ba_param: mwifiex_add_ba_param,
    pub rx_seq: [u16; MAX_NUM_TID],
    pub tos_to_tid_inv: [u8; MAX_NUM_TID],
    pub rx_reorder_tbl_ptr: list_head,
// spin lock for rx_reorder_tbl_ptr queue
    pub rx_reorder_tbl_lock: spinlock_t,
pub const MWIFIEX_ASSOC_RSP_BUF_SIZE: c_int = 500;
    pub assoc_rsp_buf: [u8; MWIFIEX_ASSOC_RSP_BUF_SIZE],
    pub assoc_rsp_size: u32,
    pub req_bss: *mut cfg80211_bss,
pub const MWIFIEX_GENIE_BUF_SIZE: c_int = 256;
    pub gen_ie_buf: [u8; MWIFIEX_GENIE_BUF_SIZE],
    pub gen_ie_buf_len: u8,
    pub vs_ie: [mwifiex_vendor_spec_cfg_ie; MWIFIEX_MAX_VSIE_NUM],
pub const MWIFIEX_ASSOC_TLV_BUF_SIZE: c_int = 256;
    pub assoc_tlv_buf: [u8; MWIFIEX_ASSOC_TLV_BUF_SIZE],
    pub assoc_tlv_buf_len: u8,
    pub curr_bcn_buf: *mut u8,
    pub curr_bcn_size: u32,
// spin lock for beacon buffer
    pub curr_bcn_buf_lock: spinlock_t,
    pub wdev: wireless_dev,
    pub cfp: mwifiex_chan_freq_power,
    pub versionstrsel: u32,
    pub version_str: [c_char; MWIFIEX_VERSION_STR_LENGTH],
    pub dfs_dev_dir: *mut dentry,

    pub current_key_index: u16,
    pub async_mutex: mutex,
    pub scan_request: *mut cfg80211_scan_request,
    pub cfg_bssid: [u8; 6],
    pub wps: wps,
    pub scan_block: u8,
    pub cqm_rssi_thold: i32,
    pub cqm_rssi_hyst: u32,
    pub subsc_evt_rssi_state: u8,
    pub async_subsc_evt_storage: mwifiex_ds_misc_subsc_evt,
    pub mgmt_ie: [mwifiex_ie; MAX_MGMT_IE_INDEX],
    pub beacon_idx: u16,
    pub proberesp_idx: u16,
    pub assocresp_idx: u16,
    pub gen_idx: u16,
    pub ap_11n_enabled: u8,
    pub ap_11ac_enabled: u8,
    pub host_mlme_reg: bool,
    pub mgmt_frame_mask: u32,
    pub roc_cfg: mwifiex_roc_cfg,
    pub scan_aborting: bool,
    pub sched_scanning: u8,
    pub csa_chan: u8,
    pub csa_expire_time: c_ulong,
    pub del_list_idx: u8,
    pub hs2_enabled: bool,
    pub bss_cfg: mwifiex_uap_bss_param,
    pub bss_chandef: cfg80211_chan_def,
    pub sta_params: *mut station_parameters,
    pub tdls_txq: sk_buff_head,
    pub check_tdls_tx: u8,
    pub auto_tdls_timer: timer_list,
    pub auto_tdls_timer_active: bool,
    pub ack_status_frames: idr,
// spin lock for ack status
    pub ack_status_lock: spinlock_t,
// rx histogram data
    pub hist_data: *mut mwifiex_histogram_data,
    pub dfs_chandef: cfg80211_chan_def,
    pub dfs_cac_workqueue: *mut workqueue_struct,
    pub dfs_cac_work: delayed_work,
    pub dfs_chan_sw_workqueue: *mut workqueue_struct,
    pub dfs_chan_sw_work: delayed_work,
    pub beacon_after: cfg80211_beacon_data,
    pub state_11h: mwifiex_11h_intf_state,
    pub mem_rw: mwifiex_ds_mem_rw,
    pub bypass_txq: sk_buff_head,
    pub hidden_chan: [mwifiex_user_scan_chan; MWIFIEX_USER_SCAN_CHAN_MAX],
    pub ht_param_present: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tx_ba_stream_tbl {
    pub list: list_head,
    pub tid: c_int,
    pub ra: [u8; ETH_ALEN],
    pub ba_status: mwifiex_ba_status,
    pub amsdu: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reorder_tmr_cnxt {
    pub timer: timer_list,
    pub ptr: *mut mwifiex_rx_reorder_tbl,
    pub priv: *mut mwifiex_private,
    pub timer_is_set: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_rx_reorder_tbl {
    pub list: list_head,
    pub tid: c_int,
    pub ta: [u8; ETH_ALEN],
    pub init_win: c_int,
    pub start_win: c_int,
    pub win_size: c_int,
    pub rx_reorder_ptr: *mut c_void,
    pub timer_context: reorder_tmr_cnxt,
    pub amsdu: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_bss_prio_node {
    pub list: list_head,
    pub priv: *mut mwifiex_private,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_bss_prio_tbl {
    pub bss_prio_head: list_head,
// spin lock for bss priority
    pub bss_prio_lock: spinlock_t,
    pub bss_prio_cur: *mut mwifiex_bss_prio_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ctrl_node {
    pub list: list_head,
    pub priv: *mut mwifiex_private,
    pub cmd_no: u32,
    pub cmd_flag: u32,
    pub cmd_skb: *mut sk_buff,
    pub resp_skb: *mut sk_buff,
    pub data_buf: *mut c_void,
    pub wait_q_enabled: u32,
    pub skb: *mut sk_buff,
    pub condition: *mut u8,
    pub cmd_wait_q_woken: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_bss_priv {
    pub band: u8,
    pub fw_tsf: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tdls_capab {
    pub capab: __le16,
    pub rates: [u8; 32],
    pub rates_len: u8,
    pub qos_info: u8,
    pub coex_2040: u8,
    pub aid: u16,
    pub ht_capb: ieee80211_ht_cap,
    pub ht_oper: ieee80211_ht_operation,
    pub extcap: ieee_types_extcap,
    pub rsn_ie: ieee_types_generic,
    pub vhtcap: ieee80211_vht_cap,
    pub vhtoper: ieee80211_vht_operation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_station_stats {
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

// This is AP/TDLS specific structure which stores information
// about associated/peer STA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_sta_node {
    pub list: list_head,
    pub mac_addr: [u8; ETH_ALEN],
    pub is_wmm_enabled: u8,
    pub is_11n_enabled: u8,
    pub is_11ac_enabled: u8,
    pub ampdu_sta: [u8; MAX_NUM_TID],
    pub rx_seq: [u16; MAX_NUM_TID],
    pub max_amsdu: u16,
    pub tdls_status: u8,
    pub tdls_cap: mwifiex_tdls_capab,
    pub stats: mwifiex_station_stats,
    pub tx_pause: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_auto_tdls_peer {
    pub list: list_head,
    pub mac_addr: [u8; ETH_ALEN],
    pub tdls_status: u8,
    pub rssi: c_int,
    pub rssi_jiffies: c_ulong,
    pub failure_count: u8,
    pub do_discover: u8,
}

pub const MWIFIEX_TYPE_AGGR_DATA_V2: c_int = 11;

pub const MWIFIEX_BUS_AGGR_MAX_LEN: c_int = 16000;
pub const MWIFIEX_BUS_AGGR_MAX_NUM: c_int = 10;
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
pub struct mwifiex_if_ops {
    pub ): *mut *mut int (init_if) (struct mwifiex_adapter,
    pub ): *mut *mut void (cleanup_if) (struct mwifiex_adapter,
    pub u32): *mut *mut *mut int (check_fw_status) (struct mwifiex_adapter ,,
    pub ): *mut *mut int (check_winner_status)(struct mwifiex_adapter,
    pub ): *mut *mut *mut int (prog_fw) (struct mwifiex_adapter , struct mwifiex_fw_image,
    pub ): *mut *mut int (register_dev) (struct mwifiex_adapter,
    pub ): *mut *mut void (unregister_dev) (struct mwifiex_adapter,
    pub ): *mut *mut int (enable_int) (struct mwifiex_adapter,
    pub ): *mut *mut void (disable_int) (struct mwifiex_adapter,
    pub ): *mut *mut int (process_int_status) (struct mwifiex_adapter,
    pub ): *mut mwifiex_tx_param,
    pub ): *mut *mut int (wakeup) (struct mwifiex_adapter,
    pub ): *mut *mut int (wakeup_complete) (struct mwifiex_adapter,
// Interface specific functions
    pub u16): *mut *mut *mut void (update_mp_end_port) (struct mwifiex_adapter ,,
    pub ): *mut *mut void (cleanup_mpa_buf) (struct mwifiex_adapter,
    pub ): *mut *mut *mut int (cmdrsp_complete) (struct mwifiex_adapter , struct sk_buff,
    pub ): *mut *mut *mut int (event_complete) (struct mwifiex_adapter , struct sk_buff,
    pub adapter): *mut *mut void (init_fw_port)(struct mwifiex_adapter,
    pub ): *mut *mut *mut int (dnld_fw) (struct mwifiex_adapter , struct mwifiex_fw_image,
    pub ): *mut *mut void (card_reset) (struct mwifiex_adapter,
    pub ): *mut *mut *mut int (reg_dump)(struct mwifiex_adapter , char,
    pub ): *mut *mut void (device_dump)(struct mwifiex_adapter,
    pub adapter): *mut *mut void (clean_pcie_ring)(struct mwifiex_adapter,
    pub work): *mut *mut void (iface_work)(struct work_struct,
    pub adapter): *mut *mut void (submit_rem_rx_urbs)(struct mwifiex_adapter,
    pub ): *mut *mut *mut void (deaggr_pkt)(struct mwifiex_adapter , struct sk_buff,
    pub ): *mut *mut void (multi_port_resync)(struct mwifiex_adapter,
    pub ): *mut *mut bool (is_port_ready)(struct mwifiex_private,
    pub ): *mut *mut void (down_dev)(struct mwifiex_adapter,
    pub ): *mut *mut void (up_dev)(struct mwifiex_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_adapter {
    pub iface_type: u8,
    pub debug_mask: c_uint,
    pub iface_limit: mwifiex_iface_comb,
    pub curr_iface_comb: mwifiex_iface_comb,
    pub priv: [*mut mwifiex_private; MWIFIEX_MAX_BSS_NUM],
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
    pub if_ops: mwifiex_if_ops,
    pub bypass_tx_pending: core::sync::atomic::AtomicI32,
    pub rx_pending: core::sync::atomic::AtomicI32,
    pub tx_pending: core::sync::atomic::AtomicI32,
    pub cmd_pending: core::sync::atomic::AtomicI32,
    pub tx_hw_pending: core::sync::atomic::AtomicI32,
    pub workqueue: *mut workqueue_struct,
    pub main_work: work_struct,
    pub rx_workqueue: *mut workqueue_struct,
    pub rx_work: work_struct,
    pub host_mlme_workqueue: *mut workqueue_struct,
    pub host_mlme_work: work_struct,
    pub rx_work_enabled: bool,
    pub rx_processing: bool,
    pub delay_main_work: bool,
    pub rx_locked: bool,
    pub main_locked: bool,
    pub bss_prio_tbl: [mwifiex_bss_prio_tbl; MWIFIEX_MAX_BSS_NUM],
// spin lock for main process
    pub main_proc_lock: spinlock_t,
    pub mwifiex_processing: u32,
    pub more_task_flag: u8,
    pub tx_buf_size: u16,
    pub curr_tx_buf_size: u16,
// sdio single port rx aggregation capability
    pub host_disable_sdio_rx_aggr: bool,
    pub sdio_rx_aggr_enable: bool,
    pub sdio_rx_block_size: u16,
    pub ioport: u32,
    pub hw_status: MWIFIEX_HARDWARE_STATUS,
    pub number_of_antenna: u16,
    pub fw_cap_info: u32,
// spin lock for interrupt handling
    pub int_lock: spinlock_t,
    pub int_status: u8,
    pub event_cause: u32,
    pub event_skb: *mut sk_buff,
    pub upld_buf: [u8; MWIFIEX_UPLD_SIZE],
    pub data_sent: u8,
    pub cmd_sent: u8,
    pub cmd_resp_received: u8,
    pub event_received: u8,
    pub data_received: u8,
    pub assoc_resp_received: u8,
    pub priv_link_lost: *mut mwifiex_private,
    pub host_mlme_link_lost: u8,
    pub seq_num: u16,
    pub cmd_pool: *mut cmd_ctrl_node,
    pub curr_cmd: *mut cmd_ctrl_node,
// spin lock for command
    pub mwifiex_cmd_lock: spinlock_t,
    pub cmd_timer: timer_list,
    pub cmd_free_q: list_head,
// spin lock for cmd_free_q
    pub cmd_free_q_lock: spinlock_t,
    pub cmd_pending_q: list_head,
// spin lock for cmd_pending_q
    pub cmd_pending_q_lock: spinlock_t,
    pub scan_pending_q: list_head,
// spin lock for scan_pending_q
    pub scan_pending_q_lock: spinlock_t,
// spin lock for RX processing routine
    pub rx_proc_lock: spinlock_t,
    pub tx_data_q: sk_buff_head,
    pub tx_queued: core::sync::atomic::AtomicI32,
    pub scan_processing: u32,
    pub region_code: u16,
    pub domain_reg: mwifiex_802_11d_domain_reg,
    pub scan_probes: u16,
    pub scan_mode: u32,
    pub specific_scan_time: u16,
    pub active_scan_time: u16,
    pub passive_scan_time: u16,
    pub scan_chan_gap_time: u16,
    pub fw_bands: u8,
    pub adhoc_start_band: u8,
    pub config_bands: u8,
    pub tx_lock_flag: u8,
    pub sleep_period: mwifiex_sleep_period,
    pub ps_mode: u16,
    pub ps_state: u32,
    pub need_to_wakeup: u8,
    pub multiple_dtim: u16,
    pub local_listen_interval: u16,
    pub null_pkt_interval: u16,
    pub sleep_cfm: *mut sk_buff,
    pub bcn_miss_time_out: u16,
    pub adhoc_awake_period: u16,
    pub is_deep_sleep: u8,
    pub delay_null_pkt: u8,
    pub delay_to_ps: u16,
    pub enhanced_ps_mode: u16,
    pub pm_wakeup_card_req: u8,
    pub gen_null_pkt: u16,
    pub pps_uapsd_mode: u16,
    pub pm_wakeup_fw_try: u32,
    pub wakeup_timer: timer_list,
    pub hs_cfg: mwifiex_hs_config_param,
    pub hs_activated: u8,
    pub hs_activated_manually: u8,
    pub hs_activate_wait_q_woken: u16,
    pub hs_activate_wait_q: wait_queue_head_t,
    pub event_body: [u8; MAX_EVENT_SIZE],
    pub hw_dot_11n_dev_cap: u32,
    pub hw_dev_mcs_support: u8,
    pub user_dev_mcs_support: u8,
    pub adhoc_11n_enabled: u8,
    pub sec_chan_offset: u8,
    pub dbg: mwifiex_dbg,
    pub arp_filter: [u8; ARP_FILTER_MAX_BUF_SIZE],
    pub arp_filter_size: u32,
    pub cmd_wait_q: mwifiex_wait_queue,
    pub scan_wait_q_woken: u8,
    pub /: *mut *mut spinlock_t queue_lock; / lock for tx queues,
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub max_mgmt_ie_index: u16,
    pub cal_data: *const firmware,
    pub rgpower_data: *const firmware,
    pub dt_node: *mut device_node,
// 11AC
    pub is_hw_11ac_capable: u32,
    pub hw_dot_11ac_dev_cap: u32,
    pub hw_dot_11ac_mcs_support: u32,
    pub usr_dot_11ac_dev_cap_bg: u32,
    pub usr_dot_11ac_dev_cap_a: u32,
    pub usr_dot_11ac_mcs_support: u32,
    pub pending_bridged_pkts: core::sync::atomic::AtomicI32,
// For synchronizing FW initialization with device lifecycle.
    pub fw_done: *mut completion,
    pub is_up: bool,
    pub ext_scan: bool,
    pub host_mlme_enabled: bool,
    pub mwifiex_mgmt_stypes: [ieee80211_txrx_stypes; NUM_NL80211_IFTYPES],
    pub fw_api_ver: u8,
    pub key_api_minor_ver: u8 key_api_major_ver,,
    pub max_sta_conn: u8 max_p2p_conn,,
    pub mem_type_mapping_tbl: *mut memory_type_mapping,
    pub num_mem_types: u8,
    pub scan_chan_gap_enabled: bool,
    pub rx_data_q: sk_buff_head,
    pub mfg_mode: bool,
    pub chan_stats: *mut mwifiex_chan_stats,
    pub num_in_chan_stats: u32,
    pub survey_idx: c_int,
    pub auto_tdls: bool,
    pub coex_scan: u8,
    pub coex_min_scan_time: u8,
    pub coex_max_scan_time: u8,
    pub coex_win_size: u8,
    pub coex_tx_win_size: u8,
    pub coex_rx_win_size: u8,
    pub drcs_enabled: bool,
    pub active_scan_triggered: u8,
    pub usb_mc_status: bool,
    pub usb_mc_setup: bool,
    pub nd_info: *mut cfg80211_wowlan_nd_info,
    pub regd: *mut ieee80211_regdomain,
// Wake-on-WLAN (WoWLAN)
    pub irq_wakeup: c_int,
    pub wake_by_wifi: bool,
// Aggregation parameters
    pub bus_aggr: bus_aggr_params,
// Device dump data/length
    pub devdump_data: *mut c_void,
    pub devdump_len: c_int,
    pub devdump_work: delayed_work,
    pub ignore_btcoex_events: bool,
}

extern "C" {
    pub fn mwifiex_process_tx_queue(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_init_lock_list(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_trans_start(dev: *mut net_device);
}
extern "C" {
    pub fn mwifiex_init_priv(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_free_priv(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_init_fw(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_shutdown_drv(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_dnld_fw(: *mut mwifiex_adapter, : *mut mwifiex_fw_image) -> c_int;
}
extern "C" {
    pub fn mwifiex_recv_packet(priv: *mut mwifiex_private, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mwifiex_process_event(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_cmd_timeout_func(t: *mut timer_list);
}
extern "C" {
    pub fn mwifiex_alloc_cmd_buffer(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_free_cmd_buffer(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_free_cmd_buffers(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_cancel_all_pending_cmd(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_cancel_pending_scan_cmd(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_cancel_scan(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_exec_next_cmd(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_process_cmdresp(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_process_assoc_resp(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_send_null_packet(priv: *mut mwifiex_private, flags: u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_clean_txrx(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_check_last_packet_indication(priv: *mut mwifiex_private) -> u8;
}
extern "C" {
    pub fn mwifiex_check_ps_cond(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_process_hs_config(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_process_sta_event(: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_process_uap_event(: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_delete_all_station_list(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_sta_init_cmd(: *mut mwifiex_private, first_sta: u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_band_to_radio_type(band: u8) -> u8;
}
extern "C" {
    pub fn mwifiex_deauthenticate(priv: *mut mwifiex_private, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_deauthenticate_all(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_cmd_802_11_bg_scan_query(cmd: *mut host_cmd_ds_command) -> c_int;
}
extern "C" {
    pub fn mwifiex_find_freq_from_band_chan(_arg: u8, _arg: u8) -> u32;
}
extern "C" {
    pub fn mwifiex_get_supported_rates(priv: *mut mwifiex_private, rates: *mut u8) -> u32;
}
extern "C" {
    pub fn mwifiex_is_rate_auto(priv: *mut mwifiex_private) -> u8;
}
extern "C" {
    pub fn mwifiex_save_curr_bcn(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_free_curr_bcn(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn is_command_pending(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_ba_params(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_update_ampdu_txwinsize(pmadapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_set_11ac_ba_params(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_stop_bg_scan(priv: *mut mwifiex_private) -> c_int;
}
//
// This function checks if the queuing is RA based or not.
//
// Currently we assume if we are in Infra, then DA=RA. This might not be
// true in the future
//
// This function copies rates.
//
// This function returns the correct private structure pointer based
// upon the BSS type and BSS number.
//
// This function returns the first available private structure pointer
// based upon the BSS role.
//
// This function checks available bss_num when adding new interface or
// changing interface type.
//
// This function returns the first available unused private structure pointer.
//
// This function returns the driver private structure of a network device.
//
// This function checks if a skb holds a management frame.
//
// This function retrieves channel closed for operation by Channel
// Switch Announcement.
//
// Clear csa channel, if DFS channel move time has passed
// Disable platform specific wakeup interrupt
// Undo our disable, since interrupt handler already
// did this.
//
// Enable platform specific wakeup interrupt
extern "C" {
    pub fn mwifiex_remove_card(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_cancel_hs(priv: *mut mwifiex_private, cmd_type: c_int) -> c_int;
}
extern "C" {
    pub fn mwifiex_enable_hs(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_disable_auto_ds(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_drv_get_data_rate(priv: *mut mwifiex_private, rate: *mut u32) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_radio(priv: *mut mwifiex_private, option: u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_gen_ie(priv: *mut mwifiex_private, ie: *const u8, ie_len: c_int) -> c_int;
}
extern "C" {
    pub fn mwifiex_get_ver_ext(priv: *mut mwifiex_private, version_str_sel: u32) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_11n_httx_cfg(priv: *mut mwifiex_private, data: c_int) -> c_int;
}
extern "C" {
    pub fn mwifiex_get_11n_httx_cfg(priv: *mut mwifiex_private, data: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_tx_rate_cfg(priv: *mut mwifiex_private, tx_rate_index: c_int) -> c_int;
}
extern "C" {
    pub fn mwifiex_get_tx_rate_cfg(priv: *mut mwifiex_private, tx_rate_index: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mwifiex_drv_set_power(priv: *mut mwifiex_private, ps_mode: *mut u32) -> c_int;
}
extern "C" {
    pub fn mwifiex_main_process(: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_queue_tx_pkt(priv: *mut mwifiex_private, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mwifiex_chan_type_to_sec_chan_offset(chan_type: nl80211_channel_type) -> u8;
}
extern "C" {
    pub fn mwifiex_get_chan_type(priv: *mut mwifiex_private) -> u8;
}
extern "C" {
    pub fn mwifiex_del_virtual_intf(wiphy: *mut wiphy, wdev: *mut wireless_dev) -> c_int;
}
extern "C" {
    pub fn mwifiex_set_sys_config_invalid_data(config: *mut mwifiex_uap_bss_param);
}
extern "C" {
    pub fn mwifiex_add_wowlan_magic_pkt_filter(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_del_mgmt_ies(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_init_11h_params(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_is_11h_active(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_11h_activate(priv: *mut mwifiex_private, flag: bool) -> c_int;
}
extern "C" {
    pub fn mwifiex_11h_handle_event_chanswann(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_dnld_txpwr_table(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_del_all_sta_list(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_del_sta_entry(priv: *mut mwifiex_private, mac: *const u8);
}
extern "C" {
    pub fn mwifiex_is_tdls_chan_switching(priv: *mut mwifiex_private) -> u8;
}
extern "C" {
    pub fn mwifiex_is_send_cmd_allowed(priv: *mut mwifiex_private) -> u8;
}
extern "C" {
    pub fn mwifiex_tdls_oper(priv: *mut mwifiex_private, peer: *const u8, action: u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_get_tdls_link_status(priv: *mut mwifiex_private, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_disable_all_tdls_links(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_is_bss_in_11ac_mode(priv: *mut mwifiex_private) -> bool;
}
extern "C" {
    pub fn mwifiex_init_channel_scan_gap(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_tdls_check_tx(priv: *mut mwifiex_private, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mwifiex_flush_auto_tdls_list(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_check_auto_tdls(t: *mut timer_list);
}
extern "C" {
    pub fn mwifiex_add_auto_tdls_peer(priv: *mut mwifiex_private, mac: *const u8);
}
extern "C" {
    pub fn mwifiex_setup_auto_tdls_timer(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_clean_auto_tdls(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_config_tdls_enable(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_config_tdls_disable(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_config_tdls_cs_params(priv: *mut mwifiex_private) -> c_int;
}
extern "C" {
    pub fn mwifiex_stop_tdls_cs(priv: *mut mwifiex_private, peer_mac: *const u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_dfs_cac_work_queue(work: *mut work_struct);
}
extern "C" {
    pub fn mwifiex_dfs_chan_sw_work_queue(work: *mut work_struct);
}
extern "C" {
    pub fn mwifiex_abort_cac(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_hist_data_reset(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_drv_info_dump(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_prepare_fw_dump_info(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_upload_device_dump(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_fw_dump_event(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_queue_main_work(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_coex_ampdu_rxwinsize(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_11n_delba(priv: *mut mwifiex_private, tid: c_int);
}
extern "C" {
    pub fn mwifiex_send_domain_info_cmd_fw(wiphy: *mut wiphy) -> c_int;
}
extern "C" {
    pub fn mwifiex_multi_chan_resync(adapter: *mut mwifiex_adapter);
}
extern "C" {
    pub fn mwifiex_devdump_tmo_func(function_context: c_ulong);
}

extern "C" {
    pub fn mwifiex_debugfs_init();
}
extern "C" {
    pub fn mwifiex_debugfs_remove();
}
extern "C" {
    pub fn mwifiex_dev_debugfs_init(priv: *mut mwifiex_private);
}
extern "C" {
    pub fn mwifiex_dev_debugfs_remove(priv: *mut mwifiex_private);
}

extern "C" {
    pub fn mwifiex_reinit_sw(adapter: *mut mwifiex_adapter) -> c_int;
}
extern "C" {
    pub fn mwifiex_shutdown_sw(adapter: *mut mwifiex_adapter) -> c_int;
}
