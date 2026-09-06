//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/mld.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024-2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_h__

//
// DOC: Introduction
//
// iwlmld is an operation mode (a.k.a. op_mode) for Intel wireless devices.
// It is used for devices that ship after 2024 which typically support
// the WiFi-7 features. MLD stands for multi-link device. Note that there are
// devices that do not support WiFi-7 or even WiFi 6E and yet use iwlmld, but
// the firmware APIs used in this driver are WiFi-7 compatible.
//
// In the architecture of iwlwifi, an op_mode is a layer that translates
// mac80211's APIs into commands for the firmware and, of course, notifications
// from the firmware to mac80211's APIs. An op_mode must implement the
// interface defined in iwl-op-mode.h to interact with the transport layer
// which allows to send and receive data to the device, start the hardware,
// etc...
//
// DOC: Locking policy
//
// iwlmld has a very simple locking policy: it doesn't have any mutexes. It
// relies on cfg80211's wiphy->mtx and takes the lock when needed. All the
// control flows originating from mac80211 already acquired the lock, so that
// part is trivial, but also notifications that are received from the firmware
// and handled asynchronously are handled only after having taken the lock.
// This is described in notif.c.
// There are spin_locks needed to synchronize with the data path, around the
// allocation of the queues, for example.
//
// DOC: Debugfs
//
// iwlmld adds its share of debugfs hooks and its handlers are synchronized
// with the wiphy_lock using wiphy_locked_debugfs. This avoids races against
// resources deletion while the debugfs hook is being used.
//
// DOC: Main resources
//
// iwlmld is designed with the life cycle of the resource in mind. The
// resources are:
//
// - struct iwl_mld (matches mac80211's struct ieee80211_hw)
//
// - struct iwl_mld_vif (matches macu80211's struct ieee80211_vif)
// iwl_mld_vif contains an array of pointers to struct iwl_mld_link
// which describe the links for this vif.
//
// - struct iwl_mld_sta (matches mac80211's struct ieee80211_sta)
// iwl_mld_sta contains an array of points to struct iwl_mld_link_sta
// which describes the link stations for this station
//
// Each object has properties that can survive a firmware reset or not.
// Asynchronous firmware notifications can declare themselves as dependent on a
// certain instance of those resources and that means that the notifications
// will be cancelled once the instance is destroyed.
//
pub const IWL_MLD_MAX_ADDRESSES: c_int = 5;
//
// struct iwl_mld - MLD op mode
//
// @fw_id_to_bss_conf: maps a fw id of a link to the corresponding
// ieee80211_bss_conf.
// @fw_id_to_vif: maps a fw id of a MAC context to the corresponding
// ieee80211_vif. Mapping is valid only when the MAC exists in the fw.
// @fw_id_to_txq: maps a fw id of a txq to the corresponding
// ieee80211_txq.
// @used_phy_ids: a bitmap of the phy IDs used. If a bit is set, it means
// that the index of this bit is already used as a PHY id.
// @num_igtks: the number if iGTKs that were sent to the FW.
// @monitor: monitor related data
// @monitor.on: does a monitor vif exist (singleton hence bool)
// @monitor.ampdu_ref: the id of the A-MPDU for sniffer
// @monitor.ampdu_toggle: the state of the previous packet to track A-MPDU
// @monitor.cur_aid: current association id tracked by the sniffer
// @monitor.cur_bssid: current bssid tracked by the sniffer
// @monitor.ptp_time: set the Rx mactime using the device's PTP clock time
// @monitor.p80: primary channel position relative to he whole bandwidth, in
// steps of 80 MHz
// @monitor.phy: PHY data information
// @monitor.phy.data: PHY data (&struct iwl_rx_phy_air_sniffer_ntfy) received
// @monitor.phy.valid: PHY data is valid (was received)
// @monitor.phy.used: PHY data was used by an RX
// @fw_id_to_link_sta: maps a fw id of a sta to the corresponding
// ieee80211_link_sta. This is not cleaned up on restart since we want to
// preserve the fw sta ids during a restart (for SN/PN restoring).
// FW ids of internal stations will be mapped to ERR_PTR, and will be
// re-allocated during a restart, so make sure to free it in restart
// cleanup using iwl_mld_free_internal_sta
// @netdetect: indicates the FW is in suspend mode with netdetect configured
// @p2p_device_vif: points to the p2p device vif if exists
// @bt_is_active: indicates that BT is active
// @dev: pointer to device struct. For printing purposes
// @trans: pointer to the transport layer
// @cfg: pointer to the device configuration
// @fw: a pointer to the fw object
// @hw: pointer to the hw object.
// @wiphy: a pointer to the wiphy struct, for easier access to it.
// @ext_capab: extended capabilities that will be set to wiphy on registration.
// @sta_ext_capab: extended capabilities for the station interface.
// @nvm_data: pointer to the nvm_data that includes all our capabilities
// @fwrt: fw runtime data
// @debugfs_dir: debugfs directory
// @notif_wait: notification wait related data.
// @async_handlers_list: a list of all async RX handlers. When a notifciation
// with an async handler is received, it is added to this list.
// When &async_handlers_wk runs - it runs these handlers one by one.
// @async_handlers_lock: a lock for &async_handlers_list. Sync
// &async_handlers_wk and RX notifcation path.
// @async_handlers_wk: A work to run all async RX handlers from
// &async_handlers_list.
// @ct_kill_exit_wk: worker to exit thermal kill
// @fw_status: bitmap of fw status bits
// @running: true if the firmware is running
// @do_not_dump_once: true if firmware dump must be prevented once
// @in_d3: indicates FW is in suspend mode and should be resumed
// @resuming: indicates the driver is resuming from wowlan
// @in_hw_restart: indicates that we are currently in restart flow.
// rather than restarted. Should be unset upon restart.
// @radio_kill: bitmap of radio kill status
// @radio_kill.hw: radio is killed by hw switch
// @radio_kill.ct: radio is killed because the device it too hot
// @power_budget_mw: maximum cTDP power budget as defined for this system and
// device
// @addresses: device MAC addresses.
// @scan: instance of the scan object
// @channel_survey: channel survey information collected during scan
// @wowlan: WoWLAN support data.
// @debug_max_sleep: maximum sleep time in D3 (for debug purposes)
// @led: the led device
// @mcc_src: the source id of the MCC, comes from the firmware
// @fw_id_to_ba: maps a fw (BA) id to a corresponding Block Ack session data.
// @num_rx_ba_sessions: tracks the number of active Rx Block Ack (BA) sessions.
// the driver ensures that new BA sessions are blocked once the maximum
// supported by the firmware is reached, preventing firmware asserts.
// @rxq_sync: manages RX queue sync state
// @txqs_to_add: a list of &ieee80211_txq's to allocate in &add_txqs_wk
// @add_txqs_wk: a worker to allocate txqs.
// @add_txqs_lock: to lock the &txqs_to_add list.
// @error_recovery_buf: pointer to the recovery buffer that will be read
// from firmware upon fw/hw error and sent back to the firmware in
// reconfig flow (after NIC reset).
// @mcast_filter_cmd: pointer to the multicast filter command.
// @mgmt_tx_ant: stores the last TX antenna index; used for setting
// TX rate_n_flags for non-STA mgmt frames (toggles on every TX failure).
// @set_tx_ant: stores the last TX antenna bitmask set by user space (if any)
// @set_rx_ant: stores the last RX antenna bitmask set by user space (if any)
// @low_latency: low-latency manager.
// @tzone: thermal zone device's data
// @cooling_dev: cooling device's related data
// @ibss_manager: in IBSS mode (only one vif can be active), indicates what
// firmware indicated about having transmitted the last beacon, i.e.
// being IBSS manager for that time and needing to respond to probe
// requests
// @ptp_data: data of the PTP clock
// @time_sync: time sync data.
// @ftm_initiator: FTM initiator data
// @nan_device_vif: points to the NAN device vif if exists
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld {
// Add here fields that need clean up on restart
    pub fw_id_to_bss_conf: [*mut ieee80211_bss_conf __rcu; IWL_FW_MAX_LINKS],
    pub fw_id_to_vif: [*mut ieee80211_vif __rcu; NUM_MAC_INDEX_DRIVER],
    pub fw_id_to_txq: [*mut ieee80211_txq __rcu; IWL_MAX_TVQM_QUEUES],
    pub NUM_PHY_CTX: u8 used_phy_ids:,
    pub num_igtks: u8,
    pub on: bool,
    pub ampdu_ref: u32,
    pub ampdu_toggle: bool,
    pub p80: u8,
    pub data: iwl_rx_phy_air_sniffer_ntfy,
    pub used:1: u8 valid:1,,
    pub phy: },

    pub cur_aid: __le16,
    pub cur_bssid: [u8; ETH_ALEN],
    pub ptp_time: bool,

    pub monitor: },

    pub netdetect: bool,

    pub p2p_device_vif: *mut ieee80211_vif,
    pub bt_is_active: bool,
    pub nan_device_vif: *mut ieee80211_vif,
    pub fw_id_to_link_sta: [*mut ieee80211_link_sta __rcu; IWL_STATION_COUNT_MAX],
