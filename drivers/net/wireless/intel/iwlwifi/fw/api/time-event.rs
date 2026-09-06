//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/time-event.h
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
// Copyright (C) 2012-2014, 2018-2020, 2022-2025 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_time_event_h__

// Time Event types, according to MAC type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_time_event_type {
// BSS Station Events
    TE_BSS_STA_AGGRESSIVE_ASSOC,
    TE_BSS_STA_ASSOC,
    TE_BSS_EAP_DHCP_PROT,
    TE_BSS_QUIET_PERIOD,

// P2P Device Events
    TE_P2P_DEVICE_DISCOVERABLE,
    TE_P2P_DEVICE_LISTEN,
    TE_P2P_DEVICE_ACTION_SCAN,
    TE_P2P_DEVICE_FULL_SCAN,

// P2P Client Events
    TE_P2P_CLIENT_AGGRESSIVE_ASSOC,
    TE_P2P_CLIENT_ASSOC,
    TE_P2P_CLIENT_QUIET_PERIOD,

// P2P GO Events
    TE_P2P_GO_ASSOC_PROT,
    TE_P2P_GO_REPETITIVET_NOA,
    TE_P2P_GO_CT_WINDOW,

// WiDi Sync Events
    TE_WIDI_TX_SYNC,

// Channel Switch NoA
    TE_CHANNEL_SWITCH_PERIOD,

    TE_MAX
}

// Time event - defines for command API v1
//
// @TE_V1_FRAG_NONE: fragmentation of the time event is NOT allowed.
// @TE_V1_FRAG_SINGLE: fragmentation of the time event is allowed, but only
// the first fragment is scheduled.
// @TE_V1_FRAG_DUAL: fragmentation of the time event is allowed, but only
// the first 2 fragments are scheduled.
// @TE_V1_FRAG_ENDLESS: fragmentation of the time event is allowed, and any
// number of fragments are valid.
//
// Other than the constant defined above, specifying a fragmentation value 'x'
// means that the event can be fragmented but only the first 'x' will be
// scheduled.
//
// If a Time Event can be fragmented, this is the max number of fragments
pub const TE_V1_FRAG_MAX_MSK: c_uint = 0x0fffffff;
// Repeat the time event endlessly (until removed)
pub const TE_V1_REPEAT_ENDLESS: c_uint = 0xffffffff;
// If a Time Event has bounded repetitions, this is the maximal value
pub const TE_V1_REPEAT_MAX_MSK_V1: c_uint = 0x0fffffff;
// Time Event dependencies: none, on another TE, or in a specific time
//
// @TE_V1_NOTIF_NONE: no notifications
// @TE_V1_NOTIF_HOST_EVENT_START: request/receive notification on event start
// @TE_V1_NOTIF_HOST_EVENT_END:request/receive notification on event end
// @TE_V1_NOTIF_INTERNAL_EVENT_START: internal FW use
// @TE_V1_NOTIF_INTERNAL_EVENT_END: internal FW use.
// @TE_V1_NOTIF_HOST_FRAG_START: request/receive notification on frag start
// @TE_V1_NOTIF_HOST_FRAG_END:request/receive notification on frag end
// @TE_V1_NOTIF_INTERNAL_FRAG_START: internal FW use.
// @TE_V1_NOTIF_INTERNAL_FRAG_END: internal FW use.
//
// Supported Time event notifications configuration.
// A notification (both event and fragment) includes a status indicating weather
// the FW was able to schedule the event or not. For fragment start/end
// notification the status is always success. There is no start/end fragment
// notification for monolithic events.
//
// Time event - defines for command API
//
// @TE_V2_FRAG_NONE: fragmentation of the time event is NOT allowed.
// @TE_V2_FRAG_SINGLE: fragmentation of the time event is allowed, but only
// the first fragment is scheduled.
// @TE_V2_FRAG_DUAL: fragmentation of the time event is allowed, but only
// the first 2 fragments are scheduled.
// @TE_V2_FRAG_ENDLESS: fragmentation of the time event is allowed, and any
// number of fragments are valid.
//
// Other than the constant defined above, specifying a fragmentation value 'x'
// means that the event can be fragmented but only the first 'x' will be
// scheduled.
//
// Repeat the time event endlessly (until removed)
pub const TE_V2_REPEAT_ENDLESS: c_uint = 0xff;
// If a Time Event has bounded repetitions, this is the maximal value
pub const TE_V2_REPEAT_MAX: c_uint = 0xfe;
pub const TE_V2_PLACEMENT_POS: c_int = 12;
pub const TE_V2_ABSENCE_POS: c_int = 15;
//
// enum iwl_time_event_policy - Time event policy values
// A notification (both event and fragment) includes a status indicating weather
// the FW was able to schedule the event or not. For fragment start/end
// notification the status is always success. There is no start/end fragment
// notification for monolithic events.
//
// @TE_V2_DEFAULT_POLICY: independent, social, present, unoticable
// @TE_V2_NOTIF_HOST_EVENT_START: request/receive notification on event start
// @TE_V2_NOTIF_HOST_EVENT_END:request/receive notification on event end
// @TE_V2_NOTIF_INTERNAL_EVENT_START: internal FW use
// @TE_V2_NOTIF_INTERNAL_EVENT_END: internal FW use.
// @TE_V2_NOTIF_HOST_FRAG_START: request/receive notification on frag start
// @TE_V2_NOTIF_HOST_FRAG_END:request/receive notification on frag end
// @TE_V2_NOTIF_INTERNAL_FRAG_START: internal FW use.
// @TE_V2_NOTIF_INTERNAL_FRAG_END: internal FW use.
// @TE_V2_START_IMMEDIATELY: start time event immediately
// @TE_V2_DEP_OTHER: depends on another time event
// @TE_V2_DEP_TSF: depends on a specific time
// @TE_V2_EVENT_SOCIOPATHIC: can't co-exist with other events of tha same MAC
// @TE_V2_ABSENCE: are we present or absent during the Time Event.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_time_event_policy {
    TE_V2_DEFAULT_POLICY = 0x0,

