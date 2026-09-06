//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/mvm.h
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
// Copyright (C) 2012-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

pub const IWL_MVM_MAX_ADDRESSES: c_int = 5;
// RSSI offset for WkP
pub const IWL_RSSI_OFFSET: c_int = 50;
pub const IWL_MVM_MISSED_BEACONS_SINCE_RX_THOLD: c_int = 4;
pub const IWL_MVM_MISSED_BEACONS_THRESHOLD: c_int = 8;
pub const IWL_MVM_MISSED_BEACONS_THRESHOLD_LONG: c_int = 19;
// A TimeUnit is 1024 microsecond

// For GO, this value represents the number of TUs before CSA "beacon
// 0" TBTT when the CSA time-event needs to be scheduled to start.  It
// must be big enough to ensure that we switch in time.
//
pub const IWL_MVM_CHANNEL_SWITCH_TIME_GO: c_int = 40;
// For client, this value represents the number of TUs before CSA
// "beacon 1" TBTT, instead.  This is because we don't know when the
// GO/AP will be in the new channel, so we switch early enough.
//
pub const IWL_MVM_CHANNEL_SWITCH_TIME_CLIENT: c_int = 10;
//
// This value (in TUs) is used to fine tune the CSA NoA end time which should
// be just before "beacon 0" TBTT.
//
pub const IWL_MVM_CHANNEL_SWITCH_MARGIN: c_int = 4;
//
// Number of beacons to transmit on a new channel until we unblock tx to
// the stations, even if we didn't identify them on a new channel
//
pub const IWL_MVM_CS_UNBLOCK_TX_TIMEOUT: c_int = 3;
// offchannel queue towards mac80211
pub const IWL_MVM_OFFCHANNEL_QUEUE: c_int = 0;
// invalid value for FW link id
pub const IWL_MVM_FW_LINK_ID_INVALID: c_uint = 0xff;
//
// struct iwl_mvm_mod_params - module parameters for iwlmvm
// @power_scheme: one of enum iwl_power_scheme
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_mod_params {
    pub power_scheme: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_phy_ctxt {
    pub id: u16,
    pub color: u16,
    pub ref: u32,
    pub width: nl80211_chan_width,
    pub channel: *mut ieee80211_channel,
// track for RLC config command
    pub center_freq1: u32,
    pub rlc_disabled: bool,
    pub channel_load_by_us: u32,
    pub channel_load_not_by_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_time_event_data {
    pub vif: *mut ieee80211_vif,
    pub list: list_head,
    pub end_jiffies: c_ulong,
    pub duration: u32,
    pub running: bool,
    pub uid: u32,
//
// The access to the 'id' field must be done when the
// mvm->time_event_lock is held, as it value is used to indicate
// if the te is in the time event list or not (when id == TE_MAX)
//
    pub id: u32,
}

// Power management
//
// enum iwl_power_scheme - iwl power schemes
// @IWL_POWER_SCHEME_CAM: Continuously Active Mode
// @IWL_POWER_SCHEME_BPS: Balanced Power Save (default)
// @IWL_POWER_SCHEME_LP: Low Power
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_power_scheme {
    IWL_POWER_SCHEME_CAM = 1,
    IWL_POWER_SCHEME_BPS,
    IWL_POWER_SCHEME_LP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dbgfs_pm_mask {
    MVM_DEBUGFS_PM_KEEP_ALIVE = BIT(0),
    MVM_DEBUGFS_PM_SKIP_OVER_DTIM = BIT(1),
    MVM_DEBUGFS_PM_SKIP_DTIM_PERIODS = BIT(2),
    MVM_DEBUGFS_PM_RX_DATA_TIMEOUT = BIT(3),
    MVM_DEBUGFS_PM_TX_DATA_TIMEOUT = BIT(4),
    MVM_DEBUGFS_PM_LPRX_ENA = BIT(6),
    MVM_DEBUGFS_PM_LPRX_RSSI_THRESHOLD = BIT(7),
    MVM_DEBUGFS_PM_SNOOZE_ENABLE = BIT(8),
    MVM_DEBUGFS_PM_UAPSD_MISBEHAVING = BIT(9),
    MVM_DEBUGFS_PM_USE_PS_POLL = BIT(10),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dbgfs_pm {
    pub keep_alive_seconds: u16,
    pub rx_data_timeout: u32,
    pub tx_data_timeout: u32,
    pub skip_over_dtim: bool,
    pub skip_dtim_periods: u8,
    pub lprx_ena: bool,
    pub lprx_rssi_threshold: u32,
    pub snooze_ena: bool,
    pub uapsd_misbehaving: bool,
    pub use_ps_poll: bool,
    pub mask: c_int,
}

// beacon filtering
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dbgfs_bf_mask {
    MVM_DEBUGFS_BF_ENERGY_DELTA = BIT(0),
    MVM_DEBUGFS_BF_ROAMING_ENERGY_DELTA = BIT(1),
    MVM_DEBUGFS_BF_ROAMING_STATE = BIT(2),
    MVM_DEBUGFS_BF_TEMP_THRESHOLD = BIT(3),
    MVM_DEBUGFS_BF_TEMP_FAST_FILTER = BIT(4),
    MVM_DEBUGFS_BF_TEMP_SLOW_FILTER = BIT(5),
    MVM_DEBUGFS_BF_ENABLE_BEACON_FILTER = BIT(6),
    MVM_DEBUGFS_BF_DEBUG_FLAG = BIT(7),
    MVM_DEBUGFS_BF_ESCAPE_TIMER = BIT(8),
    MVM_DEBUGFS_BA_ESCAPE_TIMER = BIT(9),
    MVM_DEBUGFS_BA_ENABLE_BEACON_ABORT = BIT(10),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dbgfs_bf {
    pub bf_energy_delta: u32,
    pub bf_roaming_energy_delta: u32,
    pub bf_roaming_state: u32,
    pub bf_temp_threshold: u32,
    pub bf_temp_fast_filter: u32,
    pub bf_temp_slow_filter: u32,
    pub bf_enable_beacon_filter: u32,
    pub bf_debug_flag: u32,
    pub bf_escape_timer: u32,
    pub ba_escape_timer: u32,
    pub ba_enable_beacon_abort: u32,
    pub mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_smps_type_request {
    IWL_MVM_SMPS_REQ_BT_COEX,
    IWL_MVM_SMPS_REQ_TT,
    IWL_MVM_SMPS_REQ_PROT,
    IWL_MVM_SMPS_REQ_FW,
    NUM_IWL_MVM_SMPS_REQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_force_ant_mode {
    BT_FORCE_ANT_DIS = 0,
    BT_FORCE_ANT_AUTO,
    BT_FORCE_ANT_BT,
    BT_FORCE_ANT_WIFI,

    BT_FORCE_ANT_MAX,
}

//
// enum iwl_mvm_low_latency_force - low latency force mode set by debugfs
// @LOW_LATENCY_FORCE_UNSET: unset force mode
// @LOW_LATENCY_FORCE_ON: for low latency on
// @LOW_LATENCY_FORCE_OFF: for low latency off
// @NUM_LOW_LATENCY_FORCE: max num of modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_low_latency_force {
    LOW_LATENCY_FORCE_UNSET,
    LOW_LATENCY_FORCE_ON,
    LOW_LATENCY_FORCE_OFF,
    NUM_LOW_LATENCY_FORCE
}

//
// enum iwl_mvm_low_latency_cause - low latency set causes
// @LOW_LATENCY_TRAFFIC: indicates low latency traffic was detected
// @LOW_LATENCY_DEBUGFS: low latency mode set from debugfs
// @LOW_LATENCY_VCMD: low latency mode set from vendor command
// @LOW_LATENCY_VIF_TYPE: low latency mode set because of vif type (ap)
// @LOW_LATENCY_DEBUGFS_FORCE_ENABLE: indicate that force mode is enabled
// the actual set/unset is done with LOW_LATENCY_DEBUGFS_FORCE
// @LOW_LATENCY_DEBUGFS_FORCE: low latency force mode from debugfs
// set this with LOW_LATENCY_DEBUGFS_FORCE_ENABLE flag
// in low_latency.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_low_latency_cause {
    LOW_LATENCY_TRAFFIC = BIT(0),
    LOW_LATENCY_DEBUGFS = BIT(1),
    LOW_LATENCY_VCMD = BIT(2),
    LOW_LATENCY_VIF_TYPE = BIT(3),
    LOW_LATENCY_DEBUGFS_FORCE_ENABLE = BIT(4),
    LOW_LATENCY_DEBUGFS_FORCE = BIT(5),
}

//
// struct iwl_mvm_link_bf_data - beacon filtering related data
// @ave_beacon_signal: average beacon signal
// @last_cqm_event: rssi of the last cqm event
// @bt_coex_min_thold: minimum threshold for BT coex
// @bt_coex_max_thold: maximum threshold for BT coex
// @last_bt_coex_event: rssi of the last BT coex event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_link_bf_data {
    pub ave_beacon_signal: c_int,
    pub last_cqm_event: c_int,
    pub bt_coex_min_thold: c_int,
    pub bt_coex_max_thold: c_int,
    pub last_bt_coex_event: c_int,
}

//
// struct iwl_probe_resp_data - data for NoA/CSA updates
// @rcu_head: used for freeing the data on update
// @notif: notification data
// @noa_len: length of NoA attribute, calculated from the notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_probe_resp_data {
    pub rcu_head: rcu_head,
    pub notif: iwl_probe_resp_data_notif,
    pub noa_len: c_int,
}

//
// struct iwl_mvm_vif_link_info - per link data in Virtual Interface
// @ap_sta_id: the sta_id of the AP - valid only if VIF type is STA
// @fw_link_id: the id of the link according to the FW API
// @bssid: BSSID for this (client) interface
// @bcast_sta: station used for broadcast packets. Used by the following
// vifs: P2P_DEVICE, GO and AP.
// @beacon_stats: beacon statistics, containing the # of received beacons,
// # of received beacons accumulated over FW restart, and the current
// average signal of beacons retrieved from the firmware
// @smps_requests: the SMPS requests of different parts of the driver,
// combined on update to yield the overall request to mac80211.
// @probe_resp_data: data from FW notification to store NOA and CSA related
// data to be inserted into probe response.
// @he_ru_2mhz_block: 26-tone RU OFDMA transmissions should be blocked
// @queue_params: QoS params for this MAC
// @mgmt_queue: queue number for unbufferable management frames
// @igtk: the current IGTK programmed into the firmware
// @active: indicates the link is active in FW (for sanity checking)
// @cab_queue: content-after-beacon (multicast) queue
// @listen_lmac: indicates this link is allocated to the listen LMAC
// @csa_block_tx: we got CSA with mode=1
// @mcast_sta: multicast station
// @phy_ctxt: phy context allocated to this link, if any
// @bf_data: beacon filtering data
// @average_beacon_energy: average beacon energy for beacons received during
// client connections
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_vif_link_info {
    pub bssid: [u8; ETH_ALEN],
    pub ap_sta_id: u8,
    pub fw_link_id: u8,
    pub bcast_sta: iwl_mvm_int_sta,
    pub mcast_sta: iwl_mvm_int_sta,
    pub accu_num_beacons: u32 num_beacons,,
    pub avg_signal: u8,
    pub beacon_stats: },
    pub smps_requests: [ieee80211_smps_mode; NUM_IWL_MVM_SMPS_REQ],
    pub probe_resp_data: *mut iwl_probe_resp_data __rcu,
    pub igtk: *mut ieee80211_key_conf,
    pub he_ru_2mhz_block: bool,
    pub active: bool,
    pub listen_lmac: bool,
    pub csa_block_tx: bool,
    pub cab_queue: u16,
// Assigned while mac80211 has the link in a channel context,
// or, for P2P Device, while it exists.
//
    pub phy_ctxt: *mut iwl_mvm_phy_ctxt,
// QoS data from mac80211, need to store this here
// as mac80211 has a separate callback but we need
// to have the data for the MAC context
//
    pub queue_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub mgmt_queue: u16,
    pub bf_data: iwl_mvm_link_bf_data,
    pub average_beacon_energy: u32,
}

//
// struct iwl_mvm_vif - data per Virtual Interface, it is a MAC context
// @mvm: pointer back to the mvm struct
// @id: between 0 and 3
// @color: to solve races upon MAC addition and removal
// @associated: indicates that we're currently associated, used only for
// managing the firmware state in iwl_mvm_bss_info_changed_station()
// @ap_assoc_sta_count: count of stations associated to us - valid only
// if VIF type is AP
// @uploaded: indicates the MAC context has been added to the device
// @ap_ibss_active: indicates that AP/IBSS is configured and that the interface
// should get quota etc.
// @pm_enabled - indicate if MAC power management is allowed
// @monitor_active: indicates that monitor context is configured, and that the
// interface should get quota etc.
// @low_latency: bit flags for low latency
// see enum &iwl_mvm_low_latency_cause for causes.
// @low_latency_actual: boolean, indicates low latency is set,
// as a result from low_latency bit flags and takes force into account.
// @authorized: indicates the AP station was set to authorized
// @ps_disabled: indicates that this interface requires PS to be disabled
// @csa_countdown: indicates that CSA countdown may be started
// @csa_failed: CSA failed to schedule time event, report an error later
// @csa_bcn_pending: indicates that we are waiting for a beacon on a new channel
// @csa_blocks_tx: CSA is blocking TX
// @features: hw features active for this vif
// @max_tx_op: max TXOP in usecs for all ACs, zero for no limit.
// @ap_beacon_time: AP beacon time for synchronisation (on older FW)
// @bf_enabled: indicates if beacon filtering is enabled
// @ba_enabled: indicated if beacon abort is enabled
// @bcn_prot: beacon protection data (keys; FIXME: needs to be per link)
// @deflink: default link data for use in non-MLO
// @link: link data for each link in MLO
// @pm_enabled: indicates powersave is enabled
// @roc_activity: currently running ROC activity for this vif (or
// ROC_NUM_ACTIVITIES if no activity is running).
// @p2p_in_binding: indicates that this P2P-Device interface should be
// added to the binding, i.e. is running ROC right now
// @session_prot_connection_loss: the connection was lost due to session
// protection ending without receiving a beacon, so we need to now
// protect the deauth separately
// @ap_early_keys: The firmware cannot install keys before stations etc.,
// but higher layers work differently, so we store the keys here for
// later installation.
// @ap_sta: pointer to the AP STA data structure
// @csa_count: CSA counter (old CSA implementation w/o firmware)
// @csa_misbehave: CSA AP misbehaviour flag (old implementation)
// @csa_target_freq: CSA target channel frequency (old implementation)
// @csa_work: CSA work (old implementation)
// @dbgfs_bf: beamforming debugfs data
// @dbgfs_dir: debugfs directory for this vif
// @dbgfs_pm: power management debugfs data
// @dbgfs_quota_min: debugfs value for minimal quota
// @dbgfs_slink: debugfs symlink for this interface
// @ftm_unprotected: unprotected FTM debugfs override
// @hs_time_event_data: hotspot/AUX ROC time event data
// @mac_pwr_cmd: debugfs override for MAC power command
// @target_ipv6_addrs: IPv6 addresses on this interface for offload
// @num_target_ipv6_addrs: number of @target_ipv6_addrs
// @tentative_addrs: bitmap of tentative IPv6 addresses in @target_ipv6_addrs
// @rekey_data: rekeying data for WoWLAN GTK rekey offload
// @seqno: storage for seqno for older firmware D0/D3 transition
// @seqno_valid: indicates @seqno is valid
// @time_event_data: session protection time event data
// @tsf_id: the TSF resource ID assigned in firmware (for firmware needing that)
// @tx_key_idx: WEP transmit key index for D3
// @uapsd_misbehaving_ap_addr: MLD address/BSSID of U-APSD misbehaving AP, to
// not use U-APSD on reconnection
// @uapsd_nonagg_detected_wk: worker for handling detection of no aggregation
// in U-APSD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_vif {
    pub mvm: *mut iwl_mvm,
    pub id: u16,
    pub color: u16,
    pub associated: bool,
    pub ap_assoc_sta_count: u8,
    pub uploaded: bool,
    pub ap_ibss_active: bool,
    pub pm_enabled: bool,
    pub monitor_active: bool,
    pub session_prot_connection_loss: bool,
    pub 6: u8 low_latency:,
    pub 1: u8 low_latency_actual:,
    pub authorized:1: u8,
    pub ps_disabled: bool,
    pub ap_beacon_time: u32,
    pub bf_enabled: bool,
    pub ba_enabled: bool,

// WoWLAN GTK rekey data
    pub kck: [u8; NL80211_KCK_EXT_LEN],
    pub kek: [u8; NL80211_KEK_EXT_LEN],
    pub kek_len: usize,
    pub kck_len: usize,
    pub akm: u32,
    pub replay_ctr: __le64,
    pub valid: bool,
    pub rekey_data: },
    pub tx_key_idx: c_int,
    pub seqno_valid: bool,
    pub seqno: u16,

// IPv6 addresses for WoWLAN
    pub target_ipv6_addrs: [in6_addr; IWL_PROTO_OFFLOAD_NUM_IPV6_ADDRS_MAX],
    pub tentative_addrs: [c_ulong; BITS_TO_LONGS(IWL_PROTO_OFFLOAD_NUM_IPV6_ADDRS_MAX)],
    pub num_target_ipv6_addrs: c_int,

    pub dbgfs_dir: *mut dentry,
    pub dbgfs_slink: *mut dentry,
    pub dbgfs_pm: iwl_dbgfs_pm,
    pub dbgfs_bf: iwl_dbgfs_bf,
    pub mac_pwr_cmd: iwl_mac_power_cmd_v2,
    pub dbgfs_quota_min: c_int,
    pub ftm_unprotected: bool,

// FW identified misbehaving AP
    pub __aligned(2): u8 uapsd_misbehaving_ap_addr[ETH_ALEN],
    pub uapsd_nonagg_detected_wk: delayed_work,
    pub csa_countdown: bool,
    pub csa_failed: bool,
    pub csa_bcn_pending: bool,
    pub csa_blocks_tx: bool,
    pub csa_target_freq: u16,
    pub csa_count: u16,
    pub csa_misbehave: u16,
    pub csa_work: delayed_work,
    pub tsf_id: iwl_tsf_id,
    pub time_event_data: iwl_mvm_time_event_data,
    pub hs_time_event_data: iwl_mvm_time_event_data,
    pub roc_activity: iwl_roc_activity,
    pub p2p_in_binding: bool,
// TCP Checksum Offload
    pub features: netdev_features_t,
    pub ap_sta: *mut ieee80211_sta,
// we can only have 2 GTK + 2 IGTK active at a time
    pub ap_early_keys: [*mut ieee80211_key_conf; 4],
    pub keys: [*mut ieee80211_key_conf __rcu; 2],
    pub bcn_prot: },
    pub max_tx_op: u16,
    pub deflink: iwl_mvm_vif_link_info,
    pub link: [*mut iwl_mvm_vif_link_info; IEEE80211_MLD_MAX_NUM_LINKS],
}

