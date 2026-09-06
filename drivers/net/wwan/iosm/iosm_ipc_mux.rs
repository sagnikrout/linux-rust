//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_mux.h
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
// Copyright (C) 2020-21 Intel Corporation.
//

pub const IPC_MEM_MAX_UL_DG_ENTRIES: c_int = 100;
pub const IPC_MEM_MAX_TDS_MUX_AGGR_UL: c_int = 60;
pub const IPC_MEM_MAX_TDS_MUX_AGGR_DL: c_int = 60;

// Size of the buffer for the IP MUX Lite data buffer.

// TD counts for IP MUX Lite
pub const IPC_MEM_MAX_TDS_MUX_LITE_UL: c_int = 800;
pub const IPC_MEM_MAX_TDS_MUX_LITE_DL: c_int = 1200;
// open session request (AP->CP)
pub const MUX_CMD_OPEN_SESSION: c_int = 1;
// response to open session request (CP->AP)
pub const MUX_CMD_OPEN_SESSION_RESP: c_int = 2;
// close session request (AP->CP)
pub const MUX_CMD_CLOSE_SESSION: c_int = 3;
// response to close session request (CP->AP)
pub const MUX_CMD_CLOSE_SESSION_RESP: c_int = 4;
// Flow control command with mask of the flow per queue/flow.
pub const MUX_LITE_CMD_FLOW_CTL: c_int = 5;
// ACK the flow control command. Shall have the same Transaction ID as the
// matching FLOW_CTL command.
//
pub const MUX_LITE_CMD_FLOW_CTL_ACK: c_int = 6;
// Command for report packet indicating link quality metrics.
pub const MUX_LITE_CMD_LINK_STATUS_REPORT: c_int = 7;
// Response to a report packet
pub const MUX_LITE_CMD_LINK_STATUS_REPORT_RESP: c_int = 8;
// Used to reset a command/response state.
pub const MUX_CMD_INVALID: c_int = 255;
// command response : command processed successfully
pub const MUX_CMD_RESP_SUCCESS: c_int = 0;
// MUX for route link devices

// Initiated actions to change the state of the MUX object.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mux_event {
    MUX_E_INACTIVE, /* No initiated actions. */
    MUX_E_MUX_SESSION_OPEN, /* Create the MUX channel and a session. */
    MUX_E_MUX_SESSION_CLOSE, /* Release a session. */
    MUX_E_MUX_CHANNEL_CLOSE, /* Release the MUX channel. */
    MUX_E_NO_ORDERS, /* No MUX order. */
    MUX_E_NOT_APPLICABLE, /* Defect IP MUX. */
}

// MUX session open command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_session_open {
    pub event: mux_event,
    pub if_id: __le32,
}

// MUX session close command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_session_close {
    pub event: mux_event,
    pub if_id: __le32,
}

// MUX channel close command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_channel_close {
    pub event: mux_event,
}

// Default message type to find out the right message type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_common {
    pub event: mux_event,
}

// List of ops in MUX mode.
#[repr(C)]
#[derive(Copy, Clone)]
pub union mux_msg {
    pub session_open: mux_session_open,
    pub session_close: mux_session_close,
    pub channel_close: mux_channel_close,
    pub common: mux_common,
}

// Parameter definition of the open session command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmd_open_session {
    pub /: *mut *mut u8 flow_ctrl; / 0: Flow control disabled (flow allowed).,
// 1: Flow control enabled (flow not allowed)
    pub supported.*/: *mut *mut u8 ipv4v6_hints; / 0: IPv4/IPv6 hints not,
// 1: IPv4/IPv6 hints supported
    pub /: *mut *mut __le16 reserved2; / Reserved. Set to zero.,
    pub /: *mut *mut __le32 dl_head_pad_len; / Maximum length supported,
// for DL head padding on a datagram.
}

// Parameter definition of the open session response.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmd_open_session_resp {
    pub /: *mut *mut __le32 response; / Response code,
    pub /: *mut *mut u8 flow_ctrl; / 0: Flow control disabled (flow allowed).,
// 1: Flow control enabled (flow not allowed)
    pub /: *mut *mut u8 ipv4v6_hints; / 0: IPv4/IPv6 hints not supported,
