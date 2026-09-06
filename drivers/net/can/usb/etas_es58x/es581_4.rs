//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/usb/etas_es58x/es581_4.h
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


// SPDX-License-Identifier: GPL-2.0
// Driver for ETAS GmbH ES58X USB CAN(-FD) Bus Interfaces.
//
// File es581_4.h: Definitions and declarations specific to ETAS
// ES581.4.
//
// Copyright (c) 2019 Robert Bosch Engineering and Business Solutions. All rights reserved.
// Copyright (c) 2020 ETAS K.K.. All rights reserved.
// Copyright (c) 2020, 2021 Vincent Mailhol <mailhol.vincent@wanadoo.fr>
//

pub const ES581_4_NUM_CAN_CH: c_int = 2;
pub const ES581_4_CHANNEL_IDX_OFFSET: c_int = 1;
pub const ES581_4_TX_BULK_MAX: c_int = 25;
pub const ES581_4_RX_BULK_MAX: c_int = 30;
pub const ES581_4_ECHO_BULK_MAX: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es581_4_cmd_type {
    ES581_4_CAN_COMMAND_TYPE = 0x45
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es581_4_cmd_id {
    ES581_4_CMD_ID_OPEN_CHANNEL = 0x01,
    ES581_4_CMD_ID_CLOSE_CHANNEL = 0x02,
    ES581_4_CMD_ID_SET_BITTIMING = 0x03,
    ES581_4_CMD_ID_ENABLE_CHANNEL = 0x04,
    ES581_4_CMD_ID_TX_MSG = 0x05,
    ES581_4_CMD_ID_RX_MSG = 0x06,
    ES581_4_CMD_ID_RESET_RX = 0x0A,
    ES581_4_CMD_ID_RESET_TX = 0x0B,
    ES581_4_CMD_ID_DISABLE_CHANNEL = 0x0C,
    ES581_4_CMD_ID_TIMESTAMP = 0x0E,
    ES581_4_CMD_ID_RESET_DEVICE = 0x28,
    ES581_4_CMD_ID_ECHO = 0x71,
    ES581_4_CMD_ID_DEVICE_ERR = 0x72
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es581_4_rx_type {
    ES581_4_RX_TYPE_MESSAGE = 1,
    ES581_4_RX_TYPE_ERROR = 3,
    ES581_4_RX_TYPE_EVENT = 4
}

//
// struct es581_4_tx_conf_msg - Channel configuration.
// @bitrate: Bitrate.
// @sample_point: Sample point is in percent [0..100].
// @samples_per_bit: type enum es58x_samples_per_bit.
// @bit_time: Number of time quanta in one bit.
// @sjw: Synchronization Jump Width.
// @sync_edge: type enum es58x_sync_edge.
// @physical_layer: type enum es58x_physical_layer.
// @echo_mode: type enum es58x_echo_mode.
// @channel_no: Channel number, starting from 1. Not to be confused
// with channed_idx of the ES58X FD which starts from 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_tx_conf_msg {
    pub bitrate: __le32,
    pub sample_point: __le32,
    pub samples_per_bit: __le32,
    pub bit_time: __le32,
    pub sjw: __le32,
    pub sync_edge: __le32,
    pub physical_layer: __le32,
    pub echo_mode: __le32,
    pub channel_no: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_tx_can_msg {
    pub can_id: __le32,
    pub packet_idx: __le32,
    pub flags: __le16,
    pub channel_no: u8,
    pub dlc: u8,
    pub data: [u8; CAN_MAX_DLEN],
    pub __packed: },
// The ES581.4 allows bulk transfer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_bulk_tx_can_msg {
    pub num_can_msg: u8,
// Using type "u8[]" instead of "struct es581_4_tx_can_msg[]"
// for tx_msg_buf because each member has a flexible size.
//
    pub es581_4_tx_can_msg)]: sizeof(struct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_echo_msg {
    pub timestamp: __le64,
    pub packet_idx: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_bulk_echo_msg {
    pub channel_no: u8,
    pub echo_msg: [es581_4_echo_msg; ES581_4_ECHO_BULK_MAX],
    pub __packed: },
// Normal Rx CAN Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_rx_can_msg {
    pub timestamp: __le64,
    pub /: *mut *mut u8 rx_type; / type enum es581_4_rx_type,
    pub /: *mut *mut u8 flags; / type enum es58x_flag,
    pub channel_no: u8,
    pub dlc: u8,
    pub can_id: __le32,
    pub data: [u8; CAN_MAX_DLEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_rx_err_msg {
    pub timestamp: __le64,
    pub /: *mut *mut __le16 rx_type; / type enum es581_4_rx_type,
    pub /: *mut *mut __le16 flags; / type enum es58x_flag,
    pub channel_no: u8,
    pub __padding: [u8; 2],
    pub dlc: u8,
    pub /: *mut *mut __le32 tag; / Related to the CAN filtering. Unused in this module,
    pub can_id: __le32,
    pub /: *mut *mut __le32 error; / type enum es58x_error,
    pub /: *mut *mut __le32 destination; / Unused in this module,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_rx_event_msg {
    pub timestamp: __le64,
    pub /: *mut *mut __le16 rx_type; / type enum es581_4_rx_type,
    pub channel_no: u8,
    pub __padding: u8,
    pub /: *mut *mut __le32 tag; / Related to the CAN filtering. Unused in this module,
    pub /: *mut *mut __le32 event; / type enum es58x_event,
    pub /: *mut *mut __le32 destination; / Unused in this module,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_tx_ack_msg {
    pub /: *mut *mut __le16 tx_free_entries; / Number of remaining free entries in the device TX queue,
    pub channel_no: u8,
    pub /: *mut *mut u8 rx_cmd_ret_u8; / type enum es58x_cmd_ret_code_u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_rx_cmd_ret {
    pub rx_cmd_ret_le32: __le32,
    pub channel_no: u8,
    pub __padding: [u8; 3],
    pub __packed: },
//
// struct es581_4_urb_cmd - Commands received from or sent to the
// ES581.4 device.
// @SOF: Start of Frame.
// @cmd_type: Command Type (type: enum es581_4_cmd_type). The CRC
// calculation starts at this position.
// @cmd_id: Command ID (type: enum es581_4_cmd_id).
// @msg_len: Length of the message, excluding CRC (i.e. length of the
// union).
// @tx_conf_msg: Channel configuration.
// @bulk_tx_can_msg: Tx messages.
// @rx_can_msg: Array of Rx messages.
// @bulk_echo_msg: Tx message being looped back.
// @rx_err_msg: Error message.
// @rx_event_msg: Event message.
// @tx_ack_msg: Tx acknowledgment message.
// @rx_cmd_ret: Command return code.
// @timestamp: Timestamp reply.
// @rx_cmd_ret_u8: Rx 8 bits return code (type: enum
// es58x_cmd_ret_code_u8).
// @raw_msg: Message raw payload.
// @reserved_for_crc16_do_not_use: The structure ends with a
// CRC16. Because the structures in above union are of variable
// lengths, we can not predict the offset of the CRC in
// advance. Use functions es58x_get_crc() and es58x_set_crc() to
// manipulate it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es581_4_urb_cmd {
    pub SOF: __le16,
    pub cmd_type: u8,
    pub cmd_id: u8,
    pub msg_len: __le16,
    pub tx_conf_msg: es581_4_tx_conf_msg,
    pub bulk_tx_can_msg: es581_4_bulk_tx_can_msg,
    pub rx_can_msg: [es581_4_rx_can_msg; ES581_4_RX_BULK_MAX],
    pub bulk_echo_msg: es581_4_bulk_echo_msg,
    pub rx_err_msg: es581_4_rx_err_msg,
    pub rx_event_msg: es581_4_rx_event_msg,
    pub tx_ack_msg: es581_4_tx_ack_msg,
    pub rx_cmd_ret: es581_4_rx_cmd_ret,
    pub timestamp: __le64,
    pub rx_cmd_ret_u8: u8,
    pub raw_msg): DECLARE_FLEX_ARRAY(u8,,
    pub __packed: },
    pub reserved_for_crc16_do_not_use: __le16,
    pub __packed: },