pub const IWL_MVM_SCAN_STOPPING_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_status {
    IWL_MVM_SCAN_REGULAR		= BIT(0),
    IWL_MVM_SCAN_SCHED		= BIT(1),
    IWL_MVM_SCAN_NETDETECT		= BIT(2),

    IWL_MVM_SCAN_STOPPING_REGULAR	= BIT(8),
    IWL_MVM_SCAN_STOPPING_SCHED	= BIT(9),
    IWL_MVM_SCAN_STOPPING_NETDETECT	= BIT(10),
    IWL_MVM_SCAN_STOPPING_INT_MLO	= BIT(11),

    IWL_MVM_SCAN_REGULAR_MASK	= IWL_MVM_SCAN_REGULAR |
    IWL_MVM_SCAN_STOPPING_REGULAR,
    IWL_MVM_SCAN_SCHED_MASK		= IWL_MVM_SCAN_SCHED |
    IWL_MVM_SCAN_STOPPING_SCHED,
    IWL_MVM_SCAN_NETDETECT_MASK	= IWL_MVM_SCAN_NETDETECT |
    IWL_MVM_SCAN_STOPPING_NETDETECT,

    IWL_MVM_SCAN_STOPPING_MASK	= 0xff << IWL_MVM_SCAN_STOPPING_SHIFT,
    IWL_MVM_SCAN_MASK		= 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_scan_type {
    IWL_SCAN_TYPE_NOT_SET,
    IWL_SCAN_TYPE_UNASSOC,
    IWL_SCAN_TYPE_WILD,
    IWL_SCAN_TYPE_MILD,
    IWL_SCAN_TYPE_FRAGMENTED,
    IWL_SCAN_TYPE_FAST_BALANCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_sched_scan_pass_all_states {
    SCHED_SCAN_PASS_ALL_DISABLED,
    SCHED_SCAN_PASS_ALL_ENABLED,
    SCHED_SCAN_PASS_ALL_FOUND,
}

//
// struct iwl_mvm_tt_mgmt - Thermal Throttling Management structure
// @ct_kill_exit: worker to exit thermal kill
// @dynamic_smps: Is thermal throttling enabled dynamic_smps?
// @tx_backoff: The current thremal throttling tx backoff in uSec.
// @min_backoff: The minimal tx backoff due to power restrictions
// @params: Parameters to configure the thermal throttling algorithm.
// @throttle: Is thermal throttling is active?
// @power_budget_mw: maximum cTDP power budget as defined for this system and
// device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_tt_mgmt {
    pub ct_kill_exit: delayed_work,
    pub dynamic_smps: bool,
    pub tx_backoff: u32,
    pub min_backoff: u32,
    pub params: iwl_tt_params,
    pub throttle: bool,
    pub power_budget_mw: u32,
}

//
// struct iwl_mvm_thermal_device - thermal zone related data
// @trips: temperature thresholds for report
// @tzone: thermal zone device data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_thermal_device {
    pub trips: [thermal_trip; IWL_MAX_DTS_TRIPS],
    pub tzone: *mut thermal_zone_device,
}