// 1: IPv4/IPv6 hints supported
    pub /: *mut *mut __le16 reserved2; / Reserved. Set to zero.,
    pub /: *mut *mut __le32 ul_head_pad_len; / Actual length supported for,
// UL head padding on adatagram.
}

// Parameter definition of the close session response code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmd_close_session_resp {
    pub response: __le32,
}

// Parameter definition of the flow control command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmd_flow_ctl {
    pub /: *mut *mut __le32 mask; / indicating the desired flow control,
// state for various flows/queues
}

// Parameter definition of the link status report code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmd_link_status_report {
    pub payload: u8,
}

// Parameter definition of the link status report response code.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmd_link_status_report_resp {
    pub response: __le32,
}

//
// union mux_cmd_param - Union-definition of the command parameters.
// @open_session:	Inband command for open session
// @open_session_resp:	Inband command for open session response
// @close_session_resp:	Inband command for close session response
// @flow_ctl:		In-band flow control on the opened interfaces
// @link_status:	In-band Link Status Report
// @link_status_resp:	In-band command for link status report response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mux_cmd_param {
    pub open_session: mux_cmd_open_session,
    pub open_session_resp: mux_cmd_open_session_resp,
    pub close_session_resp: mux_cmd_close_session_resp,
    pub flow_ctl: mux_cmd_flow_ctl,
    pub link_status: mux_cmd_link_status_report,
    pub link_status_resp: mux_cmd_link_status_report_resp,
}

// States of the MUX object..
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mux_state {
    MUX_S_INACTIVE, /* IP MUX is unused. */
    MUX_S_ACTIVE, /* IP MUX channel is available. */
    MUX_S_ERROR, /* Defect IP MUX. */
}

// Supported MUX protocols.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mux_protocol {
    MUX_UNKNOWN,
    MUX_LITE,
    MUX_AGGREGATION,
}

// Supported UL data transfer methods.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mux_ul_flow {
    MUX_UL_UNKNOWN,
    MUX_UL, /* Normal UL data transfer */
    MUX_UL_ON_CREDITS, /* UL data transfer will be based on credits */
}

// List of the MUX session.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_session {
    pub communication*/: *mut *mut *mut iosm_wwan wwan; /Network i/f used for,
    pub message.*/: *mut *mut int if_id; / i/f id for session open,
    pub flags: u32,
    pub /: *mut *mut u32 ul_head_pad_len; / Nr of bytes for UL head padding.,
    pub /: *mut *mut u32 dl_head_pad_len; / Nr of bytes for DL head padding.,
    pub /: *mut *mut sk_buff_head ul_list; / skb entries for an ADT.,
    pub /: *mut *mut u32 flow_ctl_mask; / UL flow control,
    pub /: *mut *mut u32 flow_ctl_en_cnt; / Flow control Enable cmd count,
    pub /: *mut *mut u32 flow_ctl_dis_cnt; / Flow Control Disable cmd count,
    pub /: *mut *mut int ul_flow_credits; / UL flow credits,
    pub /: *mut *mut flush:1; / flush net interface ?,
}

//
// struct mux_adth_dg - Structure of the datagram in the Aggregated Datagram
// Table Header.
// @datagram_index :	Index (in bytes) to the k-th datagram in the table.
// Index shall count from the start of the block including
// the 16-byte header. This value shall be non-zero.
// @datagram_length:	Length of the k-th datagram including the head padding.
// This value shall be non-zero.
// @service_class:	Service class identifier for the datagram.
// @reserved:		Reserved bytes. Set to zero
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_adth_dg {
    pub datagram_index: __le32,
    pub datagram_length: __le16,
    pub service_class: u8,
    pub reserved: u8,
}

//
// struct mux_qlth_ql - Structure of the queue level in the Aggregated
// Datagram Queue Level Table Header.
// @nr_of_bytes:	Number of bytes available to transmit in the queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_qlth_ql {
    pub nr_of_bytes: __le32,
}

