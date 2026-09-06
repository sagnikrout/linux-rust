//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/usb/etas_es58x/es58x_core.h
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
// File es58x_core.h: All common definitions and declarations.
//
// Copyright (c) 2019 Robert Bosch Engineering and Business Solutions. All rights reserved.
// Copyright (c) 2020 ETAS K.K.. All rights reserved.
// Copyright (c) 2020-2022 Vincent Mailhol <mailhol.vincent@wanadoo.fr>
//

// Driver constants

// Use this when channel index is irrelevant (e.g. device
// timestamp).
//
pub const ES58X_CHANNEL_IDX_NA: c_uint = 0xFF;

// Threshold on consecutive CAN_STATE_ERROR_PASSIVE. If we receive
// ES58X_CONSECUTIVE_ERR_PASSIVE_MAX times the event
// ES58X_ERR_CRTL_PASSIVE in a row without any successful RX or TX,
// we force the device to switch to CAN_STATE_BUS_OFF state.
//
pub const ES58X_CONSECUTIVE_ERR_PASSIVE_MAX: c_int = 254;
// A magic number sent by the ES581.4 to inform it is alive.
pub const ES58X_HEARTBEAT: c_uint = 0x11;
//
// enum es58x_driver_info - Quirks of the device.
// @ES58X_DUAL_CHANNEL: Device has two CAN channels. If this flag is
// not set, it is implied that the device has only one CAN
// channel.
// @ES58X_FD_FAMILY: Device is CAN-FD capable. If this flag is not
// set, the device only supports classical CAN.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_driver_info {
    ES58X_DUAL_CHANNEL = BIT(0),
    ES58X_FD_FAMILY = BIT(1)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_echo {
    ES58X_ECHO_OFF = 0,
    ES58X_ECHO_ON = 1
}

//
// enum es58x_physical_layer - Type of the physical layer.
// @ES58X_PHYSICAL_LAYER_HIGH_SPEED: High-speed CAN (c.f. ISO
// 11898-2).
//
// Some products of the ETAS portfolio also support low-speed CAN
// (c.f. ISO 11898-3). However, all the devices in scope of this
// driver do not support the option, thus, the enum has only one
// member.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_physical_layer {
    ES58X_PHYSICAL_LAYER_HIGH_SPEED = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_samples_per_bit {
    ES58X_SAMPLES_PER_BIT_ONE = 1,
    ES58X_SAMPLES_PER_BIT_THREE = 2
}

//
// enum es58x_sync_edge - Synchronization method.
// @ES58X_SYNC_EDGE_SINGLE: ISO CAN specification defines the use of a
// single edge synchronization.  The synchronization should be
// done on recessive to dominant level change.
//
// For information, ES582.1 and ES584.1 also support a double
// synchronization, requiring both recessive to dominant then dominant
// to recessive level change. However, this is not supported in
// SocketCAN framework, thus, the enum has only one member.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_sync_edge {
    ES58X_SYNC_EDGE_SINGLE = 1
}

//
// enum es58x_flag - CAN flags for RX/TX messages.
// @ES58X_FLAG_EFF: Extended Frame Format (EFF).
// @ES58X_FLAG_RTR: Remote Transmission Request (RTR).
// @ES58X_FLAG_FD_BRS: Bit rate switch (BRS): second bitrate for
// payload data.
// @ES58X_FLAG_FD_ESI: Error State Indicator (ESI): tell if the
// transmitting node is in error passive mode.
// @ES58X_FLAG_FD_DATA: CAN FD frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_flag {
    ES58X_FLAG_EFF = BIT(0),
    ES58X_FLAG_RTR = BIT(1),
    ES58X_FLAG_FD_BRS = BIT(3),
    ES58X_FLAG_FD_ESI = BIT(5),
    ES58X_FLAG_FD_DATA = BIT(6)
}

