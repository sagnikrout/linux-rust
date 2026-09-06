//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/tx.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//

//
// TX PATH
//
// The Tx path uses a double buffer and a tx_control structure, each located
// at a fixed address in the device's memory. On startup, the host retrieves
// the pointers to these addresses. A double buffer allows for continuous data
// flow towards the device. The host keeps track of which buffer is available
// and alternates between these two buffers on a per packet basis.
//
// The size of each of the two buffers is large enough to hold the longest
// 802.3 packet - maximum size Ethernet packet + header + descriptor.
// TX complete indication will be received a-synchronously in a TX done cyclic
// buffer which is composed of 16 tx_result descriptors structures and is used
// in a cyclic manner.
//
// The TX (HOST) procedure is as follows:
// 1. Read the Tx path status, that will give the data_out_count.
// 2. goto 1, if not possible.
// i.e. if data_in_count - data_out_count >= HwBuffer size (2 for double
// buffer).
// 3. Copy the packet (preceded by double_buffer_desc), if possible.
// i.e. if data_in_count - data_out_count < HwBuffer size (2 for double
// buffer).
// 4. increment data_in_count.
// 5. Inform the firmware by generating a firmware internal interrupt.
// 6. FW will increment data_out_count after it reads the buffer.
//
// The TX Complete procedure:
// 1. To get a TX complete indication the host enables the tx_complete flag in
// the TX descriptor Structure.
// 2. For each packet with a Tx Complete field set, the firmware adds the
// transmit results to the cyclic buffer (txDoneRing) and sets both done_1
// and done_2 to 1 to indicate driver ownership.
// 3. The firmware sends a Tx Complete interrupt to the host to trigger the
// host to process the new data. Note: interrupt will be send per packet if
// TX complete indication was requested in tx_control or per crossing
// aggregation threshold.
// 4. After receiving the Tx Complete interrupt, the host reads the
// TxDescriptorDone information in a cyclic manner and clears both done_1
// and done_2 fields.
//
pub const TX_COMPLETE_REQUIRED_BIT: c_uint = 0x80;
pub const TX_STATUS_DATA_OUT_COUNT_MASK: c_uint = 0xf;
pub const WL1251_TX_ALIGN_TO: c_int = 4;

pub const WL1251_TKIP_IV_SPACE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_control {
// Rate Policy (class) index
    pub rate_policy:3: unsigned,
// When set, no ack policy is expected
    pub ack_policy:1: unsigned,
//
// Packet type:
// 0 -> 802.11
// 1 -> 802.3
// 2 -> IP
// 3 -> raw codec
//
    pub packet_type:2: unsigned,
// If set, this is a QoS-Null or QoS-Data frame
    pub qos:1: unsigned,
//
// If set, the target triggers the tx complete INT
// upon frame sending completion.
//
    pub tx_complete:1: unsigned,
// 2 bytes padding before packet header
    pub xfer_pad:1: unsigned,
    pub reserved:7: unsigned,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_double_buffer_desc {
// Length of payload, including headers.
    pub length: __le16,
//
// A bit mask that specifies the initial rate to be used
// Possible values are:
// 0x0001 - 1Mbits
// 0x0002 - 2Mbits
// 0x0004 - 5.5Mbits
// 0x0008 - 6Mbits
// 0x0010 - 9Mbits
// 0x0020 - 11Mbits
// 0x0040 - 12Mbits
// 0x0080 - 18Mbits
// 0x0100 - 22Mbits
// 0x0200 - 24Mbits
// 0x0400 - 36Mbits
// 0x0800 - 48Mbits
// 0x1000 - 54Mbits
//
    pub rate: __le16,
// Time in us that a packet can spend in the target
    pub expiry_time: __le32,
// index of the TX queue used for this packet
    pub xmit_queue: u8,
// Used to identify a packet
    pub id: u8,
    pub control: tx_control,
//
// The FW should cut the packet into fragments
// of this size.
//
    pub frag_threshold: __le16,
// Numbers of HW queue blocks to be allocated
    pub num_mem_blocks: u8,
    pub reserved: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_result {
//
// Ownership synchronization between the host and
// the firmware. If done_1 and done_2 are cleared,
// owned by the FW (no info ready).
//
    pub done_1: u8,
// same as double_buffer_desc->id
    pub id: u8,
//
// Total air access duration consumed by this
// packet, including all retries and overheads.
//
    pub medium_usage: u16,
// Total media delay (from 1st EDCA AIFS counter until TX Complete).
    pub medium_delay: u32,
// Time between host xfer and tx complete
    pub fw_hnadling_time: u32,
// The LS-byte of the last TKIP sequence number.
    pub lsb_seq_num: u8,
// Retry count
    pub ack_failures: u8,
// At which rate we got a ACK
    pub rate: u16,
    pub reserved: u16,
// TX_*
    pub status: u8,
// See done_1
    pub done_2: u8,
    pub __packed: },
    pub QOS_AC_VO: return,
    pub QOS_AC_VI: return,
    pub QOS_AC_BE: return,
    pub QOS_AC_BK: return,
    pub QOS_AC_BE: return,
    pub work): *mut void wl1251_tx_work(struct work_struct,
    pub wl): *mut void wl1251_tx_complete(struct wl1251,
    pub wl): *mut void wl1251_tx_flush(struct wl1251,
