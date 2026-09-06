//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/usb/etas_es58x/es58x_fd.h
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
// File es58x_fd.h: Definitions and declarations specific to ETAS
// ES582.1 and ES584.1 (naming convention: we use the term "ES58X FD"
// when referring to those two variants together).
//
// Copyright (c) 2019 Robert Bosch Engineering and Business Solutions. All rights reserved.
// Copyright (c) 2020 ETAS K.K.. All rights reserved.
// Copyright (c) 2020, 2021 Vincent Mailhol <mailhol.vincent@wanadoo.fr>
//

pub const ES582_1_NUM_CAN_CH: c_int = 2;
pub const ES584_1_NUM_CAN_CH: c_int = 1;
pub const ES58X_FD_NUM_CAN_CH: c_int = 2;
pub const ES58X_FD_CHANNEL_IDX_OFFSET: c_int = 0;
pub const ES58X_FD_TX_BULK_MAX: c_int = 100;
pub const ES58X_FD_RX_BULK_MAX: c_int = 100;
pub const ES58X_FD_ECHO_BULK_MAX: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_fd_cmd_type {
    ES58X_FD_CMD_TYPE_CAN = 0x03,
    ES58X_FD_CMD_TYPE_CANFD = 0x04,
    ES58X_FD_CMD_TYPE_DEVICE = 0xFF
}

// Command IDs for ES58X_FD_CMD_TYPE_{CAN,CANFD}.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_fd_can_cmd_id {
    ES58X_FD_CAN_CMD_ID_ENABLE_CHANNEL = 0x01,
    ES58X_FD_CAN_CMD_ID_DISABLE_CHANNEL = 0x02,
    ES58X_FD_CAN_CMD_ID_TX_MSG = 0x05,
    ES58X_FD_CAN_CMD_ID_ECHO_MSG = 0x07,
    ES58X_FD_CAN_CMD_ID_RX_MSG = 0x10,
    ES58X_FD_CAN_CMD_ID_ERROR_OR_EVENT_MSG = 0x11,
    ES58X_FD_CAN_CMD_ID_RESET_RX = 0x20,
    ES58X_FD_CAN_CMD_ID_RESET_TX = 0x21,
    ES58X_FD_CAN_CMD_ID_TX_MSG_NO_ACK = 0x55
}

// Command IDs for ES58X_FD_CMD_TYPE_DEVICE.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_fd_dev_cmd_id {
    ES58X_FD_DEV_CMD_ID_GETTIMETICKS = 0x01,
    ES58X_FD_DEV_CMD_ID_TIMESTAMP = 0x02
}

//
// enum es58x_fd_ctrlmode - Controller mode.
// @ES58X_FD_CTRLMODE_ACTIVE: send and receive messages.
// @ES58X_FD_CTRLMODE_PASSIVE: only receive messages (monitor). Do not
// send anything, not even the acknowledgment bit.
// @ES58X_FD_CTRLMODE_FD: CAN FD according to ISO11898-1.
// @ES58X_FD_CTRLMODE_FD_NON_ISO: follow Bosch CAN FD Specification
// V1.0
// @ES58X_FD_CTRLMODE_DISABLE_PROTOCOL_EXCEPTION_HANDLING: How to
// behave when CAN FD reserved bit is monitored as
// dominant. (c.f. ISO 11898-1:2015, section 10.4.2.4 "Control
// field", paragraph "r0 bit"). 0 (not disable = enable): send
// error frame. 1 (disable): goes into bus integration mode
// (c.f. below).
// @ES58X_FD_CTRLMODE_EDGE_FILTER_DURING_BUS_INTEGRATION: 0: Edge
// filtering is disabled. 1: Edge filtering is enabled. Two
// consecutive dominant bits required to detect an edge for hard
// synchronization.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_fd_ctrlmode {
    ES58X_FD_CTRLMODE_ACTIVE = 0,
    ES58X_FD_CTRLMODE_PASSIVE = BIT(0),
    ES58X_FD_CTRLMODE_FD = BIT(4),
    ES58X_FD_CTRLMODE_FD_NON_ISO = BIT(5),
    ES58X_FD_CTRLMODE_DISABLE_PROTOCOL_EXCEPTION_HANDLING = BIT(6),
    ES58X_FD_CTRLMODE_EDGE_FILTER_DURING_BUS_INTEGRATION = BIT(7)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_bittiming {
    pub bitrate: __le32,
    pub /: *mut *mut __le16 tseg1; / range: [tseg1_min-1..tseg1_max-1],
    pub /: *mut *mut __le16 tseg2; / range: [tseg2_min-1..tseg2_max-1],
    pub /: *mut *mut __le16 brp; / range: [brp_min-1..brp_max-1],
    pub /: *mut *mut __le16 sjw; / range: [0..sjw_max-1],
    pub __packed: },
