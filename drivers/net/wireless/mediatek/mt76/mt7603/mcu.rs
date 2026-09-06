//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/mcu.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7603_mcu_txd {
    pub len: __le16,
    pub pq_id: __le16,
    pub cid: u8,
    pub pkt_type: u8,
    pub set_query: u8,
    pub seq: u8,
    pub uc_d2b0_rev: u8,
    pub ext_cid: u8,
    pub uc_d2b2_rev: u8,
    pub ext_cid_ack: u8,
    pub au4_d3_to_d7_rev: [u32; 5],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7603_mcu_rxd {
    pub len: __le16,
    pub pkt_type_id: __le16,
    pub eid: u8,
    pub seq: u8,
    pub __rsv: __le16,
    pub ext_eid: u8,
    pub __rsv1: [u8; 3],
}

pub const MCU_PKT_ID: c_uint = 0xa0;
pub const MCU_PORT_QUEUE: c_uint = 0x8000;
pub const MCU_PORT_QUEUE_FW: c_uint = 0xc000;
pub const MCU_FIRMWARE_ADDRESS: c_uint = 0x100000;
