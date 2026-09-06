//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/iface.h
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

// Macro flag: #define __iwl_mld_iface_h__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_cca_40mhz_wa_status {
    CCA_40_MHZ_WA_NONE,
    CCA_40_MHZ_WA_RESET,
    CCA_40_MHZ_WA_RECONNECT,
}

//
// enum iwl_mld_emlsr_blocked - defines reasons for which EMLSR is blocked
//
// These blocks are applied/stored per-VIF.
//
// @IWL_MLD_EMLSR_BLOCKED_PREVENTION: Prevent repeated EMLSR enter/exit
// @IWL_MLD_EMLSR_BLOCKED_WOWLAN: WOWLAN is preventing EMLSR
// @IWL_MLD_EMLSR_BLOCKED_ROC: remain-on-channel is preventing EMLSR
// @IWL_MLD_EMLSR_BLOCKED_NON_BSS: An active non-BSS interface's link is
// preventing EMLSR
// @IWL_MLD_EMLSR_BLOCKED_TMP_NON_BSS: An expected active non-BSS interface's
// link is preventing EMLSR. This is a temporary blocking that is set when
// there is an indication that a non-BSS interface is to be added.
// @IWL_MLD_EMLSR_BLOCKED_TPT: throughput is too low to make EMLSR worthwhile
// @IWL_MLD_EMLSR_BLOCKED_NAN: NAN is preventing EMLSR.
// @IWL_MLD_EMLSR_BLOCKED_TDLS: TDLS connection is preventing EMLSR.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_emlsr_blocked {
    IWL_MLD_EMLSR_BLOCKED_PREVENTION	= 0x1,
    IWL_MLD_EMLSR_BLOCKED_WOWLAN		= 0x2,
    IWL_MLD_EMLSR_BLOCKED_ROC		= 0x4,
    IWL_MLD_EMLSR_BLOCKED_NON_BSS		= 0x8,
    IWL_MLD_EMLSR_BLOCKED_TMP_NON_BSS	= 0x10,
    IWL_MLD_EMLSR_BLOCKED_TPT		= 0x20,
    IWL_MLD_EMLSR_BLOCKED_NAN		= 0x40,
    IWL_MLD_EMLSR_BLOCKED_TDLS		= 0x80,
}

//
// enum iwl_mld_emlsr_exit - defines reasons for exiting EMLSR
//
// Reasons to exit EMLSR may be either link specific or even specific to a
// combination of links.
//
// @IWL_MLD_EMLSR_EXIT_BLOCK: Exit due to a block reason being set
// @IWL_MLD_EMLSR_EXIT_MISSED_BEACON: Exit due to missed beacons
// @IWL_MLD_EMLSR_EXIT_FAIL_ENTRY: FW failed to enter EMLSR
// @IWL_MLD_EMLSR_EXIT_CSA: EMLSR prevented due to channel switch on link
// @IWL_MLD_EMLSR_EXIT_EQUAL_BAND: EMLSR prevented as both links share the band
// @IWL_MLD_EMLSR_EXIT_LOW_RSSI: Link RSSI is unsuitable for EMLSR
// @IWL_MLD_EMLSR_EXIT_LINK_USAGE: Exit EMLSR due to low TPT on secondary link
// @IWL_MLD_EMLSR_EXIT_BT_COEX: Exit EMLSR due to BT coexistence
// @IWL_MLD_EMLSR_EXIT_CHAN_LOAD: Exit EMLSR because the primary channel is not
// loaded enough to justify EMLSR.
// @IWL_MLD_EMLSR_EXIT_RFI: Exit EMLSR due to RFI
// @IWL_MLD_EMLSR_EXIT_FW_REQUEST: Exit EMLSR because the FW requested it
// @IWL_MLD_EMLSR_EXIT_INVALID: internal exit reason due to invalid data
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_emlsr_exit {
    IWL_MLD_EMLSR_EXIT_BLOCK		= 0x1,
    IWL_MLD_EMLSR_EXIT_MISSED_BEACON	= 0x2,
    IWL_MLD_EMLSR_EXIT_FAIL_ENTRY		= 0x4,
    IWL_MLD_EMLSR_EXIT_CSA			= 0x8,
    IWL_MLD_EMLSR_EXIT_EQUAL_BAND		= 0x10,
    IWL_MLD_EMLSR_EXIT_LOW_RSSI		= 0x20,
    IWL_MLD_EMLSR_EXIT_LINK_USAGE		= 0x40,
    IWL_MLD_EMLSR_EXIT_BT_COEX		= 0x80,
    IWL_MLD_EMLSR_EXIT_CHAN_LOAD		= 0x100,
    IWL_MLD_EMLSR_EXIT_RFI			= 0x200,
    IWL_MLD_EMLSR_EXIT_FW_REQUEST		= 0x400,
    IWL_MLD_EMLSR_EXIT_INVALID		= 0x800,
}