//
// struct mux_qlth -    Structure of Aggregated Datagram Queue Level Table
// Header.
// @signature:          Signature of the Queue Level Table Header
// Value: 0x48544C51 (ASCII characters: 'Q' 'L' 'T' 'H')
// @table_length:       Length (in bytes) of the datagram table. This length
// shall include the queue level table header size.
// Minimum value:0x10
// @if_id:              ID of the interface the queue levels in the table
// belong to.
// @reserved:           Reserved byte. Set to zero.
// @next_table_index:   Index (in bytes) to the next table in the buffer. Index
// shall count from the start of the block including the
// 16-byte header. Value of zero indicates end of the list.
// @reserved2:          Reserved bytes. Set to zero
// @ql:                 Queue level table with variable length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_qlth {
    pub signature: __le32,
    pub table_length: __le16,
    pub if_id: u8,
    pub reserved: u8,
    pub next_table_index: __le32,
    pub reserved2: __le32,
    pub ql: mux_qlth_ql,
}

//
// struct mux_adb - Structure of State of a single UL data block.
// @dest_skb:		Current UL skb for the data block.
// @buf:		ADB memory
// @adgh:		ADGH pointer
// @qlth_skb:		QLTH pointer
// @next_table_index:	Pointer to next table index.
// @free_list:		List of alloc. ADB for the UL sess.
// @size:		Size of the ADB memory.
// @if_cnt:		Statistic counter
// @dg_cnt_total:	Datagram count total
// @payload_size:	Payload Size
// @dg:			Datagram table.
// @pp_qlt:		Pointers to hold Queue Level Tables of session
// @adbh:		ADBH pointer
// @qlt_updated:	Queue level table updated
// @dg_count:		Datagram count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_adb {
    pub dest_skb: *mut sk_buff,
    pub buf: *mut u8,
    pub adgh: *mut mux_adgh,
    pub qlth_skb: *mut sk_buff,
    pub next_table_index: *mut u32,
    pub free_list: sk_buff_head,
    pub size: c_int,
    pub if_cnt: u32,
    pub dg_cnt_total: u32,
    pub payload_size: u32,
    pub pp_qlt: [*mut mux_qlth; IPC_MEM_MUX_IP_SESSION_ENTRIES],
    pub adbh: *mut mux_adbh,
    pub qlt_updated: [u32; IPC_MEM_MUX_IP_SESSION_ENTRIES],
    pub dg_count: [u32; IPC_MEM_MUX_IP_SESSION_ENTRIES],
}

//
// struct mux_acb - Structure of Temporary ACB state.
// @skb:		Used UL skb.
// @if_id:		Session id.
// @buf_p:		Command buffer.
// @wanted_response:	Wanted Response
// @got_response:	Got response
// @cmd:		command
// @got_param:		Received command/response parameter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_acb {
    pub /: *mut *mut *mut sk_buff skb; / Used UL skb.,
    pub /: *mut *mut int if_id; / Session id.,
    pub buf_p: *mut u8,
    pub wanted_response: u32,
    pub got_response: u32,
    pub cmd: u32,
    pub /: *mut *mut mux_cmd_param got_param; / Received command/response parameter,
}