// notifications (event start/stop, fragment start/stop)
    TE_V2_NOTIF_HOST_EVENT_START = BIT(0),
    TE_V2_NOTIF_HOST_EVENT_END = BIT(1),
    TE_V2_NOTIF_INTERNAL_EVENT_START = BIT(2),
    TE_V2_NOTIF_INTERNAL_EVENT_END = BIT(3),

    TE_V2_NOTIF_HOST_FRAG_START = BIT(4),
    TE_V2_NOTIF_HOST_FRAG_END = BIT(5),
    TE_V2_NOTIF_INTERNAL_FRAG_START = BIT(6),
    TE_V2_NOTIF_INTERNAL_FRAG_END = BIT(7),
    TE_V2_START_IMMEDIATELY = BIT(11),

// placement characteristics
    TE_V2_DEP_OTHER = BIT(TE_V2_PLACEMENT_POS),
    TE_V2_DEP_TSF = BIT(TE_V2_PLACEMENT_POS + 1),
    TE_V2_EVENT_SOCIOPATHIC = BIT(TE_V2_PLACEMENT_POS + 2),

// are we present or absent during the Time Event.
    TE_V2_ABSENCE = BIT(TE_V2_ABSENCE_POS),
}

//
// struct iwl_time_event_cmd - configuring Time Events
// with struct MAC_TIME_EVENT_DATA_API_S_VER_2 (see also
// with version 1. determined by IWL_UCODE_TLV_FLAGS)
// ( TIME_EVENT_CMD = 0x29 )
// @id_and_color: ID and color of the relevant MAC,
// &enum iwl_ctxt_id_and_color
// @action: action to perform, one of &enum iwl_ctxt_action
// @id: this field has two meanings, depending on the action:
// If the action is ADD, then it means the type of event to add.
// For all other actions it is the unique event ID assigned when the
// event was added by the FW.
// @apply_time: When to start the Time Event (in GP2)
// @max_delay: maximum delay to event's start (apply time), in TU
// @depends_on: the unique ID of the event we depend on (if any)
// @interval: interval between repetitions, in TU
// @duration: duration of event in TU
// @repeat: how many repetitions to do, can be TE_REPEAT_ENDLESS
// @max_frags: maximal number of fragments the Time Event can be divided to
// @policy: defines whether uCode shall notify the host or other uCode modules
// on event and/or fragment start and/or end
// using one of TE_INDEPENDENT, TE_DEP_OTHER, TE_DEP_TSF
// TE_EVENT_SOCIOPATHIC
// using TE_ABSENCE and using TE_NOTIF_*,
// &enum iwl_time_event_policy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_event_cmd {
// COMMON_INDEX_HDR_API_S_VER_1
    pub id_and_color: __le32,
    pub action: __le32,
    pub id: __le32,