//
// struct es58x_fd_tx_conf_msg - Channel configuration.
// @nominal_bittiming: Nominal bittiming.
// @samples_per_bit: type enum es58x_samples_per_bit.
// @sync_edge: type enum es58x_sync_edge.
// @physical_layer: type enum es58x_physical_layer.
// @echo_mode: type enum es58x_echo_mode.
// @ctrlmode: type enum es58x_fd_ctrlmode.
// @canfd_enabled: boolean (0: Classical CAN, 1: CAN and/or CANFD).
// @data_bittiming: Bittiming for flexible data-rate transmission.
// @tdc_enabled: Transmitter Delay Compensation switch (0: TDC is
// disabled, 1: TDC is enabled).
// @tdco: Transmitter Delay Compensation Offset.
// @tdcf: Transmitter Delay Compensation Filter window.
//
// Please refer to the microcontroller datasheet: "SAM E70/S70/V70/V71
// Family" section 49 "Controller Area Network (MCAN)" for additional
// information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_tx_conf_msg {
    pub nominal_bittiming: es58x_fd_bittiming,
    pub samples_per_bit: u8,
    pub sync_edge: u8,
    pub physical_layer: u8,
    pub echo_mode: u8,
    pub ctrlmode: u8,
    pub canfd_enabled: u8,
    pub data_bittiming: es58x_fd_bittiming,
    pub tdc_enabled: u8,
    pub tdco: __le16,
    pub tdcf: __le16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_tx_can_msg {
    pub packet_idx: u8,
    pub can_id: __le32,
    pub flags: u8,
    pub /: *mut *mut u8 dlc; / Only if cmd_id is ES58X_FD_CMD_TYPE_CAN,
    pub /: *mut *mut u8 len; / Only if cmd_id is ES58X_FD_CMD_TYPE_CANFD,
    pub __packed: },
    pub data: [u8; CANFD_MAX_DLEN],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_rx_can_msg {
    pub timestamp: __le64,
    pub can_id: __le32,
    pub flags: u8,
    pub /: *mut *mut u8 dlc; / Only if cmd_id is ES58X_FD_CMD_TYPE_CAN,
    pub /: *mut *mut u8 len; / Only if cmd_id is ES58X_FD_CMD_TYPE_CANFD,
    pub __packed: },
    pub data: [u8; CANFD_MAX_DLEN],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_echo_msg {
    pub timestamp: __le64,
    pub packet_idx: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_rx_event_msg {
    pub timestamp: __le64,
    pub can_id: __le32,
    pub /: *mut *mut u8 flags; / type enum es58x_flag,
    pub /: *mut *mut u8 error_type; / 0: event, 1: error,
    pub error_code: u8,
    pub event_code: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_tx_ack_msg {
    pub /: *mut *mut __le32 rx_cmd_ret_le32; / type enum es58x_cmd_ret_code_u32,
    pub /: *mut *mut __le16 tx_free_entries; / Number of remaining free entries in the device TX queue,
    pub __packed: },
//
// struct es58x_fd_urb_cmd - Commands received from or sent to the
// ES58X FD device.
// @SOF: Start of Frame.
// @cmd_type: Command Type (type: enum es58x_fd_cmd_type). The CRC
// calculation starts at this position.
// @cmd_id: Command ID (type: enum es58x_fd_cmd_id).
// @channel_idx: Channel index starting at 0.
// @msg_len: Length of the message, excluding CRC (i.e. length of the
// union).
// @tx_conf_msg: Channel configuration.
// @tx_can_msg_buf: Concatenation of Tx messages. Type is "u8[]"
// instead of "struct es58x_fd_tx_msg[]" because the structure
// has a flexible size.
// @rx_can_msg_buf: Concatenation Rx messages. Type is "u8[]" instead
// of "struct es58x_fd_rx_msg[]" because the structure has a
// flexible size.
// @echo_msg: Array of echo messages (e.g. Tx messages being looped
// back).
// @rx_event_msg: Error or event message.
// @tx_ack_msg: Tx acknowledgment message.
// @timestamp: Timestamp reply.
// @rx_cmd_ret_le32: Rx 32 bits return code (type: enum
// es58x_cmd_ret_code_u32).
// @raw_msg: Message raw payload.
// @reserved_for_crc16_do_not_use: The structure ends with a
// CRC16. Because the structures in above union are of variable
// lengths, we can not predict the offset of the CRC in
// advance. Use functions es58x_get_crc() and es58x_set_crc() to
// manipulate it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_fd_urb_cmd {
    pub SOF: __le16,
    pub cmd_type: u8,
    pub cmd_id: u8,
    pub channel_idx: u8,
    pub msg_len: __le16,
    pub tx_conf_msg: es58x_fd_tx_conf_msg,
    pub ES58X_FD_CANFD_TX_LEN]: *mut *mut u8 tx_can_msg_buf[ES58X_FD_TX_BULK_MAX,
    pub ES58X_FD_CANFD_RX_LEN]: *mut *mut u8 rx_can_msg_buf[ES58X_FD_RX_BULK_MAX,
    pub echo_msg: [es58x_fd_echo_msg; ES58X_FD_ECHO_BULK_MAX],
    pub rx_event_msg: es58x_fd_rx_event_msg,
    pub tx_ack_msg: es58x_fd_tx_ack_msg,
    pub timestamp: __le64,
    pub rx_cmd_ret_le32: __le32,
    pub raw_msg): DECLARE_FLEX_ARRAY(u8,,
    pub __packed: },
    pub reserved_for_crc16_do_not_use: __le16,
    pub __packed: },