// And here fields that survive a fw restart
    pub dev: *mut device,
    pub trans: *mut iwl_trans,
    pub cfg: *const iwl_rf_cfg,
    pub fw: *const iwl_fw,
    pub hw: *mut ieee80211_hw,
    pub wiphy: *mut wiphy,
    pub ext_capab: [wiphy_iftype_ext_capab; IWL_MLD_EXT_CAPA_NUM_IFTYPES],
    pub sta_ext_capab: [u8; IWL_MLD_STA_EXT_CAPA_SIZE],
    pub nvm_data: *mut iwl_nvm_data,
    pub fwrt: iwl_fw_runtime,
    pub debugfs_dir: *mut dentry,
    pub notif_wait: iwl_notif_wait_data,
    pub async_handlers_list: list_head,
    pub async_handlers_lock: spinlock_t,
    pub async_handlers_wk: wiphy_work,
    pub ct_kill_exit_wk: wiphy_delayed_work,

    pub fw_status: },
    pub radio_kill: },
    pub power_budget_mw: u32,
    pub addresses: [mac_address; IWL_MLD_MAX_ADDRESSES],
    pub scan: iwl_mld_scan,
    pub channel_survey: *mut iwl_mld_survey,

    pub wowlan: wiphy_wowlan_support,
    pub debug_max_sleep: u32,

    pub led: led_classdev,

    pub mcc_src: iwl_mcc_source,
    pub fw_id_to_ba: [*mut iwl_mld_baid_data __rcu; IWL_MAX_BAID],
    pub num_rx_ba_sessions: u8,
    pub rxq_sync: iwl_mld_rx_queues_sync,
    pub txqs_to_add: list_head,
    pub add_txqs_wk: wiphy_work,
    pub add_txqs_lock: spinlock_t,
    pub error_recovery_buf: *mut u8,
    pub mcast_filter_cmd: *mut iwl_mcast_filter_cmd,
    pub mgmt_tx_ant: u8,
    pub set_tx_ant: u8,
    pub set_rx_ant: u8,
    pub low_latency: iwl_mld_low_latency,
    pub ibss_manager: bool,

    pub tzone: *mut thermal_zone_device,
    pub cooling_dev: iwl_mld_cooling_device,

    pub ptp_data: ptp_data,
    pub time_sync: *mut iwl_mld_time_sync_data __rcu,
    pub ftm_initiator: ftm_initiator_data,
}

