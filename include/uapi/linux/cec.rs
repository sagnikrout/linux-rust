//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cec.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// cec - HDMI Consumer Electronics Control public header
//
// Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

pub const CEC_MAX_MSG_SIZE: c_int = 16;
//
// struct cec_msg - CEC message structure.
// @tx_ts:	Timestamp in nanoseconds using CLOCK_MONOTONIC. Set by the
// driver when the message transmission has finished.
// @rx_ts:	Timestamp in nanoseconds using CLOCK_MONOTONIC. Set by the
// driver when the message was received.
// @len:	Length in bytes of the message.
// @timeout:	The timeout (in ms) that is used to timeout CEC_RECEIVE.
// Set to 0 if you want to wait forever. This timeout can also be
// used with CEC_TRANSMIT as the timeout for waiting for a reply.
// If 0, then it will use a 1 second timeout instead of waiting
// forever as is done with CEC_RECEIVE.
// @sequence:	The framework assigns a sequence number to messages that are
// sent. This can be used to track replies to previously sent
// messages.
// @flags:	Set to 0.
// @msg:	The message payload.
// @reply:	This field is ignored with CEC_RECEIVE and is only used by
// CEC_TRANSMIT. If non-zero, then wait for a reply with this
// opcode. Set to CEC_MSG_FEATURE_ABORT if you want to wait for
// a possible ABORT reply. If there was an error when sending the
// msg or FeatureAbort was returned, then reply is set to 0.
// If reply is non-zero upon return, then len/msg are set to
// the received message.
// If reply is zero upon return and status has the
// CEC_TX_STATUS_FEATURE_ABORT bit set, then len/msg are set to
// the received feature abort message.
// If reply is zero upon return and status has the
// CEC_TX_STATUS_MAX_RETRIES bit set, then no reply was seen at
// all. If reply is non-zero for CEC_TRANSMIT and the message is a
// broadcast, then -EINVAL is returned.
// if reply is non-zero, then timeout is set to 1000 (the required
// maximum response time).
// @rx_status:	The message receive status bits. Set by the driver.
// @tx_status:	The message transmit status bits. Set by the driver.
// @tx_arb_lost_cnt: The number of 'Arbitration Lost' events. Set by the driver.
// @tx_nack_cnt: The number of 'Not Acknowledged' events. Set by the driver.
// @tx_low_drive_cnt: The number of 'Low Drive Detected' events. Set by the
// driver.
// @tx_error_cnt: The number of 'Error' events. Set by the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_msg {
    pub tx_ts: __u64,
    pub rx_ts: __u64,
    pub len: __u32,
    pub timeout: __u32,
    pub sequence: __u32,
    pub flags: __u32,
    pub msg: [__u8; CEC_MAX_MSG_SIZE],
    pub reply: __u8,
    pub rx_status: __u8,
    pub tx_status: __u8,
    pub tx_arb_lost_cnt: __u8,
    pub tx_nack_cnt: __u8,
    pub tx_low_drive_cnt: __u8,
    pub tx_error_cnt: __u8,
}

//
// cec_msg_initiator - return the initiator's logical address.
// @msg:	the message structure
//
// cec_msg_destination - return the destination's logical address.
// @msg:	the message structure
//
// cec_msg_opcode - return the opcode of the message, -1 for poll
// @msg:	the message structure
//
// cec_msg_is_broadcast - return true if this is a broadcast message.
// @msg:	the message structure
//
// cec_msg_init - initialize the message structure.
// @msg:	the message structure
// @initiator:	the logical address of the initiator
// @destination:the logical address of the destination (0xf for broadcast)
//
// The whole structure is zeroed, the len field is set to 1 (i.e. a poll
// message) and the initiator and destination are filled in.
//
// cec_msg_set_reply_to - fill in destination/initiator in a reply message.
// @msg:	the message structure for the reply
// @orig:	the original message structure
//
// Set the msg destination to the orig initiator and the msg initiator to the
// orig destination. Note that msg and orig may be the same pointer, in which
// case the change is done in place.
//
// It also zeroes the reply, timeout and flags fields.
//
// The destination becomes the initiator and vice versa
//
// cec_msg_recv_is_tx_result - return true if this message contains the
// result of an earlier non-blocking transmit
// @msg:	the message structure from CEC_RECEIVE
//
// cec_msg_recv_is_rx_result - return true if this message contains the
// reply of an earlier non-blocking transmit
// @msg:	the message structure from CEC_RECEIVE
//
// cec_msg flags field

// cec_msg tx/rx_status field

pub const CEC_LOG_ADDR_INVALID: c_uint = 0xff;
pub const CEC_PHYS_ADDR_INVALID: c_uint = 0xffff;
//
// The maximum number of logical addresses one device can be assigned to.
// The CEC 2.0 spec allows for only 2 logical addresses at the moment. The
// Analog Devices CEC hardware supports 3. So let's go wild and go for 4.
//
pub const CEC_MAX_LOG_ADDRS: c_int = 4;
// The logical addresses defined by CEC 2.0
pub const CEC_LOG_ADDR_TV: c_int = 0;
pub const CEC_LOG_ADDR_RECORD_1: c_int = 1;
pub const CEC_LOG_ADDR_RECORD_2: c_int = 2;
pub const CEC_LOG_ADDR_TUNER_1: c_int = 3;
pub const CEC_LOG_ADDR_PLAYBACK_1: c_int = 4;
pub const CEC_LOG_ADDR_AUDIOSYSTEM: c_int = 5;
pub const CEC_LOG_ADDR_TUNER_2: c_int = 6;
pub const CEC_LOG_ADDR_TUNER_3: c_int = 7;
pub const CEC_LOG_ADDR_PLAYBACK_2: c_int = 8;
pub const CEC_LOG_ADDR_RECORD_3: c_int = 9;
pub const CEC_LOG_ADDR_TUNER_4: c_int = 10;
pub const CEC_LOG_ADDR_PLAYBACK_3: c_int = 11;
pub const CEC_LOG_ADDR_BACKUP_1: c_int = 12;
pub const CEC_LOG_ADDR_BACKUP_2: c_int = 13;
pub const CEC_LOG_ADDR_SPECIFIC: c_int = 14;

// The logical address types that the CEC device wants to claim
pub const CEC_LOG_ADDR_TYPE_TV: c_int = 0;
pub const CEC_LOG_ADDR_TYPE_RECORD: c_int = 1;
pub const CEC_LOG_ADDR_TYPE_TUNER: c_int = 2;
pub const CEC_LOG_ADDR_TYPE_PLAYBACK: c_int = 3;
pub const CEC_LOG_ADDR_TYPE_AUDIOSYSTEM: c_int = 4;
pub const CEC_LOG_ADDR_TYPE_SPECIFIC: c_int = 5;
pub const CEC_LOG_ADDR_TYPE_UNREGISTERED: c_int = 6;
//
// Switches should use UNREGISTERED.
// Processors should use SPECIFIC.
//

//
// Use this if there is no vendor ID (CEC_G_VENDOR_ID) or if the vendor ID
// should be disabled (CEC_S_VENDOR_ID)
//
pub const CEC_VENDOR_ID_NONE: c_uint = 0xffffffff;
// The message handling modes
// Modes for initiator

pub const CEC_MODE_INITIATOR_MSK: c_uint = 0x0f;
// Modes for follower

pub const CEC_MODE_FOLLOWER_MSK: c_uint = 0xf0;
// Userspace has to configure the physical address

// Userspace has to configure the logical addresses

// Userspace can transmit messages (and thus become follower as well)

//
// Passthrough all messages instead of processing them.
//

// Supports remote control

// Hardware can monitor all messages, not just directed and broadcast.

