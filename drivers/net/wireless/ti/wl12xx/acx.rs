//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl12xx/acx.h
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
// This file is part of wl12xx
//
// Copyright (C) 1998-2009, 2011 Texas Instruments. All rights reserved.
// Copyright (C) 2008-2010 Nokia Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_acx_host_config_bitmap {
    pub header: acx_header,
    pub host_cfg_bitmap: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_tx_statistics {
    pub internal_desc_overflow: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_rx_statistics {
    pub out_of_mem: __le32,
    pub hdr_overflow: __le32,
    pub hw_stuck: __le32,
    pub dropped: __le32,
    pub fcs_err: __le32,
    pub xfr_hint_trig: __le32,
    pub path_reset: __le32,
    pub reset_counter: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_dma_statistics {
    pub rx_requested: __le32,
    pub rx_errors: __le32,
    pub tx_requested: __le32,
    pub tx_errors: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_isr_statistics {
// host command complete
    pub cmd_cmplt: __le32,
// fiqisr()
    pub fiqs: __le32,
// (INT_STS_ND & INT_TRIG_RX_HEADER)
    pub rx_headers: __le32,
// (INT_STS_ND & INT_TRIG_RX_CMPLT)
    pub rx_completes: __le32,
// (INT_STS_ND & INT_TRIG_NO_RX_BUF)
    pub rx_mem_overflow: __le32,
// (INT_STS_ND & INT_TRIG_S_RX_RDY)
    pub rx_rdys: __le32,
// irqisr()
    pub irqs: __le32,
// (INT_STS_ND & INT_TRIG_TX_PROC)
    pub tx_procs: __le32,
// (INT_STS_ND & INT_TRIG_DECRYPT_DONE)
    pub decrypt_done: __le32,
// (INT_STS_ND & INT_TRIG_DMA0)
    pub dma0_done: __le32,
// (INT_STS_ND & INT_TRIG_DMA1)
    pub dma1_done: __le32,
// (INT_STS_ND & INT_TRIG_TX_EXC_CMPLT)
    pub tx_exch_complete: __le32,
// (INT_STS_ND & INT_TRIG_COMMAND)
    pub commands: __le32,
// (INT_STS_ND & INT_TRIG_RX_PROC)
    pub rx_procs: __le32,
// (INT_STS_ND & INT_TRIG_PM_802)
    pub hw_pm_mode_changes: __le32,
// (INT_STS_ND & INT_TRIG_ACKNOWLEDGE)
    pub host_acknowledges: __le32,
// (INT_STS_ND & INT_TRIG_PM_PCI)
    pub pci_pm: __le32,
// (INT_STS_ND & INT_TRIG_ACM_WAKEUP)
    pub wakeups: __le32,
// (INT_STS_ND & INT_TRIG_LOW_RSSI)
    pub low_rssi: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_wep_statistics {
// WEP address keys configured
    pub addr_key_count: __le32,
// default keys configured
    pub default_key_count: __le32,
    pub reserved: __le32,
// number of times that WEP key not found on lookup
    pub key_not_found: __le32,
// number of times that WEP key decryption failed
    pub decrypt_fail: __le32,
// WEP packets decrypted
    pub packets: __le32,
// WEP decrypt interrupts
    pub interrupt: __le32,
    pub __packed: },
pub const ACX_MISSED_BEACONS_SPREAD: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_pwr_statistics {
// the amount of enters into power save mode (both PD & ELP)
    pub ps_enter: __le32,
// the amount of enters into ELP mode
    pub elp_enter: __le32,
// the amount of missing beacon interrupts to the host
    pub missing_bcns: __le32,
// the amount of wake on host-access times
    pub wake_on_host: __le32,
// the amount of wake on timer-expire
    pub wake_on_timer_exp: __le32,
// the number of packets that were transmitted with PS bit set
    pub tx_with_ps: __le32,
// the number of packets that were transmitted with PS bit clear
    pub tx_without_ps: __le32,
// the number of received beacons
    pub rcvd_beacons: __le32,
// the number of entering into PowerOn (power save off)
    pub power_save_off: __le32,
// the number of entries into power save mode
    pub enable_ps: __le16,
//
// the number of exits from power save, not including failed PS
// transitions
//
    pub disable_ps: __le16,
//
// the number of times the TSF counter was adjusted because
// of drift
//
    pub fix_tsf_ps: __le32,
// Gives statistics about the spread continuous missed beacons.
// The 16 LSB are dedicated for the PS mode.
// The 16 MSB are dedicated for the PS mode.
// cont_miss_bcns_spread[0] - single missed beacon.
// cont_miss_bcns_spread[1] - two continuous missed beacons.
// cont_miss_bcns_spread[2] - three continuous missed beacons.
// ...
// cont_miss_bcns_spread[9] - ten and more continuous missed beacons.
//
    pub cont_miss_bcns_spread: [__le32; ACX_MISSED_BEACONS_SPREAD],
// the number of beacons in awake mode
    pub rcvd_awake_beacons: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_mic_statistics {
    pub rx_pkts: __le32,
    pub calc_failure: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_aes_statistics {
    pub encrypt_fail: __le32,
    pub decrypt_fail: __le32,
    pub encrypt_packets: __le32,
    pub decrypt_packets: __le32,
    pub encrypt_interrupt: __le32,
    pub decrypt_interrupt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_event_statistics {
    pub heart_beat: __le32,
    pub calibration: __le32,
    pub rx_mismatch: __le32,
    pub rx_mem_empty: __le32,
    pub rx_pool: __le32,
    pub oom_late: __le32,
    pub phy_transmit_error: __le32,
    pub tx_stuck: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_ps_statistics {
    pub pspoll_timeouts: __le32,
    pub upsd_timeouts: __le32,
    pub upsd_max_sptime: __le32,
    pub upsd_max_apturn: __le32,
    pub pspoll_max_apturn: __le32,
    pub pspoll_utilization: __le32,
    pub upsd_utilization: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_rxpipe_statistics {
    pub rx_prep_beacon_drop: __le32,
    pub descr_host_int_trig_rx_data: __le32,
    pub beacon_buffer_thres_host_int_trig_rx_data: __le32,
    pub missed_beacon_host_int_trig_rx_data: __le32,
    pub tx_xfr_host_int_trig_rx_data: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_acx_statistics {
    pub header: acx_header,
    pub tx: wl12xx_acx_tx_statistics,
    pub rx: wl12xx_acx_rx_statistics,
    pub dma: wl12xx_acx_dma_statistics,
    pub isr: wl12xx_acx_isr_statistics,
    pub wep: wl12xx_acx_wep_statistics,
    pub pwr: wl12xx_acx_pwr_statistics,
    pub aes: wl12xx_acx_aes_statistics,
    pub mic: wl12xx_acx_mic_statistics,
    pub event: wl12xx_acx_event_statistics,
    pub ps: wl12xx_acx_ps_statistics,
    pub rxpipe: wl12xx_acx_rxpipe_statistics,
    pub __packed: },
    pub host_cfg_bitmap): *mut *mut int wl1271_acx_host_if_cfg_bitmap(struct wl1271 wl, u32,
