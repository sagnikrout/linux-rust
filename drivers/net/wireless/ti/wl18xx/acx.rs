//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/acx.h
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
// This file is part of wl18xx
//
// Copyright (C) 2011 Texas Instruments. All rights reserved.
//

// numbers of bits the length field takes (add 1 for the actual number)
pub const WL18XX_HOST_IF_LEN_SIZE_FIELD: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_host_config_bitmap {
    pub header: acx_header,
    pub host_cfg_bitmap: __le32,
    pub host_sdio_block_size: __le32,
// extra mem blocks per frame in TX.
    pub extra_mem_blocks: __le32,
//
// number of bits of the length field in the first TX word
// (up to 15 - for using the entire 16 bits).
//
    pub length_field_size: __le32,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_checksum_state {
    pub header: acx_header,
// enum acx_checksum_state
    pub checksum_state: u8,
    pub pad: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_error_stats {
    pub error_frame_non_ctrl: u32,
    pub error_frame_ctrl: u32,
    pub error_frame_during_protection: u32,
    pub null_frame_tx_start: u32,
    pub null_frame_cts_start: u32,
    pub bar_retry: u32,
    pub num_frame_cts_nul_flid: u32,
    pub tx_abort_failure: u32,
    pub tx_resume_failure: u32,
    pub rx_cmplt_db_overflow_cnt: u32,
    pub elp_while_rx_exch: u32,
    pub elp_while_tx_exch: u32,
    pub elp_while_tx: u32,
    pub elp_while_nvic_pending: u32,
    pub rx_excessive_frame_len: u32,
    pub burst_mismatch: u32,
    pub tbc_exch_mismatch: u32,
    pub __packed: },
pub const NUM_OF_RATES_INDEXES: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_tx_stats {
    pub tx_prepared_descs: u32,
    pub tx_cmplt: u32,
    pub tx_template_prepared: u32,
    pub tx_data_prepared: u32,
    pub tx_template_programmed: u32,
    pub tx_data_programmed: u32,
    pub tx_burst_programmed: u32,
    pub tx_starts: u32,
    pub tx_stop: u32,
    pub tx_start_templates: u32,
    pub tx_start_int_templates: u32,
    pub tx_start_fw_gen: u32,
    pub tx_start_data: u32,
    pub tx_start_null_frame: u32,
    pub tx_exch: u32,
    pub tx_retry_template: u32,
    pub tx_retry_data: u32,
    pub tx_retry_per_rate: [u32; NUM_OF_RATES_INDEXES],
    pub tx_exch_pending: u32,
    pub tx_exch_expiry: u32,
    pub tx_done_template: u32,
    pub tx_done_data: u32,
    pub tx_done_int_template: u32,
    pub tx_cfe1: u32,
    pub tx_cfe2: u32,
    pub frag_called: u32,
    pub frag_mpdu_alloc_failed: u32,
    pub frag_init_called: u32,
    pub frag_in_process_called: u32,
    pub frag_tkip_called: u32,
    pub frag_key_not_found: u32,
    pub frag_need_fragmentation: u32,
    pub frag_bad_mblk_num: u32,
    pub frag_failed: u32,
    pub frag_cache_hit: u32,
    pub frag_cache_miss: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_rx_stats {
    pub rx_beacon_early_term: u32,
    pub rx_out_of_mpdu_nodes: u32,
    pub rx_hdr_overflow: u32,
    pub rx_dropped_frame: u32,
    pub rx_done_stage: u32,
    pub rx_done: u32,
    pub rx_defrag: u32,
    pub rx_defrag_end: u32,
    pub rx_cmplt: u32,
    pub rx_pre_complt: u32,
    pub rx_cmplt_task: u32,
    pub rx_phy_hdr: u32,
    pub rx_timeout: u32,
    pub rx_rts_timeout: u32,
    pub rx_timeout_wa: u32,
    pub defrag_called: u32,
    pub defrag_init_called: u32,
    pub defrag_in_process_called: u32,
    pub defrag_tkip_called: u32,
    pub defrag_need_defrag: u32,
    pub defrag_decrypt_failed: u32,
    pub decrypt_key_not_found: u32,
    pub defrag_need_decrypt: u32,
    pub rx_tkip_replays: u32,
    pub rx_xfr: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_isr_stats {
    pub irqs: u32,
    pub __packed: },
pub const PWR_STAT_MAX_CONT_MISSED_BCNS_SPREAD: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_pwr_stats {
    pub missing_bcns_cnt: u32,
    pub rcvd_bcns_cnt: u32,
    pub connection_out_of_sync: u32,
    pub cont_miss_bcns_spread: [u32; PWR_STAT_MAX_CONT_MISSED_BCNS_SPREAD],
    pub rcvd_awake_bcns_cnt: u32,
    pub sleep_time_count: u32,
    pub sleep_time_avg: u32,
    pub sleep_cycle_avg: u32,
    pub sleep_percent: u32,
    pub ap_sleep_active_conf: u32,
    pub ap_sleep_user_conf: u32,
    pub ap_sleep_counter: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_rx_filter_stats {
    pub beacon_filter: u32,
    pub arp_filter: u32,
    pub mc_filter: u32,
    pub dup_filter: u32,
    pub data_filter: u32,
    pub ibss_filter: u32,
    pub protection_filter: u32,
    pub accum_arp_pend_requests: u32,
    pub max_arp_queue_dep: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_rx_rate_stats {
    pub rx_frames_per_rates: [u32; 50],
    pub __packed: },
pub const AGGR_STATS_TX_AGG: c_int = 16;
pub const AGGR_STATS_RX_SIZE_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_aggr_stats {
    pub tx_agg_rate: [u32; AGGR_STATS_TX_AGG],
    pub tx_agg_len: [u32; AGGR_STATS_TX_AGG],
    pub rx_size: [u32; AGGR_STATS_RX_SIZE_LEN],
    pub __packed: },
pub const PIPE_STATS_HW_FIFO: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_pipeline_stats {
    pub hs_tx_stat_fifo_int: u32,
    pub hs_rx_stat_fifo_int: u32,
    pub enc_tx_stat_fifo_int: u32,
    pub enc_rx_stat_fifo_int: u32,
    pub rx_complete_stat_fifo_int: u32,
    pub pre_proc_swi: u32,
    pub post_proc_swi: u32,
    pub sec_frag_swi: u32,
    pub pre_to_defrag_swi: u32,
    pub defrag_to_rx_xfer_swi: u32,
    pub dec_packet_in: u32,
    pub dec_packet_in_fifo_full: u32,
    pub dec_packet_out: u32,
    pub pipeline_fifo_full: [u16; PIPE_STATS_HW_FIFO],
    pub padding: u16,
    pub __packed: },
pub const DIVERSITY_STATS_NUM_OF_ANT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_diversity_stats {
    pub num_of_packets_per_ant: [u32; DIVERSITY_STATS_NUM_OF_ANT],
    pub total_num_of_toggles: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_thermal_stats {
    pub irq_thr_low: u16,
    pub irq_thr_high: u16,
    pub tx_stop: u16,
    pub tx_resume: u16,
    pub false_irq: u16,
    pub adc_source_unexpected: u16,
    pub __packed: },
pub const WL18XX_NUM_OF_CALIBRATIONS_ERRORS: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_calib_failure_stats {
    pub fail_count: [u16; WL18XX_NUM_OF_CALIBRATIONS_ERRORS],
    pub calib_count: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_roaming_stats {
    pub rssi_level: i32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_dfs_stats {
    pub num_of_radar_detections: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_statistics {
    pub header: acx_header,
    pub error: wl18xx_acx_error_stats,
    pub tx: wl18xx_acx_tx_stats,
    pub rx: wl18xx_acx_rx_stats,
    pub isr: wl18xx_acx_isr_stats,
    pub pwr: wl18xx_acx_pwr_stats,
    pub rx_filter: wl18xx_acx_rx_filter_stats,
    pub rx_rate: wl18xx_acx_rx_rate_stats,
    pub aggr_size: wl18xx_acx_aggr_stats,
    pub pipeline: wl18xx_acx_pipeline_stats,
    pub diversity: wl18xx_acx_diversity_stats,
    pub thermal: wl18xx_acx_thermal_stats,
    pub calib: wl18xx_acx_calib_failure_stats,
    pub roaming: wl18xx_roaming_stats,
    pub dfs: wl18xx_dfs_stats,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_clear_statistics {
    pub header: acx_header,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_bandwidth {
    WLCORE_BANDWIDTH_20MHZ,
    WLCORE_BANDWIDTH_40MHZ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_peer_ht_operation_mode {
    pub header: acx_header,
    pub hlid: u8,
    pub /: *mut *mut u8 bandwidth; / enum wlcore_bandwidth,
    pub padding: [u8; 2],
}

//
// ACX_PEER_CAP
// this struct is very similar to wl1271_acx_ht_capabilities, with the
// addition of supported rates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_acx_peer_cap {
    pub header: acx_header,
// bitmask of capability bits supported by the peer
    pub ht_capabilites: __le32,
// rates supported by the remote peer
    pub supported_rates: __le32,
// Indicates to which link these capabilities apply.
    pub hlid: u8,
//
// This the maximum A-MPDU length supported by the AP. The FW may not
// exceed this length when sending A-MPDUs
//
    pub ampdu_max_length: u8,
// This is the minimal spacing required when sending A-MPDUs to the AP
    pub ampdu_min_spacing: u8,
    pub padding: u8,
    pub __packed: },
//
// ACX_INTERRUPT_NOTIFY
// enable/disable fast-link/PSM notification from FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_interrupt_notify {
    pub header: acx_header,
    pub enable: u32,
}

//
// ACX_RX_BA_FILTER
// enable/disable RX BA filtering in FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_acx_rx_ba_filter {
    pub header: acx_header,
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_ap_sleep_cfg {
    pub header: acx_header,
// Duty Cycle (20-80% of staying Awake) for IDLE AP
// (0: disable)
//
    pub idle_duty_cycle: u8,
// Duty Cycle (20-80% of staying Awake) for Connected AP
// (0: disable)
//
    pub connected_duty_cycle: u8,
// Maximum stations that are allowed to be connected to AP
// (255: no limit)
//
    pub max_stations_thresh: u8,
// Timeout till enabling the Sleep Mechanism after data stops
// [unit: 100 msec]
//
    pub idle_conn_thresh: u8,
    pub __packed: },
//
// ACX_DYNAMIC_TRACES_CFG
// configure the FW dynamic traces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_dynamic_fw_traces_cfg {
    pub header: acx_header,
    pub dynamic_fw_traces: __le32,
    pub __packed: },
//
// ACX_TIME_SYNC_CFG
// configure the time sync parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acx_time_sync_cfg {
    pub header: acx_header,
    pub sync_mode: u8,
    pub zone_mac_addr: [u8; ETH_ALEN],
    pub padding: [u8; 1],
    pub __packed: },
    pub len_field_size): u32,
    pub wl): *mut int wl18xx_acx_set_checksum_state(struct wl1271,
    pub wl): *mut int wl18xx_acx_clear_statistics(struct wl1271,
    pub wide): *mut *mut int wl18xx_acx_peer_ht_operation_mode(struct wl1271 wl, u8 hlid, bool,
    pub hlid): u32 rate_set, u8,
    pub action): *mut *mut int wl18xx_acx_interrupt_notify_config(struct wl1271 wl, bool,
    pub action): *mut *mut int wl18xx_acx_rx_ba_filter(struct wl1271 wl, bool,
    pub wl): *mut int wl18xx_acx_ap_sleep(struct wl1271,
    pub wl): *mut int wl18xx_acx_dynamic_fw_traces(struct wl1271,
    pub wl): *mut int wl18xx_acx_time_sync_cfg(struct wl1271,