// MAC_TIME_EVENT_DATA_API_S_VER_2
    pub apply_time: __le32,
    pub max_delay: __le32,
    pub depends_on: __le32,
    pub interval: __le32,
    pub duration: __le32,
    pub repeat: u8,
    pub max_frags: u8,
    pub policy: __le16,
    pub /: *mut *mut } __packed; / MAC_TIME_EVENT_CMD_API_S_VER_2,
//
// struct iwl_time_event_resp - response structure to iwl_time_event_cmd
// @status: bit 0 indicates success, all others specify errors
// @id: the Time Event type
// @unique_id: the unique ID assigned (in ADD) or given (others) to the TE
// @id_and_color: ID and color of the relevant MAC,
// &enum iwl_ctxt_id_and_color
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_event_resp {
    pub status: __le32,
    pub id: __le32,
    pub unique_id: __le32,
    pub id_and_color: __le32,
    pub /: *mut *mut } __packed; / MAC_TIME_EVENT_RSP_API_S_VER_1,
//
// struct iwl_time_event_notif - notifications of time event start/stop
// ( TIME_EVENT_NOTIFICATION = 0x2a )
// @timestamp: action timestamp in GP2
// @session_id: session's unique id
// @unique_id: unique id of the Time Event itself
// @id_and_color: ID and color of the relevant MAC
// @action: &enum iwl_time_event_policy
// @status: true if scheduled, false otherwise (not executed)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_event_notif {
    pub timestamp: __le32,
    pub session_id: __le32,
    pub unique_id: __le32,
    pub id_and_color: __le32,
    pub action: __le32,
    pub status: __le32,
    pub /: *mut *mut } __packed; / MAC_TIME_EVENT_NTFY_API_S_VER_1,
//
// struct iwl_hs20_roc_req_tail - tail of iwl_hs20_roc_req
//
// @node_addr: Our MAC Address
// @reserved: reserved for alignment
// @apply_time: GP2 value to start (should always be the current GP2 value)
// @apply_time_max_delay: Maximum apply time delay value in TU. Defines max
// time by which start of the event is allowed to be postponed.
// @duration: event duration in TU To calculate event duration:
// timeEventDuration = min(duration, remainingQuota)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_hs20_roc_req_tail {
    pub node_addr: [u8; ETH_ALEN],
    pub reserved: __le16,
    pub apply_time: __le32,
    pub apply_time_max_delay: __le32,
    pub duration: __le32,
    pub __packed: },
//
// Aux ROC command
//
// Command requests the firmware to create a time event for a certain duration
// and remain on the given channel. This is done by using the Aux framework in
// the FW.
// The command was first used for Hot Spot issues - but can be used regardless
// to Hot Spot.
//
// ( HOT_SPOT_CMD 0x53 )
//
// @id_and_color: ID and color of the MAC
// @action: action to perform, see &enum iwl_ctxt_action
// @event_unique_id: If the action FW_CTXT_ACTION_REMOVE then the
// event_unique_id should be the id of the time event assigned by ucode.
// Otherwise ignore the event_unique_id.
// @sta_id_and_color: station id and color, resumed during "Remain On Channel"
// activity.
// @channel_info: channel info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_hs20_roc_req {
// COMMON_INDEX_HDR_API_S_VER_1 hdr
    pub id_and_color: __le32,
    pub action: __le32,
    pub event_unique_id: __le32,
    pub sta_id_and_color: __le32,
    pub channel_info: iwl_fw_channel_info,
    pub tail: iwl_hs20_roc_req_tail,
    pub /: *mut *mut } __packed; / HOT_SPOT_CMD_API_S_VER_1,
