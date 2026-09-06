//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_main.h
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


//
// Copyright (c) 2014 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_sta {
    pub sta: *mut ieee80211_sta,
    pub sta_id: i16,
    pub seq_start: [u16; IEEE80211_NUM_TIDS],
    pub start_tx_aggr: [bool; IEEE80211_NUM_TIDS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RSI_FSM_STATES {
    FSM_FW_NOT_LOADED,
    FSM_CARD_NOT_READY,
    FSM_COMMON_DEV_PARAMS_SENT,
    FSM_BOOT_PARAMS_SENT,
    FSM_EEPROM_READ_MAC_ADDR,
    FSM_EEPROM_READ_RF_TYPE,
    FSM_RESET_MAC_SENT,
    FSM_RADIO_CAPS_SENT,
    FSM_BB_RF_PROG_SENT,
    FSM_MAC_INIT_DONE,

    NUM_FSM_STATES
}

extern "C" {
    pub fn __printf(_arg: 2, zone: 3) void rsi_dbg(u32, fmt: *const c_char, ...) -> extern;
}
pub const RSI_MAX_BANDS: c_int = 2;
pub const RSI_MAX_VIFS: c_int = 3;
pub const NUM_EDCA_QUEUES: c_int = 4;
pub const IEEE80211_ADDR_LEN: c_int = 6;
pub const FRAME_DESC_SZ: c_int = 16;
pub const MIN_802_11_HDR_LEN: c_int = 24;
pub const RSI_DEF_KEEPALIVE: c_int = 90;
pub const RSI_WOW_KEEPALIVE: c_int = 5;
pub const RSI_BCN_MISS_THRESHOLD: c_int = 24;
pub const DATA_QUEUE_WATER_MARK: c_int = 400;
pub const MIN_DATA_QUEUE_WATER_MARK: c_int = 300;
pub const MULTICAST_WATER_MARK: c_int = 200;
pub const MAC_80211_HDR_FRAME_CONTROL: c_int = 0;
pub const WME_NUM_AC: c_int = 4;
pub const NUM_SOFT_QUEUES: c_int = 6;
pub const MAX_HW_QUEUES: c_int = 12;
pub const INVALID_QUEUE: c_uint = 0xff;
pub const MAX_CONTINUOUS_VO_PKTS: c_int = 8;
pub const MAX_CONTINUOUS_VI_PKTS: c_int = 4;
// Hardware queue info
pub const BROADCAST_HW_Q: c_int = 9;
pub const MGMT_HW_Q: c_int = 10;
pub const BEACON_HW_Q: c_int = 11;
pub const IEEE80211_MGMT_FRAME: c_uint = 0x00;
pub const IEEE80211_CTL_FRAME: c_uint = 0x04;
pub const RSI_MAX_ASSOC_STAS: c_int = 32;
pub const IEEE80211_QOS_TID: c_uint = 0x0f;
pub const IEEE80211_NONQOS_TID: c_int = 16;
pub const MAX_DEBUGFS_ENTRIES: c_int = 4;

// WoWLAN flags

pub const RSI_MAX_RX_PKTS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsi_dev_model {
    RSI_DEV_9113 = 0,
    RSI_DEV_9116
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct version_info {
    pub major: u16,
    pub minor: u16,
    pub release_num: u8,
    pub patch_num: u8,
    pub fw_ver: [u8; 8],
    pub info: },
    pub ver: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_info {
    pub rssi: i8,
    pub flags: u32,
    pub channel: u16,
    pub tid: i8,
    pub sta_id: i8,
    pub internal_hdr_size: u8,
    pub vif: *mut ieee80211_vif,
    pub vap_id: u8,
    pub have_key: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edca_queue {
    BK_Q,
    BE_Q,
    VI_Q,
    VO_Q,
    MGMT_SOFT_Q,
    MGMT_BEACON_Q
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct security_info {
    pub ptk_cipher: u32,
    pub gtk_cipher: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmm_qinfo {
    pub weight: i32,
    pub wme_params: i32,
    pub pkt_contended: i32,
    pub txop: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transmit_q_stats {
    pub 2]: u32 total_tx_pkt_send[NUM_EDCA_QUEUES +,
    pub 2]: u32 total_tx_pkt_freed[NUM_EDCA_QUEUES +,
}

pub const MAX_BGSCAN_CHANNELS_DUAL_BAND: c_int = 38;
pub const MAX_BGSCAN_PROBE_REQ_LEN: c_uint = 0x64;
pub const RSI_DEF_BGSCAN_THRLD: c_uint = 0x0;
pub const RSI_DEF_ROAM_THRLD: c_uint = 0xa;
pub const RSI_BGSCAN_PERIODICITY: c_uint = 0x1e;
pub const RSI_ACTIVE_SCAN_TIME: c_uint = 0x14;
pub const RSI_PASSIVE_SCAN_TIME: c_uint = 0x46;
pub const RSI_CHANNEL_SCAN_TIME: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_bgscan_params {
    pub bgscan_threshold: u16,
    pub roam_threshold: u16,
    pub bgscan_periodicity: u16,
    pub num_bgscan_channels: u8,
    pub two_probe: u8,
    pub active_scan_duration: u16,
    pub passive_scan_duration: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vif_priv {
    pub is_ht: bool,
    pub sgi: bool,
    pub seq_start: u16,
    pub vap_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_event {
    pub event_condition: core::sync::atomic::AtomicI32,
    pub event_queue: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_thread {
    pub ): *mut *mut void (thread_function)(void,
    pub completion: completion,
    pub task: *mut task_struct,
    pub event: rsi_event,
    pub thread_done: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqm_info {
    pub last_cqm_event_rssi: i8,
    pub rssi_thold: c_int,
    pub rssi_hyst: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsi_dfs_regions {
    RSI_REGION_FCC = 0,
    RSI_REGION_ETSI,
    RSI_REGION_TELEC,
    RSI_REGION_WORLD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_9116_features {
    pub pll_mode: u8,
    pub rf_type: u8,
    pub wireless_mode: u8,
    pub afe_type: u8,
    pub enable_ppe: u8,
    pub dpd: u8,
    pub sifs_tx_enable: u32,
    pub ps_options: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_rate_config {
    pub /: *mut *mut u32 configured_mask; / configured by mac80211 bits 0-11=legacy 12+ mcs,
    pub fixed_hw_rate: u16,
    pub fixed_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_common {
    pub priv: *mut rsi_hw,
    pub vif_info: [vif_priv; RSI_MAX_VIFS],
    pub coex_cb: *mut c_void,
    pub mgmt_q_block: bool,
    pub lmac_ver: version_info,
    pub tx_thread: rsi_thread,
    pub 2]: sk_buff_head tx_queue[NUM_EDCA_QUEUES +,
    pub wlan_init_completion: completion,
// Mutex declaration
    pub mutex: mutex,
// Mutex used for tx thread
    pub tx_lock: mutex,
// Mutex used for rx thread
    pub rx_lock: mutex,
    pub endpoint: u8,
// Channel/band related
    pub band: u8,
    pub num_supp_bands: u8,
    pub channel_width: u8,
    pub rts_threshold: u16,
    pub bitrate_mask: [u32; RSI_MAX_BANDS],
    pub rate_config: [rsi_rate_config; RSI_MAX_BANDS],
    pub rf_reset: u8,
    pub tx_stats: transmit_q_stats,
    pub secinfo: security_info,
    pub tx_qinfo: [wmm_qinfo; NUM_EDCA_QUEUES],
    pub edca_params: [ieee80211_tx_queue_params; NUM_EDCA_QUEUES],
    pub mac_addr: [u8; IEEE80211_ADDR_LEN],
// state related
    pub fsm_state: u32,
    pub init_done: bool,
    pub bb_rf_prog_count: u8,
    pub iface_down: bool,
// Generic
    pub channel: u8,
    pub rx_data_pkt: *mut u8,
    pub mac_id: u8,
    pub radio_id: u8,
    pub rate_pwr: [u16; 20],
// WMM algo related
    pub selected_qnum: u8,
    pub pkt_cnt: u32,
    pub min_weight: u8,
// bgscan related
    pub cqm_info: cqm_info,
    pub hw_data_qs_blocked: bool,
    pub driver_mode: u8,
    pub coex_mode: u8,
    pub oper_mode: u16,
    pub lp_ps_handshake_mode: u8,
    pub ulp_ps_handshake_mode: u8,
    pub uapsd_bitmap: u8,
    pub rf_power_val: u8,
    pub wlan_rf_power_mode: u8,
    pub obm_ant_sel_val: u8,
    pub tx_power: c_int,
    pub ant_in_use: u8,
// Mutex used for writing packet to bus
    pub tx_bus_mutex: mutex,
    pub hibernate_resume: bool,
    pub reinit_hw: bool,
    pub wow_flags: u8,
    pub beacon_interval: u16,
    pub dtim_cnt: u8,
// AP mode parameters
    pub beacon_enabled: u8,
    pub beacon_cnt: u16,
    pub 1]: rsi_sta stations[RSI_MAX_ASSOC_STAS +,
    pub num_stations: c_int,
    pub max_stations: c_int,
    pub key: *mut ieee80211_key_conf,
// Wi-Fi direct mode related
    pub p2p_enabled: bool,
    pub roc_timer: timer_list,
    pub roc_vif: *mut ieee80211_vif,
    pub eapol4_confirm: bool,
    pub bt_defer_attach: bool,
    pub bt_adapter: *mut c_void,
    pub hwscan: *mut cfg80211_scan_request,
    pub bgscan: rsi_bgscan_params,
    pub w9116_features: rsi_9116_features,
    pub bgscan_en: u8,
    pub mac_ops_resumed: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eepromrw_info {
    pub offset: u32,
    pub length: u32,
    pub write: u8,
    pub eeprom_erase: u16,
    pub data: [u8; 480],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_read {
    pub length: u16,
    pub off_set: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_hw {
    pub priv: *mut rsi_common,
    pub device_model: rsi_dev_model,
    pub hw: *mut ieee80211_hw,
    pub vifs: [*mut ieee80211_vif; RSI_MAX_VIFS],
    pub edca_params: [ieee80211_tx_queue_params; NUM_EDCA_QUEUES],
    pub sbands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub device: *mut device,
    pub sc_nvifs: u8,
    pub rsi_host_intf: rsi_host_intf,
    pub block_size: u16,
    pub ps_state: ps_state,
    pub ps_info: rsi_ps_info,
    pub config*/: *mut *mut spinlock_t ps_lock; /To protect power save,
    pub usb_buffer_status_reg: u32,

    pub dfsentry: *mut rsi_debugfs,
    pub num_debugfs_entries: u8,

    pub fw_file_name: *mut c_char,
    pub bl_cmd_timer: timer_list,
    pub blcmd_timer_expired: bool,
    pub flash_capacity: u32,
    pub eeprom: eepromrw_info,
    pub interrupt_status: u32,
    pub dfs_region: u8,
    pub country: [c_char; 2],
    pub rsi_dev: *mut c_void,
    pub host_intf_ops: *mut rsi_host_intf_ops,
    pub q_num): *mut *mut *mut int (check_hw_queue_status)(struct rsi_hw adapter, u8,
    pub adapter): *mut *mut int (determine_event_timeout)(struct rsi_hw,
}

extern "C" {
    pub fn rsi_print_version(common: *mut rsi_common);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_host_intf_ops {
    pub len): *mut *mut *mut *mut int (read_pkt)(struct rsi_hw adapter, u8 pkt, u32,
    pub len): *mut *mut *mut *mut int (write_pkt)(struct rsi_hw adapter, u8 pkt, u32,
    pub ms_word): *mut *mut *mut int (master_access_msword)(struct rsi_hw adapter, u16,
    pub count): *mut *mut u8 data, u16,
    pub count): *mut *mut u8 data, u16,
    pub size): *mut *mut u32 read_buf, u16,
    pub size): u16,
    pub fw): *mut u8,
    pub adapter): *mut *mut int (reinit_device)(struct rsi_hw,
    pub adapter): *mut *mut int (ta_reset)(struct rsi_hw,
}

extern "C" {
    pub fn rsi_get_host_intf(priv: *mut c_void) -> rsi_host_intf;
}
extern "C" {
    pub fn rsi_set_bt_context(priv: *mut c_void, bt_context: *mut c_void);
}
extern "C" {
    pub fn rsi_attach_bt(common: *mut rsi_common);
}
