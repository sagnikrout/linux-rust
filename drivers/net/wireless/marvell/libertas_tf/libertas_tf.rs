//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas_tf/libertas_tf.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2008, cozybit Inc.
// Copyright (C) 2007, Red Hat, Inc.
// Copyright (C) 2003-2006, Marvell International Ltd.
//

pub const MRVL_DEFAULT_RETRIES: c_int = 9;
pub const MRVL_PER_PACKET_RATE: c_uint = 0x10;
pub const MRVL_MAX_BCN_SIZE: c_int = 440;
pub const CMD_OPTION_WAITFORRSP: c_uint = 0x0002;
// Return command are almost always the same as the host command, but with
// bit 15 set high.  There are a few exceptions, though...
//

// Command codes
pub const CMD_GET_HW_SPEC: c_uint = 0x0003;
pub const CMD_802_11_RESET: c_uint = 0x0005;
pub const CMD_MAC_MULTICAST_ADR: c_uint = 0x0010;
pub const CMD_802_11_RADIO_CONTROL: c_uint = 0x001c;
pub const CMD_802_11_RF_CHANNEL: c_uint = 0x001d;
pub const CMD_802_11_RF_TX_POWER: c_uint = 0x001e;
pub const CMD_MAC_CONTROL: c_uint = 0x0028;
pub const CMD_802_11_MAC_ADDRESS: c_uint = 0x004d;
pub const CMD_SET_BOOT2_VER: c_uint = 0x00a5;
pub const CMD_802_11_BEACON_CTRL: c_uint = 0x00b0;
pub const CMD_802_11_BEACON_SET: c_uint = 0x00cb;
pub const CMD_802_11_SET_MODE: c_uint = 0x00cc;
pub const CMD_802_11_SET_BSSID: c_uint = 0x00cd;
pub const CMD_ACT_GET: c_uint = 0x0000;
pub const CMD_ACT_SET: c_uint = 0x0001;
// Define action or option for CMD_802_11_RESET
pub const CMD_ACT_HALT: c_uint = 0x0003;
// Define action or option for CMD_MAC_CONTROL
pub const CMD_ACT_MAC_RX_ON: c_uint = 0x0001;
pub const CMD_ACT_MAC_TX_ON: c_uint = 0x0002;
pub const CMD_ACT_MAC_MULTICAST_ENABLE: c_uint = 0x0020;
pub const CMD_ACT_MAC_BROADCAST_ENABLE: c_uint = 0x0040;
pub const CMD_ACT_MAC_PROMISCUOUS_ENABLE: c_uint = 0x0080;
pub const CMD_ACT_MAC_ALL_MULTICAST_ENABLE: c_uint = 0x0100;
// Define action or option for CMD_802_11_RADIO_CONTROL
pub const CMD_TYPE_AUTO_PREAMBLE: c_uint = 0x0001;
pub const CMD_TYPE_SHORT_PREAMBLE: c_uint = 0x0002;
pub const CMD_TYPE_LONG_PREAMBLE: c_uint = 0x0003;
pub const TURN_ON_RF: c_uint = 0x01;
pub const RADIO_ON: c_uint = 0x01;
pub const RADIO_OFF: c_uint = 0x00;
pub const SET_AUTO_PREAMBLE: c_uint = 0x05;
pub const SET_SHORT_PREAMBLE: c_uint = 0x03;
pub const SET_LONG_PREAMBLE: c_uint = 0x01;
// Define action or option for CMD_802_11_RF_CHANNEL
pub const CMD_OPT_802_11_RF_CHANNEL_GET: c_uint = 0x00;
pub const CMD_OPT_802_11_RF_CHANNEL_SET: c_uint = 0x01;
// Codes for CMD_802_11_SET_MODE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lbtf_mode {
    LBTF_PASSIVE_MODE,
    LBTF_STA_MODE,
    LBTF_AP_MODE,
}

