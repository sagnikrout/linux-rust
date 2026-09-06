//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_mux_codec.h
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

// Queue level size and reporting
// >1 is enable, 0 is disable
//
pub const MUX_QUEUE_LEVEL: c_int = 1;
// ADB finish timer value

// Enables the flow control (Flow is not allowed)
pub const IOSM_AGGR_MUX_CMD_FLOW_CTL_ENABLE: c_int = 5;
// Disables the flow control (Flow is allowed)
pub const IOSM_AGGR_MUX_CMD_FLOW_CTL_DISABLE: c_int = 6;
// ACK the flow control command. Shall have the same Transaction ID as the
// matching FLOW_CTL command
//
pub const IOSM_AGGR_MUX_CMD_FLOW_CTL_ACK: c_int = 7;
// Aggregation Protocol Command for report packet indicating link quality
//
pub const IOSM_AGGR_MUX_CMD_LINK_STATUS_REPORT: c_int = 8;
// Response to a report packet
pub const IOSM_AGGR_MUX_CMD_LINK_STATUS_REPORT_RESP: c_int = 9;
// ACBH: Signature of the Aggregated Command Block Header.
pub const IOSM_AGGR_MUX_SIG_ACBH: c_uint = 0x48424341;
// ADTH: Signature of the Aggregated Datagram Table Header.
pub const IOSM_AGGR_MUX_SIG_ADTH: c_uint = 0x48544441;
// ADBH: Signature of the Aggregated Data Block Header.
pub const IOSM_AGGR_MUX_SIG_ADBH: c_uint = 0x48424441;
// ADGH: Signature of the Datagram Header.
pub const IOSM_AGGR_MUX_SIG_ADGH: c_uint = 0x48474441;
// Size of the buffer for the IP MUX commands.
pub const MUX_MAX_UL_ACB_BUF_SIZE: c_int = 256;
// Maximum number of packets in a go per session
pub const MUX_MAX_UL_DG_ENTRIES: c_int = 100;
// ADGH: Signature of the Datagram Header.
pub const MUX_SIG_ADGH: c_uint = 0x48474441;
// CMDH: Signature of the Command Header.
pub const MUX_SIG_CMDH: c_uint = 0x48444D43;
// QLTH: Signature of the Queue Level Table
pub const MUX_SIG_QLTH: c_uint = 0x48544C51;
// FCTH: Signature of the Flow Credit Table
pub const MUX_SIG_FCTH: c_uint = 0x48544346;
// MUX UL session threshold factor

// Size of the buffer for the IP MUX Lite data buffer.

// MUX UL session threshold in number of packets

// Default time out for sending IPC session commands like
// open session, close session etc
// unit : milliseconds
//

// MUX UL flow control lower threshold in bytes

// MUX UL flow control higher threshold in bytes (5ms worth of data)

//
// struct mux_cmdh - Structure of Command Header.
// @signature:		Signature of the Command Header.
// @cmd_len:		Length (in bytes) of the Aggregated Command Block.
// @if_id:		ID of the interface the commands in the table belong to.
// @reserved:		Reserved. Set to zero.
// @next_cmd_index:	Index (in bytes) to the next command in the buffer.
// @command_type:	Command Enum. See table Session Management chapter for
// details.
// @transaction_id:	The Transaction ID shall be unique to the command
// @param:		Optional parameters used with the command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_cmdh {
    pub signature: __le32,
    pub cmd_len: __le16,
    pub if_id: u8,
    pub reserved: u8,
    pub next_cmd_index: __le32,
    pub command_type: __le32,
    pub transaction_id: __le32,
    pub param: mux_cmd_param,
}

//
// struct mux_acbh -    Structure of the Aggregated Command Block Header.
// @signature:          Signature of the Aggregated Command Block Header.
// @reserved:           Reserved bytes. Set to zero.
// @sequence_nr:        Block sequence number.
// @block_length:       Length (in bytes) of the Aggregated Command Block.
// @first_cmd_index:    Index (in bytes) to the first command in the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_acbh {
    pub signature: __le32,
    pub reserved: __le16,
    pub sequence_nr: __le16,
    pub block_length: __le32,
    pub first_cmd_index: __le32,
}

//
// struct mux_adbh - Structure of the Aggregated Data Block Header.
// @signature:		Signature of the Aggregated Data Block Header.
// @reserved:		Reserved bytes. Set to zero.
// @sequence_nr:	Block sequence number.
// @block_length:	Length (in bytes) of the Aggregated Data Block.
// @first_table_index:	Index (in bytes) to the first Datagram Table in
// the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_adbh {
    pub signature: __le32,
    pub reserved: __le16,
    pub sequence_nr: __le16,
    pub block_length: __le32,
    pub first_table_index: __le32,
}

//
// struct mux_adth - Structure of the Aggregated Datagram Table Header.
// @signature:          Signature of the Aggregated Datagram Table Header.
// @table_length:       Length (in bytes) of the datagram table.
// @if_id:              ID of the interface the datagrams in the table
// belong to.
// @opt_ipv4v6:         Indicates IPv4(=0)/IPv6(=1) hint.
// @reserved:           Reserved bits. Set to zero.
// @next_table_index:   Index (in bytes) to the next Datagram Table in
// the buffer.
// @reserved2:          Reserved bytes. Set to zero
// @dg:                 datagramm table with variable length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_adth {
    pub signature: __le32,
    pub table_length: __le16,
    pub if_id: u8,
    pub opt_ipv4v6: u8,
    pub next_table_index: __le32,
    pub reserved2: __le32,
    pub dg: [mux_adth_dg; ],
}