// memset the part of the struct that requires cleanup on restart

// Cleanup function for struct iwl_mld, will be called in restart

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_power_scheme {
    IWL_POWER_SCHEME_CAM = 1,
    IWL_POWER_SCHEME_BPS,
}

//
// struct iwl_mld_mod_params - module parameters for iwlmld
// @power_scheme: one of enum iwl_power_scheme
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_mod_params {
    pub power_scheme: c_int,
}

// Extract MLD priv from op_mode

extern "C" {
    pub fn iwl_mld_load_fw(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_stop_fw(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_start_fw(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_send_recovery_cmd(mld: *mut iwl_mld, flags: u32);
}
// CCK is not allowed in 5 GHz
//
// enum iwl_rx_handler_context: context for Rx handler
// @RX_HANDLER_SYNC: this means that it will be called in the Rx path
// which can't acquire the wiphy->mutex.
// @RX_HANDLER_ASYNC: If the handler needs to hold wiphy->mutex
// (and only in this case!), it should be set as ASYNC. In that case,
// it will be called from a worker with wiphy->mutex held.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_rx_handler_context {
    RX_HANDLER_SYNC,
    RX_HANDLER_ASYNC,
}

//
// struct iwl_rx_handler: handler for FW notification
// @val_fn: input validation function.
// @sizes: an array that mapps a version to the expected size.
// @fn: the function is called when notification is handled
// @cmd_id: command id
// @n_sizes: number of elements in &sizes.
// @context: see &iwl_rx_handler_context
// @obj_type: the type of the object that this handler is related to.
// See &iwl_mld_object_type. Use IWL_MLD_OBJECT_TYPE_NONE if not related.
// @cancel: function to cancel the notification. valid only if obj_type is not
// IWL_MLD_OBJECT_TYPE_NONE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_handler {
    pub pkt): *mut *mut *mut bool (val_fn)(struct iwl_mld mld, struct iwl_rx_packet,
    pub sizes: *const iwl_notif_struct_size,
}

//
// struct iwl_notif_struct_size: map a notif ver to the expected size
//
// @size: the size to expect
// @ver: the version of the notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notif_struct_size {
    pub ver:8: u32 size:24,,
}

pub const IWL_MLD_INVALID_FW_ID: c_uint = 0xff;

// fw_id = idx;								\

// Utilities
// Check if we had an error, but reconfig flow didn't start yet
extern "C" {
    pub fn iwl_mld_tdls_sta_count(mld: *mut iwl_mld) -> c_int;
}
