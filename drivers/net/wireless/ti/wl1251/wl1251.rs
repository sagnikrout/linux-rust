//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/wl1251.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008-2009 Nokia Corporation
//

pub const DEBUG_DUMP_LIMIT: c_int = 1024;

pub const WL1251_BUSY_WORD_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_attr {
    pub radio_type: u32,
    pub mac_clock: u8,
    pub arm_clock: u8,
    pub firmware_debug: c_int,
    pub minor: u32,
    pub major: u32,
    pub bugfix: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_state {
    WL1251_STATE_OFF,
    WL1251_STATE_ON,
    WL1251_STATE_PLT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_partition_type {
    PART_DOWN,
    PART_WORK,
    PART_DRPW,

    PART_TABLE_LEN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_station_mode {
    STATION_ACTIVE_MODE,
    STATION_POWER_SAVE_MODE,
    STATION_IDLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_partition {
    pub size: u32,
    pub start: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_partition_set {
    pub mem: wl1251_partition,
    pub reg: wl1251_partition,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_stats {
    pub fw_stats: *mut acx_statistics,
    pub fw_stats_update: c_ulong,
    pub retry_count: c_uint,
    pub excessive_retries: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_debugfs {
    pub rootdir: *mut dentry,
    pub fw_statistics: *mut dentry,
    pub tx_internal_desc_overflow: *mut dentry,
    pub rx_out_of_mem: *mut dentry,
    pub rx_hdr_overflow: *mut dentry,
    pub rx_hw_stuck: *mut dentry,
    pub rx_dropped: *mut dentry,
    pub rx_fcs_err: *mut dentry,
    pub rx_xfr_hint_trig: *mut dentry,
    pub rx_path_reset: *mut dentry,
    pub rx_reset_counter: *mut dentry,
    pub dma_rx_requested: *mut dentry,
    pub dma_rx_errors: *mut dentry,
    pub dma_tx_requested: *mut dentry,
    pub dma_tx_errors: *mut dentry,
    pub isr_cmd_cmplt: *mut dentry,
    pub isr_fiqs: *mut dentry,
    pub isr_rx_headers: *mut dentry,
    pub isr_rx_mem_overflow: *mut dentry,
    pub isr_rx_rdys: *mut dentry,
    pub isr_irqs: *mut dentry,
    pub isr_tx_procs: *mut dentry,
    pub isr_decrypt_done: *mut dentry,
    pub isr_dma0_done: *mut dentry,
    pub isr_dma1_done: *mut dentry,
    pub isr_tx_exch_complete: *mut dentry,
    pub isr_commands: *mut dentry,
    pub isr_rx_procs: *mut dentry,
    pub isr_hw_pm_mode_changes: *mut dentry,
    pub isr_host_acknowledges: *mut dentry,
    pub isr_pci_pm: *mut dentry,
    pub isr_wakeups: *mut dentry,
    pub isr_low_rssi: *mut dentry,
    pub wep_addr_key_count: *mut dentry,
    pub wep_default_key_count: *mut dentry,
// skipping wep.reserved
    pub wep_key_not_found: *mut dentry,
    pub wep_decrypt_fail: *mut dentry,
    pub wep_packets: *mut dentry,
    pub wep_interrupt: *mut dentry,
    pub pwr_ps_enter: *mut dentry,
    pub pwr_elp_enter: *mut dentry,
    pub pwr_missing_bcns: *mut dentry,
    pub pwr_wake_on_host: *mut dentry,
    pub pwr_wake_on_timer_exp: *mut dentry,
    pub pwr_tx_with_ps: *mut dentry,
    pub pwr_tx_without_ps: *mut dentry,
    pub pwr_rcvd_beacons: *mut dentry,
    pub pwr_power_save_off: *mut dentry,
    pub pwr_enable_ps: *mut dentry,
    pub pwr_disable_ps: *mut dentry,
    pub pwr_fix_tsf_ps: *mut dentry,
// skipping cont_miss_bcns_spread for now
    pub pwr_rcvd_awake_beacons: *mut dentry,
    pub mic_rx_pkts: *mut dentry,
    pub mic_calc_failure: *mut dentry,
    pub aes_encrypt_fail: *mut dentry,
    pub aes_decrypt_fail: *mut dentry,
    pub aes_encrypt_packets: *mut dentry,
    pub aes_decrypt_packets: *mut dentry,
    pub aes_encrypt_interrupt: *mut dentry,
    pub aes_decrypt_interrupt: *mut dentry,
    pub event_heart_beat: *mut dentry,
    pub event_calibration: *mut dentry,
    pub event_rx_mismatch: *mut dentry,
    pub event_rx_mem_empty: *mut dentry,
    pub event_rx_pool: *mut dentry,
    pub event_oom_late: *mut dentry,
    pub event_phy_transmit_error: *mut dentry,
    pub event_tx_stuck: *mut dentry,
    pub ps_pspoll_timeouts: *mut dentry,
    pub ps_upsd_timeouts: *mut dentry,
    pub ps_upsd_max_sptime: *mut dentry,
    pub ps_upsd_max_apturn: *mut dentry,
    pub ps_pspoll_max_apturn: *mut dentry,
    pub ps_pspoll_utilization: *mut dentry,
    pub ps_upsd_utilization: *mut dentry,
    pub rxpipe_rx_prep_beacon_drop: *mut dentry,
    pub rxpipe_descr_host_int_trig_rx_data: *mut dentry,
    pub rxpipe_beacon_buffer_thres_host_int_trig_rx_data: *mut dentry,
    pub rxpipe_missed_beacon_host_int_trig_rx_data: *mut dentry,
    pub rxpipe_tx_xfr_host_int_trig_rx_data: *mut dentry,
    pub tx_queue_len: *mut dentry,
    pub tx_queue_status: *mut dentry,
    pub retry_count: *mut dentry,
    pub excessive_retries: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_if_operations {
    pub len): *mut *mut *mut *mut void (read)(struct wl1251 wl, int addr, void buf, size_t,
    pub len): *mut *mut *mut *mut void (write)(struct wl1251 wl, int addr, void buf, size_t,
    pub val): *mut *mut *mut void (read_elp)(struct wl1251 wl, int addr, u32,
    pub val): *mut *mut *mut void (write_elp)(struct wl1251 wl, int addr, u32,
    pub enable): *mut *mut *mut int (power)(struct wl1251 wl, bool,
    pub wl): *mut *mut void (reset)(struct wl1251,
    pub wl): *mut *mut void (enable_irq)(struct wl1251,
    pub wl): *mut *mut void (disable_irq)(struct wl1251,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251 {
    pub hw: *mut ieee80211_hw,
    pub mac80211_registered: bool,
    pub if_priv: *mut c_void,
    pub if_ops: *const wl1251_if_operations,
    pub irq: c_int,
    pub use_eeprom: bool,
    pub vio: *mut regulator,
    pub wl_lock: spinlock_t,
    pub state: wl1251_state,
    pub mutex: mutex,
    pub physical_mem_addr: c_int,
    pub physical_reg_addr: c_int,
    pub virtual_mem_addr: c_int,
    pub virtual_reg_addr: c_int,
    pub cmd_box_addr: c_int,
    pub event_box_addr: c_int,
    pub boot_attr: boot_attr,
    pub fw: *mut u8,
    pub fw_len: usize,
    pub nvs: *mut u8,
    pub nvs_len: usize,
    pub bssid: [u8; ETH_ALEN],
    pub mac_addr: [u8; ETH_ALEN],
    pub bss_type: u8,
    pub listen_int: u8,
    pub channel: c_int,
    pub monitor_present: bool,
    pub joined: bool,
    pub target_mem_map: *mut c_void,
    pub data_path: *mut acx_data_path_params_resp,
// Number of TX packets transferred to the FW, modulo 16
    pub data_in_count: u32,
// Frames scheduled for transmission, not handled yet
    pub tx_queue: sk_buff_head,
    pub tx_queue_stopped: bool,
    pub tx_work: work_struct,
// Pending TX frames
    pub tx_frames: [*mut sk_buff; 16],
//
// Index pointing to the next TX complete entry
// in the cyclic XT complete array we get from
// the FW.
//
    pub next_tx_complete: u32,
// FW Rx counter
    pub rx_counter: u32,
// Rx frames handled
    pub rx_handled: u32,
// Current double buffer
    pub rx_current_buffer: u32,
    pub rx_last_id: u32,
// The target interrupt mask
    pub intr_mask: u32,
    pub irq_work: work_struct,
// The mbox event mask
    pub event_mask: u32,
// Mailbox pointers
    pub mbox_ptr: [u32; 2],
// Are we currently scanning
    pub scanning: bool,
// Default key (for WEP)
    pub default_key: u32,
    pub tx_mgmt_frm_rate: c_uint,
    pub tx_mgmt_frm_mod: c_uint,
    pub rx_config: c_uint,
    pub rx_filter: c_uint,
// is firmware in elp mode
    pub elp: bool,
    pub elp_work: delayed_work,
    pub station_mode: wl1251_station_mode,
// PSM mode requested
    pub psm_requested: bool,
// retry counter for PSM entries
    pub psm_entry_retry: u8,
    pub beacon_int: u16,
    pub dtim_period: u8,
// in dBm
    pub power_level: c_int,
    pub rssi_thold: c_int,
    pub stats: wl1251_stats,
    pub debugfs: wl1251_debugfs,
    pub buffer_32: __le32,
    pub buffer_cmd: u32,
    pub buffer_busyword: [u8; WL1251_BUSY_WORD_LEN],
    pub rx_descriptor: *mut wl1251_rx_descriptor,
    pub vif: *mut ieee80211_vif,
    pub chip_id: u32,
    pub fw_ver: [c_char; 21],
// Most recently reported noise in dBm
    pub noise: i8,
}

extern "C" {
    pub fn wl1251_plt_start(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_plt_stop(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_free_hw(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_init_ieee80211(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_enable_interrupts(wl: *mut wl1251);
}
extern "C" {
    pub fn wl1251_disable_interrupts(wl: *mut wl1251);
}

pub const WL1251_DEFAULT_POWER_LEVEL: c_int = 20;
pub const WL1251_TX_QUEUE_LOW_WATERMARK: c_int = 10;
pub const WL1251_TX_QUEUE_HIGH_WATERMARK: c_int = 25;
pub const WL1251_DEFAULT_BEACON_INT: c_int = 100;
pub const WL1251_DEFAULT_DTIM_PERIOD: c_int = 1;
pub const WL1251_DEFAULT_CHANNEL: c_int = 0;
pub const WL1251_DEFAULT_BET_CONSECUTIVE: c_int = 10;

pub const WL1251_PART_DOWN_MEM_START: c_uint = 0x0;
pub const WL1251_PART_DOWN_MEM_SIZE: c_uint = 0x16800;

pub const WL1251_PART_WORK_MEM_START: c_uint = 0x28000;
pub const WL1251_PART_WORK_MEM_SIZE: c_uint = 0x14000;

pub const WL1251_DEFAULT_LOW_RSSI_WEIGHT: c_int = 10;
pub const WL1251_DEFAULT_LOW_RSSI_DEPTH: c_int = 10;