//
// struct iwl_mvm_cooling_device
// @cur_state: current state
// @cdev: struct thermal cooling device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_cooling_device {
    pub cur_state: u32,
    pub cdev: *mut thermal_cooling_device,
}

pub const IWL_MVM_NUM_LAST_FRAMES_UCODE_RATES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_frame_stats {
    pub legacy_frames: u32,
    pub ht_frames: u32,
    pub vht_frames: u32,
    pub bw_20_frames: u32,
    pub bw_40_frames: u32,
    pub bw_80_frames: u32,
    pub bw_160_frames: u32,
    pub sgi_frames: u32,
    pub ngi_frames: u32,
    pub siso_frames: u32,
    pub mimo2_frames: u32,
    pub agg_frames: u32,
    pub ampdu_count: u32,
    pub success_frames: u32,
    pub fail_frames: u32,
    pub last_rates: [u32; IWL_MVM_NUM_LAST_FRAMES_UCODE_RATES],
    pub last_frame_idx: c_int,
}

pub const IWL_MVM_DEBUG_SET_TEMPERATURE_DISABLE: c_uint = 0xff;

pub const IWL_MVM_DEBUG_SET_TEMPERATURE_MAX: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_tdls_cs_state {
    IWL_MVM_TDLS_SW_IDLE = 0,
    IWL_MVM_TDLS_SW_REQ_SENT,
    IWL_MVM_TDLS_SW_RESP_RCVD,
    IWL_MVM_TDLS_SW_REQ_RCVD,
    IWL_MVM_TDLS_SW_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_traffic_load {
    IWL_MVM_TRAFFIC_LOW,
    IWL_MVM_TRAFFIC_MEDIUM,
    IWL_MVM_TRAFFIC_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_tcm_mac {
    pub pkts: [u32; IEEE80211_NUM_ACS],
    pub airtime: u32,
    pub tx: },
    pub pkts: [u32; IEEE80211_NUM_ACS],
    pub airtime: u32,
    pub last_ampdu_ref: u32,
    pub rx: },
// track AP's transfer in client mode
    pub rx_bytes: u64,
    pub rate: ewma_rate,
    pub detected: bool,
    pub uapsd_nonagg_detect: },
    pub opened_rx_ba_sessions: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_tcm {
    pub work: delayed_work,
    pub /: *mut *mut spinlock_t lock; / used when time elapsed,
    pub /: *mut *mut unsigned long ts; / timestamp when period ends,
    pub ll_ts: c_ulong,
    pub uapsd_nonagg_ts: c_ulong,
    pub paused: bool,
    pub data: [iwl_mvm_tcm_mac; NUM_MAC_INDEX_DRIVER],
    pub /: *mut *mut u32 elapsed; / milliseconds for this TCM period,
    pub airtime: [u32; NUM_MAC_INDEX_DRIVER],
    pub load: [iwl_mvm_traffic_load; NUM_MAC_INDEX_DRIVER],
    pub band_load: [iwl_mvm_traffic_load; NUM_NL80211_BANDS],
    pub global_load: iwl_mvm_traffic_load,
    pub low_latency: [bool; NUM_MAC_INDEX_DRIVER],
    pub change: [bool; NUM_MAC_INDEX_DRIVER],
    pub result: },
}