//
// struct iosm_mux - Structure of the data multiplexing over an IP channel.
// @dev:		Pointer to device structure
// @session:		Array of the MUX sessions.
// @channel:		Reference to the IP MUX channel
// @pcie:		Pointer to iosm_pcie struct
// @imem:		Pointer to iosm_imem
// @wwan:		Poinetr to iosm_wwan
// @ipc_protocol:	Pointer to iosm_protocol
// @channel_id:		Channel ID for MUX
// @protocol:		Type of the MUX protocol
// @ul_flow:		UL Flow type
// @nr_sessions:	Number of sessions
// @instance_id:	Instance ID
// @state:		States of the MUX object
// @event:		Initiated actions to change the state of the MUX object
// @tx_transaction_id:	Transaction id for the ACB command.
// @rr_next_session:	Next session number for round robin.
// @ul_adb:		State of the UL ADB/ADGH.
// @size_needed:	Variable to store the size needed during ADB preparation
// @ul_data_pend_bytes:	Pending UL data to be processed in bytes
// @acb:		Temporary ACB state
// @wwan_q_offset:	This will hold the offset of the given instance
// Useful while passing or receiving packets from
// wwan/imem layer.
// @acb_tx_sequence_nr: Sequence number for the ACB header.
// @adb_tx_sequence_nr: Sequence number for ADB header
// @acc_adb_size:       Statistic data for logging
// @acc_payload_size:   Statistic data for logging
// @initialized:	MUX object is initialized
// @ev_mux_net_transmit_pending:
// 0 means inform the IPC tasklet to pass the
// accumulated uplink ADB to CP.
// @adb_prep_ongoing:	Flag for ADB preparation status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_mux {
    pub dev: *mut device,
    pub session: [mux_session; IPC_MEM_MUX_IP_SESSION_ENTRIES],
    pub channel: *mut ipc_mem_channel,
    pub pcie: *mut iosm_pcie,
    pub imem: *mut iosm_imem,
    pub wwan: *mut iosm_wwan,
    pub ipc_protocol: *mut iosm_protocol,
    pub channel_id: c_int,
    pub protocol: ipc_mux_protocol,
    pub ul_flow: ipc_mux_ul_flow,
    pub nr_sessions: c_int,
    pub instance_id: c_int,
    pub state: mux_state,
    pub event: mux_event,
    pub tx_transaction_id: u32,
    pub rr_next_session: c_int,
    pub ul_adb: mux_adb,
    pub size_needed: c_int,
    pub ul_data_pend_bytes: c_longlong,
    pub acb: mux_acb,
    pub wwan_q_offset: c_int,
    pub acb_tx_sequence_nr: u16,
    pub adb_tx_sequence_nr: u16,
    pub acc_adb_size: c_ulonglong,
    pub acc_payload_size: c_ulonglong,
    pub __packed: },
// MUX configuration structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mux_config {
    pub protocol: ipc_mux_protocol,
    pub ul_flow: ipc_mux_ul_flow,
    pub instance_id: c_int,
}

//
// ipc_mux_init - Allocates and Init MUX instance
// @mux_cfg:	Pointer to MUX configuration structure
// @ipc_imem:	Pointer to imem data-struct
//
// Returns: Initialized mux pointer on success else NULL
//
// ipc_mux_deinit - Deallocates MUX instance
// @ipc_mux:	Pointer to the MUX instance.
//
extern "C" {
    pub fn ipc_mux_deinit(ipc_mux: *mut iosm_mux);
}
//
// ipc_mux_check_n_restart_tx - Checks for pending UL date bytes and then
// it restarts the net interface tx queue if
// device has set flow control as off.
// @ipc_mux:	Pointer to MUX data-struct
//
extern "C" {
    pub fn ipc_mux_check_n_restart_tx(ipc_mux: *mut iosm_mux);
}
//
// ipc_mux_get_active_protocol - Returns the active MUX protocol type.
// @ipc_mux:	Pointer to MUX data-struct
//
// Returns: enum of type ipc_mux_protocol
//
extern "C" {
    pub fn ipc_mux_get_active_protocol(ipc_mux: *mut iosm_mux) -> ipc_mux_protocol;
}
//
// ipc_mux_open_session - Opens a MUX session for IP traffic.
// @ipc_mux:	Pointer to MUX data-struct
// @session_nr:	Interface ID or session number
//
// Returns: channel id on success, failure value on error
//
extern "C" {
    pub fn ipc_mux_open_session(ipc_mux: *mut iosm_mux, session_nr: c_int) -> c_int;
}
//
// ipc_mux_close_session - Closes a MUX session.
// @ipc_mux:	Pointer to MUX data-struct
// @session_nr:	Interface ID or session number
//
// Returns: channel id on success, failure value on error
//
extern "C" {
    pub fn ipc_mux_close_session(ipc_mux: *mut iosm_mux, session_nr: c_int) -> c_int;
}
//
// ipc_mux_get_max_sessions - Returns the maximum sessions supported on the
// provided MUX instance..
// @ipc_mux:	Pointer to MUX data-struct
//
// Returns: Number of sessions supported on Success and failure value on error
//
extern "C" {
    pub fn ipc_mux_get_max_sessions(ipc_mux: *mut iosm_mux) -> c_int;
}