//
// values for AUX ROC result values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_hot_spot {
    HOT_SPOT_RSP_STATUS_OK,
    HOT_SPOT_RSP_STATUS_TOO_MANY_EVENTS,
    HOT_SPOT_MAX_NUM_OF_SESSIONS,
}

//
// Aux ROC command response
//
// In response to iwl_hs20_roc_req the FW sends this command to notify the
// driver the uid of the timevent.
//
// ( HOT_SPOT_CMD 0x53 )
//
// @event_unique_id: Unique ID of time event assigned by ucode
// @status: Return status 0 is success, all the rest used for specific errors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_hs20_roc_res {
    pub event_unique_id: __le32,
    pub status: __le32,
    pub /: *mut *mut } __packed; / HOT_SPOT_RSP_API_S_VER_1,
//
// Activity types for the ROC command
// @ROC_ACTIVITY_HOTSPOT: ROC for hs20 activity
// @ROC_ACTIVITY_P2P_DISC: ROC for p2p discoverability activity
// @ROC_ACTIVITY_P2P_TXRX: ROC for p2p action frames activity
// @ROC_ACTIVITY_P2P_NEG: ROC for p2p negotiation (used also for TX)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_roc_activity {
    ROC_ACTIVITY_HOTSPOT,
    ROC_ACTIVITY_P2P_DISC,
    ROC_ACTIVITY_P2P_TXRX,
    ROC_ACTIVITY_P2P_NEG,
    ROC_NUM_ACTIVITIES
}

//
// ROC command v5
//
// Command requests the firmware to remain on a channel for a certain duration.
//
// ( MAC_CONF_GROUP 0x3, ROC_CMD 0xE )
//
// @action: action to perform, see &enum iwl_ctxt_action
// @activity: type of activity, see &enum iwl_roc_activity
// @sta_id: station id, resumed during "Remain On Channel" activity.
// @channel_info: &struct iwl_fw_channel_info
// @node_addr: node MAC address for Rx filtering
// @reserved: align to a dword
// @max_delay: max delay the ROC can start in TU
// @duration: remain on channel duration in TU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_roc_req_v5 {
    pub action: __le32,
    pub activity: __le32,
    pub sta_id: __le32,
    pub channel_info: iwl_fw_channel_info,
    pub node_addr: [u8; ETH_ALEN],
    pub reserved: __le16,
    pub max_delay: __le32,
    pub duration: __le32,
    pub /: *mut *mut } __packed; / ROC_CMD_API_S_VER_5,
//
// ROC command
//
// Command requests the firmware to remain on a channel for a certain duration.
//
// ( MAC_CONF_GROUP 0x3, ROC_CMD 0xE )
//
// @action: action to perform, see &enum iwl_ctxt_action
// @activity: type of activity, see &enum iwl_roc_activity
// @sta_id: station id, resumed during "Remain On Channel" activity.
// @channel_info: &struct iwl_fw_channel_info
// @node_addr: node MAC address for Rx filtering
// @reserved1: align to a dword
// @max_delay: max delay the ROC can start in TU
// @duration: remain on channel duration in TU
// @interval: interval between repetitions (when repetitions > 1).
// @repetitions: number of repetitions
// 0xFF: infinite repetitions. 0 or 1: single repetition.
// @reserved2: align to a dword
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_roc_req {
    pub action: __le32,
    pub activity: __le32,
    pub sta_id: __le32,
    pub channel_info: iwl_fw_channel_info,
    pub node_addr: [u8; ETH_ALEN],
    pub reserved1: __le16,
    pub max_delay: __le32,
    pub duration: __le32,
    pub interval: __le32,
    pub repetitions: u8,
    pub reserved2: [u8; 3],
    pub /: *mut *mut } __packed; / ROC_CMD_API_S_VER_6,
