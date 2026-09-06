//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/core.h
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
// Copyright (c) 2017-2026 Morse Micro
//

pub const MM81X_DRIVER_SEMVER_MAJOR: c_int = 56;
pub const MM81X_DRIVER_SEMVER_MINOR: c_int = 3;
pub const MM81X_DRIVER_SEMVER_PATCH: c_int = 0;

pub const BCF_SIZE_MAX: c_int = 48;

// Max number of interfaces

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_caps_flags {
    MM81X_CAPS_FW_START = 0,
    MM81X_CAPS_2MHZ = MM81X_CAPS_FW_START,
    MM81X_CAPS_4MHZ,
    MM81X_CAPS_8MHZ,
    MM81X_CAPS_16MHZ,
    MM81X_CAPS_SGI,
    MM81X_CAPS_S1G_LONG,
    MM81X_CAPS_TRAVELING_PILOT_ONE_STREAM,
    MM81X_CAPS_TRAVELING_PILOT_TWO_STREAM,
    MM81X_CAPS_MU_BEAMFORMEE,
    MM81X_CAPS_MU_BEAMFORMER,
    MM81X_CAPS_RD_RESPONDER,
    MM81X_CAPS_STA_TYPE_SENSOR,
    MM81X_CAPS_STA_TYPE_NON_SENSOR,
    MM81X_CAPS_GROUP_AID,
    MM81X_CAPS_NON_TIM,
    MM81X_CAPS_TIM_ADE,
    MM81X_CAPS_BAT,
    MM81X_CAPS_DYNAMIC_AID,
    MM81X_CAPS_UPLINK_SYNC,
    MM81X_CAPS_FLOW_CONTROL,
    MM81X_CAPS_AMPDU,
    MM81X_CAPS_AMSDU,
    MM81X_CAPS_1MHZ_CONTROL_RESPONSE_PREAMBLE,
    MM81X_CAPS_PAGE_SLICING,
    MM81X_CAPS_RAW,
    MM81X_CAPS_MCS8,
    MM81X_CAPS_MCS9,
    MM81X_CAPS_ASYMMETRIC_BA_SUPPORT,
    MM81X_CAPS_DAC,
    MM81X_CAPS_CAC,
    MM81X_CAPS_TXOP_SHARING_IMPLICIT_ACK,
    MM81X_CAPS_NDP_PSPOLL,
    MM81X_CAPS_FRAGMENT_BA,
    MM81X_CAPS_OBSS_MITIGATION,
    MM81X_CAPS_TMP_PS_MODE_SWITCH,
    MM81X_CAPS_SECTOR_TRAINING,
    MM81X_CAPS_UNSOLICIT_DYNAMIC_AID,
    MM81X_CAPS_NDP_BEAMFORMING_REPORT,
    MM81X_CAPS_MCS_NEGOTIATION,
    MM81X_CAPS_DUPLICATE_1MHZ,
    MM81X_CAPS_TACK_AS_PSPOLL,
    MM81X_CAPS_PV1,
    MM81X_CAPS_TWT_RESPONDER,
    MM81X_CAPS_TWT_REQUESTER,
    MM81X_CAPS_BDT,
    MM81X_CAPS_TWT_GROUPING,
    MM81X_CAPS_LINK_ADAPTATION_WO_NDP_CMAC,
    MM81X_CAPS_LONG_MPDU,
    MM81X_CAPS_TXOP_SECTORIZATION,
    MM81X_CAPS_GROUP_SECTORIZATION,
    MM81X_CAPS_HTC_VHT,
    MM81X_CAPS_HTC_VHT_MFB,
    MM81X_CAPS_HTC_VHT_MRQ,
    MM81X_CAPS_2SS,
    MM81X_CAPS_3SS,
    MM81X_CAPS_4SS,
    MM81X_CAPS_SU_BEAMFORMEE,
    MM81X_CAPS_SU_BEAMFORMER,
    MM81X_CAPS_RX_STBC,
    MM81X_CAPS_TX_STBC,
    MM81X_CAPS_RX_LDPC,
    MM81X_CAPS_HW_FRAGMENT,

    MM81X_CAPS_FW_END = MM81X_CAPS_MAX_FW_VAL,
    MM81X_CAPS_LAST = MM81X_CAPS_FW_END,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_fw_caps {
    pub flags: [u32; FW_CAPABILITIES_FLAGS_WIDTH],
    pub ampdu_mss: u8,
    pub beamformee_sts_capability: u8,
    pub number_sounding_dimensions: u8,
    pub maximum_ampdu_length_exponent: u8,
    pub mm81x_mmss_offset: u8,
}

extern "C" {
    pub fn test_bit(_arg: flag, _arg: flags_ptr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_ps {
    pub wakers: u32,
    pub enable: bool,
    pub suspended: bool,
// PS state lock
    pub lock: mutex,
    pub delayed_eval_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_page_aci {
    MM81X_ACI_BE = 0,
    MM81X_ACI_BK = 1,
    MM81X_ACI_VI = 2,
    MM81X_ACI_VO = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_qos_tid_up_index {
    MM81X_QOS_TID_UP_BK = 1,
    MM81X_QOS_TID_UP_XX = 2,
    MM81X_QOS_TID_UP_BE = 0,
    MM81X_QOS_TID_UP_EE = 3,
    MM81X_QOS_TID_UP_CL = 4,
    MM81X_QOS_TID_UP_VI = 5,
    MM81X_QOS_TID_UP_VO = 6,
    MM81X_QOS_TID_UP_NC = 7,

    MM81X_QOS_TID_UP_LOWEST = MM81X_QOS_TID_UP_BK,
    MM81X_QOS_TID_UP_HIGHEST = MM81X_QOS_TID_UP_NC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_sw_version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_sta {
    pub vif: *const ieee80211_vif,
    pub addr: [u8; ETH_ALEN],
    pub state: ieee80211_sta_state,
    pub tid_tx: [bool; IEEE80211_NUM_TIDS],
    pub tid_start_tx: [bool; IEEE80211_NUM_TIDS],
    pub tid_params: [u8; IEEE80211_NUM_TIDS],
    pub max_bw_mhz: c_int,
    pub rc: mm81x_rc_sta,
    pub last_sta_tx_rate: mmrc_rate,
    pub avg_rssi: i16,
    pub tx_ps_filter_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_vif {
    pub mors: *mut mm81x,
    pub id: u16,
    pub is_assoc: bool,
    pub sta: },
    pub num_stas: u32,
    pub beacon_work: work_struct,
    pub ap: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_stale_tx_status {
// Stale Tx lock
    pub lock: spinlock_t,
    pub timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcast_filter {
    pub count: u8,
//
// Integer representation of the last four bytes of a multicast MAC
// address. The first two bytes are always 0x0100 (IPv4) or 0x3333
// (IPv6).
//
    pub addr_list: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_hw_scan_op {
    MM81X_HW_SCAN_OP_START,
    MM81X_HW_SCAN_OP_STOP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_hw_scan_params {
    pub hw: *mut ieee80211_hw,
// vif which initiated the scan
    pub vif: *mut ieee80211_vif,
    pub has_directed_ssid: bool,
    pub dwell_time_ms: u32,
    pub dwell_on_home_ms: u32,
    pub operation: mm81x_hw_scan_op,
    pub store: bool,
    pub probe_req: *mut sk_buff,
    pub num_chans: u16,
    pub allocated_chans: u16,
    pub channel: *mut ieee80211_channel,
// Index into @ref powers_qdbm for the power of this channel
    pub power_idx: u8,
    pub channels: *mut },
    pub powers_qdbm: *mut i32,
    pub n_powers: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_hw_scan_state {
    HW_SCAN_STATE_IDLE,
    HW_SCAN_STATE_RUNNING,
    HW_SCAN_STATE_ABORTING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_hw_scan {
    pub state: mm81x_hw_scan_state,
    pub scan_done: completion,
    pub params: *mut mm81x_hw_scan_params,
    pub timeout: delayed_work,
    pub home_dwell_ms: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_hif_event_flags {
    MM81X_HIF_EVT_RX_PEND,
    MM81X_HIF_EVT_PAGE_RETURN_PEND,
    MM81X_HIF_EVT_TX_COMMAND_PEND,
    MM81X_HIF_EVT_TX_BEACON_PEND,
    MM81X_HIF_EVT_TX_MGMT_PEND,
    MM81X_HIF_EVT_TX_DATA_PEND,
    MM81X_HIF_EVT_TX_PACKET_FREED_UP_PEND,
    MM81X_HIF_EVT_DATA_TRAFFIC_PAUSE_PEND,
    MM81X_HIF_EVT_DATA_TRAFFIC_RESUME_PEND,
    MM81X_HIF_EVT_UPDATE_HW_CLOCK_REFERENCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_state_flags {
    MM81X_STATE_CHIP_UNRESPONSIVE,
    MM81X_STATE_DATA_QS_STOPPED,
    MM81X_STATE_DATA_TX_STOPPED,
    MM81X_STATE_REGDOM_SET_BY_USER,
    MM81X_STATE_REGDOM_SET_BY_OTP,
    MM81X_STATE_RELOAD_FW_AFTER_START,
    MM81X_STATE_HOST_TO_CHIP_TX_BLOCKED,
    MM81X_STATE_HOST_TO_CHIP_CMD_BLOCKED,
}

pub const INVALID_VIF_INDEX: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x {
    pub chip_id: u32,
    pub host_table_ptr: u32,
// Refer to @enum mm81x_bus_type
    pub bus_type: u32,
    pub bcf_address: u32,
//
// Parsed from the release tag, which should be in the format
// 'rel_<major>_<minor>_<patch>'. If the tag is not in this format
// then corresponding version field will be 0.
//
    pub sw_ver: mm81x_sw_version,
    pub macaddr: [u8; ETH_ALEN],
    pub country: [u8; MM81X_COUNTRY_LEN],
// Mask of type @enum host_table_firmware_flags
    pub fw_flags: u32,
    pub fw_major: u32,
    pub fw_caps: mm81x_fw_caps,
    pub started: bool,
    pub chip_was_reset: bool,
    pub wiphy: *mut wiphy,
    pub hw_scan: mm81x_hw_scan,
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub vifs: [*mut ieee80211_vif __rcu; MM81X_MAX_IF],
// @mm81x_state_flags
    pub state_flags: c_ulong,
    pub cmd_seq: u16,
    pub cmd_comp: *mut completion,
// Serialises commands
    pub cmd_lock: mutex,
// Serialises command completion
    pub cmd_wait: mutex,
    pub regs: *const mm81x_regs,
    pub yaps: mm81x_yaps,
    pub u: },
    pub ops: *const mm81x_hif_ops,
// See @enum mm81x_hif_event_flags for values
    pub event_flags: c_ulong,
    pub validate_skb_checksum: bool,
    pub hif: },
    pub chip_wq: *mut workqueue_struct,
    pub hif_work: work_struct,
    pub usb_irq_work: work_struct,
    pub stale_status: mm81x_stale_tx_status,
    pub config_ps: bool,
    pub ps: mm81x_ps,
// Tx power in mBm received from the FW before association
    pub tx_power_mbm: i32,
    pub tx_max_power_mbm: i32,
    pub bus_ops: *const mm81x_bus_ops,
    pub mrc: mm81x_rc,
    pub rts_threshold: c_int,
    pub net_wq: *mut workqueue_struct,
    pub tx_stale_work: work_struct,
    pub tx_empty_waitq: wait_queue_head_t,
    pub chandef: cfg80211_chan_def,
    pub mcast_filter: *mut mcast_filter,
    pub num_bcn_vifs: core::sync::atomic::AtomicI32,
    pub beacon_irqs_enabled: c_ulong,
    pub )): *mut u8 drv_priv[] __aligned(sizeof(void,
}

// Map from mac80211 queue to Morse ACI value for page metadata
extern "C" {
    pub fn container_of()mors_vif: *mut (void, ieee80211_vif: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn mm81x_beacon_init(mors_vif: *mut mm81x_vif) -> c_int;
}
extern "C" {
    pub fn mm81x_beacon_finish(mors_vif: *mut mm81x_vif);
}
extern "C" {
    pub fn mm81x_beacon_irq_handle(mors: *mut mm81x, status: u32);
}
extern "C" {
    pub fn mm81x_core_init(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_core_register(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_core_unregister(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_core_deinit(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_core_free(mors: *mut mm81x);
}
