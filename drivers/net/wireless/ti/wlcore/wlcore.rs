//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/wlcore.h
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
// This file is part of wlcore
//
// Copyright (C) 2011 Texas Instruments Inc.
//

// The maximum number of Tx descriptors in all chip families
pub const WLCORE_MAX_TX_DESCRIPTORS: c_int = 32;
//
// We always allocate this number of mac addresses. If we don't
// have enough allocated addresses, the LAA bit is used
//
pub const WLCORE_NUM_MAC_ADDRESSES: c_int = 3;
// wl12xx/wl18xx maximum transmission power (in dBm)
pub const WLCORE_MAX_TXPWR: c_int = 25;
// Texas Instruments pre assigned OUI
pub const WLCORE_TI_OUI_ADDRESS: c_uint = 0x080028;
// forward declaration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_ops {
    pub wl): *mut *mut int (setup)(struct wl1271,
    pub wl): *mut *mut int (identify_chip)(struct wl1271,
    pub wl): *mut *mut int (identify_fw)(struct wl1271,
    pub wl): *mut *mut int (boot)(struct wl1271,
    pub wl): *mut *mut int (plt_init)(struct wl1271,
    pub len): *mut *mut void buf, size_t,
    pub wl): *mut *mut int (ack_event)(struct wl1271,
    pub timeout): *mut bool,
    pub wl): *mut *mut int (process_mailbox_events)(struct wl1271,
    pub spare_blks): *mut *mut *mut u32 (calc_tx_blocks)(struct wl1271 wl, u32 len, u32,
    pub spare_blks): u32 blks, u32,
    pub skb): *mut sk_buff,
    pub rx_desc): u32,
    pub len): *mut *mut *mut int (prepare_read)(struct wl1271 wl, u32 rx_desc, u32,
    pub data_len): u32,
    pub wl): *mut *mut int (tx_delayed_compl)(struct wl1271,
    pub wl): *mut *mut void (tx_immediate_compl)(struct wl1271,
    pub wl): *mut *mut int (hw_init)(struct wl1271,
    pub wlvif): *mut *mut *mut int (init_vif)(struct wl1271 wl, struct wl12xx_vif,
    pub fw_status): *mut wl_fw_status,
    pub wlvif): *mut wl12xx_vif,
    pub ver): *mut *mut *mut int (get_pg_ver)(struct wl1271 wl, s8,
    pub wl): *mut *mut int (get_mac)(struct wl1271,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub wlvif): *mut wl12xx_vif,
    pub rootdir): *mut *mut *mut int (debugfs_init)(struct wl1271 wl, struct dentry,
    pub static_data): *mut wl1271_static_data,
    pub req): *mut cfg80211_scan_request,
    pub wlvif): *mut *mut *mut int (scan_stop)(struct wl1271 wl, struct wl12xx_vif,
    pub ies): *mut ieee80211_scan_ies,
    pub wlvif): *mut *mut *mut void (sched_scan_stop)(struct wl1271 wl, struct wl12xx_vif,
    pub is_gem): *mut *mut *mut int (get_spare_blocks)(struct wl1271 wl, bool,
    pub key_conf): *mut ieee80211_key_conf,
    pub ch_switch): *mut ieee80211_channel_switch,
    pub last_len): *mut *mut *mut u32 (pre_pkt_send)(struct wl1271 wl, u32 buf_offset, u32,
    pub wlvif): *mut *mut *mut void (sta_rc_update)(struct wl1271 wl, struct wl12xx_vif,
    pub hlid): u32 rate_set, u8,
    pub hwaddr): *mut *mut *mut u32 (convert_hwaddr)(struct wl1271 wl, u32,
    pub lnk): *mut wl1271_link,
    pub lnk): *mut wl1271_link,
    pub action): *mut *mut *mut int (interrupt_notify)(struct wl1271 wl, bool,
    pub action): *mut *mut *mut int (rx_ba_filter)(struct wl1271 wl, bool,
    pub wl): *mut *mut int (ap_sleep)(struct wl1271,
    pub group_bitmap): *mut *mut *mut int (smart_config_start)(struct wl1271 wl, u32,
    pub wl): *mut *mut int (smart_config_stop)(struct wl1271,
    pub key): *mut u8 key_len, u8,
    pub start): bool,
    pub wlvif): *mut *mut *mut int (dfs_master_restart)(struct wl1271 wl, struct wl12xx_vif,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_partitions {
    PART_DOWN,
    PART_WORK,
    PART_BOOT,
    PART_DRPW,
    PART_TOP_PRCM_ELP_SOC,
    PART_PHY_INIT,

    PART_TABLE_LEN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_partition {
    pub size: u32,
    pub start: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_partition_set {
    pub mem: wlcore_partition,
    pub reg: wlcore_partition,
    pub mem2: wlcore_partition,
    pub mem3: wlcore_partition,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_registers {
// register addresses, used with partition translation
    REG_ECPU_CONTROL,
    REG_INTERRUPT_NO_CLEAR,
    REG_INTERRUPT_ACK,
    REG_COMMAND_MAILBOX_PTR,
    REG_EVENT_MAILBOX_PTR,
    REG_INTERRUPT_TRIG,
    REG_INTERRUPT_MASK,
    REG_PC_ON_RECOVERY,
    REG_CHIP_ID_B,
    REG_CMD_MBOX_ADDRESS,

// data access memory addresses, used with partition translation
    REG_SLV_MEM_DATA,
    REG_SLV_REG_DATA,

// raw data access memory addresses
    REG_RAW_FW_STATUS_ADDR,

    REG_TABLE_LEN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_stats {
    pub fw_stats: *mut c_void,
    pub fw_stats_update: c_ulong,
    pub fw_stats_len: usize,
    pub retry_count: c_uint,
    pub excessive_retries: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271 {
    pub initialized: bool,
    pub hw: *mut ieee80211_hw,
    pub mac80211_registered: bool,
    pub dev: *mut device,
    pub pdev: *mut platform_device,
    pub if_priv: *mut c_void,
    pub if_ops: *mut wl1271_if_operations,
    pub irq: c_int,
    pub wakeirq: c_int,
    pub irq_flags: c_int,
    pub wakeirq_flags: c_int,
    pub wl_lock: spinlock_t,
    pub state: wlcore_state,
    pub fw_type: wl12xx_fw_type,
    pub plt: bool,
    pub plt_mode: plt_mode,
    pub fem_manuf: u8,
    pub last_vif_count: u8,
    pub mutex: mutex,
    pub flags: c_ulong,
    pub curr_part: wlcore_partition_set,
    pub chip: wl1271_chip,
    pub cmd_box_addr: c_int,
    pub fw: *mut u8,
    pub fw_len: usize,
    pub nvs: *mut c_void,
    pub nvs_len: usize,
    pub hw_pg_ver: i8,
// address read from the fuse ROM
    pub fuse_oui_addr: u32,
    pub fuse_nic_addr: u32,
// we have up to 2 MAC addresses
    pub addresses: [mac_address; WLCORE_NUM_MAC_ADDRESSES],
    pub channel: c_int,
    pub system_hlid: u8,
    pub links_map: [c_ulong; BITS_TO_LONGS(WLCORE_MAX_LINKS)],
    pub roles_map: [c_ulong; BITS_TO_LONGS(WL12XX_MAX_ROLES)],
    pub roc_map: [c_ulong; BITS_TO_LONGS(WL12XX_MAX_ROLES)],
    pub session_ids: [u8; WLCORE_MAX_LINKS],
    pub wlvif_list: list_head,
    pub sta_count: u8,
    pub ap_count: u8,
    pub target_mem_map: *mut wl1271_acx_mem_map,
// Accounting for allocated / available TX blocks on HW
    pub tx_blocks_freed: u32,
    pub tx_blocks_available: u32,
    pub tx_allocated_blocks: u32,
    pub tx_results_count: u32,
// Accounting for allocated / available Tx packets in HW
    pub tx_pkts_freed: [u32; NUM_TX_QUEUES],
    pub tx_allocated_pkts: [u32; NUM_TX_QUEUES],
// Transmitted TX packets counter for chipset interface
    pub tx_packets_count: u32,
// Time-offset between host and chipset clocks
    pub time_offset: i64,
// Frames scheduled for transmission, not handled yet
    pub tx_queue_count: [c_int; NUM_TX_QUEUES],
    pub WLCORE_NUM_MAC_ADDRESSES]: *mut *mut NUM_TX_QUEUES,
// Frames received, not handled yet by mac80211
    pub deferred_rx_queue: sk_buff_head,
// Frames sent, not returned yet to mac80211
    pub deferred_tx_queue: sk_buff_head,
    pub tx_work: work_struct,
    pub freezable_wq: *mut workqueue_struct,
// Pending TX frames
    pub tx_frames_map: [c_ulong; BITS_TO_LONGS(WLCORE_MAX_TX_DESCRIPTORS)],
    pub tx_frames: [*mut sk_buff; WLCORE_MAX_TX_DESCRIPTORS],
    pub tx_frames_cnt: c_int,
// FW Rx counter
    pub rx_counter: u32,
// Intermediate buffer, used for packet aggregation
    pub aggr_buf: *mut u8,
    pub aggr_buf_size: u32,
// Reusable dummy packet template
    pub dummy_packet: *mut sk_buff,
// Network stack work
    pub netstack_work: work_struct,
// FW log buffer
    pub fwlog: *mut u8,
// Number of valid bytes in the FW log buffer
    pub fwlog_size: isize,
// FW log end marker
    pub fwlog_end: u32,
// FW memory block size
    pub fw_mem_block_size: u32,
// Hardware recovery work
    pub recovery_work: work_struct,
    pub watchdog_recovery: bool,
// Reg domain last configuration
    pub 64): DECLARE_BITMAP(reg_ch_conf_last,,
// Reg domain pending configuration
    pub 64): DECLARE_BITMAP(reg_ch_conf_pending,,
// Pointer that holds DMA-friendly block for the mailbox
    pub mbox: *mut c_void,
// The mbox event mask
    pub event_mask: u32,
// events to unmask only when ap interface is up
    pub ap_event_mask: u32,
// Mailbox pointers
    pub mbox_size: u32,
    pub mbox_ptr: [u32; 2],
// Are we currently scanning
    pub scan_wlvif: *mut wl12xx_vif,
    pub scan: wl1271_scan,
    pub scan_complete_work: delayed_work,
    pub roc_vif: *mut ieee80211_vif,
    pub roc_complete_work: delayed_work,
    pub sched_vif: *mut wl12xx_vif,
// The current band
    pub band: nl80211_band,
    pub elp_compl: *mut completion,
// in dBm
    pub power_level: c_int,
    pub stats: wl1271_stats,
    pub buffer_32: *mut __le32,
    pub buffer_cmd: u32,
    pub buffer_busyword: [u32; WL1271_BUSY_WORD_CNT],
    pub raw_fw_status: *mut c_void,
    pub fw_status: *mut wl_fw_status,
    pub tx_res_if: *mut wl1271_tx_hw_res_if,
// Current chipset configuration
    pub conf: wlcore_conf,
    pub sg_enabled: bool,
    pub enable_11a: bool,
    pub recovery_count: c_int,
// Most recently reported noise in dBm
    pub noise: i8,
// bands supported by this instance of wl12xx
    pub bands: [ieee80211_supported_band; WLCORE_NUM_BANDS],
//
// wowlan trigger was configured during suspend.
// (currently, only "ANY" trigger is supported)
//
    pub wow_enabled: bool,
    pub irq_wake_enabled: bool,
//
// AP-mode - links indexed by HLID. The global and broadcast links
// are always active.
//
    pub links: [wl1271_link; WLCORE_MAX_LINKS],
// number of currently active links
    pub active_link_count: c_int,
// Fast/slow links bitmap according to FW
    pub fw_fast_lnk_map: c_ulong,
// AP-mode - a bitmap of links currently in PS mode according to FW
    pub ap_fw_ps_map: c_ulong,
// AP-mode - a bitmap of links currently in PS mode in mac80211
    pub ap_ps_map: c_ulong,
// Quirks of specific hardware revisions
    pub quirks: c_uint,
// number of currently active RX BA sessions
    pub ba_rx_session_count: c_int,
// Maximum number of supported RX BA sessions
    pub ba_rx_session_count_max: c_int,
// AP-mode - number of currently connected stations
    pub active_sta_count: c_int,
// Flag determining whether AP should broadcast OFDM-only rates
    pub ofdm_only_ap: bool,
// last wlvif we transmitted from
    pub last_wlvif: *mut wl12xx_vif,
// work to fire when Tx is stuck
    pub tx_watchdog_work: delayed_work,
    pub ops: *mut wlcore_ops,
// pointer to the lower driver partition table
    pub ptable: *const wlcore_partition_set,
// pointer to the lower driver register table
    pub rtable: *const c_int,
// name of the firmwares to load - for PLT, single role, multi-role
    pub plt_fw_name: *const c_char,
    pub sr_fw_name: *const c_char,
    pub mr_fw_name: *const c_char,
    pub scan_templ_id_2_4: u8,
    pub scan_templ_id_5: u8,
    pub sched_scan_templ_id_2_4: u8,
    pub sched_scan_templ_id_5: u8,
    pub max_channels_5: u8,
// per-chip-family private structure
    pub priv: *mut c_void,
// number of TX descriptors the HW supports.
    pub num_tx_desc: u32,
// number of RX descriptors the HW supports.
    pub num_rx_desc: u32,
// number of links the HW supports
    pub num_links: u8,
// max stations a single AP can support
    pub max_ap_stations: u8,
// translate HW Tx rates to standard rate-indices
    pub band_rate_to_idx: *const u8,
// size of table for HW rates that can be received from chip
    pub hw_tx_rate_tbl_size: u8,
// this HW rate and below are considered HT rates for this chip
    pub hw_min_ht_rate: u8,
// HW HT (11n) capabilities
    pub ht_cap: [ieee80211_sta_ht_cap; WLCORE_NUM_BANDS],
// the current dfs region
    pub dfs_region: nl80211_dfs_regions,
    pub radar_debug_mode: bool,
// size of the private FW status data
    pub fw_status_len: usize,
    pub fw_status_priv_len: usize,
// RX Data filter rule state - enabled/disabled
    pub rx_filter_enabled: [c_ulong; BITS_TO_LONGS(WL1271_MAX_RX_FILTERS)],
// size of the private static data
    pub static_data_priv_len: usize,
// the current channel type
    pub channel_type: nl80211_channel_type,
// mutex for protecting the tx_flush function
    pub flush_mutex: mutex,
// sleep auth value currently configured to FW
    pub sleep_auth: c_int,
// the number of allocated MAC addresses in this chip
    pub num_mac_addr: c_int,
// minimum FW version required for the driver to work in single-role
    pub min_sr_fw_ver: [c_uint; NUM_FW_VER],
// minimum FW version required for the driver to work in multi-role
    pub min_mr_fw_ver: [c_uint; NUM_FW_VER],
    pub nvs_loading_complete: completion,
// interface combinations supported by the hw
    pub iface_combinations: *const ieee80211_iface_combination,
    pub n_iface_combinations: u8,
// dynamic fw traces
    pub dynamic_fw_traces: u32,
// time sync zone master
    pub zone_master_mac_addr: [u8; ETH_ALEN],
}

extern "C" {
    pub fn wlcore_probe(wl: *mut wl1271, pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn wlcore_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn wlcore_free_hw(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wlcore_regdomain_config(wl: *mut wl1271);
}
// Tell wlcore not to care about this element when checking the version

// Firmware image load chunk size
pub const CHUNK_SIZE: c_int = 16384;
// Quirks
// Each RX/TX transaction requires an end-of-transaction transfer

// wl127x and SPI don't support SDIO block size alignment

// means aggregated Rx packets are aligned to a SDIO block

// Older firmwares did not implement the FW logger over bus feature

// Older firmwares use an old NVS format

// pad only the last frame in the aggregate buffer

// extra header space is required for TKIP

// Some firmwares not support sched scans while connected

// separate probe response templates for one-shot and sched scans

// Firmware requires reg domain configuration for active calibration

// The FW only support a zero session id for AP

// TODO: move all these common registers and values elsewhere
pub const HW_ACCESS_ELP_CTRL_REG: c_uint = 0x1FFFC;
// ELP register commands
pub const ELPCTRL_WAKE_UP: c_uint = 0x1;
pub const ELPCTRL_WAKE_UP_WLAN_READY: c_uint = 0x5;
pub const ELPCTRL_SLEEP: c_uint = 0x0;
// ELP WLAN_READY bit
pub const ELPCTRL_WLAN_READY: c_uint = 0x2;
//
// Hardware to Embedded CPU Interrupts - first 32-bit register set
//
// The host sets this bit to inform the Wlan
// FW that a TX packet is in the XFER
// Buffer #0.
//

//
// The host sets this bit to inform the FW
// that it read a packet from RX XFER
// Buffer #0.
//

// Hardware to Embedded CPU Interrupts - second 32-bit register set
//
// The host sets this bit to inform the FW
// that it read a packet from RX XFER
// Buffer #1.
//

//
// The host sets this bit to inform the Wlan
// hardware that a TX packet is in the XFER
// Buffer #1.
//

pub const SOFT_RESET_MAX_TIME: c_int = 1000000;
pub const SOFT_RESET_STALL_TIME: c_int = 1000;
pub const ECPU_CONTROL_HALT: c_uint = 0x00000101;
pub const WELP_ARM_COMMAND_VAL: c_uint = 0x4;