//
// ROC notification
//
// Notification when ROC startes and when ROC ended.
//
// ( MAC_CONF_GROUP 0x3, ROC_NOTIF 0xf8 )
//
// @status: true if ROC succeeded to start
// @start_end: true if ROC started, false if ROC ended
// @activity: notification to which activity - &enum iwl_roc_activity
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_roc_notif {
    pub success: __le32,
    pub started: __le32,
    pub activity: __le32,
    pub /: *mut *mut } __packed; / ROC_NOTIF_API_S_VER_1,
//
// enum iwl_session_prot_conf_id - session protection's configurations
// @SESSION_PROTECT_CONF_ASSOC: Start a session protection for association.
// The firmware will allocate two events.
// Valid for BSS_STA and P2P_STA.
// * A rather short event that can't be fragmented and with a very
// high priority. If every goes well (99% of the cases) the
// association should complete within this first event. During
// that event, no other activity will happen in the firmware,
// which is why it can't be too long.
// The length of this event is hard-coded in the firmware: 300TUs.
// * Another event which can be much longer (it's duration is
// configurable by the driver) which has a slightly lower
// priority and that can be fragmented allowing other activities
// to run while this event is running.
// The firmware will automatically remove both events once the driver sets
// the BSS MAC as associated. Neither of the events will be removed
// for the P2P_STA MAC.
// Only the duration is configurable for this protection.
// @SESSION_PROTECT_CONF_GO_CLIENT_ASSOC: not used
// @SESSION_PROTECT_CONF_P2P_DEVICE_DISCOV: Schedule the P2P Device to be in
// listen mode. Will be fragmented. Valid only on the P2P Device MAC.
// Valid only on the P2P Device MAC. The firmware will take into account
// the duration, the interval and the repetition count.
// @SESSION_PROTECT_CONF_P2P_GO_NEGOTIATION: Schedule the P2P Device to be
// able to run the GO Negotiation. Will not be fragmented and not
// repetitive. Valid only on the P2P Device MAC. Only the duration will
// be taken into account.
// @SESSION_PROTECT_CONF_MAX_ID: not used
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_session_prot_conf_id {
    SESSION_PROTECT_CONF_ASSOC,
    SESSION_PROTECT_CONF_GO_CLIENT_ASSOC,
    SESSION_PROTECT_CONF_P2P_DEVICE_DISCOV,
    SESSION_PROTECT_CONF_P2P_GO_NEGOTIATION,
    SESSION_PROTECT_CONF_MAX_ID,
}

//
// struct iwl_session_prot_cmd - configure a session protection
// @id_and_color: the id and color of the link (or mac, for command version 1)
// for which this session protection is sent
// @action: can be either FW_CTXT_ACTION_ADD or FW_CTXT_ACTION_REMOVE,
// see &enum iwl_ctxt_action
// @conf_id: see &enum iwl_session_prot_conf_id
// @duration_tu: the duration of the whole protection in TUs.
// @repetition_count: not used
// @interval: not used
//
// Note: the session protection will always be scheduled to start as
// early as possible, but the maximum delay is configuration dependent.
// The firmware supports only one concurrent session protection per vif.
// Adding a new session protection will remove any currently running session.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_session_prot_cmd {
// COMMON_INDEX_HDR_API_S_VER_1 hdr
    pub id_and_color: __le32,
    pub action: __le32,
    pub conf_id: __le32,
    pub duration_tu: __le32,
    pub repetition_count: __le32,
    pub interval: __le32,
    pub __packed: },
// SESSION_PROTECTION_CMD_API_S_VER_1 and
// SESSION_PROTECTION_CMD_API_S_VER_2
//
// struct iwl_session_prot_notif - session protection started / ended
// @mac_link_id: the mac id (or link id, for notif ver > 2) for which the
// session protection started / ended
// @status: 1 means success, 0 means failure
// @start: 1 means the session protection started, 0 means it ended
// @conf_id: see &enum iwl_session_prot_conf_id
//
// Note that any session protection will always get two notifications: start
// and end even the firmware could not schedule it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_session_prot_notif {
    pub mac_link_id: __le32,
    pub status: __le32,
    pub start: __le32,
    pub conf_id: __le32,
    pub __packed: },
// SESSION_PROTECTION_NOTIFICATION_API_S_VER_2 and
// SESSION_PROTECTION_NOTIFICATION_API_S_VER_3
//
