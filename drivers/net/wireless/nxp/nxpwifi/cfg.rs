//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/nxp/nxpwifi/cfg.h
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
// Copyright 2011-2024 NXP
//

pub const NUM_WEP_KEYS: c_int = 4;
pub const NXPWIFI_BSS_COEX_COUNT: c_int = 2;

pub const NXPWIFI_MAX_CSA_COUNTERS: c_int = 5;
pub const NXPWIFI_DMA_ALIGN_SZ: c_int = 64;
pub const NXPWIFI_RX_HEADROOM: c_int = 64;
pub const MAX_TXPD_SZ: c_int = 32;
pub const INTF_HDR_ALIGN: c_int = 4;
// special FW 4 address management header

// + sizeof(tx_control)
//
pub const FRMCTL_LEN: c_int = 2;
pub const DURATION_LEN: c_int = 2;
pub const SEQCTL_LEN: c_int = 2;

pub const AUTH_ALG_LEN: c_int = 2;
pub const AUTH_TRANSACTION_LEN: c_int = 2;
pub const AUTH_STATUS_LEN: c_int = 2;

pub const AUTH_TX_DEFAULT_WAIT_TIME: c_int = 2400;
pub const WLAN_AUTH_NONE: c_uint = 0xFFFF;
pub const NXPWIFI_MAX_TX_BASTREAM_SUPPORTED: c_int = 2;
pub const NXPWIFI_MAX_RX_BASTREAM_SUPPORTED: c_int = 16;
pub const NXPWIFI_STA_AMPDU_DEF_TXWINSIZE: c_int = 64;
pub const NXPWIFI_STA_AMPDU_DEF_RXWINSIZE: c_int = 64;
pub const NXPWIFI_STA_COEX_AMPDU_DEF_RXWINSIZE: c_int = 16;
pub const NXPWIFI_UAP_AMPDU_DEF_TXWINSIZE: c_int = 32;
pub const NXPWIFI_UAP_COEX_AMPDU_DEF_RXWINSIZE: c_int = 16;
pub const NXPWIFI_UAP_AMPDU_DEF_RXWINSIZE: c_int = 16;
pub const NXPWIFI_11AC_STA_AMPDU_DEF_TXWINSIZE: c_int = 64;
pub const NXPWIFI_11AC_STA_AMPDU_DEF_RXWINSIZE: c_int = 64;
pub const NXPWIFI_11AC_UAP_AMPDU_DEF_TXWINSIZE: c_int = 64;
pub const NXPWIFI_11AC_UAP_AMPDU_DEF_RXWINSIZE: c_int = 64;
pub const NXPWIFI_DEFAULT_BLOCK_ACK_TIMEOUT: c_uint = 0xffff;
pub const NXPWIFI_RATE_BITMAP_MCS0: c_int = 32;

