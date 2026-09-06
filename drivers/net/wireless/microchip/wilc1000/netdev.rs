//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/netdev.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//

pub const FLOW_CONTROL_LOWER_THRESHOLD: c_int = 128;
pub const FLOW_CONTROL_UPPER_THRESHOLD: c_int = 256;
pub const PMKID_FOUND: c_int = 1;
pub const NUM_STA_ASSOCIATED: c_int = 8;
pub const TCP_ACK_FILTER_LINK_SPEED_THRESH: c_int = 54;
pub const DEFAULT_LINK_SPEED: c_int = 72;
pub const TX_BACKOFF_WEIGHT_MS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wfi_stats {
    pub rx_packets: c_ulong,
    pub tx_packets: c_ulong,
    pub rx_bytes: c_ulong,
    pub tx_bytes: c_ulong,
    pub rx_time: u64,
    pub tx_time: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wfi_key {
    pub key: *mut u8,
    pub seq: *mut u8,
    pub key_len: c_int,
    pub seq_len: c_int,
    pub cipher: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_info {
    pub sta_associated_bss: [u8; WILC_MAX_NUM_STA][ETH_ALEN],
}

// Parameters needed for host interface for remaining on channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wfi_p2p_listen_params {
    pub listen_ch: *mut ieee80211_channel,
    pub listen_duration: u32,
    pub listen_cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_priv {
    pub wdev: wireless_dev,
    pub scan_req: *mut cfg80211_scan_request,
    pub remain_on_ch_params: wilc_wfi_p2p_listen_params,
    pub tx_cookie: u64,
    pub cfg_scanning: bool,
    pub associated_bss: [u8; ETH_ALEN],
    pub assoc_stainfo: sta_info,
    pub skb: *mut sk_buff,
    pub dev: *mut net_device,
    pub hif_drv: *mut host_if_drv,
    pub pmkid_list: wilc_pmkid_attr,
// The real interface that the monitor is on
    pub real_ndev: *mut net_device,
    pub wilc_gtk: [*mut wilc_wfi_key; WILC_MAX_NUM_STA],
    pub wilc_ptk: [*mut wilc_wfi_key; WILC_MAX_NUM_STA],
    pub wilc_igtk: [*mut wilc_wfi_key; 2],
    pub wilc_groupkey: u8,
// mutexes
    pub scan_req_lock: mutex,
    pub p2p_listen_state: bool,
    pub scanned_cnt: c_int,
    pub inc_roc_cookie: u64,
}

pub const MAX_TCP_SESSION: c_int = 25;
pub const MAX_PENDING_ACKS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ack_session_info {
    pub seq_num: u32,
    pub bigger_ack_num: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub status: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_acks {
    pub ack_num: u32,
    pub session_index: u32,
    pub txqe: *mut txq_entry_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ack_filter {
    pub MAX_TCP_SESSION]: *mut *mut ack_session_info ack_session_info[2,
    pub pending_acks: [pending_acks; MAX_PENDING_ACKS],
    pub pending_base: u32,
    pub tcp_session: u32,
    pub pending_acks_idx: u32,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_vif {
    pub idx: u8,
    pub iftype: u8,
    pub monitor_flag: c_int,
    pub mac_opened: c_int,
    pub mgmt_reg_stypes: u32,
    pub netstats: net_device_stats,
    pub wilc: *mut wilc,
    pub bssid: [u8; ETH_ALEN],
    pub hif_drv: *mut host_if_drv,
    pub ndev: *mut net_device,
    pub during_ip_timer: timer_list,
    pub periodic_rssi: timer_list,
    pub periodic_stat: rf_info,
    pub ack_filter: tcp_ack_filter,
    pub connecting: bool,
    pub priv: wilc_priv,
    pub list: list_head,
    pub bss: *mut cfg80211_bss,
    pub auth: cfg80211_external_auth_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_tx_queue_status {
    pub buffer: [u8; AC_BUFFER_SIZE],
    pub end_index: u16,
    pub cnt: [u16; NQUEUES],
    pub sum: u16,
    pub initialized: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc {
    pub wiphy: *mut wiphy,
    pub hif_func: *const wilc_hif_func,
    pub io_type: c_int,
    pub mac_status: i8,
    pub rtc_clk: *mut clk,
    pub initialized: bool,
    pub chipid: u32,
    pub power_save_mode: bool,
    pub dev_irq_num: c_int,
    pub close: c_int,
    pub vif_num: u8,
    pub vif_list: list_head,
// protect vif list
    pub vif_mutex: mutex,
// Sleepable RCU struct to manipulate vif list. Sleepable version is
// needed over the classic RCU version because the driver's current
// design involves some sleeping code while manipulating a vif
// retrieved from vif list (so in a SRCU critical section), like:
// - sending commands to the chip, using info from retrieved vif
// - registering a new monitoring net device
//
    pub srcu: srcu_struct,
    pub open_ifcs: u8,
// protect head of transmit queue
    pub txq_add_to_head_cs: mutex,
// protect txq_entry_t transmit queue
    pub txq_spinlock: spinlock_t,
// protect rxq_entry_t receiver queue
    pub rxq_cs: mutex,
// lock to protect hif access
    pub hif_cs: mutex,
    pub cfg_event: completion,
    pub sync_event: completion,
    pub txq_event: completion,
    pub txq_thread_started: completion,
    pub txq_thread: *mut task_struct,
    pub quit: c_int,
// lock to protect issue of wid command to firmware
    pub cfg_cmd_lock: mutex,
    pub cfg_frame: wilc_cfg_frame,
    pub cfg_frame_offset: u32,
    pub cfg_seq_no: u8,
    pub rx_buffer: *mut u8,
    pub rx_buffer_offset: u32,
    pub tx_buffer: *mut u8,
    pub vmm_table: *mut u32,
    pub txq: [txq_handle; NQUEUES],
    pub txq_entries: c_int,
    pub tx_q_limit: wilc_tx_queue_status,
    pub rxq_head: rxq_entry_t,
    pub firmware: *const firmware,
    pub dev: *mut device,
    pub hif_workqueue: *mut workqueue_struct,
    pub cfg: wilc_cfg,
    pub bus_data: *mut c_void,
    pub monitor_dev: *mut net_device,
// deinit lock
    pub deinit_lock: mutex,
    pub sta_ch: u8,
    pub op_ch: u8,
    pub channels: [ieee80211_channel; ARRAY_SIZE(wilc_2ghz_channels)],
    pub bitrates: [ieee80211_rate; ARRAY_SIZE(wilc_bitrates)],
    pub band: ieee80211_supported_band,
    pub cipher_suites: [u32; ARRAY_SIZE(wilc_cipher_suites)],
    pub nv_mac_address: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_wfi_mon_priv {
    pub real_ndev: *mut net_device,
}

extern "C" {
    pub fn wilc_frmw_to_host(wilc: *mut wilc, buff: *mut u8, size: u32, pkt_offset: u32);
}
extern "C" {
    pub fn wilc_mac_indicate(wilc: *mut wilc);
}
extern "C" {
    pub fn wilc_netdev_cleanup(wilc: *mut wilc);
}
extern "C" {
    pub fn wilc_wfi_mgmt_rx(wilc: *mut wilc, buff: *mut u8, size: u32, is_auth: bool);
}