// Card Event definition
pub const MACREG_INT_CODE_FIRMWARE_READY: c_int = 48;
// Buffer Constants
// The size of SQ memory PPA, DPA are 8 DWORDs, that keep the physical
// addresses of TxPD buffers. Station has only 8 TxPD available, Whereas
// driver has more local TxPDs. Each TxPD on the host memory is associated
// with a Tx control node. The driver maintains 8 RxPD descriptors for
// station firmware to store Rx packet information.
//
// Current version of MAC has a 32x6 multicast address buffer.
//
// 802.11b can have up to  14 channels, the driver keeps the
// BSSID(MAC address) of each APs or Ad hoc stations it has sensed.
//
pub const MRVDRV_MAX_MULTICAST_LIST_SIZE: c_int = 32;
pub const LBS_NUM_CMD_BUFFERS: c_int = 10;

pub const MRVDRV_MAX_CHANNEL_SIZE: c_int = 14;
pub const MRVDRV_SNAP_HEADER_LEN: c_int = 8;
pub const LBS_UPLD_SIZE: c_int = 2312;
pub const DEV_NAME_LEN: c_int = 32;
// Misc constants
// This section defines 802.11 specific contants
pub const MRVDRV_MAX_REGION_CODE: c_int = 6;
//
// the table to keep region code
//
pub const LBTF_REGDOMAIN_US: c_uint = 0x10;
pub const LBTF_REGDOMAIN_CA: c_uint = 0x20;
pub const LBTF_REGDOMAIN_EU: c_uint = 0x30;
pub const LBTF_REGDOMAIN_SP: c_uint = 0x31;
pub const LBTF_REGDOMAIN_FR: c_uint = 0x32;
pub const LBTF_REGDOMAIN_JP: c_uint = 0x40;
pub const SBI_EVENT_CAUSE_SHIFT: c_int = 3;
// RxPD status
pub const MRVDRV_RXPD_STATUS_OK: c_uint = 0x0001;
// This is for firmware specific length
pub const EXTRA_LEN: c_int = 36;

pub const RF_ANTENNA_1: c_uint = 0x1;
pub const RF_ANTENNA_2: c_uint = 0x2;
pub const RF_ANTENNA_AUTO: c_uint = 0xFFFF;
pub const LBTF_EVENT_BCN_SENT: c_int = 55;
// Global Variable Declaration
// mv_ms_type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv_ms_type {
    MVMS_DAT = 0,
    MVMS_CMD = 1,
    MVMS_TXDONE = 2,
    MVMS_EVENT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbtf_offset_value {
    pub offset: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_range {
    pub regdomain: u8,
    pub start: u8,
    pub /: *mut *mut u8 end; / exclusive (channel must be less than end),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbtf_ops {
// Hardware access
    pub nb): *mut *mut u8 payload, u16,
    pub priv): *mut *mut int (hw_prog_firmware)(struct lbtf_private,
    pub priv): *mut *mut int (hw_reset_device)(struct lbtf_private,
}

// Private structure for the MV device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbtf_private {
    pub card: *mut c_void,
    pub hw: *mut ieee80211_hw,
    pub ops: *const lbtf_ops,
// Command response buffer
    pub cmd_resp_buff: [u8; LBS_UPLD_SIZE],
// Download sent:
    pub vif: *mut ieee80211_vif,
    pub cmd_work: work_struct,
    pub tx_work: work_struct,
// Wlan adapter data structure
// STATUS variables
    pub fwrelease: u32,
    pub fwcapinfo: u32,
// protected with big lock
    pub lock: mutex,
// command-related variables
    pub seqnum: u16,
// protected by big lock
    pub cmd_array: *mut cmd_ctrl_node,
// Current command
    pub cur_cmd: *mut cmd_ctrl_node,
// command Queues
// Free command buffers
    pub cmdfreeq: list_head,
// Pending command buffers
    pub cmdpendingq: list_head,
// spin locks
    pub driver_lock: spinlock_t,
// Timers
    pub command_timer: timer_list,
    pub nr_retries: c_int,
    pub cmd_timed_out: c_int,
    pub cmd_response_rxed: u8,
// capability Info used in Association, start, join
    pub capability: u16,
// MAC address information
    pub current_addr: [u8; ETH_ALEN],
    pub multicastlist: [u8; MRVDRV_MAX_MULTICAST_LIST_SIZE][ETH_ALEN],
    pub nr_of_multicastmacaddr: u32,
    pub cur_freq: c_int,
    pub skb_to_tx: *mut sk_buff,
    pub tx_skb: *mut sk_buff,
// NIC Operation characteristics
    pub mac_control: u16,
    pub regioncode: u16,
    pub range: channel_range,
    pub radioon: u8,
    pub preamble: u32,
    pub channels: [ieee80211_channel; 14],
    pub rates: [ieee80211_rate; 12],
    pub band: ieee80211_supported_band,
    pub offsetvalue: lbtf_offset_value,
    pub surpriseremoved: u8,
    pub bc_ps_buf: sk_buff_head,
// Most recently reported noise in dBm
    pub noise: i8,
}