// Hardware can use CEC only if the HDMI HPD pin is high.

// Hardware can monitor CEC pin transitions

// CEC_ADAP_G_CONNECTOR_INFO is available

// CEC_MSG_FL_REPLY_VENDOR_ID is available

//
// struct cec_caps - CEC capabilities structure.
// @driver: name of the CEC device driver.
// @name: name of the CEC device. @driver + @name must be unique.
// @available_log_addrs: number of available logical addresses.
// @capabilities: capabilities of the CEC adapter.
// @version: version of the CEC adapter framework.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_caps {
    pub driver: [c_char; 32],
    pub name: [c_char; 32],
    pub available_log_addrs: __u32,
    pub capabilities: __u32,
    pub version: __u32,
}

//
// struct cec_log_addrs - CEC logical addresses structure.
// @log_addr: the claimed logical addresses. Set by the driver.
// @log_addr_mask: current logical address mask. Set by the driver.
// @cec_version: the CEC version that the adapter should implement. Set by the
// caller.
// @num_log_addrs: how many logical addresses should be claimed. Set by the
// caller.
// @vendor_id: the vendor ID of the device. Set by the caller.
// @flags: flags.
// @osd_name: the OSD name of the device. Set by the caller.
// @primary_device_type: the primary device type for each logical address.
// Set by the caller.
// @log_addr_type: the logical address types. Set by the caller.
// @all_device_types: CEC 2.0: all device types represented by the logical
// address. Set by the caller.
// @features:	CEC 2.0: The logical address features. Set by the caller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_log_addrs {
    pub log_addr: [__u8; CEC_MAX_LOG_ADDRS],
    pub log_addr_mask: __u16,
    pub cec_version: __u8,
    pub num_log_addrs: __u8,
    pub vendor_id: __u32,
    pub flags: __u32,
    pub osd_name: [c_char; 15],
    pub primary_device_type: [__u8; CEC_MAX_LOG_ADDRS],
    pub log_addr_type: [__u8; CEC_MAX_LOG_ADDRS],
// CEC 2.0
    pub all_device_types: [__u8; CEC_MAX_LOG_ADDRS],
    pub features: [__u8; CEC_MAX_LOG_ADDRS][12],
}

// Allow a fallback to unregistered

// Passthrough RC messages to the input subsystem

// CDC-Only device: supports only CDC messages

// Configuration failed

//
// struct cec_drm_connector_info - tells which drm connector is
// associated with the CEC adapter.
// @card_no: drm card number
// @connector_id: drm connector ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_drm_connector_info {
    pub card_no: __u32,
    pub connector_id: __u32,
}

pub const CEC_CONNECTOR_TYPE_NO_CONNECTOR: c_int = 0;
pub const CEC_CONNECTOR_TYPE_DRM: c_int = 1;
//
// struct cec_connector_info - tells if and which connector is
// associated with the CEC adapter.
// @type: connector type (if any)
// @drm: drm connector info
// @raw: array to pad the union
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_connector_info {
    pub type: __u32,
    pub drm: cec_drm_connector_info,
    pub raw: [__u32; 16],
}

// Events
// Event that occurs when the adapter state changes
pub const CEC_EVENT_STATE_CHANGE: c_int = 1;
//
// This event is sent when messages are lost because the application
// didn't empty the message queue in time
//
pub const CEC_EVENT_LOST_MSGS: c_int = 2;
pub const CEC_EVENT_PIN_CEC_LOW: c_int = 3;
pub const CEC_EVENT_PIN_CEC_HIGH: c_int = 4;
pub const CEC_EVENT_PIN_HPD_LOW: c_int = 5;
pub const CEC_EVENT_PIN_HPD_HIGH: c_int = 6;
pub const CEC_EVENT_PIN_5V_LOW: c_int = 7;
pub const CEC_EVENT_PIN_5V_HIGH: c_int = 8;

//
// struct cec_event_state_change - used when the CEC adapter changes state.
// @phys_addr: the current physical address
// @log_addr_mask: the current logical address mask
// @have_conn_info: if non-zero, then HDMI connector information is available.
// This field is only valid if CEC_CAP_CONNECTOR_INFO is set. If that
// capability is set and @have_conn_info is zero, then that indicates
// that the HDMI connector device is not instantiated, either because
// the HDMI driver is still configuring the device or because the HDMI
// device was unbound.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_event_state_change {
    pub phys_addr: __u16,
    pub log_addr_mask: __u16,
    pub have_conn_info: __u16,
}

//
// struct cec_event_lost_msgs - tells you how many messages were lost.
// @lost_msgs: how many messages were lost.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_event_lost_msgs {
    pub lost_msgs: __u32,
}

//
// struct cec_event - CEC event structure
// @ts: the timestamp of when the event was sent.
// @event: the event.
// @flags: event flags.
// @state_change: the event payload for CEC_EVENT_STATE_CHANGE.
// @lost_msgs: the event payload for CEC_EVENT_LOST_MSGS.
// @raw: array to pad the union.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_event {
    pub ts: __u64,
    pub event: __u32,
    pub flags: __u32,
    pub state_change: cec_event_state_change,
    pub lost_msgs: cec_event_lost_msgs,
    pub raw: [__u32; 16],
}

// ioctls
// Adapter capabilities

//
// phys_addr is either 0 (if this is the CEC root device)
// or a valid physical address obtained from the sink's EDID
// as read by this CEC device (if this is a source device)
// or a physical address obtained and modified from a sink
// EDID and used for a sink CEC device.
// If nothing is connected, then phys_addr is 0xffff.
// See HDMI 1.4b, section 8.7 (Physical Address).
//
// The CEC_ADAP_S_PHYS_ADDR ioctl may not be available if that is handled
// internally.
//

//
// Configure the CEC adapter. It sets the device type and which
// logical types it will try to claim. It will return which
// logical addresses it could actually claim.
// An error is returned if the adapter is disabled or if there
// is no physical address assigned.
//

// Transmit/receive a CEC command

// Dequeue CEC events

//
// Get and set the message handling mode for this filehandle.
//

// Get the connector info