//
// enum es58x_err - CAN error detection.
// @ES58X_ERR_OK: No errors.
// @ES58X_ERR_PROT_STUFF: Bit stuffing error: more than 5 consecutive
// equal bits.
// @ES58X_ERR_PROT_FORM: Frame format error.
// @ES58X_ERR_ACK: Received no ACK on transmission.
// @ES58X_ERR_PROT_BIT: Single bit error.
// @ES58X_ERR_PROT_CRC: Incorrect 15, 17 or 21 bits CRC.
// @ES58X_ERR_PROT_BIT1: Unable to send recessive bit: tried to send
// recessive bit 1 but monitored dominant bit 0.
// @ES58X_ERR_PROT_BIT0: Unable to send dominant bit: tried to send
// dominant bit 0 but monitored recessive bit 1.
// @ES58X_ERR_PROT_OVERLOAD: Bus overload.
// @ES58X_ERR_PROT_UNSPEC: Unspecified.
//
// Please refer to ISO 11898-1:2015, section 10.11 "Error detection"
// and section 10.13 "Overload signaling" for additional details.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_err {
    ES58X_ERR_OK = 0,
    ES58X_ERR_PROT_STUFF = BIT(0),
    ES58X_ERR_PROT_FORM = BIT(1),
    ES58X_ERR_ACK = BIT(2),
    ES58X_ERR_PROT_BIT = BIT(3),
    ES58X_ERR_PROT_CRC = BIT(4),
    ES58X_ERR_PROT_BIT1 = BIT(5),
    ES58X_ERR_PROT_BIT0 = BIT(6),
    ES58X_ERR_PROT_OVERLOAD = BIT(7),
    ES58X_ERR_PROT_UNSPEC = BIT(31)
}

//
// enum es58x_event - CAN error codes returned by the device.
// @ES58X_EVENT_OK: No errors.
// @ES58X_EVENT_CRTL_ACTIVE: Active state: both TR and RX error count
// is less than 128.
// @ES58X_EVENT_CRTL_PASSIVE: Passive state: either TX or RX error
// count is greater than 127.
// @ES58X_EVENT_CRTL_WARNING: Warning state: either TX or RX error
// count is greater than 96.
// @ES58X_EVENT_BUSOFF: Bus off.
// @ES58X_EVENT_SINGLE_WIRE: Lost connection on either CAN high or CAN
// low.
//
// Please refer to ISO 11898-1:2015, section 12.1.4 "Rules of fault
// confinement" for additional details.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_event {
    ES58X_EVENT_OK = 0,
    ES58X_EVENT_CRTL_ACTIVE = BIT(0),
    ES58X_EVENT_CRTL_PASSIVE = BIT(1),
    ES58X_EVENT_CRTL_WARNING = BIT(2),
    ES58X_EVENT_BUSOFF = BIT(3),
    ES58X_EVENT_SINGLE_WIRE = BIT(4)
}

// enum es58x_ret_u8 - Device return error codes, 8 bit format.
//
// Specific to ES581.4.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_ret_u8 {
    ES58X_RET_U8_OK = 0x00,
    ES58X_RET_U8_ERR_UNSPECIFIED_FAILURE = 0x80,
    ES58X_RET_U8_ERR_NO_MEM = 0x81,
    ES58X_RET_U8_ERR_BAD_CRC = 0x99
}

// enum es58x_ret_u32 - Device return error codes, 32 bit format.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_ret_u32 {
    ES58X_RET_U32_OK = 0x00000000UL,
    ES58X_RET_U32_ERR_UNSPECIFIED_FAILURE = 0x80000000UL,
    ES58X_RET_U32_ERR_NO_MEM = 0x80004001UL,
    ES58X_RET_U32_WARN_PARAM_ADJUSTED = 0x40004000UL,
    ES58X_RET_U32_WARN_TX_MAYBE_REORDER = 0x40004001UL,
    ES58X_RET_U32_ERR_TIMEDOUT = 0x80000008UL,
    ES58X_RET_U32_ERR_FIFO_FULL = 0x80003002UL,
    ES58X_RET_U32_ERR_BAD_CONFIG = 0x80004000UL,
    ES58X_RET_U32_ERR_NO_RESOURCE = 0x80004002UL
}

// enum es58x_ret_type - Type of the command returned by the ES58X
// device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum es58x_ret_type {
    ES58X_RET_TYPE_SET_BITTIMING,
    ES58X_RET_TYPE_ENABLE_CHANNEL,
    ES58X_RET_TYPE_DISABLE_CHANNEL,
    ES58X_RET_TYPE_TX_MSG,
    ES58X_RET_TYPE_RESET_RX,
    ES58X_RET_TYPE_RESET_TX,
    ES58X_RET_TYPE_DEVICE_ERR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union es58x_urb_cmd {
    pub es581_4_urb_cmd: es581_4_urb_cmd,
    pub es58x_fd_urb_cmd: es58x_fd_urb_cmd,
    pub sof: __le16,
    pub cmd_type: u8,
    pub cmd_id: u8,
    pub __packed: },
    pub raw_cmd): DECLARE_FLEX_ARRAY(u8,,
}