// 802.11-related definitions
// TxPD descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpd {
// Current Tx packet status
    pub tx_status: __le32,
// Tx control
    pub tx_control: __le32,
    pub tx_packet_location: __le32,
// Tx packet length
    pub tx_packet_length: __le16,
// First 2 byte of destination MAC address
    pub tx_dest_addr_high: [u8; 2],
// Last 4 byte of destination MAC address
    pub tx_dest_addr_low: [u8; 4],
// Pkt Priority
    pub priority: u8,
// Pkt Trasnit Power control
    pub powermgmt: u8,
// Time the packet has been queued in the driver (units = 2ms)
    pub pktdelay_2ms: u8,
// reserved
    pub reserved1: u8,
    pub __packed: },
// RxPD Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxpd {
// Current Rx packet status
    pub status: __le16,
// SNR
    pub snr: u8,
// Tx control
    pub rx_control: u8,
// Pkt length
    pub pkt_len: __le16,
// Noise Floor
    pub nf: u8,
// Rx Packet Rate
    pub rx_rate: u8,
// Pkt addr
    pub pkt_ptr: __le32,
// Next Rx RxPD addr
    pub next_rxpd_ptr: __le32,
// Pkt Priority
    pub priority: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_header {
    pub command: __le16,
    pub size: __le16,
    pub seqnum: __le16,
    pub result: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ctrl_node {
    pub list: list_head,
    pub result: c_int,
// command response
    pub ): *mut unsigned long, struct cmd_header,
    pub callback_arg: c_ulong,
// command data
    pub cmdbuf: *mut cmd_header,
// wait queue
    pub cmdwaitqwoken: u16,
    pub cmdwait_q: wait_queue_head_t,
}

//
// Define data structure for CMD_GET_HW_SPEC
// This structure defines the response for the GET_HW_SPEC command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_get_hw_spec {
    pub hdr: cmd_header,
// HW Interface version number
    pub hwifversion: __le16,
// HW version number
    pub version: __le16,
// Max number of TxPD FW can handle
    pub nr_txpd: __le16,
// Max no of Multicast address
    pub nr_mcast_adr: __le16,
// MAC address
    pub permanentaddr: [u8; 6],
// region Code
    pub regioncode: __le16,
// Number of antenna used
    pub nr_antenna: __le16,
// FW release number, example 0x01030304 = 2.3.4p1
    pub fwrelease: __le32,
// Base Address of TxPD queue
    pub wcb_base: __le32,
// Read Pointer of RxPd queue
    pub rxpd_rdptr: __le32,
// Write Pointer of RxPd queue
    pub rxpd_wrptr: __le32,