//
// struct iwl_mvm_reorder_buffer - per ra/tid/queue reorder buffer
// @head_sn: reorder window head sn
// @num_stored: number of mpdus stored in the buffer
// @queue: queue of this reorder buffer
// @valid: reordering is valid for this queue
// @lock: protect reorder buffer internal state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_reorder_buffer {
    pub head_sn: u16,
    pub num_stored: u16,
    pub queue: c_int,
    pub valid: bool,
    pub lock: spinlock_t,
    pub ____cacheline_aligned_in_smp: },
//
// struct iwl_mvm_reorder_buf_entry - reorder buffer entry per-queue/per-seqno
// @frames: list of skbs stored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_reorder_buf_entry {
    pub frames: sk_buff_head,
// sparse doesn't like this construct: "bad integer constant expression"

//
// struct iwl_mvm_baid_data - BA session data
// @sta_mask: current station mask for the BAID
// @tid: tid of the session
// @baid: baid of the session
// @timeout: the timeout set in the addba request
// @buf_size: the reorder buffer size as set by the last addba request
// @entries_per_queue: # of buffers per queue, this actually gets
// aligned up to avoid cache line sharing between queues
// @last_rx: last rx jiffies, updated only if timeout passed from last update
// @session_timer: timer to check if BA session expired, runs at 2 * timeout
// @rcu_ptr: BA data RCU protected access
// @rcu_head: RCU head for freeing this data
// @mvm: mvm pointer, needed for timer context
// @reorder_buf: reorder buffer, allocated per queue
// @entries: data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_baid_data {
    pub rcu_head: rcu_head,
    pub sta_mask: u32,
    pub tid: u8,
    pub baid: u8,
    pub timeout: u16,
    pub buf_size: u16,
    pub entries_per_queue: u16,
    pub last_rx: c_ulong,
    pub session_timer: timer_list,
    pub rcu_ptr: *mut iwl_mvm_baid_data __rcu,
    pub mvm: *mut iwl_mvm,
    pub reorder_buf: [iwl_mvm_reorder_buffer; IWL_MAX_RX_HW_QUEUES],
    pub ____cacheline_aligned_in_smp: iwl_mvm_reorder_buf_entry entries[],
}

