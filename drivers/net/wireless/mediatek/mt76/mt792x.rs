//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt792x.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2023 MediaTek Inc.

pub const MT792x_MAX_INTERFACES: c_int = 4;
pub const MT792x_WTBL_SIZE: c_int = 20;

pub const MT792x_CFEND_RATE_DEFAULT: c_uint = 0x49	/* OFDM 24M */;
pub const MT792x_CFEND_RATE_11B: c_uint = 0x03	/* 11B LP, 11M */;
pub const MT792x_FW_TAG_FEATURE: c_int = 4;

// NOTE: used to map mt76_rates. idx may change if firmware expands table
pub const MT792x_BASIC_RATES_TBL: c_int = 14;

pub const MT792x_DRV_OWN_RETRY_COUNT: c_int = 10;
pub const MT792x_MCU_INIT_RETRY_COUNT: c_int = 10;
pub const MT792x_WFSYS_INIT_RETRY_COUNT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_realease_info {
    pub len: __le16,
    pub pad_len: u8,
    pub tag: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_fw_features {
    pub segment: u8,
    pub data: u8,
    pub rsv: [u8; 14],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt792x_reg_power_type {
    MT_AP_UNSET = 0,
    MT_AP_DEFAULT,
    MT_AP_LPI,
    MT_AP_SP,
    MT_AP_VLP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt792x_mlo_pm_state {
    MT792x_MLO_LINK_DISASSOC,
    MT792x_MLO_LINK_ASSOC,
    MT792x_MLO_CHANGED_PS_PENDING,
    MT792x_MLO_CHANGED_PS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_link_sta {
    pub /: *mut *mut mt76_wcid wcid; / must be first,
    pub rcu_head: rcu_head,
    pub airtime_ac: [u32; 8],
    pub ack_signal: c_int,
    pub avg_ack_signal: ewma_avg_signal,
    pub last_txs: c_ulong,
    pub bip: mt76_connac_sta_key_conf,
    pub sta: *mut mt792x_sta,
    pub pri_link: *mut ieee80211_link_sta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_sta_nan_sched {
// protects NAN peer schedule state
    pub committed_dw: u16,
    pub sch_idx: u32,
    pub idx_assigned: bool,
    pub ndp_ctx_bitmap: c_ulong,
    pub ndp_ctx_assigned: bool,
    pub /: *mut *mut u8 ndp_ctx_id; / assigned NDP context ID (for NDI sta),
    pub map_id: u8,
    pub chans: [cfg80211_chan_def; CFG80211_NAN_SCHED_NUM_TIME_SLOTS],
    pub maps: [}; CFG80211_NAN_MAX_PEER_MAPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_sta {
    pub /: *mut *mut mt792x_link_sta deflink; / must be first,
    pub link: [*mut mt792x_link_sta __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
    pub vif: *mut mt792x_vif,
    pub valid_links: u16,
    pub deflink_id: u8,
// NAN peer schedule
    pub nan_sched: mt792x_sta_nan_sched,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_chanctx {
    pub bss_conf: *mut mt792x_bss_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_bss_conf {
    pub /: *mut *mut mt76_vif_link mt76; / must be first,
    pub vif: *mut mt792x_vif,
    pub rssi: ewma_rssi,
    pub queue_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub link_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_nan_conf {
    pub master_pref: u8,
    pub bands: u8,
    pub cluster_id: [u8; ETH_ALEN],
    pub discovery_beacon_interval: u32,
    pub enable_dw_notification: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_nan {
    pub conf: mt792x_nan_conf,
// Scheduler
    pub local_sched: [cfg80211_chan_def; CFG80211_NAN_SCHED_NUM_TIME_SLOTS],
    pub seq_id: u32,
// Connection index bitmap, up to NAN_MAX_CONN_CFG peers
    pub conn_bitmap: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_vif {
    pub /: *mut *mut mt792x_bss_conf bss_conf; / must be first,
    pub link_conf: [*mut mt792x_bss_conf __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
    pub sta: mt792x_sta,
    pub wep_sta: *mut mt792x_sta,
    pub phy: *mut mt792x_phy,
    pub valid_links: u16,
    pub deflink_id: u8,
    pub mlo_pm_state: mt792x_mlo_pm_state,
    pub csa_work: work_struct,
    pub csa_timer: timer_list,
    pub nan: mt792x_nan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_phy {
    pub mt76: *mut mt76_phy,
    pub dev: *mut mt792x_dev,
    pub iftype: [ieee80211_sband_iftype_data; NUM_NL80211_BANDS][NUM_NL80211_IFTYPES],
    pub omac_mask: u64,
    pub noise: u16,
    pub coverage_class: i16,
    pub slottime: u8,
    pub rx_ampdu_ts: u32,
    pub ampdu_ref: u32,
    pub mib: mt76_mib_stats,
    pub sta_work_count: u8,
    pub clc_chan_conf: u8,
    pub power_type: mt792x_reg_power_type,
    pub scan_event_list: sk_buff_head,
    pub scan_work: delayed_work,

    pub acpisar: *mut c_void,
    pub clc: [*mut c_void; MT792x_CLC_MAX_NUM],
    pub chip_cap: u64,
    pub eml_cap: u16,
    pub roc_work: work_struct,
    pub roc_timer: timer_list,
    pub roc_wait: wait_queue_head_t,
    pub roc_token_id: u8,
    pub roc_grant: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_irq_map {
    pub host_irq_enable: u32,
    pub all_complete_mask: u32,
    pub mcu_complete_mask: u32,
    pub tx: },
    pub all_complete_mask: u32,
    pub data_complete_mask: u32,
    pub wm_complete_mask: u32,
    pub wm2_complete_mask: u32,
    pub rx: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_dma_ring {
    pub qid: u8,
    pub n_desc: u16,
    pub ring_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_dma_layout {
    pub tx_data0: mt792x_dma_ring,
    pub tx_mcu: mt792x_dma_ring,
    pub tx_fwdl: mt792x_dma_ring,
    pub tx_done: mt792x_dma_ring,
    pub rx_data: mt792x_dma_ring,
    pub rx_mcu: mt792x_dma_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_hif_ops {
    pub dev): *mut *mut int (init_reset)(struct mt792x_dev,
    pub dev): *mut *mut int (reset)(struct mt792x_dev,
    pub dev): *mut *mut int (mcu_init)(struct mt792x_dev,
    pub dev): *mut *mut int (drv_own)(struct mt792x_dev,
    pub dev): *mut *mut int (fw_own)(struct mt792x_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_pcie_reg {
    pub imask: u32,
    pub pm: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_dev {
    pub mt76: mt76_dev,
    pub mphy: mt76_phy,
}

// IPv6 addresses for WoWLAN
extern "C" {
    pub fn mt792x_vif_to_link(_arg: mvif, _arg: link_conf->link_id) -> return;
}
extern "C" {
    pub fn link_conf_dereference_protected(_arg: vif, _arg: link_id) -> return;
}
extern "C" {
    pub fn link_sta_dereference_protected(_arg: sta, _arg: link_id) -> return;
}
extern "C" {
    pub fn container_of(_arg: phy->dev, mt792x_dev: struct, _arg: mt76) -> return;
}

extern "C" {
    pub fn mt792x_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn mt792x_pm_wake_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt792x_pm_power_save_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt792x_reset(mdev: *mut mt76_dev);
}
extern "C" {
    pub fn mt792x_update_channel(mphy: *mut mt76_phy);
}
extern "C" {
    pub fn mt792x_mac_reset_counters(phy: *mut mt792x_phy);
}
extern "C" {
    pub fn mt792x_mac_init_band(dev: *mut mt792x_dev, band: u8);
}
extern "C" {
    pub fn mt792x_mac_assoc_rssi(dev: *mut mt792x_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt792x_mac_update_mib_stats(phy: *mut mt792x_phy);
}
extern "C" {
    pub fn mt792x_mac_set_timeing(phy: *mut mt792x_phy);
}
extern "C" {
    pub fn mt792x_mac_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt792x_get_tsf(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> u64;
}
extern "C" {
    pub fn mt792x_tx_worker(w: *mut mt76_worker);
}
extern "C" {
    pub fn mt792x_roc_timer(timer: *mut timer_list);
}
extern "C" {
    pub fn mt792x_csa_timer(timer: *mut timer_list);
}
extern "C" {
    pub fn mt792x_set_wakeup(hw: *mut ieee80211_hw, enabled: bool);
}
extern "C" {
    pub fn mt792x_dma_cleanup(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt792x_dma_enable(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_wpdma_reset(dev: *mut mt792x_dev, force: bool) -> c_int;
}
extern "C" {
    pub fn mt792x_wpdma_reinit_cond(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_dma_disable(dev: *mut mt792x_dev, force: bool) -> c_int;
}
extern "C" {
    pub fn mt792x_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mt792x_rx_poll_complete(mdev: *mut mt76_dev, q: mt76_rxq_id);
}
extern "C" {
    pub fn mt792x_poll_tx(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mt792x_poll_rx(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mt792x_irq_tasklet(data: c_ulong);
}
extern "C" {
    pub fn mt792x_wfsys_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_tx_stats_show(file: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mt792x_queues_acq(s: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mt792x_queues_read(s: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mt792x_pm_stats(s: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mt792x_pm_idle_timeout_set(data: *mut c_void, val: u64) -> c_int;
}
extern "C" {
    pub fn mt792x_pm_idle_timeout_get(data: *mut c_void, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn mt792x_init_wiphy(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn mt792x_init_wcid(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_mcu_drv_pmctrl(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_mcu_fw_pmctrl(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_config_mac_addr_list(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt792x_load_firmware(dev: *mut mt792x_dev) -> c_int;
}
// usb

extern "C" {
    pub fn mt792xu_dma_init(dev: *mut mt792x_dev, resume: bool) -> c_int;
}
extern "C" {
    pub fn mt792xu_mcu_power_on(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xu_wfsys_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xu_init_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xu_reset_work_init(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt792xu_reset_work_cleanup(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt792xu_check_bus(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xu_reset_on_bus_error(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xu_rr(dev: *mut mt76_dev, addr: u32) -> u32;
}
extern "C" {
    pub fn mt792xu_wr(dev: *mut mt76_dev, addr: u32, val: u32);
}
extern "C" {
    pub fn mt792xu_rmw(dev: *mut mt76_dev, addr: u32, mask: u32, val: u32) -> u32;
}
extern "C" {
    pub fn mt792xu_copy(dev: *mut mt76_dev, offset: u32, data: *const c_void, len: c_int);
}
extern "C" {
    pub fn mt792xu_disconnect(usb_intf: *mut usb_interface);
}
extern "C" {
    pub fn mt792xu_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn __mt792xe_mcu_drv_pmctrl(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xe_mcu_drv_pmctrl(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792xe_mcu_fw_pmctrl(dev: *mut mt792x_dev) -> c_int;
}

extern "C" {
    pub fn mt792x_init_acpi_sar(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt792x_init_acpi_sar_power(phy: *mut mt792x_phy, set_default: bool) -> c_int;
}
extern "C" {
    pub fn mt792x_acpi_get_flags(phy: *mut mt792x_phy) -> u8;
}
extern "C" {
    pub fn mt792x_acpi_get_mtcl_conf(phy: *mut mt792x_phy, alpha2: *mut c_char) -> u32;
}