// FW/HW capability
    pub fwcapinfo: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_mac_control {
    pub hdr: cmd_header,
    pub action: __le16,
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_mac_address {
    pub hdr: cmd_header,
    pub action: __le16,
    pub macadd: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_mac_multicast_addr {
    pub hdr: cmd_header,
    pub action: __le16,
    pub nr_of_adrs: __le16,
    pub MRVDRV_MAX_MULTICAST_LIST_SIZE]: *mut *mut u8 maclist[ETH_ALEN,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_set_mode {
    pub hdr: cmd_header,
    pub mode: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_set_bssid {
    pub hdr: cmd_header,
    pub bssid: [u8; 6],
    pub activate: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_radio_control {
    pub hdr: cmd_header,
    pub action: __le16,
    pub control: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_rf_channel {
    pub hdr: cmd_header,
    pub action: __le16,
    pub channel: __le16,
    pub /: *mut *mut __le16 rftype; / unused,
    pub /: *mut *mut __le16 reserved; / unused,
    pub /: *mut *mut u8 channellist[32]; / unused,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_set_boot2_ver {
    pub hdr: cmd_header,
    pub action: __le16,
    pub version: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_reset {
    pub hdr: cmd_header,
    pub action: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_beacon_control {
    pub hdr: cmd_header,
    pub action: __le16,
    pub beacon_enable: __le16,
    pub beacon_period: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_beacon_set {
    pub hdr: cmd_header,
    pub len: __le16,
    pub beacon: [u8; MRVL_MAX_BCN_SIZE],
    pub __packed: },
    pub cmd_ctrl_node: struct,
// Function Prototype Declaration
    pub priv): *mut void lbtf_set_mac_control(struct lbtf_private,
    pub priv): *mut int lbtf_free_cmd_buffer(struct lbtf_private,
    pub priv): *mut int lbtf_allocate_cmd_buffer(struct lbtf_private,
    pub priv): *mut int lbtf_execute_next_command(struct lbtf_private,
    pub priv): *mut int lbtf_set_radio_control(struct lbtf_private,
    pub priv): *mut int lbtf_update_hw_spec(struct lbtf_private,
    pub priv): *mut int lbtf_cmd_set_mac_multicast_addr(struct lbtf_private,
    pub mode): *mut *mut void lbtf_set_mode(struct lbtf_private priv, enum lbtf_mode,
    pub bssid): *const *const void lbtf_set_bssid(struct lbtf_private priv, bool activate, u8,
    pub mac_addr): *mut *mut int lbtf_set_mac_address(struct lbtf_private priv, uint8_t,
    pub channel): *mut *mut int lbtf_set_channel(struct lbtf_private priv, u8,
    pub beacon): *mut *mut int lbtf_beacon_set(struct lbtf_private priv, struct sk_buff,
    pub beacon_int): c_int,
    pub priv): *mut int lbtf_process_rx_command(struct lbtf_private,
    pub result): c_int,
    pub priv): *mut void lbtf_cmd_response_rx(struct lbtf_private,
// main.c
    pub ops): *const lbtf_ops,
    pub priv): *mut int lbtf_remove_card(struct lbtf_private,
    pub skb): *mut *mut int lbtf_rx(struct lbtf_private priv, struct sk_buff,
    pub fail): *mut *mut void lbtf_send_tx_feedback(struct lbtf_private priv, u8 retrycnt, u8,
    pub priv): *mut void lbtf_bcn_sent(struct lbtf_private,
// support functions for cmd.c
// lbtf_cmd() infers the size of the buffer to copy data back into, from

    pub \: uint16_t __sz = le16_to_cpu((cmd)->hdr.size);,
    pub \: *mut *mut (cmd)->hdr.size = cpu_to_le16(sizeof((cmd)));,
    pub \: __lbtf_cmd(priv, cmdnr, &(cmd)->hdr, __sz, cb, cb_arg);,

    pub in_cmd_size): *mut *mut cmd_header in_cmd, int,
    pub callback_arg): c_ulong,
    pub resp): *mut cmd_header,