//
// enum iwl_mvm_queue_status - queue status
// @IWL_MVM_QUEUE_FREE: the queue is not allocated nor reserved
// Basically, this means that this queue can be used for any purpose
// @IWL_MVM_QUEUE_RESERVED: queue is reserved but not yet in use
// This is the state of a queue that has been dedicated for some RATID
// (agg'd or not), but that hasn't yet gone through the actual enablement
// of iwl_mvm_enable_txq(), and therefore no traffic can go through it yet.
// Note that in this state there is no requirement to already know what TID
// should be used with this queue, it is just marked as a queue that will
// be used, and shouldn't be allocated to anyone else.
// @IWL_MVM_QUEUE_READY: queue is ready to be used
// This is the state of a queue that has been fully configured (including
// SCD pointers, etc), has a specific RA/TID assigned to it, and can be
// used to send traffic.
// @IWL_MVM_QUEUE_SHARED: queue is shared, or in a process of becoming shared
// This is a state in which a single queue serves more than one TID, all of
// which are not aggregated. Note that the queue is only associated to one
// RA.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_queue_status {
    IWL_MVM_QUEUE_FREE,
    IWL_MVM_QUEUE_RESERVED,
    IWL_MVM_QUEUE_READY,
    IWL_MVM_QUEUE_SHARED,
}

pub const IWL_MVM_INVALID_QUEUE: c_uint = 0xFFFF;
pub const IWL_MVM_NUM_CIPHERS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_txq {
    pub list: list_head,
    pub txq_id: u16,
    pub tx_request: core::sync::atomic::AtomicI32,
pub const IWL_MVM_TXQ_STATE_READY: c_int = 0;
pub const IWL_MVM_TXQ_STATE_STOP_FULL: c_int = 1;
pub const IWL_MVM_TXQ_STATE_STOP_REDIRECT: c_int = 2;
pub const IWL_MVM_TXQ_STATE_STOP_AP_CSA: c_int = 3;
    pub state: c_ulong,
}

