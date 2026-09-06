//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7615/mcu.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2019 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_mcu_txd {
    pub txd: [__le32; 8],
    pub len: __le16,
    pub pq_id: __le16,
    pub cid: u8,
    pub pkt_type: u8,
    pub /: *mut *mut u8 set_query; / FW don't care,
    pub seq: u8,
    pub uc_d2b0_rev: u8,
    pub ext_cid: u8,
    pub s2d_index: u8,
    pub ext_cid_ack: u8,
    pub reserved: [u32; 5],
    pub __aligned(4): } __packed,
//
// struct mt7615_uni_txd - mcu command descriptor for firmware v3
// @txd: hardware descriptor
// @len: total length not including txd
// @cid: command identifier
// @pkt_type: must be 0xa0 (cmd packet by long format)
// @frag_n: fragment number
// @seq: sequence number
// @checksum: 0 mean there is no checksum
// @s2d_index: index for command source and destination
// Definition              | value | note
// CMD_S2D_IDX_H2N         | 0x00  | command from HOST to WM
// CMD_S2D_IDX_C2N         | 0x01  | command from WA to WM
// CMD_S2D_IDX_H2C         | 0x02  | command from HOST to WA
// CMD_S2D_IDX_H2N_AND_H2C | 0x03  | command from HOST to WA and WM
//
// @option: command option
// BIT[0]: UNI_CMD_OPT_BIT_ACK
// set to 1 to request a fw reply
// if UNI_CMD_OPT_BIT_0_ACK is set and UNI_CMD_OPT_BIT_2_SET_QUERY
// is set, mcu firmware will send response event EID = 0x01
// (UNI_EVENT_ID_CMD_RESULT) to the host.
// BIT[1]: UNI_CMD_OPT_BIT_UNI_CMD
// 0: original command
// 1: unified command
// BIT[2]: UNI_CMD_OPT_BIT_SET_QUERY
// 0: QUERY command
// 1: SET command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_uni_txd {
    pub txd: [__le32; 8],
// DW1
    pub len: __le16,
    pub cid: __le16,
// DW2
    pub reserved: u8,
    pub pkt_type: u8,
    pub frag_n: u8,
    pub seq: u8,
// DW3
    pub checksum: __le16,
    pub s2d_index: u8,
    pub option: u8,
// DW4
    pub reserved2: [u8; 4],
    pub __aligned(4): } __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_mcu_rxd {
    pub rxd: [__le32; 4],
    pub len: __le16,
    pub pkt_type_id: __le16,
    pub eid: u8,
    pub seq: u8,
    pub __rsv: __le16,
    pub ext_eid: u8,
    pub __rsv1: [u8; 2],
    pub s2d_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_mcu_csa_notify {
    pub rxd: mt7615_mcu_rxd,
    pub omac_idx: u8,
    pub csa_count: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_mcu_rdd_report {
    pub rxd: mt7615_mcu_rxd,
    pub band_idx: u8,
    pub long_detected: u8,
    pub constant_prf_detected: u8,
    pub staggered_prf_detected: u8,
    pub radar_type_idx: u8,
    pub periodic_pulse_num: u8,
    pub long_pulse_num: u8,
    pub hw_pulse_num: u8,
    pub out_lpn: u8,
    pub out_spn: u8,
    pub out_crpn: u8,
    pub out_crpw: u8,
    pub out_crbn: u8,
    pub out_stgpn: u8,
    pub out_stgpw: u8,
    pub _rsv: [u8; 2],
    pub out_pri_const: __le32,
    pub out_pri_stg: [__le32; 3],
    pub start: __le32,
    pub pulse_width: __le16,
    pub pulse_power: __le16,
    pub long_pulse: [}; 32],
    pub start: __le32,
    pub pulse_width: __le16,
    pub pulse_power: __le16,
    pub periodic_pulse: [}; 32],
    pub start: __le32,
    pub pulse_width: __le16,
    pub pulse_power: __le16,
    pub sc_pass: u8,
    pub sw_reset: u8,
    pub hw_pulse: [}; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_roc_tlv {
    pub bss_idx: u8,
    pub token: u8,
    pub active: u8,
    pub primary_chan: u8,
    pub sco: u8,
    pub band: u8,
    pub /: *mut *mut u8 width; / To support 80/160MHz bandwidth,
    pub /: *mut *mut u8 freq_seg1; / To support 80/160MHz bandwidth,
    pub /: *mut *mut u8 freq_seg2; / To support 80/160MHz bandwidth,
    pub req_type: u8,
    pub dbdc_band: u8,
    pub rsv0: u8,
    pub /: *mut *mut __le32 max_interval; / ms,
    pub rsv1: [u8; 8],
    pub __packed: },
}