//
// struct mux_adgh - Aggregated Datagram Header.
// @signature:		Signature of the Aggregated Datagram Header(0x48474441)
// @length:		Length (in bytes) of the datagram header. This length
// shall include the header size. Min value: 0x10
// @if_id:		ID of the interface the datagrams belong to
// @opt_ipv4v6:		Indicates IPv4(=0)/IPv6(=1), It is optional if not
// used set it to zero.
// @reserved:		Reserved bits. Set to zero.
// @service_class:	Service class identifier for the datagram.
// @next_count:		Count of the datagrams that shall be following this
// datagrams for this interface. A count of zero means
// the next datagram may not belong to this interface.
// @reserved1:		Reserved bytes, Set to zero
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_adgh {
    pub signature: __le32,
    pub length: __le16,
    pub if_id: u8,
    pub opt_ipv4v6: u8,
    pub service_class: u8,
    pub next_count: u8,
    pub reserved1: [u8; 6],
}

//
// struct mux_lite_cmdh - MUX Lite Command Header
// @signature:		Signature of the Command Header(0x48444D43)
// @cmd_len:		Length (in bytes) of the command. This length shall
// include the header size. Minimum value: 0x10
// @if_id:		ID of the interface the commands in the table belong to.
// @reserved:		Reserved Set to zero.
// @command_type:	Command Enum.
// @transaction_id:	4 byte value shall be generated and sent along with a
// command Responses and ACKs shall have the same
// Transaction ID as their commands. It shall be unique to
// the command transaction on the given interface.
// @param:		Optional parameters used with the command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_lite_cmdh {
    pub signature: __le32,
    pub cmd_len: __le16,
    pub if_id: u8,
    pub reserved: u8,
    pub command_type: __le32,
    pub transaction_id: __le32,
    pub param: mux_cmd_param,
}

//
// struct mux_lite_vfl - value field in generic table
// @nr_of_bytes:	Number of bytes available to transmit in the queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_lite_vfl {
    pub nr_of_bytes: __le32,
}

//
// struct ipc_mem_lite_gen_tbl - Generic table format for Queue Level
// and Flow Credit
// @signature:	Signature of the table
// @length:	Length of the table
// @if_id:	ID of the interface the table belongs to
// @vfl_length:	Value field length
// @reserved:	Reserved
// @vfl:	Value field of variable length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_lite_gen_tbl {
    pub signature: __le32,
    pub length: __le16,
    pub if_id: u8,
    pub vfl_length: u8,
    pub reserved: [u32; 2],
    pub vfl: mux_lite_vfl,
}

//
// struct mux_type_cmdh - Structure of command header for mux lite and aggr
// @ack_lite:	MUX Lite Command Header pointer
// @ack_aggr:	Command Header pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mux_type_cmdh {
    pub ack_lite: *mut mux_lite_cmdh,
    pub ack_aggr: *mut mux_cmdh,
}

//
// struct mux_type_header - Structure of mux header type
// @adgh:	Aggregated Datagram Header pointer
// @adbh:	Aggregated Data Block Header pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mux_type_header {
    pub adgh: *mut mux_adgh,
    pub adbh: *mut mux_adbh,
}

extern "C" {
    pub fn ipc_mux_dl_decode(ipc_mux: *mut iosm_mux, skb: *mut sk_buff);
}
//
// ipc_mux_dl_acb_send_cmds - Respond to the Command blocks.
// @ipc_mux:		Pointer to MUX data-struct
// @cmd_type:		Command
// @if_id:		Session interface id.
// @transaction_id:	Command transaction id.
// @param:		Pointer to command params.
// @res_size:		Response size
// @blocking:		True for blocking send
// @respond:		If true return transaction ID
//
// Returns:		0 in success and failure value on error
//
// ipc_mux_netif_tx_flowctrl - Enable/Disable TX flow control on MUX sessions.
// @session:	Pointer to mux_session struct
// @idx:	Session ID
// @on:		true for Enable and false for disable flow control
//
extern "C" {
    pub fn ipc_mux_netif_tx_flowctrl(session: *mut mux_session, idx: c_int, on: bool);
}
//
// ipc_mux_ul_trigger_encode - Route the UL packet through the IP MUX layer
// for encoding.
// @ipc_mux:	Pointer to MUX data-struct
// @if_id:	Session ID.
// @skb:	Pointer to ipc_skb.
//
// Returns: 0 if successfully encoded
// failure value on error
// -EBUSY if packet has to be retransmitted.
//
// ipc_mux_ul_data_encode - UL encode function for calling from Tasklet context.
// @ipc_mux:	Pointer to MUX data-struct
//
// Returns: TRUE if any packet of any session is encoded FALSE otherwise.
//
extern "C" {
    pub fn ipc_mux_ul_data_encode(ipc_mux: *mut iosm_mux) -> bool;
}
//
// ipc_mux_ul_encoded_process - Handles the Modem processed UL data by adding
// the SKB to the UL free list.
// @ipc_mux:	Pointer to MUX data-struct
// @skb:	Pointer to ipc_skb.
//
extern "C" {
    pub fn ipc_mux_ul_encoded_process(ipc_mux: *mut iosm_mux, skb: *mut sk_buff);
}
extern "C" {
    pub fn ipc_mux_ul_adb_finish(ipc_mux: *mut iosm_mux);
}