pub const NXPWIFI_RETRY_LIMIT_MAX: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_bcast_ssid_ctl {
// Hide SSID in beacons (SSID length = 0)
    NXPWIFI_BCAST_SSID_HIDE_LEN_ZERO = 0,
// Do not hide SSID (normal broadcast)
    NXPWIFI_BCAST_SSID_VISIBLE,
// Hide SSID, clear SSID content (ASCII 0),
// but keep the original SSID length
//
    NXPWIFI_BCAST_SSID_HIDE_LEN_RETAIN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_radio_ctl {
    NXPWIFI_RADIO_CTL_DISABLE = 0,
    NXPWIFI_RADIO_CTL_ENABLE,
    __NXPWIFI_RADIO_CTL_MAX,
}

pub const NXPWIFI_WMM_VERSION: c_uint = 0x01;
pub const NXPWIFI_WMM_SUBTYPE: c_uint = 0x01;
pub const NXPWIFI_SDIO_BLOCK_SIZE: c_int = 256;

pub const NXPWIFI_BRIDGED_PKTS_THR_HIGH: c_int = 1024;
pub const NXPWIFI_BRIDGED_PKTS_THR_LOW: c_int = 128;
// 54M rates, index from 0 to 11
pub const NXPWIFI_RATE_INDEX_MCS0: c_int = 12;
// 12-27=MCS0-15(BW20)
pub const NXPWIFI_BW20_MCS_NUM: c_int = 15;
// Rate index for OFDM 0
pub const NXPWIFI_RATE_INDEX_OFDM0: c_int = 4;
pub const NXPWIFI_MAX_STA_NUM: c_int = 3;
pub const NXPWIFI_MAX_UAP_NUM: c_int = 3;
pub const NXPWIFI_A_BAND_START_FREQ: c_int = 5000;
// SDIO Aggr data packet special info

pub const BLOCK_NUMBER_OFFSET: c_int = 15;
pub const SDIO_HEADER_OFFSET: c_int = 28;
pub const NXPWIFI_SIZE_4K: c_uint = 0x4000;
pub const NXPWIFI_EXT_CAPAB_IE_LEN: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_bss_type {
    NXPWIFI_BSS_TYPE_STA = 0,
    NXPWIFI_BSS_TYPE_UAP = 1,
    NXPWIFI_BSS_TYPE_ANY = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_bss_role {
    NXPWIFI_BSS_ROLE_STA = 0,
    NXPWIFI_BSS_ROLE_UAP = 1,
    NXPWIFI_BSS_ROLE_ANY = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_data_frame_type {
    NXPWIFI_DATA_FRAME_TYPE_ETH_II = 0,
    NXPWIFI_DATA_FRAME_TYPE_802_11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_fw_image {
    pub helper_buf: *mut u8,
    pub helper_len: u32,
    pub fw_buf: *mut u8,
    pub fw_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_802_11_ssid {
    pub ssid_len: u32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_wait_queue {
    pub wait: wait_queue_head_t,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_rxinfo {
    pub parent: *mut sk_buff,
    pub bss_num: u8,
    pub bss_type: u8,
    pub use_count: u8,
    pub buf_type: u8,
    pub pkt_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_txinfo {
    pub flags: u8,
    pub bss_num: u8,
    pub bss_type: u8,
    pub aggr_num: u8,
    pub pkt_len: u32,
    pub ack_frame_id: u8,
    pub cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_wmm_ac_e {
    WMM_AC_BK,
    WMM_AC_BE,
    WMM_AC_VI,
    WMM_AC_VO
    } __packed;

    struct nxpwifi_types_wmm_info {
    u8 oui[4];
    u8 subtype;
    u8 version;
    u8 qos_info;
    u8 reserved;
    struct ieee80211_wmm_ac_param ac[IEEE80211_NUM_ACS];
    } __packed;

    struct nxpwifi_arp_eth_header {
    struct arphdr hdr;
    u8 ar_sha[ETH_ALEN];
    u8 ar_sip[4];
    u8 ar_tha[ETH_ALEN];
    u8 ar_tip[4];
    } __packed;

    struct nxpwifi_chan_stats {
    u8 chan_num;
    u8 bandcfg;
    u8 flags;
    s8 noise;
    u16 total_bss;
    u16 cca_scan_dur;
    u16 cca_busy_dur;
    } __packed;

pub const NXPWIFI_HIST_MAX_SAMPLES: c_int = 1048576;
pub const NXPWIFI_MAX_RX_RATES: c_int = 44;
pub const NXPWIFI_MAX_AC_RX_RATES: c_int = 74;
pub const NXPWIFI_MAX_SNR: c_int = 256;
pub const NXPWIFI_MAX_NOISE_FLR: c_int = 256;
pub const NXPWIFI_MAX_SIG_STRENGTH: c_int = 256;

    struct nxpwifi_histogram_data {
    atomic_t rx_rate[NXPWIFI_MAX_AC_RX_RATES];
    atomic_t snr[NXPWIFI_MAX_SNR];
    atomic_t noise_flr[NXPWIFI_MAX_NOISE_FLR];
    atomic_t sig_str[NXPWIFI_MAX_SIG_STRENGTH];
    atomic_t num_samples;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_iface_comb {
    pub sta_intf: u8,
    pub uap_intf: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_radar_params {
    pub chandef: *mut cfg80211_chan_def,
    pub cac_time_ms: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11h_intf_state {
    pub is_11h_enabled: bool,
    pub is_11h_active: bool,
    pub __packed: },
pub const NXPWIFI_FW_DUMP_IDX: c_uint = 0xff;
pub const NXPWIFI_FW_DUMP_MAX_MEMSIZE: c_uint = 0x160000;
pub const NXPWIFI_DRV_INFO_IDX: c_int = 20;
pub const FW_DUMP_MAX_NAME_LEN: c_int = 8;
pub const FW_DUMP_HOST_READY: c_uint = 0xEE;
pub const FW_DUMP_DONE: c_uint = 0xFF;
pub const FW_DUMP_READ_DONE: c_uint = 0xFE;
// Channel bandwidth
pub const CHANNEL_BW_20MHZ: c_int = 0;
pub const CHANNEL_BW_40MHZ_ABOVE: c_int = 1;
pub const CHANNEL_BW_40MHZ_BELOW: c_int = 3;
// secondary channel is 80MHz bandwidth for 11ac
pub const CHANNEL_BW_80MHZ: c_int = 4;
pub const CHANNEL_BW_160MHZ: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_type_mapping {
    pub mem_name: [u8; FW_DUMP_MAX_NAME_LEN],
    pub mem_ptr: *mut u8,
    pub mem_size: u32,
    pub done_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdwr_status {
    RDWR_STATUS_SUCCESS = 0,
    RDWR_STATUS_FAILURE = 1,
    RDWR_STATUS_DONE = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_chan_band {
    BAND_2GHZ = 0,
    BAND_5GHZ,
    BAND_6GHZ,
    BAND_4GHZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_chan_width {
    CHAN_BW_20MHZ = 0,
    CHAN_BW_10MHZ,
    CHAN_BW_40MHZ,
    CHAN_BW_80MHZ,
    CHAN_BW_8080MHZ,
    CHAN_BW_160MHZ,
    CHAN_BW_5MHZ,
}

pub const NXPWIFI_PROMISC_MODE: c_int = 1;
pub const NXPWIFI_MULTICAST_MODE: c_int = 2;
pub const NXPWIFI_ALL_MULTI_MODE: c_int = 4;
pub const NXPWIFI_MAX_MULTICAST_LIST_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_multicast_list {
    pub mode: u32,
    pub num_multicast_addr: u32,
    pub mac_list: [u8; NXPWIFI_MAX_MULTICAST_LIST_SIZE][ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_chan_freq {
    pub channel: u32,
    pub freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ssid_bssid {
    pub ssid: cfg80211_ssid,
    pub bssid: [u8; ETH_ALEN],
}

pub const NXPWIFI_WPA_PASSHPHRASE_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpa_param {
    pub pairwise_cipher_wpa: u8,
    pub pairwise_cipher_wpa2: u8,
    pub group_cipher: u8,
    pub length: u32,
    pub passphrase: [u8; NXPWIFI_WPA_PASSHPHRASE_LEN],
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
pub const NXPWIFI_AUTH_MODE_AUTO: c_uint = 0xFF;
pub const BAND_CONFIG_BG: c_uint = 0x00;
pub const BAND_CONFIG_A: c_uint = 0x01;
pub const NXPWIFI_SEC_CHAN_BELOW: c_uint = 0x03;
pub const NXPWIFI_SEC_CHAN_ABOVE: c_uint = 0x01;
pub const NXPWIFI_SUPPORTED_RATES: c_int = 14;
pub const NXPWIFI_SUPPORTED_RATES_EXT: c_int = 32;
pub const NXPWIFI_PRIO_BK: c_int = 2;
pub const NXPWIFI_PRIO_VI: c_int = 5;
pub const NXPWIFI_SUPPORTED_CHANNELS: c_int = 2;
pub const NXPWIFI_OPERATING_CLASSES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_uap_bss_param {
    pub mac_addr: [u8; ETH_ALEN],
    pub channel: u8,
    pub band_cfg: u8,
    pub rts_threshold: u16,
    pub frag_threshold: u16,
    pub retry_limit: u8,
    pub ssid: nxpwifi_802_11_ssid,
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
    pub rates: [u8; NXPWIFI_SUPPORTED_RATES],
    pub sta_ao_timer: u32,
    pub ps_sta_ao_timer: u32,
    pub power_constraint: u8,
    pub wmm_element: ieee80211_wmm_param_ie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_get_stats {
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

pub const NXPWIFI_MAX_VER_STR_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ver_ext {
    pub version_str_sel: u32,
    pub version_str: [c_char; NXPWIFI_MAX_VER_STR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_bss_info {
    pub bss_mode: u32,
    pub ssid: cfg80211_ssid,
    pub bss_chan: u32,
    pub country_code: [u8; 3],
    pub media_connected: u32,
    pub max_power_level: u32,
    pub min_power_level: u32,
    pub bcn_nf_last: signed int,
    pub wep_status: u32,
    pub is_hs_configured: u32,
    pub is_deep_sleep: u32,
    pub bssid: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_sta_info {
    pub peer_mac: [u8; ETH_ALEN],
    pub params: *mut station_parameters,
}

pub const MAX_NUM_TID: c_int = 8;
pub const MAX_RX_WINSIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_rx_reorder_tbl {
    pub tid: u16,
    pub ta: [u8; ETH_ALEN],
    pub start_win: u32,
    pub win_size: u32,
    pub buffer: [u32; MAX_RX_WINSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_tx_ba_stream_tbl {
    pub tid: u16,
    pub ra: [u8; ETH_ALEN],
    pub amsdu: u8,
}

pub const DBG_CMD_NUM: c_int = 5;
pub const NXPWIFI_DBG_SDIO_MP_NUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_debug_info {
    pub debug_mask: c_uint,
    pub int_counter: u32,
    pub packets_out: [u32; MAX_NUM_TID],
    pub tx_buf_size: u32,
    pub curr_tx_buf_size: u32,
    pub tx_tbl_num: u32,
    pub rx_tbl_num: u32,
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
    pub last_mp_wr_bitmap: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_mp_wr_ports: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_mp_wr_len: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_mp_curr_wr_port: [u32; NXPWIFI_DBG_SDIO_MP_NUM],
    pub last_sdio_mp_index: u8,
}

pub const NXPWIFI_KEY_INDEX_UNICAST: c_uint = 0x40000000;
pub const PN_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_encrypt_key {
    pub key_disable: u32,
    pub key_index: u32,
    pub key_len: u32,
    pub key_cipher: u32,
    pub key_material: [u8; WLAN_MAX_KEY_LEN],
    pub mac_addr: [u8; ETH_ALEN],
    pub /: *mut *mut u8 pn[PN_LEN]; / packet number,
    pub pn_len: u8,
    pub is_igtk_key: u8,
    pub is_current_wep_key: u8,
    pub is_rx_seq_valid: u8,
    pub is_igtk_def_key: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_power_cfg {
    pub is_power_auto: u32,
    pub is_power_fixed: u32,
    pub power_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_hs_cfg {
    pub is_invoke_hostcmd: u32,
//
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
pub struct nxpwifi_ds_wakeup_reason {
    pub hs_wakeup_reason: u16,
}

pub const DEEP_SLEEP_ON: c_int = 1;
pub const DEEP_SLEEP_OFF: c_int = 0;
pub const DEEP_SLEEP_IDLE_TIME: c_int = 100;
pub const PS_MODE_AUTO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_auto_ds {
    pub auto_ds: u16,
    pub idle_time: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_pm_cfg {
    pub ps_mode: u32,
    pub hs_cfg: nxpwifi_ds_hs_cfg,
    pub auto_deep_sleep: nxpwifi_ds_auto_ds,
    pub sleep_period: u32,
    pub param: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ac_vht_cfg {
    pub band_config: u8,
    pub misc_config: u8,
    pub cap_info: u32,
    pub mcs_tx_set: u32,
    pub mcs_rx_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_11n_tx_cfg {
    pub tx_htcap: u16,
    pub tx_htinfo: u16,
    pub /: *mut *mut u16 misc_config; / Needed for 802.11AC cards only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_11n_amsdu_aggr_ctrl {
    pub enable: u16,
    pub curr_buf_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_ant_cfg {
    pub tx_ant: u32,
    pub rx_ant: u32,
}

pub const NXPWIFI_NUM_OF_CMD_BUFFER: c_int = 50;
pub const NXPWIFI_SIZE_OF_CMD_BUFFER: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_reg_rw {
    pub type: u32,
    pub offset: u32,
    pub value: u32,
}

pub const MAX_EEPROM_DATA: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_read_eeprom {
    pub offset: u16,
    pub byte_count: u16,
    pub value: [u8; MAX_EEPROM_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_mem_rw {
    pub addr: u32,
    pub value: u32,
}

pub const IEEE_MAX_IE_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_misc_gen_ie {
    pub type: u32,
    pub len: u32,
    pub ie_data: [u8; IEEE_MAX_IE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_misc_cmd {
    pub len: u32,
    pub cmd: [u8; NXPWIFI_SIZE_OF_CMD_BUFFER],
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
pub struct nxpwifi_ds_misc_subsc_evt {
    pub action: u16,
    pub events: u16,
    pub bcn_l_rssi_cfg: subsc_evt_cfg,
    pub bcn_h_rssi_cfg: subsc_evt_cfg,
}

pub const NXPWIFI_MEF_MAX_FILTERS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_mef_filter {
    pub repeat: u16,
    pub offset: u16,
    pub 1]: s8 byte_seq[NXPWIFI_MEF_MAX_BYTESEQ +,
    pub filt_type: u8,
    pub filt_action: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_mef_entry {
    pub mode: u8,
    pub action: u8,
    pub filter: [nxpwifi_mef_filter; NXPWIFI_MEF_MAX_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_mef_cfg {
    pub criteria: u32,
    pub num_entries: u16,
    pub mef_entry: *mut nxpwifi_mef_entry,
}

pub const NXPWIFI_VSIE_MASK_CLEAR: c_uint = 0x00;
pub const NXPWIFI_VSIE_MASK_SCAN: c_uint = 0x01;
pub const NXPWIFI_VSIE_MASK_ASSOC: c_uint = 0x02;
pub const NXPWIFI_VSIE_MASK_BGSCAN: c_uint = 0x08;
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

pub const NXPWIFI_COALESCE_MAX_RULES: c_int = 8;

pub const NXPWIFI_COALESCE_MAX_FILTERS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filt_field_param {
    pub operation: u8,
    pub operand_len: u8,
    pub offset: u16,
    pub operand_byte_stream: [u8; NXPWIFI_COALESCE_MAX_BYTESEQ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_coalesce_rule {
    pub max_coalescing_delay: u16,
    pub num_of_fields: u8,
    pub pkt_type: u8,
    pub params: [filt_field_param; NXPWIFI_COALESCE_MAX_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_ds_coalesce_cfg {
    pub num_of_rules: u16,
    pub rule: [nxpwifi_coalesce_rule; NXPWIFI_COALESCE_MAX_RULES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_he_cap_cfg {
    pub id: u16,
    pub len: u16,
    pub ext_id: u8,
    pub cap_elem: ieee80211_he_cap_elem,
    pub he_txrx_mcs_support: [u8; 4],
    pub val: [u8; 28],
}

pub const HE_CAP_MAX_SIZE: c_int = 54;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_he_cfg {
    pub band: u8,
    pub he_cap_cfg: nxpwifi_11ax_he_cap_cfg,
    pub data: [u8; HE_CAP_MAX_SIZE],
}

pub const NXPWIFI_11AXCMD_CFG_ID_SR_OBSS_PD_OFFSET: c_int = 1;
pub const NXPWIFI_11AXCMD_CFG_ID_SR_ENABLE: c_int = 2;
pub const NXPWIFI_11AXCMD_CFG_ID_BEAM_CHANGE: c_int = 3;
pub const NXPWIFI_11AXCMD_CFG_ID_HTC_ENABLE: c_int = 4;
pub const NXPWIFI_11AXCMD_CFG_ID_TXOP_RTS: c_int = 5;
pub const NXPWIFI_11AXCMD_CFG_ID_TX_OMI: c_int = 6;
pub const NXPWIFI_11AXCMD_CFG_ID_OBSSNBRU_TOLTIME: c_int = 7;
pub const NXPWIFI_11AXCMD_CFG_ID_SET_BSRP: c_int = 8;
pub const NXPWIFI_11AXCMD_CFG_ID_LLDE: c_int = 9;
pub const NXPWIFI_11AXCMD_SR_SUBID: c_uint = 0x102;
pub const NXPWIFI_11AXCMD_BEAM_SUBID: c_uint = 0x103;
pub const NXPWIFI_11AXCMD_HTC_SUBID: c_uint = 0x104;
pub const NXPWIFI_11AXCMD_TXOMI_SUBID: c_uint = 0x105;
pub const NXPWIFI_11AXCMD_OBSS_TOLTIME_SUBID: c_uint = 0x106;
pub const NXPWIFI_11AXCMD_TXOPRTS_SUBID: c_uint = 0x108;
pub const NXPWIFI_11AXCMD_SET_BSRP_SUBID: c_uint = 0x109;
pub const NXPWIFI_11AXCMD_LLDE_SUBID: c_uint = 0x110;
pub const NXPWIFI_11AX_TWT_SETUP_SUBID: c_uint = 0x114;
pub const NXPWIFI_11AX_TWT_TEARDOWN_SUBID: c_uint = 0x115;
pub const NXPWIFI_11AX_TWT_REPORT_SUBID: c_uint = 0x116;
pub const NXPWIFI_11AX_TWT_INFORMATION_SUBID: c_uint = 0x119;
pub const NXPWIFI_11AX_BTWT_AP_CONFIG_SUBID: c_uint = 0x120;
pub const BTWT_AGREEMENT_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11axcmdcfg_obss_pd_offset {
// <NON_SRG_OffSET, SRG_OFFSET>
    pub offset: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11axcmdcfg_sr_control {
// 1 enable, 0 disable
    pub control: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_sr_cmd {
// type
    pub type: u16,
// length of TLV
    pub len: u16,
// value
    pub obss_pd_offset: nxpwifi_11axcmdcfg_obss_pd_offset,
    pub sr_control: nxpwifi_11axcmdcfg_sr_control,
    pub param: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_beam_cmd {
// command value: 1 is disable, 0 is enable
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_htc_cmd {
// command value: 1 is enable, 0 is disable
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_txomi_cmd {
// 11ax spec 9.2.4.6a.2 OM Control 12 bits. Bit 0 to bit 11
    pub omi: u16,
//
// tx option
// 0: send OMI in QoS NULL; 1: send OMI in QoS data; 0xFF: set OMI in
// both
//
    pub tx_option: u8,
//
// if OMI is sent in QoS data, specify the number of consecutive data
// packets containing the OMI
//
    pub num_data_pkts: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_toltime_cmd {
// OBSS Narrow Bandwidth RU Tolerance Time
    pub tol_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_txop_cmd {
//
// Two byte rts threshold value of which only 10 bits, bit 0 to bit 9
// are valid
//
    pub rts_thres: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_set_bsrp_cmd {
// command value: 1 is enable, 0 is disable
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_llde_cmd {
// Uplink LLDE: enable=1,disable=0
    pub llde: u8,
// operation mode: default=0,carplay=1,gameplay=2
    pub mode: u8,
// trigger frame rate: auto=0xff
    pub fixrate: u8,
// cap airtime limit index: auto=0xff
    pub trigger_limit: u8,
// cap peak UL rate
    pub peak_ul_rate: u8,
// Downlink LLDE: enable=1,disable=0
    pub dl_llde: u8,
// Set trigger frame interval(us): auto=0
    pub poll_interval: u16,
// Set TxOp duration
    pub tx_op_duration: u16,
// for other configurations
    pub llde_ctrl: u16,
    pub mu_rts_successcnt: u16,
    pub mu_rts_failcnt: u16,
    pub basic_trigger_successcnt: u16,
    pub basic_trigger_failcnt: u16,
    pub tbppdu_nullcnt: u16,
    pub tbppdu_datacnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_11ax_cmd_cfg {
    pub sub_command: u32,
    pub sub_id: u32,
    pub sr_cfg: nxpwifi_11ax_sr_cmd,
    pub beam_cfg: nxpwifi_11ax_beam_cmd,
    pub htc_cfg: nxpwifi_11ax_htc_cmd,
    pub txomi_cfg: nxpwifi_11ax_txomi_cmd,
    pub toltime_cfg: nxpwifi_11ax_toltime_cmd,
    pub txop_cfg: nxpwifi_11ax_txop_cmd,
    pub setbsrp_cfg: nxpwifi_11ax_set_bsrp_cmd,
    pub llde_cfg: nxpwifi_11ax_llde_cmd,
    pub param: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_twt_setup {
// Implicit, 0: TWT session is explicit, 1: Session is implicit
    pub implicit: u8,
// Announced, 0: Unannounced, 1: Announced TWT
    pub announced: u8,
// Trigger Enabled, 0: Non-Trigger enabled, 1: Trigger enabled TWT
    pub trigger_enabled: u8,
// TWT Information Disabled, 0: TWT info enabled, 1: TWT info disabled
    pub twt_info_disabled: u8,
//
// Negotiation Type, 0: Future Individual TWT SP start time, 1:
// Next Wake TBTT time
//
    pub negotiation_type: u8,
//
// TWT Wakeup Duration, time after which the TWT requesting STA can
// transition to doze state
//
    pub twt_wakeup_duration: u8,
// Flow Identifier. Range: [0-7]
    pub flow_identifier: u8,
//
// Hard Constraint, 0: FW can tweak the TWT setup parameters if it is
// rejected by AP.
// 1: Firmware should not tweak any parameters.
//
    pub hard_constraint: u8,
// TWT Exponent, Range: [0-63]
    pub twt_exponent: u8,
// TWT Mantissa Range: [0-sizeof(UINT16)]
    pub twt_mantissa: __le16,
// TWT Request Type, 0: REQUEST_TWT, 1: SUGGEST_TWT
    pub twt_request: u8,
// TWT Setup State. Set to 0 by driver, filled by FW in response
    pub twt_setup_state: u8,
// TWT link lost timeout threshold
    pub bcn_miss_threshold: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_twt_teardown {
// TWT Flow Identifier. Range: [0-7]
    pub flow_identifier: u8,
//
// Negotiation Type. 0: Future Individual TWT SP start time, 1: Next
// Wake TBTT time
//
    pub negotiation_type: u8,
// Tear down all TWT. 1: To teardown all TWT, 0 otherwise
    pub teardown_all_twt: u8,
// TWT Teardown State. Set to 0 by driver, filled by FW in response
    pub twt_teardown_state: u8,
// Reserved, set to 0.
    pub reserved: [u8; 3],
    pub __packed: },
pub const NXPWIFI_BTWT_REPORT_LEN: c_int = 9;
pub const NXPWIFI_BTWT_REPORT_MAX_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_twt_report {
// TWT report type, 0: BTWT id
    pub type: u8,
// TWT report length of value in data
    pub length: u8,
    pub reserve: [u8; 2],
// TWT report payload for FW response to fill
    pub NXPWIFI_BTWT_REPORT_MAX_NUM]: *mut *mut u8 data[NXPWIFI_BTWT_REPORT_LEN,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_twt_information {
// TWT Flow Identifier. Range: [0-7]
    pub flow_identifier: u8,
//
// Suspend Duration. Range: [0-UINT32_MAX]
// 0:Suspend forever;
// Else:Suspend agreement for specific duration in milli seconds,
// after than resume the agreement and enter SP immediately
//
    pub suspend_duration: __le32,
// TWT Information State. Set to 0 by driver, filled by FW in response
    pub twt_information_state: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btwt_set {
    pub btwt_id: u8,
    pub ap_bcast_mantissa: __le16,
    pub ap_bcast_exponent: u8,
    pub nominalwake: u8,
    pub __packed: },
pub const BTWT_AGREEMENT_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_btwt_ap_config {
    pub ap_bcast_bet_sta_wait: u8,
    pub ap_bcast_offset: __le16,
    pub bcast_twtli: u8,
    pub count: u8,
    pub btwt_sets: [btwt_set; BTWT_AGREEMENT_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_twt_cfg {
    pub action: u16,
    pub sub_id: u16,
    pub twt_setup: nxpwifi_twt_setup,
    pub twt_teardown: nxpwifi_twt_teardown,
    pub twt_report: nxpwifi_twt_report,
    pub twt_information: nxpwifi_twt_information,
    pub btwt_ap_config: nxpwifi_btwt_ap_config,
    pub param: },
}