//
// struct es58x_priv - All information specific to a CAN channel.
// @can: struct can_priv must be the first member (Socket CAN relies
// on the fact that function netdev_priv() returns a pointer to
// a struct can_priv).
// @devlink_port: devlink instance for the network interface.
// @es58x_dev: pointer to the corresponding ES58X device.
// @tx_urb: Used as a buffer to concatenate the TX messages and to do
// a bulk send. Please refer to es58x_start_xmit() for more
// details.
// @tx_tail: Index of the oldest packet still pending for
// completion. @tx_tail & echo_skb_mask represents the beginning
// of the echo skb FIFO, i.e. index of the first element.
// @tx_head: Index of the next packet to be sent to the
// device. @tx_head & echo_skb_mask represents the end of the
// echo skb FIFO plus one, i.e. the first free index.
// @tx_can_msg_cnt: Number of messages in @tx_urb.
// @tx_can_msg_is_fd: false: all messages in @tx_urb are Classical
// CAN, true: all messages in @tx_urb are CAN FD. Rationale:
// ES58X FD devices do not allow to mix Classical CAN and FD CAN
// frames in one single bulk transmission.
// @err_passive_before_rtx_success: The ES58X device might enter in a
// state in which it keeps alternating between error passive
// and active states. This counter keeps track of the number of
// error passive and if it gets bigger than
// ES58X_CONSECUTIVE_ERR_PASSIVE_MAX, es58x_rx_err_msg() will
// force the status to bus-off.
// @channel_idx: Channel index, starts at zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_priv {
    pub can: can_priv,
    pub devlink_port: devlink_port,
    pub es58x_dev: *mut es58x_device,
    pub tx_urb: *mut urb,
    pub tx_tail: u32,
    pub tx_head: u32,
    pub tx_can_msg_cnt: u8,
    pub tx_can_msg_is_fd: bool,
    pub err_passive_before_rtx_success: u8,
    pub channel_idx: u8,
}

//
// struct es58x_parameters - Constant parameters of a given hardware
// variant.
// @bittiming_const: Nominal bittimming constant parameters.
// @data_bittiming_const: Data bittiming constant parameters.
// @tdc_const: Transmission Delay Compensation constant parameters.
// @bitrate_max: Maximum bitrate supported by the device.
// @clock: CAN clock parameters.
// @ctrlmode_supported: List of supported modes. Please refer to
// can/netlink.h file for additional details.
// @tx_start_of_frame: Magic number at the beginning of each TX URB
// command.
// @rx_start_of_frame: Magic number at the beginning of each RX URB
// command.
// @tx_urb_cmd_max_len: Maximum length of a TX URB command.
// @rx_urb_cmd_max_len: Maximum length of a RX URB command.
// @fifo_mask: Bit mask to quickly convert the tx_tail and tx_head
// field of the struct es58x_priv into echo_skb
// indexes. Properties: @fifo_mask = echo_skb_max - 1 where
// echo_skb_max must be a power of two. Also, echo_skb_max must
// not exceed the maximum size of the device internal TX FIFO
// length. This parameter is used to control the network queue
// wake/stop logic.
// @dql_min_limit: Dynamic Queue Limits (DQL) absolute minimum limit
// of bytes allowed to be queued on this network device transmit
// queue. Used by the Byte Queue Limits (BQL) to determine how
// frequently the xmit_more flag will be set to true in
// es58x_start_xmit(). Set this value higher to optimize for
// throughput but be aware that it might have a negative impact
// on the latency! This value can also be set dynamically. Please
// refer to Documentation/ABI/testing/sysfs-class-net-queues for
// more details.
// @tx_bulk_max: Maximum number of TX messages that can be sent in one
// single URB packet.
// @urb_cmd_header_len: Length of the URB command header.
// @rx_urb_max: Number of RX URB to be allocated during device probe.
// @tx_urb_max: Number of TX URB to be allocated during device probe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_parameters {
    pub bittiming_const: *const can_bittiming_,
    pub data_bittiming_const: *const can_bittiming_,
    pub tdc_const: *const can_tdc_,
    pub bitrate_max: u32,
    pub clock: can_clock,
    pub ctrlmode_supported: u32,
    pub tx_start_of_frame: u16,
    pub rx_start_of_frame: u16,
    pub tx_urb_cmd_max_len: u16,
    pub rx_urb_cmd_max_len: u16,
    pub fifo_mask: u16,
    pub dql_min_limit: u16,
    pub tx_bulk_max: u8,
    pub urb_cmd_header_len: u8,
    pub rx_urb_max: u8,
    pub tx_urb_max: u8,
}