//
// struct iwl_mvm_tvqm_txq_info - maps TVQM hw queue to tid
//
// @sta_id: sta id
// @txq_tid: txq tid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_tvqm_txq_info {
    pub sta_id: u8,
    pub txq_tid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_dqa_txq_info {
    pub /: *mut *mut u8 ra_sta_id; / The RA this queue is mapped to, if exists,
    pub /: *mut *mut bool reserved; / Is this the TXQ reserved for a STA,
    pub /: *mut *mut u8 mac80211_ac; / The mac80211 AC this queue is mapped to,
    pub queue*/: *mut *mut u8 txq_tid; / The TID "owner" of this,
    pub /: *mut *mut u16 tid_bitmap; / Bitmap of the TIDs mapped to this queue,
// Timestamp for inactivation per TID of this queue
    pub 1]: unsigned long last_frame_time[IWL_MAX_TID_COUNT +,
    pub status: iwl_mvm_queue_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_data {
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub dwork: delayed_work,
// The last GP2 reading from the hw
    pub last_gp2: u32,
// number of wraparounds since scale_update_adj_time_ns
    pub wrap_counter: u32,
// GP2 time when the scale was last updated
    pub scale_update_gp2: u32,
// Adjusted time when the scale was last updated in nanoseconds
    pub scale_update_adj_time_ns: u64,
// clock frequency offset, scaled to 65536000000
    pub scaled_freq: u64,
// Delta between hardware clock and ptp clock in nanoseconds
    pub delta: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_sync_data {
    pub frame_list: sk_buff_head,
    pub peer_addr: [u8; ETH_ALEN],
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mei_scan_filter {
    pub is_mei_limited_scan: bool,
    pub scan_res: sk_buff_head,
    pub scan_work: work_struct,
}

//
// struct iwl_mvm_acs_survey_channel - per-channel survey information
//
// Stripped down version of &struct survey_info.
//
// @time: time in ms the radio was on the channel
// @time_busy: time in ms the channel was sensed busy
// @time_tx: time in ms spent transmitting data
// @time_rx: time in ms spent receiving data
// @noise: channel noise in dBm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_acs_survey_channel {
    pub time: u32,
    pub time_busy: u32,
    pub time_tx: u32,
    pub time_rx: u32,
    pub noise: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_acs_survey {
    pub bands: [*mut iwl_mvm_acs_survey_channel; NUM_NL80211_BANDS],
// Overall number of channels
    pub n_channels: c_int,
// Storage space for per-channel information follows
    pub __counted_by(n_channels): iwl_mvm_acs_survey_channel channels[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm {
// for logger access
    pub dev: *mut device,
    pub trans: *mut iwl_trans,
    pub fw: *const iwl_fw,
    pub cfg: *const iwl_rf_cfg,
    pub phy_db: *mut iwl_phy_db,
    pub hw: *mut ieee80211_hw,
// for protecting access to iwl_mvm
    pub mutex: mutex,
    pub async_handlers_list: list_head,
    pub async_handlers_lock: spinlock_t,
    pub async_handlers_wk: work_struct,
// For async rx handlers that require the wiphy lock
    pub async_handlers_wiphy_wk: wiphy_work,
    pub roc_done_wk: work_struct,
    pub init_status: c_ulong,
    pub status: c_ulong,
    pub queue_sync_cookie: u32,
    pub queue_sync_state: c_ulong,
//
// for beacon filtering -
// currently only one interface can be supported
//
    pub bf_allowed_vif: *mut iwl_mvm_vif,
    pub hw_registered: bool,
    pub rfkill_safe_init_done: bool,
    pub cca_40mhz_workaround: u8,
    pub fw_rates_ver: u8,
    pub ampdu_ref: u32,
    pub ampdu_toggle: bool,
    pub notif_wait: iwl_notif_wait_data,
    pub rx_stats_v3: mvm_statistics_rx_v3,
    pub rx_stats: mvm_statistics_rx,
}

//
// NVM built based on the SAP data but that we can't free even after
// we get ownership because it contains the cfg80211's channel.
//
// NVM sections
// EEPROM MAC addresses
// data related to data path
// note: fw_id_to_link_sta must be protected by wiphy and mvm mutexes
// configured by mac80211
// Scan status, cmd (pre-allocated) and auxiliary station
// For CDB this is low band scan type, for non-CDB - type.
// max number of simultaneous scans the FW supports
// UMAC scan tracking
// start time of last scan in TSF of the mac that requested the scan
// the vif that requested the current scan
// rx chain antennas set through debugfs for the scan command
// Internal station
// last smart fifo state that was successfully sent to firmware
//
// Leave this pointer outside the ifdef below so that it can be
// assigned without ifdef in the source code.
//

//
// A bitmap indicating the index of the key in use. The firmware
// can hold 16 keys at most. Reflect this fact.
//

// sched scan settings for net detect

// Aux ROC
// Thermal Throttling and CTkill

//
// Debug option to set the NIC temperature. This option makes the
// driver think this is the actual NIC temperature, and ignore the
// real temperature that is received from the fw
//
// Tx queues
// Indicate if device power save is allowed
// Indicate if 32Khz external clock is valid
// This vif used by CSME to send / receive traffic
// indicates that we transmitted the last beacon
// TDLS channel switch data
//
// Current cs sta - might be different from periodic cs peer
// station. Value is meaningless when the cs-state is idle.
//
// TDLS periodic channel-switch peer
// timestamp of last ch-sw request sent (GP2 time)
//
// Drop beacons from other APs in AP mode when there are no connected
// clients.
//
// does a monitor vif exist (only one can exist hence bool)
//
// primary channel position relative to he whole bandwidth,
// in steps of 80 MHz
//
// sniffer data to include in radiotap
// report rx timestamp in ptp clock time
//
// Indicates that firmware will do a product reset (and then
// therefore fail to load) when we start it (due to OTP burn),
// if so don't dump errors etc. since this is expected.
//
// Extract MVM priv from op_mode and _hw

//
// enum iwl_mvm_status - MVM status bits
// @IWL_MVM_STATUS_HW_RFKILL: HW RF-kill is asserted
// @IWL_MVM_STATUS_HW_CTKILL: CT-kill is active
// @IWL_MVM_STATUS_ROC_P2P_RUNNING: remain-on-channel on P2P is running (when
// P2P is not over AUX)
// @IWL_MVM_STATUS_HW_RESTART_REQUESTED: HW restart was requested
// @IWL_MVM_STATUS_IN_HW_RESTART: HW restart is active
// @IWL_MVM_STATUS_ROC_AUX_RUNNING: AUX remain-on-channel is running
// @IWL_MVM_STATUS_FIRMWARE_RUNNING: firmware is running
// @IWL_MVM_STATUS_IN_D3: in D3 (or at least about to go into it)
// @IWL_MVM_STATUS_SUPPRESS_ERROR_LOG_ONCE: suppress one error log
// if this is set, when intentionally triggered
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_status {
    IWL_MVM_STATUS_HW_RFKILL,
    IWL_MVM_STATUS_HW_CTKILL,
    IWL_MVM_STATUS_ROC_P2P_RUNNING,
    IWL_MVM_STATUS_HW_RESTART_REQUESTED,
    IWL_MVM_STATUS_IN_HW_RESTART,
    IWL_MVM_STATUS_ROC_AUX_RUNNING,
    IWL_MVM_STATUS_FIRMWARE_RUNNING,
    IWL_MVM_STATUS_IN_D3,
    IWL_MVM_STATUS_SUPPRESS_ERROR_LOG_ONCE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_csme_conn_info {
    pub rcu_head: rcu_head,
    pub conn_info: iwl_mei_conn_info,
}

// Keep track of completed init configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_init_status {
    IWL_MVM_INIT_STATUS_THERMAL_INIT_COMPLETE = BIT(0),
    IWL_MVM_INIT_STATUS_LEDS_INIT_COMPLETE = BIT(1),
}

extern "C" {
    pub fn test_bit(_arg: IWL_MVM_STATUS_HW_RFKILL, _arg: &mvm->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: IWL_MVM_STATUS_FIRMWARE_RUNNING, _arg: &mvm->status) -> return;
}
// Must be called with rcu_read_lock() held and it can only be
// released when mvmsta is not needed anymore.
//
// This can happen if the station has been removed right now
extern "C" {
    pub fn iwl_mvm_sta_from_mac80211(_arg: sta) -> return;
}
// This can happen if the station has been removed right now
extern "C" {
    pub fn iwl_mvm_sta_from_mac80211(_arg: sta) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: mvm->vif_id_to_mac[vif_id]) -> return;
}
// OCE should never be enabled for LMAC scan FWs
extern "C" {
    pub fn fw_has_api(_arg: &mvm->fw->ucode_capa, _arg: IWL_UCODE_TLV_API_OCE) -> return;
}
extern "C" {
    pub fn fw_has_api(_arg: &mvm->fw->ucode_capa, _arg: IWL_UCODE_TLV_API_FRAG_EBS) -> return;
}
//
// Enable LAR only if it is supported by the FW (TLV) &&
// enabled in the NVM
//
// TODO - replace with TLV once defined
// TODO - better define this
//
// TODO:
// The issue of how to determine CDB APIs and usage is still not fully
// defined.
// There is a compilation for CDB and non-CDB FW, but there may
// be also runtime check.
// For now there is a TLV for checking compilation mode, but a
// runtime check will also have to be here - once defined.
//
// TODO: should this be the same as iwl_mvm_is_cdb_supported()?
// but then there's a little bit of code in scan that won't make
// any sense...
//
// these two TLV are redundant since the responsibility to CT-kill by
// FW happens only after we send at least one command of
// temperature THs report.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rate_info {
    pub /: *mut *mut u8 plcp; / uCode API: IWL_RATE_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_siso; / uCode API: IWL_RATE_SISO_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_mimo2; / uCode API: IWL_RATE_MIMO2_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_mimo3; / uCode API: IWL_RATE_MIMO3_6M_PLCP, etc.,
    pub /: *mut *mut u8 ieee; / MAC header: IWL_RATE_6M_IEEE, etc.,
}

extern "C" {
    pub fn __iwl_mvm_mac_stop(mvm: *mut iwl_mvm, suspend: bool);
}
extern "C" {
    pub fn __iwl_mvm_mac_start(mvm: *mut iwl_mvm) -> c_int;
}
//
// MVM Methods
//
// uCode
extern "C" {
    pub fn iwl_run_init_mvm_ucode(mvm: *mut iwl_mvm) -> c_int;
}
// Utils
extern "C" {
    pub fn iwl_mvm_rate_idx_to_plcp(idx: c_int) -> u8;
}
extern "C" {
    pub fn iwl_mvm_rate_idx_to_fw_idx(fw: *const iwl_fw, rate_idx: c_int) -> u8;
}
extern "C" {
    pub fn iwl_mvm_mac80211_ac_to_ucode_ac(ac: ieee80211_ac_numbers) -> u8;
}
extern "C" {
    pub fn iwl_mvm_is_nic_ack_enabled(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> bool;
}
extern "C" {
    pub fn iwl_mvm_restart_cleanup(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn first_antenna(mask: u8) -> u8;
}
extern "C" {
    pub fn iwl_mvm_next_antenna(mvm: *mut iwl_mvm, valid: u8, last_idx: u8) -> u8;
}
extern "C" {
    pub fn iwl_mvm_get_systime(mvm: *mut iwl_mvm) -> u32;
}
// Tx / Host Commands

extern "C" {
    pub fn iwl_mvm_tx_skb_non_sta(mvm: *mut iwl_mvm, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_itxq_xmit(hw: *mut ieee80211_hw, txq: *mut ieee80211_txq);
}

extern "C" {
    pub fn iwl_mvm_flush_tx_path(mvm: *mut iwl_mvm, tfd_msk: u32) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_flush_sta(mvm: *mut iwl_mvm, sta_id: u32, tfd_queue_mask: u32) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_flush_sta_tids(mvm: *mut iwl_mvm, sta_id: u32, tids: u16) -> c_int;
}
// Utils to extract sta related data
extern "C" {
    pub fn iwl_mvm_get_sta_uapsd_acs(sta: *mut ieee80211_sta) -> u8;
}
extern "C" {
    pub fn iwl_mvm_async_handlers_purge(mvm: *mut iwl_mvm);
}
// Statistics
extern "C" {
    pub fn iwl_mvm_request_statistics(mvm: *mut iwl_mvm, clear: bool) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_accu_radio_stats(mvm: *mut iwl_mvm);
}
// NVM
extern "C" {
    pub fn iwl_nvm_init(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_load_nvm_to_nic(mvm: *mut iwl_mvm) -> c_int;
}
// ant = iwl_mvm_next_antenna(mvm, iwl_mvm_get_valid_tx_ant(mvm), *ant);
extern "C" {
    pub fn iwl_mvm_up(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_load_d3_fw(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_setup_register(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_init_mvmvif(mvm: *mut iwl_mvm, mvmvif: *mut iwl_mvm_vif);
}
//
// FW notifications / CMD responses handlers
// Convention: iwl_mvm_rx_<NAME OF THE CMD>
//
extern "C" {
    pub fn iwl_mvm_rx_rx_phy_cmd(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_mvm_rx_tx_cmd(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_mvm_send_recovery_cmd(mvm: *mut iwl_mvm, flags: u32);
}
extern "C" {
    pub fn iwl_mvm_rx_ba_notif(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_mvm_rx_fw_error(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
// MVM PHY
extern "C" {
    pub fn iwl_mvm_phy_ctx_count(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_get_channel_width(chandef: *const cfg80211_chan_def) -> u8;
}
extern "C" {
    pub fn iwl_mvm_get_ctrl_pos(chandef: *const cfg80211_chan_def) -> u8;
}
// MAC (virtual interface) programming
extern "C" {
    pub fn iwl_mvm_get_mac_type(vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mld_mac_ctxt_add(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mld_mac_ctxt_remove(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_ctxt_init(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_ctxt_add(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_ctxt_remove(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_sta_pm_notif(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
// Bindings
extern "C" {
    pub fn iwl_mvm_binding_add_vif(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_binding_remove_vif(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_get_lmac_id(mvm: *mut iwl_mvm, band: nl80211_band) -> u32;
}
// Links
extern "C" {
    pub fn iwl_mvm_init_link(link: *mut iwl_mvm_vif_link_info);
}

// AP and IBSS
// BSS Info
// ROC
//
// struct iwl_mvm_roc_ops - callbacks for the remain_on_channel()
//
// Since the only difference between both MLD and
// non-MLD versions of remain_on_channel() is these function calls,
// each version will send its specific function calls to
// %iwl_mvm_roc_common().
//
// @add_aux_sta_for_hs20: pointer to the function that adds an aux sta
// for Hot Spot 2.0
// @link: For a P2P Device interface, pointer to a function that links the
// MAC/Link to the PHY context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_roc_ops {
    pub lmac_id): *mut *mut *mut int (add_aux_sta_for_hs20)(struct iwl_mvm mvm, u32,
    pub vif): *mut *mut *mut int (link)(struct iwl_mvm mvm, struct ieee80211_vif,
}

// Session Protection
// Quota management
// iwl_mvm_quota_cmd_get_quota(struct iwl_mvm *mvm,
// Scanning
extern "C" {
    pub fn iwl_mvm_scan_size(mvm: *mut iwl_mvm) -> usize;
}
extern "C" {
    pub fn iwl_mvm_scan_stop(mvm: *mut iwl_mvm, type: c_int, notify: bool) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_max_scan_ie_len(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_report_scan_aborted(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_scan_timeout_wk(work: *mut work_struct);
}
// Scheduled scan
// UMAC scan
extern "C" {
    pub fn iwl_mvm_config_scan(mvm: *mut iwl_mvm) -> c_int;
}
// MVM debugfs

extern "C" {
    pub fn iwl_mvm_dbgfs_register(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_vif_add_debugfs(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mvm_vif_dbgfs_add_link(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mvm_vif_dbgfs_rm_link(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}

// rate scaling
extern "C" {
    pub fn iwl_mvm_send_lq_cmd(mvm: *mut iwl_mvm, lq: *mut iwl_lq_cmd) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_update_frame_stats(mvm: *mut iwl_mvm, rate: u32, agg: bool);
}
extern "C" {
    pub fn rs_pretty_print_rate_v1(buf: *mut c_char, bufsz: c_int, rate: u32) -> c_int;
}
// power management
extern "C" {
    pub fn iwl_mvm_power_update_device(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_power_update_mac(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_power_update_ps(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_power_vif_assoc(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}

extern "C" {
    pub fn iwl_mvm_leds_init(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_leds_exit(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_leds_sync(mvm: *mut iwl_mvm);
}

// D3 (WoWLAN, NetDetect)
extern "C" {
    pub fn iwl_mvm_suspend(hw: *mut ieee80211_hw, wowlan: *mut cfg80211_wowlan) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_resume(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_set_wakeup(hw: *mut ieee80211_hw, enabled: bool);
}

extern "C" {
    pub fn iwl_mvm_fast_suspend(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_fast_resume(mvm: *mut iwl_mvm) -> c_int;
}

// BT Coex
extern "C" {
    pub fn iwl_mvm_send_bt_init_conf(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_bt_coex_vif_change(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_bt_coex_is_ant_avail(mvm: *mut iwl_mvm, ant: u8) -> bool;
}
extern "C" {
    pub fn iwl_mvm_bt_coex_is_shared_ant_avail(mvm: *mut iwl_mvm) -> bool;
}
extern "C" {
    pub fn iwl_mvm_bt_coex_get_single_ant_msk(mvm: *mut iwl_mvm, enabled_ants: u8) -> u8;
}
// beacon filtering

// SMPS
// Low latency
// get SystemLowLatencyMode - only needed for beacon threshold?
extern "C" {
    pub fn iwl_mvm_low_latency(mvm: *mut iwl_mvm) -> bool;
}
extern "C" {
    pub fn iwl_mvm_low_latency_band(mvm: *mut iwl_mvm, band: nl80211_band) -> bool;
}
// get VMACLowLatencyMode
//
// should this consider associated/active/... state?
//
// Normally low-latency should only be active on interfaces
// that are active, but at least with debugfs it can also be
// enabled on interfaces that aren't active. However, when
// interface aren't active then they aren't added into the
// binding, so this has no real impact. For now, just return
// the current desired low-latency state.
//
// if LOW_LATENCY_DEBUGFS_FORCE_ENABLE is enabled no changes are
// allowed to actual mode.
//
// We enter force state
//
// Check if any other one set low latency
//
// Return a bitmask with all the hw supported queues, except for the
// command queue, which can't be flushed.
//
extern "C" {
    pub fn iwl_mvm_stop_device(mvm: *mut iwl_mvm);
}
// Thermal management and CT-kill
extern "C" {
    pub fn iwl_mvm_tt_tx_backoff(mvm: *mut iwl_mvm, backoff: u32);
}
extern "C" {
    pub fn iwl_mvm_tt_handler(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_thermal_initialize(mvm: *mut iwl_mvm, min_backoff: u32);
}
extern "C" {
    pub fn iwl_mvm_thermal_exit(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_set_hw_ctkill_state(mvm: *mut iwl_mvm, state: bool);
}
extern "C" {
    pub fn iwl_mvm_get_temp(mvm: *mut iwl_mvm, temp: *mut i32) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_ct_kill_notif(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_mvm_enter_ctkill(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_send_temp_report_ths_cmd(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_ctdp_command(mvm: *mut iwl_mvm, op: u32, budget: u32) -> c_int;
}

// vendor commands
extern "C" {
    pub fn iwl_mvm_vendor_cmds_register(mvm: *mut iwl_mvm);
}

// Location Aware Regulatory
extern "C" {
    pub fn iwl_mvm_init_mcc(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_init_fw_regd(mvm: *mut iwl_mvm, force_regd_sync: bool) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_update_changed_regdom(mvm: *mut iwl_mvm);
}
// smart fifo
// FTM responder
// FTM initiator
extern "C" {
    pub fn iwl_mvm_ftm_restart(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_ftm_abort(mvm: *mut iwl_mvm, req: *mut cfg80211_pmsr_request);
}
extern "C" {
    pub fn iwl_mvm_ftm_initiator_smooth_config(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_ftm_initiator_smooth_stop(mvm: *mut iwl_mvm);
}
// TDLS
//
// We use TID 4 (VI) as a FW-used-only TID when TDLS connections are present.
// This TID is marked as used vs the AP and all connected TDLS peers.
//
pub const IWL_MVM_TDLS_FW_TID: c_int = 4;
extern "C" {
    pub fn iwl_mvm_tdls_sta_count(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_teardown_tdls_peers(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_rx_tdls_notif(mvm: *mut iwl_mvm, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_mvm_tdls_ch_switch_work(work: *mut work_struct);
}
extern "C" {
    pub fn iwl_mvm_is_vif_assoc(mvm: *mut iwl_mvm) -> bool;
}
pub const MVM_TCM_PERIOD_MSEC: c_int = 500;

extern "C" {
    pub fn iwl_mvm_tcm_work(work: *mut work_struct);
}
extern "C" {
    pub fn iwl_mvm_pause_tcm(mvm: *mut iwl_mvm, with_cancel: bool);
}
extern "C" {
    pub fn iwl_mvm_resume_tcm(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_tcm_add_vif(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mvm_tcm_rm_vif(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mvm_tcm_load_percentage(airtime: u32, elapsed: u32) -> u8;
}
extern "C" {
    pub fn iwl_mvm_mei_scan_filter_init(mei_scan_filter: *mut iwl_mei_scan_filter);
}
extern "C" {
    pub fn iwl_mvm_ptp_init(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_ptp_remove(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_ptp_get_adj_time(mvm: *mut iwl_mvm, base_time: u64) -> u64;
}
extern "C" {
    pub fn iwl_mvm_sar_select_profile(mvm: *mut iwl_mvm, prof_a: c_int, prof_b: c_int) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_get_sar_geo_profile(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_ppag_send_cmd(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_get_bios_tables(mvm: *mut iwl_mvm);
}

// new MLD related APIs
extern "C" {
    pub fn iwl_rfi_supported(mvm: *mut iwl_mvm) -> bool;
}
// Channel Switch
extern "C" {
    pub fn iwl_mvm_channel_switch_disconnect_wk(wk: *mut work_struct);
}
// Channel Context
//
// struct iwl_mvm_switch_vif_chanctx_ops - callbacks for switch_vif_chanctx()
//
// Since the only difference between both MLD and
// non-MLD versions of switch_vif_chanctx() is these function calls,
// each version will send its specific function calls to
// %iwl_mvm_switch_vif_chanctx_common().
//
// @__assign_vif_chanctx: pointer to the function that assigns a chanctx to
// a given vif
// @__unassign_vif_chanctx: pointer to the function that unassigns a chanctx to
// a given vif
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_switch_vif_chanctx_ops {
    pub switching_chanctx): bool,
    pub switching_chanctx): bool,
}

// Channel info utils
extern "C" {
    pub fn iwl_mei_get_ownership() -> return;
}
// Callbacks for ieee80211_ops
extern "C" {
    pub fn iwl_mvm_mac_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mac_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn iwl_mvm_tx_last_beacon(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_sync_rx_queues(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn iwl_mvm_vif_is_active(mvmvif: *mut iwl_mvm_vif) -> bool;
}
extern "C" {
    pub fn iwl_mvm_update_mu_groups(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
// rate_n_flags conversion
extern "C" {
    pub fn iwl_mvm_v3_rate_from_fw(rate: __le32, rate_ver: u8) -> u32;
}
extern "C" {
    pub fn iwl_mvm_v3_rate_to_fw(rate: u32, rate_ver: u8) -> __le32;
}