//
// struct iwl_mld_emlsr - per-VIF data about EMLSR operation
//
// @primary: The current primary link
// @selected_primary: Primary link as selected during the last link selection
// @selected_links: Links as selected during the last link selection
// @blocked_reasons: Reasons preventing EMLSR from being enabled
// @last_exit_reason: Reason for the last EMLSR exit
// @last_exit_ts: Time of the last EMLSR exit (if @last_exit_reason is non-zero)
// @exit_repeat_count: Number of times EMLSR was exited for the same reason
// @last_entry_ts: the time of the last EMLSR entry (if iwl_mld_emlsr_active()
// is true)
// @unblock_tpt_wk: Unblock EMLSR because the throughput limit was reached
// @check_tpt_wk: a worker to check if IWL_MLD_EMLSR_BLOCKED_TPT should be
// added, for example if there is no longer enough traffic.
// @prevent_done_wk: Worker to remove %IWL_MLD_EMLSR_BLOCKED_PREVENTION
// @tmp_non_bss_done_wk: Worker to remove %IWL_MLD_EMLSR_BLOCKED_TMP_NON_BSS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_emlsr {
    pub primary: u8,
    pub selected_primary: u8,
    pub selected_links: u16,
    pub blocked_reasons: iwl_mld_emlsr_blocked,
    pub last_exit_reason: iwl_mld_emlsr_exit,
    pub last_exit_ts: c_ulong,
    pub exit_repeat_count: u8,
    pub last_entry_ts: c_ulong,
    pub unblock_tpt_wk: wiphy_work,
    pub check_tpt_wk: wiphy_delayed_work,
    pub prevent_done_wk: wiphy_delayed_work,
    pub tmp_non_bss_done_wk: wiphy_delayed_work,
}

//
// struct iwl_mld_vif - virtual interface (MAC context) configuration parameters
//
// @fw_id: fw id of the mac context.
// @session_protect: session protection parameters
// @ap_sta: pointer to AP sta, for easier access to it.
// Relevant only for STA vifs.
// @authorized: indicates the AP station was set to authorized
// @num_associated_stas: number of associated STAs. Relevant only for AP mode.
// @ap_ibss_active: whether the AP/IBSS was started
// @cca_40mhz_workaround: When we are connected in 2.4 GHz and 40 MHz, and the
// environment is too loaded, we work around this by reconnecting to the
// same AP with 20 MHz. This manages the status of the workaround.
// @beacon_inject_active: indicates an active debugfs beacon ie injection
// @low_latency_causes: bit flags, indicating the causes for low-latency,
// see @iwl_mld_low_latency_cause.
// @last_link_activation_time: last time a link was activated, for
// deferring MLO scans (to make them more reliable)
// @mld: pointer to the mld structure.
// @deflink: default link data, for use in non-MLO,
// @link: reference to link data for each valid link, for use in MLO.
// @emlsr: information related to EMLSR
// @wowlan_data: data used by the wowlan suspend flow
// @use_ps_poll: use ps_poll frames
// @disable_bf: disable beacon filter
// @dbgfs_slink: debugfs symlink for this interface
// @roc_activity: the id of the roc_activity running. Relevant for STA and
// p2p device only. Set to %ROC_NUM_ACTIVITIES when not in use.
// @aux_sta: station used for remain on channel. Used in P2P device.
// @mlo_scan_start_wk: worker to start a deferred MLO scan
// @nan: NAN parameters
// @nan.links: NAN links for FW (indexed by FW link ID)
// @nan.mac_added: track whether or not the MAC was added to FW
// @nan.bcast_sta: internal station used for NAN synchronization and discovery
// activities. No queue is associated with it.
// @nan.mgmt_sta: internal station used for NAN management frames, e.g., SDFs
// and NAFs.
// @nan.mcast_data_sta: internal station used for multicast NAN Data frames.
// @nan.tx_igtk: TX IGTK key for NAN, tracked separately since NAN does not
// use the vif links.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_vif {
// Add here fields that need clean up on restart
    pub fw_id: u8,
    pub session_protect: iwl_mld_session_protect,
    pub ap_sta: *mut ieee80211_sta,
    pub authorized: bool,
    pub num_associated_stas: u8,
    pub ap_ibss_active: bool,
    pub cca_40mhz_workaround: iwl_mld_cca_40mhz_wa_status,

    pub beacon_inject_active: bool,

    pub low_latency_causes: u8,
    pub last_link_activation_time: time64_t,
// And here fields that survive a fw restart
    pub mld: *mut iwl_mld,
    pub deflink: iwl_mld_link,
    pub link: [*mut iwl_mld_link __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
// use only with wiphy protection
    pub links: [iwl_mld_nan_link; IWL_FW_MAX_LINKS],
    pub mac_added: bool,
    pub bcast_sta: iwl_mld_int_sta,
    pub mgmt_sta: iwl_mld_int_sta,
    pub mcast_data_sta: iwl_mld_int_sta,
    pub tx_igtk: *mut ieee80211_key_conf,
    pub nan: },
    pub emlsr: iwl_mld_emlsr,

    pub wowlan_data: iwl_mld_wowlan_data,

    pub use_ps_poll: bool,
    pub disable_bf: bool,
    pub dbgfs_slink: *mut dentry,

    pub roc_activity: iwl_roc_activity,
    pub aux_sta: iwl_mld_int_sta,
    pub mlo_scan_start_wk: wiphy_delayed_work,
}

extern "C" {
    pub fn container_of()mld_vif: *mut (void, ieee80211_vif: struct, _arg: drv_priv) -> return;
}
// Call only for interfaces that were added to the driver!
// Should be added to FW

// Retrieve pointer to mld link from mac80211 structures
extern "C" {
    pub fn iwl_mld_link_dereference_check(_arg: mld_vif, _arg: bss_conf->link_id) -> return;
}
// Cleanup function for struct iwl_mld_vif, will be called in restart
extern "C" {
    pub fn iwl_mld_cleanup_vif(data: *mut c_void, mac: *mut u8, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mld_add_vif(mld: *mut iwl_mld, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mld_add_nan_vif(mld: *mut iwl_mld, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mld_rm_vif(mld: *mut iwl_mld, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mld_get_fw_bss_vifs_ids(mld: *mut iwl_mld) -> u8;
}
extern "C" {
    pub fn iwl_mld_link_sta_dereference_check(_arg: mld_ap_sta, _arg: link_id) -> return;
}