//
// struct es58x_operators - Function pointers used to encode/decode
// the TX/RX messages.
// @get_msg_len: Get field msg_len of the urb_cmd. The offset of
// msg_len inside urb_cmd depends of the device model.
// @handle_urb_cmd: Decode the URB command received from the device
// and dispatch it to the relevant sub function.
// @fill_urb_header: Fill the header of urb_cmd.
// @tx_can_msg: Encode a TX CAN message and add it to the bulk buffer
// cmd_buf of es58x_dev.
// @enable_channel: Start the CAN channel.
// @disable_channel: Stop the CAN channel.
// @reset_device: Full reset of the device. N.B: this feature is only
// present on the ES581.4. For ES58X FD devices, this field is
// set to NULL.
// @get_timestamp: Request a timestamp from the ES58X device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_operators {
    pub urb_cmd): *const *const u16 (get_msg_len)(union es58x_urb_cmd,
    pub urb_cmd): *const es58x_urb_cmd,
    pub cmd_len): u8 cmd_id, u8 channel_idx, u16,
    pub skb): *const *const *const int (tx_can_msg)(struct es58x_priv priv, struct sk_buff,
    pub priv): *mut *mut int (enable_channel)(struct es58x_priv,
    pub priv): *mut *mut int (disable_channel)(struct es58x_priv,
    pub es58x_dev): *mut *mut int (reset_device)(struct es58x_device,
    pub es58x_dev): *mut *mut int (get_timestamp)(struct es58x_device,
}

//
// struct es58x_sw_version - Version number of the firmware or the
// bootloader.
// @major: Version major number, represented on two digits.
// @minor: Version minor number, represented on two digits.
// @revision: Version revision number, represented on two digits.
//
// The firmware and the bootloader share the same format: "xx.xx.xx"
// where 'x' is a digit. Both can be retrieved from the product
// information string.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_sw_version {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

//
// struct es58x_hw_revision - Hardware revision number.
// @letter: Revision letter, an alphanumeric character.
// @major: Version major number, represented on three digits.
// @minor: Version minor number, represented on three digits.
//
// The hardware revision uses its own format: "axxx/xxx" where 'a' is
// an alphanumeric character and 'x' a digit. It can be retrieved from
// the product information string.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_hw_revision {
    pub letter: c_char,
    pub major: u16,
    pub minor: u16,
}

//
// struct es58x_device - All information specific to an ES58X device.
// @dev: Device information.
// @udev: USB device information.
// @netdev: Array of our CAN channels.
// @param: The constant parameters.
// @ops: Operators.
// @rx_pipe: USB reception pipe.
// @tx_pipe: USB transmission pipe.
// @rx_urbs: Anchor for received URBs.
// @tx_urbs_busy: Anchor for TX URBs which were send to the device.
// @tx_urbs_idle: Anchor for TX USB which are idle. This driver
// allocates the memory for the URBs during the probe. When a TX
// URB is needed, it can be taken from this anchor. The network
// queue wake/stop logic should prevent this URB from getting
// empty. Please refer to es58x_get_tx_urb() for more details.
// @tx_urbs_idle_cnt: number of urbs in @tx_urbs_idle.
// @firmware_version: The firmware version number.
// @bootloader_version: The bootloader version number.
// @hardware_revision: The hardware revision number.
// @ktime_req_ns: kernel timestamp when es58x_set_realtime_diff_ns()
// was called.
// @realtime_diff_ns: difference in nanoseconds between the clocks of
// the ES58X device and the kernel.
// @timestamps: a temporary buffer to store the time stamps before
// feeding them to es58x_can_get_echo_skb(). Can only be used
// in RX branches.
// @num_can_ch: Number of CAN channel (i.e. number of elements of @netdev).
// @opened_channel_cnt: number of channels opened. Free of race
// conditions because its two users (net_device_ops:ndo_open()
// and net_device_ops:ndo_close()) guarantee that the network
// stack big kernel lock (a.k.a. rtnl_mutex) is being hold.
// @rx_cmd_buf_len: Length of @rx_cmd_buf.
// @rx_cmd_buf: The device might split the URB commands in an
// arbitrary amount of pieces. This buffer is used to concatenate
// all those pieces. Can only be used in RX branches. This field
// has to be the last one of the structure because it is has a
// flexible size (c.f. es58x_sizeof_es58x_device() function).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es58x_device {
    pub dev: *mut device,
    pub udev: *mut usb_device,
    pub netdev: [*mut net_device; ES58X_NUM_CAN_CH_MAX],
    pub param: *const es58x_parameters,
    pub ops: *const es58x_operators,
    pub rx_pipe: c_uint,
    pub tx_pipe: c_uint,
    pub rx_urbs: usb_anchor,
    pub tx_urbs_busy: usb_anchor,
    pub tx_urbs_idle: usb_anchor,
    pub tx_urbs_idle_cnt: core::sync::atomic::AtomicI32,
    pub firmware_version: es58x_sw_version,
    pub bootloader_version: es58x_sw_version,
    pub hardware_revision: es58x_hw_revision,
    pub ktime_req_ns: u64,
    pub realtime_diff_ns: i64,
    pub timestamps: [u64; ES58X_ECHO_BULK_MAX],
    pub num_can_ch: u8,
    pub opened_channel_cnt: u8,
    pub rx_cmd_buf_len: u16,
    pub rx_cmd_buf: es58x_urb_cmd,
}