//
// The remainder of this header defines all CEC messages and operands.
// The format matters since it the cec-ctl utility parses it to generate
// code for implementing all these messages.
//
// Comments ending with 'Feature' group messages for each feature.
// If messages are part of multiple features, then the "Has also"
// comment is used to list the previously defined messages that are
// supported by the feature.
//
// Before operands are defined a comment is added that gives the
// name of the operand and in brackets the variable name of the
// corresponding argument in the cec-funcs.h function.
//
// Messages
// One Touch Play Feature
pub const CEC_MSG_ACTIVE_SOURCE: c_uint = 0x82;
pub const CEC_MSG_IMAGE_VIEW_ON: c_uint = 0x04;
pub const CEC_MSG_TEXT_VIEW_ON: c_uint = 0x0d;
// Routing Control Feature
//
// Has also:
// CEC_MSG_ACTIVE_SOURCE
//
pub const CEC_MSG_INACTIVE_SOURCE: c_uint = 0x9d;
pub const CEC_MSG_REQUEST_ACTIVE_SOURCE: c_uint = 0x85;
pub const CEC_MSG_ROUTING_CHANGE: c_uint = 0x80;
pub const CEC_MSG_ROUTING_INFORMATION: c_uint = 0x81;
pub const CEC_MSG_SET_STREAM_PATH: c_uint = 0x86;
// Standby Feature
pub const CEC_MSG_STANDBY: c_uint = 0x36;
// One Touch Record Feature
pub const CEC_MSG_RECORD_OFF: c_uint = 0x0b;
pub const CEC_MSG_RECORD_ON: c_uint = 0x09;
// Record Source Type Operand (rec_src_type)
pub const CEC_OP_RECORD_SRC_OWN: c_int = 1;
pub const CEC_OP_RECORD_SRC_DIGITAL: c_int = 2;
pub const CEC_OP_RECORD_SRC_ANALOG: c_int = 3;
pub const CEC_OP_RECORD_SRC_EXT_PLUG: c_int = 4;
pub const CEC_OP_RECORD_SRC_EXT_PHYS_ADDR: c_int = 5;
// Service Identification Method Operand (service_id_method)
pub const CEC_OP_SERVICE_ID_METHOD_BY_DIG_ID: c_int = 0;
pub const CEC_OP_SERVICE_ID_METHOD_BY_CHANNEL: c_int = 1;
// Digital Service Broadcast System Operand (dig_bcast_system)
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_GEN: c_uint = 0x00;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_GEN: c_uint = 0x01;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_GEN: c_uint = 0x02;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_BS: c_uint = 0x08;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_CS: c_uint = 0x09;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ARIB_T: c_uint = 0x0a;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_CABLE: c_uint = 0x10;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_SAT: c_uint = 0x11;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_ATSC_T: c_uint = 0x12;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_C: c_uint = 0x18;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_S: c_uint = 0x19;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_S2: c_uint = 0x1a;
pub const CEC_OP_DIG_SERVICE_BCAST_SYSTEM_DVB_T: c_uint = 0x1b;
// Analogue Broadcast Type Operand (ana_bcast_type)
pub const CEC_OP_ANA_BCAST_TYPE_CABLE: c_int = 0;
pub const CEC_OP_ANA_BCAST_TYPE_SATELLITE: c_int = 1;
pub const CEC_OP_ANA_BCAST_TYPE_TERRESTRIAL: c_int = 2;
// Broadcast System Operand (bcast_system)
pub const CEC_OP_BCAST_SYSTEM_PAL_BG: c_uint = 0x00;
pub const CEC_OP_BCAST_SYSTEM_SECAM_LQ: c_uint = 0x01 /* SECAM L' */;
pub const CEC_OP_BCAST_SYSTEM_PAL_M: c_uint = 0x02;
pub const CEC_OP_BCAST_SYSTEM_NTSC_M: c_uint = 0x03;
pub const CEC_OP_BCAST_SYSTEM_PAL_I: c_uint = 0x04;
pub const CEC_OP_BCAST_SYSTEM_SECAM_DK: c_uint = 0x05;
pub const CEC_OP_BCAST_SYSTEM_SECAM_BG: c_uint = 0x06;
pub const CEC_OP_BCAST_SYSTEM_SECAM_L: c_uint = 0x07;
pub const CEC_OP_BCAST_SYSTEM_PAL_DK: c_uint = 0x08;
pub const CEC_OP_BCAST_SYSTEM_OTHER: c_uint = 0x1f;
// Channel Number Format Operand (channel_number_fmt)
pub const CEC_OP_CHANNEL_NUMBER_FMT_1_PART: c_uint = 0x01;
pub const CEC_OP_CHANNEL_NUMBER_FMT_2_PART: c_uint = 0x02;
pub const CEC_MSG_RECORD_STATUS: c_uint = 0x0a;
// Record Status Operand (rec_status)
pub const CEC_OP_RECORD_STATUS_CUR_SRC: c_uint = 0x01;
pub const CEC_OP_RECORD_STATUS_DIG_SERVICE: c_uint = 0x02;
pub const CEC_OP_RECORD_STATUS_ANA_SERVICE: c_uint = 0x03;
pub const CEC_OP_RECORD_STATUS_EXT_INPUT: c_uint = 0x04;
pub const CEC_OP_RECORD_STATUS_NO_DIG_SERVICE: c_uint = 0x05;
pub const CEC_OP_RECORD_STATUS_NO_ANA_SERVICE: c_uint = 0x06;
pub const CEC_OP_RECORD_STATUS_NO_SERVICE: c_uint = 0x07;
pub const CEC_OP_RECORD_STATUS_INVALID_EXT_PLUG: c_uint = 0x09;
pub const CEC_OP_RECORD_STATUS_INVALID_EXT_PHYS_ADDR: c_uint = 0x0a;
pub const CEC_OP_RECORD_STATUS_UNSUP_CA: c_uint = 0x0b;
pub const CEC_OP_RECORD_STATUS_NO_CA_ENTITLEMENTS: c_uint = 0x0c;
pub const CEC_OP_RECORD_STATUS_CANT_COPY_SRC: c_uint = 0x0d;
pub const CEC_OP_RECORD_STATUS_NO_MORE_COPIES: c_uint = 0x0e;
pub const CEC_OP_RECORD_STATUS_NO_MEDIA: c_uint = 0x10;
pub const CEC_OP_RECORD_STATUS_PLAYING: c_uint = 0x11;
pub const CEC_OP_RECORD_STATUS_ALREADY_RECORDING: c_uint = 0x12;
pub const CEC_OP_RECORD_STATUS_MEDIA_PROT: c_uint = 0x13;
pub const CEC_OP_RECORD_STATUS_NO_SIGNAL: c_uint = 0x14;
pub const CEC_OP_RECORD_STATUS_MEDIA_PROBLEM: c_uint = 0x15;
pub const CEC_OP_RECORD_STATUS_NO_SPACE: c_uint = 0x16;
pub const CEC_OP_RECORD_STATUS_PARENTAL_LOCK: c_uint = 0x17;
pub const CEC_OP_RECORD_STATUS_TERMINATED_OK: c_uint = 0x1a;
pub const CEC_OP_RECORD_STATUS_ALREADY_TERM: c_uint = 0x1b;
pub const CEC_OP_RECORD_STATUS_OTHER: c_uint = 0x1f;
pub const CEC_MSG_RECORD_TV_SCREEN: c_uint = 0x0f;
// Timer Programming Feature
pub const CEC_MSG_CLEAR_ANALOGUE_TIMER: c_uint = 0x33;
// Recording Sequence Operand (recording_seq)
pub const CEC_OP_REC_SEQ_SUNDAY: c_uint = 0x01;
pub const CEC_OP_REC_SEQ_MONDAY: c_uint = 0x02;
pub const CEC_OP_REC_SEQ_TUESDAY: c_uint = 0x04;
pub const CEC_OP_REC_SEQ_WEDNESDAY: c_uint = 0x08;
pub const CEC_OP_REC_SEQ_THURSDAY: c_uint = 0x10;
pub const CEC_OP_REC_SEQ_FRIDAY: c_uint = 0x20;
pub const CEC_OP_REC_SEQ_SATURDAY: c_uint = 0x40;
pub const CEC_OP_REC_SEQ_ONCE_ONLY: c_uint = 0x00;
pub const CEC_MSG_CLEAR_DIGITAL_TIMER: c_uint = 0x99;
pub const CEC_MSG_CLEAR_EXT_TIMER: c_uint = 0xa1;
// External Source Specifier Operand (ext_src_spec)
pub const CEC_OP_EXT_SRC_PLUG: c_uint = 0x04;
pub const CEC_OP_EXT_SRC_PHYS_ADDR: c_uint = 0x05;
pub const CEC_MSG_SET_ANALOGUE_TIMER: c_uint = 0x34;
pub const CEC_MSG_SET_DIGITAL_TIMER: c_uint = 0x97;
pub const CEC_MSG_SET_EXT_TIMER: c_uint = 0xa2;
pub const CEC_MSG_SET_TIMER_PROGRAM_TITLE: c_uint = 0x67;
pub const CEC_MSG_TIMER_CLEARED_STATUS: c_uint = 0x43;
// Timer Cleared Status Data Operand (timer_cleared_status)
pub const CEC_OP_TIMER_CLR_STAT_RECORDING: c_uint = 0x00;
pub const CEC_OP_TIMER_CLR_STAT_NO_MATCHING: c_uint = 0x01;
pub const CEC_OP_TIMER_CLR_STAT_NO_INFO: c_uint = 0x02;
pub const CEC_OP_TIMER_CLR_STAT_CLEARED: c_uint = 0x80;
pub const CEC_MSG_TIMER_STATUS: c_uint = 0x35;
// Timer Overlap Warning Operand (timer_overlap_warning)
pub const CEC_OP_TIMER_OVERLAP_WARNING_NO_OVERLAP: c_int = 0;
pub const CEC_OP_TIMER_OVERLAP_WARNING_OVERLAP: c_int = 1;
// Media Info Operand (media_info)
pub const CEC_OP_MEDIA_INFO_UNPROT_MEDIA: c_int = 0;
pub const CEC_OP_MEDIA_INFO_PROT_MEDIA: c_int = 1;
pub const CEC_OP_MEDIA_INFO_NO_MEDIA: c_int = 2;
// Programmed Indicator Operand (prog_indicator)
pub const CEC_OP_PROG_IND_NOT_PROGRAMMED: c_int = 0;
pub const CEC_OP_PROG_IND_PROGRAMMED: c_int = 1;
// Programmed Info Operand (prog_info)
pub const CEC_OP_PROG_INFO_ENOUGH_SPACE: c_uint = 0x08;
pub const CEC_OP_PROG_INFO_NOT_ENOUGH_SPACE: c_uint = 0x09;
pub const CEC_OP_PROG_INFO_MIGHT_NOT_BE_ENOUGH_SPACE: c_uint = 0x0b;
pub const CEC_OP_PROG_INFO_NONE_AVAILABLE: c_uint = 0x0a;
// Not Programmed Error Info Operand (prog_error)
pub const CEC_OP_PROG_ERROR_NO_FREE_TIMER: c_uint = 0x01;
pub const CEC_OP_PROG_ERROR_DATE_OUT_OF_RANGE: c_uint = 0x02;
pub const CEC_OP_PROG_ERROR_REC_SEQ_ERROR: c_uint = 0x03;
pub const CEC_OP_PROG_ERROR_INV_EXT_PLUG: c_uint = 0x04;
pub const CEC_OP_PROG_ERROR_INV_EXT_PHYS_ADDR: c_uint = 0x05;
pub const CEC_OP_PROG_ERROR_CA_UNSUPP: c_uint = 0x06;
pub const CEC_OP_PROG_ERROR_INSUF_CA_ENTITLEMENTS: c_uint = 0x07;
pub const CEC_OP_PROG_ERROR_RESOLUTION_UNSUPP: c_uint = 0x08;
pub const CEC_OP_PROG_ERROR_PARENTAL_LOCK: c_uint = 0x09;
pub const CEC_OP_PROG_ERROR_CLOCK_FAILURE: c_uint = 0x0a;
pub const CEC_OP_PROG_ERROR_DUPLICATE: c_uint = 0x0e;
// System Information Feature
pub const CEC_MSG_CEC_VERSION: c_uint = 0x9e;
// CEC Version Operand (cec_version)
pub const CEC_OP_CEC_VERSION_1_3A: c_int = 4;
pub const CEC_OP_CEC_VERSION_1_4: c_int = 5;
pub const CEC_OP_CEC_VERSION_2_0: c_int = 6;
pub const CEC_MSG_GET_CEC_VERSION: c_uint = 0x9f;
pub const CEC_MSG_GIVE_PHYSICAL_ADDR: c_uint = 0x83;
pub const CEC_MSG_GET_MENU_LANGUAGE: c_uint = 0x91;
pub const CEC_MSG_REPORT_PHYSICAL_ADDR: c_uint = 0x84;
// Primary Device Type Operand (prim_devtype)
pub const CEC_OP_PRIM_DEVTYPE_TV: c_int = 0;
pub const CEC_OP_PRIM_DEVTYPE_RECORD: c_int = 1;
pub const CEC_OP_PRIM_DEVTYPE_TUNER: c_int = 3;
pub const CEC_OP_PRIM_DEVTYPE_PLAYBACK: c_int = 4;
pub const CEC_OP_PRIM_DEVTYPE_AUDIOSYSTEM: c_int = 5;
pub const CEC_OP_PRIM_DEVTYPE_SWITCH: c_int = 6;
pub const CEC_OP_PRIM_DEVTYPE_PROCESSOR: c_int = 7;
pub const CEC_MSG_SET_MENU_LANGUAGE: c_uint = 0x32;
pub const CEC_MSG_REPORT_FEATURES: c_uint = 0xa6	/* CEC 2.0 */;
// All Device Types Operand (all_device_types)
pub const CEC_OP_ALL_DEVTYPE_TV: c_uint = 0x80;
pub const CEC_OP_ALL_DEVTYPE_RECORD: c_uint = 0x40;
pub const CEC_OP_ALL_DEVTYPE_TUNER: c_uint = 0x20;
pub const CEC_OP_ALL_DEVTYPE_PLAYBACK: c_uint = 0x10;
pub const CEC_OP_ALL_DEVTYPE_AUDIOSYSTEM: c_uint = 0x08;
pub const CEC_OP_ALL_DEVTYPE_SWITCH: c_uint = 0x04;
//
// And if you wondering what happened to PROCESSOR devices: those should
// be mapped to a SWITCH.
//
// Valid for RC Profile and Device Feature operands
pub const CEC_OP_FEAT_EXT: c_uint = 0x80	/* Extension bit */;
// RC Profile Operand (rc_profile)
pub const CEC_OP_FEAT_RC_TV_PROFILE_NONE: c_uint = 0x00;
pub const CEC_OP_FEAT_RC_TV_PROFILE_1: c_uint = 0x02;
pub const CEC_OP_FEAT_RC_TV_PROFILE_2: c_uint = 0x06;
pub const CEC_OP_FEAT_RC_TV_PROFILE_3: c_uint = 0x0a;
pub const CEC_OP_FEAT_RC_TV_PROFILE_4: c_uint = 0x0e;
pub const CEC_OP_FEAT_RC_SRC_HAS_DEV_ROOT_MENU: c_uint = 0x50;
pub const CEC_OP_FEAT_RC_SRC_HAS_DEV_SETUP_MENU: c_uint = 0x48;
pub const CEC_OP_FEAT_RC_SRC_HAS_CONTENTS_MENU: c_uint = 0x44;
pub const CEC_OP_FEAT_RC_SRC_HAS_MEDIA_TOP_MENU: c_uint = 0x42;
pub const CEC_OP_FEAT_RC_SRC_HAS_MEDIA_CONTEXT_MENU: c_uint = 0x41;
// Device Feature Operand (dev_features)
pub const CEC_OP_FEAT_DEV_HAS_RECORD_TV_SCREEN: c_uint = 0x40;
pub const CEC_OP_FEAT_DEV_HAS_SET_OSD_STRING: c_uint = 0x20;
pub const CEC_OP_FEAT_DEV_HAS_DECK_CONTROL: c_uint = 0x10;
pub const CEC_OP_FEAT_DEV_HAS_SET_AUDIO_RATE: c_uint = 0x08;
pub const CEC_OP_FEAT_DEV_SINK_HAS_ARC_TX: c_uint = 0x04;
pub const CEC_OP_FEAT_DEV_SOURCE_HAS_ARC_RX: c_uint = 0x02;
pub const CEC_OP_FEAT_DEV_HAS_SET_AUDIO_VOLUME_LEVEL: c_uint = 0x01;
pub const CEC_MSG_GIVE_FEATURES: c_uint = 0xa5	/* CEC 2.0 */;
// Deck Control Feature
pub const CEC_MSG_DECK_CONTROL: c_uint = 0x42;
// Deck Control Mode Operand (deck_control_mode)
pub const CEC_OP_DECK_CTL_MODE_SKIP_FWD: c_int = 1;
pub const CEC_OP_DECK_CTL_MODE_SKIP_REV: c_int = 2;
pub const CEC_OP_DECK_CTL_MODE_STOP: c_int = 3;
pub const CEC_OP_DECK_CTL_MODE_EJECT: c_int = 4;
pub const CEC_MSG_DECK_STATUS: c_uint = 0x1b;
// Deck Info Operand (deck_info)
pub const CEC_OP_DECK_INFO_PLAY: c_uint = 0x11;
pub const CEC_OP_DECK_INFO_RECORD: c_uint = 0x12;
pub const CEC_OP_DECK_INFO_PLAY_REV: c_uint = 0x13;
pub const CEC_OP_DECK_INFO_STILL: c_uint = 0x14;
pub const CEC_OP_DECK_INFO_SLOW: c_uint = 0x15;
pub const CEC_OP_DECK_INFO_SLOW_REV: c_uint = 0x16;
pub const CEC_OP_DECK_INFO_FAST_FWD: c_uint = 0x17;
pub const CEC_OP_DECK_INFO_FAST_REV: c_uint = 0x18;
pub const CEC_OP_DECK_INFO_NO_MEDIA: c_uint = 0x19;
pub const CEC_OP_DECK_INFO_STOP: c_uint = 0x1a;
pub const CEC_OP_DECK_INFO_SKIP_FWD: c_uint = 0x1b;
pub const CEC_OP_DECK_INFO_SKIP_REV: c_uint = 0x1c;
pub const CEC_OP_DECK_INFO_INDEX_SEARCH_FWD: c_uint = 0x1d;
pub const CEC_OP_DECK_INFO_INDEX_SEARCH_REV: c_uint = 0x1e;
pub const CEC_OP_DECK_INFO_OTHER: c_uint = 0x1f;
pub const CEC_MSG_GIVE_DECK_STATUS: c_uint = 0x1a;
// Status Request Operand (status_req)
pub const CEC_OP_STATUS_REQ_ON: c_int = 1;
pub const CEC_OP_STATUS_REQ_OFF: c_int = 2;
pub const CEC_OP_STATUS_REQ_ONCE: c_int = 3;
pub const CEC_MSG_PLAY: c_uint = 0x41;
// Play Mode Operand (play_mode)
pub const CEC_OP_PLAY_MODE_PLAY_FWD: c_uint = 0x24;
pub const CEC_OP_PLAY_MODE_PLAY_REV: c_uint = 0x20;
pub const CEC_OP_PLAY_MODE_PLAY_STILL: c_uint = 0x25;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_FWD_MIN: c_uint = 0x05;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_FWD_MED: c_uint = 0x06;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_FWD_MAX: c_uint = 0x07;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_REV_MIN: c_uint = 0x09;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_REV_MED: c_uint = 0x0a;
pub const CEC_OP_PLAY_MODE_PLAY_FAST_REV_MAX: c_uint = 0x0b;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_FWD_MIN: c_uint = 0x15;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_FWD_MED: c_uint = 0x16;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_FWD_MAX: c_uint = 0x17;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_REV_MIN: c_uint = 0x19;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_REV_MED: c_uint = 0x1a;
pub const CEC_OP_PLAY_MODE_PLAY_SLOW_REV_MAX: c_uint = 0x1b;
// Tuner Control Feature
pub const CEC_MSG_GIVE_TUNER_DEVICE_STATUS: c_uint = 0x08;
pub const CEC_MSG_SELECT_ANALOGUE_SERVICE: c_uint = 0x92;
pub const CEC_MSG_SELECT_DIGITAL_SERVICE: c_uint = 0x93;
pub const CEC_MSG_TUNER_DEVICE_STATUS: c_uint = 0x07;
// Recording Flag Operand (rec_flag)
pub const CEC_OP_REC_FLAG_NOT_USED: c_int = 0;
pub const CEC_OP_REC_FLAG_USED: c_int = 1;
// Tuner Display Info Operand (tuner_display_info)
pub const CEC_OP_TUNER_DISPLAY_INFO_DIGITAL: c_int = 0;
pub const CEC_OP_TUNER_DISPLAY_INFO_NONE: c_int = 1;
pub const CEC_OP_TUNER_DISPLAY_INFO_ANALOGUE: c_int = 2;
pub const CEC_MSG_TUNER_STEP_DECREMENT: c_uint = 0x06;
pub const CEC_MSG_TUNER_STEP_INCREMENT: c_uint = 0x05;
// Vendor Specific Commands Feature
//
// Has also:
// CEC_MSG_CEC_VERSION
// CEC_MSG_GET_CEC_VERSION
//
pub const CEC_MSG_DEVICE_VENDOR_ID: c_uint = 0x87;
pub const CEC_MSG_GIVE_DEVICE_VENDOR_ID: c_uint = 0x8c;
pub const CEC_MSG_VENDOR_COMMAND: c_uint = 0x89;
pub const CEC_MSG_VENDOR_COMMAND_WITH_ID: c_uint = 0xa0;
pub const CEC_MSG_VENDOR_REMOTE_BUTTON_DOWN: c_uint = 0x8a;
pub const CEC_MSG_VENDOR_REMOTE_BUTTON_UP: c_uint = 0x8b;
// OSD Display Feature
pub const CEC_MSG_SET_OSD_STRING: c_uint = 0x64;
// Display Control Operand (disp_ctl)
pub const CEC_OP_DISP_CTL_DEFAULT: c_uint = 0x00;
pub const CEC_OP_DISP_CTL_UNTIL_CLEARED: c_uint = 0x40;
pub const CEC_OP_DISP_CTL_CLEAR: c_uint = 0x80;
// Device OSD Transfer Feature
pub const CEC_MSG_GIVE_OSD_NAME: c_uint = 0x46;
pub const CEC_MSG_SET_OSD_NAME: c_uint = 0x47;
// Device Menu Control Feature
pub const CEC_MSG_MENU_REQUEST: c_uint = 0x8d;
// Menu Request Type Operand (menu_req)
pub const CEC_OP_MENU_REQUEST_ACTIVATE: c_uint = 0x00;
pub const CEC_OP_MENU_REQUEST_DEACTIVATE: c_uint = 0x01;
pub const CEC_OP_MENU_REQUEST_QUERY: c_uint = 0x02;
pub const CEC_MSG_MENU_STATUS: c_uint = 0x8e;
// Menu State Operand (menu_state)
pub const CEC_OP_MENU_STATE_ACTIVATED: c_uint = 0x00;
pub const CEC_OP_MENU_STATE_DEACTIVATED: c_uint = 0x01;
pub const CEC_MSG_USER_CONTROL_PRESSED: c_uint = 0x44;
// UI Command Operand (ui_cmd)
pub const CEC_OP_UI_CMD_SELECT: c_uint = 0x00;
pub const CEC_OP_UI_CMD_UP: c_uint = 0x01;
pub const CEC_OP_UI_CMD_DOWN: c_uint = 0x02;
pub const CEC_OP_UI_CMD_LEFT: c_uint = 0x03;
pub const CEC_OP_UI_CMD_RIGHT: c_uint = 0x04;
pub const CEC_OP_UI_CMD_RIGHT_UP: c_uint = 0x05;
pub const CEC_OP_UI_CMD_RIGHT_DOWN: c_uint = 0x06;
pub const CEC_OP_UI_CMD_LEFT_UP: c_uint = 0x07;
pub const CEC_OP_UI_CMD_LEFT_DOWN: c_uint = 0x08;
pub const CEC_OP_UI_CMD_DEVICE_ROOT_MENU: c_uint = 0x09;
pub const CEC_OP_UI_CMD_DEVICE_SETUP_MENU: c_uint = 0x0a;
pub const CEC_OP_UI_CMD_CONTENTS_MENU: c_uint = 0x0b;
pub const CEC_OP_UI_CMD_FAVORITE_MENU: c_uint = 0x0c;
pub const CEC_OP_UI_CMD_BACK: c_uint = 0x0d;
pub const CEC_OP_UI_CMD_MEDIA_TOP_MENU: c_uint = 0x10;
pub const CEC_OP_UI_CMD_MEDIA_CONTEXT_SENSITIVE_MENU: c_uint = 0x11;
pub const CEC_OP_UI_CMD_NUMBER_ENTRY_MODE: c_uint = 0x1d;
pub const CEC_OP_UI_CMD_NUMBER_11: c_uint = 0x1e;
pub const CEC_OP_UI_CMD_NUMBER_12: c_uint = 0x1f;
pub const CEC_OP_UI_CMD_NUMBER_0_OR_NUMBER_10: c_uint = 0x20;
pub const CEC_OP_UI_CMD_NUMBER_1: c_uint = 0x21;
pub const CEC_OP_UI_CMD_NUMBER_2: c_uint = 0x22;
pub const CEC_OP_UI_CMD_NUMBER_3: c_uint = 0x23;
pub const CEC_OP_UI_CMD_NUMBER_4: c_uint = 0x24;
pub const CEC_OP_UI_CMD_NUMBER_5: c_uint = 0x25;
pub const CEC_OP_UI_CMD_NUMBER_6: c_uint = 0x26;
pub const CEC_OP_UI_CMD_NUMBER_7: c_uint = 0x27;
pub const CEC_OP_UI_CMD_NUMBER_8: c_uint = 0x28;
pub const CEC_OP_UI_CMD_NUMBER_9: c_uint = 0x29;
pub const CEC_OP_UI_CMD_DOT: c_uint = 0x2a;
pub const CEC_OP_UI_CMD_ENTER: c_uint = 0x2b;
pub const CEC_OP_UI_CMD_CLEAR: c_uint = 0x2c;
pub const CEC_OP_UI_CMD_NEXT_FAVORITE: c_uint = 0x2f;
pub const CEC_OP_UI_CMD_CHANNEL_UP: c_uint = 0x30;
pub const CEC_OP_UI_CMD_CHANNEL_DOWN: c_uint = 0x31;
pub const CEC_OP_UI_CMD_PREVIOUS_CHANNEL: c_uint = 0x32;
pub const CEC_OP_UI_CMD_SOUND_SELECT: c_uint = 0x33;
pub const CEC_OP_UI_CMD_INPUT_SELECT: c_uint = 0x34;
pub const CEC_OP_UI_CMD_DISPLAY_INFORMATION: c_uint = 0x35;
pub const CEC_OP_UI_CMD_HELP: c_uint = 0x36;
pub const CEC_OP_UI_CMD_PAGE_UP: c_uint = 0x37;
pub const CEC_OP_UI_CMD_PAGE_DOWN: c_uint = 0x38;
pub const CEC_OP_UI_CMD_POWER: c_uint = 0x40;
pub const CEC_OP_UI_CMD_VOLUME_UP: c_uint = 0x41;
pub const CEC_OP_UI_CMD_VOLUME_DOWN: c_uint = 0x42;
pub const CEC_OP_UI_CMD_MUTE: c_uint = 0x43;
pub const CEC_OP_UI_CMD_PLAY: c_uint = 0x44;
pub const CEC_OP_UI_CMD_STOP: c_uint = 0x45;
pub const CEC_OP_UI_CMD_PAUSE: c_uint = 0x46;
pub const CEC_OP_UI_CMD_RECORD: c_uint = 0x47;
pub const CEC_OP_UI_CMD_REWIND: c_uint = 0x48;
pub const CEC_OP_UI_CMD_FAST_FORWARD: c_uint = 0x49;
pub const CEC_OP_UI_CMD_EJECT: c_uint = 0x4a;
pub const CEC_OP_UI_CMD_SKIP_FORWARD: c_uint = 0x4b;
pub const CEC_OP_UI_CMD_SKIP_BACKWARD: c_uint = 0x4c;
pub const CEC_OP_UI_CMD_STOP_RECORD: c_uint = 0x4d;
pub const CEC_OP_UI_CMD_PAUSE_RECORD: c_uint = 0x4e;
pub const CEC_OP_UI_CMD_ANGLE: c_uint = 0x50;
pub const CEC_OP_UI_CMD_SUB_PICTURE: c_uint = 0x51;
pub const CEC_OP_UI_CMD_VIDEO_ON_DEMAND: c_uint = 0x52;
pub const CEC_OP_UI_CMD_ELECTRONIC_PROGRAM_GUIDE: c_uint = 0x53;
pub const CEC_OP_UI_CMD_TIMER_PROGRAMMING: c_uint = 0x54;
pub const CEC_OP_UI_CMD_INITIAL_CONFIGURATION: c_uint = 0x55;
pub const CEC_OP_UI_CMD_SELECT_BROADCAST_TYPE: c_uint = 0x56;
pub const CEC_OP_UI_CMD_SELECT_SOUND_PRESENTATION: c_uint = 0x57;
pub const CEC_OP_UI_CMD_AUDIO_DESCRIPTION: c_uint = 0x58;
pub const CEC_OP_UI_CMD_INTERNET: c_uint = 0x59;
pub const CEC_OP_UI_CMD_3D_MODE: c_uint = 0x5a;
pub const CEC_OP_UI_CMD_PLAY_FUNCTION: c_uint = 0x60;
pub const CEC_OP_UI_CMD_PAUSE_PLAY_FUNCTION: c_uint = 0x61;
pub const CEC_OP_UI_CMD_RECORD_FUNCTION: c_uint = 0x62;
pub const CEC_OP_UI_CMD_PAUSE_RECORD_FUNCTION: c_uint = 0x63;
pub const CEC_OP_UI_CMD_STOP_FUNCTION: c_uint = 0x64;
pub const CEC_OP_UI_CMD_MUTE_FUNCTION: c_uint = 0x65;
pub const CEC_OP_UI_CMD_RESTORE_VOLUME_FUNCTION: c_uint = 0x66;
pub const CEC_OP_UI_CMD_TUNE_FUNCTION: c_uint = 0x67;
pub const CEC_OP_UI_CMD_SELECT_MEDIA_FUNCTION: c_uint = 0x68;
pub const CEC_OP_UI_CMD_SELECT_AV_INPUT_FUNCTION: c_uint = 0x69;
pub const CEC_OP_UI_CMD_SELECT_AUDIO_INPUT_FUNCTION: c_uint = 0x6a;
pub const CEC_OP_UI_CMD_POWER_TOGGLE_FUNCTION: c_uint = 0x6b;
pub const CEC_OP_UI_CMD_POWER_OFF_FUNCTION: c_uint = 0x6c;
pub const CEC_OP_UI_CMD_POWER_ON_FUNCTION: c_uint = 0x6d;
pub const CEC_OP_UI_CMD_F1_BLUE: c_uint = 0x71;
pub const CEC_OP_UI_CMD_F2_RED: c_uint = 0x72;
pub const CEC_OP_UI_CMD_F3_GREEN: c_uint = 0x73;
pub const CEC_OP_UI_CMD_F4_YELLOW: c_uint = 0x74;
pub const CEC_OP_UI_CMD_F5: c_uint = 0x75;
pub const CEC_OP_UI_CMD_DATA: c_uint = 0x76;
// UI Broadcast Type Operand (ui_bcast_type)
pub const CEC_OP_UI_BCAST_TYPE_TOGGLE_ALL: c_uint = 0x00;
pub const CEC_OP_UI_BCAST_TYPE_TOGGLE_DIG_ANA: c_uint = 0x01;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE: c_uint = 0x10;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE_T: c_uint = 0x20;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE_CABLE: c_uint = 0x30;
pub const CEC_OP_UI_BCAST_TYPE_ANALOGUE_SAT: c_uint = 0x40;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL: c_uint = 0x50;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_T: c_uint = 0x60;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_CABLE: c_uint = 0x70;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_SAT: c_uint = 0x80;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_COM_SAT: c_uint = 0x90;
pub const CEC_OP_UI_BCAST_TYPE_DIGITAL_COM_SAT2: c_uint = 0x91;
pub const CEC_OP_UI_BCAST_TYPE_IP: c_uint = 0xa0;
// UI Sound Presentation Control Operand (ui_snd_pres_ctl)
pub const CEC_OP_UI_SND_PRES_CTL_DUAL_MONO: c_uint = 0x10;
pub const CEC_OP_UI_SND_PRES_CTL_KARAOKE: c_uint = 0x20;
pub const CEC_OP_UI_SND_PRES_CTL_DOWNMIX: c_uint = 0x80;
pub const CEC_OP_UI_SND_PRES_CTL_REVERB: c_uint = 0x90;
pub const CEC_OP_UI_SND_PRES_CTL_EQUALIZER: c_uint = 0xa0;
pub const CEC_OP_UI_SND_PRES_CTL_BASS_UP: c_uint = 0xb1;
pub const CEC_OP_UI_SND_PRES_CTL_BASS_NEUTRAL: c_uint = 0xb2;
pub const CEC_OP_UI_SND_PRES_CTL_BASS_DOWN: c_uint = 0xb3;
pub const CEC_OP_UI_SND_PRES_CTL_TREBLE_UP: c_uint = 0xc1;
pub const CEC_OP_UI_SND_PRES_CTL_TREBLE_NEUTRAL: c_uint = 0xc2;
pub const CEC_OP_UI_SND_PRES_CTL_TREBLE_DOWN: c_uint = 0xc3;
pub const CEC_MSG_USER_CONTROL_RELEASED: c_uint = 0x45;
// Remote Control Passthrough Feature
//
// Has also:
// CEC_MSG_USER_CONTROL_PRESSED
// CEC_MSG_USER_CONTROL_RELEASED
//
// Power Status Feature
pub const CEC_MSG_GIVE_DEVICE_POWER_STATUS: c_uint = 0x8f;
pub const CEC_MSG_REPORT_POWER_STATUS: c_uint = 0x90;
// Power Status Operand (pwr_state)
pub const CEC_OP_POWER_STATUS_ON: c_int = 0;
pub const CEC_OP_POWER_STATUS_STANDBY: c_int = 1;
pub const CEC_OP_POWER_STATUS_TO_ON: c_int = 2;
pub const CEC_OP_POWER_STATUS_TO_STANDBY: c_int = 3;
// General Protocol Messages
pub const CEC_MSG_FEATURE_ABORT: c_uint = 0x00;
// Abort Reason Operand (reason)
pub const CEC_OP_ABORT_UNRECOGNIZED_OP: c_int = 0;
pub const CEC_OP_ABORT_INCORRECT_MODE: c_int = 1;
pub const CEC_OP_ABORT_NO_SOURCE: c_int = 2;
pub const CEC_OP_ABORT_INVALID_OP: c_int = 3;
pub const CEC_OP_ABORT_REFUSED: c_int = 4;
pub const CEC_OP_ABORT_UNDETERMINED: c_int = 5;
pub const CEC_MSG_ABORT: c_uint = 0xff;
// System Audio Control Feature
//
// Has also:
// CEC_MSG_USER_CONTROL_PRESSED
// CEC_MSG_USER_CONTROL_RELEASED
//
pub const CEC_MSG_GIVE_AUDIO_STATUS: c_uint = 0x71;
pub const CEC_MSG_GIVE_SYSTEM_AUDIO_MODE_STATUS: c_uint = 0x7d;
pub const CEC_MSG_REPORT_AUDIO_STATUS: c_uint = 0x7a;
// Audio Mute Status Operand (aud_mute_status)
pub const CEC_OP_AUD_MUTE_STATUS_OFF: c_int = 0;
pub const CEC_OP_AUD_MUTE_STATUS_ON: c_int = 1;
pub const CEC_MSG_REPORT_SHORT_AUDIO_DESCRIPTOR: c_uint = 0xa3;
pub const CEC_MSG_REQUEST_SHORT_AUDIO_DESCRIPTOR: c_uint = 0xa4;
pub const CEC_MSG_SET_SYSTEM_AUDIO_MODE: c_uint = 0x72;
// System Audio Status Operand (sys_aud_status)
pub const CEC_OP_SYS_AUD_STATUS_OFF: c_int = 0;
pub const CEC_OP_SYS_AUD_STATUS_ON: c_int = 1;
pub const CEC_MSG_SYSTEM_AUDIO_MODE_REQUEST: c_uint = 0x70;
pub const CEC_MSG_SYSTEM_AUDIO_MODE_STATUS: c_uint = 0x7e;
// Audio Format ID Operand (audio_format_id)
pub const CEC_OP_AUD_FMT_ID_CEA861: c_int = 0;
pub const CEC_OP_AUD_FMT_ID_CEA861_CXT: c_int = 1;
pub const CEC_MSG_SET_AUDIO_VOLUME_LEVEL: c_uint = 0x73	/* CEC 2.0 */;
// Audio Rate Control Feature
pub const CEC_MSG_SET_AUDIO_RATE: c_uint = 0x9a;
// Audio Rate Operand (audio_rate)
pub const CEC_OP_AUD_RATE_OFF: c_int = 0;
pub const CEC_OP_AUD_RATE_WIDE_STD: c_int = 1;
pub const CEC_OP_AUD_RATE_WIDE_FAST: c_int = 2;
pub const CEC_OP_AUD_RATE_WIDE_SLOW: c_int = 3;
pub const CEC_OP_AUD_RATE_NARROW_STD: c_int = 4;
pub const CEC_OP_AUD_RATE_NARROW_FAST: c_int = 5;
pub const CEC_OP_AUD_RATE_NARROW_SLOW: c_int = 6;
// Audio Return Channel Control Feature
pub const CEC_MSG_INITIATE_ARC: c_uint = 0xc0;
pub const CEC_MSG_REPORT_ARC_INITIATED: c_uint = 0xc1;
pub const CEC_MSG_REPORT_ARC_TERMINATED: c_uint = 0xc2;
pub const CEC_MSG_REQUEST_ARC_INITIATION: c_uint = 0xc3;
pub const CEC_MSG_REQUEST_ARC_TERMINATION: c_uint = 0xc4;
pub const CEC_MSG_TERMINATE_ARC: c_uint = 0xc5;
// Dynamic Audio Lipsync Feature
pub const CEC_MSG_REQUEST_CURRENT_LATENCY: c_uint = 0xa7;
pub const CEC_MSG_REPORT_CURRENT_LATENCY: c_uint = 0xa8;
// Low Latency Mode Operand (low_latency_mode)
pub const CEC_OP_LOW_LATENCY_MODE_OFF: c_int = 0;
pub const CEC_OP_LOW_LATENCY_MODE_ON: c_int = 1;
// Audio Output Compensated Operand (audio_out_compensated)
pub const CEC_OP_AUD_OUT_COMPENSATED_NA: c_int = 0;
pub const CEC_OP_AUD_OUT_COMPENSATED_DELAY: c_int = 1;
pub const CEC_OP_AUD_OUT_COMPENSATED_NO_DELAY: c_int = 2;
pub const CEC_OP_AUD_OUT_COMPENSATED_PARTIAL_DELAY: c_int = 3;
// Latency Indication Protocol Feature
pub const CEC_MSG_REQUEST_LIP_SUPPORT: c_uint = 0x50	/* CEC 2.0 */;
pub const CEC_MSG_REPORT_LIP_SUPPORT: c_uint = 0x51	/* CEC 2.0 */;
pub const CEC_MSG_REQUEST_AUDIO_AND_VIDEO_LATENCY: c_uint = 0x52	/* CEC 2.0 */;
// HDR Format Operand (hdr_format)
pub const CEC_OP_HDR_FORMAT_GAMMA_SDR: c_int = 0;
pub const CEC_OP_HDR_FORMAT_GAMMA_HDR: c_int = 1;
pub const CEC_OP_HDR_FORMAT_PQ: c_int = 2;
pub const CEC_OP_HDR_FORMAT_HLG: c_int = 3;
pub const CEC_OP_HDR_FORMAT_DYNAMIC_HDR_TYPE_1: c_int = 8;
pub const CEC_OP_HDR_FORMAT_DYNAMIC_HDR_TYPE_2: c_int = 9;
pub const CEC_OP_HDR_FORMAT_DYNAMIC_HDR_TYPE_4: c_int = 11;
pub const CEC_OP_HDR_FORMAT_DV_SINK_LED: c_int = 16;
pub const CEC_OP_HDR_FORMAT_DV_SOURCE_LED: c_int = 17;
pub const CEC_OP_HDR_FORMAT_HDR10PLUS: c_int = 24;
pub const CEC_OP_HDR_FORMAT_ETSI_TS_103_433: c_int = 32;
pub const CEC_MSG_REPORT_AUDIO_AND_VIDEO_LATENCY: c_uint = 0x53	/* CEC 2.0 */;
pub const CEC_MSG_REQUEST_AUDIO_LATENCY: c_uint = 0x54	/* CEC 2.0 */;
pub const CEC_MSG_REPORT_AUDIO_LATENCY: c_uint = 0x55	/* CEC 2.0 */;
pub const CEC_MSG_REQUEST_VIDEO_LATENCY: c_uint = 0x56	/* CEC 2.0 */;
pub const CEC_MSG_REPORT_VIDEO_LATENCY: c_uint = 0x57	/* CEC 2.0 */;
pub const CEC_MSG_UPDATE_SQID: c_uint = 0x58	/* CEC 2.0 */;
// Capability Discovery and Control Feature
pub const CEC_MSG_CDC_MESSAGE: c_uint = 0xf8;
// Ethernet-over-HDMI: nobody ever does this...
pub const CEC_MSG_CDC_HEC_INQUIRE_STATE: c_uint = 0x00;
pub const CEC_MSG_CDC_HEC_REPORT_STATE: c_uint = 0x01;
// HEC Functionality State Operand (hec_func_state)
pub const CEC_OP_HEC_FUNC_STATE_NOT_SUPPORTED: c_int = 0;
pub const CEC_OP_HEC_FUNC_STATE_INACTIVE: c_int = 1;
pub const CEC_OP_HEC_FUNC_STATE_ACTIVE: c_int = 2;
pub const CEC_OP_HEC_FUNC_STATE_ACTIVATION_FIELD: c_int = 3;
// Host Functionality State Operand (host_func_state)
pub const CEC_OP_HOST_FUNC_STATE_NOT_SUPPORTED: c_int = 0;
pub const CEC_OP_HOST_FUNC_STATE_INACTIVE: c_int = 1;
pub const CEC_OP_HOST_FUNC_STATE_ACTIVE: c_int = 2;
// ENC Functionality State Operand (enc_func_state)
pub const CEC_OP_ENC_FUNC_STATE_EXT_CON_NOT_SUPPORTED: c_int = 0;
pub const CEC_OP_ENC_FUNC_STATE_EXT_CON_INACTIVE: c_int = 1;
pub const CEC_OP_ENC_FUNC_STATE_EXT_CON_ACTIVE: c_int = 2;
// CDC Error Code Operand (cdc_errcode)
pub const CEC_OP_CDC_ERROR_CODE_NONE: c_int = 0;
pub const CEC_OP_CDC_ERROR_CODE_CAP_UNSUPPORTED: c_int = 1;
pub const CEC_OP_CDC_ERROR_CODE_WRONG_STATE: c_int = 2;
pub const CEC_OP_CDC_ERROR_CODE_OTHER: c_int = 3;
// HEC Support Operand (hec_support)
pub const CEC_OP_HEC_SUPPORT_NO: c_int = 0;
pub const CEC_OP_HEC_SUPPORT_YES: c_int = 1;
// HEC Activation Operand (hec_activation)
pub const CEC_OP_HEC_ACTIVATION_ON: c_int = 0;
pub const CEC_OP_HEC_ACTIVATION_OFF: c_int = 1;
pub const CEC_MSG_CDC_HEC_SET_STATE_ADJACENT: c_uint = 0x02;
pub const CEC_MSG_CDC_HEC_SET_STATE: c_uint = 0x03;
// HEC Set State Operand (hec_set_state)
pub const CEC_OP_HEC_SET_STATE_DEACTIVATE: c_int = 0;
pub const CEC_OP_HEC_SET_STATE_ACTIVATE: c_int = 1;
pub const CEC_MSG_CDC_HEC_REQUEST_DEACTIVATION: c_uint = 0x04;
pub const CEC_MSG_CDC_HEC_NOTIFY_ALIVE: c_uint = 0x05;
pub const CEC_MSG_CDC_HEC_DISCOVER: c_uint = 0x06;
// Hotplug Detect messages
pub const CEC_MSG_CDC_HPD_SET_STATE: c_uint = 0x10;
// HPD State Operand (hpd_state)
pub const CEC_OP_HPD_STATE_CP_EDID_DISABLE: c_int = 0;
pub const CEC_OP_HPD_STATE_CP_EDID_ENABLE: c_int = 1;
pub const CEC_OP_HPD_STATE_CP_EDID_DISABLE_ENABLE: c_int = 2;
pub const CEC_OP_HPD_STATE_EDID_DISABLE: c_int = 3;
pub const CEC_OP_HPD_STATE_EDID_ENABLE: c_int = 4;
pub const CEC_OP_HPD_STATE_EDID_DISABLE_ENABLE: c_int = 5;
pub const CEC_MSG_CDC_HPD_REPORT_STATE: c_uint = 0x11;
// HPD Error Code Operand (hpd_error)
pub const CEC_OP_HPD_ERROR_NONE: c_int = 0;
pub const CEC_OP_HPD_ERROR_INITIATOR_NOT_CAPABLE: c_int = 1;
pub const CEC_OP_HPD_ERROR_INITIATOR_WRONG_STATE: c_int = 2;
pub const CEC_OP_HPD_ERROR_OTHER: c_int = 3;
pub const CEC_OP_HPD_ERROR_NONE_NO_VIDEO: c_int = 4;
// End of Messages
// Helper functions to identify the 'special' CEC devices
//
// It is a second TV if the logical address is 14 or 15 and the
// primary device type is a TV.
//
// It is a processor if the logical address is 12-15 and the
// primary device type is a Processor.
//
// It is a switch if the logical address is 15 and the
// primary device type is a Switch and the CDC-Only flag is not set.
//
// It is a CDC-only device if the logical address is 15 and the
// primary device type is a Switch and the CDC-Only flag is set.
//