//
// es58x_sizeof_es58x_device() - Calculate the maximum length of
// struct es58x_device.
// @es58x_dev_param: The constant parameters of the device.
//
// The length of struct es58x_device depends on the length of its last
// field: rx_cmd_buf. This macro allows to optimize the memory
// allocation.
//
// Return: length of struct es58x_device.
//
// es58x_dev_param)
//
// es58x_check_msg_len() - Check the size of a received message.
// @dev: Device, used to print error messages.
// @msg: Received message, must not be a pointer.
// @actual_len: Length of the message as advertised in the command header.
//
// Must be a macro in order to accept the different types of messages
// as an input. Can be use with any of the messages which have a fixed
// length. Check for an exact match of the size.
//
// Return: zero on success, -EMSGSIZE if @actual_len differs from the
// expected length.
//

//
// es58x_check_msg_max_len() - Check the maximum size of a received message.
// @dev: Device, used to print error messages.
// @msg: Received message, must not be a pointer.
// @actual_len: Length of the message as advertised in the command header.
//
// Must be a macro in order to accept the different types of messages
// as an input. To be used with the messages of variable sizes. Only
// check that the message is not bigger than the maximum expected
// size.
//
// Return: zero on success, -EOVERFLOW if @actual_len is greater than
// the expected length.
//

//
// es58x_msg_num_element() - Check size and give the number of
// elements in a message of array type.
// @dev: Device, used to print error messages.
// @msg: Received message, must be an array.
// @actual_len: Length of the message as advertised in the command
// header.
//
// Must be a macro in order to accept the different types of messages
// as an input. To be used on message of array type. Array's element
// has to be of fixed size (else use es58x_check_msg_max_len()). Check
// that the total length is an exact multiple of the length of a
// single element.
//
// Return: number of elements in the array on success, -EOVERFLOW if
// @actual_len is greater than the expected length, -EMSGSIZE if
// @actual_len is not a multiple of a single element.
//

//
// es58x_priv() - Get the priv member and cast it to struct es58x_priv.
// @netdev: CAN network device.
//
// Return: ES58X device.
//
// ES58X_SIZEOF_URB_CMD() - Calculate the maximum length of an urb
// command for a given message field name.
// @es58x_urb_cmd_type: type (either "struct es581_4_urb_cmd" or
// "struct es58x_fd_urb_cmd").
// @msg_field: name of the message field.
//
// Must be a macro in order to accept the different command types as
// an input.
//
// Return: length of the urb command.
//

//
// es58x_get_urb_cmd_len() - Calculate the actual length of an urb
// command for a given message length.
// @es58x_dev: ES58X device.
// @msg_len: Length of the message.
//
// Add the header and CRC lengths to the message length.
//
// Return: length of the urb command.
//
// es58x_get_netdev() - Get the network device.
// @es58x_dev: ES58X device.
// @channel_no: The channel number as advertised in the urb command.
// @channel_idx_offset: Some of the ES58x starts channel numbering
// from 0 (ES58X FD), others from 1 (ES581.4).
// @netdev: CAN network device.
//
// Do a sanity check on the index provided by the device.
//
// Return: zero on success, -ECHRNG if the received channel number is
// out of range and -ENODEV if the network device is not yet
// configured.
//
// netdev = NULL;
// netdev = es58x_dev->netdev[channel_idx];
//
// es58x_get_raw_can_id() - Get the CAN ID.
// @cf: CAN frame.
//
// Mask the CAN ID in order to only keep the significant bits.
//
// Return: the raw value of the CAN ID.
//
// es58x_get_flags() - Get the CAN flags.
// @skb: socket buffer of a CAN message.
//
// Return: the CAN flag as an enum es58x_flag.
//
// Remote frames are only defined in Classical CAN frames
// es58x_core.c.
extern "C" {
    pub fn es58x_rx_timestamp(es58x_dev: *mut es58x_device, timestamp: u64);
}
// es58x_devlink.c.
extern "C" {
    pub fn es58x_parse_product_info(es58x_dev: *mut es58x_device);
}
// es581_4.c.
// es58x_fd.c.
